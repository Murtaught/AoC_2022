use std::io::*;

fn main() {
    let fld: Vec<Vec<u8>> = stdin().lines().map(|rs| rs.unwrap().into_bytes()).collect();
    let n = fld.len();
    let m = fld[0].len();
    assert!(fld.iter().all(|line| line.len() == m));

    eprintln!("{n} x {m}");

    let mut visible = vec![vec![false; m]; n];

    let mut visit = |i: usize, j: usize, tallest: &mut Option<u8>| {
        if tallest.is_none() || fld[i][j] > tallest.unwrap() {
            visible[i][j] = true;
            *tallest = Some(fld[i][j]);
        }
    };

    for i in 0..n {
        let mut tallest = None;
        for j in 0..m {
            visit(i, j, &mut tallest);
        }

        tallest = None;
        for j in (0..m).into_iter().rev() {
            visit(i, j, &mut tallest);
        }
    }

    for j in 0..m {
        let mut tallest = None;
        for i in 0..n {
            visit(i, j, &mut tallest);
        }

        tallest = None;
        for i in (0..n).into_iter().rev() {
            visit(i, j, &mut tallest);
        }
    }

    // for i in 0..n {
    //     for j in 0..m {
    //         print!("{}", visible[i][j] as u8);
    //     }
    //     println!();
    // }

    let ans = visible
        .into_iter()
        .map(|row| row.into_iter().map(|v| v as u64).sum::<u64>())
        .sum::<u64>();
    
    println!("{ans}");
    println!("{}", max_scenic_score(&fld));
}

fn max_scenic_score(fld: &[Vec<u8>]) -> u64 {
    let n = fld.len();
    let m = fld[0].len();

    let mut ans = 0;
    for y in 0..n {
        for x in 0..m {
            let score = scenic_score(fld, y, x);
            ans = ans.max(score);
        }
    }

    ans
}

fn scenic_score(fld: &[Vec<u8>], y: usize, x: usize) -> u64 {
    let n = fld.len();
    let m = fld[0].len();
    let height = fld[y][x];

    let mut top = 0_u64;
    for i in (0..y).into_iter().rev() {
        top += 1;

        if fld[i][x] >= height {
            break;
        }
    }

    let mut bottom = 0_u64;
    for i in (y + 1)..n {
        bottom += 1;

        if fld[i][x] >= height {
            break;
        }
    }

    let mut left = 0_u64;
    for j in (0..x).into_iter().rev() {
        left += 1;

        if fld[y][j] >= height {
            break;
        }
    }

    let mut right = 0_u64;
    for j in (x + 1)..m {
        right += 1;

        if fld[y][j] >= height {
            break;
        }
    }

    right * top * left * bottom
}

fn scan<T: std::str::FromStr>() -> T {
    static mut BUFFER: Vec<String> = vec![];
    loop {
        if let Some(token) = unsafe { BUFFER.pop() } {
            return token.parse().ok().unwrap();
        }
        let mut line = String::new();
        stdin().read_line(&mut line).ok();
        unsafe {
            BUFFER = line.split_whitespace().rev().map(String::from).collect();
        }
    }
}

fn gets() -> Option<String> {
    let mut line = String::new();
    let count = stdin().read_line(&mut line).unwrap();
    if count > 0 {
        while let Some(c) = line.chars().last() {
            if !c.is_whitespace() {
                break;
            }

            line.pop();
        }
        Some(line)
    } else {
        None
    }
}
