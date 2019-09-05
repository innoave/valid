# Change Log

All user visible changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/), as described
for Rust libraries in [RFC #1105](https://github.com/rust-lang/rfcs/blob/master/text/1105-api-evolution.md)

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
