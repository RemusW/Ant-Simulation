use bevy::prelude::Plugin;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::{prelude::*, math::vec3, sprite::collide_aabb::{collide, Collision}};
extern crate rand;
use rand::thread_rng;
use rand::Rng;

use crate::stimulant::{Food, Collider};

const TIME_STEP: f32 = 1.0 / 60.0;
const ANT_MOVESPEED: f32 = 20.0;

// Sensor configs
const SENSOR_ANGLE: f32 = 45.0;
const SENSOR_SIZE: f32 = 10.0;
const SENSOR_DIST: f32 = 20.0;

pub struct AntPlugin;

#[derive(Bundle)]
struct AntBundle {
    ant: Ant,
    #[bundle]
    sprite: SpriteBundle,
}

#[derive(Component)]
struct Sensor;

// impl Sensor {
//     fn new(angle: f32, pos: Vec3) -> Self {
//         Sensor {
//             meshbundle: MaterialMesh2dBundle {
//                 ..default()
//             }
//         }
//     }
// }

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
            .add_system(move_ants);
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
            // sensors: [Sensor; 3]
            // left_sensor: Sensor,
            // right_sensor: Sensor,
            // center_sensor: Sensor,
        }).id();
        let left_sensor = commands.spawn()
            .insert(Sensor)
            .insert_bundle(MaterialMesh2dBundle {
                // mesh: meshes.add(Mesh::from(shape::Circle::default())).into(),
                mesh: meshes.add(shape::Circle::new(100.).into()).into(),
                // transform: Transform::default().with_scale(Vec3::splat(128.)),
                transform: Transform {
                    translation: Vec3::new(200.0, 150.0, 0.0),
                    // scale: Vec3::splat(128.),
                    ..default()
                },
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                ..default()
            })
            .id();
        let right_sensor = commands.spawn()
            .insert(Sensor)
            .insert_bundle(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(100.).into()).into(),
                transform: Transform {
                    translation: Vec3::new(-200.0, 150.0, 0.0),
                    ..default()
                },
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                ..default()
            })
            .id();
        let center_sensor = commands.spawn()
            .insert(Sensor)
            .insert_bundle(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(100.).into()).into(),
                transform: Transform {
                    translation: Vec3::new(0.0, 200.0, 0.0),
                    ..default()
                },
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                ..default()
            })
            .id();
        
        // Add sensors as children to the ant
        commands.entity(ant_id).push_children(&[left_sensor]);
        commands.entity(ant_id).push_children(&[right_sensor]);
        commands.entity(ant_id).push_children(&[center_sensor]);
    }
}


fn move_ants(
    mut command: Commands,
    time: Res<Time>,
    mut ants: Query<(&mut Ant, &mut Transform)>,
    stimulants: Query<(Entity, &Transform, Option<&Food>), (With<Collider>, Without<Ant>)>,
) {
    for (mut ant, mut transform) in &mut ants {
        let mut found_food = false;
        match ant.state {
            AntState::Forage => {
                // Look for any food source near the ant
                for (food_entity, food_trans, maybe_food) in &stimulants {
                    // Check for ant collision with a food pellet
                    if transform.translation.distance(food_trans.translation) < 80.0 {
                        let diff = food_trans.translation - transform.translation;
                        let angle = diff.y.atan2(diff.x); // Add/sub FRAC_PI here optionally
                        transform.rotation = Quat::from_rotation_z(angle+3.14/-2.0);
                    }
                    let collision = collide(
                        transform.translation,
                        transform.scale.truncate(),
                        food_trans.translation,
                        food_trans.scale.truncate(),
                    );
                    if let Some(collision) = collision {
                        // Ant detected a collision
                        if maybe_food.is_some() {
                            found_food = true;
                            command.entity(food_entity).despawn();
                            ant.state = AntState::ToHome;
                        }
                    }
                }
            },
            AntState::ToHome => {

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