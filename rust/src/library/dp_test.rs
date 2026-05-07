use crate::library::dp::*;

// --- dp_knapsack_01 ---
#[test]
fn knapsack_01_basic() {
    assert_eq!(dp_knapsack_01(&[1, 2, 3, 5], &[1, 6, 5, 9], 5), 11);
}
#[test]
fn knapsack_01_empty() {
    assert_eq!(dp_knapsack_01(&[], &[], 10), 0);
}
#[test]
fn knapsack_01_capacity_zero() {
    assert_eq!(dp_knapsack_01(&[1, 2], &[10, 20], 0), 0);
}
#[test]
fn knapsack_01_all_too_heavy() {
    assert_eq!(dp_knapsack_01(&[5, 6], &[100, 200], 4), 0);
}
#[test]
fn knapsack_01_single_item_fits() {
    assert_eq!(dp_knapsack_01(&[3], &[99], 3), 99);
}
#[test]
fn knapsack_01_each_item_once() {
    // w=3,v=10 を2個: 容量5では1個しか入らない -> 10
    assert_eq!(dp_knapsack_01(&[3, 3], &[10, 10], 5), 10);
}

// --- dp_knapsack_unbounded ---
#[test]
fn knapsack_unbounded_basic() {
    // w=2,v=3 を繰り返す: 容量6 -> 3個 -> 9
    assert_eq!(dp_knapsack_unbounded(&[2], &[3], 6), 9);
}
#[test]
fn knapsack_unbounded_choose_best_ratio() {
    // (w=3,v=4) vs (w=2,v=3): 容量6では(w=2,v=3)x3=9 > (w=3,v=4)x2=8
    assert_eq!(dp_knapsack_unbounded(&[3, 2], &[4, 3], 6), 9);
}
#[test]
fn knapsack_unbounded_capacity_zero() {
    assert_eq!(dp_knapsack_unbounded(&[1], &[10], 0), 0);
}
#[test]
fn knapsack_unbounded_empty() {
    assert_eq!(dp_knapsack_unbounded(&[], &[], 5), 0);
}

// --- dp_knapsack_bounded ---
#[test]
fn knapsack_bounded_basic() {
    // w=2,v=3 を最大2個: 容量6 -> 2個=6 (unboundedなら3個=9だが上限2個)
    assert_eq!(dp_knapsack_bounded(&[2], &[3], &[2], 6), 6);
}
#[test]
fn knapsack_bounded_vs_01() {
    // count=1のとき0/1ナップサックと同じ結果
    let w = &[1, 2, 3];
    let v = &[2, 3, 4];
    let cap = 5;
    assert_eq!(
        dp_knapsack_bounded(w, v, &[1, 1, 1], cap),
        dp_knapsack_01(w, v, cap)
    );
}
#[test]
fn knapsack_bounded_empty() {
    assert_eq!(dp_knapsack_bounded(&[], &[], &[], 10), 0);
}
#[test]
fn knapsack_bounded_count_zero() {
    assert_eq!(dp_knapsack_bounded(&[1], &[100], &[0], 5), 0);
}

// --- dp_subset_sum ---
#[test]
fn subset_sum_basic_true() {
    assert!(dp_subset_sum(&[1, 2, 3, 5], 5));
}
#[test]
fn subset_sum_basic_false() {
    assert!(!dp_subset_sum(&[2, 4, 6], 5));
}
#[test]
fn subset_sum_target_zero() {
    assert!(dp_subset_sum(&[1, 2, 3], 0));
}
#[test]
fn subset_sum_empty_target_zero() {
    assert!(dp_subset_sum(&[], 0));
}
#[test]
fn subset_sum_each_used_once() {
    assert!(dp_subset_sum(&[3, 3], 6));
    assert!(!dp_subset_sum(&[3], 6));
}

