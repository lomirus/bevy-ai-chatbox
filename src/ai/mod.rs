mod config;
mod dialog;

use bevy::prelude::*;

pub(crate) use config::Config;
pub(crate) use dialog::Dialog;

pub(crate) struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        let config = Config::get_or_init();
        let dialog = Dialog::get_or_init();
        app.insert_resource(config);
        app.insert_resource(dialog);
    }
}


