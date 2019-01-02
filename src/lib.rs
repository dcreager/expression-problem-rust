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

#![feature(optin_builtin_traits, specialization)]

// ------------------------------------------------------------------------------------------------
// Data types

pub auto trait NotEq {}
impl<X> !NotEq for (X, X) {}

pub struct Constant {
    pub value: u64,
}

impl Constant {
    fn new(value: u64) -> Constant {
        Constant { value }
    }
}

pub fn constant<E: From<Constant>>(value: u64) -> E {
    E::from(Constant::new(value))
}

impl From<u64> for Constant {
    fn from(value: u64) -> Constant {
        Constant::new(value)
    }
}

pub struct Add<E> {
    pub lhs: Box<E>,
    pub rhs: Box<E>,
}

impl<E> Add<E> {
    pub fn new(lhs: E, rhs: E) -> Add<E> {
        Add {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }
}

pub fn add<E: From<Add<E>>>(lhs: E, rhs: E) -> E {
    E::from(Add::new(lhs, rhs))
}

// ------------------------------------------------------------------------------------------------

pub struct CoproductSingleton<L> {
    left: L,
}

impl<L> From<L> for CoproductSingleton<L> {
    fn from(left: L) -> CoproductSingleton<L> {
        CoproductSingleton { left }
    }
}

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
    { $A:ty } => { CoproductSingleton<$A> };
    { $A:ty, $($B:ty),+ } => { CoproductPair<$A, Coproduct![$($B),+]> };
}

// ------------------------------------------------------------------------------------------------
// Evaluate

pub trait Result: From<u64> + std::ops::Add<Output = Self> {}
impl Result for u64 {}

pub trait Evaluate {
    fn evaluate<V: Result>(&self) -> V;
}

impl<L> Evaluate for CoproductSingleton<L>
where
    L: Evaluate,
{
    fn evaluate<V: Result>(&self) -> V {
        self.left.evaluate()
    }
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

impl Evaluate for Constant {
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

pub type Sig<E> = Coproduct![Constant, Add<E>];
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

impl<L, R> Evaluate<u64> for Subtract<L, R> where L: Evaluate<u64>, R: Evaluate<u64>
{
    fn evaluate(&self) -> u64 {
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
    fn can_evaluate_constant() {
        let one: Constant = Constant::new(1);
        assert_eq!(one.evaluate::<u64>(), 1);
    }

    #[test]
    fn can_evaluate_add() {
        let one: Constant = Constant::new(1);
        let two: Constant = Constant::new(2);
        let add = Add::new(one, two);
        assert_eq!(add.evaluate::<u64>(), 3);
    }

    /*
    #[test]
    fn can_evaluate_add3() {
        let one: Constant = Constant::new(1);
        let two: Constant = Constant::new(2);
        let three: Constant = Constant::new(3);
        let add = Add::new(one, Add::new(two, three));
        assert_eq!(add.evaluate::<u64>(), 6);
    }
    */

    #[test]
    fn can_evaluate_expr_constant() {
        let one: Expr = constant(1);
        assert_eq!(one.evaluate::<u64>(), 1);
    }

    #[test]
    fn can_evaluate_expr_add() {
        let add: Expr = add(constant(1), constant(2));
        assert_eq!(add.evaluate::<u64>(), 3);
    }

    #[test]
    fn can_evaluate_expr_add3() {
        let add: Expr = add(constant(1), add(constant(2), constant(3)));
        assert_eq!(add.evaluate::<u64>(), 6);
    }

    /*
    #[test]
    fn can_evaluate_subtract() {
        let one = new_constant(1);
        let two = new_constant(2);
        let sub = Subtract::new(two, one);
        assert_eq!(evaluate(sub), 1);
    }
    */
}
