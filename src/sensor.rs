use bevy::{sprite::{MaterialMesh2dBundle, Mesh2dHandle, Material2d}, prelude::*, sprite::collide_aabb::collide};
use crate::stimulant::*;
use crate::ant::*;

// Sensor configs
pub const SENSOR_ANGLE: f32 = 40.0; 
const SENSOR_RADIUS: f32 = 50.0;

#[derive(Bundle)]
pub struct SensorBundle {
    sensor: Sensor,
    collider: Collider,
    #[bundle]
    meshbundle: MaterialMesh2dBundle<ColorMaterial>,
}

#[derive(Component)]
pub struct Sensor {
    pub intensity: f32,
    pub positioning: SensorPosition,
}


#[derive(Debug)]
pub enum SensorPosition {
    Left,
    Right,
    Center,
}

impl SensorBundle {
    pub fn new(
        sensorposition: SensorPosition,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
    ) -> Self {
        let pos: Vec3;
        match sensorposition {
            SensorPosition::Left =>
                pos = Vec3::new(-100.0, 100.0, 0.0),
            SensorPosition::Right =>
                pos = Vec3::new(100.0, 100.0, 0.0),
            SensorPosition::Center =>
                pos = Vec3::new(0.0, 150.0, 0.0),
        }
        SensorBundle {
            sensor: Sensor { intensity: 0.0, positioning: sensorposition },
            collider: Collider,
            meshbundle: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(shape::Circle::new(SENSOR_RADIUS).into()).into()),
                transform: Transform {
                    translation: pos,
                    ..default()
                },
                visibility: Visibility { is_visible: false },
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                ..default()
            }
        }
    }
}

pub fn update_sensor(
    mut command: Commands,
    ants: Query<&Ant>,
    mut sensors: Query<(&Parent, &mut Sensor, &GlobalTransform), With<Collider>>,
    pheromones: Query<(&Pheromone, &Transform)>,
    foods: Query<(Entity, &Transform), (With<Food>, Without<Pheromone>, Without<Sensor>)>,
) {
    for (parent, mut sensor, sensor_transform) in &mut sensors {
        // Get the sensor's parent ant
        let parent_ant = ants.get(parent.get());
        sensor.intensity = 0.;
        if let Ok(ant) = parent_ant {
            for (pheromone, pheromone_transform) in &pheromones {
                let search_type: PheromoneType;
                match ant.state {
                    AntState::Forage => search_type = PheromoneType::FoodMarker,
                    AntState::ToHome => search_type = PheromoneType::HomeMarker,
                }
                if matches!(&pheromone.pheromone_type, search_type) {
                    let sensor_transform = sensor_transform.compute_transform();
                    let collision = collide(
                        sensor_transform.translation,
                        sensor_transform.scale.truncate(),
                        pheromone_transform.translation,
                        pheromone_transform.scale.truncate(),
                    );
                    if let Some(_collision) = collision {
                        sensor.intensity += pheromone.intensity;
                    }
                }
            }
            for (food_id, food_trans) in &foods {
                let sensor_transform = sensor_transform.compute_transform();
                let collision = collide(
                    sensor_transform.translation,
                    sensor_transform.scale.truncate(),
                    food_trans.translation,
                    food_trans.scale.truncate(),
                );
                if let Some(_collision) = collision {
                    // Ant detected a collision
                    sensor.intensity += 100.0;
                }
            }
        }
    }
}