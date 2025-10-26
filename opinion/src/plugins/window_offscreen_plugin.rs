use bevy::{
    app::{App, Plugin, Update},
    ecs::{message::MessageReader, system::Query},
    window::{PresentMode, Window, WindowFocused},
};

const PR: PresentMode = PresentMode::Fifo;

pub struct WindowOffscreen;

impl Plugin for WindowOffscreen {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_focus);
    }
}

/// Listen for focus changes and toggle VSync
fn handle_focus(mut ev_focus: MessageReader<WindowFocused>, mut windows: Query<&mut Window>) {
    for ev in ev_focus.read() {
        if let Ok(mut window) = windows.get_mut(ev.window) {
            if ev.focused && matches!(window.present_mode, PresentMode::Immediate) {
                window.present_mode = PR;
            } else if matches!(window.present_mode, PR) {
                window.present_mode = PresentMode::Immediate;
            }
        }
    }
}
