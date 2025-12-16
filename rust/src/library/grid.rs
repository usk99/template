// 二次元を一次元で管理
use {
    super::coord::*,
    crate::log,
    itertools::Itertools,
    std::{fmt::*, ops::*, slice::*},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Grid<T> {
    vec: Vec<T>,
    pub row: usize,
    pub col: usize,
}
impl<T> Grid<T> {
    /// 新しいGridを作成
    pub const fn new(vec: Vec<T>, row: usize, col: usize) -> Self {
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
    pub const fn contains(&self, coord: Coord) -> bool {
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
        let size = if self.col > 0 { self.col } else { 0 };
        self.vec.chunks(size)
    }

    /// 行ごと
    #[inline]
    pub fn rows_mut(&mut self) -> ChunksMut<'_, T> {
        let size = if self.col > 0 { self.col } else { 0 };
        self.vec.chunks_mut(size)
    }
}

impl<T: Clone> From<Vec<Vec<T>>> for Grid<T> {
    /// 二次元VecからGridを作成
    fn from(value: Vec<Vec<T>>) -> Self {
        let row = value.len();
        let col = if row == 0 { 0 } else { value[0].len() };
        let vec = value.into_iter().flatten().collect();
        Self { vec, row, col }
    }
}

impl<T: Clone> From<Grid<T>> for Vec<Vec<T>> {
    /// Gridから二次元Vecを作成
    fn from(val: Grid<T>) -> Self {
        if val.col == 0 {
            vec![Vec::new(); val.row]
        } else {
            val.vec.chunks(val.col).map(|ch| ch.to_vec()).collect()
        }
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
        self.get(coord).unwrap_or_else(|| {
            panic!(
                "Grid index out of bounds: {:?} is not within {}x{}",
                coord, self.row, self.col
            )
        })
    }
}
impl<T> IndexMut<Coord> for Grid<T> {
    #[inline(always)]
    fn index_mut(&mut self, coord: Coord) -> &mut Self::Output {
        let (r, c) = (self.row, self.col);
        self.get_mut(coord).unwrap_or_else(|| {
            panic!(
                "Grid index out of bounds: {:?} is not within {}x{}",
                coord, r, c
            )
        })
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
            log!("[Empty Grid {}x{}]", self.row, self.col);
            return "[]\n".to_string();
        }
        let formatted_cells: Vec<String> = self.vec.iter().map(&mut formatter).collect();
        let data_max_width = formatted_cells.iter().map(String::len).max().unwrap_or(1);
        let mut row_header_width = 0;
        if self.row != 0 {
            row_header_width = (self.row - 1).to_string().len()
        };
        let mut result = String::new();
        result.push_str(&" ".repeat(row_header_width));
        result.push_str(" | ");

        for c in 0..self.col {
            result.push_str(&format!("{col:>width$} ", col = c, width = data_max_width));
        }
        result.push('\n');
        result.push_str(&"-".repeat(row_header_width));
        result.push_str("-+-");
        let data_line_width = (data_max_width + 1) * self.col;
        result.push_str(&"-".repeat(data_line_width.saturating_sub(1)));
        result.push('\n');

        for r in 0..self.row {
            result.push_str(&format!("{row:>width$}", row = r, width = row_header_width));
            result.push_str(" | ");
            for c in 0..self.col {
                let idx = r * self.col + c;
                result.push_str(&format!(
                    "{val:>width$} ",
                    val = formatted_cells[idx],
                    width = data_max_width
                ));
            }
            result.push('\n');
        }
        log!("{result}");
        result
    }
}

