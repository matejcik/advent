use std::{
    io::BufRead,
    ops::{Index, IndexMut},
};

use crate::Solver;

const FOREST_DIMENSION: usize = 100;

pub struct Trees<T> {
    entry_len: usize,
    line_width: usize,
    pub entries: Vec<T>,
}

pub struct Stepper {
    start: usize,
    end: usize,
    step: usize,
}

impl Trees<u8> {
    pub fn load(input: &mut dyn BufRead) -> Self {
        let expected_data_size = (FOREST_DIMENSION + 1) * FOREST_DIMENSION;
        let mut entries = Vec::with_capacity(expected_data_size);
        input.read_to_end(&mut entries).unwrap();
        let entry_len = entries.iter().position(|&c| c == b'\n').unwrap();
        Self {
            entry_len,
            line_width: entry_len + 1,
            entries,
        }
    }
}

impl<T: Copy> Trees<T> {
    pub fn new(width: usize, height: usize, initial: T) -> Self {
        Self {
            entry_len: width,
            line_width: width + 1,
            entries: vec![initial; (width + 1) * height],
        }
    }
}

impl<T> Trees<T> {
    pub const fn width(&self) -> usize {
        self.entry_len
    }

    pub fn height(&self) -> usize {
        self.entries.len() / self.line_width
    }

    pub fn rows_steppers(&self) -> impl Iterator<Item = Stepper> {
        let line_width = self.line_width;
        let entry_len = self.entry_len;
        (0..self.height()).map(move |y| Stepper::new_horiz(y, line_width, entry_len))
    }

    pub fn col_steppers(&self) -> impl Iterator<Item = Stepper> {
        let line_width = self.line_width;
        let total_len = self.entries.len();
        (0..self.width()).map(move |x| Stepper::new_vert(x, line_width, total_len))
    }
}

impl<T> Index<(usize, usize)> for Trees<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &T {
        &self.entries[y * self.line_width + x]
    }
}

impl<T> IndexMut<(usize, usize)> for Trees<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut T {
        &mut self.entries[y * self.line_width + x]
    }
}

impl Stepper {
    fn new_horiz(y: usize, line_width: usize, row_len: usize) -> Self {
        Self {
            start: y * line_width,
            end: y * line_width + row_len,
            step: 1,
        }
    }

    pub fn new_vert(x: usize, line_width: usize, total_len: usize) -> Self {
        Self {
            start: x,
            end: total_len + x,
            step: line_width,
        }
    }

    pub const fn len(&self) -> usize {
        ((self.end - self.start) / self.step) as usize
    }

    pub fn iter(&self) -> impl DoubleEndedIterator<Item = usize> {
        (self.start..self.end).step_by(self.step)
    }
}

fn line_visibility(
    trees: &Trees<u8>,
    visibility: &mut Trees<u8>,
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
    let trees = Trees::load(input);
    let mut visibility = Trees::new(trees.width(), trees.height(), 0u8);

    for row in trees.rows_steppers() {
        let maybe_visible_from_right = line_visibility(&trees, &mut visibility, row.iter(), row.len());
        line_visibility(&trees, &mut visibility, row.iter().rev(), maybe_visible_from_right);
    }

    for col in trees.col_steppers() {
        let maybe_visible_from_right = line_visibility(&trees, &mut visibility, col.iter(), col.len());
        line_visibility(&trees, &mut visibility, col.iter().rev(), maybe_visible_from_right);
    }

    visibility
        .entries
        .iter()
        .map(|c| *c as u32)
        .sum::<u32>()
        .to_string()
}

fn part2_find_best_view(input: &mut dyn BufRead) -> String {
    /*let trees = Trees::load(input);
    let mut views = Trees::new(trees.width(), trees.height(), 0u32);

    let mut line = vec![0u32; trees.width()];
    for y in 1..trees.height() - 1 {
        let line_iter = LineIter::new(Line::Row(y), 0, trees.width());
        let tallest = line_iter.map(|coords| trees[coords]).max().unwrap();
        let mut prevx = 0;
        for x in 0..trees.width() {
            let height = trees[(x, y)];
            if height == tallest {
                line[x] = (x - prevx) as u32;
                prevx = x - 1;
            }
        }
        prevx = trees.width() - 1;
        for x in (0..trees.width()).rev() {
            let height = trees[(x, y)];
            if height == tallest {
                views[(x, y)] = line[x] * (prevx - x) as u32;
                prevx = x + 1;
            }
        }
    }

    let mut column = vec![0u32; trees.height()];
    for x in 1..trees.width() - 1 {
        let line_iter = LineIter::new(Line::Col(x), 0, trees.height());
        let tallest = line_iter.map(|coords| trees[coords]).max().unwrap();
        let mut prevy = 0;
        for y in 0..trees.height() {
            let height = trees[(x, y)];
            if height == tallest {
                column[y] = (y - prevy) as u32;
                prevy = y - 1;
            }
        }
        prevy = trees.height() - 1;
        for y in (0..trees.height()).rev() {
            let height = trees[(x, y)];
            if height == tallest {
                views[(x, y)] *= column[y] * (prevy - y) as u32;
                prevy = y + 1;
            }
        }
    }

    views.entries.iter().max().unwrap().to_string()*/
    "".to_string()
}

pub const SOLVERS: &[Solver] = &[part1_count_visible_trees, part2_find_best_view];
