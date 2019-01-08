// -*- coding: utf-8 -*-
// ------------------------------------------------------------------------------------------------
// Copyright © 2019, Douglas Creager.
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except
// in compliance with the License.  You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software distributed under the
// License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either
// express or implied.  See the License for the specific language governing permissions and
// limitations under the License.
// ------------------------------------------------------------------------------------------------

//! We have a lot of cool features from previous modules, but they require a lot of annoying
//! boilerplate.  Let's try to eliminate as much of that boilerplate as we can, using our new
//! `Expression` trait.

use crate::ch02_open_sum::*;
use crate::ch05a_multiplication::*;
use crate::ch07a_pairs::*;
use crate::ch07c_pair_evaluation::*;
use crate::ch08a_expressions::*;

// Ideally we would be able to reuse EvaluateAny.  It's quite nice!  But as much as we want to, we
// can't create a generic impl for any Expression.  It would look like this:
//
//     impl<V< E> EvaluateAny<V> for E
//     where E: Expression, E::Signature: EvaluateAny<V>
//     {
//         fn evaluate(&self) -> V {
//             self.unwrap().evaluate()
//         }
//     }
//
// That will compile just fine, but the compiler will never actually **use** that impl, because the
// bounds are recursive: to prove that Expr implements EvaluateAny, we have to show that
// Expr::Signature (aka Sig<Expr>) implements EvaluateAny.  To do that, we have to (skipping a
// couple of steps) show that Add<Expr> implements EvaluateAny.  And to do **that**, we have to
// show that Expr implements EvaluateAny — which is what we're trying to prove in the first place.
//
// Doh!
//
// To get around this, we have to use *open recursion*.  All of our EvaluateAny impls assume that
// the subexpression type E also implements EvaluateAny, and calls the evaluate() method directly
// to evaluate subexpressions before combining them together.  (Look add Add<E>'s implementation
// for a prime example.)  Instead of doing that, our new eval() method is going to take in a
// parameter the function that it should use to evaluate subexpressions!  That means that the impls
// don't have to depend on their subexpression types implementing the trait too — which ends up
// breaking the recursive cycle that got us into trouble!
//
// (If you've read Swierstra's paper deeply, what we've done here is fuse together each term type's
// `fmap` and `evalAlgebra` functions into a single `eval` method.)

/// Each term type should implement this trait to define how it should be evaluated.  If the term
/// has any subexpressions, it should use `eval_subexpr` to evaluate them.
pub trait Eval<V, E> {
    fn eval<F>(&self, eval_subexpr: F) -> V
    where
        F: FnMut(&E) -> V;
}

// This impls should all look very much like their EvaluateAny counterparts, but will all calls to
// `foo.evaluate()` being replaced with `eval_subexpr(foo)`.

impl<V, E> Eval<V, E> for IntegerLiteral
where
    V: From<i64>,
{
    fn eval<F>(&self, _eval_subexpr: F) -> V
    where
        F: FnMut(&E) -> V,
    {
        V::from(self.value)
    }
}

impl<V, E> Eval<V, E> for Add<E>
where
    V: std::ops::Add<Output = V>,
{
    fn eval<F>(&self, mut eval_subexpr: F) -> V
    where
        F: FnMut(&E) -> V,
    {
        eval_subexpr(&self.lhs) + eval_subexpr(&self.rhs)
    }
}

impl<V, E> Eval<V, E> for Multiply<E>
where
    V: std::ops::Mul<Output = V>,
{
    fn eval<F>(&self, mut eval_subexpr: F) -> V
    where
        F: FnMut(&E) -> V,
    {
        eval_subexpr(&self.lhs) * eval_subexpr(&self.rhs)
    }
}

impl<V, E> Eval<V, E> for Pair<E>
where
    V: From<(V, V)>,
{
    fn eval<F>(&self, mut eval_subexpr: F) -> V
    where
        F: FnMut(&E) -> V,
    {
        V::from((eval_subexpr(&self.first), eval_subexpr(&self.second)))
    }
}

