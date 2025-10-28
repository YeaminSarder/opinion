use bevy::{
    app::{App, Plugin, Startup},
    camera::Camera3d,
    ecs::{entity::Entity, resource::Resource, system::Commands},
    light::PointLight,
    math::Vec3,
    transform::components::Transform,
};

#[derive(Resource)]
struct C2DID(Entity);

pub struct CameraOpinion;

impl Plugin for CameraOpinion {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_camera_and_light);
    }
}

fn add_camera_and_light(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, -16.0, 16.0).looking_at(Vec3::ZERO, Vec3::Z),
    ));

    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..Default::default()
        },
        Transform::from_xyz(-16.0, -16.0, 16.0),
    ));
}
