use bevy::{sprite::{MaterialMesh2dBundle, Mesh2dHandle, Material2d}, prelude::*, sprite::collide_aabb::collide};
use crate::ant::*;
use crate::stimulant::Collider;

const COLONY_RADIUS: f32 = 50.0;
const COLONY_POP: u16 = 10;

#[derive(Component)]
pub struct Colony {
    pub food_store: u16,
}

#[derive(Bundle)]
pub struct ColonyBundle {
    colony: Colony,
    collider: Collider,
    #[bundle]
    meshbundle: MaterialMesh2dBundle<ColorMaterial>,
}

impl ColonyBundle {
    pub fn new(
        location: Vec3,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
    ) -> Self {
        ColonyBundle {
            colony: Colony { food_store: 0 },
            collider: Collider,
            meshbundle: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(shape::Circle::new(COLONY_RADIUS).into()).into()),
                transform: Transform {
                    translation: location,
                    ..default()
                },
                // material: materials.add(ColorMaterial::from(Color::Rgba { red: 1., green: 0., blue: 0., alpha: 0.75 })),
                material: materials.add(ColorMaterial::from(Color::ORANGE_RED)),
                ..default()
            }
        }
    }
}

pub fn spawn_colony(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>
) {
    println!("spawning colony");
    let starting_location = Vec3::new(0., 0., 0.);
    let colony_id = commands.spawn_bundle(ColonyBundle::new(starting_location, &mut meshes, &mut materials)).id();
    for _i in 0..100 {
        spawn_ant(&mut commands, &mut meshes, &mut materials, &asset_server, starting_location, colony_id);
    }
}

pub fn feed_colony(
    colonies: &mut Query<(Entity, &Transform)>,
    ants: &mut Query<(&mut Ant, &Transform)>
) {
    for (colony_id, colony_trans) in colonies.iter_mut() {
        for (mut ant, ant_trans) in ants.iter_mut() {
            if matches!(ant.state, AntState::ToHome) {
                let collision = collide(
                    colony_trans.translation,
                    colony_trans.scale.truncate(),
                    ant_trans.translation,
                    ant_trans.scale.truncate(),
                );
                let is_colony = ant.colony == colony_id;
                if let Some(_collision) = collision {
                    ant.state = AntState::Forage;
                    
                }
            }
        }
    }
}