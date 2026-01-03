use crate::library::{coord::*, direction::*};
use std::ops::{Add, AddAssign};

impl Coord {
    /// 指定した方向に移動する。グリッド外なら None を返す。
    #[inline]
    pub fn next(&self, dir: Direction, h: usize, w: usize) -> Option<Self> {
        let nx = *self + dir;
        if nx.within(h, w) { Some(nx) } else { None }
    }

    /// 4方向の隣接座標イテレータ
    #[inline]
    pub fn neighbors4(&self, rows: usize, cols: usize) -> impl Iterator<Item = Self> + '_ {
        Direction::iter().filter_map(move |&dir| {
            let next = *self + dir;
            if next.within(rows, cols) {
                Some(next)
            } else {
                None
            }
        })
    }

    /// 指定座標への方向を取得
    #[inline]
    pub fn direction_to(&self, other: &Self) -> Option<Direction> {
        let row_diff = other.row as isize - self.row as isize;
        let col_diff = other.col as isize - self.col as isize;
        match (row_diff.signum(), col_diff.signum()) {
            (-1, 0) => Some(Direction::U),
            (1, 0) => Some(Direction::D),
            (0, -1) => Some(Direction::L),
            (0, 1) => Some(Direction::R),
            _ => None,
        }
    }
}

/// + 演算子で座標の加算
impl Add<Direction> for Coord {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Direction) -> Self::Output {
        let (dr, dc) = rhs.to_delta();
        Self {
            row: self.row.wrapping_add(dr as CoordType),
            col: self.col.wrapping_add(dc as CoordType),
        }
    }
}

/// += 演算子で座標の加算
impl AddAssign<Direction> for Coord {
    #[inline]
    fn add_assign(&mut self, rhs: Direction) {
        *self = *self + rhs;
    }
}
