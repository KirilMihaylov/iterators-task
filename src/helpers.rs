use std::cell::RefCell;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::{from_fn, FromFn};
use std::ops::Add;
use std::path::Path;

use crate::{MapStringToPair, NumberIterator, PairOfPairs};

/// Opens file pointed by `path` for reading.
/// # Errors
/// * When file couldn't be opened.
pub(crate) fn open_file<P>(path: P) -> Result<BufReader<File>, &'static str>
where
    P: AsRef<Path>,
{
    if let Ok(file) = File::open(path) {
        Ok(BufReader::new(file))
    } else {
        Err("Couldn't open the file for reading!")
    }
}

pub(crate) fn map_string_to_pair<I, N>(line_iter: I) -> MapStringToPair<I, N>
where
    I: NumberIterator<N>,
{
    line_iter
        .numbers()
        .enumerate()
        .filter_map(|(line, number): (usize, Option<N>)| Some((line, number?)))
}

#[cfg(test)]
#[test]
fn map_string_to_pair_test() {
    type N = i32;

    const TABLE: &[&str] = &["1", "a", " 3 ", "-4", "5b"];

    assert_eq!(
        map_string_to_pair(TABLE.iter())
            .collect::<Vec<(_, N)>>()
            .as_slice(),
        &[(0, 1), (2, 3), (3, -4)]
    );
}

/// Creates an iterator that reads line by line and returns them on demand.
/// # Panics
/// * When couldn't read from the buffer.
pub(crate) fn borrow_read_line<R>(buf: &RefCell<R>) -> FromFn<impl FnMut() -> Option<String> + '_>
where
    R: BufRead,
{
    from_fn(|| {
        let mut string_buf: String = String::new();

        if buf
            .borrow_mut()
            .read_line(&mut string_buf)
            .expect("Couldn't read from file!")
            == 0
        {
            None
        } else {
            Some(string_buf)
        }
    })
}

#[cfg(test)]
#[test]
fn borrow_read_line_test() {
    let mut bytes: &[u8] = b"Hello \nHello World\nHello World!";

    let buf: RefCell<BufReader<&mut &[u8]>> = RefCell::new(BufReader::new(&mut bytes));

    assert_eq!(
        borrow_read_line(&buf).collect::<Vec<_>>().as_slice(),
        &["Hello \n", "Hello World\n", "Hello World!"]
    );
}

/// Gets the maximal pair with smaller `i` when pairs are equal.
pub(crate) fn reduction_functor<N>(
    left @ ((_, left_i), (_, left_j)): PairOfPairs<N>,
    right @ ((_, right_i), (_, right_j)): PairOfPairs<N>,
) -> PairOfPairs<N>
where
    N: Add<Output = N> + Ord + Copy,
{
    match (left_i + left_j).cmp(&(right_i + right_j)) {
        Ordering::Greater => left,
        Ordering::Equal if left_i < right_i => left,
        _ => right,
    }
}

#[cfg(test)]
#[test]
fn reduction_functor_test() {
    type N = i32;

    const I5J7: PairOfPairs<N> = ((0, 5), (1, 7));

    const I6J8: PairOfPairs<N> = ((0, 6), (1, 8));

    const I6J6: PairOfPairs<N> = ((0, 6), (1, 6));

    assert_eq!(reduction_functor(I5J7, I6J8), I6J8);

    assert_eq!(reduction_functor(I6J8, I5J7), I6J8);

    assert_eq!(reduction_functor(I5J7, I6J6), I5J7);

    assert_eq!(reduction_functor(I6J6, I5J7), I5J7);

    assert_eq!(reduction_functor(I6J8, I6J6), I6J8);

    assert_eq!(reduction_functor(I6J6, I6J8), I6J8);
}
