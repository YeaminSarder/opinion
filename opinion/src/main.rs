use bevy::{
    app::PluginGroup,
    asset::{AssetPlugin, RenderAssetUsages},
    mesh::{Indices, PrimitiveTopology},
    prelude as bp,
};
use wasm_bindgen::prelude::wasm_bindgen;

mod plugins;
use plugins::{CameraOpinion, Fps, PanOrbitCamera, TestCubeUp, WindowOffscreen, XYZ};

#[wasm_bindgen]
extern "C" {
    // import window.alert from JS
    #[wasm_bindgen(js_namespace = window)]
    fn alert(s: &str);
}

fn main() {
    let asset_path = "../assets/";
    let mut app = bp::App::new();
    app.add_plugins(bp::DefaultPlugins.set(AssetPlugin {
        file_path: asset_path.into(),
        ..Default::default()
    }))
    .add_plugins(WindowOffscreen)
    // .add_plugins(CameraOpinion)
    .add_plugins(PanOrbitCamera)
    .add_plugins(Fps)
    // .add_plugins(TestCubeUp)
    // .add_plugins(XYZ {
    //     radius: 0.1,
    //     height: 4.,
    //     resolution: 10,
    // })
    .add_systems(bp::Startup, setup)
    .add_systems(bp::Update, update);

    app.run();
}

// set up a simple 3D scene
fn setup(
    mut commands: bp::Commands,
    mut meshes: bp::ResMut<bp::Assets<bp::Mesh>>,
    mut materials: bp::ResMut<bp::Assets<bp::StandardMaterial>>,
    mut images: bp::ResMut<bp::Assets<bp::Image>>,
    asset_server: bp::Res<bp::AssetServer>,
) {
    #[cfg(target_arch = "wasm32")]
    alert("hey bro!");

    // circular base
    commands.spawn((
        bp::Mesh3d(meshes.add(bp::Circle::new(4.0))),
        bp::MeshMaterial3d(materials.add(bp::Color::WHITE)),
        // bp::Transform::from_rotation(bp::Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));
    // cube
    // commands.spawn((
    //     MyCube,
    //     bp::Mesh3d(meshes.add(bp::Cuboid::new(1.0, 1.0, 1.0))),
    //     bp::MeshMaterial3d(materials.add(bp::Color::srgb_u8(124, 144, 255))),
    //     bp::Transform::from_xyz(0.0, 0.5, 0.0),
    // ));

    use bevy::{
        asset::RenderAssetUsages,
        camera::RenderTarget,
        color::palettes::css::{BLUE, GRAY, RED},
        prelude::*,
        render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
    };

    let size = Extent3d {
        width: 512,
        height: 1024,
        ..default()
    };

    // This is the texture that will be rendered to.
    let mut image = Image::new_fill(
        size,
        TextureDimension::D2,
        &[0, 0, 0, 0],
        TextureFormat::Bgra8UnormSrgb,
        RenderAssetUsages::default(),
    );
    // You need to set these texture usage flags in order to use the image as a render target
    image.texture_descriptor.usage =
        TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::RENDER_ATTACHMENT;

    let image_handle = images.add(image);

    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..Default::default()
        },
        Transform::from_xyz(5.0, 0.0, 10.0).looking_at(bp::Vec3::ZERO, bp::Vec3::Z),
    ));

    // Light
    // commands.spawn(DirectionalLight::default());

    let texture_camera = commands
        .spawn((
            Camera2d,
            Camera {
                // render before the "main pass" camera
                order: -1,
                target: RenderTarget::Image(image_handle.clone().into()),
                ..default()
            },
        ))
        .id();

    commands
        .spawn((
            Node {
                // Cover the whole image
                width: percent(100),
                height: percent(100),
                flex_direction: FlexDirection::Column,
                // top: percent(55),
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Start,
                ..default()
            },
            BackgroundColor(GRAY.into()),
            UiTargetCamera(texture_camera),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        width: Val::Auto,
                        height: Val::Auto,
                        top: percent(55),
                        align_items: AlignItems::Start,
                        padding: UiRect::all(Val::Px(20.)),
                        ..default()
                    },
                    BorderRadius::all(Val::Px(10.)),
                    // BackgroundColor(BLUE.into()),
                ))
                .observe(
                    |drag: On<Pointer<Drag>>, mut nodes: Query<(&mut Node, &ComputedNode)>| {
                        let (mut node, computed) = nodes.get_mut(drag.entity).unwrap();
                        node.left =
                            Val::Px(drag.pointer_location.position.x - computed.size.x / 2.0);
                        node.top = Val::Px(drag.pointer_location.position.y - 50.0);
                    },
                )
                .observe(
                    |over: On<Pointer<Over>>, mut colors: Query<&mut BackgroundColor>| {
                        colors.get_mut(over.entity).unwrap().0 = RED.into();
                    },
                )
                .observe(
                    |out: On<Pointer<Out>>, mut colors: Query<&mut BackgroundColor>| {
                        colors.get_mut(out.entity).unwrap().0 = BLUE.into();
                    },
                )
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("The Spactator"),
                        TextFont {
                            font_size: 50.0,
                            ..default()
                        },
                        TextColor::WHITE,
                    ));
                });
        });

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(image_handle),
        reflectance: 0.02,
        unlit: false,
        ..default()
    });

    let cube = commands
        .spawn((
            MyCube,
            bp::Mesh3d(meshes.add(bp::Cuboid::new(2.0, 3.0, 0.5))),
            // bp::MeshMaterial3d(materials.add(bp::Color::srgb_u8(124, 144, 255))),
            bp::MeshMaterial3d(material_handle),
            bp::Transform::from_rotation(bp::Quat::from_rotation_x(std::f32::consts::PI)),
        ))
        .id();

    // commands.entity(cube).with_children(|parent| {
    //     parent.spawn(Text2dBundle {
    //         text: Text::from_section(
    //             "Hello\nCube",
    //             TextStyle {
    //                 font: font.clone(),
    //                 font_size: 40.0,
    //                 color: Color::WHITE,
    //             },
    //         )
    //         .with_alignment(TextAlignment::Center),
    //         // place slightly outside the face so it doesn't z-fight with the cube
    //         transform: Transform::from_xyz(0.0, 0.0, 0.51).with_scale(Vec3::splat(0.02)), // scale down so the text fits the face
    //         ..Default::default()
    //     });
    // });

    // commands.spawn((
    //     bp::Text2d::new("The Spactator"),
    //     bp::TextFont::default(),
    //     // bp::Transform::from_translation(bp::Vec3::new(0.0, 4.0, 0.5)),
    //     // bp::Transform::from_rotation(bp::Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
    // ));

    let custom_texture_handle: Handle<Image> = asset_server.load("array_texture.png");
    // Create and save a handle to the mesh.
    let cube_mesh_handle: Handle<Mesh> = meshes.add(ccube(1.0, 1.0, 1.0));

    // Render the mesh with the custom texture, and add the marker.
    commands.spawn((
        Mesh3d(cube_mesh_handle),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(custom_texture_handle),
            reflectance: 0.2,
            ..default()
        })),
        bp::Transform {
            translation: bp::vec3(0.0, 0.0, 2.0),
            rotation: bp::Quat::from_rotation_x(-10.0_f32.to_radians()),
            ..Default::default()
        },
        // CustomUV,
    ));
}
//
// struct UvConfig {
//     front: Option<>
// }

