use std::{cmp::Reverse, collections::BinaryHeap};

// =====================================================
// 定数定義
// =====================================================
pub static INF: i64 = 1_000_000_000_000_000_000; // オーバーフローしないでかい値
pub static NONE: usize = usize::MAX;

// =====================================================
// 定数定義
// =====================================================
#[derive(Debug, Clone, Copy)]
pub struct Edge {
    pub to: usize,
    pub weight: i64,
}

// =====================================================
// グラフ構造体
// =====================================================
#[derive(Debug, Clone)]
pub struct Graph {
    pub n: usize,
    pub edges: Vec<Vec<Edge>>,
    pub is_directed: bool,
}
impl Graph {
    /// `n`: 頂点数, `is_directed`: 有向ならtrue,無向ならfalse
    pub fn new(n: usize, is_directed: bool) -> Self {
        Self {
            n,
            edges: vec![Vec::new(); n],
            is_directed,
        }
    }

    /// 辺の追加(重み有)
    pub fn add_edge(&mut self, from: usize, to: usize, weight: i64) {
        self.edges[from].push(Edge { to, weight });
        if !self.is_directed {
            self.edges[to].push(Edge { to: from, weight });
        }
    }

    /// 辺の追加(重み無)
    pub fn add_unweighted_edge(&mut self, from: usize, to: usize) {
        self.add_edge(from, to, 1);
    }
}

// =====================================================
// アルゴリズム(最短系)
// =====================================================
impl Graph {
    /// ダイクストラ法 : O(E log V)
    /// 有向無向どちらも可,負の重みは不可
    pub fn dijkstra(&self, start: usize) -> (Vec<i64>, Vec<usize>) {
        let mut dist = vec![INF; self.n];
        let mut pre = vec![NONE; self.n];
        let mut heap: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();

        dist[start] = 0;
        heap.push(Reverse((0, start)));

        while let Some(Reverse((d, u))) = heap.pop() {
            if d > dist[u] {
                continue;
            }
            for edge in &self.edges[u] {
                let next_dist = d + edge.weight;
                if next_dist < dist[edge.to] {
                    dist[edge.to] = next_dist;
                    pre[edge.to] = u;
                    heap.push(Reverse((next_dist, edge.to)));
                }
            }
        }
        (dist, pre)
    }

    /// ベルマンフォード法:O(V*E)
    /// 負の重み対応。負の閉路あればNone
    pub fn bellman_ford(&self, start: usize) -> Option<(Vec<i64>, Vec<usize>)> {
        let mut dist = vec![INF; self.n];
        let mut pre = vec![NONE; self.n];
        dist[start] = 0;
        // V-1回の緩和
        for _ in 0..self.n.saturating_sub(1) {
            let mut updated = false;
            for u in 0..self.n {
                if dist[u] == INF {
                    continue;
                }
                for edge in &self.edges[u] {
                    let next_dist = dist[u] + edge.weight;
                    if next_dist < dist[edge.to] {
                        dist[edge.to] = next_dist;
                        pre[edge.to] = u;
                        updated = true;
                    }
                }
            }
            if !updated {
                break;
            }
        }
        // V回目の緩和
        for u in 0..self.n {
            if dist[u] == INF {
                continue;
            }
            for edge in &self.edges[u] {
                if dist[u] + edge.weight < dist[edge.to] {
                    return None;
                }
            }
        }
        Some((dist, pre))
    }

    /// ワーシャル・フロイド法 : O(V^3)
    /// 全点最短経路。頂点数V<=500が目安
    pub fn warshall_floyd(&self) -> Option<Vec<Vec<i64>>> {
        let mut dist = vec![vec![INF; self.n]; self.n];
        for (i, item) in dist.iter_mut().enumerate().take(self.n) {
            item[i] = 0;
        }
        for (u, edges) in self.edges.iter().enumerate() {
            for edge in edges {
                dist[u][edge.to] = dist[u][edge.to].min(edge.weight);
            }
        }
        for k in 0..self.n {
            for i in 0..self.n {
                if dist[i][k] == INF {
                    continue;
                }
                for j in 0..self.n {
                    if dist[k][j] != INF {
                        let new_dist = dist[i][k] + dist[k][j];
                        if new_dist < dist[i][j] {
                            dist[i][j] = new_dist;
                        }
                    }
                }
            }
        }
        // 負の閉路チェック
        for (i, item) in dist.iter().enumerate().take(self.n) {
            if item[i] < 0 {
                return None;
            }
        }
        Some(dist)
    }

