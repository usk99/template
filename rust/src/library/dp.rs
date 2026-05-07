// ナップサック系

/// 0/1ナップサック問題を解く。各アイテムは最大1個まで選べる。
/// `capacity` 以下の重さで価値の合計を最大化した値を返す。
/// 時間: O(n * capacity)、空間: O(capacity)
pub fn dp_knapsack_01(weights: &[usize], values: &[isize], capacity: usize) -> isize {
    let n = weights.len();
    let mut dp = vec![0isize; capacity + 1];
    for i in 0..n {
        for w in (weights[i]..=capacity).rev() {
            dp[w] = dp[w].max(dp[w - weights[i]] + values[i]);
        }
    }
    dp[capacity]
}

/// 非有界ナップサック問題を解く。各アイテムは何個でも選べる。
/// `capacity` 以下の重さで価値の合計を最大化した値を返す。
/// 時間: O(n * capacity)、空間: O(capacity)
pub fn dp_knapsack_unbounded(weights: &[usize], values: &[isize], capacity: usize) -> isize {
    let n = values.len();
    let mut dp = vec![0isize; capacity + 1];
    for w in 1..=capacity {
        for i in 0..n {
            if w >= weights[i] {
                dp[w] = dp[w].max(dp[w - weights[i]] + values[i]);
            }
        }
    }
    dp[capacity]
}

/// 有界ナップサック問題を解く。各アイテムは最大 `counts[i]` 個まで選べる。
/// `capacity` 以下の重さで価値の合計を最大化した値を返す。
/// 時間: O(n * capacity * max_count)、空間: O(capacity)
pub fn dp_knapsack_bounded(
    weights: &[usize],
    values: &[isize],
    counts: &[usize],
    capacity: usize,
) -> isize {
    let n = weights.len();
    let mut dp = vec![0isize; capacity + 1];
    for i in 0..n {
        for w in (weights[i]..=capacity).rev() {
            for k in 1..=counts[i] {
                if k * weights[i] > w {
                    break;
                }
                dp[w] = dp[w].max(dp[w - k * weights[i]] + k as isize * values[i]);
            }
        }
    }
    dp[capacity]
}

/// 部分和判定。`items` の中からいくつか選んで合計がちょうど `target` になるか判定する。
/// 時間: O(n * target)、空間: O(target)
pub fn dp_subset_sum(items: &[usize], target: usize) -> bool {
    let mut dp = vec![false; target + 1];
    dp[0] = true;
    for &item in items {
        for w in (item..=target).rev() {
            if dp[w - item] {
                dp[w] = true;
            }
        }
    }
    dp[target]
}

/// コイン問題。`coins` の額面を何枚でも使って合計 `amount` にする最小枚数を返す。
/// 達成不可能な場合は `None` を返す。
/// 時間: O(|coins| * amount)、空間: O(amount)
pub fn dp_coin_min(coins: &[usize], amount: usize) -> Option<usize> {
    let mut dp = vec![usize::MAX; amount + 1];
    dp[0] = 0;
    for a in 1..=amount {
        for &c in coins {
            if c <= a && dp[a - c] != usize::MAX {
                dp[a] = dp[a].min(dp[a - c] + 1);
            }
        }
    }
    if dp[amount] == usize::MAX {
        None
    } else {
        Some(dp[amount])
    }
}

// 列系

/// LIS（最長増加部分列）の長さを返す。
/// `strict = true` で狭義単調増加（重複を含まない）、`false` で広義単調増加（重複を含む）。
/// 時間: O(n log n)、空間: O(n)
pub fn dp_lis_len(seq: &[isize], strict: bool) -> usize {
    let mut tails: Vec<isize> = Vec::new();
    for &x in seq {
        let pos = if strict {
            tails.partition_point(|&t| t < x)
        } else {
            tails.partition_point(|&t| t <= x)
        };
        if pos == tails.len() { tails.push(x); } else { tails[pos] = x; }
    }
    tails.len()
}

/// LIS（最長増加部分列）の実際の列を返す。狭義単調増加。
/// 複数存在する場合は辞書順最小のものを返す。
/// 時間: O(n^2)、空間: O(n)  ※復元のprev探索がO(n)のため
pub fn dp_lis_seq(seq: &[isize]) -> Vec<isize> {
    let n = seq.len();
    if n == 0 {
        return vec![];
    }
    let mut tails: Vec<isize> = Vec::new();
    let mut pos = vec![0usize; n];
    let mut prev = vec![usize::MAX; n];
    for i in 0..n {
        let x = seq[i];
        let p = tails.partition_point(|&t| t < x);
        if p == tails.len() {
            tails.push(x);
        } else {
            tails[p] = x;
        }
        pos[i] = p;
        if p > 0 {
            prev[i] = (0..i)
                .rev()
                .find(|&j| pos[j] == p - 1 && seq[j] < x)
                .unwrap_or(usize::MAX);
        }
    }
    let lis_len = tails.len();
    let mut cur = (0..n).rev().find(|&i| pos[i] == lis_len - 1).unwrap();
    let mut result = Vec::with_capacity(lis_len);
    loop {
        result.push(seq[cur]);
        if prev[cur] == usize::MAX {
            break;
        }
        cur = prev[cur];
    }
    result.reverse();
    result
}

// 文字列系

/// LCS（最長共通部分列）の長さを返す。
/// 時間: O(|s| * |t|)、空間: O(|s| * |t|)
pub fn dp_lcs_len(s: &[isize], t: &[isize]) -> usize {
    let (n, m) = (s.len(), t.len());
    let mut dp = vec![vec![0usize; m + 1]; n + 1];
    for i in 1..=n {
        for j in 1..=m {
            dp[i][j] = if s[i - 1] == t[j - 1] {
                dp[i - 1][j - 1] + 1
            } else {
                dp[i - 1][j].max(dp[i][j - 1])
            };
        }
    }
    dp[n][m]
}

