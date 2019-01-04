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

pub trait Functor<'a, A, B>
where
    A: 'a,
{
    type Output;
    fn fmap(&'a self, f: impl Fn(&'a A) -> B) -> Self::Output;
}

impl<'a, A, B> Functor<'a, A, B> for IntegerLiteral
where
    A: 'a,
{
    type Output = IntegerLiteral;
    fn fmap(&'a self, _f: impl Fn(&'a A) -> B) -> IntegerLiteral {
        IntegerLiteral { value: self.value }
    }
}

impl<'a, A, B> Functor<'a, A, B> for Add<A>
where
    A: 'a,
{
    type Output = Add<B>;
    fn fmap(&'a self, f: impl Fn(&'a A) -> B) -> Add<B> {
        Add {
            lhs: Box::new(f(self.lhs.as_ref())),
            rhs: Box::new(f(self.rhs.as_ref())),
        }
    }
}

impl<'a, A, B> Functor<'a, A, B> for Multiply<A>
where
    A: 'a,
{
    type Output = Multiply<B>;
    fn fmap(&'a self, f: impl Fn(&'a A) -> B) -> Multiply<B> {
        Multiply {
            lhs: Box::new(f(self.lhs.as_ref())),
            rhs: Box::new(f(self.rhs.as_ref())),
        }
    }
}

impl<'a, A, B, L, R> Functor<'a, A, B> for Sum<L, R>
where
    A: 'a,
    L: Functor<'a, A, B>,
    R: Functor<'a, A, B>,
{
    type Output = Sum<L::Output, R::Output>;
    fn fmap(&'a self, f: impl Fn(&'a A) -> B) -> Sum<L::Output, R::Output> {
        match self {
            Sum::Left(left) => Sum::Left(left.fmap(f)),
            Sum::Right(right) => Sum::Right(right.fmap(f)),
        }
    }
}

impl<'a, B> Functor<'a, Expr, B> for Expr
where
    Expr: 'a,
{
    type Output = Sig<B>;
    fn fmap(&'a self, f: impl Fn(&'a Expr) -> B) -> Sig<B> {
        self.0.fmap(f)
    }
}

impl<'a, B> Functor<'a, MultExpr, B> for MultExpr
where
    MultExpr: 'a,
{
    type Output = MultSig<B>;
    fn fmap(&'a self, f: impl Fn(&'a MultExpr) -> B) -> MultSig<B> {
        self.0.fmap(f)
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

pub fn eval<'a, E>(expr: &'a E) -> i64
where
    E: Functor<'a, E, i64>,
    E::Output: EvalAlgebra,
{
    expr.fmap(eval).eval()
}

use crate::ch04_smart_constructors::*;
pub fn fwomp() -> i64 {
    let add: MultExpr = multiply(
        integer_literal(30000),
        add(integer_literal(1330), integer_literal(7)),
    );
    eval(&add)
}

#[cfg(test)]
mod tests {
    use super::*;

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
