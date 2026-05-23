// 二次元を一次元で管理
use {
    super::coord::*,
    itertools::Itertools,
    std::{fmt::*, ops::*, slice::*},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Grid<T> {
    pub vec: Vec<T>,
    pub row: usize,
    pub col: usize,
}
impl<T> Grid<T> {
    /// 新しいGridを作成
    pub fn new(vec: Vec<T>, row: usize, col: usize) -> Self {
        assert!(
            row > 0 && col > 0 && vec.len() == row * col,
            "Grid({row}x{col}, len={})",
            vec.len()
        );
        Self { vec, row, col }
    }

    #[inline]
    /// 安全に要素を取得
    pub fn get(&self, coord: Coord) -> Option<&T> {
        if coord.within(self.row, self.col) {
            Some(self.get_unchecked(coord))
        } else {
            None
        }
    }

    #[inline(always)]
    /// 非安全に要素を取得
    pub fn get_unchecked(&self, coord: Coord) -> &T {
        unsafe { self.vec.get_unchecked(coord.to_index(self.col)) }
    }

    #[inline]
    /// 安全にmut要素を取得
    pub fn get_mut(&mut self, coord: Coord) -> Option<&mut T> {
        if coord.within(self.row, self.col) {
            Some(self.get_unchecked_mut(coord))
        } else {
            None
        }
    }

    #[inline(always)]
    /// 非安全にmut要素を取得
    pub fn get_unchecked_mut(&mut self, coord: Coord) -> &mut T {
        unsafe { self.vec.get_unchecked_mut(coord.to_index(self.col)) }
    }

    #[inline(always)]
    /// 指定座標が有効か
    pub const fn within(&self, coord: Coord) -> bool {
        coord.within(self.row, self.col)
    }

    #[inline]
    /// 要素を入れ替え
    pub fn swap(&mut self, a: Coord, b: Coord) {
        self.vec.swap(a.to_index(self.col), b.to_index(self.col));
    }

    #[inline(always)]
    /// 要素数取得
    pub const fn size(&self) -> usize {
        self.vec.len()
    }

    #[inline(always)]
    /// イテレータ取得
    pub fn iter(&self) -> Iter<'_, T> {
        self.vec.iter()
    }

    #[inline(always)]
    /// mutイテレータ取得
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        self.vec.iter_mut()
    }

    #[inline(always)]
    /// スライス取得
    pub fn as_slice(&self) -> &[T] {
        &self.vec
    }

    #[inline(always)]
    /// mutスライス取得
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.vec
    }

    /// マッピング
    pub fn map<U, F: FnMut(&T) -> U>(&self, f: F) -> Grid<U> {
        Grid {
            vec: self.vec.iter().map(f).collect(),
            row: self.row,
            col: self.col,
        }
    }

    /// 行ごと
    #[inline]
    pub fn rows(&self) -> Chunks<'_, T> {
        self.vec.chunks(self.col)
    }

    /// 行ごと
    #[inline]
    pub fn rows_mut(&mut self) -> ChunksMut<'_, T> {
        self.vec.chunks_mut(self.col)
    }
}

impl<T: Clone> From<Vec<Vec<T>>> for Grid<T> {
    /// 二次元VecからGridを作成
    fn from(value: Vec<Vec<T>>) -> Self {
        let row = value.len();
        assert!(row > 0, "Grid requires at least 1 row");
        let col = value[0].len();
        assert!(col > 0, "Grid requires at least 1 col");
        let vec = value.into_iter().flatten().collect();
        Self { vec, row, col }
    }
}

impl<T: Clone> From<Grid<T>> for Vec<Vec<T>> {
    /// Gridから二次元Vecを作成
    fn from(val: Grid<T>) -> Self {
        val.vec.chunks(val.col).map(<[T]>::to_vec).collect()
    }
}

impl<T: Clone> Grid<T> {
    /// 全要素を指定値で埋める
    pub fn fill(&mut self, value: T) {
        self.vec.fill(value);
    }

    /// 時計回りに90度回転した新しいグリッドを返す
    pub fn rotate_right(&self) -> Self {
        let (h, w) = (self.row, self.col);
        let mut new_vec = Vec::with_capacity(self.vec.len());
        // 元の(r, c) -> 回転後(c, h-1-r)
        for c in 0..w {
            for r in (0..h).rev() {
                new_vec.push(self[(r, c)].clone());
            }
        }
        Self {
            vec: new_vec,
            row: w,
            col: h,
        } // 行と列が入れ替わる
    }

    /// 反時計回りに90度回転
    pub fn rotate_left(&self) -> Self {
        let (h, w) = (self.row, self.col);
        let mut new_vec = Vec::with_capacity(self.vec.len());
        // 元の(r, c) -> 回転後(w-1-c, r)
        for c in (0..w).rev() {
            for r in 0..h {
                new_vec.push(self[(r, c)].clone());
            }
        }
        Self {
            vec: new_vec,
            row: w,
            col: h,
        }
    }

    /// 180度回転
    pub fn rotate_180(&self) -> Self {
        let mut vec = self.vec.clone();
        vec.reverse();
        Self {
            vec,
            row: self.row,
            col: self.col,
        }
    }

    /// 転置（行と列の入れ替え）
    pub fn transpose(&self) -> Self {
        let (h, w) = (self.row, self.col);
        let mut new_vec = Vec::with_capacity(self.vec.len());
        for c in 0..w {
            for r in 0..h {
                new_vec.push(self[(r, c)].clone());
            }
        }
        Self {
            vec: new_vec,
            row: w,
            col: h,
        }
    }

