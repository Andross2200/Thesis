use simple_matrix::Matrix;

pub struct Game {
    pub level_matrix: Matrix<char>,
    init_fen: String,
    pub rows: u32,
    pub columns: u32,
    pub green_coord: (u32, u32),
    pub orange_coord: (u32, u32)
}

pub fn init() -> Game {
    let mut game = Game {
        level_matrix: Matrix::new(5, 5),
        init_fen: String::new(),
        rows: 5,
        columns: 5,
        green_coord: (0,0),
        orange_coord: (0,0)
    };
    for i in 0..5 {
        for j in 0..5 {
         game.level_matrix.set(i, j, 'f');
        }
    }
 game
}

pub fn init_from_fen(fen: String) -> Game {
    let mut iter = fen.split_whitespace();
    let num_of_rows: u32 = iter.next().unwrap().parse().unwrap();
    let num_of_columns: u32 = iter.next().unwrap().parse().unwrap();
    let binding = String::from(iter.next().unwrap());
    let mut level_iter = binding.split('/').peekable();
    let mut matrix: Matrix<char> = Matrix::new(num_of_rows.try_into().unwrap(), num_of_columns.try_into().unwrap());
    let mut green: (u32, u32) = (0,0);
    let mut orange: (u32, u32) = (0,0);
    for i in 0..num_of_rows {
        let mut line_chars = level_iter.next().unwrap().chars().peekable();
        let mut col_counter: u32 = 0;
        while line_chars.peek() != None {
            let c: char = line_chars.next().unwrap();
            if c.is_alphabetic() {
                matrix.set(i.try_into().unwrap(), col_counter.try_into().unwrap(), c);
                if is_pawn(c) {
                    if c == 'p' {
                        green = (i,col_counter);
                    } else if c == 'P' {
                        orange = (i,col_counter);
                    }
                }
                col_counter+=1;
            } else if c.is_numeric() {
                let num = c.to_digit(10).unwrap();
                for n in 1..num+1 {
                    matrix.set(i.try_into().unwrap(), col_counter.try_into().unwrap(), '_');
                    col_counter+=1;
                }
            }
        }
    }
    let mut game = Game {
        level_matrix: matrix,
        init_fen: fen,
        rows: num_of_rows,
        columns: num_of_columns,
        green_coord: green,
        orange_coord: orange,
    };
    game
}

pub fn print(game: Game) {
    for val in game.level_matrix {
        print!("{} ", val);
    }
    print!("\n{} ", game.init_fen);
}

fn is_pawn(c: char) -> bool {
    if c == 'p' || c == 'P' {
        return true;
    }
    false
}