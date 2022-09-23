use proconio::{
    input,
    marker::{Chars, Usize1},
};
use std::collections::VecDeque;

fn main() {
    input! {
        h: usize, w: usize,
        sy: Usize1, sx: Usize1,
        gy: Usize1, gx: Usize1,
        c: [Chars; h],
    }

    let mut que = VecDeque::new();
    que.push_back((sy, sx));
    let mut dist: Vec<Vec<Option<usize>>> = vec![vec![None; w]; h];
    dist[sy][sx] = Some(0);

    while let Some((y, x)) = que.pop_front() {
        if (y, x) == (gy, gx) {
            println!("{}", dist[y][x].unwrap());
            return;
        }

        let dy = vec![1, 0, -1, 0];
        let dx = vec![0, 1, 0, -1];

        for i in 0..4 {
            let dy = dy[i];
            let dx = dx[i];

            let next_y = y as isize + dy;
            let next_x = x as isize + dx;

            if 0 <= next_y
                && next_y < h as isize
                && 0 <= next_x
                && next_x < w as isize
                && dist[next_y as usize][next_x as usize].is_none()
                && c[next_y as usize][next_x as usize] == '.'
            {
                que.push_back((next_y as usize, next_x as usize));
                dist[next_y as usize][next_x as usize] = Some(dist[y][x].unwrap() + 1);
            }
        }
    }
}