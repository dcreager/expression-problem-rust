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

//! Now we should be able to define evaluation rules for pairs, where the results might be pairs or
//! integers, and reuse the evaluation rules for all of the existing terms.

use crate::ch07a_pairs::*;
use crate::ch07b_generic_evaluation::*;

/// We have pairs in Rust, so we can use that when defining the result for a pair expression.  Just
/// like integer literals, we can lift the Rust pairs into the result type as long as it has the
/// right From impl.
impl<V, E> EvaluateAny<V> for Pair<E>
where
    E: EvaluateAny<V>,
    V: From<(V, V)>,
{
    fn evaluate(&self) -> V {
        V::from((self.first.evaluate(), self.second.evaluate()))
    }
}

/// We don't have a trait that we can reuse for the evaluation rules for First and Second, like we
/// could with std::ops::Add for our Add term.  So let's make one!
pub trait ProjectPair {
    fn first(self) -> Self;
    fn second(self) -> Self;
}

impl<V, E> EvaluateAny<V> for First<E>
where
    E: EvaluateAny<V>,
    V: ProjectPair,
{
    fn evaluate(&self) -> V {
        self.pair.evaluate().first()
    }
}

impl<V, E> EvaluateAny<V> for Second<E>
where
    E: EvaluateAny<V>,
    V: ProjectPair,
{
    fn evaluate(&self) -> V {
        self.pair.evaluate().second()
    }
}

/// And the EvaluateAny impl for our expression type needs to reference all of these constraints.
impl<V> EvaluateAny<V> for PairExpr
where
    V: From<i64> + From<(V, V)> + std::ops::Add<Output = V> + ProjectPair,
{
    fn evaluate(&self) -> V {
        self.0.evaluate()
    }
}

/// Now we need a value type that can be either an integer or a pair, with all of the various value
/// impls that we've defined or used so far.
#[derive(Debug, PartialEq)]
pub enum IntOrPair {
    Int(i64),
    Pair(Box<IntOrPair>, Box<IntOrPair>),
}

impl From<i64> for IntOrPair {
    fn from(value: i64) -> IntOrPair {
        IntOrPair::Int(value)
    }
}

impl std::ops::Add for IntOrPair {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        if let IntOrPair::Int(lhs) = self {
            if let IntOrPair::Int(rhs) = other {
                return IntOrPair::Int(lhs + rhs);
            }
        }
        panic!("Cannot add non-integers");
    }
}

impl From<(IntOrPair, IntOrPair)> for IntOrPair {
    fn from(value: (IntOrPair, IntOrPair)) -> IntOrPair {
        IntOrPair::Pair(Box::new(value.0), Box::new(value.1))
    }
}

impl ProjectPair for IntOrPair {
    fn first(self) -> IntOrPair {
        if let IntOrPair::Pair(first, _) = self {
            return *first;
        }
        panic!("Cannot project non-pairs");
    }

    fn second(self) -> IntOrPair {
        if let IntOrPair::Pair(_, second) = self {
            return *second;
        }
        panic!("Cannot project non-pairs");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ch04_smart_constructors::*;

    // We can still evaluate all of the expressions that don't mention pairs, even though pairs are
    // now possible!  The names of the types are different, but otherwise these tests are exactly
    // the same as in ch03 and ch07b!

    #[test]
    fn can_evaluate_ugly_expression() {
        // 118 + 1219
        let add: PairExpr = add(integer_literal(118), integer_literal(1219));
        // Kind of gross
        assert_eq!(
            (&add as &dyn EvaluateAny<IntOrPair>).evaluate(),
            IntOrPair::Int(1337)
        );
        // A little bit nicer
        assert_eq!(evaluate_any::<IntOrPair, _>(&add), IntOrPair::Int(1337));
    }

    #[test]
    fn can_evaluate_nested_expression() {
        // 30000 + 1330 + 7
        let add: PairExpr = add(
            integer_literal(30000),
            add(integer_literal(1330), integer_literal(7)),
        );
        assert_eq!(
            (&add as &dyn EvaluateAny<IntOrPair>).evaluate(),
            IntOrPair::Int(31337)
        );
        assert_eq!(evaluate_any::<IntOrPair, _>(&add), IntOrPair::Int(31337));
    }

    // And we can also evaluate expressions that mention pairs.

    #[test]
    fn can_evaluate_pair() {
        let expr: PairExpr = pair(integer_literal(7), integer_literal(6));
        assert_eq!(
            (&expr as &dyn EvaluateAny<IntOrPair>).evaluate(),
            IntOrPair::Pair(Box::new(IntOrPair::Int(7)), Box::new(IntOrPair::Int(6)))
        );
        assert_eq!(
            evaluate_any::<IntOrPair, _>(&expr),
            IntOrPair::Pair(Box::new(IntOrPair::Int(7)), Box::new(IntOrPair::Int(6)))
        );
    }

    #[test]
    fn can_evaluate_pair_projection() {
        let expr: PairExpr = first(pair(integer_literal(7), integer_literal(6)));
        assert_eq!(
            (&expr as &dyn EvaluateAny<IntOrPair>).evaluate(),
            IntOrPair::Int(7)
        );
        assert_eq!(evaluate_any::<IntOrPair, _>(&expr), IntOrPair::Int(7));
    }

    // But we'd better get the types right, or we'll panic!

    #[test]
    fn cannot_project_integer() {
        let expr: PairExpr = first(integer_literal(7));
        let result = std::panic::catch_unwind(|| (&expr as &dyn EvaluateAny<IntOrPair>).evaluate());
        assert!(result.is_err());
    }

    #[test]
    fn cannot_add_pairs() {
        let expr: PairExpr = add(
            pair(integer_literal(1), integer_literal(2)),
            integer_literal(3),
        );
        let result = std::panic::catch_unwind(|| (&expr as &dyn EvaluateAny<IntOrPair>).evaluate());
        assert!(result.is_err());
    }
}