// --- dp_coin_min ---
#[test]
fn coin_min_basic() {
    assert_eq!(dp_coin_min(&[1, 5, 10, 25], 41), Some(4)); // 25+10+5+1
}
#[test]
fn coin_min_impossible() {
    assert_eq!(dp_coin_min(&[3, 5], 1), None);
}
#[test]
fn coin_min_zero() {
    assert_eq!(dp_coin_min(&[1, 5], 0), Some(0));
}
#[test]
fn coin_min_exact() {
    assert_eq!(dp_coin_min(&[7], 21), Some(3));
}
#[test]
fn coin_min_greedy_fails() {
    // 貪欲法では6=4+1+1(3枚)だが最適は6=3+3(2枚)
    assert_eq!(dp_coin_min(&[1, 3, 4], 6), Some(2));
}

// --- dp_lis_len ---
#[test]
fn lis_len_basic() {
    assert_eq!(dp_lis_len(&[3, 1, 4, 1, 5, 9, 2, 6], true), 4);
}
#[test]
fn lis_len_all_same() {
    assert_eq!(dp_lis_len(&[5, 5, 5, 5], true), 1);
}
#[test]
fn lis_len_descending() {
    assert_eq!(dp_lis_len(&[5, 4, 3, 2, 1], true), 1);
}
#[test]
fn lis_len_ascending() {
    assert_eq!(dp_lis_len(&[1, 2, 3, 4, 5], true), 5);
}
#[test]
fn lis_len_empty() {
    assert_eq!(dp_lis_len(&[], true), 0);
}
#[test]
fn lis_len_single() {
    assert_eq!(dp_lis_len(&[42], true), 1);
}
#[test]
fn lis_len_nonstrict_all_same() {
    // 広義: 同値を含む -> 4
    assert_eq!(dp_lis_len(&[3, 3, 3, 3], false), 4);
}
#[test]
fn lis_len_strict_vs_nonstrict() {
    let seq = &[1, 2, 2, 3];
    assert_eq!(dp_lis_len(seq, true), 3);
    assert_eq!(dp_lis_len(seq, false), 4);
}
#[test]
fn lis_len_nonstrict_empty() {
    assert_eq!(dp_lis_len(&[], false), 0);
}

// --- dp_lis_seq ---
#[test]
fn lis_seq_basic() {
    let result = dp_lis_seq(&[3, 1, 4, 1, 5, 9, 2, 6]);
    assert_eq!(result.len(), 4);
    assert!(result.windows(2).all(|w| w[0] < w[1]));
}
#[test]
fn lis_seq_empty() {
    assert_eq!(dp_lis_seq(&[]), vec![]);
}
#[test]
fn lis_seq_single() {
    assert_eq!(dp_lis_seq(&[7]), vec![7]);
}
#[test]
fn lis_seq_all_same() {
    assert_eq!(dp_lis_seq(&[3, 3, 3]).len(), 1);
}
#[test]
fn lis_seq_len_matches_lis_len() {
    let seq = &[10, 9, 2, 5, 3, 7, 101, 18];
    assert_eq!(dp_lis_seq(seq).len(), dp_lis_len(seq, true));
}

// --- dp_lcs_len ---
#[test]
fn lcs_basic() {
    assert_eq!(dp_lcs_len(&[1, 2, 3, 4, 5], &[2, 4, 5]), 3);
}
#[test]
fn lcs_no_common() {
    assert_eq!(dp_lcs_len(&[1, 2], &[3, 4]), 0);
}
#[test]
fn lcs_empty() {
    assert_eq!(dp_lcs_len(&[], &[1, 2, 3]), 0);
    assert_eq!(dp_lcs_len(&[1, 2, 3], &[]), 0);
}
#[test]
fn lcs_identical() {
    assert_eq!(dp_lcs_len(&[1, 2, 3], &[1, 2, 3]), 3);
}
#[test]
fn lcs_duplicates() {
    assert_eq!(dp_lcs_len(&[1, 1, 2], &[1, 2]), 2);
}

