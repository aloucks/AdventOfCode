use std::env;
use utility::format_err;
use utility::get_file_as_vec_string;
use utility::Result;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(format_err!(
            "need to pass input text path as first argument"
        ));
    }

    let contents = get_file_as_vec_string(&args[1])?;

    let board = contents
        .iter()
        .map(|l| String::from(l))
        .collect::<Vec<String>>();

    let count = count_trees_a(&board);

    println!("{}", count);

    let count1 = count_trees_b(&board, 1, 1);

    println!("{}", count1);

    let count2 = count_trees_b(&board, 1, 3);

    println!("{}", count2);

    let count3 = count_trees_b(&board, 1, 5);

    println!("{}", count3);

    let count4 = count_trees_b(&board, 1, 7);

    println!("{}", count4);

    let count5 = count_trees_b(&board, 2, 1);

    println!("{}", count5);

    let z: u64 = count1 as u64 * count2 as u64 * count3 as u64 * count4 as u64 * count5 as u64;

    println!("{}", z);

    Ok(())
}

struct StateA {
    width: usize,
    tree_count: u32,
}

// logic
// start at board index 0
// keep track of how much left we've gone
// walk down one at at time
// check if tree
// ...

fn count_trees_a(board: &[String]) -> u32 {
    let board_length = board.len();
    let line_length = board[0].len();

    let mut state = StateA {
        width: 0,
        tree_count: 0,
    };

    let right = 3_usize;

    for idx in 1..(board_length) {
        state.width += right;

        if state.width >= line_length {
            state.width -= line_length;
        }

        let check = board[idx].chars().nth(state.width).unwrap();

        if check == '#' {
            state.tree_count += 1;
        }
    }

    return state.tree_count;
}

fn count_trees_b(board: &[String], down: usize, right: usize) -> u32 {
    let board_length = board.len();
    let line_length = board[0].len();

    let mut height: usize = down;
    let mut width: usize = right;
    let mut tree_count: u32 = 0;

    while height < board_length {
        let x = board[height].chars().nth(width).unwrap();

        if x == '#' {
            tree_count += 1;
        }

        height += down;

        width += right;
        if width >= line_length {
            width -= line_length;
        }
    }

    return tree_count;
}
