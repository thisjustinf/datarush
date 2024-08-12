mod cli;
mod config;
mod processor;

use cli::Cli;
use processor::{CsvProcessor, JsonProcessor, Processor};
use structopt::StructOpt;

#[tokio::main]
async fn main() {
    let args = Cli::from_args();
    let settings = config::load_config();

    let output_format = output::format::determine_format(&args.output_format);

    if let Some(csv_path) = args.csv_file {
        let csv_processor = CsvProcessor::new(csv_path);
        csv_processor.process(output_format).unwrap();
    }

    if let Some(json_path) = args.json_file {
        let json_processor = JsonProcessor::new(json_path);
        json_processor.process(output_format).unwrap();
    }
}