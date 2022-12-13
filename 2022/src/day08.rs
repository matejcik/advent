use std::io::BufRead;

use crate::{tiles::Tiles, Solver};

const FOREST_DIMENSION: usize = 100;
const MAP_SIZE: usize = FOREST_DIMENSION * (FOREST_DIMENSION + 1);

fn line_visibility(
    trees: &Tiles<u8>,
    visibility: &mut Tiles<u8>,
    stepper: impl Iterator<Item = usize>,
    stop_at: usize,
) -> usize {
    let mut tallest_visible_from_left = b'0' - 1;
    let mut maybe_visible_from_right = 0;
    for idx in stepper {
        if idx == stop_at {
            break;
        }
        let tree = trees.entries[idx];
        if tree > tallest_visible_from_left {
            visibility.entries[idx] = 1;
            tallest_visible_from_left = tree;
            maybe_visible_from_right = idx;
        }
    }
    maybe_visible_from_right
}

fn part1_count_visible_trees(input: &mut dyn BufRead) -> String {
    let trees = Tiles::load(input, MAP_SIZE);
    let mut visibility = Tiles::new(trees.width(), trees.height(), 0u8);

    for line in trees.rows_steppers().chain(trees.col_steppers()) {
        let maybe_visible_from_right =
            line_visibility(&trees, &mut visibility, line.iter(), line.len());
        line_visibility(
            &trees,
            &mut visibility,
            line.iter().rev(),
            maybe_visible_from_right,
        );
    }

    visibility
        .entries
        .iter()
        .map(|c| *c as u32)
        .sum::<u32>()
        .to_string()
}

fn part2_brute_force(input: &mut dyn BufRead) -> String {
    let trees = Tiles::load(input, MAP_SIZE);
    let mut views = Tiles::new(trees.width(), trees.height(), 0u32);

    for y in 1..trees.height() - 1 {
        for x in 1..trees.width() - 1 {
            let mut view_count = 1;
            let xx = (0..x)
                .rev()
                .skip_while(|xx| trees[(*xx, y)] < trees[(x, y)])
                .next()
                .unwrap_or(0);
            view_count *= x - xx;
            let xx = (x + 1..trees.width())
                .skip_while(|xx| trees[(*xx, y)] < trees[(x, y)])
                .next()
                .unwrap_or(trees.width() - 1);
            view_count *= xx - x;
            let yy = (0..y)
                .rev()
                .skip_while(|yy| trees[(x, *yy)] < trees[(x, y)])
                .next()
                .unwrap_or(0);
            view_count *= y - yy;
            let yy = (y + 1..trees.height())
                .skip_while(|yy| trees[(x, *yy)] < trees[(x, y)])
                .next()
                .unwrap_or(trees.height() - 1);
            view_count *= yy - y;
            views[(x, y)] = view_count as u32;
        }
    }

    views.entries.iter().max().unwrap().to_string()
}

pub const SOLVERS: &[Solver] = &[part1_count_visible_trees, part2_brute_force];
