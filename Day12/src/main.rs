use std::env;
use std::fs;

#[derive(Clone, Copy)]
struct GridSquare {
    char: char,
    visited: bool,
}

struct Grid {
    len_x: usize,
    squares: Vec<Vec<GridSquare>>,
    start_point: (usize, usize),
    end_point: (usize, usize),
}

impl Grid {
    fn solve(&mut self, path: &mut Vec<char>, x: usize, y: usize) {
        //println!("Solving... p: {}, x: {}, y: {}", path.iter().collect::<String>(), x, y);
        if (x, y) == self.end_point {
            println!(
                "Solved! Path: {}, len = {}",
                path.iter().collect::<String>(),
                path.len()
            );
            return;
        }
        let len = self.squares.len();
        let len_x = self.len_x;
        // x - 1, x + 1, y - 1, y + 1
        let y_min = if y == 0 { y } else { y - 1 };
        let y_max = if y == len - 1 { y } else { y + 1 };
        let x_min = if x == 0 { x } else { x - 1 };
        let x_max = if x == len_x - 1 { x } else { x + 1 };
        
        println!("Y: min({}), max({}), X: min({}), max({})", y_min, y_max, x_min, x_max);
        let curr = self.get_square_mut(x, y);
        curr.visited = true;
        
        for y_inner in y_min..=y_max {
            for x_inner in x_min..=x_max {
                if (x, y) == (x_inner, y_inner) {
                    continue;
                }
                let next = self.get_square_mut(x_inner, y_inner);
                
                if !self.possible(curr, next) {
                    continue;
                }

                //println!("LOOP");
                next.visited = true;
                path.push(next.char);
                self.solve(path, x_inner, y_inner);
                // Backtrack
                next.visited = false;
                path.pop();
            }
        }
    }

    fn get_square(&self, x: usize, y: usize) -> &GridSquare {
        self.squares.get(y).unwrap().get(x).unwrap()
    }

    fn get_square_mut(&mut self, x: usize, y: usize) -> &mut GridSquare {
        &mut self.squares[y][x]
    }

    fn possible(&self, curr: &GridSquare, next: &GridSquare) -> bool {
        !next.visited && (next.char as i16 - curr.char as i16) <= 1
    }

    fn possible_mut(&mut self, curr: &GridSquare, next: &GridSquare) -> bool {
        println!("Possible? Curr: c({}) v({}), Next: c({}) v({})", curr.char, curr.visited, next.char, next.visited);
        !next.visited && (next.char as i16 - curr.char as i16) <= 1
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut len = 0;
    let mut grid_squares: Vec<Vec<GridSquare>> = Vec::new();
    let mut starting_point = (0, 0);
    let mut end_point = (0, 0);
    for y in 0..contents.lines().count() {
        let line = contents.lines().nth(y).unwrap();
        len = line.len();
        if let Some(index) = line.find('S') {
            starting_point = (index, y);
        }

        if let Some(index) = line.find('E') {
            end_point = (index, y);
        }
        let chars: Vec<GridSquare> = line
            .chars()
            .map(|c| GridSquare {
                char: if c != 'S' && c != 'E' {c} else {
                    if c == 'S' {'a'} else {'z'}
                },
                visited: false,
            })
            .collect();
        grid_squares.push(chars);
    }

    let mut grid = Grid {
        len_x: len,
        squares: grid_squares,
        start_point: starting_point,
        end_point,
    };

    let mut path: Vec<char> = Vec::new();
    grid.solve(&mut path, starting_point.0, starting_point.1);

    // Solve with backtracking
    // Start at starting point
    // Iterate over possible options
    // If stuck => Backtrack
    // If found => Add to solved solutions and backtrack
}
