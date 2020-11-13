//! Tests that don't fit a single category

use super::*;

// Regression test for rust-lang/chalk#111
#[test]
fn futures_ambiguity() {
    test! {
        program {
            struct Result<T, E> { }

            trait Future {
                type Output;
            }

            trait FutureResult
                where
                Self: Future<Output = Result<
                    <Self as FutureResult>::Item,
                    <Self as FutureResult>::Error
                >>
            {
                type Item;
                type Error;
            }

            impl<T, I, E> FutureResult for T
                where
                T: Future<Output = Result<I, E>>
            {
                type Item = I;
                type Error = E;
            }
        }

        goal {
            forall<T> { if (T: FutureResult) { exists<I, E> { T: Future<Output = Result<I, E>> } } }
        } yields {
            r"Unique; substitution [?0 := (FutureResult::Item)<!1_0>, ?1 := (FutureResult::Error)<!1_0>], lifetime constraints []"
        }
    }
}

#[test]
fn basic() {
    test! {
        program {
            trait Sized { }

            struct Foo { }
            impl Sized for Foo { }
        }

        goal {
            forall<T> { if (T: Sized) { T: Sized } }
        } yields_all[SolverChoice::slg(10, None)] {
            "substitution [], lifetime constraints []"
        }
    }
}

/// Make sure we don't get a stack overflow or other badness for this
/// test from scalexm.
#[test]
fn subgoal_abstraction() {
    test! {
        program {
            trait Foo { }
            struct Box<T> { }
            impl<T> Foo for T where Box<T>: Foo { }
        }

        goal {
            exists<T> { T: Foo }
        } yields_first[SolverChoice::slg(50, None)] {
            "Floundered"
        }
    }
}

#[test]
fn flounder() {
    test! {
        program {
            trait A { }

            struct Vec<T> { }
            impl<T> A for Vec<T> { }
        }

        goal {
            exists<T> { not { T: A } }
        } yields_first[SolverChoice::slg(10, None)] {
            "Floundered"
        }
    }
}

// Test that, when solving `?T: Sized`, we only wind up pulling a few
// answers before we stop.
// Also tests that we search breadth-first.
#[test]
fn only_draw_so_many() {
    test! {
        program {
            trait Sized { }

            struct Vec<T> { }
            impl<T> Sized for Vec<T> where T: Sized { }

            struct Foo { }
            impl Sized for Foo { }


            struct Slice<T> { }
            impl<T> Sized for Slice<T> where T: Sized { }
        }

        goal {
            exists<T> { T: Sized }
        } yields_first[SolverChoice::slg(10, None)] {
            "substitution [?0 := Foo], lifetime constraints []",
            "substitution [?0 := Slice<Foo>], lifetime constraints []",
            "substitution [?0 := Vec<Foo>], lifetime constraints []",
            "substitution [?0 := Slice<Slice<Foo>>], lifetime constraints []",
            "substitution [?0 := Vec<Slice<Foo>>], lifetime constraints []"
        }

        goal {
            exists<T> { T: Sized }
        } yields[SolverChoice::slg(10, Some(2))] {
            "Ambiguous; no inference guidance"
        } yields[SolverChoice::recursive_default()] {
            "Ambiguous; no inference guidance"
        }
    }
}

#[test]
fn only_draw_so_many_blow_up() {
    test! {
        program {
            trait Sized { }
            trait Foo { }

            struct Vec<T> { }
            impl<T> Sized for Vec<T> where T: Sized { }
            impl<T> Foo for Vec<T> where T: Sized { }

            struct Alice { }
            impl Sized for Alice { }

            struct Slice<T> { }
            impl<T> Sized for Slice<T> where T: Sized { }
        }

        goal {
            exists<T> { T: Foo }
        } yields[SolverChoice::slg(10, Some(2))] {
            "Ambiguous; definite substitution for<?U0> { [?0 := Vec<^0.0>] }"
        } yields[SolverChoice::recursive_default()] {
            "Ambiguous; definite substitution for<?U0> { [?0 := Vec<^0.0>] }"
        }
    }
}

