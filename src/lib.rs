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

use std::marker::PhantomData;

// ------------------------------------------------------------------------------------------------
// Data types

pub struct Constant<E> {
    pub value: u64,
    phantom: PhantomData<E>,
}

impl<E> Constant<E> {
    fn new(value: u64) -> Constant<E> {
        Constant {
            value,
            phantom: PhantomData,
        }
    }
}

pub fn constant<E>(value: u64) -> Constant<E> {
    Constant::new(value)
}

impl<E> From<u64> for Constant<E> {
    fn from(value: u64) -> Constant<E> {
        constant(value)
    }
}

pub struct Add<L, R> {
    pub lhs: Box<L>,
    pub rhs: Box<R>,
}

impl<L, R> Add<L, R> {
    pub fn new(lhs: L, rhs: R) -> Add<L, R> {
        Add {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }
}

/*
impl<E> std::ops::Add for E
where
    E: From<Add<E, E>>,
{
    fn add(self, other: Self) -> Self {
        Add::new(self, other)
    }
}
*/

// ------------------------------------------------------------------------------------------------

pub enum Coproduct<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Coproduct<L, R> {
    pub fn new_left(left: L) -> Coproduct<L, R> {
        Coproduct::Left(left)
    }
    pub fn new_right(right: R) -> Coproduct<L, R> {
        Coproduct::Right(right)
    }
}

/*
impl<L, R> From<L> for Coproduct<L, R> {
    fn from(left: L) -> Coproduct<L, R> {
        Coproduct::new_left(left)
    }
}

impl<L, R> From<R> for Coproduct<L, R> {
    fn from(right: R) -> Coproduct<L, R> {
        Coproduct::new_right(right)
    }
}
*/

// ------------------------------------------------------------------------------------------------
// Evaluate

pub trait Result: From<u64> + std::ops::Add<Output = Self> {}
impl Result for u64 {}

pub trait Evaluate {
    fn evaluate<V: Result>(&self) -> V;
}

impl<L, R> Evaluate for Coproduct<L, R>
where
    L: Evaluate,
    R: Evaluate,
{
    fn evaluate<V: Result>(&self) -> V {
        match self {
            Coproduct::Left(l) => l.evaluate(),
            Coproduct::Right(r) => r.evaluate(),
        }
    }
}

impl<E> Evaluate for Constant<E> {
    fn evaluate<V: Result>(&self) -> V {
        V::from(self.value)
    }
}

impl<L, R> Evaluate for Add<L, R>
where
    L: Evaluate,
    R: Evaluate,
{
    fn evaluate<V: Result>(&self) -> V {
        self.lhs.evaluate::<V>() + self.rhs.evaluate::<V>()
    }
}

// ------------------------------------------------------------------------------------------------
// Expr

pub type Sig<E> = Coproduct<Constant<E>, Add<E, E>>;

pub struct Expr {
    pub sig: Sig<Expr>,
}

impl Expr {
    pub fn new(sig: Sig<Expr>) -> Expr {
        Expr { sig }
    }
}

impl Evaluate for Expr {
    fn evaluate<V: Result>(&self) -> V {
        self.sig.evaluate()
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
        let one: Constant<u64> = constant(1);
        assert_eq!(one.evaluate::<u64>(), 1);
    }

    #[test]
    fn can_evaluate_add() {
        let one: Constant<u64> = constant(1);
        let two: Constant<u64> = constant(2);
        let add = Add::new(one, two);
        assert_eq!(add.evaluate::<u64>(), 3);
    }

    #[test]
    fn can_evaluate_add3() {
        let one: Constant<u64> = constant(1);
        let two: Constant<u64> = constant(2);
        let three: Constant<u64> = constant(3);
        let add = Add::new(one, Add::new(two, three));
        assert_eq!(add.evaluate::<u64>(), 6);
    }

    #[test]
    fn can_evaluate_expr_constant() {
        let one = Expr::new(Coproduct::new_left(constant(1)));
        assert_eq!(one.evaluate::<u64>(), 1);
    }

    #[test]
    fn can_evaluate_expr_add() {
        let add: Expr = Expr::from(Add::new(constant(1), constant(2)));
        assert_eq!(add.evaluate::<u64>(), 3);
    }

    #[test]
    fn can_evaluate_expr_add3() {
        let one = Expr::new(Coproduct::new_left(constant(1)));
        let two = Expr::new(Coproduct::new_left(constant(2)));
        let three = Expr::new(Coproduct::new_left(constant(3)));
        let add = Expr::new(Coproduct::new_right(Add::new(
            one,
            Expr::new(Coproduct::new_right(Add::new(two, three))),
        )));
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
