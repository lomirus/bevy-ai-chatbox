use bevy::prelude::*;
use crossbeam_channel::Receiver;
use futures::pin_mut;

use super::{Config, Dialog};
use deepseek_api::AsyncIteratorNext;
use deepseek_api::{Delta, Model};

#[derive(Resource, Deref)]
pub(crate) struct TokioRuntime(pub(crate) tokio::runtime::Runtime);

#[derive(Resource)]
pub(crate) struct StreamReceiver(Receiver<ReceiveMessage>);

#[derive(Message, Clone, Deref)]
pub(crate) struct SendMessage(pub(crate) String);

#[derive(Message, Clone)]
pub(crate) enum ReceiveMessage {
    Content(String),
    Finished,
}

impl SendMessage {
    pub(crate) fn new(message: &str) -> Self {
        Self(message.to_string())
    }
}

pub(crate) fn on_send_message(
    mut commands: Commands,
    mut send_message: MessageReader<SendMessage>,
    mut dialog: ResMut<Dialog>,
    tokio_runtime: Res<TokioRuntime>,
    config: Res<Config>,
    stream_receiver: Option<ResMut<StreamReceiver>>,
) {
    let is_chatting = stream_receiver.is_some();
    if is_chatting {
        return;
    }
    if let Some(message) = send_message.read().next() {
        let message = message.clone();
        let mut client = deepseek_api::Client::new(Model::DeepSeekChat, &config.api_key);

        dialog
            .0
            .push(deepseek_api::message::Message::user(&message));
        let (tx, rx) = crossbeam_channel::unbounded();
        commands.insert_resource(StreamReceiver(rx));

        tokio_runtime.spawn(async move {
            let stream = client.streaming_chat(&message).await;
            pin_mut!(stream);
            while let Some(delta) = stream.next().await {
                match delta {
                    Delta::Content { content, .. } => {
                        tx.send(ReceiveMessage::Content(content)).unwrap()
                    }
                    _ => unreachable!(),
                }
            }
            tx.send(ReceiveMessage::Finished)
        });
    }
}

pub(crate) fn read_stream(
    mut commands: Commands,
    stream_receiver: Option<ResMut<StreamReceiver>>,
    mut receive_message: MessageWriter<ReceiveMessage>,
) {
    if let Some(receiver) = stream_receiver {
        for chunk in receiver.0.try_iter() {
            receive_message.write(chunk.clone());
            if let ReceiveMessage::Finished = chunk {
                commands.remove_resource::<StreamReceiver>();
                break;
            }
        }
    }
}
