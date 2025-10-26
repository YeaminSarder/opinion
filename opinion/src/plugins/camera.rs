use bevy::{
    app::{App, Plugin, Startup},
    camera::Camera3d,
    ecs::system::Commands,
    math::Vec3,
    transform::components::Transform,
};

const DISTENCE: f32 = 15.0;

pub struct CameraOpinion;

impl Plugin for CameraOpinion {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_camera);
    }
}

fn add_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-5.0, DISTENCE, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
