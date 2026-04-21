// 座標系
use {
    super::io::*,
    std::{cmp::*, fmt::*, ops::*},
};

/// 座標の型
/// -  u8:<255
/// - u16:<65,535
/// - u32:<4,294,967,295
/// - 境界値ちょうどの座標はwrapping_add等で範囲外に出る可能性があるため注意
/// - その場合一つ上の型にすると安全
pub type CoordType = u32;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Direction {
    /// 上(Up)
    U = 0,
    /// 右(Right)
    R = 1,
    /// 下(Down)
    D = 2,
    /// 左(Left)
    L = 3,
}
crate::impl_elem!(Direction);
impl Direction {
    /// 全方向定数
    pub const ALL: [Self; 4] = [Direction::U, Direction::R, Direction::D, Direction::L];

    pub const fn new(i: usize) -> Self {
        match i % 4 {
            0 => Self::U,
            1 => Self::R,
            2 => Self::D,
            3 => Self::L,
            _ => unreachable!(),
        }
    }

    /// イテレータ
    pub fn iter() -> std::slice::Iter<'static, Self> {
        Self::ALL.iter()
    }

    /// 差分 (dr, dc) を取得
    #[inline]
    pub const fn delta(self) -> (CoordType, CoordType) {
        match self {
            Self::U => (!0, 0),
            Self::R => (0, 1),
            Self::D => (1, 0),
            Self::L => (0, !0),
        }
    }

    /// 差分から逆変換
    #[inline]
    pub const fn from_delta(delta: (isize, isize)) -> Option<Self> {
        match delta {
            (-1, 0) => Some(Self::U),
            (0, 1) => Some(Self::R),
            (1, 0) => Some(Self::D),
            (0, -1) => Some(Self::L),
            _ => None,
        }
    }

    /// 反対方向
    pub const fn opposite(self) -> Self {
        match self {
            Self::U => Self::D,
            Self::R => Self::L,
            Self::D => Self::U,
            Self::L => Self::R,
        }
    }

    /// 時計回りに90度
    pub const fn clockwise(self) -> Self {
        match self {
            Self::U => Self::R,
            Self::R => Self::D,
            Self::D => Self::L,
            Self::L => Self::U,
        }
    }

    /// 反時計回りに90度
    pub const fn counter_clockwise(self) -> Self {
        match self {
            Self::U => Self::L,
            Self::R => Self::U,
            Self::D => Self::R,
            Self::L => Self::D,
        }
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Coord {
    pub r: CoordType,
    pub c: CoordType,
}
crate::impl_elem!(Coord);
impl Coord {
    /// 原点
    pub const ORIGIN: Self = Self { r: 0, c: 0 };

    /// 座標生成
    /// - row, col は `CoordType::MAX` 未満であること (境界値はwrapping判定が破綻するためNG)
    #[inline]
    pub fn new(row: usize, col: usize) -> Self {
        debug_assert!(
            row < CoordType::MAX as usize && col < CoordType::MAX as usize,
            "Coord({}, {}) overflows {} (MAX={}). Wrapping logic requires strict inequality.",
            row,
            col,
            std::any::type_name::<CoordType>(),
            CoordType::MAX
        );
        Self {
            r: row as CoordType,
            c: col as CoordType,
        }
    }

    /// 行(y座標)をusizeで取得
    #[inline(always)]
    pub const fn row(&self) -> usize {
        self.r as usize
    }

    /// 列(x座標)をusizeで取得
    #[inline(always)]
    pub const fn col(&self) -> usize {
        self.c as usize
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
            r: self.r.abs_diff(other.r),
            c: self.c.abs_diff(other.c),
        }
    }

    /// 面積取得
    #[inline]
    pub const fn rect_area(&self, other: &Self) -> usize {
        let d = self.diff(other);
        (d.r as usize) * (d.c as usize)
    }

    /// ユークリッド距離(直線距離)の2乗を計算
    #[inline]
    pub const fn dist_euclidean2(&self, other: &Self) -> usize {
        let d = self.diff(other);
        let r = d.row();
        let c = d.col();
        r * r + c * c
    }

    /// マンハッタン距離(上下左右の移動量の合計)を計算
    #[inline]
    pub const fn dist_manhattan(&self, other: &Self) -> usize {
        let d = self.diff(other);
        d.row() + d.col()
    }

    /// チェビシェフ距離(8方向移動時の最短手数)を計算
    #[inline]
    pub const fn dist_chebyshev(&self, other: &Self) -> usize {
        let d = self.diff(other);
        if d.row() > d.col() { d.row() } else { d.col() }
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
        const NEIGHBOR_8: [(CoordType, CoordType); 8] = [
            (!0, 0),
            (0, 1),
            (1, 0),
            (0, !0),
            (!0, 1),
            (1, 1),
            (1, !0),
            (!0, !0),
        ];
        NEIGHBOR_8.iter().filter_map(move |&(dr, dc)| {
            let next = Self {
                r: self.r.wrapping_add(dr as CoordType),
                c: self.c.wrapping_add(dc as CoordType),
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
        let row_diff = other.r as isize - self.r as isize;
        let col_diff = other.c as isize - self.c as isize;
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
impl Add for Coord {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r.wrapping_add(rhs.r),
            c: self.c.wrapping_add(rhs.c),
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
            r: self.r.wrapping_sub(rhs.r),
            c: self.c.wrapping_sub(rhs.c),
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
        write!(f, "{} {}", self.r, self.c)
    }
}

impl Debug for Coord {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Coord({}, {})", self.r, self.c)
    }
}

/// + 演算子で座標の加算
impl Add<Direction> for Coord {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Direction) -> Self::Output {
        let (dr, dc) = rhs.delta();
        Self {
            r: self.r.wrapping_add(dr as CoordType),
            c: self.c.wrapping_add(dc as CoordType),
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

impl Mul<CoordType> for Direction {
    type Output = Coord;
    #[inline]
    fn mul(self, rhs: CoordType) -> Coord {
        let (dr, dc) = self.delta();
        Coord {
            r: dr.wrapping_mul(rhs),
            c: dc.wrapping_mul(rhs),
        }
    }
}

/// Coord * n でスケール
impl Mul<CoordType> for Coord {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: CoordType) -> Self {
        Self {
            r: self.r.wrapping_mul(rhs),
            c: self.c.wrapping_mul(rhs),
        }
    }
}
