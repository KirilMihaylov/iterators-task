use std::iter::Map;
use std::str::FromStr;

pub trait NumberIterator<N>
where
    Self: Iterator,
{
    type OutputIter: Iterator<Item = Option<N>>;

    fn numbers(self) -> Self::OutputIter
    where
        Self: Sized;
}

impl<I, N> NumberIterator<N> for I
where
    I: Iterator,
    I::Item: AsRef<str>,
    N: FromStr,
{
    type OutputIter = Map<Self, fn(I::Item) -> Option<N>>;

    fn numbers(self) -> Self::OutputIter
    where
        Self: Sized,
    {
        self.map(|value: I::Item| value.as_ref().trim().parse().ok())
    }
}

#[cfg(test)]
#[test]
fn number_iterator_test() {
    type N = i32;

    const TABLE: &[&str] = &["1", "a", " 3 ", "-4", "5b"];

    assert_eq!(
        TABLE
            .iter()
            .numbers()
            .collect::<Vec<Option<N>>>()
            .as_slice(),
        &[Some(1), None, Some(3), Some(-4), None]
    );
}
