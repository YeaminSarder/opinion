use std::f32::consts::PI;

use bevy::{
    app::{App, Plugin, Startup},
    asset::Assets,
    color::Color,
    ecs::{
        resource::Resource,
        system::{Commands, Res, ResMut},
    },
    math::{Affine3A, Mat4, Quat, Vec3, VectorSpace, primitives::Cylinder},
    mesh::{Mesh, Mesh3d, Meshable},
    pbr::{MeshMaterial3d, StandardMaterial},
    transform::components::Transform,
};

#[derive(Resource, Copy, Clone)]
pub struct XYZ {
    pub radius: f32,
    pub height: f32,
    pub resolution: u32,
}
impl Plugin for XYZ {
    fn build(&self, app: &mut App) {
        app.insert_resource(*self);
        app.add_systems(Startup, startup);
    }
}

fn startup(
    xyz: Res<XYZ>,
    mut command: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    command.spawn((
        Mesh3d(
            meshes.add(
                Cylinder::new(xyz.radius, xyz.height)
                    .mesh()
                    .resolution(xyz.resolution),
            ),
        ),
        MeshMaterial3d(materials.add(Color::srgb(1., 0., 0.))),
        Transform::from_matrix(Mat4::from_rotation_translation(
            Quat::from_rotation_z(-PI / 2.),
            Vec3::new(xyz.height / 2., 0., 0.),
        )),
    ));
    command.spawn((
        Mesh3d(
            meshes.add(
                Cylinder::new(xyz.radius, xyz.height)
                    .mesh()
                    .resolution(xyz.resolution),
            ),
        ),
        MeshMaterial3d(materials.add(Color::srgb(0., 1., 0.))),
        Transform::from_matrix(Mat4::from_rotation_translation(
            Quat::IDENTITY,
            Vec3::new(0., xyz.height / 2., 0.),
        )),
    ));
    command.spawn((
        Mesh3d(
            meshes.add(
                Cylinder::new(xyz.radius, xyz.height)
                    .mesh()
                    .resolution(xyz.resolution),
            ),
        ),
        MeshMaterial3d(materials.add(Color::srgb(0., 0., 1.))),
        Transform::from_matrix(Mat4::from_rotation_translation(
            Quat::from_rotation_x(PI / 2.),
            Vec3::new(0., 0., xyz.height / 2.),
        )),
    ));
}
