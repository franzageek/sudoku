mod flags;
mod game;
mod grid;
mod solver;
mod tile;
mod ui;

use raylib;

fn main() {
    let mut grid: grid::Grid = grid::Grid::new();
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        eprintln!(
            "sudoku solver, v0.1\nUsage: {} <file> [options]\n\n\tfile: path to a sudoku file in SDK format\n\toptions: can include the following flags:\n\t\t-f: suppress UI refresh (increase in solving speed)\n\t\t-s: suppress logging (increase in solving speed)",
            args[0]
        );
        return;
    }
    flags::init_flags(&args);
    grid.polulate(&args[1]);
    game::solve(
        raylib::init().size(ui::WINDOW_SIZE as i32, ui::WINDOW_SIZE as i32).title("sudoku v0.1").build(),
        &mut grid,
    );
    println!("Thanks for playing sudoku ;)");
    return;
}
