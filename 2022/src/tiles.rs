use std::{
    fmt::Display,
    io::BufRead,
    ops::{Add, Index, IndexMut},
};

use num_traits::AsPrimitive;

pub type CoordType = i16;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point {
    pub x: CoordType,
    pub y: CoordType,
}

impl Point {
    pub const CARDINAL_DIRECTIONS: [Self; 4] = [
        Point::new(0, 1),
        Point::new(1, 0),
        Point::new(0, -1),
        Point::new(-1, 0),
    ];

    pub const fn new(x: CoordType, y: CoordType) -> Self {
        Self { x, y }
    }

    pub fn neighbors(&self) -> impl Iterator<Item = Self> + '_ {
        Self::CARDINAL_DIRECTIONS
            .iter()
            .map(move |dir| *dir + *self)
    }

    pub fn quad_distance(&self, other: Self) -> CoordType {
        let xdist = self.x - other.x;
        let ydist = self.y - other.y;
        xdist * xdist + ydist * ydist
    }

    pub fn manhattan_distance(&self, other: Self) -> CoordType {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl<N> Into<(N, N)> for Point
where
    N: Copy + 'static,
    CoordType: AsPrimitive<N>,
{
    fn into(self) -> (N, N) {
        (self.x.as_(), self.y.as_())
    }
}

impl<N> From<(N, N)> for Point
where
    N: AsPrimitive<CoordType>,
{
    fn from((x, y): (N, N)) -> Self {
        Self::new(x.as_(), y.as_())
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

pub struct Tiles<T> {
    entry_len: usize,
    line_width: usize,
    pub entries: Vec<T>,
}

pub struct Stepper {
    start: usize,
    end: usize,
    step: usize,
}

impl Tiles<u8> {
    pub fn load(input: &mut dyn BufRead, capacity: usize) -> Self {
        let mut entries = Vec::with_capacity(capacity);
        input.read_to_end(&mut entries).unwrap();
        let entry_len = entries.iter().position(|&c| c == b'\n').unwrap();
        Self {
            entry_len,
            line_width: entry_len + 1,
            entries,
        }
    }
}

impl<T: Clone> Tiles<T> {
    pub fn new(width: usize, height: usize, initial: T) -> Self {
        Self {
            entry_len: width,
            line_width: width + 1,
            entries: vec![initial; (width + 1) * height],
        }
    }
}

impl<T: Copy> Tiles<T> {
    pub fn iter_with<'a>(
        &'a self,
        stepper: impl Iterator<Item = usize> + 'a,
    ) -> impl Iterator<Item = T> + 'a {
        stepper.map(move |idx| self.entries[idx])
    }
}

impl<T> Tiles<T> {
    pub const fn width(&self) -> usize {
        self.entry_len
    }

    pub fn height(&self) -> usize {
        self.entries.len() / self.line_width
    }

    pub fn contains(&self, point: Point) -> bool {
        point.x >= 0
            && point.y >= 0
            && point.x < self.width() as CoordType
            && point.y < self.height() as CoordType
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

impl Display for Tiles<u8> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.rows_steppers() {
            for idx in row.iter() {
                write!(f, "{}", self.entries[idx] as char)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Display for Tiles<u32> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.rows_steppers() {
            for idx in row.iter() {
                write!(f, "{:3} ", self.entries[idx])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T, C> Index<C> for Tiles<T>
where
    C: Into<(usize, usize)>,
{
    type Output = T;

    fn index(&self, point: C) -> &T {
        let (x, y) = point.into();
        &self.entries[y * self.line_width + x]
    }
}

impl<T, C> IndexMut<C> for Tiles<T>
where
    C: Into<(usize, usize)>,
{
    fn index_mut(&mut self, point: C) -> &mut T {
        let (x, y) = point.into();
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
