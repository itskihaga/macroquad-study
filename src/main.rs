use macroquad::prelude::*;
const SQUARES: i16 = 32;

type Point = (i16, i16);

struct SizeConfig {
    game_size: f32,
    square_size: f32,
    offset_x: f32,
    offset_y: f32, 
}

impl SizeConfig {
    fn new() -> Self {
        let game_size = screen_width().min(screen_height());
        let offset_x = (screen_width() - game_size) / 2. + 10.;
        let offset_y = (screen_height() - game_size) / 2. + 10.;
        let square_size = (screen_height() - offset_y * 2.) / SQUARES as f32;
        Self {
            game_size,
            square_size,
            offset_x,
            offset_y,
        }
    }
}

fn draw_background(config: &SizeConfig) {

    draw_rectangle(config.offset_x, config.offset_y, config.game_size - 20., config.game_size - 20., WHITE);

    for i in 1..SQUARES {
        draw_line(
            config.offset_x,
            config.offset_y + config.square_size * i as f32,
            screen_width() - config.offset_x,
            config.offset_y + config.square_size * i as f32,
            1.,
            LIGHTGRAY,
        );
    }

    for i in 1..SQUARES {
        draw_line(
            config.offset_x + config.square_size * i as f32,
            config.offset_y,
            config.offset_x + config.square_size * i as f32,
            screen_height() - config.offset_y,
            1.,
            LIGHTGRAY,
        );
    }
}

fn draw_fruit(fruit: &Point, config: &SizeConfig) {
    draw_rectangle(
        config.offset_x + config.square_size * fruit.0 as f32,
        config.offset_y + config.square_size * fruit.1 as f32,
        config.square_size,
        config.square_size,
        RED,
    )
}

#[macroquad::main("Snake")]
async fn main() {
    let size_config = SizeConfig::new();
    let fruit: Point = (rand::gen_range(0, SQUARES), rand::gen_range(0, SQUARES));
    loop {
        clear_background(LIGHTGRAY);
        draw_background(&size_config);
        draw_fruit(&fruit, &size_config);
        next_frame().await
    }
}
