import Lean

/-!
# Semantics.Authoring

The authoring boundary inspects elaborated terms before reduction erases their macro calls.
Its retained tree records named expansion calls and data composition. The kernel checks
that the tree contains no raw semantic constructor; correspondence between the elaborated
term and this tree is the elaborator's responsibility.
-/

namespace Semantics

/-- Expansion-time lexical depth. This argument is eliminated when a macro constructs its body. -/
class MacroContext where
  depth : Nat

abbrev MacroContext.root : MacroContext := ⟨0⟩

instance : MacroContext := MacroContext.root

abbrev MacroContext.child (context : MacroContext) : MacroContext := ⟨context.depth + 1⟩

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
  def Form.onlyMacros : Form → Bool
    | .data children | .call _ children | .constructor _ children => Forms.onlyMacros children
    | .literal _ => true
    | .parameter _ => true
    | .binder _ body => body.onlyMacros
    | .raw _ => false

  def Forms.onlyMacros : List Form → Bool
    | [] => true
    | form :: forms => form.onlyMacros && Forms.onlyMacros forms
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

initialize macroAttribute : TagAttribute ←
  registerTagAttribute `semantic_macro "A named, trusted semantic macro expansion."

initialize expressionAttribute : TagAttribute ←
  registerTagAttribute `semantic_expression "Semantic expressions require macro authoring."

initialize literalAttribute : TagAttribute ←
  registerTagAttribute `semantic_literal "A literal leaf value, rather than an authored expression."

initialize internalAttribute : TagAttribute ←
  registerTagAttribute `internal_expansion "An expansion carrier without an authorable primitive macro."

