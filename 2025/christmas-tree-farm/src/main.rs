use std::{fs, collections::HashSet};

fn norm(s: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let s: Vec<_> = s.into_iter().filter(|r| r.iter().any(|&b| b)).collect();
    if s.is_empty() { return s; }
    let (minc, maxc) = s.iter().flat_map(|r| r.iter().enumerate().filter(|&(_, &b)| b).map(|(c, _)| c))
        .fold((usize::MAX, 0), |(mn, mx), c| (mn.min(c), mx.max(c)));
    s.iter().map(|r| r[minc..=maxc].to_vec()).collect()
}

fn rot(s: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let (h, w) = (s.len(), s[0].len());
    norm((0..w).map(|c| (0..h).map(|r| s[h - 1 - r][c]).collect()).collect())
}

fn flip(s: &[Vec<bool>]) -> Vec<Vec<bool>> {
    norm(s.iter().map(|r| r.iter().rev().copied().collect()).collect())
}

fn variants(s: &[Vec<bool>]) -> Vec<Vec<(usize, usize)>> {
    let mut seen = HashSet::new();
    let mut out = vec![];
    let mut cur = norm(s.to_vec());
    for _ in 0..4 {
        for v in [cur.clone(), flip(&cur)] {
            if seen.insert(format!("{:?}", v)) {
                out.push(v.iter().enumerate().flat_map(|(r, row)| 
                    row.iter().enumerate().filter(|&(_, &b)| b).map(move |(c, _)| (r, c))).collect());
            }
        }
        cur = rot(&cur);
    }
    out
}

fn solve(pv: &[Vec<Vec<(usize, usize)>>], i: usize, g: &mut [bool], w: usize, h: usize) -> bool {
    if i == pv.len() { return true; }
    for coords in &pv[i] {
        let (sh, sw) = (coords.iter().map(|c| c.0).max().unwrap() + 1, coords.iter().map(|c| c.1).max().unwrap() + 1);
        if sh > h || sw > w { continue; }
        for oy in 0..=h - sh {
            for ox in 0..=w - sw {
                let cells: Vec<_> = coords.iter().map(|(r, c)| (oy + r) * w + ox + c).collect();
                if cells.iter().all(|&c| !g[c]) {
                    cells.iter().for_each(|&c| g[c] = true);
                    if solve(pv, i + 1, g, w, h) { return true; }
                    cells.iter().for_each(|&c| g[c] = false);
                }
            }
        }
    }
    false
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let parts: Vec<_> = input.split("\n\n").collect();
    
    let shapes: Vec<Vec<Vec<(usize, usize)>>> = parts.iter()
        .filter(|p| p.contains(':') && !p.contains('x'))
        .map(|p| {
            let grid: Vec<Vec<bool>> = p.lines().skip(1)
                .map(|l| l.chars().map(|c| c == '#').collect()).collect();
            variants(&norm(grid))
        }).collect();

    let count = parts.iter().flat_map(|p| p.lines())
        .filter(|l| l.contains('x') && l.contains(':'))
        .filter(|l| {
            let (dims, cnts) = l.split_once(':').unwrap();
            let (w, h): (usize, usize) = dims.split_once('x').map(|(a, b)| (a.trim().parse().unwrap(), b.trim().parse().unwrap())).unwrap();
            let counts: Vec<usize> = cnts.split_whitespace().map(|s| s.parse().unwrap()).collect();
            let mut pieces: Vec<_> = counts.iter().enumerate().flat_map(|(i, &c)| vec![i; c]).collect();
            let total: usize = pieces.iter().map(|&i| shapes[i][0].len()).sum();
            if total > w * h { return false; }
            pieces.sort_by_key(|&i| std::cmp::Reverse(shapes[i][0].len()));
            let pv: Vec<_> = pieces.iter().map(|&i| shapes[i].clone()).collect();
            solve(&pv, 0, &mut vec![false; w * h], w, h)
        }).count();

    println!("{}", count);
}
