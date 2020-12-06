use raylib::prelude::*;

struct Vec2<T> {
    x: T,
    y: T
}

struct State {
    dir: Vector2,
    pos: Vector2
}

impl State {
    pub fn new() -> Self {
        Self {
            dir: Vector2 { x: 1.0, y: 0.0 },
            pos: Vector2 { x: 0.0, y: 0.0 }
        }
    }
}

fn main() {
    let screen_width = 640;
    let screen_height = 480;
    let block_size = 30.0;
    let velocity = 3.0;
    let max_x = screen_width as f32 - block_size;
    let max_y = screen_height as f32 - block_size;
    let min_x = 0.0;
    let min_y = 0.0;

    let (mut rl, thread) = raylib::init()
        .size(screen_width, screen_height)
        .title("snek")
        .build();
    
    let mut state = State::new();
    
    while !rl.window_should_close() {
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            state.dir.x = 1.0;
            state.dir.y = 0.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            state.dir.x = -1.0;
            state.dir.y = 0.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_UP) {
            state.dir.x = 0.0;
            state.dir.y = -1.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_DOWN) {
            state.dir.x = 0.0;
            state.dir.y = 1.0;
        }

        let t = rl.get_frame_time();
        dbg!(t);

        state.pos.x += state.dir.x * block_size * velocity * t;
        state.pos.y += state.dir.y * block_size * velocity * t;

        if state.pos.x < min_x { state.pos.x = min_x }
        if state.pos.y < min_y { state.pos.y = min_y }
        if state.pos.x > max_x { state.pos.x = max_x }
        if state.pos.y > max_y { state.pos.y = max_y }

        let screen_x = ((state.pos.x / block_size).floor() * block_size) as i32;
        let screen_y = ((state.pos.y / block_size).floor() * block_size) as i32;

        let mut d = rl.begin_drawing(&thread);
        
        d.clear_background(Color::BLACK);
        d.draw_rectangle(screen_x, screen_y, block_size as i32, block_size as i32, Color::GREEN);
    }
}
