import Lean

/-!
# Semantics.Authoring

The authoring boundary inspects elaborated terms before reduction erases their template and definition calls.
Its retained tree records named expansion calls and data composition. The kernel checks
that the tree contains no raw semantic constructor; correspondence between the elaborated
term and this tree is the elaborator's responsibility.
-/

namespace Semantics

/-- Expansion-time lexical depth. This argument is eliminated when a template or definition
constructs its body. -/
class AuthoringContext where
  depth : Nat

abbrev AuthoringContext.root : AuthoringContext := ⟨0⟩

instance : AuthoringContext := AuthoringContext.root

abbrev AuthoringContext.child (context : AuthoringContext) : AuthoringContext := ⟨context.depth + 1⟩

end Semantics

namespace Semantics.Authoring

open Lean Meta Elab Command

inductive Form where
  | data (children : List Form)
  | literal (value : Literal)
  | constructor (name : Name) (fields : List Form)
  | call (name : Name) (arguments : List Form)
  | parameter (name : Name)
  | binder (name : Name) (body : Form)
  | raw (name : Name)
  deriving Repr, ToExpr

mutual
  def Form.onlyAuthored : Form → Bool
    | .data children | .call _ children | .constructor _ children => Forms.onlyAuthored children
    | .literal _ => true
    | .parameter _ => true
    | .binder _ body => body.onlyAuthored
    | .raw _ => false

  def Forms.onlyAuthored : List Form → Bool
    | [] => true
    | form :: forms => form.onlyAuthored && Forms.onlyAuthored forms
end

mutual
  def Form.rawNames : Form → List Name
    | .data children | .call _ children | .constructor _ children => Forms.rawNames children
    | .literal _ => []
    | .parameter _ => []
    | .binder _ body => body.rawNames
    | .raw name => [name]

  def Forms.rawNames : List Form → List Name
    | [] => []
    | form :: forms => form.rawNames ++ Forms.rawNames forms
end

private opaque FinalizeContext : Type

/-- Elaborate a mutual inductive block with all standard recursors, `casesOn`, `noConfusion`, and
`below`, omitting the eager `mkBRecOn`, `SizeOf`, and injectivity-theorem loops across all nested
occurrences (`mkBRecOn` can be invoked on demand for individual spine types). -/
private unsafe def runFastInductives (inductives : Array (Modifiers × Syntax)) : CommandElabM Unit := do
  let env ← getEnv
  let checkNoConflicts ← IO.ofExcept <| env.evalConst
    (Array InductiveElabStep1 → Bool → CommandElabM Unit)
    Options.empty
    "_private.Lean.Elab.MutualInductive.0.Lean.Elab.Command.checkNoInductiveNameConflicts".toName
  let mkInductiveDecl ← IO.ofExcept <| env.evalConst
    (Array Expr → Array InductiveElabStep1 → TermElabM FinalizeContext)
    Options.empty
    "_private.Lean.Elab.MutualInductive.0.Lean.Elab.Command.mkInductiveDecl".toName
  let elabInductiveViewsFinalize ← IO.ofExcept <| env.evalConst
    (Array InductiveView → FinalizeContext → CommandElabM Unit)
    Options.empty
    "_private.Lean.Elab.MutualInductive.0.Lean.Elab.Command.elabInductiveViewsFinalize".toName
  let elabInductiveViewsPostprocessing ← IO.ofExcept <| env.evalConst
    (Array InductiveView → CommandElabM Unit)
    Options.empty
    "_private.Lean.Elab.MutualInductive.0.Lean.Elab.Command.elabInductiveViewsPostprocessing".toName
  let elabs ← runTermElabM fun _ => inductives.mapM fun (modifiers, stx) => mkInductiveView modifiers stx
  checkNoConflicts elabs false
  elabs.forM fun e => checkValidInductiveModifier e.view.modifiers
  liftTermElabM <| elabs.forM fun e =>
    withRef e.view.ref <| Term.applyAttributesAt e.view.declName e.view.modifiers.attrs .beforeElaboration
  let res ← runTermElabM fun vars => do
    let view0 := elabs[0]!.view
    Term.withDeclName view0.declName <| withRef view0.ref <| withExporting (isExporting := !isPrivateName view0.declName) do
      let res ← mkInductiveDecl vars elabs
      let declNames := elabs.map (·.view.declName)
      compileDecls declNames
      withoutExporting (when := view0.ctors.any fun x => isPrivateName x.declName) do
        for n in declNames do
          mkCasesOn n
          mkRecOn n
          mkCtorIdx n
          mkCtorElim n
          mkNoConfusion n
          mkBelow n
      for e in elabs do
        enableRealizationsForConst e.view.declName
        for ctor in e.view.ctors do
          enableRealizationsForConst ctor.declName
      pure res
  let views := elabs.map (·.view)
  elabInductiveViewsFinalize views res
  elabInductiveViewsPostprocessing views

