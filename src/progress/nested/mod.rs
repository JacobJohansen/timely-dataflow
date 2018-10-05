//! Coordination of progress information between a scope-as-operator and its children operators.

pub use self::subgraph::{Subgraph, SubgraphBuilder};
pub use self::subgraph::{Source, Target};
// pub use self::summary::Summary;

pub mod pointstamp_counter;
// pub mod summary;
pub mod product;
pub mod subgraph;
// pub mod subgraph_neu;

pub mod reachability;
pub mod reachability_neu;

use super::Timestamp;

/// Conversion between pointstamp types.
///
/// This trait is central to nested scopes, for which the inner timestamp must be
/// related to the outer timestamp. These methods define those relationships.
///
/// It would be ideal to use Rust's From and Into traits, but they seem to be messed
/// up due to coherence: we can't implement `Into` because it induces a from implementation
/// we can't control.
pub trait Refines<T: Timestamp> : Timestamp {
    /// Converts the outer timestamp to an inner timestamp.
    fn to_inner(other: T) -> Self;
    /// Converts the inner timestamp to an outer timestamp.
    fn to_outer(self) -> T;
    /// Summarizes an inner path summary as an outer path summary.
    ///
    /// It is crucial for correctness that the result of this summarization's `results_in`
    /// method is equivalent to `|time| path.results_in(time.to_inner()).to_outer()`, or
    /// at least produces times less or equal to that result.
    fn summarize(path: <Self as Timestamp>::Summary) -> <T as Timestamp>::Summary;
}

impl<T: Timestamp> Refines<T> for T {
    fn to_inner(other: T) -> T { other }
    fn to_outer(self) -> T { self }
    fn summarize(path: <T as Timestamp>::Summary) -> <T as Timestamp>::Summary { path }
}

use progress::timestamp::{RootTimestamp, RootSummary};
impl Refines<RootTimestamp> for usize {
    fn to_inner(_: RootTimestamp) -> usize { Default::default() }
    fn to_outer(self) -> RootTimestamp { RootTimestamp }
    fn summarize(_: <usize as Timestamp>::Summary) -> RootSummary { RootSummary }
}

impl Refines<RootTimestamp> for u64 {
    fn to_inner(_: RootTimestamp) -> u64 { Default::default() }
    fn to_outer(self) -> RootTimestamp { RootTimestamp }
    fn summarize(_: <u64 as Timestamp>::Summary) -> RootSummary { RootSummary }
}

impl Refines<RootTimestamp> for u32 {
    fn to_inner(_: RootTimestamp) -> u32 { Default::default() }
    fn to_outer(self) -> RootTimestamp { RootTimestamp }
    fn summarize(_: <u32 as Timestamp>::Summary) -> RootSummary { RootSummary }
}

impl Refines<RootTimestamp> for i32 {
    fn to_inner(_: RootTimestamp) -> i32 { Default::default() }
    fn to_outer(self) -> RootTimestamp { RootTimestamp }
    fn summarize(_: <i32 as Timestamp>::Summary) -> RootSummary { RootSummary }
}

impl Refines<RootTimestamp> for () {
    fn to_inner(_: RootTimestamp) -> () { Default::default() }
    fn to_outer(self) -> RootTimestamp { RootTimestamp }
    fn summarize(_: <() as Timestamp>::Summary) -> RootSummary { RootSummary }
}


use std::time::Duration;
impl Refines<RootTimestamp> for Duration {
    fn to_inner(_: RootTimestamp) -> Duration { Default::default() }
    fn to_outer(self) -> RootTimestamp { RootTimestamp }
    fn summarize(_: <Duration as Timestamp>::Summary) -> RootSummary { RootSummary }
}