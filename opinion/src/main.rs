use bevy::prelude as bp;
use wasm_bindgen::prelude::wasm_bindgen;

mod plugins;
use plugins::{CameraOpinion, Fps, TestCubeUp, WindowOffscreen};

#[wasm_bindgen]
extern "C" {
    // import window.alert from JS
    #[wasm_bindgen(js_namespace = window)]
    fn alert(s: &str);
}

fn main() {
    let mut app = bp::App::new();
    app.add_plugins(bp::DefaultPlugins)
        .add_plugins(WindowOffscreen)
        .add_plugins(CameraOpinion)
        .add_plugins(Fps)
        .add_plugins(TestCubeUp)
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
        height: 512,
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

    // Light
    commands.spawn(DirectionalLight::default());

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
            bp::Transform::from_rotation(bp::Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
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

    commands.spawn((
        bp::Text2d::new("The Spactator"),
        bp::TextFont::default(),
        // bp::Transform::from_translation(bp::Vec3::new(0.0, 4.0, 0.5)),
        // bp::Transform::from_rotation(bp::Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
    ));
}

#[derive(bp::Component)]
struct MyCube;

fn update(time: bp::Res<bp::Time>, query: bp::Query<&mut bp::Transform, bp::With<MyCube>>) {
    // for mut transform in query {
    //     transform.rotate_y(time.delta_secs() / 2.);
    // }
}
