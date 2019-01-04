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

//! Let's add pairs to the language!  Note that we've switched from "Data types à la carte" over to
//! "Compositional data types".

use crate::ch02_open_sum::*;

/// Creates a new pair, whose contents are given by two subexpressions.
pub struct Pair<E> {
    pub first: E,
    pub second: E,
}

/// Extract the first element of a pair.
pub struct First<E> {
    pub pair: E,
}

/// Extract the second element of a pair.
pub struct Second<E> {
    pub pair: E,
}

// And some smart constructors

pub fn pair<E: From<Pair<E>>>(first: E, second: E) -> E {
    E::from(Pair { first, second })
}

pub fn first<E: From<First<E>>>(pair: E) -> E {
    E::from(First { pair })
}

pub fn second<E: From<Second<E>>>(pair: E) -> E {
    E::from(Second { pair })
}

// All of these nested Sums are getting cumbersome.  Let's add a macro.

macro_rules! Sum {
    { $A:ty, $B:ty } => { Sum<$A, $B> };
    { $A:ty, $($B:ty),+ } => { Sum<$A, Sum![$($B),+]> };
}

// Now we create an expression type that can include pairs.

pub type PairSig<E> = Sum![Pair<E>, First<E>, Second<E>, Sig<E>];
pub struct PairExpr(pub Box<PairSig<PairExpr>>);

impl<X> From<X> for PairExpr
where
    PairSig<PairExpr>: From<X>,
{
    fn from(x: X) -> PairExpr {
        PairExpr(Box::new(PairSig::<PairExpr>::from(x)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ch04_smart_constructors::*;

    #[test]
    fn can_create_pair_expressions() {
        let _: PairExpr = first(pair(integer_literal(7), integer_literal(6)));
    }
}
