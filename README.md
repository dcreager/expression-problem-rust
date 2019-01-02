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
