mod save;
use save::{load_save, save};
use macroquad::prelude::*;
use ::rand;
use ::rand::Rng;


#[derive(PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}


#[derive(PartialEq, Eq)]
enum Status {
    Continuing,
    Stopped,
    Failed,
}


struct Player {
    x: f32,
    y: f32,
    radius: f32,
    color: Color,
    existing: bool,
    history: Vec<(f32, f32)>,
}


struct Rope {
    x: f32,
    y: f32,
    radius: f32,
    color: Color,
}


impl Player {
    fn draw(&self) {
        if self.existing {
            draw_circle(self.x, self.y, self.radius, self.color);
        }
    }
}


impl Rope {
    fn draw(&self) {
        draw_circle(self.x, self.y, self.radius, self.color)
    }
}

#[macroquad::main("BasicShapes")]
async fn main() {

    let mut player = Player {
        x: 0.0,
        y: 0.0,
        radius: 4.0,
        color: WHITE,
        existing: true,
        history: Vec::new(),
    };
    let mut special = Player {
        x: 0.0,
        y: 0.0,
        radius: 4.0,
        color: RED,
        existing: true,
        history: Vec::new(),
    };
    let mut goal = Player {
        x: rand::rng().random_range(0.0..screen_width() - 40.0),
        y: rand::rng().random_range(0.0..screen_height() - 40.0),
        radius: 15.0,
        color: GREEN,
        existing: true,
        history: Vec::new(),
    };
    //let mut balast: Vec<Rope> = Vec::new();
    let mut ropes: Vec<Rope> = Vec::new();

    let mut dir = Direction::Right;

    //let mut history: Vec<(f32, f32)> = Vec::new();

    //let mut other_history: Vec<(f32, f32)> = Vec::new();
    //add_rope(&mut balast, &player);
    let mut timer = 0;

    let mut status = Status::Continuing;

    let mut saved = load_save("save.json");

    let mut score = 0.0;


    loop {
        clear_background(BLACK);
        if status == Status::Continuing {
            player.history.push((player.x, player.y));
            if player.history.len() >= 10 {
                if let Some(&(x, y)) = player.history.get(player.history.len() - 10) {
                    special.x = x;
                    special.y = y;
                    special.history.push((x, y));
                }
            }
            if is_key_down(KeyCode::LeftShift) {
                sprint(&mut player, &mut dir);
            } else {
                change_position(&mut player, &mut dir);
            }

            goal.draw();
            if on_collision(&player, &goal) {
                goal.existing = false;

            }
            if goal.existing == false {
                timer += 1;
            }

            if goal.existing == false && timer == 1 {
                add_rope(&mut ropes, &player);
                score += 10.0;
            }
            if timer == 300 {
                goal.x = rand::rng().random_range(0.0..screen_width() - 40.0);
                goal.y = rand::rng().random_range(0.0..screen_height() - 40.0);
                goal.existing = true;
                timer = 0;
            }

            for (i, rope) in ropes.iter_mut().enumerate() {
                let delay = 10 * (i + 1);
                if special.history.len() > delay {
                    rope.x = special.history[special.history.len() - delay].0;
                    rope.y = special.history[special.history.len() - delay].1;
                }
            }
            for rope in &ropes {
                rope.draw();
            }
            player.draw();
            if player.history.len() >= 10 {
                special.draw()
            }
            draw_text(&score.to_string(), 20.0, 60.0, 30.0, WHITE);
            draw_text(&saved.to_string(), 20.0, 20.0, 30.0, WHITE);

            if is_key_pressed(KeyCode::Escape) {
                status = Status::Stopped;
            }

            if touches(&player, &ropes) || player.x == screen_width() || player.y == screen_height() {
                status = Status::Failed;
            }
            if is_key_pressed(KeyCode::Q) {
                break
            }
        } else if status == Status::Stopped {
            draw_text("PAUSED, GOD DAMN IT!", 100.0, 100.0, 30.0, WHITE);
            if is_key_pressed(KeyCode::Escape) {
                status = Status::Continuing;
            }
            if is_key_pressed(KeyCode::Q) {
                break
            }
        } else {
            draw_text("YOU FAILED MY GAME, DAMN IT. PRESS F/Q", 100.0, 100.0, 30.0, WHITE);
            draw_text("Results:", 140.0, 140.0, 30.0, WHITE);
            draw_text( &score.to_string(), 250.0, 140.0, 30.0, WHITE);
            if score > saved.highscore {
                saved.highscore = score;
                save("save.json", &saved);
            }
            if is_key_pressed(KeyCode::F) {
                ropes.clear();
                player.history.clear();
                player.x = 0.0;
                player.y = 0.0;
                status = Status::Continuing;
                score = 0.0;
            }
            if is_key_pressed(KeyCode::Q) {
                break
            }
        }

        next_frame().await;
    }
}

fn on_collision(player: &Player, goal: &Player) -> bool {
    let dx = player.x - goal.x;
    let dy = player.y - goal.y;
    let distance = (dx*dx + dy*dy).sqrt();

    distance < (player.radius + goal.radius)
}
fn touches(player: &Player, ropes: & Vec<Rope>) -> bool {
    for rope in ropes {
        let dx = player.x - rope.x;
        let dy = player.y - rope.y;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance < (player.radius + rope.radius) {
            return true;
        }
    }

    false
}

fn sprint(player: &mut Player, dir: &mut Direction) {
    if is_key_pressed(KeyCode::W) && *dir != Direction::Up{
        *dir = Direction::Down;
    }
    if is_key_pressed(KeyCode::S) && *dir != Direction::Down{
        *dir = Direction::Up;
    }
    if is_key_pressed(KeyCode::A) && *dir != Direction::Right{
        *dir = Direction::Left;
    }
    if is_key_pressed(KeyCode::D) && *dir != Direction::Left{
        *dir = Direction::Right;
    }

    match dir {
        Direction::Up => {
            player.y += 2.0;
        }
        Direction::Down => {
            player.y -= 2.0;
        }
        Direction::Left => {
            player.x -= 2.0;
        }
        Direction::Right => {
            player.x += 2.0;
        }
    }

    player.x = player.x.clamp(player.radius, screen_width() - player.radius);
    player.y = player.y.clamp(player.radius, screen_height() - player.radius);
}

fn change_position(player: &mut Player, dir: &mut Direction) {
    if is_key_pressed(KeyCode::W) && *dir != Direction::Up{
        *dir = Direction::Down;
    }
    if is_key_pressed(KeyCode::S) && *dir != Direction::Down{
        *dir = Direction::Up;
    }
    if is_key_pressed(KeyCode::A) && *dir != Direction::Right{
        *dir = Direction::Left;
    }
    if is_key_pressed(KeyCode::D) && *dir != Direction::Left{
        *dir = Direction::Right;
    }

    match dir {
        Direction::Up => {
            player.y += 1.0;
        }
        Direction::Down => {
            player.y -= 1.0;
        }
        Direction::Left => {
            player.x -= 1.0;
        }
        Direction::Right => {
            player.x += 1.0;
        }
    }

    player.x = player.x.clamp(player.radius, screen_width() - player.radius);
    player.y = player.y.clamp(player.radius, screen_height() - player.radius);
}

fn add_rope(ropes: &mut Vec<Rope>, player: &Player) {
    for _ in 0..5 {
        ropes.push(Rope {
            x: player.x,
            y: player.y,
            radius: player.radius,
            color: WHITE,
        });
    }
}