#[test]
fn subgoal_cycle_uninhabited() {
    test! {
        program {
            trait Foo { }
            struct Box<T> { }
            struct Vec<T> { }
            struct Alice { }
            impl<T> Foo for Box<T> where Box<Vec<T>>: Foo { }
        }

        // Infinite recursion -> we flounder
        // Still return the necessary substitution T = Box<..>
        goal {
            exists<T> { T: Foo }
        } yields_first[SolverChoice::slg(2, None)] {
            "Ambiguous(for<?U0> { substitution [?0 := Box<^0.0>], lifetime constraints [] })"
        }

        // Unsurprisingly, applying negation also flounders.
        goal {
            not { exists<T> { T: Foo } }
        } yields_first[SolverChoice::slg(2, None)] {
            "Floundered"
        }

        // Equivalent to the previous.
        goal {
            forall<T> { not { T: Foo } }
        } yields_first[SolverChoice::slg(2, None)] {
            "Floundered"
        }

        // However, if we come across a negative goal that exceeds our
        // size threshold, we have a problem.
        goal {
            exists<T> { T = Vec<Alice>, not { Vec<Vec<T>>: Foo } }
        } yields_first[SolverChoice::slg(2, None)] {
            "Ambiguous(substitution [?0 := Vec<Alice>], lifetime constraints [])"
        }

        // Same query with larger threshold works fine, though.
        goal {
            exists<T> { T = Vec<Alice>, not { Vec<Vec<T>>: Foo } }
        } yields_all[SolverChoice::slg(4, None)] {
            "substitution [?0 := Vec<Alice>], lifetime constraints []"
        }

        // Here, due to the hypothesis, there does indeed exist a suitable T, `U`.
        goal {
            forall<U> { if (U: Foo) { exists<T> { T: Foo } } }
        } yields_first[SolverChoice::slg(2, None)] {
            "substitution [?0 := !1_0], lifetime constraints []",
            "Ambiguous(for<?U1> { substitution [?0 := Box<^0.0>], lifetime constraints [] })"
        }
    }
}

#[test]
fn subgoal_cycle_inhabited() {
    test! {
        program {
            trait Foo { }
            struct Box<T> { }
            struct Vec<T> { }
            struct Alice { }
            impl<T> Foo for Box<T> where Box<Vec<T>>: Foo { }
            impl Foo for Alice { }
        }

        // Exceeds size threshold -> flounder
        // Still return necessary substitution T = Box<..>
        goal {
            exists<T> { T: Foo }
        } yields_first[SolverChoice::slg(3, None)] {
            "substitution [?0 := Alice], lifetime constraints []",
            "Ambiguous(for<?U0> { substitution [?0 := Box<^0.0>], lifetime constraints [] })"
        }
    }
}

#[test]
fn basic_region_constraint_from_positive_impl() {
    test! {
        program {
            trait Foo { }
            struct Ref<'a, 'b, T> { }
            struct Bar { }
            impl<'x, T> Foo for Ref<'x, 'x, T> { }
        }

        goal {
            forall<'a, 'b, T> { Ref<'a, 'b, T>: Foo }
        } yields_all[SolverChoice::slg(3, None)] {
            "substitution [], lifetime constraints [\
            InEnvironment { environment: Env([]), goal: '!1_0: '!1_1 }, \
            InEnvironment { environment: Env([]), goal: '!1_1: '!1_0 } \
            ]"
        }
    }
}

#[test]
#[allow(non_snake_case)]
fn example_2_1_EWFS() {
    test! {
        program {
            trait Edge<B> { }
            trait TransitiveClosure<B> { }
            struct a { }
            struct b { }
            struct c { }

            forall<> { a: Edge<b> }
            forall<> { b: Edge<c> }
            forall<> { b: Edge<a> }
            forall<X, Y> { X: TransitiveClosure<Y> if X: Edge<Y> }
            forall<X, Y, Z> { X: TransitiveClosure<Y> if X: Edge<Z>, Z: TransitiveClosure<Y> }
        }

        goal {
            exists<V> { a: TransitiveClosure<V> }
        } yields_all[SolverChoice::slg(3, None)] {
            "substitution [?0 := b], lifetime constraints []",
            "substitution [?0 := c], lifetime constraints []",
            "substitution [?0 := a], lifetime constraints []"
        }
    }
}

