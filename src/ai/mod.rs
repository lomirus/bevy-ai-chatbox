mod chat;
mod config;
mod dialog;

pub(crate) use chat::{ReceiveMessage, SendMessage};
pub(crate) use config::Config;
pub(crate) use dialog::Dialog;

use bevy::prelude::*;

use chat::{TokioRuntime, on_send_message, read_stream};

pub(crate) struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        let config = Config::get_or_init();
        let dialog = Dialog::get_or_init();

        let runtime = tokio::runtime::Runtime::new().unwrap();

        app.insert_resource(config)
            .insert_resource(dialog)
            .insert_resource(TokioRuntime(runtime))
            .add_message::<SendMessage>()
            .add_message::<ReceiveMessage>()
            .add_systems(FixedUpdate, (on_send_message, read_stream));
    }
}
