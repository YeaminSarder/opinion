use bevy::{
    app::{Plugin, Startup, Update},
    asset::Assets,
    color::Color,
    ecs::{component::Component, query::With, system::{Commands, ResMut, Single}},
    math::{primitives::Cuboid, Vec3},
    mesh::{Mesh, Mesh3d},
    pbr::{MeshMaterial3d, StandardMaterial},
    transform::components::Transform
};

pub struct TestCubeUp;

#[derive(Component)]
struct TestCube;

impl Plugin for TestCubeUp {
    fn build(&self, app: &mut bevy::app::App) {
	app.add_systems(Startup, startup)
	.add_systems(Update, update);
    }
}


fn startup (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
	
) {
    commands.spawn((
	Mesh3d(meshes.add(Cuboid::new(1.,1.,1.))),
	MeshMaterial3d(materials.add(Color::srgb(0., 1.,0.))),
	Transform::from_xyz(0., 0., 0.),
	TestCube
    ));
}

fn update (mut t: Single<&mut Transform, With<TestCube>>) {
    t.translation += Vec3::Z * 0.1;
    t.translation %= 16.;
}
