use super::comparer::ComparerTrait;
use super::hasher::HasherTrait;

pub trait HasherAndComparerTrait: HasherTrait + ComparerTrait {}