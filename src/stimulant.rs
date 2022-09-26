use bevy::{prelude::*};
use std::time::Duration;
use crate::ant::*;

const TIME_STEP: f32 = 1.0 / 60.0;
const PHEROMONE_LIFETIME: f32 = 30.0;

const RED_FOOD: Color = Color::rgba(1.0, 0.0, 0.0, 1.0);
const BLUE_HOME: Color = Color::rgba(0.0, 0.0, 1.0, 1.0);

enum PheromoneType {
    HomeMarker,
    FoodMarker,
}

#[derive(Component)]
pub struct Collider;

#[derive(Component)]
pub struct Stimulant;

#[derive(Component)]
pub struct Food;

#[derive(Component)]
pub struct Pheromone {
    pheromone_type: PheromoneType,
    pub intensity: f32,
    time: Timer,
}

#[derive(Bundle)]
struct PheromoneBundle {
    pheromone: Pheromone,

    #[bundle]
    sprite_bundle: SpriteBundle,
}

impl Default for Pheromone {
    fn default() -> Self {
        Pheromone { 
            pheromone_type: (PheromoneType::HomeMarker),
            intensity: 0.0,
            time: Timer::new(Duration::from_secs(PHEROMONE_LIFETIME as u64), false)
        }
    }
}

impl PheromoneBundle {
    fn new(pheromone_type: PheromoneType, position: Vec3) -> PheromoneBundle {
        let color: Color;
        match pheromone_type {
            PheromoneType::HomeMarker => color = BLUE_HOME,
            PheromoneType::FoodMarker => color = RED_FOOD,
        }
        PheromoneBundle {
            pheromone: Pheromone {
                pheromone_type: pheromone_type,
                ..default()
            },
            sprite_bundle: SpriteBundle {
                    sprite: Sprite {
                        color: color,
                        ..default()
                    },
                    transform: Transform {
                        translation: position,
                        scale: Vec3::new(5.0, 5.0, 5.0),
                        ..default()
                    },
                ..default()   
            },
        }
    }
}


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
        timer: Timer::new(Duration::from_secs_f32(0.5), true),
    })
}

fn spawn_pheromones(
    mut commands: Commands,
    time: Res<Time>,
    mut config: ResMut<PheromoneSpawnConfig>,
    ants: Query<(&Ant, &Transform)>,
) {
    // tick the timer
    config.timer.tick(time.delta());

    if config.timer.finished() {
        for (ant, transform) in &ants{
            match ant.state {
                AntState::Forage => commands.spawn_bundle(PheromoneBundle::new(PheromoneType::HomeMarker, transform.translation)),
                AntState::ToHome => commands.spawn_bundle(PheromoneBundle::new(PheromoneType::FoodMarker, transform.translation)),
            };
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
        pheromone.intensity = newalpha;
        sprite.color.set_a(newalpha);
        if pheromone.time.finished() {
            commands.entity(entity).despawn();
        }
    }
}