/// Test (along with the other `cached_answers` tests) that the
/// ordering in which we we encounter clauses doesn't affect the final
/// set of answers we get. In particular, all of them should get 5
/// answers, but in Ye Olde Days Of Yore there were sometimes bugs
/// that came up when replaying tabled answers that led to fewer
/// answers being produced.
///
/// This test is also a test for ANSWER ABSTRACTION: the only reason
/// we get 5 answers is because of the max size of 2.
#[test]
fn cached_answers_1() {
    test! {
        program {
            trait Sour { }
            struct Lemon { }
            struct Vinegar { }
            struct HotSauce<T> { }

            // Use explicit program clauses here rather than traits
            // and impls to avoid hashmaps and other things that
            // sometimes alter the final order of the program clauses:
            forall<> { Lemon: Sour }
            forall<> { Vinegar: Sour }
            forall<T> { HotSauce<T>: Sour if T: Sour }
        }

        goal {
            exists<T> { T: Sour }
        } yields_first[SolverChoice::slg(2, None)] {
            "substitution [?0 := Lemon], lifetime constraints []",
            "substitution [?0 := Vinegar], lifetime constraints []",
            "substitution [?0 := HotSauce<Lemon>], lifetime constraints []",
            "substitution [?0 := HotSauce<Vinegar>], lifetime constraints []",
            "Floundered"
        }
    }
}

/// See `cached_answers_1`.
#[test]
fn cached_answers_2() {
    test! {
        program {
            trait Sour { }
            struct Lemon { }
            struct Vinegar { }
            struct HotSauce<T> { }

            forall<T> { HotSauce<T>: Sour if T: Sour }
            forall<> { Lemon: Sour }
            forall<> { Vinegar: Sour }
        }

        goal {
            exists<T> { T: Sour }
        } yields_first[SolverChoice::slg(2, None)] {
            "substitution [?0 := Lemon], lifetime constraints []",
            "substitution [?0 := Vinegar], lifetime constraints []",
            "substitution [?0 := HotSauce<Lemon>], lifetime constraints []",
            "substitution [?0 := HotSauce<Vinegar>], lifetime constraints []",
            "Floundered"
        }
    }
}

/// See `cached_answers_1`.
#[test]
fn cached_answers_3() {
    test! {
        program {
            trait Sour { }
            struct Lemon { }
            struct Vinegar { }
            struct HotSauce<T> { }

            forall<> { Lemon: Sour }
            forall<T> { HotSauce<T>: Sour if T: Sour }
            forall<> { Vinegar: Sour }
        }

        goal {
            exists<T> { T: Sour }
        } yields_first[SolverChoice::slg(2, None)] {
            "substitution [?0 := Lemon], lifetime constraints []",
            "substitution [?0 := HotSauce<Lemon>], lifetime constraints []",
            "substitution [?0 := Vinegar], lifetime constraints []",
            "Floundered"
        }
    }
}

#[test]
fn non_enumerable_traits_direct() {
    test! {
        program {
            struct Foo { }
            struct Bar { }

            #[non_enumerable]
            trait NonEnumerable { }
            impl NonEnumerable for Foo { }
            impl NonEnumerable for Bar { }

            trait Enumerable { }
            impl Enumerable for Foo { }
            impl Enumerable for Bar { }
        }

        goal {
            exists<A> { A: NonEnumerable }
        } yields_first[SolverChoice::slg(3, None)] {
            "Floundered"
        }

        goal {
            exists<A> { A: Enumerable }
        } yields_all[SolverChoice::slg(3, None)] {
            "substitution [?0 := Foo], lifetime constraints []",
            "substitution [?0 := Bar], lifetime constraints []"
        }

        goal {
            Foo: NonEnumerable
        } yields_all[SolverChoice::slg(3, None)] {
            "substitution [], lifetime constraints []"
        }
    }
}

