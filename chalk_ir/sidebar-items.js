initSidebarItems({"enum":[["AliasTy",""],["ClausePriority",""],["ConstValue",""],["Constraint","A constraint on lifetimes."],["DomainGoal","A \"domain goal\" is a goal that is directly about Rust, rather than a pure logical statement. As much as possible, the Chalk solver should avoid decomposing this enum, and instead treat its values opaquely."],["FloatTy",""],["FromEnv",""],["GenericArgData",""],["GoalData","A general goal; this is the full range of questions you can pose to Chalk."],["IntTy",""],["LifetimeData",""],["Mutability",""],["QuantifierKind",""],["Scalar",""],["TyData",""],["TyKind","Represents some extra knowledge we may have about the type variable. `ignore let x: &[u32]; let i = 1; x[i] ` In this example, `i` is known to be some type of integer. We can infer that it is `usize` because that is the only integer type that slices have an `Index` impl for. `i` would have a `TyKind` of `Integer` to guide the inference process."],["TypeName",""],["UintTy",""],["VariableKind",""],["Void",""],["WellFormed",""],["WhereClause","Where clauses that can be written by a Rust programmer."]],"macro":[["const_visit",""],["copy_fold",""],["debug",""],["debug_heading",""],["id_fold",""],["id_visit",""],["info",""],["info_heading",""]],"mod":[["cast",""],["could_match",""],["debug",""],["debug_macros",""],["fold","Traits for transforming bits of IR."],["interner",""],["visit","Traits for visiting bits of IR."],["zip",""]],"struct":[["AdtId",""],["AliasEq","Proves equality between an alias and a type."],["AnswerSubst",""],["ApplicationTy",""],["AssocTypeId","The id for the associated type member of a trait. The details of the type can be found by invoking the [`associated_ty_data`] method."],["Binders","Indicates that the `value` is universally quantified over `N` parameters of the given kinds, where `N == self.binders.len()`. A variable with depth `i < N` refers to the value at `self.binders[i]`. Variables with depth `>= N` are free."],["BindersIntoIterator",""],["BoundVar","Identifies a particular bound variable within a binder. Variables are identified by the combination of a [`DebruijnIndex`], which identifies the binder, and an index within that binder."],["Canonical","Wraps a \"canonicalized item\". Items are canonicalized as follows:"],["CanonicalVarKinds",""],["ClauseId",""],["ConcreteConst",""],["Const",""],["ConstData",""],["ConstrainedSubst","Combines a substitution (`subst`) with a set of region constraints (`constraints`). This represents the result of a query; the substitution stores the values for the query's unknown variables, and the constraints represents any region constraints that must additionally be solved."],["DebruijnIndex","References the binder at the given depth. The index is a [de Bruijn index], so it counts back through the in-scope binders, with 0 being the innermost binder. This is used in impls and the like. For example, if we had a rule like `for<T> { (T: Clone) :- (T: Copy) }`, then `T` would be represented as a `BoundVar(0)` (as the `for` is the innermost binder)."],["DynTy","A \"DynTy\" represents a trait object (`dyn Trait`). Trait objects are conceptually very related to an \"existential type\" of the form `exists<T> { T: Trait }` (another exaple of such type is `impl Trait`). `DynTy` represents the bounds on that type."],["Environment","The set of assumptions we've made so far, and the current number of universal (forall) quantifiers we're within."],["EqGoal",""],["Floundered","Error type for the `UnificationOps::program_clauses` method -- indicates that the complete set of program clauses for this goal cannot be enumerated."],["Fn","for<'a...'z> X -- all binders are instantiated at once, and we use deBruijn indices within `self.ty`"],["FnDefId",""],["GenericArg",""],["Goal","A general goal; this is the full range of questions you can pose to Chalk."],["Goals","A list of goals."],["ImplId",""],["InEnvironment",""],["InferenceVar",""],["Lifetime",""],["LifetimeOutlives",""],["NoSolution","Indicates that the attempted operation has \"no solution\" -- i.e., cannot be performed."],["Normalize","Proves that the given type alias normalizes to the given type. A projection `T::Foo` normalizes to the type `U` if we can match it to an impl and that impl has a `type Foo = V` where `U = V`."],["OpaqueTy",""],["OpaqueTyId",""],["PlaceholderIndex","Index of an universally quantified parameter in the environment. Two indexes are required, the one of the universe itself and the relative index inside the universe."],["ProgramClause",""],["ProgramClauseData",""],["ProgramClauseImplication","Represents one clause of the form `consequence :- conditions` where `conditions = cond_1 && cond_2 && ...` is the conjunction of the individual conditions."],["ProgramClauses",""],["ProjectionTy",""],["QuantifiedWhereClauses",""],["SubstFolder",""],["Substitution","A mapping of inference variables to instantiations thereof."],["TraitId","The id of a trait definition; could be used to load the trait datum by invoking the [`trait_datum`] method."],["TraitRef",""],["Ty",""],["UCanonical","A \"universe canonical\" value. This is a wrapper around a `Canonical`, indicating that the universes within have been \"renumbered\" to start from 0 and collapse unimportant distinctions."],["UniverseIndex","An universe index is how a universally quantified parameter is represented when it's binder is moved into the environment. An example chain of transformations would be: `forall<T> { Goal(T) }` (syntactical representation) `forall { Goal(?0) }` (used a DeBruijn index) `Goal(!U1)` (the quantifier was moved to the environment and replaced with a universe index) See https://rustc-dev-guide.rust-lang.org/borrow_check/region_inference.html#placeholders-and-universes for more."],["UniverseMap","Maps the universes found in the `u_canonicalize` result (the \"canonical\" universes) to the universes found in the original value (and vice versa). When used as a folder -- i.e., from outside this module -- converts from \"canonical\" universes to the original (but see the `UMapToCanonical` folder)."],["VariableKinds",""],["WithKind",""]],"trait":[["AsParameters",""],["ToGenericArg",""]],"type":[["CanonicalVarKind",""],["Fallible","Many of our internal operations (e.g., unification) are an attempt to perform some operation which may not complete."],["QuantifiedWhereClause",""]]});