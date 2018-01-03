
pub trait IteratorMapOkExt {
    fn map_ok<F, T, U, E>(self, f: F) -> iterators::MapOk<Self, F>
            where Self: Sized + Iterator<Item = Result<T, E>>,
                  F: Fn(T) -> U {
        iterators::MapOk { inner: self, function: f }
    }

    fn flat_map_ok<F, T, U, E, G>(self, f: F) -> iterators::FlatMapOk<Self, F>
            where Self: Sized + Iterator<Item = Result<T, E>>,
                  F: Fn(T) -> Result<U, G>,
                  E: Into<G> {
        iterators::FlatMapOk { inner: self, function: f }
    }
}

impl<I, T, E> IteratorMapOkExt for I
        where I: Sized + Iterator<Item = Result<T, E>> {}

pub mod iterators {
    pub struct MapOk<I, F> {
        pub(super) inner: I,
        pub(super) function: F,
    }

    impl<I, F, T, U, E> Iterator for MapOk<I, F>
            where I: Sized + Iterator<Item = Result<T, E>>,
                  F: Fn(T) -> U {
        type Item = Result<U, E>;

        fn next(&mut self) -> Option<Self::Item> {
            self.inner.next().map(|r| r.map(|v| (self.function)(v)))
        }
    }

    pub struct FlatMapOk<I, F> {
        pub(super) inner: I,
        pub(super) function: F,
    }

    impl<I, F, T, U, E, G> Iterator for FlatMapOk<I, F>
            where I: Sized + Iterator<Item = Result<T, E>>,
                  F: Fn(T) -> Result<U, G>,
                  E: Into<G> {
        type Item = Result<U, G>;

        fn next(&mut self) -> Option<Self::Item> {
            self.inner.next().map(
                |r| match r {
                    Ok(v) => (self.function)(v),
                    Err(e) => Err(e.into()),
                })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::ParseIntError;

    #[test]
    fn map_ok_can_be_called() {
        let source = vec!["1", "no"];

        let result = source.iter().map(|s| s.parse::<i32>())
            .map_ok(|i| i + 5).collect::<Vec<_>>();

        assert_eq!(2, result.len());
        assert_eq!(6, *result[0].as_ref().unwrap());
        assert!(result[1].is_err());
    }

    #[derive(Debug)]
    enum FlatMapOkTestError {
        Parse(ParseIntError),
        DivisionByZero,
    }

    use self::FlatMapOkTestError::*;

    impl From<ParseIntError> for FlatMapOkTestError {
        fn from(e: ParseIntError) -> Self {
            FlatMapOkTestError::Parse(e)
        }
    }

    #[test]
    fn flat_map_ok_can_be_called() {
        let source = vec!["0", "2", "no"];

        let result = source.iter().map(|s| s.parse::<i32>())
            .flat_map_ok(|i| 10_i32.checked_div(i).ok_or(DivisionByZero))
            .collect::<Vec<_>>();

        assert_eq!(3, result.len());
        assert!(match result[0] { Err(DivisionByZero) => true, _ => false });
        assert_eq!(5, *result[1].as_ref().unwrap());
        assert!(match result[2] { Err(Parse(_)) => true, _ => false });
    }
}
