use std::{collections::{HashMap, HashSet}, hash::Hash, io};

use textwrap::fill;

struct PageRank {
    damping: f64,
    iterations: usize,
}

impl PageRank {
    fn new(damping: f64, iterations: usize) -> Self {
        Self {
            damping,
            iterations,
        }
    }

    fn rank(&self, graph: &[Vec<usize>]) -> Vec<f64> {
        let n = graph.len();

        let mut ranks = vec![1.0 / (n as f64); n];

        for _ in 0..self.iterations {
            let mut new_ranks = vec![0.0; n];

            for (node, edges) in graph.iter().enumerate() {
                let rank_contribution = ranks[node] / (edges.len() as f64);
                for &neighbor in edges {
                    new_ranks[neighbor] += rank_contribution;
                }
            }

            for rank in &mut new_ranks {
                *rank = *rank * self.damping + (1.0 - self.damping) / (n as f64);
            }

            ranks = new_ranks;
        }

        ranks
    }
}

pub struct Node<TIndex: Hash + Clone + Eq, TContent>
{
    index: TIndex,
    content: TContent
}


pub struct Graph<TIndex: Hash + Clone + Eq, TContent>
{
    nodes: HashMap<TIndex,Node<TIndex, TContent>>,

    adjancents: HashMap<TIndex, HashSet<TIndex>>,

}


impl<TIndex: Hash + Clone + Eq, TContent> Graph<TIndex, TContent>
{
    fn num_nodes(&self) -> usize
    {
        self.nodes.len()
    }

    fn num_edges(&self) -> usize
    {
        let mut result: usize = 0;

        for node_idx in self.nodes.keys()
        {
            result += self.adjancents[node_idx].len()
        } 

        result / 2

    }
}


pub trait CentralityScore<TScore>
{
    fn compute<TIndex: Hash + Clone + Eq, TContent>(&self, graph: &Graph<TIndex, TContent>) -> HashMap<TIndex, TScore>;
}


impl CentralityScore<f64> for PageRank
{
    fn compute<TIndex: Hash + Clone + Eq, TContent>(&self, graph: &Graph<TIndex, TContent>) -> HashMap<TIndex, f64> {
        let n = graph.num_nodes();

        let mut ranks: HashMap<TIndex, f64> = graph.nodes.keys().map(|k| (k.clone(), 0.0)).collect();

        let num_edge = graph.num_edges();

        for _ in 0..self.iterations
        {
            let mut new_ranks = ranks.clone();

            for node_idx in graph.nodes.keys(){
                let rank_contrib = ranks[node_idx] / (num_edge as f64);

                for neighbor in graph.adjancents[node_idx].iter()
                {
                    new_ranks.entry(neighbor.clone()).and_modify(|e| *e += rank_contrib);
                }

            }

            for (_, rank) in new_ranks.iter_mut()
            {
                *rank = *rank * self.damping + (1.0 - self.damping) / (n as f64);
            }
            ranks = new_ranks
        }

        ranks
    }
}



fn default()
{
    let graph = [
        vec![1, 2], // ESPN links to NFL, NBA
        vec![0],    // NFL links to ESPN
        vec![0, 3], // NBA links to ESPN, UFC
        vec![0],    // UFC links to ESPN
        vec![0, 1], // MLB links to ESPN, NFL
    ];

    let names = ["ESPN", "NFL", "NBA", "UFC", "MLB"];

    let pagerank = PageRank::new(0.85, 100);

    let ranks = pagerank.rank(&graph);

    for (i, rank) in ranks.iter().enumerate() {
        println!("The PageRank of {} is {}", names[i], rank);
    }

    let explanation = "PageRank is a link analysis algorithm used by Google that uses the hyperlink structure of the web to determine a quality ranking for each web page. It works by counting the number and quality of links to a page to determine a rough estimate of how important the website is.";
    println!("\n{}", fill(explanation, 80));
}

