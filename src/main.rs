mod game;
mod grid;
mod solver;
mod tile;
mod ui;

use raylib;

fn main() {
    let mut grid: grid::Grid = grid::Grid::new();
    grid.polulate("new.sdk");
    game::solve(
        raylib::init()
            .size(ui::WINDOW_SIZE as i32, ui::WINDOW_SIZE as i32)
            .title("sudoku v0.1")
            .build(),
        &mut grid,
    );
    println!("Thanks for playing sudoku ;)");
    return;
}
