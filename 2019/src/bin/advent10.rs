use advent2019;
use std::collections::HashSet;

fn gcd((x, y): (i32, i32)) -> i32 {
    let ax = x.abs();
    let ay = y.abs();
    let (mut a, mut b) = if ax > ay { (ax, ay) } else { (ay, ax) };
    while b > 0 {
        let m = a % b;
        a = b;
        b = m;
    }
    a
}

fn one_down((x, y): (i32, i32)) -> Option<(i32, i32)> {
    let g = gcd((x, y));
    if g == 1 {
        None
    } else {
        Some((x - x / g, y - y / g))
    }
}

fn main() {
    let input = advent2019::load_input("10.txt");
    let mut asteroids = HashSet::<(i32, i32)>::new();

    let mut max_x = 0usize;
    let mut max_y = 0usize;
    for (y, line) in input.iter().enumerate() {
        max_y = y;
        for (x, ch) in line.chars().enumerate() {
            max_x = x;
            if ch == '#' {
                asteroids.insert((x as i32, y as i32));
            }
        }
    }

    let mut max_tot = 0;
    let mut at_x = 0;
    let mut at_y = 0;
    let mut all_seen = Vec::<(i32, i32)>::with_capacity(asteroids.len());
    let mut tmp_seen = Vec::<(i32, i32)>::with_capacity(asteroids.len());
    for y in 0..=max_y as i32 {
        for x in 0..=max_x as i32 {
            if !asteroids.contains(&(x, y)) { continue }
            let mut total_seen = 0;
            for sx in 0..=max_x as i32 {
                for sy in 0..=max_y as i32 {
                    if !asteroids.contains(&(sx, sy)) {
                        continue;
                    }
                    if sx == x && sy == y {
                        continue;
                    }
                    let (mut lx, mut ly) = (sx - x, sy - y);
                    loop {
                        match one_down((lx, ly)) {
                            Some((tx, ty)) if asteroids.contains(&(x + tx, y + ty)) => break,
                            Some((tx, ty)) => {
                                lx = tx;
                                ly = ty;
                            }
                            None => {
                                tmp_seen.push((x + lx, y + ly));
                                total_seen += 1;
                                break;
                            }
                        }
                    }
                }
            }
            if total_seen > max_tot {
                max_tot = total_seen;
                all_seen.clear();
                all_seen.append(&mut tmp_seen);
                at_x = x;
                at_y = y;
            }
            tmp_seen.clear();
        }
    }
    println!("{} at {},{}", max_tot, at_x, at_y);

    fn key(x: i32, y: i32) -> f64 {
        let res = ((y as f64).atan2(x as f64)) + std::f64::consts::PI/2f64;
        if res < 0f64 { res + std::f64::consts::PI * 2f64 } else { res }
    }

    all_seen.sort_by(|(ax, ay), (bx, by)| {
        let a = key(*ax - at_x, *ay - at_y);
        let b = key(*bx - at_x, *by - at_y);
        a.partial_cmp(&b).unwrap()
    });
    assert!(all_seen.len() >= 200, "not enough asteroids are visible");
    assert!(all_seen.len() == max_tot, "overflowing with asteroids");

    let mut i = 1;
    for (sx, sy) in all_seen.iter() {
        let res = key(*sx - at_x, *sy - at_y);
        let res_deg = res * 180f64 / std::f64::consts::PI;
        println!("{}. asteroid at coords {},{} is at angle {}deg", i, sx, sy, res_deg);
        i+=1;
    }

    let (elx, ely) = all_seen[199];
    println!("asteroid at {},{}, code is {}", elx, ely, elx*100 + ely);
}
