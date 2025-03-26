/*
 * use std::iter::Iterator;
 *
 * pub trait Iterator {
 *     // Required Associated Types:
 *     type Item;
 * 
 *     // Required methods:
 *     fn next(&mut self) -> Option<Self::Item>
 * 
 *     // Provided methods (76 methods!)
 *     ...
 * }
 */

/*
 * use std::iter::IntoIterator;
 * 
 * pub trait IntoIterator {
 *     // Required Associated Types:
 *     type Item;
 *     type IntoIter: Iterator<Item = Self::Item>;
 * 
 *     // Required methods:
 *     fn into_iter(self) -> Self::IntoIter;
 * }
 */
