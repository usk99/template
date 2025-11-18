// 方向
use super::coord::Coord;
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
            0 => Direction::U,
            1 => Direction::R,
            2 => Direction::D,
            3 => Direction::L,
            _ => unreachable!(),
        }
    }

    /// 方向をインデックスに
    pub fn to_index_(self) -> usize {
        self as usize
    }

    /// 差分 (dr, dc) を取得
    pub fn to_delta(self) -> (isize, isize) {
        match self {
            Direction::U => (-1, 0),
            Direction::R => (0, 1),
            Direction::D => (1, 0),
            Direction::L => (0, -1),
        }
    }

    /// 反対方向
    pub fn opposite_(self) -> Self {
        match self {
            Direction::U => Direction::D,
            Direction::R => Direction::L,
            Direction::D => Direction::U,
            Direction::L => Direction::R,
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
impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::U => "U",
                Direction::R => "R",
                Direction::D => "D",
                Direction::L => "L",
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
