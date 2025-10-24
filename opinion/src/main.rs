use bevy::prelude as bp;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    // import window.alert from JS
    #[wasm_bindgen(js_namespace = window)]
    fn alert(s: &str);
}

fn main() {
    let mut app = bp::App::new();
    app.add_plugins(bp::DefaultPlugins)
        .add_systems(bp::Startup, setup)
        .add_systems(bp::Update, update);

    app.run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: bp::Commands,
    mut meshes: bp::ResMut<bp::Assets<bp::Mesh>>,
    mut materials: bp::ResMut<bp::Assets<bp::StandardMaterial>>,
) {
    #[cfg(target_arch = "wasm32")]
    alert("hey bro!");

    // circular base
    commands.spawn((
        bp::Mesh3d(meshes.add(bp::Circle::new(4.0))),
        bp::MeshMaterial3d(materials.add(bp::Color::WHITE)),
        bp::Transform::from_rotation(bp::Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));
    // cube
    commands.spawn((
        MyCube,
        bp::Mesh3d(meshes.add(bp::Cuboid::new(1.0, 1.0, 1.0))),
        bp::MeshMaterial3d(materials.add(bp::Color::srgb_u8(124, 144, 255))),
        bp::Transform::from_xyz(0.0, 0.5, 0.0),
    ));
    // light
    commands.spawn((
        bp::PointLight {
            shadows_enabled: true,
            ..bp::default()
        },
        bp::Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    // camera
    commands.spawn((
        bp::Camera3d::default(),
        bp::Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(bp::Vec3::ZERO, bp::Vec3::Y),
    ));
}

#[derive(bp::Component)]
struct MyCube;

fn update(time: bp::Res<bp::Time>, query: bp::Query<&mut bp::Transform, bp::With<MyCube>>) {
    for mut transform in query {
        transform.rotate_y(time.delta_secs() / 2.);
    }
}