/// Bool型の記号化設定
impl Grid<bool> {
    pub fn visualize_bool(&self) -> String {
        self.visualize_with(|&val| if val { "#" } else { "." }.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // テスト用のヘルパー: 2x3のグリッドを作成
    // 0 1 2
    // 3 4 5
    fn setup_2x3() -> Grid<i32> {
        let vec = vec![0, 1, 2, 3, 4, 5];
        Grid::new(vec, 2, 3)
    }

    #[test]
    fn test_basic_construction_and_access() {
        let grid = setup_2x3();

        assert_eq!(grid.row, 2);
        assert_eq!(grid.col, 3);
        assert_eq!(grid.size(), 6);

        // Coordでのアクセス
        assert_eq!(grid[Coord::new(0, 0)], 0);
        assert_eq!(grid[Coord::new(0, 2)], 2);
        assert_eq!(grid[Coord::new(1, 0)], 3);
        assert_eq!(grid[Coord::new(1, 2)], 5);

        // Tupleでのアクセス
        assert_eq!(grid[(0, 1)], 1);
        assert_eq!(grid[(1, 1)], 4);

        // getメソッド
        assert_eq!(grid.get(Coord::new(0, 0)), Some(&0));
        assert_eq!(grid.get(Coord::new(2, 0)), None); // 範囲外
    }

    #[test]
    fn test_mutability() {
        let mut grid = setup_2x3();

        // IndexMut
        grid[Coord::new(0, 0)] = 10;
        grid[(1, 2)] = 50;

        assert_eq!(grid[(0, 0)], 10);
        assert_eq!(grid[(1, 2)], 50);

        // get_mut
        if let Some(val) = grid.get_mut(Coord::new(0, 1)) {
            *val = 100;
        }
        assert_eq!(grid[(0, 1)], 100);

        // fill
        grid.fill(999);
        for x in grid.iter() {
            assert_eq!(*x, 999);
        }
    }

    #[test]
    fn test_swap() {
        let mut grid = setup_2x3();
        // 0 1 2
        // 3 4 5

        // (0,0)=0 と (1,2)=5 を入れ替え
        // idx: 0 と 5
        grid.swap(Coord::new(0, 0), Coord::new(1, 2));

        assert_eq!(grid[(0, 0)], 5);
        assert_eq!(grid[(1, 2)], 0);
    }

    #[test]
    fn test_rotate_right() {
        let grid = setup_2x3();
        // Original:
        // 0 1 2
        // 3 4 5

        let rot1 = grid.rotate_right();
        // Expected (3x2):
        // 3 0
        // 4 1
        // 5 2
        assert_eq!(rot1.row, 3);
        assert_eq!(rot1.col, 2);
        assert_eq!(rot1[(0, 0)], 3);
        assert_eq!(rot1[(0, 1)], 0);
        assert_eq!(rot1[(2, 0)], 5);
        assert_eq!(rot1[(2, 1)], 2);

        let rot2 = rot1.rotate_right();
        // Expected (2x3) 180度回転:
        // 5 4 3
        // 2 1 0
        assert_eq!(rot2.row, 2);
        assert_eq!(rot2.col, 3);
        assert_eq!(rot2[(0, 0)], 5);
        assert_eq!(rot2[(1, 2)], 0);

        let rot4 = rot2.rotate_right().rotate_right();
        // 360度回転で元に戻るはず
        assert_eq!(rot4.vec, grid.vec);
        assert_eq!(rot4.row, grid.row);
        assert_eq!(rot4.col, grid.col);
    }

    #[test]
    fn test_rotate_left() {
        let grid = setup_2x3();
        // Original:
        // 0 1 2
        // 3 4 5

        let rot = grid.rotate_left();
        // Expected (3x2):
        // 2 5
        // 1 4
        // 0 3
        assert_eq!(rot.row, 3);
        assert_eq!(rot.col, 2);
        assert_eq!(rot[(0, 0)], 2);
        assert_eq!(rot[(0, 1)], 5);
        assert_eq!(rot[(2, 0)], 0);
        assert_eq!(rot[(2, 1)], 3);

        // 左に4回回せば元に戻る
        let rot4 = rot.rotate_left().rotate_left().rotate_left();
        assert_eq!(rot4.vec, grid.vec);
    }

    #[test]
    fn test_transpose() {
        let grid = setup_2x3();
        // Original:
        // 0 1 2
        // 3 4 5

        let trans = grid.transpose();
        // Expected (3x2):
        // 0 3
        // 1 4
        // 2 5
        assert_eq!(trans.row, 3);
        assert_eq!(trans.col, 2);
        assert_eq!(trans[(0, 0)], 0);
        assert_eq!(trans[(0, 1)], 3);
        assert_eq!(trans[(1, 0)], 1);
        assert_eq!(trans[(2, 1)], 5);

        // 転置の転置は元に戻る
        let trans_trans = trans.transpose();
        assert_eq!(trans_trans.vec, grid.vec);
        assert_eq!(trans_trans.row, grid.row);
        assert_eq!(trans_trans.col, grid.col);
    }

    #[test]
    fn test_find_and_count() {
        let vec = vec![1, 2, 3, 2, 1, 0];
        let grid = Grid::new(vec, 2, 3);
        // 1 2 3
        // 2 1 0

        // find (最初に見つかるもの)
        assert_eq!(grid.find(&2), Some(Coord::new(0, 1)));
        assert_eq!(grid.find(&9), None);

        // count
        assert_eq!(grid.count(&1), 2);
        assert_eq!(grid.count(&3), 1);
        assert_eq!(grid.count(&9), 0);
    }

    #[test]
    fn test_map() {
        let grid = setup_2x3();
        // 偶数をtrue, 奇数をfalseに変換
        let bool_grid = grid.map(|&x| x % 2 == 0);

        assert_eq!(bool_grid.row, 2);
        assert_eq!(bool_grid.col, 3);
        assert!(bool_grid[(0, 0)]); // 0
        assert!(!bool_grid[(0, 1)]); // 1
        assert!(bool_grid[(0, 2)]); // 2
    }

    #[test]
    fn test_conversions() {
        let vec2d = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
        let grid = Grid::from(vec2d.clone());

        assert_eq!(grid.row, 3);
        assert_eq!(grid.col, 2);
        assert_eq!(grid[(2, 1)], 6);

        let back_to_vec2d: Vec<Vec<i32>> = grid.into();
        assert_eq!(vec2d, back_to_vec2d);
    }

    #[test]
    fn test_conversions_empty_rows() {
        // 空のVec<Vec>からの変換
        let empty_vec: Vec<Vec<i32>> = vec![];
        let grid = Grid::from(empty_vec);
        assert_eq!(grid.row, 0);
        assert_eq!(grid.col, 0);
        assert_eq!(grid.vec.len(), 0);
    }

    #[test]
    #[should_panic(expected = "0行")]
    fn test_new_panic_row() {
        Grid::<i32>::new(vec![], 0, 5);
    }

    #[test]
    #[should_panic(expected = "0列")]
    fn test_new_panic_col() {
        Grid::<i32>::new(vec![], 5, 0);
    }

    #[test]
    #[should_panic(expected = "Grid index out of bounds")]
    fn test_index_out_of_bounds_panic() {
        let grid = setup_2x3();
        // 2行3列なので (2, 0) は範囲外
        let _ = grid[(2, 0)];
    }

    #[test]
    #[should_panic(expected = "Grid index out of bounds")]
    fn test_index_coord_out_of_bounds_panic() {
        let grid = setup_2x3();
        // (1, 3) は範囲外 (colは0,1,2)
        let _ = grid[Coord::new(1, 3)];
    }

    #[test]
    #[should_panic(expected = "Grid index out of bounds")]
    fn test_index_mut_out_of_bounds_panic() {
        let mut grid = setup_2x3();
        grid[(5, 5)] = 100;
    }

    #[test]
    fn test_visualization() {
        // パニックしないことだけを確認
        let grid = setup_2x3();
        let s = grid.visualize();
        assert!(!s.is_empty());

        let bool_grid = grid.map(|&x| x % 2 == 0);
        let s_bool = bool_grid.visualize_bool();
        assert!(s_bool.contains("#"));
        assert!(s_bool.contains("."));
    }

    #[test]
    fn test_edge_1x1() {
        let grid = Grid::new(vec![42], 1, 1);
        assert_eq!(grid.row, 1);
        assert_eq!(grid.col, 1);
        assert_eq!(grid[(0, 0)], 42);

        let rot = grid.rotate_right();
        assert_eq!(rot[(0, 0)], 42);

        let trans = grid.transpose();
        assert_eq!(trans[(0, 0)], 42);
    }

    #[test]
    fn test_large_grid_stress() {
        // 100x100 程度の少し大きめのグリッド操作
        let size = 100;
        let vec: Vec<usize> = (0..size * size).collect();
        let grid = Grid::new(vec, size, size);

        assert_eq!(grid[(0, 0)], 0);
        assert_eq!(grid[(size - 1, size - 1)], size * size - 1);

        // 転置してもデータが壊れていないか
        let trans = grid.transpose();
        assert_eq!(trans[(0, 1)], grid[(1, 0)]); // (r,c) -> (c,r)

        // 探索
        let target = size * 5 + 10; // 5行10列目の値想定
        let found = trans.find(&target);
        // Transposeしたので (10, 5) にあるはず
        assert_eq!(found, Some(Coord::new(10, 5)));
    }
}
