use raylib::prelude::*;

struct State {
    dir: Vector2,
    pos: Vector2,
    segments: Vec::<Vector2>
}

impl State {
    pub fn new() -> Self {
        let mut s = Self {
            dir: Vector2 { x: 1.0, y: 0.0 },
            pos: Vector2 { x: 0.0, y: 0.0 },
            segments: Vec::new()
        };
        for _ in 0..3 {
            s.segments.push(Vector2 { x: 0.0, y: 0.0 })
        }
        return s;
    }
}

fn main() {
    let block_size = 30.0;
    let block_count = 25.0;
    let screen_width = block_size * block_count;
    let screen_height = block_size * block_count;
    let velocity = 3.0;
    let max_x = screen_width as f32 - block_size;
    let max_y = screen_height as f32 - block_size;
    let min_x = 0.0;
    let min_y = 0.0;

    let (mut rl, thread) = raylib::init()
        .size(screen_width as i32, screen_height as i32)
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

        let discretize = |n: f32| (n / block_size).floor() * block_size;
        let to_square = |v: Vector2| Vector2 { x: discretize(v.x), y: discretize(v.y) };

        let mut old_square = to_square(state.pos);

        state.pos.x += state.dir.x * block_size * velocity * t;
        state.pos.y += state.dir.y * block_size * velocity * t;

        if state.pos.x < min_x { return }
        if state.pos.y < min_y { return }
        if state.pos.x > max_x { return }
        if state.pos.y > max_y { return }

        let new_square = to_square(state.pos);

        if new_square != old_square {
            for segment in &mut state.segments {
                let temp = *segment;
                *segment = old_square;
                old_square = temp;
            }
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_rectangle(new_square.x as i32, new_square.y as i32, block_size as i32, block_size as i32, Color::GREEN);
        for segment in &state.segments {
            let x = (segment.x) as i32;
            let y = (segment.y) as i32;
            d.draw_rectangle(x, y, block_size as i32, block_size as i32, Color::GREEN);
        }
    }
}
