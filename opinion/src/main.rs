#![allow(dead_code)]

use macroquad::prelude::coroutines::tweens::linear;
//use macroquad::prelude::{camera::mouse};
use macroquad::prelude::{self as mcp, debug};
//use std::{fmt, hint::select_unpredictable};
use rand::seq::SliceRandom;
use rand::{Rng, distr::Uniform, rng};
use std::fmt; // for choose()

//use macroquad::hash;
//use macroquad::ui::root_ui;
//use macroquad::ui::widgets::Window;
//use macroquad::ui::{Id, Ui};

// #[cfg(not(target_arch = "wasm32"))]
// #[allow(unused_imports)]
// use rdev::display_size;

static mut __UID: u32 = 0;
pub fn new_uid() -> u32 {
    unsafe { __UID += 1 }
    unsafe { __UID }
}

fn should_quit() -> bool {
    mcp::is_key_down(mcp::KeyCode::Q)
        && (mcp::is_key_down(mcp::KeyCode::LeftControl)
            || mcp::is_key_down(mcp::KeyCode::RightControl))
}

fn draw_fps() {
    mcp::draw_text(&mcp::get_fps().to_string(), 20.0, 20.0, 30.0, mcp::WHITE);
}

pub trait RectExt {
    fn from_y(&mut self, y: f32) -> &mut mcp::Rect;
    fn with_height(&mut self, w: f32) -> &mut mcp::Rect;
    fn with_width(&mut self, h: f32) -> &mut mcp::Rect;
    fn clip_by(&mut self, d: f32) -> &mut mcp::Rect;

    fn update_pos_n_size(&mut self, x: f32, y: f32, w: f32, h: f32);

    fn render(&self);
}

impl RectExt for mcp::Rect {
    fn from_y(&mut self, y: f32) -> &mut mcp::Rect {
        self.y = y;
        self
    }

    fn with_width(&mut self, w: f32) -> &mut mcp::Rect {
        self.w = w;
        self
    }

    fn with_height(&mut self, h: f32) -> &mut mcp::Rect {
        self.h = h;
        self
    }

    fn clip_by(&mut self, d: f32) -> &mut mcp::Rect {
        self.x += d;
        self.y += d;
        self.w -= d * 2.0;
        self.h -= d * 2.0;
        self
    }

    fn update_pos_n_size(&mut self, x: f32, y: f32, w: f32, h: f32) {
        self.x = x;
        self.y = y;
        self.w = w;
        self.h = h;
    }

    fn render(&self) {
        mcp::draw_rectangle(self.x, self.y, self.w, self.h, mcp::BROWN);
    }
}

struct SizeRatio;
impl SizeRatio {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> mcp::Rect {
        mcp::Rect {
            x: mcp::screen_width() * x,
            y: mcp::screen_height() * y,
            w: mcp::screen_width() * w,
            h: mcp::screen_height() * h,
        }
    }

    pub fn get_x(val: f32) -> f32 {
        mcp::screen_width() * val
    }

    pub fn get_y(val: f32) -> f32 {
        mcp::screen_height() * val
    }

    pub fn get(x: f32, y: f32) -> (f32, f32) {
        (mcp::screen_width() * x, mcp::screen_height() * y)
    }
}

#[derive(Debug, Clone)]
pub enum CardType {
    Attack,
    Defense,
    Magic,
    Support,
}

