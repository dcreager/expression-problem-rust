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

//! Let's create a Rust trait for the `Expr` Haskell typeclass from the papers.

use crate::ch02_open_sum::*;
use crate::ch05a_multiplication::*;
use crate::ch07a_pairs::*;

//! An Expression represents the AST of one of our mini-languages.  It has a `Signature` associated
//! type, which is a `Sum` of all of the possible terms in the language, along with methods for
//! converting between the signature and the expression.  This is the Rust equivalent of the `Expr`
//! type family from the papers; it ties the knot on the recursive generic type parameters that
//! allow expressions to contain subexpressions.
pub trait Expression {
    type Signature;
    fn wrap(sig: Self::Signature) -> Self;
    fn unwrap(&self) -> &Self::Signature;
}

// And then we define an Expression impl for each of our actual expression AST types.  They're all
// *very* boilerplate.  But!  If we've done this right, it will eliminate *all* of the other
// per-AST-type boilerplate!

impl Expression for Expr {
    type Signature = Sig<Expr>;
    fn wrap(sig: Self::Signature) -> Self {
        Self(Box::new(sig))
    }
    fn unwrap(&self) -> &Self::Signature {
        &self.0
    }
}

impl Expression for MultExpr {
    type Signature = MultSig<MultExpr>;
    fn wrap(sig: Self::Signature) -> Self {
        Self(Box::new(sig))
    }
    fn unwrap(&self) -> &Self::Signature {
        &self.0
    }
}

impl Expression for NoAddExpr {
    type Signature = NoAddSig<NoAddExpr>;
    fn wrap(sig: Self::Signature) -> Self {
        Self(Box::new(sig))
    }
    fn unwrap(&self) -> &Self::Signature {
        &self.0
    }
}

impl Expression for PairExpr {
    type Signature = PairSig<PairExpr>;
    fn wrap(sig: Self::Signature) -> Self {
        Self(Box::new(sig))
    }
    fn unwrap(&self) -> &Self::Signature {
        &self.0
    }
}
