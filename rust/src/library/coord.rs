// 座標
use {
    super::direction::Direction,
    std::{cmp::*, fmt::*, ops::*},
};

/// 座標の型
/// -  u8:<255
/// - u16:<65,535
/// - u32:<4,294,967,295
/// - 境界値ちょうどの座標はwrapping_add等で範囲外に出る可能性があるため注意
/// - その場合一つ上の型にすると安全
pub type CoordType = u8;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Coord {
    pub row: CoordType,
    pub col: CoordType,
}

impl Coord {
    /// 原点
    pub const ORIGIN: Self = Self { row: 0, col: 0 };

    /// 座標生成
    #[inline]
    pub fn new(row: usize, col: usize) -> Self {
        // デバッグ時のみ、型チェック
        #[cfg(debug_assertions)]
        if row > CoordType::MAX as usize || col > CoordType::MAX as usize {
            panic!(
                "Coord overflow: ({} {}) fits in {:?}",
                row,
                col,
                std::any::type_name::<CoordType>()
            );
        }
        // 型の上限と協会が同じ場合は、Direction移動時に範囲チェックが正常に動かない可能性があるのでアウト
        // 型の上限とグリッドサイズが同じになりそうな場合の警告
        // (Wrapping戦略が破綻するリスクがあるため)
        if row == CoordType::MAX as usize || col == CoordType::MAX as usize {
            eprintln!(
                "Warning: Coord value ({}, {}) is at the limit of {:?}. \
                    If grid size > {}, wrapping check logic will fail.",
                row,
                col,
                std::any::type_name::<CoordType>(),
                CoordType::MAX
            );
        }

        Self {
            row: row as CoordType,
            col: col as CoordType,
        }
    }

    /// 行(y座標)をusizeで取得
    #[inline(always)]
    pub const fn row(&self) -> usize {
        self.row as usize
    }

    /// 列(x座標)をusizeで取得
    #[inline(always)]
    pub const fn col(&self) -> usize {
        self.col as usize
    }

    /// 領域内チェック
    #[inline(always)]
    pub const fn within(&self, rows: usize, cols: usize) -> bool {
        // 負の値は来ないようになっている
        self.row() < rows && self.col() < cols
    }

    /// 2次元座標を1次元インデックスに変換
    #[inline(always)]
    pub const fn to_index(self, cols: usize) -> usize {
        self.row() * cols + self.col()
    }

    /// 1次元インデックスから座標に変換
    #[inline(always)]
    pub fn from_index(idx: usize, cols: usize) -> Self {
        Self::new(idx / cols, idx % cols)
    }

    /// 2次元全座標イテレータ
    #[inline]
    pub fn all(rows: usize, cols: usize) -> impl Iterator<Item = Self> {
        (0..rows).flat_map(move |r| (0..cols).map(move |c| Self::new(r, c)))
    }

    /// 絶対差分
    #[inline]
    pub const fn diff(&self, other: &Self) -> Self {
        Self {
            row: self.row.abs_diff(other.row),
            col: self.col.abs_diff(other.col),
        }
    }

    /// 面積取得
    #[inline]
    pub fn area(&self, other: &Self) -> usize {
        let d = self.diff(other);
        (d.row as usize) * (d.col as usize)
    }

    /// ユークリッド距離(直線距離)を計算
    #[inline]
    pub fn dist_euclidean(&self, other: &Self) -> f64 {
        let d = self.diff(other);
        let r = d.row as f64;
        let c = d.col as f64;
        (r * r + c * c).sqrt()
    }

    /// マンハッタン距離(上下左右の移動量の合計)を計算
    #[inline]
    pub const fn dist_manhattan(&self, other: &Self) -> usize {
        let d = self.diff(other);
        d.row() + d.col()
    }

    /// チェビシェフ距離(8方向移動時の最短手数)を計算
    #[inline]
    pub fn dist_chebyshev(&self, other: &Self) -> usize {
        let d = self.diff(other);
        max(d.row(), d.col())
    }

    /// 指定した方向に移動する。グリッド外なら None を返す。
    #[inline]
    pub fn next(&self, dir: Direction, h: usize, w: usize) -> Option<Self> {
        let nx = *self + dir;
        if nx.within(h, w) {
            Some(nx)
        } else {
            None
        }
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

    /// 8方向の隣接座標イテレータ
    #[inline]
    pub fn neighbors8(&self, rows: usize, cols: usize) -> impl Iterator<Item = Self> + '_ {
        const NEIGHBOR_8: [(isize, isize); 8] = [
            (-1, 0),
            (0, 1),
            (1, 0),
            (0, -1),
            (-1, 1),
            (1, 1),
            (1, -1),
            (-1, -1),
        ];
        NEIGHBOR_8.iter().filter_map(move |&(dr, dc)| {
            let next = Self {
                row: self.row.wrapping_add(dr as CoordType),
                col: self.col.wrapping_add(dc as CoordType),
            };
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

/// + 演算子で座標の加算
impl Add for Coord {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            row: self.row.wrapping_add(rhs.row),
            col: self.col.wrapping_add(rhs.col),
        }
    }
}

/// += 演算子で座標の加算
impl AddAssign for Coord {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

/// - 演算子で座標の減算
impl Sub for Coord {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            row: self.row.wrapping_sub(rhs.row),
            col: self.col.wrapping_sub(rhs.col),
        }
    }
}

/// -= 演算子で座標の減算
impl SubAssign for Coord {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl From<(usize, usize)> for Coord {
    #[inline]
    fn from((row, col): (usize, usize)) -> Self {
        Self::new(row, col)
    }
}

/// 出力形式
impl Display for Coord {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} {}", self.row, self.col)
    }
}

impl Debug for Coord {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Coord({}, {})", self.row, self.col)
    }
}
