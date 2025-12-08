// EX 4
// calculate the area and circumference of a graphical object, which can
// either be a rectangle, a right triangle with sides of equal length, or a circle.
enum Shapes {
    Rectangle,
    RightTriangle,
    Circle
}

struct Geometry {
    shape: Shapes,
    width: f32,
    height: f32,
    side: f32,
    radius: f32
}

impl Geometry {
    fn area(&self) -> f32 {
        match self.shape {
            Shapes::Rectangle => self.width * self.height,
            Shapes::RightTriangle => (self.side * self.side) / 2.0,
            Shapes::Circle => 3.14 * self.radius * self.radius
        }
    }
    
    fn circumference(&self) -> f32 {
        match self.shape {
            Shapes::Rectangle => 2.0 * (self.width + self.height),
            Shapes::RightTriangle => {
                let hipoteza = (2.0_f32).sqrt() * self.side;
                self.side + self.side + hipoteza
            }
            Shapes::Circle => 2.0 * 3.14 * self.radius
        }
    }
    // visualize such a graphical object using print statements.
    fn visualize(&self) {
        match self.shape {
            Shapes::Rectangle => {
                for _ in 0..(self.height as usize) {
                    for _ in 0..(self.width as usize) {
                        print!("█");
                    }
                    println!();
                }
            }
            Shapes::RightTriangle => {
                for i in 1..=(self.side as usize) {
                    for _ in 0..i {
                        print!("█");
                    }
                    println!();
                }
            }
            Shapes::Circle => {
                let r = self.radius as i32;
                for x in -r..=r {
                    for y in -r..=r {
                        if x * x + y * y <= r * r {
                            print!("█");
                        } else {
                            print!(" ");
                        }
                    }
                    println!();
                }
            }
        }
    }
}

// stepwise build and manipulate a Sudoku board.
struct SudokuBoard {
    cells: Vec<Vec<u8>>
}

impl SudokuBoard {
    fn new_board() -> SudokuBoard {
        let mut rows = Vec::new();
        for _ in 0..9 {
            let mut row = Vec::new();
            for _ in 0..9 {
                row.push(0);
            }
            rows.push(row);
        }
        SudokuBoard { cells: rows }
    }
    
    fn set_cell(&mut self, row: usize, col: usize, value: u8) {
        if row < 9 && col < 9 && value <= 9 {
            self.cells[row][col] = value;
            println!("Row {}, col {} set to {}", row, col, value);
        } else {
            println!("Invalid row, column or value!");
        }
    }
    
    fn clear_cell(&mut self, row: usize, col: usize) {
        if row < 9 && col < 9 {
            self.cells[row][col] = 0;
            println!("Row {}, col {} reset to 0", row, col);
        } else {
            println!("Invalid row or column!");
        }
    }
    // visualize such a Sudoku board using print statements.
    fn print_board(&self) {
        println!("+-------+-------+-------+");
        let mut row = 0;
        while row < 9 {
            let mut col = 0;
            print!("|");
            while col < 9 {
                let value = self.cells[row][col];
                
                if value == 0 {
                    print!(" .");
                } else {
                    print!(" {}", value);
                }
                
                if col % 3 == 2 {
                    print!(" |");
                }
                col += 1;
            }
            println!();
            if row % 3 == 2 {
                println!("+-------+-------+-------+");
            }
            row += 1;
        }
    }
}

fn main() {
    // area and circumference
    let rect = Geometry {
        shape: Shapes::Rectangle,
        width: 10.0,
        height: 5.0,
        side: 0.0,
        radius: 0.0
    };
    let tri = Geometry {
        shape: Shapes::RightTriangle,
        width: 0.0,
        height: 0.0,
        side: 6.0,
        radius: 0.0
    };
    let circ = Geometry {
        shape: Shapes::Circle,
        width: 0.0,
        height: 0.0,
        side: 0.0,
        radius: 4.0
    };
    println!("Rect area: {}", rect.area());
    println!("Rect circumference: {}", rect.circumference());
    println!("Triangle area: {}", tri.area());
    println!("Triangle circumference: {}", tri.circumference());
    println!("Circle area: {}", circ.area());
    println!("Circle circumference: {}", circ.circumference());
    // visualization
    rect.visualize();
    tri.visualize();
    circ.visualize();
    // sudoku board
    let mut board = SudokuBoard::new_board();
    // print empty board
    board.print_board();
    // fill some values
    board.set_cell(0, 0, 5);
    board.set_cell(0, 1, 3);
    board.set_cell(0, 4, 7);
    board.set_cell(1, 0, 6);
    board.set_cell(1, 3, 1);
    board.set_cell(1, 4, 9);
    board.set_cell(1, 5, 5);
    // visualize current board
    board.print_board();
    // modify something
    board.set_cell(0, 1, 9);
    board.print_board();
    // clear a cell
    board.clear_cell(0, 4);
    board.print_board();
}