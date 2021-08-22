use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use either::Either;
use pin_project::pin_project;

use super::{Filter, FilterBase, Internal};

impl<L, R> FilterBase for Either<L, R>
where
    L: Filter,
    R: Filter<Extract = L::Extract, Error = L::Error>,
{
    type Extract = L::Extract;
    type Error = L::Error;
    type Future = EitherFuture<L::Future, R::Future>;

    #[inline]
    fn filter(&self, _: Internal) -> Self::Future {
        match self {
            Either::Left(left) => EitherFuture::Left(left.filter(Internal)),
            Either::Right(right) => EitherFuture::Right(right.filter(Internal)),
        }
    }
}

#[allow(missing_debug_implementations)]
#[pin_project(project = EitherProj)]
pub enum EitherFuture<L, R> {
    Left(#[pin] L),
    Right(#[pin] R),
}

impl<L, R> Future for EitherFuture<L, R>
where
    L: Future,
    R: Future<Output = L::Output>,
{
    type Output = L::Output;

    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        match self.project() {
            EitherProj::Left(left) => left.poll(cx),
            EitherProj::Right(right) => right.poll(cx)
        }
    }
}
