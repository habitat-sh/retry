//! Asynchronous implementation of `retry` and `retry_with_index`. This module is enabled with the
//! `"asynchronous"` feature.

use crate::{Error, OperationResult};
use std::{future::Future, time::Duration};
use tokio::time;

/// Retry the given asynchronous operation until it succeeds, or until the given `Duration`
/// iterator ends.
pub async fn retry<I, O, R, E, OR, F>(iterable: I, mut operation: O) -> Result<R, Error<E>>
where
    I: IntoIterator<Item = Duration>,
    O: FnMut() -> F,
    OR: Into<OperationResult<R, E>>,
    F: Future<Output = OR>,
{
    retry_with_index(iterable, |_| operation()).await
}

/// Retry the given asynchronous operation until it succeeds, or until the given `Duration`
/// iterator ends, with each iteration of the operation receiving the number of the attempt as an
/// argument.
pub async fn retry_with_index<I, O, R, E, OR, F>(
    iterable: I,
    mut operation: O,
) -> Result<R, Error<E>>
where
    I: IntoIterator<Item = Duration>,
    O: FnMut(u64) -> F,
    OR: Into<OperationResult<R, E>>,
    F: Future<Output = OR>,
{
    let mut iterator = iterable.into_iter();
    let mut current_try = 1;
    let mut total_delay = Duration::default();

    loop {
        match operation(current_try).await.into() {
            OperationResult::Ok(value) => return Ok(value),
            OperationResult::Retry(error) => {
                if let Some(delay) = iterator.next() {
                    time::sleep(delay).await;
                    current_try += 1;
                    total_delay += delay;
                } else {
                    return Err(Error {
                        error,
                        total_delay,
                        tries: current_try,
                    });
                }
            }
            OperationResult::Err(error) => {
                return Err(Error {
                    error,
                    total_delay,
                    tries: current_try,
                });
            }
        }
    }
}

