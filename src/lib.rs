#![forbid(unsafe_code, clippy::pedantic)]
#![deny(warnings)]

use std::cell::RefCell;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::ops::Add;
use std::path::Path;
use std::str::FromStr;

use crate::helpers::{borrow_read_line, map_string_to_pair, open_file, reduction_functor};
use crate::number_iterator::NumberIterator;
use crate::types::{IoResult, MainIter, MapStringToPair, OuterPairIter, Pair, PairOfPairs};

mod helpers;
mod number_iterator;
mod types;

/// # Errors
/// * When `a` is below zero.
/// * When couldn't open the file.
/// * When no pair satisfies the equation.
/// * When there are no integers that could be parsed from the file.
/// # Panics
/// * When couldn't seek into the beginning of the file.
/// # Inherited panics
/// * When couldn't read from the file.
pub fn iterators_main<N, P>(a: N, path: P) -> Result<(usize, usize), &'static str>
where
    N: FromStr + Add<Output = N> + Ord + Default + Copy,
    P: AsRef<Path>,
{
    if a < N::default() {
        return Err("Passed argument for 'A' is below zero!");
    }

    // Using `RefCell` instead of `Rc`/`Arc` with a semaphore is possible,
    // because access doesn't occur in parallel.
    let inner_file_handle: RefCell<BufReader<File>> = RefCell::new(open_file(&path)?);

    // Can't describe specific type;
    // `impl Trait` is not stable in bindings.
    let inner_pair_map: _ = |pair_i: Pair<N>| {
        inner_file_handle
            .borrow_mut()
            .seek(SeekFrom::Start(0))
            .expect("Couldn't seek into the beginning of the file!");

        map_string_to_pair(borrow_read_line(&inner_file_handle))
            .map(move |pair_j: Pair<N>| (pair_i, pair_j))
    };

    let outer_pair_iter: OuterPairIter = open_file(&path)?
        .lines()
        .map(|result: IoResult<String>| result.expect("Couldn't read from the file!"));

    // Main iterator; Acts as `for` loop with a nested one.
    //
    // Can't describe specific type;
    // `impl Trait` is not stable in bindings.
    let iter: MainIter<N, _, _, _> = map_string_to_pair(outer_pair_iter).flat_map(inner_pair_map);

    // Using `Cell` instead of `AtomicBool` to lower runtime costs since it's single threaded.
    let mut non_empty: bool = false;

    if let Some((line_i, line_j)) = iter
        .inspect(|_| non_empty = true)
        .filter(|&((_, number_i), (_, number_j)): &PairOfPairs<N>| number_j == number_i + a)
        .reduce(reduction_functor)
        .map(|((line_i, _), (line_j, _)): PairOfPairs<N>| {
            (line_i + 1, line_j + 1) // Map item index to line number.
        })
    {
        Ok((line_i, line_j))
    } else {
        Err(if non_empty {
            "No pair satisfies the equation: J = I + A"
        } else {
            "No integer values loaded from file!"
        })
    }
}
