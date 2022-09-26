use bevy::{sprite::{MaterialMesh2dBundle, Mesh2dHandle}, prelude::*, sprite::collide_aabb::collide};
use crate::stimulant::*;
use crate::ant::*;

// Sensor configs
pub const SENSOR_ANGLE: f32 = 55.0; 
const SENSOR_RADIUS: f32 = 100.0;

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
                pos = Vec3::new(-200.0, 150.0, 0.0),
            SensorPosition::Right =>
                pos = Vec3::new(0.0, 150.0, 0.0),
            SensorPosition::Center =>
                pos = Vec3::new(200.0, 150.0, 0.0),
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
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                ..default()
            }
        }
    }
}

pub fn update_sensor(
   ants: Query<&Ant>,
   mut sensors: Query<(&Parent, &mut Sensor, &Transform), With<Collider>>,
   pheromones: Query<(&Pheromone, &Transform)>,
) {
    // for (ant, children) in ants.iter() {
    //     for &child in children.iter() {
    //         let sensor = 
    //     }
    // }
    // Get the sensor's parent ant
    for (parent, mut sensor, sensor_transform) in &mut sensors {
        let parent_ant = ants.get(parent.get());
        sensor.intensity = 0.;
        if let Ok(ant) = parent_ant {
            if matches!(ant.state, AntState::ToHome) {
                
            }
        }
        for (pheromone, pheromone_transform) in &pheromones {
            let collision = collide(
                sensor_transform.translation,
                sensor_transform.scale.truncate(),
                pheromone_transform.translation,
                pheromone_transform.scale.truncate(),
            );
            if let Some(collision) = collision {
                sensor.intensity += pheromone.intensity;
            }
            print!("{:?} {}", sensor.positioning, sensor.intensity);
        }
    }
}