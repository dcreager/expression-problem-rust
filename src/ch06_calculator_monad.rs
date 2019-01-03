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

//! I'm going to make the bold claim that most of Swierstra §6 isn't relevant in Rust — we don't
//! typically use monads to express stateful computations, we just write Rust code.  And traits
//! give us the means to express the requirements of a function piecewise.

/// A memory store can be incremented by a delta value, but this requires mutable access to it.
pub trait Increment {
    fn increment(&mut self, delta: i64) -> ();
}

/// If you only want to read the contents of the memory, you can get away with non-mutable access
/// to it.
pub trait Recall {
    fn recall(&self) -> i64;
}

/// The simplest memory store is just a struct containing the current contents.
pub struct Mem {
    value: i64,
}

impl Increment for Mem {
    fn increment(&mut self, delta: i64) -> () {
        self.value += delta;
    }
}

impl Recall for Mem {
    fn recall(&self) -> i64 {
        self.value
    }
}

/// The tick function from the paper needs both operations, and Rust's trait bounds already give us
/// a great language for expressing that constraint.
pub fn tick<M>(mem: &mut M) -> i64
where
    M: Increment + Recall,
{
    let y = mem.recall();
    mem.increment(1);
    y
}

/// This function (which doesn't appear in the paper) only needs the Recall operation, and as such,
/// works with a non-mutable reference to the store.
pub fn get<M>(mem: &M) -> i64
where
    M: Recall,
{
    mem.recall()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_run_tick() {
        let mut mem = Mem { value: 4 };
        let result = tick(&mut mem);
        assert_eq!(result, 4);
        assert_eq!(mem.value, 5);
    }

    #[test]
    fn can_run_get() {
        assert_eq!(get(&Mem { value: 4 }), 4);
        assert_eq!(get(&Mem { value: 10 }), 10);
    }
}