initialize contextualMacroAttribute : TagAttribute ←
  registerTagAttribute `contextual_macro "A semantic declaration with generated lexical context."

private def addSpliceSchema (schemas : NameMap (Array Name)) (entry : Name × Array Name) :=
  schemas.insert entry.1 entry.2

initialize spliceSchemas : SimplePersistentEnvExtension (Name × Array Name) (NameMap (Array Name)) ←
  registerSimplePersistentEnvExtension {
    addEntryFn := addSpliceSchema
    addImportedFn := mkStateFromImportedEntries addSpliceSchema {}
  }

/-- A parameter whose subject or numeric value is captured at this expansion. -/
syntax (name := captureMode) "capture " term : term
/-- A supplied expression or parameterized body retaining its caller's scope. -/
syntax (name := spliceMode) "splice " term : term

/-- Resolve supplied-body aliases in the declaration's ordinary binder context. -/
elab "macro_splice_type% " type:term : term => do
  let type ← Term.elabType type
  if (← whnf type).isForall then
    return mkForall .anonymous .instImplicit (mkConst ``MacroContext) (type.liftLooseBVars 0 1)
  return type

/-- Declare and register one named semantic expansion, preserving ordinary Lean binders. -/
syntax (name := semanticMacroDecl) declModifiers "semantic_macro " declId
  ppIndent(optDeclSig) declVal : command

elab_rules : command
  | `($mods:declModifiers semantic_macro $name:declId $signature:optDeclSig $value:declVal) => do
    let context ← `(bracketedBinder| [Semantics.MacroContext])
    let mut binders := #[context.raw]
    let mut captures : Array (TSyntax `ident) := #[]
    let mut splices : Array (TSyntax `ident) := #[]
    for binder in signature.raw[0].getArgs do
      let mut binder := binder
      if binder[2].getNumArgs == 2 then
        let annotated := binder[2][1]
        if annotated.isOfKind ``captureMode || annotated.isOfKind ``spliceMode then
          let mut type := annotated[1]
          let isSplice := annotated.isOfKind ``spliceMode
          if isSplice then
            let resultType : Term := ⟨type⟩
            type := (← `(term| macro_splice_type% $resultType)).raw
          binder := binder.setArg 2 (binder[2].setArg 1 type)
          for identifier in binder[1].getArgs do
            unless identifier.isIdent do throwErrorAt identifier "Macro parameters must be named"
            if isSplice then splices := splices.push ⟨identifier⟩
            else captures := captures.push ⟨identifier⟩
      binders := binders.push binder
    let signature : TSyntax ``Lean.Parser.Command.optDeclSig :=
      ⟨signature.raw.setArg 0 (mkNullNode binders)⟩
    let mut value := value
    if !captures.isEmpty || !splices.isEmpty then
      unless value.raw.isOfKind ``Parser.Command.declValSimple do
        throwErrorAt value "A parameterized macro uses `:=` with one expansion body"
      let scope := mkIdent (← liftCoreM (mkFreshUserName `macroScope))
      let inputs := mkIdent (← liftCoreM (mkFreshUserName `macroInputs))
      let inputFn := mkIdent `Semantics.MacroCapture.input
      let readFn := mkIdent `Semantics.MacroCapture.read
      let bindFn := mkIdent `Semantics.MacroExpansion.bind
      let closeFn := mkIdent `Semantics.MacroSplice.close
      let sourceInputs ← captures.mapM fun identifier =>
        `(term| $inputFn $identifier)
      let body : Term := ⟨value.raw[1]⟩
      let mut body ← `(term| $bindFn $scope $inputs $body)
      for identifier in splices.reverse do
        body ← `(term| let $identifier := $closeFn $scope $identifier; $body)
      for (identifier, index) in captures.zipIdx |>.reverse do
        let index := Syntax.mkNumLit (toString index)
        body ← `(term| let $identifier := $readFn $scope $index $identifier; $body)
      body ← `(term|
        let $scope := (inferInstance : Semantics.MacroContext).depth
        let $inputs := [$sourceInputs,*]
        letI : Semantics.MacroContext :=
          Semantics.MacroContext.child (inferInstance : Semantics.MacroContext)
        $body)
      value := ⟨value.raw.setArg 1 body.raw⟩
    elabCommand (← `($mods:declModifiers def $name:declId $signature:optDeclSig $value:declVal))
    let identifier : TSyntax `ident := ⟨name.raw[0]⟩
    elabCommand (← `(attribute [«semantic_macro», contextual_macro] $identifier:ident))
    let declaration ← resolveGlobalConstNoOverload identifier
    modifyEnv fun env => spliceSchemas.addEntry env (declaration, splices.map (·.getId))

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
        let publicName := name.replacePrefix `Semantics `Semantics.Macros.Primitives
        let current ← getEnv
        let type := ctor.type.replace fun expr =>
          match expr with
          | .const original levels =>
            let macroName := original.replacePrefix `Semantics `Semantics.Macros.Primitives
            if macroAttribute.hasTag current macroName then some (mkConst macroName levels)
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
        macroAttribute.setTag publicName

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
        if macroAttribute.hasTag env name || expressionAttribute.hasTag env name then (true, seen)
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
        throwError "Semantic parameter {term} has no macro-authoring evidence"
      return .data []
    | .lam name type body info =>
      withLocalDecl name info type fun arg => do
        if bodyParameter && (← isDefEq type (mkConst ``MacroContext)) then
          return ← inspectCore cache clean (body.instantiate1 arg) visited arg parameters true
        if bodyParameter && (← whnf type).isConstOf `Semantics.NounPhrase then
          return .binder name (← inspectCore cache clean (body.instantiate1 arg) visited context
            (arg.fvarId! :: parameters) true)
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
        if macroAttribute.hasTag env name then
          let mut arguments := []
          let splices := (spliceSchemas.getState env |>.find? name).getD #[]
          let mut type := info.type.instantiateLevelParams info.levelParams levels
          for arg in args do
            let mut isBody := false
            if let .forallE parameter _ next _ := ← whnf type then
              type := next.instantiate1 arg
              isBody := splices.contains parameter.eraseMacroScopes
            if contextualMacroAttribute.hasTag env name &&
                (← isDefEq (← inferType arg) (mkConst ``MacroContext)) then
              unless ← isDefEq arg context do
                throwError "Macro {name} uses a lexical context outside its authoring scope"
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
            throwError "Definition {name} has no macro-authoring evidence"
          return .data (← args.toList.mapM fun arg => inspectCore cache clean arg visited context parameters bodyParameter)
      | _ =>
        if args.isEmpty then throwError "Cannot establish macro authorship for {term}"
        let arguments ← args.toList.mapM fun arg => inspectCore cache clean arg visited context parameters bodyParameter
        let body ← inspectCore cache clean (if fn.isLambda then fn.beta args else fn) visited context parameters bodyParameter
        return .data (body :: arguments)
  cache.modify (·.insert key form)
  return form

/-- Cache only identical inspections, including lexical context, permitted parameters,
and the unfolding path. Each occurrence still retains its own place in the authoring tree. -/
def inspect (term : Expr) (visited : List Name := [])
    (context : Expr := mkConst ``MacroContext.root) (parameters : List FVarId := [])
    (bodyParameter : Bool := false) : MetaM Form := do
  let cache ← IO.mkRef ({} : InspectionCache)
  let clean ← IO.mkRef ({} : Std.HashSet Name)
  inspectCore cache clean term visited context parameters bodyParameter

end Semantics.Authoring

/-- Inspect an expansion explicitly without changing its semantic result or granting authorship. -/
def Semantics.expand.{u} {α : Sort u} (value : α) : α := value
