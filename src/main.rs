use bracket_lib::prelude::*;

#[derive(Debug)]
struct State {
    mode: GameModes,
}

impl State {
    fn new() -> Self {
        Self {
            mode: GameModes::Menu,
        }
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

#[derive(Debug)]
enum GameModes {
    Playing,
    Menu,
    End,
}

fn main() -> BError {
    println!("Hello, world! ");
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy ascii 😛")
        .build()?;

    main_loop(context, State::new())
}
