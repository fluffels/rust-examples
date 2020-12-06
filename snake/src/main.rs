use raylib::prelude::*;

struct Vec2<T> {
    x: T,
    y: T
}

struct State {
    pos: Vec2<i32>
}

impl State {
    pub fn new() -> Self {
        Self {
            pos: Vec2 { x: 0, y: 0 }
        }
    }
}

fn main() {
    let screen_width = 640;
    let screen_height = 480;
    let block_size = 10;
    let max_x = screen_width - block_size;
    let max_y = screen_height - block_size;
    let min_x = 0;
    let min_y = 0;

    let (mut rl, thread) = raylib::init()
        .size(screen_width, screen_height)
        .title("snek")
        .build();
    
    let mut state = State::new();
    
    while !rl.window_should_close() {
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) { state.pos.x += block_size }
        if rl.is_key_down(KeyboardKey::KEY_LEFT) { state.pos.x -= block_size }
        if rl.is_key_down(KeyboardKey::KEY_UP) { state.pos.y -= block_size }
        if rl.is_key_down(KeyboardKey::KEY_DOWN) { state.pos.y += block_size }

        if state.pos.x < min_x { state.pos.x = min_x }
        if state.pos.y < min_y { state.pos.y = min_y }
        if state.pos.x > max_x { state.pos.x = max_x }
        if state.pos.y > max_y { state.pos.y = max_y }

        let mut d = rl.begin_drawing(&thread);
        
        d.clear_background(Color::BLACK);
        d.draw_rectangle(state.pos.x, state.pos.y, block_size, block_size, Color::GREEN);
    }
}
