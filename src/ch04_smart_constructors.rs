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

//! Let's make it not hideously ugly to create instances of our expression type.

use crate::ch02_open_sum::*;

// In Rust, we already have the equivalent of the :<: typeclass.  It's called std::convert::From!
// So we just need to define an impl for our Sum type.
//
// Complicating things is that these two impls overlap.  In the paper, Swierstra runs into the same
// difficulty, and relies on a Haskell extension that allows overlapping instances of typeclasses.
// Rust has something similar in #![feature(specialization)], but it unfortunately has more
// restrictions and doesn't work for this example.  Instead, we need to add some extra constraints
// to the second impl to make them no longer conflict.  These extra constraints rely on
// #![feature(optin_builtin_traits)] to define NotEq, which lets us assert that some of the type
// variables in the second impl represent distinct types.
//
// Also note that, like in the paper, we expect the Sum type to be used in a "list-like",
// right-associative fashion.  That is, if you want the sum of A, B, or C, you need to use `Sum<A,
// Sum<B, C>>`, and not `Sum<Sum<A, B>, C>`.

pub auto trait NotEq {}
impl<X> !NotEq for (X, X) {}

impl<L, R> From<L> for Sum<L, R> {
    fn from(left: L) -> Sum<L, R> {
        Sum::Left(left)
    }
}

impl<X, L, R> From<X> for Sum<L, R>
where
    R: From<X>,
    (X, L): NotEq,
    (X, Self): NotEq,
{
    fn from(x: X) -> Sum<L, R> {
        Sum::Right(R::from(x))
    }
}

// And like EvaluateInt, we have to explicitly write an impl for our Expr type.
impl<X> From<X> for Expr
where
    Sig<Expr>: From<X>,
{
    fn from(x: X) -> Expr {
        Expr(Box::new(Sig::<Expr>::from(x)))
    }
}

// With those impls in place, we can define smart constructors like we did in ch01.

pub fn integer_literal<E: From<IntegerLiteral>>(value: i64) -> E {
    E::from(IntegerLiteral { value })
}

pub fn add<E: From<Add<E>>>(lhs: E, rhs: E) -> E {
    E::from(Add { lhs, rhs })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ch03_evaluation::*;

    #[test]
    fn can_evaluate_ugly_expression() {
        // 118 + 1219
        // Much nicer!
        let add: Expr = add(integer_literal(118), integer_literal(1219));
        assert_eq!(add.evaluate(), 1337);
    }

    #[test]
    fn can_evaluate_nested_expression() {
        // 30000 + 1330 + 7
        let add: Expr = add(
            integer_literal(30000),
            add(integer_literal(1330), integer_literal(7)),
        );
        assert_eq!(add.evaluate(), 31337);
    }
}
