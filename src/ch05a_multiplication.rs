// -*- coding: utf-8 -*-
// ------------------------------------------------------------------------------------------------
// Copyright © 2018-2019, Douglas Creager.
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

//! Look how easy it is to add a new term!

use crate::ch02_open_sum::*;
use crate::ch03_evaluation::*;

/// First a type for the new term
pub struct Multiply<E> {
    pub lhs: E,
    pub rhs: E,
}

/// Then an evaluation rule for it
impl<E> EvaluateInt for Multiply<E>
where
    // We can only evaluate an addition if we know how to evaluate its subexpressions.
    E: EvaluateInt,
{
    fn evaluate(&self) -> i64 {
        self.lhs.evaluate() * self.rhs.evaluate()
    }
}

/// And a smart constructor
pub fn multiply<E: From<Multiply<Box<E>>>>(lhs: E, rhs: E) -> E {
    E::from(Multiply {
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
    })
}

// And then an expression that can contain it, along with the existing terms.
pub type MultSig<E> = Sum<Multiply<Box<E>>, Sig<E>>;
pub struct MultExpr(pub MultSig<MultExpr>);

impl EvaluateInt for MultExpr {
    fn evaluate(&self) -> i64 {
        self.0.evaluate()
    }
}

impl<X> From<X> for MultExpr
where
    MultSig<MultExpr>: From<X>,
{
    fn from(x: X) -> MultExpr {
        MultExpr(MultSig::<MultExpr>::from(x))
    }
}

// And to show off, we can create an expression that isn't allowed to contain addition!
pub type NoAddSig<E> = Sum<IntegerLiteral, Multiply<Box<E>>>;
pub struct NoAddExpr(pub NoAddSig<NoAddExpr>);

impl EvaluateInt for NoAddExpr {
    fn evaluate(&self) -> i64 {
        self.0.evaluate()
    }
}

impl<X> From<X> for NoAddExpr
where
    NoAddSig<NoAddExpr>: From<X>,
{
    fn from(x: X) -> NoAddExpr {
        NoAddExpr(NoAddSig::<NoAddExpr>::from(x))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ch04_smart_constructors::*;

    #[test]
    fn can_evaluate_multiplication() {
        let mult: MultExpr = add(
            multiply(integer_literal(80), integer_literal(5)),
            integer_literal(4),
        );
        assert_eq!(mult.evaluate(), 404);
    }

    #[test]
    fn can_evaluate_no_add_multiplication() {
        let mult: NoAddExpr = multiply(integer_literal(6), integer_literal(7));
        assert_eq!(mult.evaluate(), 42);
    }
}
