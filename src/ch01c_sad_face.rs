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

// We've defined the AST representation of our language in another module.  Our goal is to add a
// new kind of term to the language — without editing or copying the original definitions.
//
// Note that we're not using a * import so that we can be clear whether we're referring to the new
// Expression, which includes the new kind of term, or the old one, which doesn't.
use crate::ch01a_before;

/// This is the closest we can get: we create a new Expression type, containing each of the new
/// kinds of term, and use a wrapper variant at the end to include all of the old terms without
/// duplicating their definitions.
pub enum Expression {
    /// We can now negate numbers, too!
    Negate(Box<Expression>),
    /// But we want to be able to use all of the existing terms without copying their definitions.
    Existing(ch01a_before::Expression),
}

/// We can wrap any old expression in our new AST type.
impl std::convert::From<ch01a_before::Expression> for Expression {
    fn from(wrapped: ch01a_before::Expression) -> Expression {
        Expression::Existing(wrapped)
    }
}

/// We can implement a new evaluate function that knows what to do with the new kind of term, but
/// which delegates to the existing function for all of the existing kinds of term.
impl Expression {
    pub fn evaluate(&self) -> i64 {
        match self {
            Expression::Negate(nested) => -nested.evaluate(),
            Expression::Existing(existing) => existing.evaluate(),
        }
    }
}

// We can use the existing smart constructors as-is because of how we used From.
pub use crate::ch01a_before::add;
pub use crate::ch01a_before::integer_literal;
pub use crate::ch01a_before::subtract;

// And then a smart constructor for the new term

pub fn negate(nested: Expression) -> Expression {
    Expression::Negate(Box::new(nested))
}

#[cfg(test)]
mod tests {
    use super::*;

    // All of the old smart constructors and evaluation rules Just Work:

    #[test]
    fn can_evaluate_integer_literal() {
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

    // And so do the new ones:

    #[test]
    fn can_evaluate_negate() {
        let negate: Expression = negate(integer_literal(1));
        assert_eq!(negate.evaluate(), -1);
    }

    #[test]
    fn can_evaluate_nested_negate() {
        let negate: Expression = negate(subtract(integer_literal(2), integer_literal(3)));
        assert_eq!(negate.evaluate(), 1);
    }

    // But!  We cannot put a negation inside of any of the old kinds of terms!

    /*
    #[test]
    fn cannot_evaluate_negate_inside_add() {
        // This line won't compile!
        let add: Expression = add(integer_literal(1), negate(integer_literal(2)));
        assert_eq!(negate.evaluate(), -1);
    }
    */
}
