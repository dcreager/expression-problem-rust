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
// new method that operates on those ASTs without touching the original definitions.
use crate::ch01a_before::*;

use std::fmt;

/// No problem!  We can't use this definition of std::fmt::Display back in ch01a_before, but that's
/// okay, since we don't need to.
impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::IntegerLiteral(value) => write!(f, "{}", value),
            Expression::Add(lhs, rhs) => write!(f, "({} + {})", lhs, rhs),
            Expression::Subtract(lhs, rhs) => write!(f, "({} - {})", lhs, rhs),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_evaluate_integer_literal() {
        let one: Expression = integer_literal(1);
        assert_eq!(format!("{}", one), "1");
    }

    #[test]
    fn can_evaluate_add() {
        let add: Expression = add(integer_literal(1), integer_literal(2));
        assert_eq!(format!("{}", add), "(1 + 2)");
    }

    #[test]
    fn can_evaluate_subtract() {
        let subtract: Expression = subtract(integer_literal(1), integer_literal(2));
        assert_eq!(format!("{}", subtract), "(1 - 2)");
    }

    #[test]
    fn can_evaluate_nested() {
        let add: Expression = add(
            integer_literal(1),
            subtract(integer_literal(2), integer_literal(3)),
        );
        assert_eq!(format!("{}", add), "(1 + (2 - 3))");
    }
}