    /// プリム法:O(E log V)
    /// 無向グラフ専用
    pub fn prim(&self) -> Option<i64> {
        assert!(
            !self.is_directed,
            "Prim's algorithm requires an undirected graph"
        );
        if self.n == 0 {
            return Some(0);
        }
        let mut visited = vec![false; self.n];
        let mut heap: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();
        let mut total_cost = 0;
        let mut visited_count = 0;
        heap.push(Reverse((0, 0)));
        while let Some(Reverse((cost, u))) = heap.pop() {
            if visited[u] {
                continue;
            }
            visited[u] = true;
            total_cost += cost;
            visited_count += 1;

            for edge in &self.edges[u] {
                if !visited[edge.to] {
                    heap.push(Reverse((edge.weight, edge.to)));
                }
            }
        }
        if visited_count == self.n {
            Some(total_cost)
        } else {
            None // 非連結で全域木むり
        }
    }
}

// =====================================================
// アルゴリズム(構造系)
// =====================================================
impl Graph {
    /// 強連結成分分解(scc)コサラジュ法:O(V+E)
    /// 逆操作なし,非再帰。戻り値:(成分数,各頂点のID)
    pub fn scc(&self) -> (usize, Vec<usize>) {
        let mut ord = vec![NONE; self.n];
        let mut low = vec![NONE; self.n];
        let mut in_stack = vec![false; self.n];
        let mut stack = Vec::new();

        let mut call_stack = Vec::new();
        let mut timestamp = 0;

        let mut group = vec![NONE; self.n];
        let mut count = 0;

        for i in 0..self.n {
            if ord[i] != NONE {
                continue;
            }
            call_stack.push((i, 0));

            while let Some(&(u, edge_idx)) = call_stack.last() {
                if edge_idx == 0 {
                    ord[u] = timestamp;
                    low[u] = timestamp;
                    timestamp += 1;
                    stack.push(u);
                    in_stack[u] = true;
                }
                if edge_idx < self.edges[u].len() {
                    let v = self.edges[u][edge_idx].to;
                    call_stack.last_mut().unwrap().1 += 1;
                    if ord[v] == NONE {
                        call_stack.push((v, 0));
                    } else if in_stack[v] {
                        low[u] = low[u].min(ord[v]);
                    }
                } else {
                    call_stack.pop();
                    if let Some(&(p, _)) = call_stack.last() {
                        low[p] = low[p].min(low[u]);
                    }
                    if low[u] == ord[u] {
                        while let Some(v) = stack.pop() {
                            in_stack[v] = false;
                            group[v] = count;
                            if u == v {
                                break;
                            }
                        }
                        count += 1;
                    }
                }
            }
        }
        (count, group)
    }

    /// トポロジカルソート:O(V+E)
    /// 閉路があればNone
    pub fn topological_sort(&self) -> Option<Vec<usize>> {
        let mut ind = vec![0; self.n];
        for u in 0..self.n {
            for edge in &self.edges[u] {
                ind[edge.to] += 1;
            }
        }
        let mut queue = std::collections::VecDeque::new();
        for (i, &d) in ind.iter().enumerate() {
            if d == 0 {
                queue.push_back(i);
            }
        }
        let mut result = Vec::with_capacity(self.n);
        while let Some(u) = queue.pop_front() {
            result.push(u);
            for edge in &self.edges[u] {
                ind[edge.to] -= 1;
                if ind[edge.to] == 0 {
                    queue.push_back(edge.to);
                }
            }
        }

        (result.len() == self.n).then_some(result)
    }
}