/// Retry a future with the syntax
/// `retry_future!(IntoIterator<Item = Duration>, Future<Output = Into<OperationResult<R, E>>>)`
///
/// This is a workaround for cases when using `retry` is not possible because it is not possible
/// to return a value capturing a reference from a closure [1].
///
/// [1] https://github.com/rustasync/team/issues/19
#[macro_export]
macro_rules! retry_future {
    ($delays:expr, $future:expr) => {
        async {
            let mut iterator = $delays.into_iter();
            let mut current_try = 1;
            let mut total_delay = ::std::time::Duration::default();

            loop {
                match $future.await.into() {
                    $crate::OperationResult::Ok(value) => return Ok(value),
                    $crate::OperationResult::Retry(error) => {
                        if let Some(delay) = iterator.next() {
                            ::tokio::time::sleep(delay).await;
                            current_try += 1;
                            total_delay += delay;
                        } else {
                            return Err($crate::Error {
                                error,
                                total_delay,
                                tries: current_try,
                            });
                        }
                    }
                    $crate::OperationResult::Err(error) => {
                        return Err($crate::Error {
                            error,
                            total_delay,
                            tries: current_try,
                        });
                    }
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use futures::future;
    use rand::Rng;
    use std::{sync::Arc, time::Duration};
    use tokio;

    use super::{retry, retry_with_index};
    use crate::{
        delay::{Exponential, Fixed, NoDelay, Range},
        opresult::OperationResult,
        retry_future, Error,
    };

    #[tokio::test]
    async fn succeeds_with_infinite_retries() {
        let mut collection = vec![1, 2, 3, 4, 5].into_iter();

        let value = retry(NoDelay, || match collection.next() {
            Some(n) if n == 5 => future::ready(Ok(n)),
            Some(_) => future::ready(Err("not 5")),
            None => future::ready(Err("not 5")),
        })
        .await
        .unwrap();

        assert_eq!(value, 5);
    }

    #[tokio::test]
    async fn succeeds_with_maximum_retries() {
        let mut collection = vec![1, 2].into_iter();

        let value = retry(NoDelay.take(1), || match collection.next() {
            Some(n) if n == 2 => future::ready(Ok(n)),
            Some(_) => future::ready(Err("not 2")),
            None => future::ready(Err("not 2")),
        })
        .await
        .unwrap();

        assert_eq!(value, 2);
    }

    #[tokio::test]
    async fn fails_after_last_try() {
        let mut collection = vec![1].into_iter();

        let res = retry(NoDelay.take(1), || match collection.next() {
            Some(n) if n == 2 => future::ready(Ok(n)),
            Some(_) => future::ready(Err("not 2")),
            None => future::ready(Err("not 2")),
        })
        .await;

        assert_eq!(
            res,
            Err(Error {
                error: "not 2",
                tries: 2,
                total_delay: Duration::from_millis(0)
            })
        );
    }

    #[tokio::test]
    async fn fatal_errors() {
        let mut collection = vec![1].into_iter();

        let res = retry(NoDelay.take(2), || match collection.next() {
            Some(n) if n == 2 => future::ready(OperationResult::Ok(n)),
            Some(_) => future::ready(OperationResult::Err("no retry")),
            None => future::ready(OperationResult::Err("not 2")),
        })
        .await;

        assert_eq!(
            res,
            Err(Error {
                error: "no retry",
                tries: 1,
                total_delay: Duration::from_millis(0)
            })
        );
    }

    #[tokio::test]
    async fn succeeds_with_fixed_delay() {
        let mut collection = vec![1, 2].into_iter();

        let value = retry(Fixed::from_millis(1), || match collection.next() {
            Some(n) if n == 2 => future::ready(Ok(n)),
            Some(_) => future::ready(Err("not 2")),
            None => future::ready(Err("not 2")),
        })
        .await
        .unwrap();

        assert_eq!(value, 2);
    }

    #[tokio::test]
    async fn succeeds_with_exponential_delay() {
        let mut collection = vec![1, 2].into_iter();

        let value = retry(Exponential::from_millis(1), || match collection.next() {
            Some(n) if n == 2 => future::ready(Ok(n)),
            Some(_) => future::ready(Err("not 2")),
            None => future::ready(Err("not 2")),
        })
        .await
        .unwrap();

        assert_eq!(value, 2);
    }

    #[tokio::test]
    async fn succeeds_with_ranged_delay() {
        let mut collection = vec![1, 2].into_iter();

        let value = retry(Range::from_millis_exclusive(1, 10), || {
            match collection.next() {
                Some(n) if n == 2 => future::ready(Ok(n)),
                Some(_) => future::ready(Err("not 2")),
                None => future::ready(Err("not 2")),
            }
        })
        .await
        .unwrap();

        assert_eq!(value, 2);
    }

    #[tokio::test]
    async fn succeeds_with_index() {
        let mut collection = vec![1, 2, 3].into_iter();

        let value = retry_with_index(NoDelay, |current_try| match collection.next() {
            Some(n) if n == current_try => future::ready(Ok(n)),
            Some(_) => future::ready(Err("not current_try")),
            None => future::ready(Err("not current_try")),
        })
        .await
        .unwrap();

        assert_eq!(value, 1);
    }

    #[tokio::test]
    async fn succeeds_with_index_async_closure() {
        let collection = Arc::new(vec![0, 2, 3]);
        let mut i = 0;
        let value = retry_with_index(NoDelay, |current_try| {
            let collection_copy = Arc::clone(&collection);
            let f = async move {
                match collection_copy.get(i).copied() {
                    Some(n) if n == current_try => Ok(n),
                    Some(_) => Err("not current_try"),
                    None => Err("not current_try"),
                }
            };
            i += 1;
            f
        })
        .await
        .unwrap();

        assert_eq!(value, 2);
    }

    #[tokio::test]
    async fn succeeds_with_index_async_fn() {
        async fn op(current_try: u64) -> Result<(), &'static str> {
            if current_try == 2 {
                Ok(())
            } else {
                Err("not 2")
            }
        }
        let value = retry_with_index(NoDelay, op).await;

        assert!(value.is_ok());
    }

    #[tokio::test]
    async fn retry_future_macro() {
        async fn random() -> Result<u8, &'static str> {
            let mut rng = rand::thread_rng();
            let n = rng.gen();
            if n < 100 {
                Ok(n)
            } else {
                Err("not < 100")
            }
        }
        let value = retry_future!(NoDelay, random()).await.unwrap();

        assert!(value < 100);
    }
}
