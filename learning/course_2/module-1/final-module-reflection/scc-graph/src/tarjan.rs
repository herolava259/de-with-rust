use std::{cmp::min, collections::{HashSet, VecDeque}};

use crate::contract::{DirectedGraph, FindingSCCInLargeGraphSolution, FindingStronglyConnectedComponentSolver};



pub struct TarjanAlgSolver<'a>
{
    n_vertex: usize,
    graph: &'a DirectedGraph,
    time_in: Vec<usize>,
    time_lowest: Vec<usize>,
    free: Vec<bool>,
    strong_connected_components: Vec<HashSet<usize>>,
    timer: usize,
    stack: VecDeque<usize>,
    finished: bool
}

impl<'a> TarjanAlgSolver<'a>
{
    pub fn new(n_vertex: usize, graph: &'a DirectedGraph) -> Self
    {
        Self 
        {
            n_vertex: n_vertex,
            graph: graph,
            time_in: vec![0; n_vertex],
            time_lowest: vec![0, n_vertex],
            free: vec![false; n_vertex],
            strong_connected_components: vec![],
            timer: 0,
            stack: VecDeque::new(),
            finished: false
        }
    }

    fn dfs_visit(&mut self, u: usize, p: usize)
    {
        self.timer += 1;
        self.time_in[u] = self.timer;
        self.time_lowest[u] = self.timer;

        self.stack.push_back(u);

        let u_adj = self.graph.adj[u].clone();

        for v in u_adj.into_iter()
        {
            if v == p || self.free[v]
            {
                continue;
            }

            if self.time_in[v] == 0
            {
                self.dfs_visit(v, u);
            }

            self.time_lowest[u] = min(self.time_lowest[u], self.time_lowest[v]);
        }

        if self.time_lowest[u] < self.time_in[u]
        {
            return;
        }

        let mut component: HashSet<usize> = HashSet::new();

        while let Some(v) = self.stack.pop_back() && self.time_lowest[v] == self.time_lowest[u]
        {
            component.insert(v);

            self.free[v] = true;
        }

        self.strong_connected_components.push(component);

    }

    fn deep_dfs_visit(&mut self)
    {
        for r in 0..self.n_vertex
        {
            if self.time_in[r] != 0
            {
                continue;
            }

            let mut call_stack: VecDeque<(usize, usize)> = VecDeque::with_capacity(self.n_vertex);

            self.timer += 1;
            self.time_in[r] = self.timer;
            self.time_lowest[r] = self.timer;

            self.stack.push_back(r);

            call_stack.push_back((r, 0));


            while let Some((u_arg, local_counter)) = call_stack.pop_back()
            {
                if let Some(&v) = self.graph.adj[u_arg].get(local_counter) && self.time_in[v] == 0
                {
                    // it is the first time visiting vertex v
                    // call recrusively to v to traverse
                    self.timer += 1;
                    self.time_in[v] = self.timer;
                    self.time_lowest[v] = self.timer;
                    self.stack.push_back(v);

                    call_stack.push_back((u_arg, local_counter));
                    call_stack.push_back((v, 0));
                }
                else if let Some(&v) = self.graph.adj[u_arg].get(local_counter)
                {
                    if ! self.free[v]
                    {
                        self.time_lowest[u_arg] = min(self.time_lowest[u_arg], self.time_lowest[v]);
                    }
                    call_stack.push_back((u_arg, local_counter+1));
                }
                else if self.time_in[u_arg] == self.time_lowest[u_arg]
                {
                    let mut component: HashSet<usize> = HashSet::new();

                    while let Some(&v) = self.stack.back() && self.time_lowest[v] == self.time_in[u_arg]
                    {
                        component.insert(v);
                        self.free[v] = true;
                        self.stack.pop_back();
                    }

                    self.strong_connected_components.push(component);
                }
                
            }

        }
    }
    
}


impl<'a> FindingStronglyConnectedComponentSolver<usize> for TarjanAlgSolver<'a> {
    fn solve(&mut self) -> Vec<HashSet<usize>> {

        if self.finished
        {
            return self.strong_connected_components.clone();
        }

        for u in 0..self.n_vertex
        {
            if self.time_in[u] == 0
            {
                self.dfs_visit(u, self.n_vertex);
            }
        }

        self.finished = true;

        self.strong_connected_components.clone()
    }
}

impl <'a> FindingSCCInLargeGraphSolution<usize> for TarjanAlgSolver<'a> 
{
    fn compute(&mut self) -> Vec<HashSet<usize>> {
        if self.finished
        {
            return self.strong_connected_components.clone();
        }

        self.deep_dfs_visit();

        self.finished = true;

        return self.strong_connected_components.clone();
    }
}