// =====================================================
// 経路復元
// =====================================================
/// `pre`配列を辿って`start`から`goal`へのパスを復元
pub fn get_path(pre: &[usize], start: usize, goal: usize) -> Vec<usize> {
    if goal >= pre.len() || (goal != start && pre[goal] == NONE) {
        return vec![];
    }
    let mut path = Vec::new();
    let mut curr = goal;
    while curr != NONE {
        path.push(curr);
        if curr == start {
            break;
        }
        curr = pre[curr];
        if path.len() > pre.len() {
            return vec![];
        }
    }
    path.reverse();
    path
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // 1. 最小全域木 (Prim) のテスト
    // =========================================================================

    #[test]
    #[should_panic(expected = "Prim's algorithm requires an undirected graph")]
    fn test_prim_directed_panic() {
        let mut g = Graph::new(3, true); // 有向グラフ
        g.add_edge(0, 1, 1);
        g.prim(); // パニックになることを確認
    }

    #[test]
    fn test_prim_mst() {
        let mut g = Graph::new(4, false);
        g.add_edge(0, 1, 1);
        g.add_edge(0, 2, 4);
        g.add_edge(1, 2, 2);
        g.add_edge(1, 3, 6);
        g.add_edge(2, 3, 3);

        // 0-1 (cost 1), 1-2 (cost 2), 2-3 (cost 3) の辺が選ばれて合計6
        assert_eq!(g.prim(), Some(6));
    }

    #[test]
    fn test_prim_disconnected() {
        let mut g = Graph::new(4, false);
        g.add_edge(0, 1, 1);
        g.add_edge(2, 3, 1); // 0,1 と 2,3 が分断されている
        assert_eq!(g.prim(), None, "非連結グラフならNoneを返す");
    }

    // =========================================================================
    // 2. 最短経路・経路復元のテスト
    // =========================================================================

    #[test]
    fn test_dijkstra() {
        let mut g = Graph::new(4, true);
        g.add_edge(0, 1, 1);
        g.add_edge(1, 2, 2);
        g.add_edge(0, 2, 5);
        g.add_edge(2, 3, 1);

        let (dist, _) = g.dijkstra(0);
        assert_eq!(dist, vec![0, 1, 3, 4]);
    }

    #[test]
    fn test_undirected_dijkstra() {
        let mut g = Graph::new(3, false);
        g.add_edge(0, 1, 10);
        g.add_edge(1, 2, 20);

        let (dist, pre) = g.dijkstra(2);
        assert_eq!(dist[0], 30);
        assert_eq!(get_path(&pre, 2, 0), vec![2, 1, 0]);
    }

    #[test]
    fn test_bellman_ford_negative_edges() {
        let mut g = Graph::new(4, true);
        g.add_edge(0, 1, 5);
        g.add_edge(1, 2, -2);
        g.add_edge(2, 3, 3);
        g.add_edge(0, 3, 10);

        let res = g.bellman_ford(0);
        assert!(res.is_some());
        let (dist, pre) = res.unwrap();
        assert_eq!(dist[3], 6);
        assert_eq!(get_path(&pre, 0, 3), vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_bellman_ford_negative_cycle() {
        let mut g = Graph::new(3, true);
        g.add_edge(0, 1, 1);
        g.add_edge(1, 2, -1);
        g.add_edge(2, 1, -1);

        assert!(g.bellman_ford(0).is_none());
    }

    #[test]
    fn test_warshall_floyd() {
        let mut g = Graph::new(3, true);
        g.add_edge(0, 1, 2);
        g.add_edge(1, 2, 3);

        let res = g.warshall_floyd();
        assert!(res.is_some());
        let dist = res.unwrap();
        assert_eq!(dist[0][2], 5);
        assert_eq!(dist[1][0], INF);
    }

    #[test]
    fn test_warshall_floyd_negative_cycle() {
        let mut g = Graph::new(3, true);
        g.add_edge(0, 1, 1);
        g.add_edge(1, 2, -1);
        g.add_edge(2, 1, -1);

        assert!(g.warshall_floyd().is_none());
    }

    #[test]
    fn test_get_path_safety() {
        let pre_broken = vec![NONE, 2, 1, NONE];
        assert!(get_path(&pre_broken, 0, 2).is_empty());
    }

    // =========================================================================
    // 3. 構造・閉路 (SCC, トポロジカルソート) のテスト
    // =========================================================================

    #[test]
    fn test_scc_and_topological_sort() {
        let mut g_cycle = Graph::new(3, true);
        g_cycle.add_edge(0, 1, 1);
        g_cycle.add_edge(1, 2, 1);
        g_cycle.add_edge(2, 0, 1);

        let (count, _) = g_cycle.scc();
        assert_eq!(count, 1);
        assert_eq!(g_cycle.topological_sort(), None);

        let mut g_dag = Graph::new(3, true);
        g_dag.add_edge(0, 1, 1);
        g_dag.add_edge(1, 2, 1);
        assert_eq!(g_dag.topological_sort(), Some(vec![0, 1, 2]));
    }

    // =========================================================================
    // 4. エッジケース・コーナーケースのテスト
    // =========================================================================

    #[test]
    fn test_single_vertex() {
        let g = Graph::new(1, true);
        assert_eq!(g.dijkstra(0).0, vec![0]);
        assert_eq!(g.bellman_ford(0).unwrap().0, vec![0]);
        assert_eq!(g.warshall_floyd().unwrap(), vec![vec![0]]);
        assert_eq!(g.scc().0, 1);
        assert_eq!(g.topological_sort(), Some(vec![0]));
    }

    #[test]
    fn test_disconnected_components() {
        let mut g = Graph::new(4, true);
        g.add_edge(0, 1, 10);
        g.add_edge(2, 3, 20);

        let (dist, pre) = g.dijkstra(0);
        assert_eq!(dist, vec![0, 10, INF, INF]);
        assert!(get_path(&pre, 0, 2).is_empty());
    }

    #[test]
    fn test_multiple_edges_and_self_loops() {
        let mut g = Graph::new(3, true);
        g.add_edge(0, 1, 10);
        g.add_edge(0, 1, 2); // 最短の多重辺
        g.add_edge(1, 1, 5); // 自己ループ
        g.add_edge(1, 2, 10);

        let (dist, _) = g.dijkstra(0);
        assert_eq!(dist[1], 2);
        assert_eq!(dist[2], 12);
    }

    #[test]
    fn test_negative_self_loop() {
        let mut g = Graph::new(1, true);
        g.add_edge(0, 0, -1);
        assert!(g.bellman_ford(0).is_none());
    }

    #[test]
    fn test_start_equals_goal_path() {
        let g = Graph::new(3, true);
        let (_, pre) = g.dijkstra(0);
        assert_eq!(get_path(&pre, 0, 0), vec![0]);
    }

    #[test]
    fn test_scc_complex_disconnected() {
        let mut g = Graph::new(5, true);
        g.add_edge(0, 1, 1);
        g.add_edge(1, 0, 1);
        g.add_edge(3, 4, 1);
        g.add_edge(4, 3, 1);

        let (count, group) = g.scc();
        assert_eq!(count, 3);
        assert_eq!(group[0], group[1]);
        assert_eq!(group[3], group[4]);
        assert_ne!(group[0], group[2]);
        assert_ne!(group[2], group[3]);
    }

    // =========================================================================
    // 5. 極限ストレステスト（悪意のあるケース・限界突破）
    // =========================================================================

    #[test]
    fn test_scc_deep_recursion_prevention() {
        // 深さ10万の直線グラフ（非再帰実装の恩恵確認）
        let n = 100_000;
        let mut g = Graph::new(n, true);
        for i in 0..n - 1 {
            g.add_edge(i, i + 1, 1);
        }
        let (count, _) = g.scc();
        assert_eq!(count, n, "10万頂点でStack Overflowせずに完走できるか");
    }

    #[test]
    fn test_zero_weight_cycle() {
        // 合計コストがピッタリ『0』の閉路の誤検知防止
        let mut g = Graph::new(3, true);
        g.add_edge(0, 1, 1);
        g.add_edge(1, 2, -1);
        g.add_edge(2, 0, 0); // 1 - 1 + 0 = 0

        assert!(
            g.bellman_ford(0).is_some(),
            "コスト0の閉路を負の閉路と誤検知していないか"
        );
        assert!(
            g.warshall_floyd().is_some(),
            "コスト0の閉路を負の閉路と誤検知していないか"
        );
    }

    #[test]
    fn test_near_overflow_weights() {
        // INFに近い巨大な重みでのオーバーフロー耐性
        let mut g = Graph::new(3, true);
        let huge = INF / 3;
        g.add_edge(0, 1, huge);
        g.add_edge(1, 2, huge);

        let (dist, _) = g.dijkstra(0);
        assert_eq!(
            dist[2],
            huge * 2,
            "巨大な重みの加算でパニックを起こさないか"
        );
    }

    #[test]
    fn test_dense_graph_prim() {
        // 完全グラフ（密グラフ）での最小全域木
        let n = 50;
        let mut g = Graph::new(n, false);
        for i in 0..n {
            for j in i + 1..n {
                g.add_edge(i, j, (j - i) as i64);
            }
        }
        assert_eq!(
            g.prim(),
            Some((n - 1) as i64),
            "密グラフで正しく最小コストを計算できるか"
        );
    }
}
