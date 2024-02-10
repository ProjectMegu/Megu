//! sccアルゴルズムを実装する

use std::collections::VecDeque;

struct Scc {
    g: Vec<Vec<usize>>,
    r_g: Vec<Vec<usize>>,
    post_order: VecDeque<usize>,
    visited: Vec<bool>,
}

impl Scc {
    fn new(nodes: usize, edges: &Vec<(usize, usize)>) -> Self {
        let mut g = vec![vec![]; nodes];
        let mut r_g = vec![vec![]; nodes];
        for edge in edges {
            g[edge.0].push(edge.1);
            r_g[edge.1].push(edge.0);
        }

        Self {
            g,
            r_g,
            post_order: VecDeque::new(),
            visited: vec![false; nodes],
        }
    }

    // 帰り掛け順でノードを記録する
    fn dfs(&mut self, u: usize) {
        let mut stack = vec![u];
        while let Some(v) = stack.pop() {
            if !self.visited[v] {
                // 行き
                self.visited[v] = true;
                stack.push(v);

                for &w in &self.g[v] {
                    if !self.visited[w] {
                        stack.push(w);
                    }
                }
            } else {
                // 帰り
                self.post_order.push_front(v);
            }
        }
    }

    // 各エッジを逆向きにしたグラフ上で到達可能なノード集合を調べる
    fn rdfs(&mut self, u: usize) -> Vec<usize> {
        let mut stack = vec![u];
        let mut scc = Vec::new();
        while let Some(v) = stack.pop() {
            self.visited[v] = true;
            scc.push(v);
            for &u in &self.r_g[v] {
                if !self.visited[u] {
                    stack.push(u);
                }
            }
        }
        scc
    }

    // 強連結成分を求める
    fn build(&mut self) -> Vec<Vec<usize>> {
        for v in 0..self.g.len() {
            if !self.visited[v] {
                self.dfs(v);
            }
        }

        self.visited = vec![false; self.g.len()];
        let mut sccs = Vec::new();
        for i in 0..self.post_order.len() {
            let v = self.post_order[i];
            if !self.visited[v] {
                sccs.push(self.rdfs(v));
            }
        }
        sccs
    }
}

pub fn scc(n: usize, edges: &Vec<(usize, usize)>) -> Vec<Vec<usize>> {
    Scc::new(n, edges).build()
}