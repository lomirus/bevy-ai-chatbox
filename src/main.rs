#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ai;
mod ui_scroll;

use std::fs;

use bevy::{
    ecs::{query::QuerySingleError, relationship::RelatedSpawner},
    feathers::{
        FeathersPlugins,
        controls::{ButtonProps, ButtonVariant, button},
        dark_theme::create_dark_theme,
        theme::{ThemedText, UiTheme},
    },
    picking::hover::Hovered,
    prelude::*,
    text::LineHeight,
    ui_widgets::{Activate, ControlOrientation, CoreScrollbarThumb, Scrollbar, observe},
    window::WindowResolution,
};

use ai::{AiPlugin, ReceiveMessage, SendMessage};
use ui_scroll::UiScrollPlugin;

use crate::ui_scroll::GRAY1;

const TEXT_COLOR: Color = Color::Srgba(Srgba::rgb(0.9764706, 0.98039216, 0.9843137));
const BUBBLE_BACKGROUND_COLOR: Color = Color::Srgba(Srgba::rgb(0.17254902, 0.17254902, 0.18039216));
const BACKGROUND_COLOR: Color = Color::Srgba(Srgba::rgb(0.08235294, 0.08235294, 0.09019608));

const DEFAULT_FONT_PATH: &str = "assets/fonts/NotoSansSC-Regular.ttf";

#[derive(Component)]
struct SendButton;

#[derive(Component, Clone, Copy)]
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
                System => unreachable!(),
                User => JustifyContent::End,
                Assistant => JustifyContent::Start,
            },
            ..default()
        },
        role,
        children![(
            Node {
                padding: UiRect::axes(px(16), px(10)),
                align_items: AlignItems::Center,
                ..default()
            },
            BorderRadius::all(px(22)),
            BackgroundColor(match role {
                System => unreachable!(),
                User => BUBBLE_BACKGROUND_COLOR,
                Assistant => BACKGROUND_COLOR,
            }),
            Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
                let text_color = TextColor(match role {
                    System => unreachable!(),
                    User => TEXT_COLOR,
                    Assistant => TEXT_COLOR,
                });

                let text_font = TextFont {
                    font_size: 16.0,
                    line_height: LineHeight::Px(24.0),
                    ..default()
                };

                if is_streaming {
                    parent.spawn((Text::new(content), text_color, text_font, StreamingMessage));
                } else {
                    parent.spawn((Text::new(content), text_color, text_font));
                }
            })),
        )],
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
            display: Display::Grid,
            width: percent(100),
            height: percent(100),
            grid_template_columns: vec![RepeatedGridTrack::flex(1, 1.), RepeatedGridTrack::auto(1)],
            grid_template_rows: vec![RepeatedGridTrack::flex(1, 1.), RepeatedGridTrack::auto(1)],
            ..default()
        },
        BackgroundColor(Srgba::hex("#151517").unwrap().into()),
        Children::spawn(SpawnWith(|parent: &mut RelatedSpawner<ChildOf>| {
            let scroll_area_id = parent
                .spawn((
                    Dialog,
                    Node {
                        flex_direction: FlexDirection::Column,
                        row_gap: px(8),
                        overflow: Overflow::scroll(),
                        padding: UiRect::all(px(8)),
                        ..default()
                    },
                    Children::spawn(SpawnIter(
                        messages
                            .into_iter()
                            .filter(|message| message.role != deepseek_api::Role::System)
                            .map(|message| {
                                message_box(message.role.into(), message.content, false)
                            }),
                    )),
                ))
                .id();

            parent.spawn((
                Node {
                    min_width: px(8),
                    grid_row: GridPlacement::start(1),
                    grid_column: GridPlacement::start(2),
                    ..default()
                },
                Scrollbar {
                    orientation: ControlOrientation::Vertical,
                    target: scroll_area_id,
                    min_thumb_length: 8.0,
                },
                Children::spawn(Spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                    Hovered::default(),
                    BackgroundColor(GRAY1.into()),
                    BorderRadius::all(px(4)),
                    CoreScrollbarThumb,
                ))),
            ));

            parent.spawn((
                Node {
                    grid_row: GridPlacement::start(2),
                    grid_column: GridPlacement::start_span(1, 2),
                    padding: UiRect::all(px(8)),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                Children::spawn(Spawn((
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
                    )],
                ))),
            ));
        })),
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
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(480, 720),
                position: WindowPosition::Centered(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }),
        FeathersPlugins,
        AiPlugin,
        UiScrollPlugin,
    ));

    let font_data = fs::read(DEFAULT_FONT_PATH).unwrap();
    let asset = Font::try_from_bytes(font_data).unwrap();
    let mut assets = app.world_mut().resource_mut::<Assets<_>>();
    assets.insert(AssetId::default(), asset).unwrap();

    app.insert_resource(UiTheme(create_dark_theme()))
        .add_systems(Startup, setup_ui)
        .add_systems(Update, (update_send_message, update_receive_message))
        .run();
}