/// 編集距離（Levenshtein距離）を返す。挿入・削除・置換の最小操作数。
/// 時間: O(|s| * |t|)、空間: O(|s| * |t|)
pub fn dp_edit_distance(s: &[isize], t: &[isize]) -> usize {
    let (n, m) = (s.len(), t.len());
    let mut dp = vec![vec![0usize; m + 1]; n + 1];
    for (i, row) in dp.iter_mut().enumerate().take(n + 1) {
        row[0] = i;
    }
    for (j, cell) in dp[0].iter_mut().enumerate().take(m + 1) {
        *cell = j;
    }
    for i in 1..=n {
        for j in 1..=m {
            dp[i][j] = if s[i - 1] == t[j - 1] {
                dp[i - 1][j - 1]
            } else {
                1 + dp[i - 1][j].min(dp[i][j - 1]).min(dp[i - 1][j - 1])
            };
        }
    }
    dp[n][m]
}

// グリッド系

/// グリッドの左上(0,0)から右下(h-1,w-1)へ、右か下にのみ移動したときの最大スコアを返す。
/// `grid` は h×w の行列。
/// 時間: O(h * w)、空間: O(h * w)
pub fn dp_grid_max_path(grid: &[Vec<isize>]) -> isize {
    let h = grid.len();
    let w = grid[0].len();
    let mut dp = vec![vec![isize::MIN; w]; h];
    dp[0][0] = grid[0][0];
    for j in 1..w {
        dp[0][j] = dp[0][j - 1] + grid[0][j];
    }
    for i in 1..h {
        dp[i][0] = dp[i - 1][0] + grid[i][0];
    }
    for i in 1..h {
        for j in 1..w {
            dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]) + grid[i][j];
        }
    }
    dp[h - 1][w - 1]
}

/// グリッドの左上(0,0)から右下(h-1,w-1)へ、右か下にのみ移動する経路数を `modulo` で割った余りで返す。
/// `obstacles` は通過できないセル (row, col) のリスト。
/// 時間: O(h * w + |obstacles|)、空間: O(h * w + |obstacles|)
pub fn dp_grid_path_count(
    h: usize,
    w: usize,
    obstacles: &[(usize, usize)],
    modulo: usize,
) -> usize {
    let blocked: std::collections::HashSet<(usize, usize)> = obstacles.iter().cloned().collect();
    let mut dp = vec![vec![0usize; w]; h];
    if blocked.contains(&(0, 0)) {
        return 0;
    }
    dp[0][0] = 1;
    for j in 1..w {
        if !blocked.contains(&(0, j)) {
            dp[0][j] = dp[0][j - 1];
        }
    }
    for i in 1..h {
        if !blocked.contains(&(i, 0)) {
            dp[i][0] = dp[i - 1][0];
        }
    }
    for i in 1..h {
        for j in 1..w {
            if !blocked.contains(&(i, j)) {
                dp[i][j] = (dp[i - 1][j] + dp[i][j - 1]) % modulo;
            }
        }
    }
    dp[h - 1][w - 1]
}

// その他

/// 区間DP。`cost(l, mid, r)` を区間 `[l, r]` を分割点 `mid` で結合するコストとして、全体の最小コストを返す。
/// `n` 要素に対して区間 `[0, n-1]` の最小コストを求める。
/// 時間: O(n^3)、空間: O(n^2)
pub fn dp_interval<F>(n: usize, cost: F) -> isize
where
    F: Fn(usize, usize, usize) -> isize,
{
    let mut dp = vec![vec![isize::MAX / 2; n]; n];
    for (i, row) in dp.iter_mut().enumerate() {
        row[i] = 0;
    }
    for len in 2..=n {
        for l in 0..=n - len {
            let r = l + len - 1;
            for mid in l..r {
                let c = dp[l][mid] + dp[mid + 1][r] + cost(l, mid, r);
                if c < dp[l][r] {
                    dp[l][r] = c;
                }
            }
        }
    }
    dp[0][n - 1]
}

/// 巡回セールスマン問題（TSP）。全頂点を1度ずつ訪問して戻る最小コストを返す。
/// `dist[i][j]` は頂点 `i` から `j` へのコスト。頂点0を始点・終点とする。
/// 到達不可能な場合は `None` を返す。
/// 時間: O(2^n * n^2)、空間: O(2^n * n)
pub fn dp_tsp(dist: &[Vec<isize>]) -> Option<isize> {
    let n = dist.len();
    let inf = isize::MAX / 2;
    let mut dp = vec![vec![inf; n]; 1 << n];
    dp[1][0] = 0;
    #[allow(clippy::needless_range_loop)]
    for s in 1..1usize << n {
        for v in 0..n {
            if dp[s][v] == inf || s >> v & 1 == 0 {
                continue;
            }
            for u in 0..n {
                if s >> u & 1 == 1 {
                    continue;
                }
                let ns = s | 1 << u;
                let nc = dp[s][v] + dist[v][u];
                if nc < dp[ns][u] {
                    dp[ns][u] = nc;
                }
            }
        }
    }
    let full = (1 << n) - 1;
    let ans = (0..n)
        .filter(|&v| dp[full][v] < inf)
        .map(|v| dp[full][v] + dist[v][0])
        .min()
        .unwrap_or(inf);
    if ans >= inf { None } else { Some(ans) }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
