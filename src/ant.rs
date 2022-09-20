use bevy::prelude::Plugin;
use bevy::{prelude::*, math::vec3};
extern crate rand;
use rand::thread_rng;
use rand::Rng;

use crate::stimulant::{Food};
// use crate::stimulant::FoodPellet;


const TIME_STEP: f32 = 1.0 / 60.0;
const ANT_MOVESPEED: f32 = 30.0;

pub struct AntPlugin;

#[derive(Bundle)]
struct AntBundle {
    ant: Ant,
    #[bundle]
    sprite: SpriteBundle,
}

#[derive(Component)]
pub struct Ant {
    /// linear speed in meters per second
    movement_speed: f32,
    /// rotation speed in radians per second
    rotation_speed: f32,
    pub state: AntState,
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
    asset_server: Res<AssetServer>,
) {
    for _i in 0..9 {
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
                movement_speed: ANT_MOVESPEED,
                rotation_speed: f32::to_radians(0.0),
                state: AntState::Forage,
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
    mut command: Commands,
    time: Res<Time>,
    mut ants: Query<(&mut Ant, &mut Transform)>,
    stimulants: Query<(Entity, &Transform), (With<Food>, Without<Ant>)>,
) {
    for (mut ant, mut transform) in &mut ants {
        match ant.state {
            AntState::Forage => {
                // Look for any food source near the ant
                for (food_entity, food_trans) in &stimulants {
                    if transform.translation.distance(food_trans.translation) < 50.0 {
                        // transform.look_at(food_trans.translation, Vec3::Y);
                        command.entity(food_entity).despawn();
        
                        let diff = food_trans.translation - transform.translation;
                        let angle = diff.y.atan2(diff.x); // Add/sub FRAC_PI here optionally
                        transform.rotation = Quat::from_rotation_z(angle+3.14/-2.0);
                        println!("{}", angle);

                        ant.state = AntState::ToHome;
                    }
                }
            },
            AntState::ToHome => {

            },
        }

        // Randomly walk around
        let mut rng = thread_rng();
        let mut rotation = ant.rotation_speed;
        let rot_delta: f32 = rng.gen_range(-50.0, 50.0);
        rotation = rotation + f32::to_radians(rot_delta);
        rotation = f32::clamp(rotation, f32::to_radians(-45.0), f32::to_radians(45.0));
        transform.rotate_z(rotation * TIME_STEP*2.0);
        ant.rotation_speed = rotation;

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