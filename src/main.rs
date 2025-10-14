use macroquad::prelude::*;

fn should_quit() -> bool {
    is_key_down(KeyCode::Q)
        && (is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl))
}

fn draw_fps() {
    draw_text(&get_fps().to_string(), 20.0, 20.0, 30.0, WHITE);
}

#[macroquad::main("MyGame")]
async fn main() {
    loop {
        clear_background(Color::from_rgba(31, 31, 31, 255));

        if should_quit() {
            break;
        }

        draw_fps();
        next_frame().await
    }
}
