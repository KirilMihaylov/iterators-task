use std::fs::File;
use std::io::{BufReader, Lines, Result};
use std::iter::{Enumerate, FilterMap, FlatMap, FromFn, Map};

use crate::NumberIterator;

/// Represents the pair of line and it's respective number.
pub(crate) type Pair<N> = (usize, N);

/// Represents a pair of two line-number pairs.
pub(crate) type PairOfPairs<N> = (Pair<N>, Pair<N>);

pub(crate) type MapStringToPair<I, N> = FilterMap<
    Enumerate<<I as NumberIterator<N>>::OutputIter>,
    fn((usize, Option<N>)) -> Option<Pair<N>>,
>;

pub(crate) type OuterPairIter = Map<Lines<BufReader<File>>, fn(Result<String>) -> String>;

pub(crate) type MainIter<N, FromFnF, MapF, FlatMapF> = FlatMap<
    MapStringToPair<OuterPairIter, N>,
    Map<MapStringToPair<FromFn<FromFnF>, N>, MapF>,
    FlatMapF,
>;

pub(crate) type IoResult<T> = Result<T>;
