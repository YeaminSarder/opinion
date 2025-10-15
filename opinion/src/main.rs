use macroquad::prelude::*;
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

#[derive(Debug)]
pub enum CardType {
    Attack,
    Defense,
    Magic,
    Support,
}

pub struct Card {
    // img: Vec<bool>
    pub name: String,
    pub desc: String,
    pub power: u32,
    pub card_type: CardType,
    rect: Rect,
}

impl Card {
    pub fn new(name: &str, desc: &str, power: u32, card_type: CardType, rect: Rect) -> Self {
        Self {
            name: name.to_string(),
            desc: desc.to_string(),
            power,
            card_type,
            rect,
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

struct Renderee;

impl Renderee {
    fn render_card(card: &Card) {
        let offset = 4.0;
        let border = 4.0;
        let font_size = 30.0;
        let dec_font_size = 20.0;

        let mut img = card.rect.clone();
        img.with_height(card.rect.h / 2.0).clip_by(border);

        Shape::draw_rect(card.rect, DARKGRAY);
        Shape::draw_rect(img, BLUE);

        let mut y = img.h + img.y + font_size;

        draw_text(&card.name, img.x, y, font_size, WHITE);

        y += font_size + offset;

        draw_text(&card.desc, img.x, y, dec_font_size, WHITE);
        // draw_multiline_text(&card.desc, img.x, y, dec_font_size, None, WHITE);
    }
}

fn render() {}

#[macroquad::main("MyGame")]
async fn main() {
    let card_width = 150.0;
    let card_height = 250.0;

    let fireball = Card::new(
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

    loop {
        clear_background(Color::from_rgba(31, 31, 31, 255));

        if should_quit() {
            break;
        }

        Renderee::render_card(&fireball);

        draw_fps();
        next_frame().await
    }
}
