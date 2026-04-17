use iced::futures::SinkExt;
use iced::futures::stream::{BoxStream, StreamExt};
use iced::{Subscription, stream};
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc};

use crate::service::gui::structs::TaskId;

// 1. Define an enum to hold either receiver type
#[derive(Debug)]
enum ReceiverType<T> {
    Bounded(mpsc::Receiver<T>),
    Unbounded(mpsc::UnboundedReceiver<T>),
}

#[derive(Debug)]
pub struct ReceiverHandle<T> {
    id: TaskId,
    // Update to use the enum
    rx: Arc<Mutex<Option<ReceiverType<T>>>>,
}

struct WatchContext<T, M> {
    id: TaskId,
    rx: Arc<Mutex<Option<ReceiverType<T>>>>,
    on_data: Arc<dyn Fn(TaskId, T) -> M + Send + Sync>,
    on_finish: Arc<dyn Fn(TaskId) -> M + Send + Sync + 'static>,
}

impl<T, M> Hash for WatchContext<T, M> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<T, M> PartialEq for WatchContext<T, M> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T, M> Eq for WatchContext<T, M> {}

impl<T> Clone for ReceiverHandle<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            rx: self.rx.clone(),
        }
    }
}

impl<T> ReceiverHandle<T>
where
    T: Send + 'static,
{
    /// Constructor for standard bounded receiver
    pub fn new(id: TaskId, rx: mpsc::Receiver<T>) -> Self {
        Self {
            id,
            rx: Arc::new(Mutex::new(Some(ReceiverType::Bounded(rx)))),
        }
    }

    /// New constructor for unbounded receiver
    pub fn new_unbounded(id: TaskId, rx: mpsc::UnboundedReceiver<T>) -> Self {
        Self {
            id,
            rx: Arc::new(Mutex::new(Some(ReceiverType::Unbounded(rx)))),
        }
    }

    pub fn watch<M>(
        &self,
        on_data: impl Fn(TaskId, T) -> M + Send + Sync + 'static,
        on_finish: impl Fn(TaskId) -> M + Send + Sync + 'static,
    ) -> Subscription<M>
    where
        M: 'static + Send,
    {
        let context = WatchContext {
            id: self.id,
            rx: self.rx.clone(),
            on_data: Arc::new(on_data),
            on_finish: Arc::new(on_finish),
        };
        Subscription::run_with(context, stream_builder::<T, M>)
    }

    pub fn id(&self) -> TaskId {
        self.id
    }
    pub fn map<U, F>(self, f: F) -> ReceiverHandle<U>
    where
        U: Send + 'static,
        F: Fn(T) -> U + Send + Sync + 'static,
    {
        // Create a new channel for the mapped type
        let (tx, rx) = mpsc::unbounded_channel::<U>();
        let old_rx_mutex = self.rx.clone();

        // Spawn a background "translator" task
        tokio::spawn(async move {
            let mut locked = old_rx_mutex.lock().await;
            if let Some(mut internal_rx) = locked.take() {
                loop {
                    // Pull from the original receiver
                    let next = match &mut internal_rx {
                        ReceiverType::Bounded(r) => r.recv().await,
                        ReceiverType::Unbounded(r) => r.recv().await,
                    };

                    match next {
                        Some(val) => {
                            // Apply the mapping function and send to the new channel
                            if tx.send(f(val)).is_err() {
                                break;
                            }
                        }
                        None => break, // Original channel closed
                    }
                }
            }
        });

        ReceiverHandle::new_unbounded(self.id, rx)
    }
}

fn stream_builder<T, M>(ctx: &WatchContext<T, M>) -> BoxStream<'static, M>
where
    T: Send + 'static,
    M: 'static + Send,
{
    let id = ctx.id;
    let safe_rx = ctx.rx.clone();
    let on_data = ctx.on_data.clone();
    let on_finish = ctx.on_finish.clone();

    stream::channel::<M>(
        100,
        move |mut output: iced::futures::channel::mpsc::Sender<M>| async move {
            let mut rx_opt = safe_rx.lock().await.take();

            let mut rx = match rx_opt {
                Some(r) => r,
                None => {
                    // This handles cases where the subscription might be
                    // re-polled after the receiver is already gone
                    std::future::pending().await
                }
            };

            // 2. Loop and match on the receiver type inside the loop
            loop {
                let msg = match &mut rx {
                    ReceiverType::Bounded(r) => r.recv().await,
                    ReceiverType::Unbounded(r) => r.recv().await,
                };

                if let Some(data) = msg {
                    let _ = output.send(on_data(id, data)).await;
                } else {
                    // Channel closed
                    break;
                }
            }

            let _ = output.send(on_finish(id)).await;
        },
    )
    .boxed()
}
