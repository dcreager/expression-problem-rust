// -*- coding: utf-8 -*-
// ------------------------------------------------------------------------------------------------
// Copyright © 2018, Douglas Creager.
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

// ------------------------------------------------------------------------------------------------
// Data types

pub auto trait NotEq {}
impl<X> !NotEq for (X, X) {}

pub struct IntegerLiteral {
    pub value: i64,
}

pub fn integer_literal<E: From<IntegerLiteral>>(value: i64) -> E {
    E::from(IntegerLiteral { value })
}

pub struct Add<E> {
    pub lhs: Box<E>,
    pub rhs: Box<E>,
}

pub fn add<E: From<Add<E>>>(lhs: E, rhs: E) -> E {
    E::from(Add {
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
    })
}

// ------------------------------------------------------------------------------------------------
// Open sums

pub enum CoproductPair<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> CoproductPair<L, R> {
    pub fn new_left(left: L) -> CoproductPair<L, R> {
        CoproductPair::Left(left)
    }
    pub fn new_right(right: R) -> CoproductPair<L, R> {
        CoproductPair::Right(right)
    }
}

impl<L, R> From<L> for CoproductPair<L, R> {
    fn from(left: L) -> CoproductPair<L, R> {
        CoproductPair::Left(left)
    }
}

impl<X, L, R> From<X> for CoproductPair<L, R>
where
    R: From<X>,
    (X, L): NotEq,
    (X, Self): NotEq,
{
    fn from(x: X) -> CoproductPair<L, R> {
        CoproductPair::Right(R::from(x))
    }
}

macro_rules! Coproduct {
    { $A:ty, $B:ty } => { CoproductPair<$A, $B> };
    { $A:ty, $($B:ty),+ } => { CoproductPair<$A, Coproduct![$($B),+]> };
}

// ------------------------------------------------------------------------------------------------
// Evaluate

pub trait Result: From<i64> + std::ops::Add<Output = Self> {}
impl Result for i64 {}

pub trait Evaluate {
    fn evaluate<V: Result>(&self) -> V;
}

impl<L, R> Evaluate for CoproductPair<L, R>
where
    L: Evaluate,
    R: Evaluate,
{
    fn evaluate<V: Result>(&self) -> V {
        match self {
            CoproductPair::Left(l) => l.evaluate(),
            CoproductPair::Right(r) => r.evaluate(),
        }
    }
}

impl Evaluate for IntegerLiteral {
    fn evaluate<V: Result>(&self) -> V {
        V::from(self.value)
    }
}

impl<E> Evaluate for Add<E>
where
    E: Evaluate,
{
    fn evaluate<V: Result>(&self) -> V {
        self.lhs.evaluate::<V>() + self.rhs.evaluate::<V>()
    }
}

// ------------------------------------------------------------------------------------------------
// Expr

pub type Sig<E> = Coproduct![IntegerLiteral, Add<E>];
pub struct Expr(Sig<Expr>);

impl<X> From<X> for Expr
where
    Sig<Expr>: From<X>,
{
    fn from(x: X) -> Expr {
        Expr(Sig::<Expr>::from(x))
    }
}

impl Evaluate for Expr {
    fn evaluate<V: Result>(&self) -> V {
        self.0.evaluate()
    }
}

// ------------------------------------------------------------------------------------------------
// Desugaring

/*
pub struct Subtract<L, R> {
    pub lhs: Box<L>,
    pub rhs: Box<R>,
}

impl<L, R> Subtract<L, R> {
    pub fn new(lhs: L, rhs: R) -> Subtract<L, R> {
        Subtract { lhs: Box::new(lhs), rhs: Box::new(rhs) }
    }
}

impl<L, R> Evaluate<i64> for Subtract<L, R> where L: Evaluate<i64>, R: Evaluate<i64>
{
    fn evaluate(&self) -> i64 {
        self.lhs.evaluate() - self.rhs.evaluate()
    }
}
*/

// ------------------------------------------------------------------------------------------------
// Evaluate

#[cfg(test)]
mod eval_tests {
    use super::*;

    #[test]
    fn can_evaluate_integer_literal() {
        let one = IntegerLiteral { value: 1 };
        assert_eq!(one.evaluate::<i64>(), 1);
    }

    #[test]
    fn can_evaluate_add() {
        let one = IntegerLiteral { value: 1 };
        let two = IntegerLiteral { value: 2 };
        let add = Add {
            lhs: Box::new(one),
            rhs: Box::new(two),
        };
        assert_eq!(add.evaluate::<i64>(), 3);
    }

    /*
    #[test]
    fn can_evaluate_add3() {
        let one = IntegerLiteral { value: 1 };
        let two = IntegerLiteral { value: 2 };
        let three = IntegerLiteral { value: 3 };
        let add = Add {
            lhs: Box::new(one),
            rhs: Box::new(Add {
                lhs: Box::new(two),
                rhs: Box::new(three),
            }),
        };
        assert_eq!(add.evaluate::<i64>(), 6);
    }
    */

    #[test]
    fn can_evaluate_expr_integer_literal() {
        let one: Expr = integer_literal(1);
        assert_eq!(one.evaluate::<i64>(), 1);
    }

    #[test]
    fn can_evaluate_expr_add() {
        let add: Expr = add(integer_literal(1), integer_literal(2));
        assert_eq!(add.evaluate::<i64>(), 3);
    }

    #[test]
    fn can_evaluate_expr_add3() {
        let add: Expr = add(
            integer_literal(1),
            add(integer_literal(2), integer_literal(3)),
        );
        assert_eq!(add.evaluate::<i64>(), 6);
    }

    /*
    #[test]
    fn can_evaluate_subtract() {
        let one = new_integer_literal(1);
        let two = new_integer_literal(2);
        let sub = Subtract::new(two, one);
        assert_eq!(evaluate(sub), 1);
    }
    */
}
