fn main() -> utility::Result<()> {
    let input_path = utility::get_input_path()?;
    let input_data = utility::get_file_as_vec_string(&input_path)?;

    parse_1(&input_data);

    Ok(())
}

fn parse_1(input: &[String]) {
    let trees: Vec<_> = input
        .iter()
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let width = trees[0].len();
    let height = trees.len();

    let mut visible = 0;

    let mut max_score = 0;
    let mut max_w = 0;
    let mut max_h = 0;

    for h in 0..height {
        for w in 0..width {
            let score = calculate_scenic_score(&trees, h, w);
            if score >= max_score {
                max_score = score;
                max_w = w;
                max_h = h;
            }

            if h == 0 || h == height - 1 || w == 0 || w == width - 1 {
                visible += 1;
                continue;
            }

            if is_visible(&trees, h, w) {
                visible += 1;
            }
        }
    }

    println!("{}", visible);

    println!("{},{} => {}", max_h, max_w, max_score);
}

fn is_visible(trees: &Vec<Vec<u32>>, h: usize, w: usize) -> bool {
    let val = trees[h][w];

    let left = (0..w).any(|c| trees[h][c] >= val);

    let right = (w + 1..trees[h].len()).any(|c| trees[h][c] >= val);

    let up = (0..h).any(|c| trees[c][w] >= val);

    let down = (h + 1..trees.len()).any(|c| trees[c][w] >= val);

    !(left && right && up && down)
}

fn calculate_scenic_score(trees: &Vec<Vec<u32>>, h: usize, w: usize) -> u32 {
    if h == 0 || h == trees.len() - 1 || w == 0 || w == trees[h].len() - 1 {
        return 0;
    }

    let val = trees[h][w];

    let mut up = 0;
    let mut down = 0;
    let mut left = 0;
    let mut right = 0;

    // up
    for u in (0..h).rev() {
        let check = trees[u][w];

        if check >= val {
            up += 1;
            break;
        }

        up += 1;
    }

    for d in trees.iter().skip(h + 1) {
        let check = d[w];

        if check >= val {
            down += 1;
            break;
        }

        down += 1;
    }

    for l in (0..w).rev() {
        let check = trees[h][l];

        if check >= val {
            left += 1;
            break;
        }

        left += 1;
    }

    for r in w + 1..trees[h].len() {
        let check = trees[h][r];

        if check >= val {
            right += 1;
            break;
        }

        right += 1;
    }

    up * down * left * right
}
