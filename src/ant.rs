
use bevy::prelude::Plugin;
use bevy::{prelude::*, math::vec3, sprite::collide_aabb::{collide}};
extern crate rand;
use rand::thread_rng;
use rand::Rng;

use crate::sensor::*;
use crate::stimulant::{Food};

const TIME_STEP: f32 = 1.0 / 60.0;
const ANT_MOVESPEED: f32 = 20.0;



pub struct AntPlugin;

#[derive(Bundle)]
struct AntBundle {
    ant: Ant,
    #[bundle]
    sprite: SpriteBundle,
}

#[derive(Component)]
pub struct Ant {
    pub state: AntState,
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
            .add_startup_system(spawn_ants)
            .add_system(move_ants)
            .add_system(update_sensor);
    }
}

fn spawn_ants(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    for _i in 0..10 {
        let ant_id = commands.spawn_bundle(AntBundle {
            sprite: SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    scale: vec3(0.1, 0.1, 1.0),
                    ..default()
                },
                texture: asset_server.load("ant.png"),
                ..default()
            },
            ant: Ant {
                movement_speed: ANT_MOVESPEED,
                rotation_speed: f32::to_radians(0.0),
                state: AntState::Forage,
            },
        }).id();
       let left_sensor = commands.spawn_bundle(SensorBundle::new(SensorPosition::Left, &mut meshes, &mut materials)).id();
       let right_sensor = commands.spawn_bundle(SensorBundle::new(SensorPosition::Right, &mut meshes, &mut materials)).id();
       let center_sensor = commands.spawn_bundle(SensorBundle::new(SensorPosition::Center, &mut meshes, &mut materials)).id();
        // Add sensors as children to the ant
        commands.entity(ant_id).push_children(&[center_sensor]);
        commands.entity(ant_id).push_children(&[left_sensor]);
        commands.entity(ant_id).push_children(&[right_sensor]);
    }
}


fn move_ants(
    mut command: Commands,
    time: Res<Time>,
    mut ants: Query<(&mut Ant, &mut Transform, &Children), With<Ant>>,
    stimulants: Query<(Entity, &Transform, &Food), (With<Food>, Without<Ant>, Without<Sensor>)>,
    sensors: Query<&Sensor, (With<Sensor>, Without<Food>, Without<Ant>)>,
) {
    for (mut ant, mut transform, children) in &mut ants {
        let mut found_food = false;
        match ant.state {
            AntState::Forage => {
                // Look for any food source near the ant
                for (food_entity, food_trans, maybe_food) in &stimulants {
                    if transform.translation.distance(food_trans.translation) < 80.0 {
                        let diff = food_trans.translation - transform.translation;
                        let angle = diff.y.atan2(diff.x); // Add/sub FRAC_PI here optionally
                        transform.rotation = Quat::from_rotation_z(angle+3.14/-2.0);
                    }
                    // Check for ant collision with a food pellet
                    let collision = collide(
                        transform.translation,
                        transform.scale.truncate(),
                        food_trans.translation,
                        food_trans.scale.truncate(),
                    );
                    if let Some(collision) = collision {
                        // Ant detected a collision
                        found_food = true;
                        command.entity(food_entity).despawn();
                        ant.state = AntState::ToHome;
                    }
                }
            },
            AntState::ToHome => {
                let mut max_intensity = -1.0;
                let mut max_sensor: Option<&Sensor> = None;
                for &child in children.iter() {
                    // let sensor: Result<(&Sensor, &Transform), QueryEntityError> = sensors.get(child);
                    // let sensor: Option<(&Sensor, &Transform)> = sensors.get(child).ok();
                    // let sensor = sensors.get(child);
                    if let Ok(sensor) = sensors.get(child) {
                        if sensor.intensity > max_intensity {
                            max_sensor = Some(sensor);
                            max_intensity = sensor.intensity;
                            println!("Assigning new sensor with intensity: {}", max_intensity);
                        }
                    }
                }
                let angle: f32; // Add/sub FRAC_PI here optionally
                if let Some(sensor) = max_sensor {

                    match sensor.positioning {
                        SensorPosition::Left => angle = SENSOR_ANGLE*-1.0,
                        SensorPosition::Right => angle = SENSOR_ANGLE,
                        SensorPosition::Center => angle = 0.0,
                    }
                    println!("New angle: {}", angle);
                    transform.rotate_z(f32::to_radians(angle) * TIME_STEP);
                    // transform.rotation = Quat::from_rotation_z(f32::to_radians(angle)+3.14/-2.0);
                }
            },
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