impl<V, E> Eval<V, E> for First<E>
where
    V: ProjectPair,
{
    fn eval<F>(&self, mut eval_subexpr: F) -> V
    where
        F: FnMut(&E) -> V,
    {
        eval_subexpr(&self.pair).first()
    }
}

impl<V, E> Eval<V, E> for Second<E>
where
    V: ProjectPair,
{
    fn eval<F>(&self, mut eval_subexpr: F) -> V
    where
        F: FnMut(&E) -> V,
    {
        eval_subexpr(&self.pair).second()
    }
}

impl<V, E, L, R> Eval<V, E> for Sum<L, R>
where
    L: Eval<V, E>,
    R: Eval<V, E>,
{
    fn eval<F>(&self, eval_subexpr: F) -> V
    where
        F: FnMut(&E) -> V,
    {
        match self {
            Sum::Left(lhs) => lhs.eval(eval_subexpr),
            Sum::Right(rhs) => rhs.eval(eval_subexpr),
        }
    }
}

// And now, because there are no horrible cycles in our impls anymore, we can define a generic Eval
// impl for *any* Expression type!  And even better, unlike our EvaluateAny impls, these don't need
// to copy all of the constraints from the per-term impls!

impl<V, E> Eval<V, E> for E
where
    E: Expression,
    E::Signature: Eval<V, E>,
{
    fn eval<F>(&self, eval_subexpr: F) -> V
    where
        F: FnMut(&E) -> V,
    {
        self.unwrap().eval(eval_subexpr)
    }
}

// One last bow to tie this together.  We need something that can call these `eval` methods with
// the right recursion function.  The simplest version would look like this:
//
//     pub fn evaluate<V, E>(expr: &E) -> V where E: Eval<V, E> {
//         e.eval(evaluate)
//     }
//
// and you'd call it like this:
//
//     let expr: Expr = /* whatever */;
//     evaluate::<i64, _>(&expr);
//
// Not bad, but we can do better.  The following lets us call the following for *any* Expression
// type:
//
//     let expr: Expr = /* whatever */;
//     expr.evaluate::<i64>();

trait Evaluate: Sized {
    fn evaluate<V>(&self) -> V
    where
        Self: Eval<V, Self>;
}

impl<E> Evaluate for E
where
    E: Sized,
{
    fn evaluate<V>(&self) -> V
    where
        Self: Eval<V, Self>,
    {
        self.eval(|e| e.evaluate())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ch04_smart_constructors::*;

    // Hey look!  All of the test cases work without modification (except for that we're calling
    // our new evaluate method)!

    #[test]
    fn can_evaluate_ugly_expression() {
        let add: Expr = add(integer_literal(118), integer_literal(1219));
        assert_eq!(add.evaluate::<i64>(), 1337);
    }

    #[test]
    fn can_evaluate_nested_expression() {
        // 30000 + 1330 + 7
        let add: Expr = add(
            integer_literal(30000),
            add(integer_literal(1330), integer_literal(7)),
        );
        assert_eq!(add.evaluate::<i64>(), 31337);
    }

    #[test]
    fn can_evaluate_multiplication() {
        let mult: MultExpr = add(
            multiply(integer_literal(80), integer_literal(5)),
            integer_literal(4),
        );
        assert_eq!(mult.evaluate::<i64>(), 404);
    }

    #[test]
    fn can_evaluate_no_add_multiplication() {
        let mult: NoAddExpr = multiply(integer_literal(6), integer_literal(7));
        assert_eq!(mult.evaluate::<i64>(), 42);
    }

    #[test]
    fn can_evaluate_pair() {
        let expr: PairExpr = pair(integer_literal(7), integer_literal(6));
        assert_eq!(
            expr.evaluate::<IntOrPair>(),
            IntOrPair::Pair(Box::new(IntOrPair::Int(7)), Box::new(IntOrPair::Int(6)))
        );
    }

    #[test]
    fn can_evaluate_pair_projection() {
        let expr: PairExpr = first(pair(integer_literal(7), integer_literal(6)));
        assert_eq!(expr.evaluate::<IntOrPair>(), IntOrPair::Int(7));
    }
}
