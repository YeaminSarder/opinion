use bevy::{
    app::{App, Plugin, Startup},
    camera::{Camera, Camera2d, Camera3d, RenderTarget},
    ecs::{entity::Entity, resource::Resource, system::Commands},
    light::PointLight,
    math::Vec3,
    transform::components::Transform,
};

const TOP: f32 = 12.0;
const FORWORD: f32 = -12.0;

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
        Transform::from_xyz(0.0, TOP, FORWORD).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..Default::default()
        },
        Transform::from_xyz(0.0, TOP, 0.0),
    ));
}
