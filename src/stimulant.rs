use bevy::prelude::*;
use std::time::Duration;
use crate::ant::*;

const TIME_STEP: f32 = 1.0 / 60.0;
const PHEROMONE_LIFETIME: f32 = 10.0;

enum PheromoneType {
    HomeMarker,
    FoodMarker,
}

#[derive(Component)]
pub struct Stimulant;

#[derive(Component)]
pub struct Food;

#[derive(Component)]
struct Pheromone {
    pheromone_type: PheromoneType,
    time: Timer,
}

impl Default for Pheromone {
    fn default() -> Self {
        Pheromone { 
            pheromone_type: (PheromoneType::HomeMarker),
            time: Timer::new(Duration::from_secs(PHEROMONE_LIFETIME as u64), false)
        }
    }
}

#[derive(Component)]
struct Collider;

struct PheromoneSpawnConfig {
    /// How often to spawn a new bomb? (repeating timer)
    timer: Timer,
}

pub struct StimulantPlugin;

impl Plugin for StimulantPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup)
            .add_system(spawn_pheromones)
            .add_system(pheromone_lifecycle);
    }
}

fn setup(mut commands: Commands) {
    commands.insert_resource(PheromoneSpawnConfig {
        timer: Timer::new(Duration::from_secs(1), true),
    })
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

fn pheromone_lifecycle (
    mut commands: Commands,
    mut pheromones: Query<(Entity, &mut Pheromone, &mut Sprite)>,
    time: Res<Time>,
) {
    for (entity, mut pheromone, mut sprite) in pheromones.iter_mut() {
        // let lifetime = pheromone.time.elapsed_secs();
        pheromone.time.tick(time.delta());
        // pheromone.time.elapsed_secs()
        
        // Decrease the alpha every frame
        let newalpha = sprite.color.a()-(1.0/PHEROMONE_LIFETIME)*TIME_STEP;
        sprite.color.set_a(newalpha);
        if pheromone.time.finished() {
            commands.entity(entity).despawn();
        }
    }
}