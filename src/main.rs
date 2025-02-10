use bracket_lib::prelude::*;

struct State {
    player: Player,
    frame_time: f32,
    mode: GameModes,
}

impl State {
    fn new() -> Self {
        Self {
            player: Player::new(5, 25),
            frame_time: 0.0,
            mode: GameModes::Menu,
        }
    }

    fn restart(&mut self) {
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.mode = GameModes::Playing;
    }

    fn play(&mut self, ctx: &mut BTerm) {
        self.mode = GameModes::Playing;
        ctx.print(1, 1, "Playing");
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        self.mode = GameModes::Menu;
        ctx.cls();
        ctx.print_centered(5, "Welcome to flappy ascii");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.play(ctx),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        self.mode = GameModes::End;
        ctx.print(1, 1, "Game Over");
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        match self.mode {
            GameModes::Playing => self.play(ctx),
            GameModes::Menu => self.main_menu(ctx),
            GameModes::End => self.dead(ctx),
        }
    }
}

enum GameModes {
    Playing,
    Menu,
    End,
}

struct Player {
    x: i32,
    y: i32,
    velocity: f32,
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Player {
            x,
            y,
            velocity: 0.0,
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(self.x, self.y, RED, BLACK, to_cp437('@'));
    }

    fn gravity_and_move(&mut self) {
        if self.velocity < 2.0 {
            self.velocity += 0.2;
        }
        self.y += self.velocity as i32;
        self.x += 1;
        if self.y < 0 {
            self.y = 0;
        }
    }

    fn flap(&mut self) {
        self.velocity = -2.0;
    }
}

fn main() -> BError {
    println!("Hello, world! ");
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy ascii 😛")
        .build()?;

    main_loop(context, State::new())
}
