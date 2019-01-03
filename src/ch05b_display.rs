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

//! We can add a new function that operates on all of these terms that we've created so far.

use crate::ch02_open_sum::*;
use crate::ch05a_multiplication::*;

use std::fmt;

// Add an impl for each term.

impl fmt::Display for IntegerLiteral {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.value.fmt(f)
    }
}

impl<E> fmt::Display for Add<E>
where
    E: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} + {})", self.lhs, self.rhs)
    }
}

impl<E> fmt::Display for Multiply<E>
where
    E: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} * {})", self.lhs, self.rhs)
    }
}

// And one for the open sum!

impl<L, R> fmt::Display for Sum<L, R>
where
    L: fmt::Display,
    R: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Sum::Left(lhs) => lhs.fmt(f),
            Sum::Right(rhs) => rhs.fmt(f),
        }
    }
}

// And then the boilerplate impl for each expression type.

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Display for MultExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Display for NoAddExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ch04_smart_constructors::*;

    #[test]
    fn can_render_ugly_expression() {
        let add: Expr = add(integer_literal(118), integer_literal(1219));
        assert_eq!(format!("{}", add), "(118 + 1219)");
    }

    #[test]
    fn can_render_multiplication() {
        let mult: MultExpr = add(
            multiply(integer_literal(80), integer_literal(5)),
            integer_literal(4),
        );
        assert_eq!(format!("{}", mult), "((80 * 5) + 4)");
    }

    #[test]
    fn can_evaluate_no_add_multiplication() {
        let mult: NoAddExpr = multiply(integer_literal(6), integer_literal(7));
        assert_eq!(format!("{}", mult), "(6 * 7)");
    }
}
