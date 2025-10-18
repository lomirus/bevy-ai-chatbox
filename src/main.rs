#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ai;
mod ui_scroll;

use std::fs;

use bevy::{
    ecs::query::QuerySingleError,
    feathers::{
        FeathersPlugins,
        controls::{ButtonProps, ButtonVariant, button},
        dark_theme::create_dark_theme,
        theme::{ThemedText, UiTheme},
    },
    prelude::*,
    ui_widgets::{Activate, observe},
};

use ai::{AiPlugin, ReceiveMessage, SendMessage};
use ui_scroll::UiScrollPlugin;

const DEFAULT_FONT_PATH: &str = "assets/fonts/NotoSansSC-Regular.ttf";

#[derive(Component)]
struct SendButton;
#[derive(Component)]
enum MessageRole {
    System,
    User,
    Assistant,
}

fn message_box(role: MessageRole, content: String, is_streaming: bool) -> impl Bundle + use<> {
    use MessageRole::*;
    (
        Node {
            justify_content: match role {
                System => JustifyContent::Center,
                User => JustifyContent::End,
                Assistant => JustifyContent::Start,
            },
            ..default()
        },
        children![(
            Node {
                border: UiRect::all(px(2)),
                ..default()
            },
            BorderColor::all(match role {
                System => Srgba::hex("#E6A23C").unwrap(),
                User => Srgba::hex("#67C23A").unwrap(),
                Assistant => Srgba::hex("#409EFF").unwrap(),
            }),
            Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
                if is_streaming {
                    parent.spawn((Text::new(content), StreamingMessage));
                } else {
                    parent.spawn(Text::new(content));
                }
            })),
        )],
        role,
    )
}

#[derive(Component)]
struct StreamingMessage;

impl From<deepseek_api::Role> for MessageRole {
    fn from(value: deepseek_api::Role) -> Self {
        use deepseek_api::Role::*;
        match value {
            System => MessageRole::System,
            User => MessageRole::User,
            Assistant => MessageRole::Assistant,
            Tool => unreachable!(),
        }
    }
}

#[derive(Component)]
struct Dialog;

fn ui(messages: Vec<deepseek_api::Message>) -> impl Bundle {
    (
        Node {
            width: percent(100),
            height: percent(100),
            flex_direction: FlexDirection::Column,
            overflow: Overflow::scroll_y(),
            ..default()
        },
        children![
            (
                Dialog,
                Node {
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                Children::spawn(SpawnIter(messages.into_iter().map(|message| message_box(
                    message.role.into(),
                    message.content,
                    false
                ))))
            ),
            (
                Node {
                    width: px(200),
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
                    observe(
                        |_event: On<Activate>, mut send_message: MessageWriter<SendMessage>| {
                            send_message.write(SendMessage::new("给我讲一个故事。"));
                        }
                    )
                )]
            )
        ],
    )
}

fn update_send_message(
    mut send_message: MessageReader<SendMessage>,
    mut dialog: Query<Entity, With<Dialog>>,
    mut commands: Commands,
) {
    let dialog = dialog.single_mut().unwrap();

    for send_message in send_message.read() {
        let message_box = commands
            .spawn(message_box(
                MessageRole::User,
                send_message.0.clone(),
                false,
            ))
            .id();
        commands.entity(dialog).add_child(message_box);
    }
}

fn update_receive_message(
    mut receive_message: MessageReader<ReceiveMessage>,
    mut dialog: Query<Entity, With<Dialog>>,
    mut text_query: Query<(Entity, &mut Text), With<StreamingMessage>>,
    mut commands: Commands,
) {
    let dialog = dialog.single_mut().unwrap();
    let receive_message = receive_message.read();
    if receive_message.len() == 0 {
        return;
    }

    match text_query.single_mut() {
        Ok((entity, mut text)) => {
            for receive_message in receive_message {
                text.0 += &receive_message.content;
                if receive_message.finished {
                    commands.entity(entity).remove::<StreamingMessage>();
                }
            }
        }
        Err(QuerySingleError::NoEntities(_)) => {
            let mut text = String::new();
            let mut is_finished = false;
            for receive_message in receive_message {
                text += &receive_message.content;
                if receive_message.finished {
                    is_finished = true;
                }
            }
            let message_box = commands
                .spawn(message_box(MessageRole::Assistant, text, !is_finished))
                .id();
            commands.entity(dialog).add_child(message_box);
        }
        _ => unreachable!(),
    };
}

fn setup_ui(mut commands: Commands, messages: Res<ai::Dialog>) {
    commands.spawn(Camera2d);
    commands.spawn(ui(messages.0.clone()));
}

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, FeathersPlugins, AiPlugin, UiScrollPlugin));

    let font_data = fs::read(DEFAULT_FONT_PATH).unwrap();
    let asset = Font::try_from_bytes(font_data).unwrap();
    let mut assets = app.world_mut().resource_mut::<Assets<_>>();
    assets.insert(AssetId::default(), asset).unwrap();

    app.insert_resource(UiTheme(create_dark_theme()))
        .add_systems(Startup, setup_ui)
        .add_systems(Update, (update_send_message, update_receive_message))
        .run();
}
