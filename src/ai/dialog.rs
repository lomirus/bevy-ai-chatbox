use std::{fs, io::ErrorKind};

use bevy::prelude::*;
use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Resource, Deref)]
pub(crate) struct Dialog(pub(crate) Vec<deepseek_api::Message>);

impl Default for Dialog {
    fn default() -> Self {
        Self(vec![
            deepseek_api::Message::system("你是一个智能助手。"),
            deepseek_api::Message::user("你好！"),
        ])
    }
}

impl Dialog {
    pub(crate) fn get_or_init() -> Dialog {
        match fs::read("dialog.ron") {
            Ok(file) => {
                let dialog_str = String::from_utf8(file).unwrap();
                let dialog: Vec<deepseek_api::Message> = ron::from_str(&dialog_str).unwrap();
                Dialog(dialog)
            }
            Err(err) => match err.kind() {
                ErrorKind::NotFound => {
                    let dialog: Dialog = Dialog::default();
                    let dialog_str =
                        ron::ser::to_string_pretty(&dialog.0, PrettyConfig::default()).unwrap();
                    fs::write("dialog.ron", dialog_str).unwrap();
                    dialog
                }
                _ => panic!("{err}"),
            },
        }
    }
}