#[derive(Clone)]
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
fn mouse_near_edge(rect: mcp::Rect, mouse_x: f32, mouse_y: f32, edge_margin: f32) -> ResizeEdge {
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
fn resize_rect(rect: &mut mcp::Rect, edge: ResizeEdge, mouse_dx: f32, mouse_dy: f32) {
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

#[derive(Clone)]
pub struct Card {
    id: u32,
    img: CardImage,
    pub name: String,
    pub desc: String,
    pub power: u32,
    pub card_type: CardType,
    rect: mcp::Rect,
}

impl Card {
    pub fn new(
        img: CardImage,
        name: &str,
        desc: &str,
        power: u32,
        card_type: CardType,
        rect: mcp::Rect,
    ) -> Self {
        Self {
            id: new_uid(),
            img,
            rect,
            power,
            card_type,
            name: name.to_string(),
            desc: desc.to_string(),
        }
    }

    pub fn update(&mut self, mouse: &mut Mouse, ind: usize) {
        let edge_margin = 10.0;

        let (mx, my) = mcp::mouse_position();

        if mcp::is_mouse_button_pressed(mcp::MouseButton::Left) {
            let edge = mouse_near_edge(self.rect, mx, my, edge_margin);
            if edge != ResizeEdge::None {
                mouse.grab_it(Obj::Card(ind), Action::Resize(edge));
                return;
            }

            if self.rect.contains(mcp::Vec2::new(mx, my)) {
                mouse.grab_it(Obj::Card(ind), Action::Move);
            }
        }
    }

    fn resize(&mut self, (dx, dy): (f32, f32), edge: ResizeEdge) {
        resize_rect(&mut self.rect, edge, dx, dy);
    }

    fn move_to(&mut self, (mx, my): (f32, f32)) {
        self.rect.move_to(mcp::Vec2::new(mx, my));
    }
}

impl Grabbable for Card {
    fn resize(&mut self, (dx, dy): (f32, f32), edge: &ResizeEdge) {
        resize_rect(&mut self.rect, *edge, dx, dy);
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
    fn draw_rect(rect: mcp::Rect, color: mcp::Color) {
        mcp::draw_rectangle(rect.x, rect.y, rect.w, rect.h, color);
    }
}

fn draw_text_in_rect_char_wrap(
    text: &str,
    rect: mcp::Rect,
    font: &mcp::Font,
    font_size: u16,
    color: mcp::Color,
) {
    let scale = 1.0;
    let mut line = String::new();
    let mut y = rect.y + font_size as f32;

    for ch in text.chars() {
        let test_line = format!("{}{}", line, ch);
        let dims = mcp::measure_text(&test_line, Some(font), font_size, scale);

        if dims.width < rect.w {
            line = test_line;
        } else {
            // Draw the line
            mcp::draw_text_ex(
                &line,
                rect.x,
                y,
                mcp::TextParams {
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
        mcp::draw_text_ex(
            &line,
            rect.x,
            y,
            mcp::TextParams {
                font: Some(font),
                font_size,
                color,
                ..Default::default()
            },
        );
    }

    // Optional debug rectangle
    // draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, mcp::GRAY);
}

fn draw_text_in_rect(text: &str, rect: mcp::Rect, font_size: u16, color: mcp::Color) {
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

        let dims = mcp::measure_text(&test_line, None, font_size, scale);

        // If line fits, keep adding words
        if dims.width < rect.w {
            line = test_line;
        } else {
            // Draw current line
            mcp::draw_text(&line, rect.x, y, font_size as f32, color);
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
        mcp::draw_text(&line, rect.x, y, font_size as f32, color);
    }

    // Optional: draw border to visualize rect
    mcp::draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, mcp::GRAY);
}

struct Renderer;
impl Renderer {
    fn render_grid(x: f32, y: f32, rows: u16, cols: u16, cell_size: u16) {
        for r in 0..rows {
            let py = y + (r * cell_size) as f32;
            mcp::draw_line(x, py, x + (cell_size * cols) as f32, py, 2.0, mcp::BLACK);
        }

        for c in 0..cols {
            let px = x + (c * cell_size) as f32;
            mcp::draw_line(px, y, px, y + (cell_size * rows) as f32, 2.0, mcp::BLACK);
        }
    }

    fn render_card_img(img: &CardImage) {
        for r in 0..img.rows {
            let y = (r * img.cell_size) as f32;
            for c in 0..img.cols {
                let x = (c * img.cell_size) as f32;

                mcp::draw_rectangle(x, y, img.cell_size as f32, img.cell_size as f32, mcp::BLUE);
            }
        }

        Renderer::render_grid(0.0, 0.0, img.rows, img.cols, img.cell_size);
    }

    fn render_card_minimal(card: &Card, font: &mcp::Font) {
        let offset = 4.0;
        let border = 4.0;
        let font_size = card.rect.h / 6.0;
        let dec_font_size = font_size / 1.5;

        let mut img = card.rect.clone();
        img.with_height(card.rect.h / 2.0).clip_by(border);

        Shape::draw_rect(card.rect, mcp::DARKGRAY);
        // Shape::draw_rect(img, BLUE);

        // for c in card.name.chars() {
        //     let dim = mcp::measure_text(&c.to_string(), Some(font), font_size as u16, 1.0);
        //     println!("dim: {} {:?}", String::from(c), dim);
        // }

        let mut y = img.h + img.y + font_size;

        mcp::draw_text_ex(
            &card.name,
            img.x,
            y,
            mcp::TextParams {
                font: Some(&font),
                font_size: font_size as u16,
                color: mcp::WHITE,
                ..Default::default()
            },
        );

        y += dec_font_size + offset;

        draw_text_in_rect_char_wrap(
            &card.desc,
            mcp::Rect::new(img.x, y, img.w, img.h),
            font,
            dec_font_size as u16,
            mcp::WHITE,
        );

        // Renderer::render_card_img(&card.img);
    }

    fn render_card(card: &Card, font: &mcp::Font) {
        let offset = 4.0;
        let border = 4.0;
        let font_size = 30.0;
        let dec_font_size = 20.0;

        let mut img = card.rect.clone();
        img.with_height(card.rect.h / 2.0).clip_by(border);

        Shape::draw_rect(card.rect, mcp::DARKGRAY);
        // Shape::draw_rect(img, BLUE);

        // for c in card.name.chars() {
        //     let dim = mcp::measure_text(&c.to_string(), Some(font), font_size as u16, 1.0);
        //     println!("dim: {} {:?}", String::from(c), dim);
        // }

        let mut y = img.h + img.y + font_size;

        mcp::draw_text_ex(
            &card.name,
            img.x,
            y,
            mcp::TextParams {
                font: Some(&font),
                font_size: font_size as u16,
                color: mcp::WHITE,
                ..Default::default()
            },
        );

        y += dec_font_size + offset;

        draw_text_in_rect_char_wrap(
            &card.desc,
            mcp::Rect::new(img.x, y, img.w, img.h),
            font,
            dec_font_size as u16,
            mcp::WHITE,
        );

        Renderer::render_card_img(&card.img);
    }
}

fn handle_grid_click(grid: &mut Vec<bool>, cols: usize, rows: usize, cell_size: f32) {
    if mcp::is_mouse_button_pressed(mcp::MouseButton::Left) {
        let (mx, my) = mcp::mouse_position();

        let col = (mx / cell_size) as usize;
        let row = (my / cell_size) as usize;

        if col < cols && row < rows {
            let idx = row * cols + col;
            grid[idx] = !grid[idx]; // toggle value
        }
    }
}

fn window_conf() -> mcp::Conf {
    let default_win_size = (800, 600);
    let (width, height) = default_win_size;
    // (width, height) = display_size().unwrap_or(default_win_size);
    println!("width: {} heigt: {}", width, height);

    mcp::Conf {
        window_title: "Opinion".to_owned(),
        // fullscreen: true,
        window_resizable: true,
        window_width: width as i32,
        window_height: height as i32,
        ..Default::default()
    }
}

enum Tern {
    Player1,
    Player2,
}

pub struct Player {
    hand: Vec<Card>,
    library: Vec<Card>,
    graveyard: Vec<Card>,

    arena_rect: mcp::Rect,
    hand_rect: mcp::Rect,
    def_card_size: (f32, f32),
}

impl Player {
    pub fn new(col: &CardCollection) -> Self {
        let mut library = col.get_random(20);
        for c in &library {
            debug!("{}", c);
        }

        let hand = library.drain(library.len().saturating_sub(3)..).collect();

        println!("---------------------------");

        for c in &hand {
            debug!("{}", c);
        }

        let arena_rect = SizeRatio::new(0.2, 0.52, 0.6, 0.18);
        let hand_rect = SizeRatio::new(0.1, 0.78, 0.8, 0.18);
        let def_card_size = (hand_rect.w / 7.0, hand_rect.h);

        let mut ret = Self {
            hand,
            library,
            graveyard: Vec::new(),
            def_card_size,

            arena_rect,
            hand_rect,
        };

        ret.update_card_position();

        ret
    }

    pub fn update_card_position(&mut self) {
        let x = self.hand_rect.x;
        let offset = 2.0;

        for (ind, card) in self.hand.iter_mut().enumerate() {
            let px = x + ind as f32 * (self.def_card_size.0 + offset);
            card.rect.update_pos_n_size(
                px,
                self.hand_rect.y,
                self.def_card_size.0,
                self.def_card_size.1,
            );
        }
    }

    fn render_arena(&self, font: &mcp::Font) {
        self.arena_rect.render();

        let arena_text = "Arena";
        let arena_text_width =
            mcp::measure_text(arena_text, Some(font), self.arena_rect.h as u16, 1.);

        mcp::draw_text_ex(
            arena_text,
            self.arena_rect.center().x - arena_text_width.width / 2.0,
            self.arena_rect.bottom() - 20.0,
            mcp::TextParams {
                font: Some(&font),
                font_size: self.arena_rect.h as u16,
                color: mcp::WHITE,
                ..Default::default()
            },
        );
    }

    pub fn render_hand(&self, font: &mcp::Font) {
        self.hand_rect.render();
        self.render_arena(font);

        for card in self.hand.iter() {
            Renderer::render_card_minimal(&card, font);
        }
    }
}

pub struct Game {
    player1: Player,
    player2: Player,
    turn: Tern,

    is_running: bool,
    card_collection: Vec<Card>,
}

pub struct CardCollection {
    col: Vec<Card>,
}

impl CardCollection {
    pub fn generate() -> Self {
        Self {
            col: generate_card_collection(),
        }
    }

    pub fn get_random(&self, count: usize) -> Vec<Card> {
        rand::rng()
            .sample_iter(Uniform::new_inclusive(0, self.col.len() - 1).unwrap())
            .take(count)
            .map(|ind| self.col[ind].clone())
            .collect()
    }
}

fn generate_card_collection() -> Vec<Card> {
    let mut col = vec![];

    let card_width = SizeRatio::get_x(0.4);
    let card_height = SizeRatio::get_y(0.6);

    let def_rect = mcp::Rect::new(0., 0., card_width, card_height);

    let (img_rows, img_cols) = (10, 10);
    let img = CardImage::new(img_rows, img_cols);

    col.push(Card::new(
        img.clone(),
        "Smithy",
        "Card draw 1",
        0,
        CardType::Support,
        def_rect,
    ));

    col.push(Card::new(
        img.clone(),
        "Smithy2",
        "Card draw 2",
        0,
        CardType::Support,
        def_rect,
    ));

    col.push(Card::new(
        img.clone(),
        "Smithy3",
        "Card draw 3",
        0,
        CardType::Support,
        def_rect,
    ));

    col
}

#[cfg(not(target_arch = "wasm32"))]
const FONT_PATH: &str = "public/assets/font/JetBrainsMono-Medium.ttf";

#[cfg(target_arch = "wasm32")]
const FONT_PATH: &str = "assets/font/JetBrainsMono-Medium.ttf";

#[macroquad::main(window_conf)]
async fn main() {
    let card_width = SizeRatio::get_x(0.4);
    let card_height = SizeRatio::get_y(0.6);

    let fireball = Card::new(
        CardImage::new(10, 10),
        "Fireball",
        "Deals fire damage to enemies.",
        50,
        CardType::Magic,
        mcp::Rect::new(
            mcp::screen_width() / 2.0 - card_width / 2.0,
            mcp::screen_height() / 2.0 - card_height / 2.0,
            card_width,
            card_height,
        ),
    );

    println!("{}", fireball);

    let mut cards = vec![fireball];

    let mut card_collection = CardCollection::generate();

    let mut player1 = Player::new(&mut card_collection);

    let font = mcp::load_ttf_font(FONT_PATH).await.unwrap();

    let mut mouse = Mouse::new();

    loop {
        mcp::clear_background(mcp::Color::from_rgba(31, 31, 31, 255));

        if should_quit() {
            break;
        }

        // Window::new(hash!(), vec2(20., 20.), vec2(420., 400.))
        //     .label("Particles")
        //     .close_button(true)
        //     .ui(&mut root_ui(), |ui| {});

        //update
        // for (ind, card) in cards.iter_mut().enumerate() {
        //     card.update(&mut mouse, ind);
        // }
        //
        // for card in cards.iter() {
        //     Renderer::render_card(&card, &font);
        // }

        player1.render_hand(&font);

        let mouse_contex = MouseContex {
            cards: Some(&mut cards),
        };

        mouse.update(mouse_contex);
        draw_fps();
        mcp::next_frame().await
    }
}

#[derive(Clone, Copy)]
pub enum Action {
    Resize(ResizeEdge),
    Move,
}

#[derive(Clone, Copy)]
pub enum Obj {
    Card(usize),
}

pub struct MouseContex<'a> {
    cards: Option<&'a mut Vec<Card>>,
}

trait Grabbable {
    fn resize(&mut self, delta: (f32, f32), edge: &ResizeEdge);
}

pub struct Mouse {
    last_pos: (f32, f32),
    grab: Option<(Obj, Action)>,
}

impl Mouse {
    pub fn new() -> Self {
        Self {
            last_pos: mcp::mouse_position(),
            grab: None,
        }
    }

    pub fn update(&mut self, ctx: MouseContex) {
        let delta = self.delta();

        if let Some((grab, act)) = self.grab {
            match (grab, ctx.cards) {
                (Obj::Card(ind), Some(cards)) => match act {
                    Action::Resize(edge) => cards[ind].resize(delta, edge),
                    Action::Move => cards[ind].move_to(mcp::mouse_position()),
                },
                (_, _) => {}
            }
        }

        if mcp::is_mouse_button_released(mcp::MouseButton::Left) {
            self.grab = None;
        }

        self.last_pos = mcp::mouse_position();
    }

    pub fn delta(&mut self) -> (f32, f32) {
        let (x, y) = mcp::mouse_position();
        let dx = x - self.last_pos.0;
        let dy = y - self.last_pos.1;
        // self.last_pos = (x, y);
        (dx, dy)
    }

    pub fn grab_it(&mut self, obj: Obj, act: Action) -> bool {
        if self.grab.is_some() {
            return false;
        }

        self.grab = Some((obj, act));
        true
    }
}
