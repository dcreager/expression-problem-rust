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

//! Instead of having a single Expression enum, with different variants for each kind of term, we
//! create a separate type for each kind of term.  We're going to name each type the same as the
//! enum variant from ch01a, to make it clear how they line up.

/// An integer constant with a particular value.  Note that unlike in the paper, and unlike the Add
/// and Subtract terms below, this is **not** parameterized by the `e` type!  We don't have
/// functors in Rust, and so we don't need to force each of our term representations to have the
/// same kind.
pub struct IntegerLiteral {
    pub value: i64,
}

/// We can add two expressions together, but since we don't have an Expression type (yet), we don't
/// know what type the left- and right-hand sides should have.  Let's punt for now, and take that
/// in as a generic type parameter.  (Just like Swierstra does in the paper!)
pub struct Add<E> {
    pub lhs: Box<E>,
    pub rhs: Box<E>,
}

/// This is how we'll create the different Expression types from ch01!  This corresponds to the :+:
/// "coproduct" operator from the paper.
pub enum Sum<L, R> {
    Left(L),
    Right(R),
}

// To create the analogue of `Expr (Val :+: Add)` in Rust, we'd ideally want to do:
//
// pub type Expr = Sum<IntegerLiteral, Add<Expr>>;
//
// But that won't compile, since you end up with a cycle in the type expansion.  We end up having
// to define the `Val :+: Add` part and the `Expr` wrapper separately:

pub type Sig<E> = Sum<IntegerLiteral, Add<E>>;
pub struct Expr(pub Sig<Expr>);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_instantiate_ugly_expression() {
        // 118 + 1219
        // This is ugly, but we can instantiate it!
        let _: Expr = Expr(Sum::Right(Add::<Expr> {
            lhs: Box::new(Expr(Sum::Left(IntegerLiteral { value: 118 }))),
            rhs: Box::new(Expr(Sum::Left(IntegerLiteral { value: 1219 }))),
        }));
    }
}