@[implemented_by runFastInductives]
private opaque fastInductives (inductives : Array (Modifiers × Syntax)) : CommandElabM Unit

@[scoped command_elab Lean.Parser.Command.mutual]
def elabFastMutual : CommandElab := fun stx => do
  if ← isMutualInductive stx then
    let mut inductives := #[]
    for elem in stx[1].getArgs do
      let modifiers ← elabModifiers ⟨elem[0]⟩
      inductives := inductives.push (modifiers, elem[1])
    fastInductives inductives
  else
    throwUnsupportedSyntax


initialize definitionAttribute : TagAttribute ←
  registerTagAttribute `semantic_definition "A rules-defined semantic declaration."

initialize templateAttribute : TagAttribute ←
  registerTagAttribute `semantic_template "A transparent semantic phrasing template."

initialize expressionAttribute : TagAttribute ←
  registerTagAttribute `semantic_expression "Semantic expressions require template or definition authoring."

initialize literalAttribute : TagAttribute ←
  registerTagAttribute `semantic_literal "A literal leaf value, rather than an authored expression."

initialize internalAttribute : TagAttribute ←
  registerTagAttribute `internal_expansion "An expansion carrier without an authorable primitive template."

initialize contextualAuthoringAttribute : TagAttribute ←
  registerTagAttribute `contextual_authoring "A semantic declaration with generated lexical context."

private def addSpliceSchema (schemas : NameMap (Array Name)) (entry : Name × Array Name) :=
  schemas.insert entry.1 entry.2

initialize spliceSchemas : SimplePersistentEnvExtension (Name × Array Name) (NameMap (Array Name)) ←
  registerSimplePersistentEnvExtension {
    addEntryFn := addSpliceSchema
    addImportedFn := mkStateFromImportedEntries addSpliceSchema {}
  }

/-- A supplied expression or parameterized body retaining its caller's scope. -/
syntax (name := spliceMode) "splice " term : term

/-- Resolve supplied-body aliases in the declaration's ordinary binder context. -/
elab "caller_splice_type% " type:term : term => do
  let type ← Term.elabType type
  if (← whnf type).isForall then
    return mkForall .anonymous .instImplicit (mkConst ``AuthoringContext) (type.liftLooseBVars 0 1)
  return type

/-- Whether `name` is an authored declaration: a `semantic_definition` or a `semantic_template`
(the generated primitives are templates). -/
def isAuthored (env : Environment) (name : Name) : Bool :=
  definitionAttribute.hasTag env name || templateAttribute.hasTag env name

