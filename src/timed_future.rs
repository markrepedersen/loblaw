use {
    futures::Future,
    pin_project::pin_project,
    std::time::{Duration, Instant},
    std::{
        pin::Pin,
        task::{Context, Poll},
    },
};

/// Code that will time how long it takes to resolve a future, from poll time to completion time.
/// Taken from: https://stackoverflow.com/questions/53291554/whats-a-clean-way-to-get-how-long-a-future-takes-to-resolve
#[pin_project]
pub struct Timed<Fut, F>
where
    Fut: Future,
    F: FnMut(&Fut::Output, Duration),
{
    #[pin]
    inner: Fut,
    f: F,
    start: Option<Instant>,
}

impl<Fut, F> Future for Timed<Fut, F>
where
    Fut: Future,
    F: FnMut(&Fut::Output, Duration),
{
    type Output = Fut::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let this = self.project();
        let start = this.start.get_or_insert_with(Instant::now);

        match this.inner.poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(v) => {
                let elapsed = start.elapsed();
                (this.f)(&v, elapsed);

                Poll::Ready(v)
            }
        }
    }
}

pub trait TimedExt: Sized + Future {
    fn timed<F>(self, f: F) -> Timed<Self, F>
    where
        F: FnMut(&Self::Output, Duration),
    {
        Timed {
            inner: self,
            f,
            start: None,
        }
    }
}

impl<F: Future> TimedExt for F {}
