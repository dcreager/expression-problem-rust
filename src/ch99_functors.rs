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

use crate::ch02_open_sum::*;
use crate::ch05a_multiplication::*;

pub trait Function
where
    Self: Sized,
{
    type Input;
    type Output;
    fn call(input: &Self::Input) -> Self::Output;
}

pub trait Functor<Op>
where
    Self: Sized,
    Op: Function,
{
    type Output;
    fn fmap(&self) -> Self::Output;
}

pub fn fmap<Op, X>(x: &X) -> X::Output
where
    X: Functor<Op>,
    Op: Function,
{
    x.fmap()
}

impl<Op> Functor<Op> for IntegerLiteral
where
    Op: Function,
{
    type Output = IntegerLiteral;
    fn fmap(&self) -> IntegerLiteral {
        IntegerLiteral { value: self.value }
    }
}

impl<Op, E> Functor<Op> for Add<E>
where
    Op: Function<Input = E>,
{
    type Output = Add<Op::Output>;
    fn fmap(&self) -> Add<Op::Output> {
        Add {
            lhs: Box::new(Op::call(self.lhs.as_ref())),
            rhs: Box::new(Op::call(self.rhs.as_ref())),
        }
    }
}

impl<Op, E> Functor<Op> for Multiply<E>
where
    Op: Function<Input = E>,
{
    type Output = Multiply<Op::Output>;
    fn fmap(&self) -> Multiply<Op::Output> {
        Multiply {
            lhs: Box::new(Op::call(self.lhs.as_ref())),
            rhs: Box::new(Op::call(self.rhs.as_ref())),
        }
    }
}

impl<Op, L, R> Functor<Op> for Sum<L, R>
where
    Op: Function,
    L: Functor<Op>,
    R: Functor<Op>,
{
    type Output = Sum<L::Output, R::Output>;
    fn fmap(&self) -> Sum<L::Output, R::Output> {
        match self {
            Sum::Left(left) => Sum::Left(left.fmap()),
            Sum::Right(right) => Sum::Right(right.fmap()),
        }
    }
}

impl<Op> Functor<Op> for Expr
where
    Op: Function,
    Sig<Expr>: Functor<Op>,
{
    type Output = <Sig<Expr> as Functor<Op>>::Output;
    fn fmap(&self) -> Self::Output {
        self.0.fmap()
    }
}

impl<Op> Functor<Op> for MultExpr
where
    Op: Function,
    MultSig<MultExpr>: Functor<Op>,
{
    type Output = <MultSig<MultExpr> as Functor<Op>>::Output;
    fn fmap(&self) -> Self::Output {
        self.0.fmap()
    }
}

pub trait EvalAlgebra {
    fn eval(&self) -> i64;
}

impl EvalAlgebra for IntegerLiteral {
    fn eval(&self) -> i64 {
        self.value
    }
}

impl EvalAlgebra for Add<i64> {
    fn eval(&self) -> i64 {
        *self.lhs + *self.rhs
    }
}

impl EvalAlgebra for Multiply<i64> {
    fn eval(&self) -> i64 {
        *self.lhs * *self.rhs
    }
}

impl<L, R> EvalAlgebra for Sum<L, R>
where
    L: EvalAlgebra,
    R: EvalAlgebra,
{
    fn eval(&self) -> i64 {
        match self {
            Sum::Left(lhs) => lhs.eval(),
            Sum::Right(rhs) => rhs.eval(),
        }
    }
}

pub struct Eval;

impl Function for Eval {
    type Input = Expr;
    type Output = i64;
    fn call(expr: &Self::Input) -> Self::Output {
        eval(expr)
    }
}

pub fn eval(expr: &Expr) -> i64 {
    fmap::<Eval, _>(expr).eval()
}

/*
pub fn eval<'a, E>(expr: &'a E) -> i64
where
    E: Functor<'a, E, i64>,
    E::Output: EvalAlgebra,
{
    expr.fmap(eval).eval()
}
*/

use crate::ch04_smart_constructors::*;
pub fn fwomp() -> i64 {
    let add: Expr = add(
        integer_literal(30000),
        add(integer_literal(1330), integer_literal(7)),
    );
    eval(&add)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ch04_smart_constructors::*;

    #[test]
    fn can_evaluate_ugly_expression() {
        let add: Expr = add(integer_literal(118), integer_literal(1219));
        assert_eq!(eval(&add), 1337);
    }

    #[test]
    fn can_evaluate_nested_expression() {
        let add: Expr = add(
            integer_literal(30000),
            add(integer_literal(1330), integer_literal(7)),
        );
        assert_eq!(eval(&add), 31337);
    }
}
