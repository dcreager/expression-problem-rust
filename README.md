# Exploring the "expression problem" in Rust

Let's explore some interesting papers in Rust!

- ["Data types à la carte"][data-types], Wouter Swierstra.
- ["Compositional data types"][compositional], Patrick Bahr, Tom Hvitved.

These papers all discuss the "expression problem", as described by [Phil
Wadler][expression-problem]:

> The goal is to define a data type by cases, where one can add new cases to the data type
and new functions over the data type, without recompiling existing code, and while retaining
static type safety.

[compositional]: http://bahr.io/pubs/files/bahr11wgp-paper.pdf
[data-types]: http://www.cs.ru.nl/%7EW.Swierstra/Publications/DataTypesALaCarte.pdf
[expression-problem]: http://homepages.inf.ed.ac.uk/wadler/papers/expression/expression.txt

Since these are FP papers, they all present solutions in Haskell.  I'd like to
explore the problem in Rust!  Unlike some other "lets implement some papers in
language X" explorations, my goal here is **_not_** to translate the Haskell
solutions into Rust; instead, I want to see what a *Rust solution* would look
like.  Partly that's because (at least as of right now) we don't have some of
the same building blocks available in Rust (e.g., functors, higher-kinded
types).  But my hunch is that we have *different* building blocks that let us
produce solutions with the same properties.  Let's see if that's true!

Since a big part of the expression problem talks about what you can do without
editing existing code, I'm going to implement this is a bunch of different Rust
modules.  That will help enforce that we're building new capabilities by only
writing new code, and not by editing any existing code.

### Data types à la carte

#### §1: Introduction

- [ch01a\_before](src/ch01a_before.rs): We create a toy language and an AST for
  it, and write some evaluation rules.

- [ch01b\_new\_method](src/ch01b_new_method.rs): We add a new method that
  operates on our AST.

- [ch01c\_sad\_face](src/ch01c_sad_face.rs): We try to add a new kind of term to
  the language, and get pretty far before running into a brick wall.

#### §2: Fixing the expression problem

- [ch02\_open\_sum](src/ch02_open_sum.rs): Create separate types for each kind
  of term, instead of force-glomming them into a single enum type.  Define a
  generic `Sum` type to be able to pick and choose which ones to include.

#### §3: Evaluation

- [ch03\_evaluation][]: Define evaluation using a trait,
  with an impl of that trait for each of our terms.

[ch03\_evaluation]: src/ch03_evaluation.rs

#### §4: Automating injections

- [ch04\_smart\_constructors](src/ch04_smart_constructors.rs): Make it not so
  hideously ugly to create instances of our new types.

#### §5: Examples

- [ch05a\_multiplication](src/ch05a_multiplication.rs): It's pretty easy to add
  a new kind of term!

- [ch05b\_display](src/ch05b_display.rs): It's also pretty easy to add a new
  function that operates on all of the terms we've defined so far!

#### §6: Monads for free

- [ch06\_calculator\_monad](src/ch06_calculator_monad.rs): In Rust, the monads
  are even more free?  Since we don't really need them?

### Compositional data types

#### §2.1 Evaluating expressions

- [ch07a\_pairs](src/ch07a_pairs.rs): Let's add a new type of value to our
  language.

- [ch07b\_generic\_evaluation](src/ch07b_generic_evaluation.rs): How could we
  have done a better job of defining our evaluation rules from
  [ch03\_evaluation][] — so that we could reuse them as-is with our new
  pair-equipped language?

- [ch07c\_pair\_evaluation](src/ch07c_pair_evaluation.rs): And now we can add
  evaluation rules for pairs too!  And we can reuse the existing evaluation
  rules for integers, even though we now have a more complicated value type.

- [ch07d\_safer\_pair\_evaluation](src/ch07d_safer_pair_evaluation.rs): Let's do
  that again without panicking when we encounter a type error.  No changes
  needed to the evaluation rules — we just need to define a value type that can
  encode errors!
