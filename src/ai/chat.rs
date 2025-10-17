use bevy::prelude::*;
use crossbeam_channel::Receiver;
use futures::{StreamExt, pin_mut};

use super::{Config, Dialog};
use deepseek_api::Model;

#[derive(Resource, Deref)]
pub(crate) struct TokioRuntime(pub(crate) tokio::runtime::Runtime);

#[derive(Resource)]
pub(crate) struct StreamReceiver(Receiver<String>);

#[derive(Message, Deref)]
pub(crate) struct SendMessage(String);

#[derive(Message, Deref)]
pub(crate) struct ReceiveMessage(pub(crate) String);

#[derive(Resource)]
pub(crate) struct IsChatting;

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
    is_chatting: Option<Res<IsChatting>>,
) {
    if is_chatting.is_some() {
        return;
    }
    if let Some(message) = send_message.read().next() {
        let client = deepseek_api::Client::new(Model::DeepSeekChat, &config.api_key);
        dialog.0.push(deepseek_api::Message::user(&message));
        let messages = dialog.clone();
        let (tx, rx) = crossbeam_channel::unbounded();
        commands.insert_resource(StreamReceiver(rx));

        tokio_runtime.spawn(async move {
            let stream = client.streaming_chat(messages).await;
            pin_mut!(stream);
            while let Some(chunk) = stream.next().await {
                assert_eq!(chunk.choices.len(), 1);
                tx.send(chunk.choices[0].delta.content.clone()).unwrap();
            }
        });
    }
}

pub(crate) fn read_stream(
    stream_receiver: Option<ResMut<StreamReceiver>>,
    mut receive_message: MessageWriter<ReceiveMessage>,
) {
    if let Some(receiver) = stream_receiver {
        for chunk in receiver.0.try_iter() {
            receive_message.write(ReceiveMessage(chunk));
        }
    }
}
