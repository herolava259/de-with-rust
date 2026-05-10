use clap::{Parser, Subcommand};
use fruit_salad_maker::{create_fruit_salad, csv_to_vec, display_fruit_salad};


#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Your Name",
    about = "A simple fruit salad generator"
)]
struct Cli {
    #[clap(short, long)]
    fruits: Option<String>,

    #[clap(short, long)]
    movies: Option<String>,

    #[clap(short, long)]
    fruit_csv_file: Option<String>,

    #[clap(short, long)]
    book_csv_file: Option<String>,

    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands
{
    PrintOut ,
    CustomSalad
    {
        #[arg(short, long)]
        shuffle: bool,

        #[arg(short, long, default_value_t=false)]
        salty: bool,

        #[arg(short, long, default_value_t=false)]
        add_oil: bool,

        #[arg(short, long)]
        output_file: Option<String>

    },

    ExportToCsv{
        #[arg(short, long)]
        csvfile: String
    }
}

fn main() {
    let cli: Cli = Cli::parse();

    match cli.command {
        Commands::CustomSalad 
        { shuffle, salty, add_oil, output_file }
        =>{

        },
        Commands::ExportToCsv { csvfile } =>{

        },

        Commands::PrintOut => {
            
        }
    }
    

}