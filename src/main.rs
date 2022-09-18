use std::time::Duration;

use bevy::time::Stopwatch;
use bevy::{transform, sprite};
use bevy::{prelude::*, math::vec3};
extern crate rand;
use rand::thread_rng;
use rand::Rng;

const TIME_STEP: f32 = 1.0 / 60.0;

enum PheromoneType {
    Home,
    Food,
    Avoid,
}

#[derive(Bundle)]
struct AntBundle {
    ant: Ant,
    #[bundle]
    sprite: SpriteBundle,
}

#[derive(Component)]
struct Ant {
    /// linear speed in meters per second
    movement_speed: f32,
    /// rotation speed in radians per second
    rotation_speed: f32,
}

#[derive(Component)]
struct Pheromone {
    pheromone_type: PheromoneType,
    time: Timer,
}

impl Default for Pheromone {
    fn default() -> Self {
        Pheromone { 
            pheromone_type: (PheromoneType::Home),
            time: Timer::new(Duration::from_secs(10), false)
        }
    }
}


#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Collider;

struct PheromoneSpawnConfig {
    /// How often to spawn a new bomb? (repeating timer)
    timer: Timer,
}

// impl AntBundle {
//     fn default() -> AntBundle {
//         AntBundle {
//             sprite: SpriteBundle {
//                 transform: Transform {
//                     translation: Vec3::new(0.0, 0.0, 0.0),
//                     scale: vec3(0.1, 0.1, 1.0),
//                     ..default()
//                 },
//                 // texture: asset_server.load("ant.png"),
//                 ..default()
//             },
//             ant: Ant {
//                 movement_speed: 30.0,
//                 rotation_speed: f32::to_radians(0.0),
//             },
//         }
//     }
// }


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(spawn_ants)
        .add_system(move_ants)
        .add_system(spawn_pheromones)
        // .add_system(pheromone_life)
        .run();
}

fn setup(
    mut commands: Commands, 
) {
    commands.spawn_bundle(Camera2dBundle::default());

    commands.insert_resource(PheromoneSpawnConfig {
        // create the repeating timer
        timer: Timer::new(Duration::from_secs(1), true),
    })
}

fn spawn_ants(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for _i in 0..1 {
        commands.spawn_bundle(AntBundle {
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
                movement_speed: 30.0,
                rotation_speed: f32::to_radians(0.0),
            },
        });
    }

    // commands
    //     .spawn_bundle( SpriteBundle {
    //         transform: Transform {
    //             translation: Vec3::new(0.0, 0.0, 0.0),
    //             scale: vec3(0.5, 0.5, 1.0),
    //             ..default()
    //         },
    //         texture: asset_server.load("ant.png"),
    //         ..default()
    //     })
    //     .insert(Ant);
        
}


fn move_ants(
    time: Res<Time>,
    mut sprite_position: Query<(&mut Ant, &mut Transform)>,
) {
    for (mut _ant, mut transform) in &mut sprite_position {
        // if transform.translation.y > 100.0 {
        //     transform.translation.x += 150. * time.delta_seconds();
        // }
        // transform.translation.y += 150. * time.delta_seconds();
        // print!("moving");
        let mut rng = thread_rng();
        let mut rotation = _ant.rotation_speed;
        let rot_delta: f32 = rng.gen_range(-30.0, 30.0);
        rotation = rotation + f32::to_radians(rot_delta);
        rotation = f32::clamp(rotation, f32::to_radians(-45.0), f32::to_radians(45.0));
        transform.rotate_z(rotation * TIME_STEP*2.0);
        _ant.rotation_speed = rotation;
        // f32::to_degrees(_ant.rotation_speed) + 


        // get the ship's forward vector by applying the current rotation to the ships initial facing vector
        let movement_direction = transform.rotation * Vec3::Y;
        // get the distance the ship will move based on direction, the ship's movement speed and delta time
        let movement_distance = _ant.movement_speed * TIME_STEP;
        // create the change in translation using the new movement direction and distance
        let translation_delta = movement_direction * movement_distance;
        // update the ship translation with our new translation delta
        transform.translation += translation_delta;
    }
}

fn spawn_pheromones(
    mut commands: Commands,
    time: Res<Time>,
    mut config: ResMut<PheromoneSpawnConfig>,
    mut ants: Query<&mut Transform, With<Ant>>,
) {
    // tick the timer
    config.timer.tick(time.delta());

    if config.timer.finished() {
        for transform in &mut ants{
            commands.spawn()
                .insert(Pheromone {
                    ..default()
                })
                .insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::Rgba { red: 0.0, green: 0.0, blue: 1.0, alpha: 1.0 },
                        ..default()
                    },
                    transform: Transform {
                        translation: transform.translation,
                        scale: Vec3::new(5.0, 5.0, 5.0),
                        ..default()
                    },
                ..default()
            });
        }
    }
}

fn pheromone_life (
    mut commands: Commands,
    mut pheromones: Query<(Entity, &mut Pheromone, &mut Sprite)>,
    time: Res<Time>,
) {
    for (entity, mut pheromone, mut sprite) in pheromones.iter_mut() {
        // let lifetime = pheromone.time.elapsed_secs();
        pheromone.time.tick(time.delta());
        // pheromone.time.elapsed_secs()
        
        // let newalpha = TIME_STEP * (sprite.color.a()-0.1);
        // sprite.color.set_a(newalpha);
        if pheromone.time.finished() {
            commands.entity(entity).despawn();
        }
    }
}