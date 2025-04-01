/*
 * pub trait Eq: PartialEq {}
 *
 * The Eq Trait has 3 requirements:
 *   - Reflexive: `a == a` (Reflexivity Principle);
 *   - Symmetric: `a == b` implies `b == a` (required by `PartialEq` as well);
 *   - Transitive: `a == b` and `b == c` implies `a == c` (required by
 * `PartialEq` as well);
 */