fn ccube(x: f32, y: f32, z: f32) -> bp::Mesh {
    let half_size = bp::vec3(x / 2.0, y / 2.0, z / 2.0);
    let min = -half_size;
    let max = half_size;

    // Suppose Y-up right hand, and camera look from +Z to -Z
    let vertices = &[
        // Front
        ([min.x, min.y, max.z], [0.0, 0.0, 1.0], [0.0, 0.75]),
        ([max.x, min.y, max.z], [0.0, 0.0, 1.0], [1.0, 0.75]),
        ([max.x, max.y, max.z], [0.0, 0.0, 1.0], [1.0, 1.0]),
        ([min.x, max.y, max.z], [0.0, 0.0, 1.0], [0.0, 1.0]),
    ];

    let positions: Vec<_> = vertices.iter().map(|(p, _, _)| *p).collect();
    let normals: Vec<_> = vertices.iter().map(|(_, n, _)| *n).collect();
    let uvs: Vec<_> = vertices.iter().map(|(_, _, uv)| *uv).collect();

    #[rustfmt::skip]
    let indices = Indices::U32(vec![
        0, 1, 2, 2, 3, 0, // front
        // 4, 5, 6, 6, 7, 4, // back
        // 8, 9, 10, 10, 11, 8, // right
        // 12, 13, 14, 14, 15, 12, // left
        // 16, 17, 18, 18, 19, 16, // top
        // 20, 21, 22, 22, 23, 20, // bottom
    ]);

    bp::Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(bp::Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(bp::Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(bp::Mesh::ATTRIBUTE_UV_0, uvs)
    .with_inserted_indices(indices)
}

#[derive(bp::Component)]
struct MyCube;

fn update(time: bp::Res<bp::Time>, query: bp::Query<&mut bp::Transform, bp::With<MyCube>>) {
    // for mut transform in query {
    //     transform.rotate_y(time.delta_secs() / 2.);
    // }
}
