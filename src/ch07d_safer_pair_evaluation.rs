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

//! The panics in our previous evaluation rules for pairs weren't that great.  Let's create a safer
//! value type that treats errors as their own kind of result.  We won't need to change any of the
//! evaluation rules, just the definition of our result type!

use crate::ch07c_pair_evaluation::*;

#[derive(Debug, PartialEq)]
pub struct SafeIntOrPair(Option<IntOrPair>);

impl From<Option<IntOrPair>> for SafeIntOrPair {
    fn from(value: Option<IntOrPair>) -> SafeIntOrPair {
        SafeIntOrPair(value)
    }
}

impl From<i64> for SafeIntOrPair {
    fn from(value: i64) -> SafeIntOrPair {
        Some(IntOrPair::Int(value)).into()
    }
}

impl std::ops::Add for SafeIntOrPair {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        if let SafeIntOrPair(Some(IntOrPair::Int(lhs))) = self {
            if let SafeIntOrPair(Some(IntOrPair::Int(rhs))) = other {
                return Some(IntOrPair::Int(lhs + rhs)).into();
            }
        }
        None.into()
    }
}

impl From<(SafeIntOrPair, SafeIntOrPair)> for SafeIntOrPair {
    fn from(value: (SafeIntOrPair, SafeIntOrPair)) -> SafeIntOrPair {
        if let SafeIntOrPair(Some(first)) = value.0 {
            if let SafeIntOrPair(Some(second)) = value.1 {
                return Some(IntOrPair::Pair(Box::new(first), Box::new(second))).into();
            }
        }
        None.into()
    }
}

impl ProjectPair for SafeIntOrPair {
    fn first(self) -> SafeIntOrPair {
        if let SafeIntOrPair(Some(IntOrPair::Pair(first, _))) = self {
            return SafeIntOrPair(Some(*first));
        }
        None.into()
    }

    fn second(self) -> SafeIntOrPair {
        if let SafeIntOrPair(Some(IntOrPair::Pair(_, second))) = self {
            return SafeIntOrPair(Some(*second));
        }
        None.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ch04_smart_constructors::*;
    use crate::ch07a_pairs::*;
    use crate::ch07b_generic_evaluation::*;

    // All of the successful evaluations look exactly the same, except that we're referencing the
    // new result type (SafeIntOrPair instead of IntOrPair).

    #[test]
    fn can_evaluate_ugly_expression() {
        // 118 + 1219
        let add: PairExpr = add(integer_literal(118), integer_literal(1219));
        // Kind of gross
        assert_eq!(
            (&add as &Evaluate<SafeIntOrPair>).evaluate(),
            Some(IntOrPair::Int(1337)).into()
        );
        // A little bit nicer
        assert_eq!(
            evaluate::<SafeIntOrPair, _>(&add),
            Some(IntOrPair::Int(1337)).into()
        );
    }

    #[test]
    fn can_evaluate_nested_expression() {
        // 30000 + 1330 + 7
        let add: PairExpr = add(
            integer_literal(30000),
            add(integer_literal(1330), integer_literal(7)),
        );
        assert_eq!(
            (&add as &Evaluate<SafeIntOrPair>).evaluate(),
            Some(IntOrPair::Int(31337)).into()
        );
        assert_eq!(
            evaluate::<SafeIntOrPair, _>(&add),
            Some(IntOrPair::Int(31337)).into()
        );
    }

    #[test]
    fn can_evaluate_pair() {
        let expr: PairExpr = pair(integer_literal(7), integer_literal(6));
        assert_eq!(
            (&expr as &Evaluate<SafeIntOrPair>).evaluate(),
            Some(IntOrPair::Pair(
                Box::new(IntOrPair::Int(7)),
                Box::new(IntOrPair::Int(6))
            ))
            .into()
        );
        assert_eq!(
            evaluate::<SafeIntOrPair, _>(&expr),
            Some(IntOrPair::Pair(
                Box::new(IntOrPair::Int(7)),
                Box::new(IntOrPair::Int(6))
            ))
            .into()
        );
    }

    #[test]
    fn can_evaluate_pair_projection() {
        let expr: PairExpr = first(pair(integer_literal(7), integer_literal(6)));
        assert_eq!(
            (&expr as &Evaluate<SafeIntOrPair>).evaluate(),
            Some(IntOrPair::Int(7)).into()
        );
        assert_eq!(
            evaluate::<SafeIntOrPair, _>(&expr),
            Some(IntOrPair::Int(7)).into()
        );
    }

    // The failed evaluations now produce an Error value, instead of panicking!  Nice!

    #[test]
    fn cannot_project_integer() {
        let expr: PairExpr = first(integer_literal(7));
        assert_eq!((&expr as &Evaluate<SafeIntOrPair>).evaluate(), None.into());
        assert_eq!(evaluate::<SafeIntOrPair, _>(&expr), None.into());
    }

    #[test]
    fn cannot_add_pairs() {
        let expr: PairExpr = add(
            pair(integer_literal(1), integer_literal(2)),
            integer_literal(3),
        );
        assert_eq!((&expr as &Evaluate<SafeIntOrPair>).evaluate(), None.into());
        assert_eq!(evaluate::<SafeIntOrPair, _>(&expr), None.into());
    }
}
