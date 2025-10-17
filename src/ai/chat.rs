use bevy::prelude::*;
use crossbeam_channel::Receiver;
use futures::{StreamExt, pin_mut};

use super::{Config, Dialog};
use deepseek_api::{FinishReason, Model, streaming::Choice};

#[derive(Resource, Deref)]
pub(crate) struct TokioRuntime(pub(crate) tokio::runtime::Runtime);

#[derive(Resource)]
pub(crate) struct StreamReceiver(Receiver<Choice>);

#[derive(Message, Deref)]
pub(crate) struct SendMessage(String);

#[derive(Message, Deref)]
pub(crate) struct ReceiveMessage(pub(crate) String);

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
        let client = deepseek_api::Client::new(Model::DeepSeekChat, &config.api_key);
        dialog.0.push(deepseek_api::Message::user(message));
        let messages = dialog.clone();
        let (tx, rx) = crossbeam_channel::unbounded();
        commands.insert_resource(StreamReceiver(rx));

        tokio_runtime.spawn(async move {
            let stream = client.streaming_chat(messages).await;
            pin_mut!(stream);
            while let Some(chunk) = stream.next().await {
                assert_eq!(chunk.choices.len(), 1);
                tx.send(chunk.choices[0].clone()).unwrap();
            }
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
            match chunk.finish_reason {
                Some(FinishReason::Stop) => {
                    commands.remove_resource::<StreamReceiver>();
                    break;
                }
                _ => {
                    receive_message.write(ReceiveMessage(chunk.delta.content));
                }
            }
        }
    }
}
