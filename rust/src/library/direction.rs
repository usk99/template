// 方向
use super::coord::Coord;
use std::fmt::*;
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug)]
pub enum Direction {
    /// 上
    U = 0,
    /// 右
    R = 1,
    /// 下
    D = 2,
    /// 左
    L = 3,
}

impl Direction {
    /// インデックスを方向へ
    pub fn from_index_(i: usize) -> Self {
        match i % 4 {
            0 => Self::U,
            1 => Self::R,
            2 => Self::D,
            3 => Self::L,
            _ => unreachable!(),
        }
    }

    /// 方向をインデックスに
    pub const fn to_index_(self) -> usize {
        self as usize
    }

    /// 差分 (dr, dc) を取得
    pub const fn to_delta(self) -> (isize, isize) {
        match self {
            Self::U => (-1, 0),
            Self::R => (0, 1),
            Self::D => (1, 0),
            Self::L => (0, -1),
        }
    }

    /// 反対方向
    pub const fn opposite_(self) -> Self {
        match self {
            Self::U => Self::D,
            Self::R => Self::L,
            Self::D => Self::U,
            Self::L => Self::R,
        }
    }

    /// 時計回りに90度
    pub fn clockwise_(self) -> Self {
        Self::from_index_(self as usize + 1)
    }

    /// 反時計回りに90度
    pub fn counter_clockwise_(self) -> Self {
        Self::from_index_(self as usize + 3)
    }
}
impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            match self {
                Self::U => "U",
                Self::R => "R",
                Self::D => "D",
                Self::L => "L",
            }
        )
    }
}
pub const DIRECTION: [Direction; 4] = [Direction::U, Direction::R, Direction::D, Direction::L];
pub const NEIGHBOR: [Coord; 4] = [
    Coord::new(!0, 0), // ↑
    Coord::new(0, 1),  // →
    Coord::new(1, 0),  // ↓
    Coord::new(0, !0), // ←
];
pub const NEIGHBOR_8: [Coord; 8] = [
    Coord::new(!0, 0),  // ↑
    Coord::new(0, 1),   // →
    Coord::new(1, 0),   // ↓
    Coord::new(0, !0),  // ←
    Coord::new(!0, 1),  // →↑
    Coord::new(1, 1),   // →↓
    Coord::new(1, !0),  // ←↓
    Coord::new(!0, !0), // ←↑
];