#[test]
fn non_enumerable_traits_indirect() {
    test! {
        program {
            struct Foo { }
            struct Bar { }

            #[non_enumerable]
            trait NonEnumerable { }
            impl NonEnumerable for Foo { }
            impl NonEnumerable for Bar { }

            trait Debug { }
            impl<T> Debug for T where T: NonEnumerable { }
        }

        goal {
            exists<A> { A: Debug }
        } yields_first[SolverChoice::slg(3, None)] {
            "Floundered"
        }
    }
}

#[test]
fn non_enumerable_traits_double() {
    test! {
        program {
            struct Foo { }
            struct Bar { }

            #[non_enumerable]
            trait NonEnumerable1 { }
            impl NonEnumerable1 for Foo { }
            impl NonEnumerable1 for Bar { }

            #[non_enumerable]
            trait NonEnumerable2 { }
            impl NonEnumerable2 for Foo { }
            impl NonEnumerable2 for Bar { }

            trait Debug { }
            impl<T> Debug for T where T: NonEnumerable1, T: NonEnumerable2  { }
        }

        goal {
            exists<A> { A: Debug }
        } yields_first[SolverChoice::slg(3, None)] {
            "Floundered"
        }
    }
}

#[test]
fn non_enumerable_traits_reorder() {
    test! {
        program {
            struct Foo { }
            struct Bar { }

            #[non_enumerable]
            trait NonEnumerable { }
            impl NonEnumerable for Foo { }
            impl NonEnumerable for Bar { }

            trait Enumerable { }
            impl Enumerable for Foo { }

            // In this test, we first try to solve to solve `T:
            // NonEnumerable` but then we discover it's
            // non-enumerable, and so we push it off for later. Then
            // we try to solve the `T: Enumerable` trait.

            trait Debug1 { }
            impl<T> Debug1 for T where T: Enumerable, T: NonEnumerable { }

            trait Debug2 { }
            impl<T> Debug2 for T where T: NonEnumerable, T: Enumerable { }
        }

        goal {
            exists<A> { A: Debug1 }
        } yields_all[SolverChoice::slg(3, None)] {
            "substitution [?0 := Foo], lifetime constraints []"
        }


        goal {
            exists<A> { A: Debug2 }
        } yields_all[SolverChoice::slg(3, None)] {
            "substitution [?0 := Foo], lifetime constraints []"
        }
    }
}

#[test]
fn builtin_impl_enumeration() {
    test! {
        program {
            #[lang(copy)]
            trait Copy { }

            #[lang(sized)]
            trait Sized { }

            #[lang(clone)]
            trait Clone { }

            impl Copy for u8 {}
            impl Clone for u8 {}
        }

        goal {
            exists<T> { T: Copy }
        } yields {
            "Ambiguous; no inference guidance"
        }

        goal {
            exists<T> { T: Clone }
        } yields {
            "Ambiguous; no inference guidance"
        }

        goal {
            exists<T> { T: Sized }
        } yields {
            "Ambiguous; no inference guidance"
        }
    }
}

/// Don't return definite guidance if we flounder after finding one solution.
#[test]
fn flounder_ambiguous() {
    test! {
        program {
            trait IntoIterator { }
            #[non_enumerable]
            trait OtherTrait { }

            struct Ref<T> { }
            struct A { }

            impl IntoIterator for Ref<A> { }
            impl<T> IntoIterator for Ref<T> where T: OtherTrait { }
        }

        goal {
            exists<T> { Ref<T>: IntoIterator }
        } yields {
            "Ambiguous; no inference guidance"
        }
    }
}

/// Don't return definite guidance if we are able to merge two solutions and the
/// third one matches that as well (the fourth may not).
#[test]
fn normalize_ambiguous() {
    test! {
        program {
            trait IntoIterator { type Item; }

            struct Ref<T> { }
            struct A { }
            struct B { }
            struct C { }

            struct D { }

            impl IntoIterator for Ref<A> { type Item = Ref<A>; }
            impl IntoIterator for Ref<B> { type Item = Ref<B>; }
            impl IntoIterator for Ref<C> { type Item = Ref<C>; }
            impl IntoIterator for Ref<D> { type Item = D; }
        }

        goal {
            exists<T, U> {
                Normalize(<Ref<T> as IntoIterator>::Item -> U)
            }
        } yields {
            "Ambiguous; no inference guidance"
        }
    }
}

