use std::{thread,time::Duration};
use macroquad::{window,shapes,color, input, time::get_time, text,rand, prelude::KeyCode, miniquad::date};

const GAME_WIDTH: usize = 10;
const GAME_HEIGHT: usize = 20;
const FALL_SPEED: f64 = 0.2; // smaller number faster
const FRAME_TIME: f64 = 1.0 / 60.0;
const X_OFFSET: f32 = 100.0;
const Y_OFFSET: f32 = 20.0;

fn window_conf() -> window::Conf {
    window::Conf {
        window_title: "Tetris".to_owned(),
        window_width: 700,
        window_height: 1000,
        window_resizable: false,
        ..Default::default()
    }
}
enum MainState {
    StartMenu,
    TetrisLoop,
    GameOver,
    ExitGame,
}
#[macroquad::main(window_conf)]
async fn main() {
    let main_state = MainState::TetrisLoop;
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
        MainState::TetrisLoop => {
            let mut current_piece = Piece::new(PieceType::rand());
            let mut last_time = get_time();
            loop {
                // input
                match input::get_last_key_pressed() {
                    Some(KeyCode::Left) => current_piece.x -= 1.0,
                    Some(KeyCode::Right) => current_piece.x += 1.0,
                    Some(KeyCode::Down) => current_piece.y += 1.0,
                    Some(KeyCode::R) => current_piece.rotate_right(),
                    Some(KeyCode::Escape) => break,
                    _ => (),
                }
                let delta_time = get_time() - last_time;
                if delta_time > FALL_SPEED {
                    last_time = get_time();
                    current_piece.y += 1.0;
                    if current_piece.y as usize > GAME_HEIGHT {
                        current_piece = Piece::new(PieceType::rand());
                    }
                }
                // render stuff
                window::clear_background(color::DARKGRAY);
                current_piece.draw(scale);
                tetris_grid.draw(scale);      
                window::next_frame().await;
                thread::sleep(Duration::from_millis(15));
            }
        },
        MainState::GameOver => (),
        MainState::ExitGame => (),
    }
}

struct Piece {
    grid:[Option<color::Color>; 9],
    p_type: PieceType,
    x: f32,
    y: f32,
}
impl Piece {
    fn new (piece_type: PieceType) -> Piece {
        let c = piece_type.get_color();
        let grid = match piece_type {
                PieceType::I => [None, Some(c), None,
                                None, Some(c), None,
                                None, Some(c), None],
                PieceType::O => [Some(c),Some(c),None,
                                Some(c),Some(c),None,
                                None, None, None],
                PieceType::S => [None,  Some(c),Some(c),
                                Some(c), Some(c), None,
                                None,None,None],
                PieceType::Z => [Some(c),Some(c), None,
                                None, Some(c), Some(c),
                                None, None, None],
                PieceType::L => [Some(c),None,None,
                                Some(c), None, None,
                                Some(c),Some(c),None],
                PieceType::J => [None, None,Some(c),
                                None, None, Some(c),
                                None,Some(c),Some(c)],
                PieceType::T => [Some(c),Some(c),Some(c),
                                None, Some(c), None,
                                None, None, None],
        };
        Piece {
            grid: grid, 
            p_type: piece_type, 
            x: 2.0, y: 0.0,
        }
    }
    fn rotate_right(&mut self) {
        if let PieceType::O = self.p_type {
            return;
        }
        let mut new_area: [Option<color::Color>; 9] = [None; 9]; //think array of 3 by 3 quandrant 4
        for x in 0..3 {
            new_area[x * 3 + 2] = self.grid[x]; // sets right 3 to top 3
        }
        for y in 0..3 {
            new_area[8 - y] = self.grid[y * 3 + 2]; // sets bottom 3 to right 3
        }
        for x in 0..3 {
            new_area[x * 3] = self.grid[6 + x]; // sets left 3 to bottom 3
        }
        for y in 0..3 {
            new_area[2 - y] = self.grid[y * 3]; // sets top 3 right 3
        }
        new_area[4] = self.grid[4]; // sets middle to middle
        if self.p_type != PieceType::I && new_area[0..3] == [None, None, None] {
            new_area.rotate_left(3);
        }
        self.grid = new_area;
    }
    fn draw(&self, scale: f32) {
        for grid_y in 0..3 {
            for grid_x in 0..3 {
                if let Some(c) = self.grid[3*grid_y + grid_x] {
                    draw_block(c, self.x + grid_x as f32, self.y + grid_y as f32, scale)
                }
            }
        }
    }
}
#[derive(Debug, PartialEq)]
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
            PieceType::O => color::Color::from_rgba(208, 245, 22, 255),
            PieceType::I => color::Color::from_rgba(9, 180, 214,255),
            PieceType::S => color::Color::from_rgba(232, 12, 15, 255),
            PieceType::Z => color::Color::from_rgba(5, 153, 24,255),
            PieceType::L => color::Color::from_rgba(245, 178, 22,255),
            PieceType::J => color::Color::from_rgba(240, 31, 205,255),
            PieceType::T => color::Color::from_rgba(113, 6, 158,255),
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
    fn delete_rows(&mut self) {
        let mut to_be_deleted = self.check_lines();
        while to_be_deleted.is_some() {
            /* Clears rows then shifts everything above cleared rows down*/
            if let Some((num_r, start_r)) = to_be_deleted {
                let end_r = start_r + num_r;
                for i in GAME_WIDTH*start_r..GAME_WIDTH*end_r {
                    self.grid[i] = None;
                }
                // moves everything down
                self.grid[0..GAME_WIDTH*end_r].rotate_right(GAME_WIDTH* (end_r - start_r));
            }
            to_be_deleted = self.check_lines();
        }
    }

    fn check_lines(&self) -> Option<(usize, usize)> {
        /* if there is a row to be deleted, it returns  the amount
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
    fn draw(&self, scale: f32) {
        for y in 0..GAME_HEIGHT {
            for x in 0..GAME_WIDTH {
                if let Some(c) = self.grid[GAME_WIDTH * y + x] {
                    draw_block(c, x as f32, y as f32, scale);
                }
            }
        }
    }
}

fn draw_block(block_color: color::Color, x: f32, y: f32, scale: f32) {
    shapes::draw_rectangle(x * scale + X_OFFSET, y * scale + Y_OFFSET, scale, scale, block_color);
    shapes::draw_rectangle_lines(x * scale + X_OFFSET, y * scale + Y_OFFSET, scale, scale, 2.0, color::BLACK);
}