use bevy::{
    app::{App, Plugin, Startup, Update},
    ecs::{
        component::Component,
        query::With,
        resource::Resource,
        system::{Commands, Res, ResMut, Single},
    },
    text::TextFont,
    time::{Time, Timer, TimerMode},
    ui::{Node, PositionType, px, widget::Text},
};

pub struct Fps;

impl Plugin for Fps {
    fn build(&self, app: &mut App) {
        app.insert_resource(UpdateTimer(Timer::from_seconds(0.5, TimerMode::Repeating)));
        app.add_systems(Startup, setup);
        app.add_systems(Update, update_fps);
    }
}

#[derive(Resource)]
struct UpdateTimer(Timer);

#[derive(Component)]
struct FpsText;

fn setup(mut commands: Commands) {
    let text_style = TextFont::default();
    commands.spawn((
        Text::default(),
        text_style.clone(),
        Node {
            position_type: PositionType::Absolute,
            top: px(12),
            right: px(15),
            ..Default::default()
        },
        FpsText,
    ));
}

fn update_fps(
    time: Res<Time>,
    mut timer: ResMut<UpdateTimer>,
    mut text: Single<&mut Text, With<FpsText>>,
) {
    let fps = 1.0 / time.delta_secs();

    if timer.0.tick(time.delta()).just_finished() {
        text.clear();
        text.push_str(&format!("FPS: {:.2}", fps));
    }
}
