
use std::thread;

use futures::{
    SinkExt,
    channel::{
        mpsc, 
        oneshot,
    },
    executor::{
        block_on,
        block_on_stream,
    },
};

const CHANNEL_BUFFER_SIZE: usize = 1024;

pub struct Dispatch<Input, Output> {
    sender: mpsc::Sender<Box<Message<Input, Output>>>,
}

impl<Input, Output> Dispatch<Input, Output> 
where Input: Send + Sync + 'static, 
      Output: Send + Sync + 'static,
{
    pub fn new<F>(handle: F) -> Self 
    where F: Fn(Input) -> Output, 
          F: Send + 'static,
    {
        let (tx, rx) = mpsc::channel::<Box<Message<Input, Output>>>(CHANNEL_BUFFER_SIZE);
        thread::spawn(move || {
            let mut stream = block_on_stream(rx);
            while let Some(mut message) = stream.next() {
                let output = handle(message.take_value());
                message.reply(output);
            }
        });
        let dispatch = Dispatch {
            sender: tx,
        };
        dispatch
    }

    pub async fn invoke(&self, input: Input) -> Output {
        let (message, receiver) = Message::new(input);
        let mut sender = self.sender.clone();
        sender.send(message)
            .await
            .unwrap();
        receiver
            .await
            .unwrap()
    }

    pub fn invoke_sync(&self, input: Input) -> Output {
        block_on(self.invoke(input))
    }
}

struct Message<Input, Output> {
    value: Option<Input>,
    reply: Option<oneshot::Sender<Output>>,
}

impl<Input, Output> Message<Input, Output> {
    fn new(value: Input) -> (Box<Self>, oneshot::Receiver<Output>) {
        let (tx, rx) = oneshot::channel();
        let message = Message {
            value: Some(value),
            reply: Some(tx)
        };
        let message = Box::new(message);
        (message, rx)
    }

    fn reply(&mut self, value: Output) {
        let sender = self.reply.take().unwrap();
        sender.send(value)
            .map_err(|_| ())
            .expect("could not send reply");
    }

    fn take_value(&mut self) -> Input {
        self.value.take().unwrap()
    }
}