    /// 上下反転（行の順序を逆にする）
    pub fn flip_v(&self) -> Self {
        let mut new_vec = Vec::with_capacity(self.vec.len());
        for r in (0..self.row).rev() {
            for c in 0..self.col {
                new_vec.push(self[(r, c)].clone());
            }
        }
        Self {
            vec: new_vec,
            row: self.row,
            col: self.col,
        }
    }

    /// 左右反転（列の順序を逆にする）
    pub fn flip_h(&self) -> Self {
        let mut new_vec = Vec::with_capacity(self.vec.len());
        for r in 0..self.row {
            for c in (0..self.col).rev() {
                new_vec.push(self[(r, c)].clone());
            }
        }
        Self {
            vec: new_vec,
            row: self.row,
            col: self.col,
        }
    }

    /// 指定値で埋めた新しいGridを作成
    pub fn new_filled(val: T, row: usize, col: usize) -> Self {
        Self {
            vec: vec![val; row * col],
            row,
            col,
        }
    }
}

impl<T: PartialEq> Grid<T> {
    /// 指定した値の座標を探す（最初に見つかったもの）
    pub fn find(&self, target: &T) -> Option<Coord> {
        self.vec
            .iter()
            .position(|x| x == target)
            .map(|idx| Coord::from_index(idx, self.col))
    }

    /// 指定した値の個数を数える
    pub fn count(&self, target: &T) -> usize {
        self.vec.iter().filter(|&x| x == target).count()
    }
}

impl<T> Index<usize> for Grid<T> {
    type Output = [T];
    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        let start = index * self.col;
        &self.vec[start..start + self.col]
    }
}
impl<T> IndexMut<usize> for Grid<T> {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let start = index * self.col;
        &mut self.vec[start..start + self.col]
    }
}
impl<T> Index<Coord> for Grid<T> {
    type Output = T;
    #[inline(always)]
    fn index(&self, coord: Coord) -> &Self::Output {
        debug_assert!(
            coord.within(self.row, self.col),
            "Grid out of bounds: {:?} not within {}x{}",
            coord,
            self.row,
            self.col
        );
        &self.vec[coord.to_index(self.col)]
    }
}
impl<T> IndexMut<Coord> for Grid<T> {
    #[inline(always)]
    fn index_mut(&mut self, coord: Coord) -> &mut Self::Output {
        debug_assert!(
            coord.within(self.row, self.col),
            "Grid out of bounds: {:?} not within {}x{}",
            coord,
            self.row,
            self.col
        );
        &mut self.vec[coord.to_index(self.col)]
    }
}
impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;
    #[inline(always)]
    fn index(&self, (r, c): (usize, usize)) -> &Self::Output {
        &self[Coord::new(r, c)]
    }
}
impl<T> IndexMut<(usize, usize)> for Grid<T> {
    #[inline(always)]
    fn index_mut(&mut self, (r, c): (usize, usize)) -> &mut Self::Output {
        &mut self[Coord::new(r, c)]
    }
}

/// 視覚化
impl<T: Display> Grid<T> {
    pub fn visualize(&self) -> String {
        self.visualize_with(|val| val.to_string())
    }

    /// 二次元vecを表示
    pub fn to_string(&self, sep: &str) -> String {
        self.vec
            .chunks(self.col)
            .map(|row| row.iter().join(sep))
            .collect::<Vec<String>>()
            .join("\n")
    }
}
impl<T> Grid<T> {
    pub fn visualize_with<F: FnMut(&T) -> String>(&self, mut formatter: F) -> String {
        if self.vec.is_empty() {
            eprintln!("[Empty Grid {}x{}]", self.row, self.col);
            return "[]\n".to_string();
        }
        let formatted_cells: Vec<String> = self.vec.iter().map(&mut formatter).collect();
        let data_max_width = formatted_cells.iter().map(String::len).max().unwrap_or(1);
        let mut row_header_width = 0;
        if self.row != 0 {
            row_header_width = (self.row - 1).to_string().len();
        }
        let mut result = String::new();
        result.push_str(&" ".repeat(row_header_width));
        result.push_str(" | ");

        for c in 0..self.col {
            let _ = write!(result, "{c:>data_max_width$} ");
        }
        result.push('\n');
        result.push_str(&"-".repeat(row_header_width));
        result.push_str("-+-");
        let data_line_width = (data_max_width + 1) * self.col;
        result.push_str(&"-".repeat(data_line_width.saturating_sub(1)));
        result.push('\n');

        for r in 0..self.row {
            let _ = write!(result, "{r:>row_header_width$}");
            result.push_str(" | ");
            for c in 0..self.col {
                let idx = r * self.col + c;
                let _ = write!(
                    result,
                    "{val:>data_max_width$} ",
                    val = formatted_cells[idx]
                );
            }
            result.push('\n');
        }
        eprintln!("{result}");
        result
    }
}

/// Bool型の記号化設定
impl Grid<bool> {
    pub fn visualize_bool(&self) -> String {
        self.visualize_with(|&val| if val { "#" } else { "." }.to_string())
    }
}

/// gridを作成するマクロ
#[macro_export]
macro_rules! grid {
    ($val:expr, $r:expr, $c:expr) => {
        $crate::library::grid::Grid::new_filled($val, $r, $c)
    };
}
