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

/// We can use an enum to represent all of the different kinds of term that can appear in our toy
/// language.
pub enum Expression {
    /// An integer constant with a particular value (i.e., we've already parsed whatever string
    /// representations are allowed in our source language; in the AST, we stored the parsed value
    /// of the constant).
    IntegerLiteral(i64),
    /// To add two numbers together, we need the ASTs of the left- and right-hand sides; to make
    /// this recursion work, we have to Box them so that we don't have an infinitely large type.
    Add(Box<Expression>, Box<Expression>),
    /// Ditto for subtraction.
    Subtract(Box<Expression>, Box<Expression>),
}

/// Then we can implement an evaluation function that returns the value of any expression in our
/// language.  We use a `match` statement to define how to evaluate each kind of term.
impl Expression {
    pub fn evaluate(&self) -> i64 {
        match self {
            Expression::IntegerLiteral(value) => *value,
            Expression::Add(lhs, rhs) => lhs.evaluate() + rhs.evaluate(),
            Expression::Subtract(lhs, rhs) => lhs.evaluate() - rhs.evaluate(),
        }
    }
}

// Some smart constructors.  Note that we don't require the return value to be Expression; it can
// be anything that we can create from an Expression.  This will be needed in ch01c.

pub fn integer_literal<E: From<Expression>>(value: i64) -> E {
    E::from(Expression::IntegerLiteral(value))
}

pub fn add<E: From<Expression>>(lhs: Expression, rhs: Expression) -> E {
    E::from(Expression::Add(Box::new(lhs), Box::new(rhs)))
}

pub fn subtract<E: From<Expression>>(lhs: Expression, rhs: Expression) -> E {
    E::from(Expression::Subtract(Box::new(lhs), Box::new(rhs)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_evaluate_integer_literal() {
        // The type annotation is needed because of how we're using the From trait in our smart
        // constructors.
        let one: Expression = integer_literal(1);
        assert_eq!(one.evaluate(), 1);
    }

    #[test]
    fn can_evaluate_add() {
        let add: Expression = add(integer_literal(1), integer_literal(2));
        assert_eq!(add.evaluate(), 3);
    }

    #[test]
    fn can_evaluate_subtract() {
        let subtract: Expression = subtract(integer_literal(1), integer_literal(2));
        assert_eq!(subtract.evaluate(), -1);
    }

    #[test]
    fn can_evaluate_nested() {
        let add: Expression = add(
            integer_literal(1),
            subtract(integer_literal(2), integer_literal(3)),
        );
        assert_eq!(add.evaluate(), 0);
    }
}
