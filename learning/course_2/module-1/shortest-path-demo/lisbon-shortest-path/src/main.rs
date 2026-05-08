use petgraph::algo::dijkstra;
use petgraph::prelude::*;

use clap::{Subcommand, Parser};
use regex::Regex;

#[derive(Parser)]
#[command(name = "shortest-path")]
#[command(version = "1.0")]
#[command(about, long_about=None)]
#[command(next_line_help = true)]
struct Cli 
{
    #[arg(short, long, default_value_t = false)]
    dry_run: bool, 

    #[arg(short, long)]
    node: Vec<String>,

    /// format examples: a-b:1
    #[arg(short, long)] 
    edge: Vec<String>,

    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands
{
    StartAndEndNodes{
        #[arg(long, short)]
        start_node: Option<String>,

        #[arg(long, short)]
        end_node: Option<String>
    }

}


fn dry_run(start_node: Option<String>, end_node: Option<String>)
{
    let mut graph = Graph::<&str, u32, Undirected>::new_undirected();

    let belem_tower = graph.add_node("Belem Tower");

    let monastery = graph.add_node("Jerónimos Monastery");

    let lx_factory = graph.add_node("LX Factory");

    let commerce_square = graph.add_node("Commerce Square");

    let lisbon_cathedral = graph.add_node("Lisbon Cathedral");

    graph.extend_with_edges([
        (belem_tower, monastery, 1),
        (belem_tower, lx_factory, 3),
        (monastery, commerce_square, 6),
        (lx_factory, commerce_square, 5),
        (commerce_square, lisbon_cathedral, 1),
    ]);

    let mut source = belem_tower;
    let mut dest = Some(lisbon_cathedral);

    if let Some(st_node) = start_node
    {
        source = graph.node_indices().find(|&i| graph[i] == st_node.as_str()).unwrap_or(source);
    }

    if let Some(en_node) = end_node
    {
        dest = graph.node_indices().find(|&i| graph[i] == en_node.as_str());
    }

    let node_map = dijkstra(&graph, source, dest, |e| *e.weight());

    if node_map.is_empty()
    {
        println!("No shortest path from {}", graph[source]);
    }
    else {
        println!("Shortest paths from {}:", graph[source]);

        for (node_idx, cost) in node_map
        {
            println!("To: {} - Cost: {}", graph[node_idx], cost)
        }
    }
}


fn main() {

    let cli = Cli::parse();

    if cli.dry_run
    {
        match cli.command
        {
            Commands::StartAndEndNodes { start_node, end_node } => {
                dry_run(start_node, end_node);
            }
        }
        return
    }

    let nodes = cli.node;

    let mut graph = Graph::<&str, u32, Undirected>::new_undirected();
    

    let node_indexes: Vec<NodeIndex> = nodes.iter().map(|u|{
        graph.add_node(u.as_str())
    }).collect();

    let edge_re = Regex::new(r"(?<u>\w+)-(?<v>\w+):(?<w>\d+)$").unwrap();

    let default_node= node_indexes[0];

    let weighted_edges: Vec<(NodeIndex, NodeIndex, u32)> = cli.edge.iter().filter_map(|e_raw| {
        let caps = edge_re.captures(e_raw)?;
        Some((
            caps["u"].parse::<String>().ok()?,
            caps["v"].parse::<String>().ok()?,
            caps["w"].parse::<u32>().ok()?,
        ))
    })
    .filter_map(|(u, v, w)|{
        let u_idx = graph.node_indices().find(|&i| graph[i] == u.as_str())?;
        let v_idx = graph.node_indices().find(|&i| graph[i] == v.as_str())?;

        Some((u_idx, v_idx, w))
    })
    .collect();

    graph.extend_with_edges(weighted_edges);

    match cli.command 
    {
        Commands::StartAndEndNodes { start_node, end_node } => {
            if let Some(st_node) = start_node{
               let st_node_idx = graph.node_indices().find(|&i| graph[i] == st_node.as_str()).unwrap_or(default_node);
               let mut en_node_idx: Option<NodeIndex> = None;

               if let Some(en_node) = end_node
               {
                en_node_idx = graph.node_indices().find(|&u| graph[u] == en_node.as_str());
               }

               let node_map = dijkstra(&graph, st_node_idx, en_node_idx, |e| *e.weight());

                println!("Shortest paths from {}:", st_node);

                for (node_idx, cost) in node_map
                {
                    println!("To: {} - Cost: {}", graph[node_idx], cost)
                }
            }
        }
    }

}