fn run_pagerank_alg(graph: Vec<Vec<usize>>, name_of_nodes: Vec<String>)
{

    let pagerank = PageRank::new(0.85, 100);

    let ranks = pagerank.rank(&graph);

    for (i, rank) in ranks.iter().enumerate() {
        println!("The PageRank of {} is {}", name_of_nodes[i], rank);
    }

    let explanation = "PageRank is a link analysis algorithm used by Google that uses the hyperlink structure of the web to determine a quality ranking for each web page. It works by counting the number and quality of links to a page to determine a rough estimate of how important the website is.";
    println!("\n{}", fill(explanation, 80));
}

fn input_node_names(n: usize, names: &mut Vec<String>)
{
    println!("Input the names of each node");
    println!("Total number of nodes in the graph: {}", n);

    let mut input: String = String::new();

    for i in 0..n{
        print!("Typing name of node {}: ", i);
        input.clear();

        io::stdin().read_line(&mut input).expect("Failed to read line");

        names.push(input.clone());
    }
}

fn input_node_connections(edges: &mut Vec<Vec<usize>>)
{
    println!("Type the edge between 2 nodes, separate between the pair by the comma(,) and a space ( )");
    println!("Typing the character 'q' is you want to stop. ");

    let mut input: String = String::new();

    loop
    {
        input.clear();

        io::stdin().read_line(&mut input).expect("Fault to read nodes");

        let pair: Vec<usize> = input
                                .trim()
                                .split(", ")
                                .map(|s| s.parse().expect("Please enter valid intergers"))
                                .collect();
        if(pair.len() != 2)
        {
            panic!("You should typing following by the above rule");
        }

        edges.push(pair);

    }

}

use clap::{Subcommand, Parser, Args};

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Commands
}


#[derive(Args, Debug)]
#[group(required= true, multiple = false)]
struct Versions
{
    /// set version manually
    #[arg(long, value_name = "VER")]
    set_ver: Option<String>,

    /// auto inc major
    #[arg(long)]
    major: bool,

    /// auto inc minor
    #[arg(long)]
    minor: bool,

    /// auto inc patch
    #[arg(long)]
    patch: bool,
}




#[derive(Subcommand, Debug)]
enum Commands {
    Default,
    CustomInput{
        #[arg(short, long, default_value_t = 1)]
        total: usize,

        #[arg(short, long, default_value_t = String::from( "a, b, c"))]
        names: String,

        #[arg(short, long, default_value_t = String::from("(1, 2); (1, 3), (2, 3)"))]
        edges: String
    },
    StepByStep
}

use regex::Regex;

fn main() {

    let cli = Cli::parse();

    match cli.command {
        Commands::Default => default(),
        Commands::CustomInput { total,names,  edges } => {

            let re= Regex::new(r"\((?<u>\d+),\s*(?<v>\d+)\)").unwrap();

            let edges: Vec<Vec<usize>> = re.captures_iter(&edges)
                                           .map( |caps| {

                                            let u: usize = caps["u"].parse().unwrap();
                                            let v: usize = caps["v"].parse().unwrap();
                                            vec![u, v]
                                           })
                                           .collect();
            let names: Vec<String> = Regex::new(r"(?<name>\w+),\s*").unwrap()
                                            .captures_iter(&names)
                                            .map( |caps| {
                                                let name = caps["name"].parse().unwrap();
                                                name
                                            })
                                            .collect();
            run_pagerank_alg(edges, names);
        },
        Commands::StepByStep => {
            let mut input = String::new();
            print!("Type total nodes: ");
            io::stdin().read_line(&mut input).expect("Failed to read num of nodes");

            let num_nodes: usize = input.parse().expect("Please enter the valid positive interger");

            let mut name_of_nodes: Vec<String> = Vec::new();
            
            let mut edges: Vec<Vec<usize>> = Vec::new();

            input_node_names(num_nodes, &mut name_of_nodes);

            input_node_connections(&mut edges);

            run_pagerank_alg(edges, name_of_nodes);

        }
    }


}
