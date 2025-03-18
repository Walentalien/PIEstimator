use bevy::math::Vec3;
use bevy::prelude::{Commands, Transform, TransformBundle};
use bevy_rapier3d::dynamics::{GravityScale, RigidBody, Velocity};
use bevy_rapier3d::geometry::Collider;
use bevy_rapier3d::math::Vect;

pub fn spawn_rigidbody(mut commands: Commands) {
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::cuboid(10.0, 10.0, 10.0))
        // Adds Transform and GlobalTransform components
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 50.0, 0.0)))
        // Adds movement to our dynamic body
        .insert(Velocity {
            linvel: Vec3::new(1.0, 2.0, 0.0),
            angvel:Vect::new( 0.4, 0.0,0.0),
        })

        // Changes the scale of our gravity for this entity
        .insert(GravityScale(2.0));
}

