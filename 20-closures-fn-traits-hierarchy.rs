/***********************
 * Fn Traits Hierarchy *
 ***********************
 *
 * ----------
 * | FnOnce |
 * ----------
 *     - Closure captures values by **move** (transferring ownership);
 *     - Closure will be invoked once.
 *
 *     ^
 *     |
 *
 * ---------
 * | FnMut |
 * ---------
 *     - Captures values by **mutable reference**;
 *     - Closure can be invoked multiple times.
 *
 *     ^
 *     |
 *
 * ------
 * | Fn | (Strictest)
 * ------
 *     - Closure captures values by **immutable reference** (read-only) or doesn't
 *   capture anything at all;
 *     - Closure can be invoked multiple times.
 *
 */
