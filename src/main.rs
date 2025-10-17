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

use ai::{AiPlugin, ReceiveMessage, SendMessage};

const DEFAULT_FONT_PATH: &str = "assets/fonts/NotoSansSC-Regular.ttf";

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
    mut send_message: MessageWriter<SendMessage>,
) {
    if button_query.get_mut(event.event_target()).is_ok() {
        send_message.write(SendMessage::new("给我讲一个故事。"));
    }
}

fn update_text(
    mut receive_message: MessageReader<ReceiveMessage>,
    mut text_query: Query<&mut Text, With<MessageText>>,
) {
    let mut text = text_query.single_mut().unwrap();
    for message in receive_message.read() {
        text.0 += &message.0;
    }
}

fn setup_ui(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn(ui());
}

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, FeathersPlugins, AiPlugin));

    let font_data = fs::read(DEFAULT_FONT_PATH).unwrap();
    let asset = Font::try_from_bytes(font_data).unwrap();
    let mut assets = app.world_mut().resource_mut::<Assets<_>>();
    assets.insert(AssetId::default(), asset).unwrap();

    app.insert_resource(UiTheme(create_dark_theme()))
        .add_systems(Startup, setup_ui)
        .add_systems(Update, update_text)
        .add_observer(on_button_click)
        .run();
}
