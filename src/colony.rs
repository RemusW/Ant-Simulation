use bevy::{sprite::{MaterialMesh2dBundle, Mesh2dHandle, Material2d}, prelude::*, sprite::collide_aabb::collide};
use crate::ant::*;
use crate::stimulant::Collider;

const COLONY_RADIUS: f32 = 50.0;
const COLONY_POP: u16 = 10;

#[derive(Component)]
pub struct Colony;

#[derive(Bundle)]
pub struct ColonyBundle {
    colony: Colony,
    collider: Collider,
    #[bundle]
    meshbundle: MaterialMesh2dBundle<ColorMaterial>,
}

impl ColonyBundle {
    pub fn new(
        location: Vec2,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
    ) -> Self {
        ColonyBundle {
            colony: Colony,
            collider: Collider,
            meshbundle: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(shape::Circle::new(COLONY_RADIUS).into()).into()),
                transform: Transform {
                    translation: location.extend(0.),
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
    &mut asset_server: &mut Res<AssetServer>,
) {
    let starting_location = Vec2::new(0., 0.);
    commands.spawn_bundle(ColonyBundle::new(starting_location, &mut meshes, &mut materials));
    for _i in 0..COLONY_POP {
        AntBundle::spawn_ant(&mut commands, &mut meshes, &mut materials, &mut asset_server, starting_location);
    }
}