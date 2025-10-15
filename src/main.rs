use macroquad::prelude::{camera::mouse, *};
use rdev::display_size;
use std::fmt;

fn should_quit() -> bool {
    is_key_down(KeyCode::Q)
        && (is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl))
}

fn draw_fps() {
    draw_text(&get_fps().to_string(), 20.0, 20.0, 30.0, WHITE);
}

pub trait RectExt {
    fn from_y(&mut self, y: f32) -> &mut Rect;
    fn with_height(&mut self, w: f32) -> &mut Rect;
    fn with_width(&mut self, h: f32) -> &mut Rect;
    fn clip_by(&mut self, d: f32) -> &mut Rect;
}

impl RectExt for Rect {
    fn from_y(&mut self, y: f32) -> &mut Rect {
        self.y = y;
        self
    }

    fn with_width(&mut self, w: f32) -> &mut Rect {
        self.w = w;
        self
    }

    fn with_height(&mut self, h: f32) -> &mut Rect {
        self.h = h;
        self
    }

    fn clip_by(&mut self, d: f32) -> &mut Rect {
        self.x += d;
        self.y += d;
        self.w -= d * 2.0;
        self.h -= d * 2.0;
        self
    }
}

struct SizeRatio;
impl SizeRatio {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Rect {
        Rect {
            x: screen_width() * x,
            y: screen_height() * y,
            w: screen_width() * w,
            h: screen_height() * h,
        }
    }

    pub fn get_x(val: f32) -> f32 {
        screen_width() * val
    }

    pub fn get_y(val: f32) -> f32 {
        screen_height() * val
    }
}

#[derive(Debug)]
pub enum CardType {
    Attack,
    Defense,
    Magic,
    Support,
}

