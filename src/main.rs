use bevy::prelude::*;

// Components are instance variables per entity in the world

// All components used for dragging stuff
#[derive(Component)]
struct Draggable;

#[derive(Resource, Default)]
struct DragState {
    entity: Option<Entity>,
}

#[derive(Component)]
struct Gate;

// What gates should we include?
#[derive(Component)]
enum GateType {
    NAND,
    NOR,
    AND,
    OR,
    NOT,
    XOR,
    XNOR,
}

#[derive(Component)]
struct Inputs {
    in_a: bool,
    in_b: bool,
}

#[derive(Component)]
struct Output {
    out: bool,
}

const GRID_SIZE: f32 = 32.0;

// CustomPlugin: Custom plugin to use for the app
/* 
pub struct CustomPlugin;
impl Plugin for CustomPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, drag_system);
    }
}
*/

fn main() {
    App::new() // Create new app
    .insert_resource(DragState::default()) // Crate new global resource to track drag state
    .add_plugins(DefaultPlugins) // Plugins for Bevy game development
    .add_systems(Startup, setup) // Run Setup once
    .add_systems(Update, ( // Update drag system once per frame / every 60 secs
        start_drag_system,
        drag_system,
        end_drag_system,
    ))
    .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    let gate_texture: Handle<Image> = asset_server.load("textures/nand.png");

    spawn_block(&mut commands, Vec3::new(-100.0, 0.0, 0.0), gate_texture.clone()); // Green
    spawn_block(&mut commands, Vec3::new(100.0, 0.0, 0.0), gate_texture.clone()); // Red
    spawn_block(&mut commands, Vec3::new(0.0, 100.0, 0.0), gate_texture.clone()); // Blue
}

// Spawn custom objects
fn spawn_block(commands: &mut Commands, pos: Vec3, texture: Handle<Image>) {
    commands.spawn((
        Sprite {
            image: texture,
            custom_size: Some(Vec2::splat(100.0)),
            ..default()
        },
        Transform::from_translation(pos),
        Draggable,
    ));
}

// Work
// Convert cursor to world position
fn cursor_to_world(
    windows: &Query<&Window>,
    cameras: &Query<(&Camera, &GlobalTransform)>,
) -> Option<Vec2> {
    let Ok(window) = windows.single() else {
        return None;
    };

    let cursor = window.cursor_position()?;

    let Ok((camera, cam_transform)) = cameras.single() else {
        return None;
    };

    camera.viewport_to_world_2d(cam_transform, cursor).ok()
}

// Start dragging on click
fn start_drag_system(
    mut drag_state: ResMut<DragState>,
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    query: Query<(Entity, &Transform), With<Draggable>>,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    let Some(cursor_pos) = cursor_to_world(&windows, &cameras) else {
        return;
    };

    // naive hit test (good for circles/small objects)
    for (entity, transform) in &query {
        let dist = transform.translation.truncate().distance(cursor_pos);

        if dist < 20.0 {
            drag_state.entity = Some(entity);
            break;
        }
    }
}

// Update dragged entity position
fn drag_system(
    drag_state: Res<DragState>,
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut query: Query<&mut Transform>,
) {
    let Some(entity) = drag_state.entity else {
        return;
    };

    if !mouse.pressed(MouseButton::Left) {
        return;
    }

    let Some(cursor_pos) = cursor_to_world(&windows, &cameras) else {
        return;
    };

    if let Ok(mut transform) = query.get_mut(entity) {
        transform.translation.x = snap_to_grid(cursor_pos.x);
        transform.translation.y = snap_to_grid(cursor_pos.y);
    }
}

// Helper: snap to grid
fn snap_to_grid(value: f32) -> f32 {
    (value / GRID_SIZE).round() * GRID_SIZE
}

// Stop dragging
fn end_drag_system(
    mut drag_state: ResMut<DragState>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    if mouse.just_released(MouseButton::Left) {
        drag_state.entity = None;
    }
}

/* The real work
fn mouse_position(windows: Query<&Window>) {
    if let Ok(window) = windows.single() {
        if let Some(pos) = window.cursor_position() {
            println!("Cursor at: {:?}", pos);
        }
    }
}
    */


