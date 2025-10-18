use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    picking::hover::{HoverMap, Hovered},
    prelude::*,
    ui_widgets::{CoreScrollbarDragState, CoreScrollbarThumb},
};

pub(crate) const GRAY1: Srgba = Srgba::new(0.224, 0.224, 0.243, 1.0);
pub(crate) const GRAY2: Srgba = Srgba::new(0.486, 0.486, 0.529, 1.0);
pub(crate) const GRAY3: Srgba = Srgba::new(1.0, 1.0, 1.0, 1.0);

/// UI scrolling event.
#[derive(EntityEvent, Debug)]
#[entity_event(propagate, auto_propagate)]
struct Scroll {
    entity: Entity,
    /// Scroll delta in logical coordinates.
    delta: Vec2,
}

const LINE_HEIGHT: f32 = 21.;

fn send_scroll_events(
    mut mouse_wheel_reader: MessageReader<MouseWheel>,
    hover_map: Res<HoverMap>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    for mouse_wheel in mouse_wheel_reader.read() {
        let mut delta = -Vec2::new(mouse_wheel.x, mouse_wheel.y);

        if mouse_wheel.unit == MouseScrollUnit::Line {
            delta *= LINE_HEIGHT;
        }

        if keyboard_input.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]) {
            std::mem::swap(&mut delta.x, &mut delta.y);
        }

        for pointer_map in hover_map.values() {
            for entity in pointer_map.keys().copied() {
                commands.trigger(Scroll { entity, delta });
            }
        }
    }
}

fn on_scroll_handler(
    mut scroll: On<Scroll>,
    mut query: Query<(&mut ScrollPosition, &Node, &ComputedNode)>,
) {
    let Ok((mut scroll_position, node, computed)) = query.get_mut(scroll.entity) else {
        return;
    };

    let max_offset = (computed.content_size() - computed.size()) * computed.inverse_scale_factor();

    let delta = &mut scroll.delta;
    if node.overflow.x == OverflowAxis::Scroll && delta.x != 0. {
        // Is this node already scrolled all the way in the direction of the scroll?
        let max = if delta.x > 0. {
            scroll_position.x >= max_offset.x
        } else {
            scroll_position.x <= 0.
        };

        if !max {
            scroll_position.x += delta.x;
            // Consume the X portion of the scroll delta.
            delta.x = 0.;
        }
    }

    if node.overflow.y == OverflowAxis::Scroll && delta.y != 0. {
        // Is this node already scrolled all the way in the direction of the scroll?
        let max = if delta.y > 0. {
            scroll_position.y >= max_offset.y
        } else {
            scroll_position.y <= 0.
        };

        if !max {
            scroll_position.y += delta.y;
            // Consume the Y portion of the scroll delta.
            delta.y = 0.;
        }
    }

    // Stop propagating when the delta is fully consumed.
    if *delta == Vec2::ZERO {
        scroll.propagate(false);
    }
}

// Update the color of the scrollbar thumb.
fn update_scrollbar_thumb(
    mut q_thumb: Query<
        (&mut BackgroundColor, &Hovered, &CoreScrollbarDragState),
        (
            With<CoreScrollbarThumb>,
            Or<(Changed<Hovered>, Changed<CoreScrollbarDragState>)>,
        ),
    >,
) {
    for (mut thumb_bg, Hovered(is_hovering), drag) in q_thumb.iter_mut() {
        let color: Color = if *is_hovering || drag.dragging {
            // If hovering, use a lighter color
            GRAY3
        } else {
            // Default color for the slider
            GRAY2
        }
        .into();

        if thumb_bg.0 != color {
            // Update the color of the thumb
            thumb_bg.0 = color;
        }
    }
}

pub(crate) struct UiScrollPlugin;

impl Plugin for UiScrollPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (send_scroll_events, update_scrollbar_thumb))
            .add_observer(on_scroll_handler);
    }
}