pub struct CardImage {
    rows: u16,
    cols: u16,
    img: Vec<bool>,
    cell_size: u16,
}
impl CardImage {
    pub fn new(rows: u16, cols: u16) -> Self {
        let img = std::iter::repeat(false).take(20).collect();

        Self {
            rows,
            cols,
            img,
            cell_size: 20,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ResizeEdge {
    Left,
    Right,
    Top,
    Bottom,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    None,
}

/// Check if mouse is near the edge of a rect
fn mouse_near_edge(rect: Rect, mouse_x: f32, mouse_y: f32, edge_margin: f32) -> ResizeEdge {
    let left = (mouse_x - rect.x).abs() < edge_margin;
    let right = (mouse_x - (rect.x + rect.w)).abs() < edge_margin;
    let top = (mouse_y - rect.y).abs() < edge_margin;
    let bottom = (mouse_y - (rect.y + rect.h)).abs() < edge_margin;

    match (top, bottom, left, right) {
        (true, _, true, _) => ResizeEdge::TopLeft,
        (true, _, _, true) => ResizeEdge::TopRight,
        (_, true, true, _) => ResizeEdge::BottomLeft,
        (_, true, _, true) => ResizeEdge::BottomRight,
        (true, _, false, false) => ResizeEdge::Top,
        (_, true, false, false) => ResizeEdge::Bottom,
        (false, false, true, _) => ResizeEdge::Left,
        (false, false, _, true) => ResizeEdge::Right,
        _ => ResizeEdge::None,
    }
}

/// Example: resize rect while dragging
fn resize_rect(rect: &mut Rect, edge: ResizeEdge, mouse_dx: f32, mouse_dy: f32) {
    match edge {
        ResizeEdge::Left => {
            rect.x += mouse_dx;
            rect.w -= mouse_dx;
        }
        ResizeEdge::Right => {
            rect.w += mouse_dx;
        }
        ResizeEdge::Top => {
            rect.y += mouse_dy;
            rect.h -= mouse_dy;
        }
        ResizeEdge::Bottom => {
            rect.h += mouse_dy;
        }
        ResizeEdge::TopLeft => {
            rect.x += mouse_dx;
            rect.w -= mouse_dx;
            rect.y += mouse_dy;
            rect.h -= mouse_dy;
        }
        ResizeEdge::TopRight => {
            rect.w += mouse_dx;
            rect.y += mouse_dy;
            rect.h -= mouse_dy;
        }
        ResizeEdge::BottomLeft => {
            rect.x += mouse_dx;
            rect.w -= mouse_dx;
            rect.h += mouse_dy;
        }
        ResizeEdge::BottomRight => {
            rect.w += mouse_dx;
            rect.h += mouse_dy;
        }
        ResizeEdge::None => {}
    }

    // Prevent negative size
    if rect.w < 10.0 {
        rect.w = 10.0;
    }
    if rect.h < 10.0 {
        rect.h = 10.0;
    }
}

pub struct Card {
    img: CardImage,
    pub name: String,
    pub desc: String,
    pub power: u32,
    pub card_type: CardType,
    rect: Rect,
}

impl Card {
    pub fn new(
        img: CardImage,
        name: &str,
        desc: &str,
        power: u32,
        card_type: CardType,
        rect: Rect,
    ) -> Self {
        Self {
            img,
            name: name.to_string(),
            desc: desc.to_string(),
            power,
            card_type,
            rect,
        }
    }

    pub fn update(&mut self, mouse: &mut MouseTracker) {
        let mut resize_edge = ResizeEdge::None;
        let edge_margin = 10.0;

        let (mx, my) = mouse_position();

        if (is_mouse_button_down(MouseButton::Left)) {
            resize_edge = mouse_near_edge(self.rect, mx, my, edge_margin);
            if resize_edge != ResizeEdge::None {
                let (dx, dy) = mouse.delta();
                resize_rect(&mut self.rect, resize_edge, dx, dy);
            }
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Card: {} Type: {:?} Power: {}\nDescription: {}\n",
            self.name, self.card_type, self.power, self.desc
        )
    }
}

struct Shape;
impl Shape {
    fn draw_rect(rect: Rect, color: Color) {
        draw_rectangle(rect.x, rect.y, rect.w, rect.h, color);
    }
}

fn draw_text_in_rect_char_wrap(text: &str, rect: Rect, font: &Font, font_size: u16, color: Color) {
    let scale = 1.0;
    let mut line = String::new();
    let mut y = rect.y + font_size as f32;

    for ch in text.chars() {
        let test_line = format!("{}{}", line, ch);
        let dims = measure_text(&test_line, Some(font), font_size, scale);

        if dims.width < rect.w {
            line = test_line;
        } else {
            // Draw the line
            draw_text_ex(
                &line,
                rect.x,
                y,
                TextParams {
                    font: Some(font),
                    font_size,
                    color,
                    ..Default::default()
                },
            );

            // Move to next line
            line = ch.to_string();
            y += font_size as f32 * 1.2;
        }

        // Stop if we exceed height
        if y > rect.y + rect.h {
            break;
        }
    }

    // Draw last line
    if !line.is_empty() && y <= rect.y + rect.h {
        draw_text_ex(
            &line,
            rect.x,
            y,
            TextParams {
                font: Some(font),
                font_size,
                color,
                ..Default::default()
            },
        );
    }

    // Optional debug rectangle
    // draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, GRAY);
}

fn draw_text_in_rect(text: &str, rect: Rect, font_size: u16, color: Color) {
    let scale = 1.0;
    let words: Vec<&str> = text.split_whitespace().collect();

    let mut line = String::new();
    let mut y = rect.y + font_size as f32;

    for word in words {
        let test_line = if line.is_empty() {
            word.to_string()
        } else {
            format!("{} {}", line, word)
        };

        let dims = measure_text(&test_line, None, font_size, scale);

        // If line fits, keep adding words
        if dims.width < rect.w {
            line = test_line;
        } else {
            // Draw current line
            draw_text(&line, rect.x, y, font_size as f32, color);
            // Start new line
            line = word.to_string();
            y += font_size as f32 * 1.2; // line spacing
        }

        // Stop drawing if we exceed rect height
        if y > rect.y + rect.h {
            break;
        }
    }

    // Draw last line if any left
    if !line.is_empty() && y <= rect.y + rect.h {
        draw_text(&line, rect.x, y, font_size as f32, color);
    }

    // Optional: draw border to visualize rect
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, GRAY);
}

struct Renderer;
impl Renderer {
    fn render_grid(x: f32, y: f32, rows: u16, cols: u16, cell_size: u16) {
        for r in 0..rows {
            let py = y + (r * cell_size) as f32;
            draw_line(x, py, x + (cell_size * cols) as f32, py, 2.0, BLACK);
        }

        for c in 0..cols {
            let px = x + (c * cell_size) as f32;
            draw_line(px, y, px, y + (cell_size * rows) as f32, 2.0, BLACK);
        }
    }

    fn render_card_img(img: &CardImage) {
        let mut y: f32 = 0.0;
        let mut x: f32 = 0.0;
        for r in 0..img.rows {
            y = (r * img.cell_size) as f32;
            for c in 0..img.cols {
                x = (c * img.cell_size) as f32;

                draw_rectangle(x, y, img.cell_size as f32, img.cell_size as f32, BLUE);
            }
        }

        Renderer::render_grid(0.0, 0.0, img.rows, img.cols, img.cell_size);
    }

    fn render_card(card: &Card, font: &Font) {
        let offset = 4.0;
        let border = 4.0;
        let font_size = 30.0;
        let dec_font_size = 20.0;

        let mut img = card.rect.clone();
        img.with_height(card.rect.h / 2.0).clip_by(border);

        Shape::draw_rect(card.rect, DARKGRAY);
        // Shape::draw_rect(img, BLUE);

        let mut y = img.h + img.y + font_size;

        draw_text_ex(
            &card.name,
            img.x,
            y,
            TextParams {
                font: Some(&font),
                font_size: font_size as u16,
                color: WHITE,
                ..Default::default()
            },
        );

        y += dec_font_size + offset;

        draw_text_in_rect_char_wrap(
            &card.desc,
            Rect::new(img.x, y, img.w, img.h),
            font,
            dec_font_size as u16,
            WHITE,
        );

        Renderer::render_card_img(&card.img);
    }
}

fn handle_grid_click(grid: &mut Vec<bool>, cols: usize, rows: usize, cell_size: f32) {
    if is_mouse_button_pressed(MouseButton::Left) {
        let (mx, my) = mouse_position();

        let col = (mx / cell_size) as usize;
        let row = (my / cell_size) as usize;

        if col < cols && row < rows {
            let idx = row * cols + col;
            grid[idx] = !grid[idx]; // toggle value
        }
    }
}

fn window_conf() -> Conf {
    let default_win_size = (800, 600);
    let (mut width, mut height) = default_win_size;
    // (width, height) = display_size().unwrap_or(default_win_size);
    println!("width: {} heigt: {}", width, height);

    Conf {
        window_title: "Opinion".to_owned(),
        // fullscreen: true,
        window_resizable: true,
        window_width: width as i32,
        window_height: height as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let card_width = SizeRatio::get_x(0.4);
    let card_height = SizeRatio::get_y(0.6);

    let mut fireball = Card::new(
        CardImage::new(10, 10),
        "Fireball",
        "Deals fire damage to enemies.",
        50,
        CardType::Magic,
        Rect::new(
            screen_width() / 2.0 - card_width / 2.0,
            screen_height() / 2.0 - card_height / 2.0,
            card_width,
            card_height,
        ),
    );

    println!("{}", fireball);

    let font = load_ttf_font("/usr/share/fonts/TTF/Roboto-Regular.ttf")
        .await
        .unwrap();

    let mut mouse = MouseTracker::new();

    loop {
        clear_background(Color::from_rgba(31, 31, 31, 255));

        if should_quit() {
            break;
        }

        // update
        fireball.update(&mut mouse);

        Renderer::render_card(&fireball, &font);

        mouse.update();
        draw_fps();
        next_frame().await
    }
}

/// Stores last frame’s mouse position and gives delta
pub struct MouseTracker {
    last_pos: (f32, f32),
}

impl MouseTracker {
    pub fn new() -> Self {
        Self {
            last_pos: mouse_position(),
        }
    }

    pub fn update(&mut self) {
        self.last_pos = mouse_position();
    }

    pub fn delta(&mut self) -> (f32, f32) {
        let (x, y) = mouse_position();
        let dx = x - self.last_pos.0;
        let dy = y - self.last_pos.1;
        self.last_pos = (x, y);
        (dx, dy)
    }
}