/-- Declare one named semantic expansion, preserving ordinary Lean binders, with its generated
lexical context and splice schema. The caller tags it a definition or a template. -/
private def elabAuthoredDecl (mods : TSyntax ``Parser.Command.declModifiers)
    (name : TSyntax ``Parser.Command.declId) (signature : TSyntax ``Parser.Command.optDeclSig)
    (value : TSyntax ``Parser.Command.declVal) : CommandElabM Unit := do
    let context ← `(bracketedBinder| [Semantics.AuthoringContext])
    let mut binders := #[context.raw]
    let mut splices : Array (TSyntax `ident) := #[]
    for binder in signature.raw[0].getArgs do
      let mut binder := binder
      if binder[2].getNumArgs == 2 then
        let annotated := binder[2][1]
        if annotated.isOfKind ``spliceMode then
          let resultType : Term := ⟨annotated[1]⟩
          let type := (← `(term| caller_splice_type% $resultType)).raw
          binder := binder.setArg 2 (binder[2].setArg 1 type)
          for identifier in binder[1].getArgs do
            unless identifier.isIdent do throwErrorAt identifier "Authored parameters must be named"
            splices := splices.push ⟨identifier⟩
      binders := binders.push binder
    let signature : TSyntax ``Lean.Parser.Command.optDeclSig :=
      ⟨signature.raw.setArg 0 (mkNullNode binders)⟩
    let mut value := value
    if !splices.isEmpty then
      unless value.raw.isOfKind ``Parser.Command.declValSimple do
        throwErrorAt value "A parameterized template or definition uses `:=` with one expansion body"
      let closeFn := mkIdent `Semantics.CallerSplice.close
      let mut body : Term := ⟨value.raw[1]⟩
      for identifier in splices.reverse do
        body ← `(term| let $identifier := $closeFn $identifier; $body)
      body ← `(term|
        letI : Semantics.AuthoringContext :=
          Semantics.AuthoringContext.child (inferInstance : Semantics.AuthoringContext)
        $body)
      value := ⟨value.raw.setArg 1 body.raw⟩
    elabCommand (← `($mods:declModifiers def $name:declId $signature:optDeclSig $value:declVal))
    let identifier : TSyntax `ident := ⟨name.raw[0]⟩
    elabCommand (← `(attribute [contextual_authoring] $identifier:ident))
    let declaration ← resolveGlobalConstNoOverload identifier
    modifyEnv fun env => spliceSchemas.addEntry env (declaration, splices.map (·.getId))

/-- The label a keyword ability Definition is written under (`.keyword (.named "kicker") …`), read
from its expansion with its parameters left abstract; `none` for any other Definition. -/
def keywordDefinitionLabel? (name : Name) : MetaM (Option String) := do
  let some value := (← getConstInfo name).value? | return none
  lambdaTelescope value fun _ body => do
    unless (← whnf (← inferType body)).isConstOf `Semantics.Ability do return none
    let body ← whnf body
    unless body.isAppOfArity `Semantics.Ability.keyword 3 do return none
    let label ← whnf body.appFn!.appFn!.appArg!
    unless label.isAppOfArity `Semantics.KeywordLabel.named 1 do return none
    match ← whnf label.appArg! with
    | .lit (.strVal label) => return some label
    | _ => return none

/-- The authored `KeywordLabel` a keyword ability Definition generates: `kicker.label` is the
label `kicker` is written under. A card names a keyword through it (`elabKeywordName`), never by
the internal `KeywordLabel.named`. -/
def keywordLabelConstant (name : Name) : Name := name ++ `label

/-- The keyword-action deed an Action Definition establishes its own Reference Scope for
(`.ownScope (some (.action "exile")) …`), read from its expansion with its parameters left
abstract; `none` for any other Definition, or a deed that depends on a parameter. -/
def actionDefinitionDeed? (name : Name) : MetaM (Option Expr) := do
  let some value := (← getConstInfo name).value? | return none
  lambdaTelescope value fun _ body => do
    unless (← whnf (← inferType body)).isConstOf `Semantics.Action do return none
    let body ← whnf body
    unless body.isAppOfArity `Semantics.Action.ownScope 2 do return none
    let deed ← whnf body.appFn!.appArg!
    unless deed.isAppOfArity `Option.some 2 do return none
    let deed ← instantiateMVars deed.appArg!
    let deed ← whnf deed
    unless deed.isAppOfArity `Semantics.Deed.action 2 do return none
    let label ← whnf deed.appFn!.appArg!
    let permanentActor ← whnf deed.appArg!
    unless label.isStringLit && !permanentActor.hasFVar && !permanentActor.hasMVar do
      return none
    return some (mkApp2 (mkConst `Semantics.Deed.action) label permanentActor)

/-- The authored `Deed` an Action Definition generates: `exile.deed` is the keyword action
`exile` performs. A card names a deed through it (`elabDeedName`), never by the internal
`Deed.action`. -/
def deedConstant (name : Name) : Name := name ++ `deed

syntax (name := semanticDefinitionDecl) declModifiers "semantic_definition " declId
  ppIndent(optDeclSig) declVal : command

syntax (name := semanticTemplateDecl) declModifiers "semantic_template " declId
  ppIndent(optDeclSig) declVal : command

elab_rules : command
  | `($mods:declModifiers semantic_definition $name:declId
      $signature:optDeclSig $value:declVal) => do
    elabAuthoredDecl mods name signature value
    let identifier : TSyntax `ident := ⟨name.raw[0]⟩
    let declaration ← resolveGlobalConstNoOverload identifier
    liftTermElabM do
      let info ← getConstInfo declaration
      if let .defnInfo definition := info then
        lambdaTelescope definition.value fun _ body => do
          if (← whnf (← inferType body)).isConstOf `Semantics.Action then
            unless (← whnf body).getAppFn.isConstOf `Semantics.Action.ownScope do
              throwError "An Action Definition must establish its own Reference Scope"
    elabCommand (← `(attribute [«semantic_definition»] $identifier:ident))
    if let some label ← liftTermElabM (keywordDefinitionLabel? declaration) then
      let labelName := keywordLabelConstant declaration
      liftCoreM <| addAndCompile <| .defnDecl {
        name := labelName
        levelParams := []
        type := mkConst `Semantics.KeywordLabel
        value := mkApp (mkConst `Semantics.KeywordLabel.named) (mkStrLit label)
        hints := .regular 0
        safety := .safe
      }
      definitionAttribute.setTag labelName
    if let some deed ← liftTermElabM (actionDefinitionDeed? declaration) then
      let deedName := deedConstant declaration
      liftCoreM <| addAndCompile <| .defnDecl {
        name := deedName
        levelParams := []
        type := mkConst `Semantics.Deed
        value := deed
        hints := .regular 0
        safety := .safe
      }
      definitionAttribute.setTag deedName
  | `($mods:declModifiers semantic_template $name:declId
      $signature:optDeclSig $value:declVal) => do
    elabAuthoredDecl mods name signature value
    let identifier : TSyntax `ident := ⟨name.raw[0]⟩
    elabCommand (← `(attribute [«semantic_template»] $identifier:ident))

