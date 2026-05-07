use std::{cmp::Reverse, collections::BinaryHeap};

// =====================================================
// 定数定義
// =====================================================
pub const INF: i64 = 1_000_000_000_000_000_000; // オーバーフローしないでかい値
pub const NONE: usize = usize::MAX;

// =====================================================
// Edge構造体
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
        if !self.is_directed && from != to {
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
                    // 巨大な weight でのオーバーフローを回避
                    let Some(next_dist) = dist[u].checked_add(edge.weight) else {
                        continue;
                    };
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
                // checked_add: 巨大な weight でも検出時にオーバーフローを起こさない
                if let Some(next_dist) = dist[u].checked_add(edge.weight)
                    && next_dist < dist[edge.to]
                {
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
    /// 強連結成分分解(scc)タルヤン法:O(V+E)
    /// 非再帰。戻り値:(成分数,各頂点のSCC_ID)
    /// SCC_IDはトポロジカル逆順（ACLと同じ仕様）。辺はID大→ID小の向きになる
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
