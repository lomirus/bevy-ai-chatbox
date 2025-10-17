#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ai;

use std::fs;

use bevy::{
    feathers::{
        FeathersPlugins,
        controls::{ButtonProps, ButtonVariant, button},
        dark_theme::create_dark_theme,
        theme::{ThemedText, UiTheme},
    },
    prelude::*,
    ui::Pressed,
};
use crossbeam_channel::Receiver;
use futures::{StreamExt, pin_mut};

use ai::{AiPlugin, Config};
use deepseek_api::Model;

use crate::ai::Dialog;

const DEFAULT_FONT_PATH: &str = "assets/fonts/NotoSansSC-Regular.ttf";

#[derive(Resource, Deref)]
struct TokioRuntime(tokio::runtime::Runtime);

#[derive(Resource)]
struct MessageReceiver(Receiver<String>);

#[derive(Component)]
struct SendButton;

#[derive(Component)]
struct MessageText;

fn ui() -> impl Bundle {
    (
        Node {
            flex_direction: FlexDirection::Column,
            overflow: Overflow::scroll_y(),
            ..default()
        },
        children![
            (
                Node {
                    width: Val::Vw(100.0),
                    ..default()
                },
                children![(MessageText, Text::new(""))]
            ),
            (
                Node {
                    width: Val::Px(200.0),
                    ..default()
                },
                children![(
                    SendButton,
                    button(
                        ButtonProps {
                            variant: ButtonVariant::Primary,
                            ..default()
                        },
                        (),
                        Spawn((Text::new("Start"), ThemedText))
                    ),
                )]
            )
        ],
    )
}

fn on_button_click(
    event: On<Add, Pressed>,
    mut button_query: Query<(), With<SendButton>>,
    tokio_runtime: Res<TokioRuntime>,
    config: Res<Config>,
    dialog: Res<Dialog>,
    mut commands: Commands,
) {
    if button_query.get_mut(event.event_target()).is_ok() {
        let client = deepseek_api::Client::new(Model::DeepSeekChat, &config.api_key);
        let messages = dialog.clone();
        let (tx, rx) = crossbeam_channel::unbounded();
        commands.insert_resource(MessageReceiver(rx));

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

fn update_text(
    mut text_query: Query<&mut Text, With<MessageText>>,
    message_receiver: Option<ResMut<MessageReceiver>>,
) {
    let mut text = text_query.single_mut().unwrap();
    if let Some(receiver) = message_receiver {
        for from_stream in receiver.0.try_iter() {
            text.0 += &from_stream;
        }
    }
}

fn setup_ui(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn(ui());
}

fn main() {
    let runtime = tokio::runtime::Runtime::new().unwrap();

    let mut app = App::new();
    app.add_plugins((DefaultPlugins, FeathersPlugins, AiPlugin));

    let font_data = fs::read(DEFAULT_FONT_PATH).unwrap();
    let asset = Font::try_from_bytes(font_data).unwrap();
    let mut assets = app.world_mut().resource_mut::<Assets<_>>();
    assets.insert(AssetId::default(), asset).unwrap();

    app.insert_resource(UiTheme(create_dark_theme()))
        .insert_resource(TokioRuntime(runtime))
        .add_systems(Startup, setup_ui)
        .add_systems(FixedUpdate, update_text)
        .add_observer(on_button_click)
        .run();
}
