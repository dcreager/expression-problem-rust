// -*- coding: utf-8 -*-
// ------------------------------------------------------------------------------------------------
// Copyright © 2018, Douglas Creager.
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

use crate::ch02_open_sum::*;
use crate::ch04_smart_constructors::*;
use crate::ch05_multiplication::*;

/// Assuming that X is one of the variants of an open sum E, checks whether a value of E is an
/// instance of variant X.  If so, extracts that out into just an instance of X (i.e., no longer
/// wrapped in the open sum).  If not, returns E as-is.
pub trait Decompose<X, E> {
    fn decompose(self) -> Result<X, E>;
}

impl<L, R> Decompose<L, Self> for Sum<L, R> {
    fn decompose(self) -> Result<L, Self> {
        match self {
            Sum::Left(left) => Ok(left),
            Sum::Right(_) => Err(self),
        }
    }
}

impl<X, L, R> Decompose<X, Self> for Sum<L, R>
where
    R: Decompose<X, R>,
    (X, L): NotEq,
    (X, Self): NotEq,
{
    fn decompose(self) -> Result<X, Self> {
        match self {
            Sum::Left(_) => Err(self),
            Sum::Right(right) => match right.into() {
                Ok(x) => Ok(x),
                Err(r) => Err(Sum::Right(r)),
            },
        }
    }
}

pub fn decompose<X, E>(expr: E) -> Result<X, E>
where
    E: Decompose<X, E>,
{
    expr.decompose()
}

pub fn distribute<E>(expr: E) -> E
where
    E: Decompose<Multiply<E>, E> + Decompose<Add<E>, E>,
{
    match decompose::<Multiply<E>, _>(expr) {
        Ok(mult) => match decompose::<Add<E>, _>(mult.lhs) {
            Ok(add_expr) => add(
                multiply(add_expr.lhs, mult.rhs.clone()),
                multiply(add_expr.rhs, mult.rhs),
            ),
            Err(mult_lhs) => multiply(mult, mult.rhs),
        },
        Err(expr) => expr,
    }
}
