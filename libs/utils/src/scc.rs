//! sccアルゴルズムを実装する

use std::{
    collections::{hash_map::DefaultHasher, HashMap, VecDeque},
    hash::{Hash, Hasher},
    marker::PhantomData,
};

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

#[derive(Debug, Default)]
pub struct SccMap<'a, T: PartialEq> {
    map: HashMap<usize, &'a T>,
    edges: Vec<(usize, usize)>,
    node_num: usize,
}

impl<'a, T: PartialEq> SccMap<'a, T> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            edges: Vec::new(),
            node_num: 0,
        }
    }

    pub fn add_edge(&mut self, edge: (&'a T, &'a T)) {
        let (a, b) = edge;

        if !self.map.iter().any(|(_, k)| *k == a) {
            self.map.insert(self.node_num, a);
            self.node_num += 1;
        }

        if !self.map.iter().any(|(_, k)| *k == b) {
            self.map.insert(self.node_num, b);
            self.node_num += 1;
        }

        self.edges.push((
            *self.map.iter().find(|(_, k)| **k == a).unwrap().0,
            *self.map.iter().find(|(_, k)| **k == b).unwrap().0,
        ));
    }

    pub fn run(&self) -> Vec<Vec<&'a T>> {
        let scc = scc(self.node_num, &self.edges);
        let mut ret = Vec::new();
        for scc in &scc {
            ret.push(scc.iter().map(|i| *self.map.get(i).unwrap()).collect());
        }

        ret
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_scc() {
        let edges = vec![(0, 1), (1, 2), (2, 0), (1, 3), (3, 4), (4, 5), (5, 3)];
        let scc = scc(6, &edges);
        assert_eq!(scc, vec![vec![0, 2, 1], vec![3, 5, 4]]);
    }

    #[test]
    fn test_scc_map() {
        let mut scc_map = SccMap::<'_, String>::new();
        
        let a = "a".to_string();
        let b = "b".to_string();
        let c = "c".to_string();
        let d = "d".to_string();
        
        scc_map.add_edge((&a, &b));
        scc_map.add_edge((&b, &c));
        scc_map.add_edge((&c, &a));
        scc_map.add_edge((&d, &a));

        println!("{:?}",scc_map.run());
    }
}
