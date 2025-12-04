// 二次元を一次元で管理
use {
    super::{config::*, coord::*},
    crate::log,
    core::panic,
    std::{fmt::*, ops::*, slice::*},
};
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct VecSeq<T> {
    vec: Vec<T>,
    row: usize,
    col: usize,
}
impl<T> VecSeq<T> {
    pub fn new(vec: Vec<T>, row: usize, col: usize) -> Self {
        if row == 0 {
            panic!("0行")
        } else if col == 0 {
            panic!("0列")
        }
        Self { vec, row, col }
    }

    #[inline(always)]
    pub fn get(&self, coord: Coord) -> Option<&T> {
        if coord.within(self.row, self.col) {
            Some(&self.vec[coord.to_index(self.col)])
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn get_mut(&mut self, coord: Coord) -> Option<&mut T> {
        if coord.within(self.row, self.col) {
            Some(&mut self.vec[coord.to_index(self.col)])
        } else {
            None
        }
    }

    #[inline(always)]
    pub const fn is_valid(&self, coord: Coord) -> bool {
        coord.within(self.row, self.col)
    }

    pub fn iter_coords(&self) -> impl Iterator<Item = Coord> + '_ {
        (0..self.row).flat_map(move |r| (0..self.col).map(move |c| Coord::new(r, c)))
    }

    pub fn swap(&mut self, i: usize, j: usize) {
        self.vec.swap(i, j);
    }

    #[inline(always)]
    pub const fn size(&self) -> usize {
        self.vec.len()
    }

    #[inline(always)]
    pub fn iter(&self) -> Iter<'_, T> {
        self.vec.iter()
    }

    #[inline(always)]
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        self.vec.iter_mut()
    }

    #[inline(always)]
    pub fn as_slice(&self) -> &[T] {
        &self.vec
    }

    #[inline(always)]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.vec
    }

    #[inline(always)]
    pub const fn row(&self) -> usize {
        self.row
    }

    #[inline(always)]
    pub const fn col(&self) -> usize {
        self.col
    }

    #[inline(always)]
    pub const fn get_idx(&self, row: usize, col: usize) -> usize {
        row * self.col + col
    }
}

impl<T: Clone> From<Vec<Vec<T>>> for VecSeq<T> {
    fn from(value: Vec<Vec<T>>) -> Self {
        let row = value.len();
        let col = if row == 0 { 0 } else { value[0].len() };
        let vec = value.into_iter().flatten().collect();
        Self { vec, row, col }
    }
}

impl<T: Clone> From<VecSeq<T>> for Vec<Vec<T>> {
    fn from(val: VecSeq<T>) -> Self {
        if val.col == 0 {
            vec![Vec::new(); val.row]
        } else {
            val.vec.chunks(val.col).map(|ch| ch.to_vec()).collect()
        }
    }
}

impl<T: Clone> VecSeq<T> {
    pub fn fill(&mut self, value: T) {
        self.vec.fill(value);
    }
}

impl<T: Default + Clone> VecSeq<T> {
    pub fn new_filled(row: usize, col: usize, val: T) -> Self {
        Self {
            vec: vec![val; row * col],
            row,
            col,
        }
    }

    pub fn new_default(row: usize, col: usize) -> Self {
        Self {
            vec: vec![T::default(); row * col],
            row,
            col,
        }
    }
}

impl<T> Index<usize> for VecSeq<T> {
    type Output = T;
    #[inline(always)]
    fn index(&self, idx: usize) -> &Self::Output {
        &self.vec[idx]
    }
}

impl<T> IndexMut<usize> for VecSeq<T> {
    #[inline(always)]
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.vec[idx]
    }
}

impl<T> Index<Coord> for VecSeq<T> {
    type Output = T;
    #[inline]
    fn index(&self, coord: Coord) -> &Self::Output {
        &self.vec[coord.to_index(self.col)]
    }
}

impl<T> IndexMut<Coord> for VecSeq<T> {
    #[inline]
    fn index_mut(&mut self, coord: Coord) -> &mut Self::Output {
        &mut self.vec[coord.to_index(self.col)]
    }
}

impl<T> Index<&Coord> for VecSeq<T> {
    type Output = T;
    #[inline]
    fn index(&self, coord: &Coord) -> &Self::Output {
        &self.vec[coord.to_index(self.col)]
    }
}

impl<T> IndexMut<&Coord> for VecSeq<T> {
    #[inline]
    fn index_mut(&mut self, coord: &Coord) -> &mut Self::Output {
        &mut self.vec[coord.to_index(self.col)]
    }
}

impl<T> Index<(usize, usize)> for VecSeq<T> {
    type Output = T;
    #[inline]
    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.vec[row * self.col + col]
    }
}
impl<T> IndexMut<(usize, usize)> for VecSeq<T> {
    #[inline]
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.vec[row * self.col + col]
    }
}

/// 視覚化
impl<T: Display> VecSeq<T> {
    pub fn visualize(&self) -> String {
        if self.vec.is_empty() {
            // CHANGED: eprintln! -> log!
            log!("[Empty VecSeq {}x{}]", self.row, self.col);
            return "[]\n".to_string();
        }

        let data_max_width = self
            .vec
            .iter()
            .map(|val| format!("{}", val).len())
            .max()
            .unwrap_or(1);

        let row_header_width = if self.row == 0 {
            0
        } else {
            (self.row - 1).to_string().len()
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
                result.push_str(&format!(
                    "{val:>width$} ",
                    val = self[(r, c)],
                    width = data_max_width
                ));
            }
            result.push('\n');
        }
        log!("{result}");
        result
    }
}
impl<T> VecSeq<T> {
    pub fn visualize_with<F>(&self, mut formatter: F) -> String
    where
        F: FnMut(&T) -> String,
    {
        if self.vec.is_empty() {
            log!("[Empty VecSeq {}x{}]", self.row, self.col);
            return "[]\n".to_string();
        }

        let formatted_cells: Vec<String> = self.vec.iter().map(&mut formatter).collect();
        let data_max_width = formatted_cells.iter().map(String::len).max().unwrap_or(1);
        let row_header_width = if self.row == 0 {
            0
        } else {
            (self.row - 1).to_string().len()
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
                let idx = self.get_idx(r, c);
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
impl VecSeq<bool> {
    pub fn visualize_bool(&self) -> String {
        self.visualize_with(|&val| if val { "#" } else { "." }.to_string())
    }
}
