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

pub trait Functor<'a, A, B, F>
where
    A: 'a,
    F: Fn(&'a A) -> B,
{
    type Output;
    fn fmap(&'a self, f: F) -> Self::Output;
}

impl<'a, A, B, F> Functor<'a, A, B, F> for IntegerLiteral
where
    A: 'a,
    F: Fn(&'a A) -> B,
{
    type Output = IntegerLiteral;
    fn fmap(&'a self, _f: F) -> IntegerLiteral {
        IntegerLiteral { value: self.value }
    }
}

impl<'a, A, B, F> Functor<'a, A, B, F> for Add<A>
where
    A: 'a,
    F: Fn(&'a A) -> B,
{
    type Output = Add<B>;
    fn fmap(&'a self, f: F) -> Add<B> {
        Add {
            lhs: Box::new(f(self.lhs.as_ref())),
            rhs: Box::new(f(self.rhs.as_ref())),
        }
    }
}

impl<'a, A, B, F, L, R> Functor<'a, A, B, F> for Sum<L, R>
where
    A: 'a,
    F: Fn(&'a A) -> B,
    L: Functor<'a, A, B, F>,
    R: Functor<'a, A, B, F>,
{
    type Output = Sum<L::Output, R::Output>;
    fn fmap(&'a self, f: F) -> Sum<L::Output, R::Output> {
        match self {
            Sum::Left(left) => Sum::Left(left.fmap(f)),
            Sum::Right(right) => Sum::Right(right.fmap(f)),
        }
    }
}

impl<'a, B, F> Functor<'a, Expr, B, F> for Expr
where
    Expr: 'a,
    F: Fn(&'a Expr) -> B,
{
    type Output = Sig<B>;
    fn fmap(&'a self, f: F) -> Sig<B> {
        self.0.fmap(f)
    }
}

trait EvalAlgebra {
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

pub fn eval(expr: &Expr) -> i64 {
    expr.fmap(eval).eval()
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
