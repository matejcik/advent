use std::{collections::HashSet, io::BufRead, ops::{Sub, Add}};

use bstr::io::BufReadExt;

use crate::{parse_nums, Solver};

const MAX_VOXELS: usize = 5000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Voxel {
    x: i32,
    y: i32,
    z: i32,
}

impl Voxel {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    fn load(line: &[u8]) -> Self {
        let mut numbers = [0; 3];
        let nums = parse_nums(line, &mut numbers);
        assert_eq!(nums, 3);
        Self::new(numbers[0] as i32, numbers[1] as i32, numbers[2] as i32)
    }

    fn adjacent(&self) -> impl Iterator<Item = Self> + '_ {
        const OFFSETS: &[(i32, i32, i32)] = &[
            (0, 0, 1),
            (0, 0, -1),
            (0, 1, 0),
            (0, -1, 0),
            (1, 0, 0),
            (-1, 0, 0),
        ];
        OFFSETS
            .iter()
            .map(move |(dx, dy, dz)| Self::new(self.x + dx, self.y + dy, self.z + dz))
    }

    fn min(&self, other: &Self) -> Self {
        Self::new(
            self.x.min(other.x),
            self.y.min(other.y),
            self.z.min(other.z),
        )
    }

    fn max(&self, other: &Self) -> Self {
        Self::new(
            self.x.max(other.x),
            self.y.max(other.y),
            self.z.max(other.z),
        )
    }

    fn in_bounds(&self, min: &Self, max: &Self) -> bool {
        self.x >= min.x
            && self.x <= max.x
            && self.y >= min.y
            && self.y <= max.y
            && self.z >= min.z
            && self.z <= max.z
    }
}

impl Sub<Voxel> for Voxel {
    type Output = Voxel;

    fn sub(self, other: Voxel) -> Self::Output {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Sub<i32> for Voxel {
    type Output = Voxel;

    fn sub(self, other: i32) -> Self::Output {
        Self::new(self.x - other, self.y - other, self.z - other)
    }
}

impl Add<i32> for Voxel {
    type Output = Voxel;

    fn add(self, other: i32) -> Self::Output {
        Self::new(self.x + other, self.y + other, self.z + other)
    }
}

fn part1_count_faces(mut input: &mut dyn BufRead) -> String {
    let mut voxels = HashSet::with_capacity(MAX_VOXELS);
    let mut faces = 0;
    input
        .for_byte_line(|line| {
            voxels.insert(Voxel::load(line));
            Ok(true)
        })
        .unwrap();

    for voxel in voxels.iter() {
        for neighbor in voxel.adjacent() {
            if !voxels.contains(&neighbor) {
                faces += 1;
            }
        }
    }
    faces.to_string()
}

fn part2_count_exterior_faces_bfs(mut input: &mut dyn BufRead) -> String {
    let mut voxels = HashSet::with_capacity(MAX_VOXELS);
    let mut faces = 0;
    let mut top_left_close = Voxel::new(i32::MAX, i32::MAX, i32::MAX);
    let mut bottom_right_far = Voxel::new(0, 0, 0);

    input
        .for_byte_line(|line| {
            let voxel = Voxel::load(line);
            voxels.insert(voxel);
            top_left_close = top_left_close.min(&voxel);
            bottom_right_far = bottom_right_far.max(&voxel);
            Ok(true)
        })
        .unwrap();

    let sizes = bottom_right_far - top_left_close;
    let volume = sizes.x * sizes.y * sizes.z;
    let mut queue = Vec::with_capacity(volume as usize);
    let mut visited = HashSet::with_capacity(volume as usize);
    queue.push(top_left_close);
    visited.insert(top_left_close);
    let min = top_left_close - 1;
    let max = bottom_right_far + 1;
    while let Some(voxel) = queue.pop() {
        for neighbor in voxel.adjacent() {
            if !neighbor.in_bounds(&min, &max) {
                continue;
            } else if voxels.contains(&neighbor) {
                faces += 1;
            } else if !visited.contains(&neighbor) {
                queue.push(neighbor);
                visited.insert(neighbor);
            }
        }
    }
    faces.to_string()
}

pub const SOLVERS: &[Solver] = &[part1_count_faces, part2_count_exterior_faces_bfs];
