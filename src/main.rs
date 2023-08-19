use std::{thread,time::Duration};
use macroquad::{window,shapes,color, input, time::get_time, text,rand, prelude::KeyCode, miniquad::date, telemetry::frame};

const GAME_WIDTH: usize = 10;
const GAME_HEIGHT: usize = 20;
const SPEED: f64 = 0.1;
const X_OFFSET: f32 = 100.0;
const Y_OFFSET: f32 = 20.0;

fn window_conf() -> window::Conf {
    window::Conf {
        window_title: "Snake".to_owned(),
        window_width: 700,
        window_height: 1000,
        window_resizable: false,
        ..Default::default()
    }
}
enum MainState {
    StartMenu,
    SnakeLoop,
    GameOver,
    ExitGame,
}
#[macroquad::main(window_conf)]
async fn main() {
    let main_state = MainState::SnakeLoop;
    let scale: f32 = {
        let x_scale: f32 = window::screen_width()/GAME_WIDTH as f32 - X_OFFSET*2.0/GAME_WIDTH as f32;
        let y_scale: f32 = window::screen_height()/GAME_HEIGHT as f32 - Y_OFFSET*2.0/GAME_HEIGHT as f32;
        if x_scale < y_scale {x_scale}
        else {y_scale}
    };
    let tetris_grid: TetrisGrid = TetrisGrid{grid: [None; GAME_WIDTH * GAME_HEIGHT]};
    rand::srand(date::now() as u64);
    match main_state {
        MainState::StartMenu => (),
        MainState::SnakeLoop => {
            let current_piece = Piece::new(PieceType::rand());
            let mut last_time = get_time();
            loop {
                // input
                match input::get_last_key_pressed() {
                    Some(KeyCode::Left) => (),
                    Some(KeyCode::Escape) => break,
                    _ => (),
                }
                if get_time() - last_time > SPEED {
                    last_time = get_time();
                }
                // render stuff
                window::clear_background(color::DARKGRAY);
                draw_blocks(&tetris_grid.grid, scale);      
                window::next_frame().await;
                thread::sleep(Duration::from_millis(15));;
            }
        },
        MainState::GameOver => (),
        MainState::ExitGame => (),
    }
}

struct Piece([Option<color::Color>; 9], PieceType);
impl Piece {
    fn new (piece_type: PieceType) -> Piece {
        let c = piece_type.get_color();
        match piece_type {
            PieceType::I => Piece([None, Some(c), None,
                        None, Some(c), None,
                        None, Some(c), None], PieceType::I),
            PieceType::O => Piece([Some(c),Some(c),None,
                        Some(c),Some(c),None,
                        None, None, None], PieceType::O),
            PieceType::S => Piece([None,  Some(c),Some(c),
                        Some(c), Some(c), None,
                        None,None,None], PieceType::S),
            PieceType::Z => Piece([Some(c),Some(c), None,
                        None, Some(c), Some(c),
                        None, None, None], PieceType::Z),
            PieceType::L => Piece([Some(c),None,None,
                        Some(c), None, None,
                        Some(c),Some(c),None], PieceType::L),
            PieceType::J => Piece([None, None,Some(c),
                        None, None, Some(c),
                        None,Some(c),Some(c)], PieceType::J),
            PieceType::T => Piece([Some(c),Some(c),Some(c),
                        None, Some(c), None,
                        None, None, None], PieceType::T),
        }
    }
    fn rotate_right(&mut self) {
        if let PieceType::O = self.1 {
            return;
        }
        let mut new_area: [Option<color::Color>; 9] = [None; 9]; //think array of 3 by 3 quandrant 4
        for x in 0..3 {
            new_area[x * 3 + 2] = self.0[x]; // sets right 3 to top 3
        }
        for y in 0..3 {
            new_area[8 - y] = self.0[y * 3 + 2]; // sets bottom 3 to right 3
        }
        for x in 0..3 {
            new_area[x * 3] = self.0[6 + x]; // sets left 3 to bottom 3
        }
        for y in 0..3 {
            new_area[2 - y] = self.0[y * 3]; // sets top 3 right 3
        }
        new_area[4] = self.0[4]; // sets middle to middle
        self.0 = new_area;
    }
    
}
enum PieceType {
    O,
    I,
    S,
    Z,
    L,
    J,
    T,
}
impl PieceType {
    fn get_color(&self) -> color::Color {
        match *self {
            PieceType::O => color::Color::from_rgba(208, 245, 22, 1),
            PieceType::I => color::Color::from_rgba(9, 180, 214,1),
            PieceType::S => color::Color::from_rgba(232, 12, 15, 1),
            PieceType::Z => color::Color::from_rgba(5, 153, 24,1),
            PieceType::L => color::Color::from_rgba(245, 178, 22,1),
            PieceType::J => color::Color::from_rgba(240, 31, 205,1),
            PieceType::T => color::Color::from_rgba(113, 6, 158,1),
        }
    }
    fn rand() -> PieceType {
        let r_numb:u32 = rand::rand() % 7;
        match r_numb {
            0 => PieceType::O,
            1 => PieceType::I,
            2 => PieceType::S,
            3 => PieceType::Z,
            4 => PieceType::L,
            5 => PieceType::J,
            6 => PieceType::T,
            _ => panic!("error getting random piecetype"),
        }
    }
}

struct TetrisGrid {
    grid: [Option<color::Color>; GAME_WIDTH*GAME_HEIGHT],
}

impl TetrisGrid {
    fn delete_rows(&mut self, start_r: usize, end_r: usize) {
        /* Clears rows then shifts everything above cleared rows down*/
        for i in GAME_WIDTH * start_r..GAME_WIDTH * end_r {
            self.grid[i] = None;
        }
        self.grid[0..GAME_WIDTH * end_r].rotate_right(GAME_WIDTH * (end_r - start_r));
    }

    fn check_lines(&self) -> Option<(usize, usize)> {
        /* if there is a row to be deleted, it returns  the ammount
        of rows detected and the starting row location*/
        let mut tetris_count = 0;
        for y in 0..GAME_HEIGHT {
            let mut full_row = true;
            for x in 0..GAME_WIDTH {
                if self.grid[x * y].is_none() {
                    full_row = false;
                    break;
                }
            }
            if full_row {
                tetris_count += 1;
            } else if tetris_count > 0 {
                return Some((tetris_count, y - tetris_count));
            }
        }
        if tetris_count > 0 {
            Some((tetris_count, GAME_HEIGHT - tetris_count))
        } else {
            None
        }
    }
}
fn draw_blocks(grid: &[Option<color::Color>; GAME_WIDTH * GAME_HEIGHT], scale: f32) {
    for y in 0..GAME_HEIGHT {
        for x in 0..GAME_WIDTH {
            draw_block(grid[x * y], x as f32, y as f32, scale);
        }
    }
}

fn draw_block(block_color: Option<color::Color>, x: f32, y: f32, scale: f32) {
    match block_color {
        Some(c) => shapes::draw_rectangle(x * scale + X_OFFSET, y * scale + Y_OFFSET, scale, scale, c),
        None => shapes::draw_rectangle_lines(x * scale + X_OFFSET, y * scale + Y_OFFSET, scale, scale, 2.0, color::BLACK),
    }
}