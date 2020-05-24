# Change Log

All user visible changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/), as described
for Rust libraries in [RFC #1105](https://github.com/rust-lang/rfcs/blob/master/text/1105-api-evolution.md)

## 0.3.1 : 2020-05-24

### Fixes

* implementations of traits `Clone`, `Copy`, `PartialEq`, `Eq` and `Debug` for `Validated<C, T>`
  no longer require type parameter `C` to implement those traits.
  
### Internal

* fix clippy warnings
* bump versions of dev-dependencies
* deny unsafe code (instead of warn)


## 0.3.0 : 2019-09-07

### Breaking changes

* use parameters of type `usize` instead `u32` for the constraints `Length` and `CharCount`, which
  enables an implementation that should never panic (issue #4)
* rename constraint `FromTo` to `MustDefineRange` (issue #6)

### Enhancements
  
* implement `TryFrom<usize>` for `Value`
* add new constraint `NonZero` implemented for all primitive integer and float types as well as for
  all types that implement the `Zero` trait of the `num-traits` crate
* add support for validating `BigInt` values of the `num-bigint` crate
* add new `Pattern` constraint which is implemented using the `regex` crate (optional crate feature)

### Fixes

* wrong error code for constraint `CharCount::MinMax` when value has too few characters (issue #5)


## 0.2.0 : 2019-09-05

### Changes

* implementation of `Validate<MustMatch, RelatedFields>` for `(T, T)` requires only `PartialEq`
  instead of total `Eq`
* implement `HasEmptyValue`, `HasLength` and `HasMember` for `VecDeque`, `LinkedList`, `BTreeSet`
  and `BTreeMap` (issue #2)

### Fixes

* make struct `Parameter` public (issue #3)
* minor fixes in documentation


## 0.1.0 : 2019-09-04

First release