/-- Every exposed primitive has a named declaration to which a spelling can attach.
Internal expansion carriers are deliberately absent from this generated interface. -/
elab "declare_semantic_primitives" : command => do
  let env ← getEnv
  let constructors := env.constants.toList.filterMap fun (name, info) =>
    match info with
    | .ctorInfo ctor => some (name, ctor)
    | _ => none
  for (name, ctor) in constructors.mergeSort (fun a b => a.2.numFields ≤ b.2.numFields) do
      if expressionAttribute.hasTag env ctor.induct &&
          !internalAttribute.hasTag env name && !internalAttribute.hasTag env ctor.induct then
        let publicName := name.replacePrefix `Semantics `Semantics.Primitives
        let current ← getEnv
        let type := ctor.type.replace fun expr =>
          match expr with
          | .const original levels =>
            let primitiveName := original.replacePrefix `Semantics `Semantics.Primitives
            if isAuthored current primitiveName then some (mkConst primitiveName levels)
            else none
          | _ => none
        let declaration : Declaration := .defnDecl {
          name := publicName
          levelParams := ctor.levelParams
          type := type
          value := mkConst name (ctor.levelParams.map Level.param)
          hints := .regular 0
          safety := .safe
        }
        liftCoreM <| addAndCompile declaration
        templateAttribute.setTag publicName

/-- Semantic content includes expressions nested inside otherwise ordinary data records. -/
partial def containsExpression (env : Environment) (type : Expr)
    (visited : List Name := []) : Bool :=
  match type with
  | .const name _ =>
    if expressionAttribute.hasTag env name then true
    else if visited.contains name then false
    else match env.find? name with
      | some (.inductInfo info) => info.ctors.any fun ctor =>
        match env.find? ctor with
        | some (.ctorInfo info) => containsExpression env info.type (name :: visited)
        | _ => false
      | some (.defnInfo info) => containsExpression env info.value (name :: visited)
      | _ => false
  | .app fn arg => containsExpression env fn visited || containsExpression env arg visited
  | .forallE _ domain body _ | .lam _ domain body _ =>
    containsExpression env domain visited || containsExpression env body visited
  | .letE _ type value body _ => containsExpression env type visited ||
    containsExpression env value visited || containsExpression env body visited
  | .mdata _ value | .proj _ _ value => containsExpression env value visited
  | _ => false

