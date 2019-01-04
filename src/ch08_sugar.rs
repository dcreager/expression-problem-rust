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

//! Let's add a new kind of term that's just syntactic sugar for expressions that you can already
//! express.  We can define evaluation rules for expressions that include the new terms, and also
//! an operation that desugars expressions into ones that don't include the new terms — and the
//! Rust types let us **enforce** that the new terms don't exist anymore!

use crate::ch02_open_sum::*;
use crate::ch04_smart_constructors::*;
use crate::ch05a_multiplication::*;
use crate::ch07b_generic_evaluation::*;
use crate::ch99_functors::*;

/// To negate a number, we could just multiply by -1!
pub struct Negate<E> {
    pub value: Box<E>,
}

impl<'a, A, B, F> Functor<'a, A, B, F> for Negate<A>
where
    A: 'a,
    F: Fn(&'a A) -> B,
{
    type Output = Negate<B>;
    fn fmap(&'a self, f: F) -> Negate<B> {
        Add {
            value: Box::new(f(self.value.as_ref())),
        }
    }
}

// And a smart constructor
pub fn negate<E: From<Negate<E>>>(value: E) -> E {
    E::from(Negate {
        value: Box::new(value),
    })
}

// Now we create an expression type that can include negation.

pub type NegateSig<E> = Sum<Negate<E>, MultSig<E>>;
pub struct NegateExpr(pub NegateSig<NegateExpr>);

impl<'a, B, F> Functor<'a, NegateExpr, B, F> for NegateExpr
where
    NegateExpr: 'a,
    F: Fn(&'a NegateExpr) -> B,
{
    type Output = NegateSig<B>;
    fn fmap(&'a self, f: F) -> NegateSig<B> {
        self.0.fmap(f)
    }
}

// We can evaluate NegateExprs directly

impl EvalAlgebra for Negate<i64> {
    fn eval(&self) -> i64 {
        -self.value
    }
}

// And we can desugar the expression to remove the new Negate term.

trait Desugar<D> {
    fn desugar(self) -> D;
}

impl<D, L, R> Desugar<D> for Sum<L, R>
where
    L: Desugar<D>,
    R: Desugar<D>,
{
    fn desugar(self) -> D {
        match self {
            Sum::Left(left) => left.desugar(),
            Sum::Right(right) => right.desugar(),
        }
    }
}

impl<D> Desugar<D> for IntegerLiteral
where
    D: From<IntegerLiteral>,
{
    fn desugar(self) -> D {
        self.into()
    }
}

impl<D, S> Desugar<D> for Add<S>
where
    D: From<Add<D>>,
    S: Desugar<D>,
{
    fn desugar(self) -> D {
        add(self.lhs.desugar(), self.rhs.desugar())
    }
}

impl<D, S> Desugar<D> for Multiply<S>
where
    D: From<Multiply<D>>,
    S: Desugar<D>,
{
    fn desugar(self) -> D {
        multiply(self.lhs.desugar(), self.rhs.desugar())
    }
}

impl<D, S> Desugar<D> for Negate<S>
where
    D: From<IntegerLiteral> + From<Multiply<D>>,
    S: Desugar<D>,
{
    fn desugar(self) -> D {
        multiply(integer_literal(-1), (*self.value).desugar())
    }
}

impl<D> Desugar<D> for NegateExpr
where
    D: From<IntegerLiteral> + From<Add<D>> + From<Multiply<D>>,
{
    fn desugar(self) -> D {
        self.0.desugar()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_evaluate_negation() {
        let sugared: NegateExpr = negate(add(integer_literal(118), integer_literal(1219)));
        assert_eq!((&sugared as &Evaluate<i64>).evaluate(), -1337);
        assert_eq!(eval(&sugared), -1337);
    }

    #[test]
    fn can_desugar() {
        let sugared: NegateExpr = negate(add(integer_literal(118), integer_literal(1219)));
        let desugared: MultExpr = sugared.desugar();
        assert_eq!(format!("{}", desugared), "(-1 * (118 + 1219))");
    }

    #[test]
    fn can_evaluate_desugared() {
        let sugared: NegateExpr = negate(add(integer_literal(118), integer_literal(1219)));
        let desugared: MultExpr = sugared.desugar();
        assert_eq!((&desugared as &Evaluate<i64>).evaluate(), -1337);
        assert_eq!(eval(&desugared), -1337);
    }
}