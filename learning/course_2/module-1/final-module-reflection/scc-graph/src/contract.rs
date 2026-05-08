
use std::{collections::HashSet, hash::Hash};

#[derive(Clone)]
pub struct DirectedGraph
{
    pub n_vertex: usize,
    pub adj: Vec<Vec<usize>>

}

impl DirectedGraph 
{
    pub fn new(n: usize, adj: &Vec<Vec<usize>>) -> Self
    {
        Self {
            n_vertex: n,
            adj: adj.iter().cloned().collect()
        }
    }

    pub fn from_empty(n: usize) -> Self
    {
        Self {
            n_vertex: n,
            adj: vec![vec![]; n]
        }
    }

    pub fn add_edge(&mut self, u: usize, v: usize)
    {
        self.adj[u].push(v);
    }

    pub fn transpose(&self) -> Self {
        let mut g = DirectedGraph::from_empty(self.n_vertex);

        for u in 0..self.n_vertex{
            for &v in &self.adj[u]{
                g.add_edge(v, u);
            }
        }

        g
    }


}

pub trait FindingStronglyConnectedComponentSolver<TVertexIndex>
where TVertexIndex : Clone + Eq + Hash
{
    fn solve(&mut self) -> Vec<HashSet<TVertexIndex>>;
}

pub trait FindingSCCInLargeGraphSolution<TVertexIndex>
where TVertexIndex: Clone + Eq + Hash
{
    fn compute(&mut self) -> Vec<HashSet<TVertexIndex>>;
}