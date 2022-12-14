use bevy::{prelude::*, sprite::collide_aabb::{collide, Collision}, ecs::bundle, render::color, window::PresentMode};
use ant::*;
use stimulant::*;
mod ant;
mod stimulant;
mod sensor;
mod colony;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "I am a window!".to_string(),
            width: 2000.,
            height: 1000.,
            present_mode: PresentMode::AutoVsync,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_plugin(AntPlugin)
        .add_plugin(StimulantPlugin)
        .add_system(mouse_button_input)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(
    mut commands: Commands, 
) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn mouse_button_input(
    mut commands: Commands,
    wnds: Res<Windows>,
    buttons: Res<Input<MouseButton>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) { 
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(world_pos) = get_cursor_world_pos(wnds, q_camera) {
            commands.spawn()
            .insert(Food)
            .insert(Collider)
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::Rgba { red: 0.0, green: 1.0, blue: 0.0, alpha: 1.0 },
                    ..default()
                },
                transform: Transform {
                    translation: world_pos.extend(0.0),
                    scale:  Vec3::new(5.0, 5.0, 5.0),
                    ..default()
                },
                ..default()
            });
        }
    }
    else if buttons.pressed(MouseButton::Right) {
        if let Some(world_pos) = get_cursor_world_pos(wnds, q_camera) {
            commands.spawn_bundle(PheromoneBundle::new(PheromoneType::FoodMarker, world_pos.extend(0.)));
        }
    }
}

fn get_cursor_world_pos(wnds: Res<Windows>, q_camera: Query<(&Camera, &GlobalTransform)>) -> Option<Vec2>{
    let wnd = wnds.get_primary().unwrap();

    if let Some(_position) = wnd.cursor_position() {
        let (camera, camera_transform) = q_camera.single();
        // get the size of the window
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (_position / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();
        return Some(world_pos)
    }
    None
}