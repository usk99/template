// 方向
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Debug)]
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

impl Direction {
    /// 全方向定数
    pub const ALL: [Self; 4] = [Direction::U, Direction::R, Direction::D, Direction::L];

    /// イテレータ
    pub fn iter() -> std::slice::Iter<'static, Self> {
        Self::ALL.iter()
    }

    /// 差分 (dr, dc) を取得
    #[inline]
    pub const fn to_delta(self) -> (isize, isize) {
        match self {
            Self::U => (-1, 0),
            Self::R => (0, 1),
            Self::D => (1, 0),
            Self::L => (0, -1),
        }
    }

    /// 差分から逆変換
    #[inline]
    pub fn from_delta(delta: (isize, isize)) -> Option<Self> {
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

impl From<usize> for Direction {
    fn from(i: usize) -> Self {
        match i % 4 {
            0 => Self::U,
            1 => Self::R,
            2 => Self::D,
            3 => Self::L,
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
