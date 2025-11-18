// 座標
use {
    super::direction::{Direction, NEIGHBOR, NEIGHBOR_8},
    std::{
        fmt::{Display, Formatter, Result},
        ops::{Add, AddAssign, Sub, SubAssign},
    },
};

/// 座標の型
/// -  u8:<=255
/// - u16:<=65,535
/// - u32:<=4,294,967,295
pub type CoordType = u8;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Coord {
    row: CoordType,
    col: CoordType,
}
impl Coord {
    /// 原点
    pub const ORIGIN: Coord = Coord::new(0, 0);

    /// 座標生成
    pub const fn new(row: usize, col: usize) -> Self {
        Self {
            row: row as CoordType,
            col: col as CoordType,
        }
    }

    /// 行(y座標)を取得
    #[inline(always)]
    pub const fn row(&self) -> usize {
        self.row as usize
    }

    /// 列(x座標)を取得
    #[inline(always)]
    pub const fn col(&self) -> usize {
        self.col as usize
    }

    /// 2次元座標を1次元インデックスに変換
    pub const fn to_index(self, cols: usize) -> usize {
        self.row() * cols + self.col()
    }

    /// 1次元インデックスから座標に変換
    pub const fn from_index(idx: usize, cols: usize) -> Self {
        Coord::new(idx / cols, idx % cols)
    }

    /// 領域内に存在するかチェック
    pub const fn within(&self, rows: usize, cols: usize) -> bool {
        self.row() < rows && self.col() < cols
    }

    /// 軸ごとの距離
    pub const fn diff(&self, other: &Self) -> Self {
        Self::new(
            self.row().abs_diff(other.row()),
            self.col().abs_diff(other.col()),
        )
    }

    /// 面積取得
    pub const fn area(&self, other: &Self) -> usize {
        let diff = self.diff(other);
        diff.row() * diff.col()
    }

    /// ユークリッド距離(直線距離)を計算
    pub fn dist_euclidean(&self, other: &Self) -> f64 {
        let rowdiff = self.row().abs_diff(other.row());
        let coldiff = self.col().abs_diff(other.col());
        ((rowdiff * rowdiff + coldiff * coldiff) as f64).sqrt()
    }

    /// マンハッタン距離(上下左右の移動量の合計)を計算
    pub const fn dist_manhattan(&self, other: &Self) -> usize {
        self.row().abs_diff(other.row()) + self.col().abs_diff(other.col())
    }

    /// チェビシェフ距離(8方向移動時の最短手数)を計算
    pub fn dist_chebyshev(&self, other: &Self) -> usize {
        std::cmp::max(
            self.row().abs_diff(other.row()),
            self.col().abs_diff(other.col()),
        )
    }

    /// タプルから変換
    pub const fn from_tuple(tuple: (usize, usize)) -> Self {
        Self::new(tuple.0, tuple.1)
    }

    /// タプルに変換
    pub const fn to_tuple(self) -> (usize, usize) {
        (self.row(), self.col())
    }

    /// 4方向の隣接座標イテレータ
    pub fn neighbors4(&self, rows: usize, cols: usize) -> impl Iterator<Item = Coord> + '_ {
        NEIGHBOR.iter().filter_map(move |&dir| {
            let next = *self + dir;
            if next.within(rows, cols) {
                Some(next)
            } else {
                None
            }
        })
    }

    /// 8方向の隣接座標イテレータ
    pub fn neighbors8(&self, rows: usize, cols: usize) -> impl Iterator<Item = Coord> + '_ {
        NEIGHBOR_8.iter().filter_map(move |&dir| {
            let next = *self + dir;
            if next.within(rows, cols) {
                Some(next)
            } else {
                None
            }
        })
    }

    /// 指定座標への方向を取得
    pub const fn direction_to(&self, other: &Self) -> Option<Direction> {
        let row_diff = other.row() as i32 - self.row() as i32;
        let col_diff = other.col() as i32 - self.col() as i32;

        match (row_diff.signum(), col_diff.signum()) {
            (-1, 0) => Some(Direction::U),
            (1, 0) => Some(Direction::D),
            (0, -1) => Some(Direction::L),
            (0, 1) => Some(Direction::R),
            _ => None,
        }
    }

    /// 指定した方向に移動する。グリッド外なら None を返す。
    pub fn next(&self, dir: Direction, h: usize, w: usize) -> Option<Self> {
        let nxt = *self + dir;
        if nxt.within(h, w) {
            Some(nxt)
        } else {
            None
        }
    }
}

/// + 演算子で座標の加算
impl Add<Coord> for Coord {
    type Output = Coord;
    fn add(self, rhs: Coord) -> Self::Output {
        Coord {
            row: self.row.wrapping_add(rhs.row),
            col: self.col.wrapping_add(rhs.col),
        }
    }
}

/// + 演算子で座標の加算
impl Add<Direction> for Coord {
    type Output = Coord;
    fn add(self, rhs: Direction) -> Self::Output {
        let (dr, dc) = rhs.to_delta();
        Coord {
            row: self.row.wrapping_add(dr as CoordType),
            col: self.col.wrapping_add(dc as CoordType),
        }
    }
}

/// += 演算子で座標の加算
impl AddAssign<Coord> for Coord {
    fn add_assign(&mut self, rhs: Coord) {
        *self = *self + rhs;
    }
}

/// += 演算子で座標の加算
impl AddAssign<Direction> for Coord {
    fn add_assign(&mut self, rhs: Direction) {
        *self = *self + rhs;
    }
}

/// - 演算子で座標の減算
impl Sub<Coord> for Coord {
    type Output = Coord;
    fn sub(self, rhs: Coord) -> Self::Output {
        Coord {
            row: self.row.wrapping_sub(rhs.row),
            col: self.col.wrapping_sub(rhs.col),
        }
    }
}

/// -= 演算子で座標の減算
impl SubAssign<Coord> for Coord {
    fn sub_assign(&mut self, rhs: Coord) {
        *self = *self - rhs;
    }
}

/// タプルからの変換
impl From<(usize, usize)> for Coord {
    fn from((row, col): (usize, usize)) -> Self {
        Self::new(row, col)
    }
}

/// タプルへの変換
impl From<Coord> for (usize, usize) {
    fn from(coord: Coord) -> Self {
        (coord.row(), coord.col())
    }
}

/// 出力形式
impl Display for Coord {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} {}", self.row, self.col)
    }
}
