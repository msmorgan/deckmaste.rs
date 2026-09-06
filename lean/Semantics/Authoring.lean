import Lean

/-!
# Semantics.Authoring

The authoring boundary inspects elaborated terms before reduction erases their macro calls.
Its retained tree records named expansion calls and data composition. The kernel checks
that the tree contains no raw semantic constructor; correspondence between the elaborated
term and this tree is the elaborator's responsibility.
-/

namespace Semantics.Authoring

open Lean Meta Elab Command

inductive Form where
  | data (children : List Form)
  | literal (value : Literal)
  | constructor (name : Name) (fields : List Form)
  | call (name : Name) (arguments : List Form)
  | raw (name : Name)
  deriving Repr, ToExpr

mutual
  def Form.onlyMacros : Form → Bool
    | .data children | .call _ children | .constructor _ children => Forms.onlyMacros children
    | .literal _ => true
    | .raw _ => false

  def Forms.onlyMacros : List Form → Bool
    | [] => true
    | form :: forms => form.onlyMacros && Forms.onlyMacros forms
end

mutual
  def Form.rawNames : Form → List Name
    | .data children | .call _ children | .constructor _ children => Forms.rawNames children
    | .literal _ => []
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

/-- Register the public definitions in the current macro namespace, including defaults. -/
elab "register_semantic_macros" : command => do
  let ns ← getCurrNamespace
  let env ← getEnv
  let names := env.constants.toList.filterMap fun (name, info) =>
    if ns.isPrefixOf name && (env.getModuleIdxFor? name).isNone && info.isDefinition then
      some name else none
  for name in names do
    macroAttribute.setTag name

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

/-- Inspect helper definitions as well as nested macro arguments; aliases cannot hide raw
syntax. Macro bodies are trusted definitions and are not expanded by this traversal. -/
partial def inspect (term : Expr) (visited : List Name := []) : MetaM Form := do
  let term ← instantiateMVars term
  if (← isProof term) || (← isType term) then return .data []
  let term := term.consumeMData
  match term with
  | .lit value => return .literal value
  | .fvar _ =>
    if containsExpression (← getEnv) (← inferType term) then
      throwError "Semantic parameter {term} has no macro-authoring evidence"
    return .data []
  | .lam name type body info =>
    withLocalDecl name info type fun arg => inspect (body.instantiate1 arg) visited
  | .letE _ _ value body _ =>
    return .data [← inspect value visited, ← inspect (body.instantiate1 value) visited]
  | .proj _ _ object =>
    if containsExpression (← getEnv) (← inferType object) then inspect object visited
    else pure (.data [])
  | _ =>
    let fn := term.getAppFn
    let args := term.getAppArgs
    match fn with
    | .const name levels =>
      let env ← getEnv
      if macroAttribute.hasTag env name then
        return .call name (← args.toList.mapM fun arg => inspect arg visited)
      let info ← getConstInfo name
      match info with
      | .ctorInfo ctor =>
        if expressionAttribute.hasTag env ctor.induct && !literalAttribute.hasTag env name then
          return .raw name
        return .constructor name (← args.toList.mapM fun arg => inspect arg visited)
      | .defnInfo defn =>
        let arguments ← args.toList.mapM fun arg => inspect arg visited
        if visited.contains name then return .data arguments
        let body ← inspect
          ((defn.value.instantiateLevelParams defn.levelParams levels).beta args) (name :: visited)
        return .data (body :: arguments)
      | _ =>
        if containsExpression env (← inferType term) then
          throwError "Definition {name} has no macro-authoring evidence"
        return .data (← args.toList.mapM fun arg => inspect arg visited)
    | _ =>
      if args.isEmpty then throwError "Cannot establish macro authorship for {term}"
      let arguments ← args.toList.mapM fun arg => inspect arg visited
      let body ← inspect (if fn.isLambda then fn.beta args else fn) visited
      return .data (body :: arguments)

end Semantics.Authoring
