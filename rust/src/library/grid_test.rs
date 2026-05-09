use crate::library::coord::*;
use crate::library::grid::*;

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
#[should_panic(expected = "Grid requires at least 1 row")]
fn test_conversions_empty_rows() {
    // 空のVec<Vec>からの変換は禁止 (0x0 Grid は仕様上不可)
    let empty_vec: Vec<Vec<i32>> = vec![];
    let _ = Grid::from(empty_vec);
}

#[test]
#[should_panic(expected = "Grid(0x5, len=0)")]
fn test_new_panic_row() {
    Grid::<i32>::new(vec![], 0, 5);
}

#[test]
#[should_panic(expected = "Grid(5x0, len=0)")]
fn test_new_panic_col() {
    Grid::<i32>::new(vec![], 5, 0);
}

#[test]
#[should_panic(expected = "Grid out of bounds")]
fn test_index_out_of_bounds_panic() {
    let grid = setup_2x3();
    // 2行3列なので (2, 0) は範囲外
    let _ = grid[(2, 0)];
}

#[test]
#[should_panic(expected = "Grid out of bounds")]
fn test_index_coord_out_of_bounds_panic() {
    let grid = setup_2x3();
    // (1, 3) は範囲外 (colは0,1,2)
    let _ = grid[Coord::new(1, 3)];
}

#[test]
#[should_panic(expected = "Grid out of bounds")]
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
