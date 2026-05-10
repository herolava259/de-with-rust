

use std::{collections::{HashSet, VecDeque}};

use crate::contract::{DirectedGraph, FindingSCCInLargeGraphSolution, FindingStronglyConnectedComponentSolver};



pub struct KosarajuAlgSolver<'a>
{
    forward_order: VecDeque<usize>,
    strong_connected_components: Vec<HashSet<usize>>,
    visited: Vec<bool>,
    n_vertex: usize,
    in_neighbor: &'a DirectedGraph,
    out_neighbor: DirectedGraph
}

impl<'a> KosarajuAlgSolver<'a>
{
    pub fn new(n_vertex: usize, graph: &'a DirectedGraph) -> Self
    {
        Self 
        {
            forward_order: VecDeque::with_capacity(n_vertex),
            strong_connected_components: vec![],
            visited: vec![false; n_vertex],
            n_vertex: n_vertex,
            in_neighbor: graph,
            out_neighbor: graph.transpose()
        }
    }

    fn dfs_forward(&mut self, u: usize, p: usize)
    {
        self.visited[u] = true;

        let adj_u = self.out_neighbor.adj[u].clone();

        for v in adj_u.into_iter()
        {
            if v == p || self.visited[v]
            {
                continue
            }

            self.dfs_forward(v, u);
        }

        self.forward_order.push_front(u);
    }

    pub fn deep_dfs_forward(&mut self)
    {
        for r in 0..self.n_vertex
        {
            if self.visited[r]
            {
                continue;
            }
            let mut call_stack: VecDeque<(usize, usize, usize)> = VecDeque::new();
            let mut is_return = true;

            let mut u_arg = r;
            let mut p_arg: usize = self.n_vertex;
            let mut adj_idx: usize = 0;
            self.visited[u_arg] = true;

            call_stack.push_back((u_arg, p_arg, adj_idx));

            loop {
                if is_return && let Some((prev_u_arg, prev_p_arg, prev_adj_idx)) = call_stack.pop_back(){
                    u_arg = prev_u_arg;
                    p_arg = prev_p_arg;
                    adj_idx = prev_adj_idx;
                }
                else if call_stack.is_empty()
                {
                    break;
                }

                while adj_idx < self.in_neighbor.adj[u_arg].len() && (self.visited[self.in_neighbor.adj[u_arg][adj_idx]]
                    || self.in_neighbor.adj[u_arg][adj_idx] == p_arg)
                {
                    adj_idx += 1;
                }

                if adj_idx >= self.in_neighbor.adj[u_arg].len()
                {
                    self.forward_order.insert(0, u_arg);
                    is_return = true;
                    continue;
                }

                is_return = false;

                let v = self.in_neighbor.adj[u_arg][adj_idx];

                self.visited[v] = true;

                call_stack.push_back((u_arg, p_arg, adj_idx+1));

                p_arg = u_arg;
                u_arg = v;
                adj_idx = 0;

            }
        }
    }


    fn dfs_backward(&mut self, u: usize, p: usize, component: &mut HashSet<usize>)
    {
        self.visited[u] = true;

        let adj_u = self.in_neighbor.adj[u].clone();

        for v in adj_u.into_iter()
        {
            if self.visited[v] || v == p 
            {
                continue;
            }
            component.insert(v);
            self.dfs_backward(v, u, component);
        }
    }

    pub fn deep_dfs_backward(&mut self)
    {
        for r in 0..self.n_vertex
        {
            if self.visited[r]
            {
                continue;
            }

            let mut call_stack: VecDeque<(usize, usize, usize)> = VecDeque::new();
            let mut is_return: bool = true;

            let mut u_arg = r;
            let mut p_arg: usize = self.n_vertex;
            let mut adj_idx: usize = 0;
            self.visited[u_arg] = true;
            
            let mut component: HashSet<usize> = HashSet::new();

            component.insert(u_arg);

            call_stack.push_back((u_arg, p_arg, adj_idx));

            loop {
                if is_return && let Some((p_u_arg, p_p_arg, p_adj_idx)) = call_stack.pop_back()
                {
                    u_arg = p_u_arg;
                    p_arg = p_p_arg;
                    adj_idx = p_adj_idx;
                }
                else if call_stack.is_empty()
                {
                    break;
                }

                while adj_idx < self.out_neighbor.adj[u_arg].len() && (self.visited[self.out_neighbor.adj[u_arg][adj_idx]]
                    || self.in_neighbor.adj[u_arg][adj_idx] == p_arg)
                {
                    adj_idx += 1;
                }

                if adj_idx >= self.out_neighbor.adj[u_arg].len()
                {
                    is_return = true;
                    continue;
                }

                is_return = false;

                let v = self.out_neighbor.adj[u_arg][adj_idx];

                self.visited[v] = true;
                component.insert(v);

                call_stack.push_back((u_arg, p_arg, adj_idx+1));

                p_arg = u_arg;
                u_arg = v;
                adj_idx = 0;
            }
            self.strong_connected_components.push(component);
        }
    }
}


impl<'a> FindingStronglyConnectedComponentSolver<usize> for KosarajuAlgSolver<'a>
{
    fn solve(&mut self) -> Vec<HashSet<usize>> {
        for u in 0..self.n_vertex
        {
            if !self.visited[u]
            {
                self.dfs_forward(u, self.n_vertex);
            }
        }

        self.visited.fill(false);

        while let Some(u) = self.forward_order.pop_front()
        {
            if self.visited[u]
            {
                continue;
            }
            let mut component = HashSet::new();
            self.dfs_backward(u, self.n_vertex, &mut component);

            self.strong_connected_components.push(component);
        }

        return self.strong_connected_components.clone();
    }
}

impl<'a> FindingSCCInLargeGraphSolution<usize> for KosarajuAlgSolver<'a>
{
    fn compute(&mut self) -> Vec<HashSet<usize>> {
        self.deep_dfs_forward();

        self.visited.fill(false);

        self.deep_dfs_backward();

        return self.strong_connected_components.clone();
    }
}
