use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiPrimaryContextPass};

// Components are instance variables per entity in the world

// All components used for dragging stuff
#[derive(Component)]
struct Draggable;

#[derive(Resource, Default)]
struct DragState {
    entity: Option<Entity>,
}

#[derive(Component)]
struct Gate; // Keep here for now

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

// Each gate will have inputs and outputs
#[derive(Component)]
struct Inputs {
    in_a: bool,
    in_b: bool,
}

#[derive(Component)]
struct Output {
    out: bool,
}

// List of game states to track for UI transitions
#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
enum GameState {
    #[default]
    MainMenu,
    Editor,
    Credits,
}

// Used for snapping to grid
const GRID_SIZE: f32 = 32.0;

// Overall startup, creating the app, running throught the assets and running the program.
fn main() {
    App::new() // Create new app
    .insert_resource(DragState::default()) // Create new global resource to track drag state
    .add_plugins(DefaultPlugins) // Plugins for Bevy game development
    .add_plugins(EguiPlugin::default()) // Plugins for Bevy egui
    .init_state::<GameState>() // Set initial game state
    .add_systems(Startup, setup) // Run setup process once
    .add_systems(EguiPrimaryContextPass, user_interface) // Load user interface
    .add_systems(Update, ( // Run certain functions once per frame / every 60 secs
        start_drag_system,
        drag_system,
        end_drag_system,
    ))
    .run();
}


// Run the UI system for rendering egui menus + state logic
// Run every frame and depending on whenever a state button is pressed, render different UI
fn user_interface(
    mut contexts: EguiContexts, // Give access to egui to draw UI
    state: Res<State<GameState>>, // Read what state the game is currently in
    mut next_state: ResMut<NextState<GameState>>, // What state to change to next frame?
) -> Result {
    let ctx = contexts.ctx_mut()?; // Get access to bevy_egui's internal state

    match state.get() { // Depending on current state, show a certain window's contents
        GameState::MainMenu => { // If main menu, show main menu -> transition to other pages
            egui::CentralPanel::default().show(ctx, |ui| {
                // LET'S MAKE THIS STUFF BEAUTIFUL
                ui.label(
                    egui::RichText::new("Ferroforge")
                    .size(64.0)
                    .strong()
                ); // Set label for the window

                ui.separator(); // Add strikethrough border

                if ui // If Start Editor button pushed
                    .add_sized([250.0, 80.0], egui::Button::new("Start Editor"))
                    .clicked()
                {
                    next_state.set(GameState::Editor);
                }

                if ui // If Credits button pushed
                    .add_sized([250.0, 80.0], egui::Button::new("Credits"))
                    .clicked()
                {
                    next_state.set(GameState::Credits);
                }

                if ui // If Quit button pushed
                    .add_sized([250.0, 80.0], egui::Button::new("Quit"))
                    .clicked()
                {
                    std::process::exit(0);
                }
            });
        }

        GameState::Editor => { // If editor, show editor
            egui::SidePanel::left("Panel").show(ctx, |ui| {
                if ui.button("Back to Menu").clicked() { // Go back to main menu
                    next_state.set(GameState::MainMenu);
                }

                ui.label("Editor Mode"); // Set header as Editor Mode
            });
        }

        GameState::Credits => {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.label(
                    egui::RichText::new("Credits")
                    .size(100.0)
                    .strong()
                ); // Set label for the window

                ui.separator(); // Add strikethrough border

                // Credits text
                ui.label("Project created by:");
                ui.label("Noah Springer (noahds4)");
                ui.label("Daniel Moraga (dmora59)");
                ui.label("Ferroforge - created for CS128 Honors Project");

                // Button logic
                if ui
                    .add_sized([250.0, 80.0], egui::Button::new("Back to Menu"))
                    .clicked()
                {
                    next_state.set(GameState::MainMenu);
                }
            });
        }
    }

    Ok(())
}



// creates the texture of the gates themselves, while using nand.png. Setting these objects
// in the set coords, for example Vec3::new(-100.0, 0.0, 0.0) is put in the set coords given.

//used in setting up the system *
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    let gate_texture: Handle<Image> = asset_server.load("textures/nand.png");

    spawn_block(&mut commands, Vec3::new(-100.0, 0.0, 0.0), gate_texture.clone()); // Green
    spawn_block(&mut commands, Vec3::new(100.0, 0.0, 0.0), gate_texture.clone()); // Red
    spawn_block(&mut commands, Vec3::new(0.0, 100.0, 0.0), gate_texture.clone()); // Blue
}
//      ^
//      |
//      |
// Spawn custom objects


//Helper function, creates said object
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
//enables the ability to drag objects given
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


