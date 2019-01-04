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

//! Our existing evaluation trait doesn't work with our new expressions, since the result can only
//! be an integer.  We need a result type that allows pairs, as well!  Can we construct an
//! evaluation trait that lets evaluate the old Expr type (which doesn't include pairs), but which
//! we can extend in a different module to work with PairExpr and pairs?  (Yes.)

use crate::ch02_open_sum::*;
use crate::ch05a_multiplication::*;

/// Well that was easy.  (Not really!  Don't worry, we'll run into wrinkles.)
pub trait Evaluate<V> {
    fn evaluate(&self) -> V;
}

/// This helper function will make things easier for us when we try to call our evaluate trait
/// method.  (We'll have to explicitly call out which value type we want to use, and it's somewhat
/// less verbose to do that on a function than trying to cast our Expr type to the right trait
/// instantiation.)
pub fn evaluate<V, E>(expr: &E) -> V
where
    E: Evaluate<V>,
{
    expr.evaluate()
}

/// Integer literals evaluate to their value.  We can lift them into any result type that can be
/// constructed from an integer.
impl<V> Evaluate<V> for IntegerLiteral
where
    V: From<i64>,
{
    fn evaluate(&self) -> V {
        V::from(self.value)
    }
}

impl<V, E> Evaluate<V> for Add<E>
where
    // We can only evaluate an addition if we know how to evaluate its subexpressions.
    E: Evaluate<V>,
    // and if the result type can itself be added together!
    V: std::ops::Add<Output = V>,
{
    fn evaluate(&self) -> V {
        self.lhs.evaluate() + self.rhs.evaluate()
    }
}

impl<V, E> Evaluate<V> for Multiply<E>
where
    E: Evaluate<V>,
    V: std::ops::Mul<Output = V>,
{
    fn evaluate(&self) -> V {
        self.lhs.evaluate() * self.rhs.evaluate()
    }
}

/// We can evaluate a sum if we know how to evaluate both of its variants; we just delegate to the
/// underlying type's impl.
impl<V, L, R> Evaluate<V> for Sum<L, R>
where
    L: Evaluate<V>,
    R: Evaluate<V>,
{
    fn evaluate(&self) -> V {
        match self {
            Sum::Left(lhs) => lhs.evaluate(),
            Sum::Right(rhs) => rhs.evaluate(),
        }
    }
}

/// Like before, we have to explicitly provide an Evaluate impl for our expression types.  The main
/// wrinkle is that we **also** have to explicitly carry over any of the constraints that the
/// individual terms require of the value type — Rust won't propagate those for us.
impl<V> Evaluate<V> for Expr
where
    V: From<i64> + std::ops::Add<Output = V>,
{
    fn evaluate(&self) -> V {
        self.0.evaluate()
    }
}

impl<V> Evaluate<V> for MultExpr
where
    V: From<i64> + std::ops::Add<Output = V> + std::ops::Mul<Output = V>,
{
    fn evaluate(&self) -> V {
        self.0.evaluate()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ch04_smart_constructors::*;

    #[test]
    fn can_evaluate_ugly_expression() {
        // 118 + 1219
        let add: Expr = add(integer_literal(118), integer_literal(1219));
        // Kind of gross
        assert_eq!((&add as &Evaluate<i64>).evaluate(), 1337);
        // A little bit nicer
        assert_eq!(evaluate::<i64, _>(&add), 1337);
    }

    #[test]
    fn can_evaluate_nested_expression() {
        // 30000 + 1330 + 7
        let add: Expr = add(
            integer_literal(30000),
            add(integer_literal(1330), integer_literal(7)),
        );
        assert_eq!((&add as &Evaluate<i64>).evaluate(), 31337);
        assert_eq!(evaluate::<i64, _>(&add), 31337);
    }

    #[test]
    fn can_evaluate_multiplication() {
        let mult: MultExpr = add(
            multiply(integer_literal(80), integer_literal(5)),
            integer_literal(4),
        );
        assert_eq!((&mult as &Evaluate<i64>).evaluate(), 404);
        assert_eq!(evaluate::<i64, _>(&mult), 404);
    }
}