#[test]
fn lifetime_outlives_constraints() {
    test! {
        program {
            trait Foo<'a, 'b> where 'a: 'b {}
            struct Bar {}

            impl<'a, 'b> Foo<'a, 'b> for Bar where 'a: 'b {}
        }

        goal {
            exists<'a, 'b> {
                Bar: Foo<'a, 'b>
            }
        } yields {
            "Unique; for<?U0,?U0> { substitution [?0 := '^0.0, ?1 := '^0.1], lifetime constraints [InEnvironment { environment: Env([]), goal: '^0.0: '^0.1 }] }"
        }

        goal {
            forall<'a> {
                exists<'b> {
                    Bar: Foo<'a, 'b>
                }
            }
        } yields {
            "Unique; for<?U1> { substitution [?0 := '^0.0], lifetime constraints [InEnvironment { environment: Env([]), goal: '!1_0: '^0.0 }] }"
        }
    }
}

#[test]
fn type_outlives_constraints() {
    test! {
        program {
            trait Foo<'a, T> where T: 'a {}
            struct Bar {}
            impl<'a, T> Foo<'a, T> for Bar where T: 'a {}
        }

        goal {
            exists<'a, T> {
                Bar: Foo<'a, T>
            }
        } yields {
            "Unique; for<?U0,?U0> { substitution [?0 := '^0.0, ?1 := ^0.1], lifetime constraints [InEnvironment { environment: Env([]), goal: ^0.1: '^0.0 }] }"
        }

        goal {
            forall<T> {
                exists<'a> {
                    Bar: Foo<'a, T>
                }
            }
        } yields {
            "Unique; for<?U1> { substitution [?0 := '^0.0], lifetime constraints [InEnvironment { environment: Env([]), goal: !1_0: '^0.0 }] }"
        }
    }
}

/// Example of fundamental ambiguity in the recursive solver, used in the
/// recursive solver book documentation.
#[test]
fn not_really_ambig() {
    test! {
        program {
            struct Vec<T> { }

            trait A { }
            trait B { }

            impl<T> A for Vec<T> where T: A, T: B { }

            impl A for u32 { }
            impl B for u32 { }

            impl A for i32 { }
            impl B for i8 { }
        }

        goal {
            exists<T> { Vec<T>: A }
        } yields[SolverChoice::slg_default()] {
            "Unique; substitution [?0 := Uint(U32)], lifetime constraints []"
        } yields[SolverChoice::recursive_default()] {
            "Ambiguous; no inference guidance"
        }
    }
}

#[test]
fn canonicalization_regression() {
    test! {
        program {
            trait ForAny<X> {}
            trait ForSame<X> {}

            impl<X, Y> ForAny<X> for Y {}
            impl<X> ForSame<X> for X {}
        }

        goal {
            forall<A> {
                forall<B> {
                    exists<E> {
                        A: ForAny<E>,
                        B: ForSame<E>
                    }
                }
            }
        } yields {
            "Unique; substitution [?0 := !2_0], lifetime constraints []"
        }
    }
}

#[test]
fn empty_definite_guidance() {
    test! {
        disable_coherence;
        program {
            trait Trait<T> {}

            struct S<'a> {}
            struct A {}

            impl<'a> Trait<S<'a>> for A {}
            impl<'a> Trait<S<'a>> for A where A: 'a {}

            trait OtherTrait<'a> {}
            impl<'a> OtherTrait<'a> for A where A: Trait<S<'a>> {}
        }

        goal {
            forall<'a> {
                A: OtherTrait<'a>
            }
            // the program fails coherence, so which answer we get here exactly
            // isn't that important -- this is mainly a regression test for a
            // recursive solver infinite loop.
        } yields[SolverChoice::slg_default()] {
            "Unique"
        } yields[SolverChoice::recursive_default()] {
            "Ambiguous"
        }
    }
}