// --- dp_edit_distance ---
#[test]
fn edit_distance_basic() {
    // "kitten" -> "sitting": 3
    let s: Vec<isize> = "kitten".bytes().map(|b| b as isize).collect();
    let t: Vec<isize> = "sitting".bytes().map(|b| b as isize).collect();
    assert_eq!(dp_edit_distance(&s, &t), 3);
}
#[test]
fn edit_distance_empty() {
    assert_eq!(dp_edit_distance(&[], &[1, 2, 3]), 3);
    assert_eq!(dp_edit_distance(&[1, 2, 3], &[]), 3);
}
#[test]
fn edit_distance_identical() {
    assert_eq!(dp_edit_distance(&[1, 2, 3], &[1, 2, 3]), 0);
}
#[test]
fn edit_distance_single() {
    assert_eq!(dp_edit_distance(&[1], &[2]), 1);
    assert_eq!(dp_edit_distance(&[], &[1]), 1);
}

// --- dp_grid_max_path ---
#[test]
fn grid_max_path_basic() {
    let grid = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
    assert_eq!(dp_grid_max_path(&grid), 12); // 1->3->5->2->1
}
#[test]
fn grid_max_path_1x1() {
    assert_eq!(dp_grid_max_path(&[vec![42]]), 42);
}
#[test]
fn grid_max_path_negative() {
    let grid = vec![vec![-1, -2], vec![-3, -4]];
    assert_eq!(dp_grid_max_path(&grid), -7);
}
#[test]
fn grid_max_path_1xn() {
    assert_eq!(dp_grid_max_path(&[vec![1, 2, 3, 4]]), 10);
}

// --- dp_grid_path_count ---
#[test]
fn grid_path_count_basic() {
    // 3x3, 障害物なし -> C(4,2) = 6
    assert_eq!(dp_grid_path_count(3, 3, &[], 1_000_000_007), 6);
}
#[test]
fn grid_path_count_1x1() {
    assert_eq!(dp_grid_path_count(1, 1, &[], 1_000_000_007), 1);
}
#[test]
fn grid_path_count_obstacle_blocks_all() {
    assert_eq!(dp_grid_path_count(2, 2, &[(1, 1)], 1_000_000_007), 0);
}
#[test]
fn grid_path_count_obstacle_start() {
    assert_eq!(dp_grid_path_count(3, 3, &[(0, 0)], 1_000_000_007), 0);
}
#[test]
fn grid_path_count_modulo() {
    // C(18,9) = 48620, 48620 % 7 = 0
    assert_eq!(dp_grid_path_count(10, 10, &[], 7), 48620 % 7);
}
#[test]
fn grid_path_count_obstacle_in_middle() {
    // 3x3で中央(1,1)を塞ぐ -> 2
    assert_eq!(dp_grid_path_count(3, 3, &[(1, 1)], 1_000_000_007), 2);
}

// --- dp_interval ---
#[test]
fn interval_matrix_chain() {
    // 行列積順序: dims=[10,30,5,60] -> 最小コスト=4500
    let dims = [10usize, 30, 5, 60];
    let result = dp_interval(3, |l, mid, r| {
        (dims[l] * dims[mid + 1] * dims[r + 1]) as isize
    });
    assert_eq!(result, 4500);
}
#[test]
fn interval_single() {
    assert_eq!(dp_interval(1, |_, _, _| 0), 0);
}
#[test]
fn interval_two() {
    assert_eq!(dp_interval(2, |_, _, _| 10), 10);
}

// --- dp_tsp ---
#[test]
fn tsp_basic() {
    let dist = vec![
        vec![0, 10, 15, 20],
        vec![10, 0, 35, 25],
        vec![15, 35, 0, 30],
        vec![20, 25, 30, 0],
    ];
    assert_eq!(dp_tsp(&dist), Some(80)); // 0->1->3->2->0
}
#[test]
fn tsp_single() {
    assert_eq!(dp_tsp(&[vec![0]]), Some(0));
}
#[test]
fn tsp_two() {
    assert_eq!(dp_tsp(&[vec![0, 5], vec![5, 0]]), Some(10));
}
