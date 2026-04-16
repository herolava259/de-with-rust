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

            for rank in &mut ranks {
                *rank = *rank * self.damping + (1.0 - self.damping) / (n as f64);
            }

            ranks = new_ranks;
        }

        ranks
    }
}

fn main() {
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
