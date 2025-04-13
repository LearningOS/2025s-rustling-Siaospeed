/*
	bfs
	This problem requires you to implement a basic BFS algorithm
*/
use std::collections::VecDeque;

// 定义图结构
struct Graph {
    adj: Vec<Vec<usize>>, 
}

impl Graph {
    // 创建一个包含 n 个节点的图
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    // 添加一条无向边
    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest); 
        self.adj[dest].push(src); 
    }

    // BFS 搜索，从 start 开始，返回访问顺序
    fn bfs_with_return(&self, start: usize) -> Vec<usize> {
        let mut visited = vec![false; self.adj.len()];
        let mut queue = VecDeque::new();
        let mut visit_order = vec![];

        visited[start] = true;
        queue.push_back(start);

        while let Some(node) = queue.pop_front() {
            visit_order.push(node);

            for &neighbor in &self.adj[node] {
                if !visited[neighbor] {
                    visited[neighbor] = true;
                    queue.push_back(neighbor);
                }
            }
        }

        visit_order
    }
}