/-- Grammar modules classify their non-record, non-enumeration syntax uniformly. -/
elab "classify_semantic_syntax" : command => do
  let env ← getEnv
  for (name, info) in env.constants.toList do
    if (env.getModuleIdxFor? name).isNone && !isStructure env name then
      if let .inductInfo induct := info then
        let hasFields := induct.ctors.any fun ctor =>
          match env.find? ctor with
          | some (.ctorInfo ci) => ci.numFields > 0
          | _ => false
        if hasFields && !expressionAttribute.hasTag env name then
          expressionAttribute.setTag name

/-- Ordinary helper bodies can be skipped only after checking the reachable definitions
of their instantiated expansion. Semantic types matter too: they can hide lambda parameters
inside containers or discarded values. Opaque code remains subject to ordinary inspection. -/
private partial def helperNeedsInspection (env : Environment) (body : Expr)
    (clean : IO.Ref (Std.HashSet Name)) : IO Bool := do
  let known ← clean.get
  let rec visit (pending : List Name) (seen : Std.HashSet Name) : Bool × Std.HashSet Name :=
    match pending with
    | [] => (false, seen)
    | name :: rest =>
      if known.contains name || seen.contains name then visit rest seen
      else
        let seen := seen.insert name
        if isAuthored env name || expressionAttribute.hasTag env name then (true, seen)
        else match env.find? name with
          | some (.ctorInfo ctor) =>
            if expressionAttribute.hasTag env ctor.induct && !literalAttribute.hasTag env name then
              (true, seen)
            else visit rest seen
          | some (.defnInfo info) =>
            visit (info.value.foldConsts rest fun name names => name :: names) seen
          | some (.axiomInfo _) | some (.opaqueInfo _) => (true, seen)
          | _ => visit rest seen
  let (needed, seen) := visit (body.foldConsts [] fun name names => name :: names) {}
  unless needed do clean.modify fun previous => seen.fold (fun acc name => acc.insert name) previous
  return needed

private structure InspectionKey where
  term : Expr
  visited : List Name
  context : Expr
  parameters : List FVarId
  bodyParameter : Bool
  deriving BEq, Hashable

private abbrev InspectionCache := Std.HashMap InspectionKey Form

