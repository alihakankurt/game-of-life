mod game;
mod grid;

pub use game::Game;
pub use grid::Grid;

fn main() {
    let width = 10;
    let height = 10;
    let mut game = Game::new(width, height);

    game.grid.set(1, 0);
    game.grid.set(2, 1);
    game.grid.set(2, 2);
    game.grid.set(1, 2);
    game.grid.set(0, 2);
    game.grid.update();

    game.render();
    println!("-------");

    loop {
        game.update();
        game.render();
        println!("-------");
    }
}
