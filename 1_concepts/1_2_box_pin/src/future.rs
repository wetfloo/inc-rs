use std::{future::Future, pin::Pin, task::Poll, time::Instant};

struct MeasureableFuture<Fut: Future> {
    pub inner_future: Pin<Box<Fut>>,
    pub started_at: Option<Instant>,
}

impl<Fut: Future> MeasureableFuture<Fut> {
    pub fn new(fut: Fut) -> Self {
        let pin = Box::pin(fut);
        MeasureableFuture {
            inner_future: pin,
            started_at: None,
        }
    }
}

impl<Fut: Future> Future for MeasureableFuture<Fut> {
    type Output = Fut::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        // Because `Unpin` is `auto trait`, and all members of `MeasurableFuture`
        // impelement `Unpin`, we don't need to provide `Unpin` trait bound.
        // If not for boxing, I would have to add `Unpin` trait bound to the `impl` block
        let this = self.get_mut();
        // Only add start time if it's not present yet
        let started_at = this.started_at.get_or_insert(Instant::now());

        let poll_result = this.inner_future.as_mut().poll(cx);
        match poll_result {
            Poll::Ready(_) => {
                let nanos = started_at.elapsed().as_nanos();
                println!("Future took {} nanoseconds to complete", nanos);
            }
            Poll::Pending => (),
        }

        // Done polling, transparently return the result
        poll_result
    }
}
