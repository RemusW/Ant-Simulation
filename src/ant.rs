
use bevy::prelude::Plugin;
use bevy::{prelude::*, math::vec3, sprite::collide_aabb::{collide}};
extern crate rand;
use rand::thread_rng;
use rand::Rng;

use crate::sensor::*;
use crate::stimulant::{Food};
use crate::colony::*;

const TIME_STEP: f32 = 1.0 / 60.0;
const ANT_MOVESPEED: f32 = 20.0;

pub struct AntPlugin;

#[derive(Bundle)]
pub struct AntBundle {
    ant: Ant,
    #[bundle]
    sprite: SpriteBundle,
}

#[derive(Component)]
pub struct Ant {
    pub state: AntState,
    pub colony: Entity, 
    /// linear speed in meters per second
    movement_speed: f32,
    /// rotation speed in radians per second
    rotation_speed: f32,
}

pub enum AntState {
    Forage,
    ToHome,
}

impl Plugin for AntPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_colony)
            // .add_startup_system(spawn_ant)
            .add_system(update_sensor)
            .add_system(collide_food)
            .add_system(move_ants);
    }
}

impl AntBundle {
    pub fn new(
        asset_server: &AssetServer,
        position: Vec3,
        colony_id: Entity,
    ) -> AntBundle {
        let mut rng = rand::thread_rng();
        AntBundle {
            sprite: SpriteBundle {
                transform: Transform {
                    translation: position,
                    scale: vec3(0.15, 0.15, 1.0),
                    rotation: Quat::from_rotation_z(rng.gen_range(-3.14, 3.14)),
                    ..default()
                },
                texture: asset_server.load("ant.png"),
                ..default()
            },
            ant: Ant {
                state: AntState::Forage,
                colony: colony_id,
                movement_speed: ANT_MOVESPEED,
                rotation_speed: f32::to_radians(0.0),
            },
        }
    }
    
}

pub fn spawn_ant(
    commands: &mut Commands,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<ColorMaterial>>,
    asset_server: &Res<AssetServer>,
    colony_position: Vec3,
    colony_id: Entity,
) {
    let ant_id = commands.spawn_bundle(AntBundle::new(&asset_server, colony_position, colony_id)).id();
    let left_sensor = commands.spawn_bundle(SensorBundle::new(SensorPosition::Left, &mut meshes, &mut materials)).id();
    let right_sensor = commands.spawn_bundle(SensorBundle::new(SensorPosition::Right, &mut meshes, &mut materials)).id();
    let center_sensor = commands.spawn_bundle(SensorBundle::new(SensorPosition::Center, &mut meshes, &mut materials)).id();
    // Add sensors as children to the ant
    commands.entity(ant_id).push_children(&[left_sensor]);
    commands.entity(ant_id).push_children(&[right_sensor]);
    commands.entity(ant_id).push_children(&[center_sensor]);
}

fn move_ants(
    mut ants: Query<(&mut Ant, &mut Transform, &Children), With<Ant>>,
    sensors: Query<&Sensor, (With<Sensor>, Without<Food>, Without<Ant>)>,
) {
    for (mut ant, mut transform, children) in &mut ants {
        let mut found_food = false;
        let mut max_intensity = -1.0;
        let mut max_sensor: Option<&Sensor> = None;
        for &child in children.iter() {
            if let Ok(sensor) = sensors.get(child) {
                if sensor.intensity > max_intensity {
                    max_sensor = Some(sensor);
                    max_intensity = sensor.intensity;
                }
            }
        }
        let angle: f32; // Add/sub FRAC_PI here optionally
        if let Some(sensor) = max_sensor {
            match sensor.positioning {
                SensorPosition::Left => angle = SENSOR_ANGLE,
                SensorPosition::Right => angle = SENSOR_ANGLE*-1.0,
                SensorPosition::Center => angle = 0.0,
            }
            if max_intensity > 0.0 {
                transform.rotate_z(f32::to_radians(angle) * TIME_STEP*3.);
                found_food = true;
                // transform.rotation = Quat::from_rotation_z(f32::to_radians(angle)+3.14/-2.0);
            }
        }
        
        if found_food == false {
            // Randomly walk around
            let mut rng = thread_rng();
            let mut rotation = ant.rotation_speed;
            let rot_delta: f32 = rng.gen_range(-50.0, 50.0);
            rotation = rotation + f32::to_radians(rot_delta);
            rotation = f32::clamp(rotation, f32::to_radians(-45.0), f32::to_radians(45.0));
            transform.rotate_z(rotation * TIME_STEP*2.0);
            ant.rotation_speed = rotation;
        }

        // get the ship's forward vector by applying the current rotation to the ships initial facing vector
        let movement_direction = transform.rotation * Vec3::Y;
        // get the distance the ship will move based on direction, the ship's movement speed and delta time
        let movement_distance = ant.movement_speed * TIME_STEP;
        // create the change in translation using the new movement direction and distance
        let translation_delta = movement_direction * movement_distance;
        // update the ship translation with our new translation delta
        transform.translation += translation_delta;
    }
}

fn collide_food(
    mut command: Commands,
    mut ants: Query<(&mut Ant, &mut Transform), With<Ant>>,
    foods: Query<(Entity, &Transform), (With<Food>, Without<Ant>)>,
) {
    for (mut ant, ant_trans) in &mut ants {
        for (food_id, food_trans) in &foods {
            // Check for ant collision with a food pellet
            let collision = collide(
                ant_trans.translation,
                ant_trans.scale.truncate(),
                food_trans.translation,
                food_trans.scale.truncate(),
            );
            if let Some(_collision) = collision {
                // Ant detected a collision
                command.entity(food_id).despawn();
                ant.state = AntState::ToHome;
            }
        }
    }
}