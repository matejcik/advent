use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    io::{stdout, BufRead, Write},
};

use crossterm::{
    cursor, queue,
    style::{Attribute, Color},
    ExecutableCommand,
};

use crate::{
    tiles::{Point, Tiles},
    Solver,
};

const MAX_MAP_SIZE: usize = 200 * 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct VisitInfo {
    path_len: u32,
    prev: Point,
}

#[allow(unused)]
fn visualize_astar(
    elevation: &Tiles<u8>,
    start: Point,
    end: Point,
    pos: Point,
    visited: &Tiles<Option<VisitInfo>>,
    queue: &BinaryHeap<(Reverse<i32>, Point, VisitInfo)>,
) {
    let mut vis = Tiles::new(
        elevation.width(),
        elevation.height(),
        (0 as char, Attribute::Reset, Color::Rgb { r: 0, g: 0, b: 0 }),
    );
    for y in 0..elevation.height() {
        for x in 0..elevation.width() {
            let ch = match Point::from((x, y)) {
                p if p == start => b'S',
                p if p == end => b'E',
                p => elevation[p],
            } as char;
            vis[(x, y)] = match visited[(x, y)] {
                Some(_) => (ch, Attribute::Reset, Color::Rgb { r: 255, g: 0, b: 0 }),
                _ => (ch, Attribute::Reset, Color::Rgb { r: 0, g: 0, b: 0 }),
            };
        }
    }
    for (_, qpos, _) in queue.iter() {
        vis[*qpos] = (
            vis[*qpos].0,
            Attribute::Reset,
            Color::Rgb { r: 0, g: 0, b: 255 },
        );
    }
    let mut path = pos;
    while let Some(prev) = visited[path] {
        vis[path] = match vis[path] {
            (ch, _, _) => (ch, Attribute::Bold, Color::Rgb { r: 0, g: 128, b: 0 }),
        };
        if prev.path_len == 0 {
            break;
        }
        path = prev.prev;
    }

    let mut stdout = stdout();
    queue!(stdout, cursor::MoveTo(0, 0)).unwrap();
    for y in 0..elevation.height() {
        for x in 0..elevation.width() {
            let (ch, attr, color) = vis[(x, y)];
            queue!(
                stdout,
                crossterm::style::SetAttribute(attr),
                crossterm::style::SetForegroundColor(color),
                crossterm::style::Print(ch),
            )
            .unwrap();
        }
        queue!(
            stdout,
            crossterm::style::Print('\n'),
            crossterm::style::SetAttribute(Attribute::Reset),
            crossterm::style::SetForegroundColor(Color::Reset)
        )
        .unwrap();
    }
    stdout.flush().unwrap();
}

fn astar(
    elevation: &Tiles<u8>,
    start: Point,
    end: Point,
    can_visit: fn(u8, u8) -> bool,
) -> Option<u32> {
    let mut visited = Tiles::new(elevation.width(), elevation.height(), None::<VisitInfo>);

    let score_func = |p: Point, dist: u32| Reverse(p.manhattan_distance(end) + dist as i16);

    let mut queue = BinaryHeap::with_capacity(MAX_MAP_SIZE);
    queue.push((
        Reverse(0),
        start,
        VisitInfo {
            path_len: 0,
            prev: start,
        },
    ));

    while let Some((_, pos, visit)) = queue.pop() {
        if let Some(v) = visited[pos] {
            if v.path_len <= visit.path_len {
                continue;
            }
        }
        visited[pos] = Some(visit);
        if pos == end {
            return Some(visit.path_len);
        }

        for neighbor in pos.neighbors().filter(|&p| elevation.contains(p)) {
            if can_visit(elevation[pos], elevation[neighbor])
                && visited[neighbor]
                    .map(|v| v.path_len > visit.path_len + 1)
                    .unwrap_or(true)
            {
                queue.push((
                    score_func(neighbor, visit.path_len + 1),
                    neighbor,
                    VisitInfo {
                        path_len: visit.path_len + 1,
                        prev: pos,
                    },
                ));
            }
        }

        //visualize_astar(elevation, start, end, pos, &visited, &queue);
    }
    None
}

fn part1_shortest_path(input: &mut dyn BufRead) -> String {
    let mut elevation = Tiles::load(input, MAX_MAP_SIZE);

    let mut start = Point::new(0, 0);
    let mut end = Point::new(0, 0);
    for y in 0..elevation.height() {
        for x in 0..elevation.width() {
            if elevation[(x, y)] == b'S' {
                start = Point::new(x as _, y as _);
                elevation[start] = b'a';
            } else if elevation[(x, y)] == b'E' {
                end = Point::new(x as _, y as _);
                elevation[end] = b'z';
            }
        }
    }

    let mut stdout = stdout();
    stdout.execute(cursor::SavePosition).unwrap();
    stdout.execute(cursor::Hide).unwrap();

    let result = astar(&elevation, end, start, |prev, next| {
        next as i8 - prev as i8 >= -1
    });

    stdout.execute(cursor::Show).unwrap();
    stdout.execute(cursor::RestorePosition).unwrap();
    result.unwrap().to_string()
}

pub const SOLVERS: &[Solver] = &[part1_shortest_path];
