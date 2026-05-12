use super::util::*;

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

/// 0/1ナップサック（最小コスト版）。各アイテムは最大1個まで選び、価値の合計が `target` 以上になる最小コストを返す。
/// 達成不可能な場合は `None` を返す。
/// 時間: O(n * target)、空間: O(target)
pub fn dp_knapsack_01_min_cost(costs: &[usize], values: &[usize], target: usize) -> Option<usize> {
    let mut dp = vec![usize::MAX; target + 1];
    dp[0] = 0;
    for i in 0..costs.len() {
        for v in (0..target).rev() {
            if dp[v] == usize::MAX {
                continue;
            }
            let nv = (v + values[i]).min(target);
            dp[nv] = dp[nv].min(dp[v] + costs[i]);
        }
    }
    if dp[target] == usize::MAX {
        None
    } else {
        Some(dp[target])
    }
}

/// 非有界ナップサック（最小コスト版）。各アイテムは何個でも選び、価値の合計が `target` 以上になる最小コストを返す。
/// 達成不可能な場合は `None` を返す。
/// 時間: O(n * target)、空間: O(target)
pub fn dp_knapsack_unbounded_min_cost(
    costs: &[usize],
    values: &[usize],
    target: usize,
) -> Option<usize> {
    let mut dp = vec![usize::MAX; target + 1];
    dp[0] = 0;
    for v in 0..target {
        if dp[v] == usize::MAX {
            continue;
        }
        for i in 0..costs.len() {
            let nv = (v + values[i]).min(target);
            dp[nv] = dp[nv].min(dp[v] + costs[i]);
        }
    }
    if dp[target] == usize::MAX {
        None
    } else {
        Some(dp[target])
    }
}

/// 部分和判定。`items` の中からいくつか選んで合計がちょうど `target` になるか判定する。
/// 時間: O(n * target)、空間: O(target)
pub fn dp_subset_sum(items: &[usize], target: usize) -> bool {
    let mut dp = vec![false; target + 1];
    dp[0] = true;
    for &item in items {
        for w in (item..=target).rev() {
            dp[w] |= dp[w - item];
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

/// コイン問題。`coins` の額面を何枚でも使って合計 `amount` にする組み合わせ数を `modulo` で割った余りで返す。
/// 時間: O(|coins| * amount)、空間: O(amount)
pub fn dp_coin_count(coins: &[usize], amount: usize, modulo: usize) -> usize {
    let mut dp = vec![0usize; amount + 1];
    dp[0] = 1;
    for &c in coins {
        for a in c..=amount {
            dp[a] = (dp[a] + dp[a - c]) % modulo;
        }
    }
    dp[amount]
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
        if pos == tails.len() {
            tails.push(x);
        } else {
            tails[pos] = x;
        }
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
    for (i, row) in dp.iter_mut().enumerate() {
        row[0] = i;
    }
    for (j, val) in dp[0].iter_mut().enumerate() {
        *val = j;
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

/// グリッドの左上(0,0)から右下(h-1,w-1)へ、右か下にのみ移動したときの最小スコアを返す。
/// `grid` は h×w の行列。
/// 時間: O(h * w)、空間: O(h * w)
pub fn dp_grid_min_path(grid: &[Vec<isize>]) -> isize {
    let h = grid.len();
    let w = grid[0].len();
    let mut dp = vec![vec![isize::MAX; w]; h];
    dp[0][0] = grid[0][0];
    for j in 1..w {
        dp[0][j] = dp[0][j - 1] + grid[0][j];
    }
    for i in 1..h {
        dp[i][0] = dp[i - 1][0] + grid[i][0];
    }
    for i in 1..h {
        for j in 1..w {
            dp[i][j] = dp[i - 1][j].min(dp[i][j - 1]) + grid[i][j];
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
    let blocked: Set<(usize, usize)> = obstacles.iter().cloned().collect();
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
    for s in 1..1usize << n {
        for (v, row) in dist.iter().enumerate() {
            if dp[s][v] == inf || s >> v & 1 == 0 {
                continue;
            }
            for (u, &cost) in row.iter().enumerate() {
                if s >> u & 1 == 1 {
                    continue;
                }
                let ns = s | 1 << u;
                let nc = dp[s][v] + cost;
                if nc < dp[ns][u] {
                    dp[ns][u] = nc;
                }
            }
        }
    }
    let full = (1 << n) - 1;
    if n == 1 {
        return Some(0);
    }
    let ans = (1..n)
        .map(|v| dp[full][v] + dist[v][0])
        .min()
        .unwrap_or(inf);
    if ans >= inf { None } else { Some(ans) }
}
