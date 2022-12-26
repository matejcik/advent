use std::{
    fmt::Display,
    io::BufRead,
    ops::{Add, Index, IndexMut, Mul, Rem, Sub},
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

    pub fn min_bound(&self, other: Self) -> Self {
        Self::new(self.x.min(other.x), self.y.min(other.y))
    }

    pub fn max_bound(&self, other: Self) -> Self {
        Self::new(self.x.max(other.x), self.y.max(other.y))
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

impl Mul<CoordType> for Point {
    type Output = Self;

    fn mul(self, rhs: CoordType) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl Sub<Point> for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Rem<Point> for Point {
    type Output = Self;

    fn rem(self, rhs: Point) -> Self::Output {
        Self::new(self.x.rem_euclid(rhs.x), self.y.rem_euclid(rhs.y))
    }
}

pub struct Tiles<T> {
    pub entry_len: usize,
    pub line_width: usize,
    pub entries: Vec<T>,
}

#[derive(Debug)]
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

    pub fn from_vec(entries: Vec<u8>, entry_len: usize, terminator_len: usize) -> Self {
        assert!(entries.len() % (entry_len + terminator_len) == 0);
        Self {
            entry_len,
            line_width: entry_len + terminator_len,
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

    pub fn reset(&mut self, initial: T) {
        self.entries.iter_mut().for_each(|e| *e = initial.clone());
    }
}

impl<T: Clone> Clone for Tiles<T> {
    fn clone(&self) -> Self {
        Self {
            entry_len: self.entry_len,
            line_width: self.line_width,
            entries: self.entries.clone(),
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

    pub fn region(self, x: usize, y: usize, mx: usize, my: usize) -> Self {
        assert!(mx <= self.width());
        assert!(my <= self.height());
        let mut start = y * self.line_width + x;
        let step = self.line_width;
        let width = mx - x;
        let height = my - y;
        let mut entries = Vec::with_capacity(width * height);
        for _ in 0..height {
            let end = start + width;
            entries.extend_from_slice(&self.entries[start..end]);
            start += step;
        }
        Self {
            line_width: width,
            entry_len: width,
            entries,
        }
    }

    pub fn copy(&mut self, x: usize, y: usize, other: &Self) {
        let mut start = y * self.line_width + x;
        let step = self.line_width;
        for oy in 0..other.height() {
            let other_start = oy * other.line_width;
            let len = other.entry_len;
            self.entries[start..start + len]
                .copy_from_slice(&other.entries[other_start..other_start + len]);
            start += step;
        }
    }
}

impl<T> Tiles<T> {
    pub const fn width(&self) -> usize {
        self.entry_len
    }

    pub fn height(&self) -> usize {
        self.entries.len() / self.line_width
    }

    pub fn size(&self) -> Point {
        Point::new(self.width() as CoordType, self.height() as CoordType)
    }

    pub fn contains(&self, point: Point) -> bool {
        point.x >= 0
            && point.y >= 0
            && point.x < self.width() as CoordType
            && point.y < self.height() as CoordType
    }

    pub fn get(&self, point: Point) -> Option<&T> {
        if self.contains(point) {
            Some(&self.entries[self.index_for(point)])
        } else {
            None
        }
    }

    pub fn index_for(&self, point: Point) -> usize {
        (point.y as usize) * self.line_width + (point.x as usize)
    }

    pub fn coords_for(&self, idx: usize) -> Point {
        Point::new(
            (idx % self.line_width) as CoordType,
            (idx / self.line_width) as CoordType,
        )
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
