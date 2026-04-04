use clap::Parser;
use cli_salad::{create_fruit_salad, create_fruit_salad_with_fruit_options, validate_inputs};



#[derive(Parser)]
#[command(
    version = "1.0",
    author = "Your Name <your.email@example.com>",
    about = "Number of fruits to include in the salad"
)]
struct Args {
    #[arg(short, long)]
    number: usize,

    #[arg(short, long, default_value_t = String::from("orange, apple"))]
    optional_fruit: String
}


fn main() {

    let opts: Args = Args::parse();

    let num_fruits = opts.number;

    let mut optional_fruits: Vec<String> = opts.optional_fruit.split(",")
                                                        .into_iter()
                                                        .map(|s| s.to_string())
                                                        .collect();
    if !validate_inputs(&optional_fruits)
    {
        panic!("Arguments exists duplication of intgrations");
    }

    let final_salad = create_fruit_salad_with_fruit_options(num_fruits, &mut optional_fruits);
    let new_salad: Vec<_> = final_salad.into_iter().collect();
    println!(
        "Created Fruit salad wiht {} fruits: {:?}",
        num_fruits,
        new_salad
    )
}
