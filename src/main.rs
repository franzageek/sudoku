mod grid;
mod tile;

fn main() {
    let mut grid: grid::Grid = grid::Grid::new();
    grid.polulate("test.sdk");
}
