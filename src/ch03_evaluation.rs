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

//! And now let's add some evaluation rules for the new types.

use crate::ch02_open_sum::*;

// Again, we don't have functors available, so our evaluation trait in Rust won't exactly line up
// with the typeclass and recursion scheme from Haskell.  But it will have the same capabilities,
// and its definition won't be any more complex.

/// Each kind of term in our language should implement this trait to define how it's evaluated.
pub trait EvaluateInt {
    fn evaluate(&self) -> i64;
}

/// Integer literals evaluate to their value.
impl EvaluateInt for IntegerLiteral {
    fn evaluate(&self) -> i64 {
        self.value
    }
}

impl<E> EvaluateInt for Add<E>
where
    // We can only evaluate an addition if we know how to evaluate its subexpressions.
    E: EvaluateInt,
{
    fn evaluate(&self) -> i64 {
        self.lhs.evaluate() + self.rhs.evaluate()
    }
}

/// We can evaluate a sum if we know how to evaluate both of its variants; we just delegate to the
/// underlying type's impl.
impl<L, R> EvaluateInt for Sum<L, R>
where
    // But of course, we need to be able to evaluate each of the possible subexpressions.
    L: EvaluateInt,
    R: EvaluateInt,
{
    fn evaluate(&self) -> i64 {
        match self {
            Sum::Left(lhs) => lhs.evaluate(),
            Sum::Right(rhs) => rhs.evaluate(),
        }
    }
}

// Since we don't have functors, we can't define a generic `foldExpr` that works for any expression
// type; instead, we have a small amount of boilerplate.  Note that `Sig` does get an `EvaluateInt`
// impl for free — it's just an alias for a particular `Sum`, after all.

impl EvaluateInt for Expr {
    fn evaluate(&self) -> i64 {
        self.0.evaluate()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_evaluate_ugly_expression() {
        // 118 + 1219
        let add: Expr = Expr(Box::new(Sum::Right(Add::<Expr> {
            lhs: Expr(Box::new(Sum::Left(IntegerLiteral { value: 118 }))),
            rhs: Expr(Box::new(Sum::Left(IntegerLiteral { value: 1219 }))),
        })));
        assert_eq!(add.evaluate(), 1337);
    }
}
