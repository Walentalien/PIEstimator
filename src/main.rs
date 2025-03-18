use bevy::prelude::*;
use bevy_flycam::{FlyCam, NoCameraPlayerPlugin};
use bevy_rapier3d::na::Scale;
use bevy_rapier3d::prelude::*;
mod lib;
use crate::lib::*;
fn main() {
    App::new()
        .add_plugins(NoCameraPlayerPlugin)
        .insert_resource(ClearColor(Color::srgb(
            0xF9 as f32 / 255.0,
            0xF9 as f32 / 255.0,
            0xFF as f32 / 255.0,
        )))
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
        ))
        .add_systems(Startup, (setup_graphics, setup_physics))
        .add_systems(PostUpdate, display_events)
        // test systems
        .add_systems(Startup, spawn_rigidbody)
        .run();
}

pub fn setup_graphics(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 25.0).looking_at(Vec3::ZERO, Vec3::Y),
        FlyCam,
    ));
}

pub fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
) {
    for collision_event in collision_events.read() {
        println!("Received collision event: {collision_event:?}");
    }

    for contact_force_event in contact_force_events.read() {
        println!("Received contact force event: {contact_force_event:?}");
    }
}

pub fn setup_physics(mut commands: Commands,
                     mut meshes: ResMut<Assets<Mesh>>,
                     mut materials: ResMut<Assets<StandardMaterial>>,
) {
    /*
     * Ground
     */
    commands.spawn((
        Transform::from_xyz(0.0, -1.2, 0.0),
        Collider::cuboid(10.0, 1.0, 10.0),
        Mesh3d(meshes.add(Cuboid::new(20.0, 2.0, 20.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.0, 1.0, 1.0))),


    ));

    commands.spawn((
        Transform::from_xyz(0.0, 5.0, 0.0),
        Collider::cuboid(4.0, 1.5, 1.0),
        Mesh3d(meshes.add(Cuboid::new(5.0, 1.5, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.0, 1.0, 0.0))),
        Sensor,
    ));
    // the falling cube
    commands.spawn((
        Transform::from_xyz(0.0, 20.0, 0.0),
        RigidBody::Dynamic,
        //Collider::cuboid(0.5, 0.5, 0.5),
        Collider::capsule(Default::default(), Default::default(), (3.0)),
        ActiveEvents::COLLISION_EVENTS,
        ContactForceEventThreshold(30.0),
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(Color::srgb(1.0, 0.0, 0.1))),
    ));
}