private partial def inspectCore (cache : IO.Ref InspectionCache)
    (clean : IO.Ref (Std.HashSet Name)) (term : Expr) (visited : List Name) (context : Expr) (parameters : List FVarId)
    (bodyParameter : Bool) : MetaM Form := do
  let term := (← instantiateMVars term).consumeMData
  let key : InspectionKey := ⟨term, visited, context, parameters, bodyParameter⟩
  if let some form := (← cache.get)[key]? then return form
  let form ← do
    if (← isProof term) || (← isType term) then return .data []
    match term with
    | .lit value => return .literal value
    | .fvar id =>
      if parameters.contains id then return .parameter (← id.getDecl).userName
      if containsExpression (← getEnv) (← inferType term) then
        throwError "Semantic parameter {term} has no authoring evidence"
      return .data []
    | .lam name type body info =>
      withLocalDecl name info type fun arg => do
        if bodyParameter && (← isDefEq type (mkConst ``AuthoringContext)) then
          return ← inspectCore cache clean (body.instantiate1 arg) visited arg parameters true
        if bodyParameter && (← whnf type).isConstOf `Semantics.NounPhrase then
          return .binder name (← inspectCore cache clean (body.instantiate1 arg) visited context
            (arg.fvarId! :: parameters) true)
        -- A binder of an ordinary record that only holds semantic expressions (a `KeywordTerm`,
        -- holding a `KeywordLabel`) is filled by the template the function is passed to, from
        -- arguments inspected where they are written: an applied function is beta-reduced
        -- first. Such a record has no Reference Scope to close, so unlike a semantic
        -- expression's binder (`NounPhrase`) it needs no supplied body.
        let env ← getEnv
        if let .const typeName _ := (← whnf type).getAppFn then
          if !expressionAttribute.hasTag env typeName && containsExpression env type then
            return .binder name (← inspectCore cache clean (body.instantiate1 arg) visited context
              (arg.fvarId! :: parameters) bodyParameter)
        inspectCore cache clean (body.instantiate1 arg) visited context parameters bodyParameter
    | .letE _ _ value body _ =>
      return .data [← inspectCore cache clean value visited context parameters bodyParameter,
        ← inspectCore cache clean (body.instantiate1 value) visited context parameters bodyParameter]
    | .proj _ _ object => inspectCore cache clean object visited context parameters bodyParameter
    | _ =>
      let fn := term.getAppFn
      let args := term.getAppArgs
      match fn with
      | .const name levels =>
        let env ← getEnv
        let info ← getConstInfo name
        if isAuthored env name then
          let mut arguments := []
          let splices := (spliceSchemas.getState env |>.find? name).getD #[]
          let mut type := info.type.instantiateLevelParams info.levelParams levels
          for arg in args do
            let mut isBody := false
            if let .forallE parameter _ next _ := ← whnf type then
              type := next.instantiate1 arg
              isBody := splices.contains parameter.eraseMacroScopes
            if contextualAuthoringAttribute.hasTag env name &&
                (← isDefEq (← inferType arg) (mkConst ``AuthoringContext)) then
              unless ← isDefEq arg context do
                throwError "{name} uses a lexical context outside its authoring scope"
            else
              arguments := arguments ++ [← inspectCore cache clean arg visited context parameters isBody]
          return .call name arguments
        match info with
        | .ctorInfo ctor =>
          if expressionAttribute.hasTag env ctor.induct && !literalAttribute.hasTag env name then
            return .raw name
          return .constructor name (← args.toList.mapM fun arg =>
            inspectCore cache clean arg visited context parameters bodyParameter)
        | .defnInfo defn =>
          let arguments ← args.toList.mapM fun arg =>
            inspectCore cache clean arg visited context parameters bodyParameter
          if visited.contains name then return .data arguments
          let resultType ← whnf (← inferType term)
          let semanticFunction := resultType.isForall && containsExpression env resultType
          let expansion := (defn.value.instantiateLevelParams defn.levelParams levels).beta args
          if !semanticFunction && !(← helperNeedsInspection env expansion clean) then
            return .data arguments
          let body ← inspectCore cache clean expansion
            (name :: visited) context parameters bodyParameter
          return .data (body :: arguments)
        | _ =>
          if containsExpression env (← inferType term) then
            throwError "Definition {name} has no authoring evidence"
          return .data (← args.toList.mapM fun arg => inspectCore cache clean arg visited context parameters bodyParameter)
      | _ =>
        if args.isEmpty then throwError "Cannot establish authorship for {term}"
        let arguments ← args.toList.mapM fun arg => inspectCore cache clean arg visited context parameters bodyParameter
        let body ← inspectCore cache clean (if fn.isLambda then fn.beta args else fn) visited context parameters bodyParameter
        return .data (body :: arguments)
  cache.modify (·.insert key form)
  return form

/-- Cache only identical inspections, including lexical context, permitted parameters,
and the unfolding path. Each occurrence still retains its own place in the authoring tree. -/
def inspect (term : Expr) (visited : List Name := [])
    (context : Expr := mkConst ``AuthoringContext.root) (parameters : List FVarId := [])
    (bodyParameter : Bool := false) : MetaM Form := do
  let cache ← IO.mkRef ({} : InspectionCache)
  let clean ← IO.mkRef ({} : Std.HashSet Name)
  inspectCore cache clean term visited context parameters bodyParameter

end Semantics.Authoring

/-- Inspect an expansion explicitly without changing its semantic result or granting authorship. -/
def Semantics.expand.{u} {α : Sort u} (value : α) : α := value
