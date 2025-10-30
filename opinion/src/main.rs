use bevy::app::PluginGroup;
use bevy::prelude as bp;
use custom_meshes::UV1x3;
use wasm_bindgen::prelude::wasm_bindgen;

mod custom_meshes;
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
    app.add_plugins(bp::DefaultPlugins.set(bp::AssetPlugin {
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
    .add_systems(bp::Startup, create_texture)
    .add_systems(bp::Update, update);

    app.run();
}

// set up a simple 3D scene
fn setup(
    mut commands: bp::Commands,
    mut meshes: bp::ResMut<bp::Assets<bp::Mesh>>,
    mut materials: bp::ResMut<bp::Assets<bp::StandardMaterial>>,
    asset_server: bp::Res<bp::AssetServer>,
) {
    #[cfg(target_arch = "wasm32")]
    alert("hey bro!");

    commands.spawn((
        bp::PointLight {
            shadows_enabled: true,
            ..Default::default()
        },
        bp::Transform::from_xyz(5.0, 0.0, 10.0).looking_at(bp::Vec3::ZERO, bp::Vec3::Z),
    ));

    // circular base
    commands.spawn((
        bp::Mesh3d(meshes.add(bp::Circle::new(4.0))),
        bp::MeshMaterial3d(materials.add(bp::Color::WHITE)),
        // bp::Transform::from_rotation(bp::Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));

    let custom_texture_handle: bp::Handle<bp::Image> = asset_server.load("array_texture2.png");
    let cube_mesh_handle: bp::Handle<bp::Mesh> = meshes.add(custom_meshes::card(1.0, 1.0, 1.0,UV1x3::default()));

    commands.spawn((
        MyCard,
        bp::Mesh3d(cube_mesh_handle),
        bp::MeshMaterial3d(materials.add(bp::StandardMaterial {
            base_color_texture: Some(custom_texture_handle),
            reflectance: 0.2,
            // emissive: bp::LinearRgba::rgb(0.0, 1.0, 0.0),
            ..Default::default()
        })),
        bp::Transform {
            translation: bp::vec3(0.0, 0.0, 2.0),
            rotation: bp::Quat::from_rotation_x(-10.0_f32.to_radians()),
            ..Default::default()
        },
    ));
}

fn create_texture(
    mut commands: bp::Commands,
    mut meshes: bp::ResMut<bp::Assets<bp::Mesh>>,
    mut materials: bp::ResMut<bp::Assets<bp::StandardMaterial>>,
    mut images: bp::ResMut<bp::Assets<bp::Image>>,
    asset_server: bp::Res<bp::AssetServer>,
) {
    use bevy::{
        asset::RenderAssetUsages,
        camera::RenderTarget,
        color::palettes::css::{BLUE, GRAY, RED},
        prelude::*,
        render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
    };

    let size = Extent3d {
        width: 512,
        height: 512 * 3,
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

    commands.spawn((
        MyCube,
        bp::Mesh3d(meshes.add(bp::Cuboid::new(2.0, 3.0, 0.5))),
        bp::MeshMaterial3d(material_handle),
        bp::Transform::from_rotation(bp::Quat::from_rotation_x(std::f32::consts::PI)),
    ));
}

#[derive(bp::Component)]
struct MyCard;

#[derive(bp::Component)]
struct MyCube;

fn update(time: bp::Res<bp::Time>, query: bp::Query<&mut bp::Transform, bp::With<MyCard>>) {
    for mut transform in query {
        transform.rotate_y(time.delta_secs());
    }
}
