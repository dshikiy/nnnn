pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {
    (table[0][0] == player && table[1][1] == player && table[2][2] == player)
        || (table[0][2] == player && table[1][1] == player && table[2][0] == player)
}

pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {
    for row in 0..3 {
        if table[row][0] == player && table[row][1] == player && table[row][2] == player {
            return true;
        }
    }
    false
}

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
    for col in 0..3 {
        if table[0][col] == player && table[1][col] == player && table[2][col] == player {
            return true;
        }
    }
    false
}

pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {
    let check_win = |player: char| {
        diagonals(player, table) || horizontal(player, table) || vertical(player, table)
    };

    if check_win('X') {
        "player X won".to_string()
    } else if check_win('O') {
        "player O won".to_string()
    } else {
        "tie".to_string()
    }
}
