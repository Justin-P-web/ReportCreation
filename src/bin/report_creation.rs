use clap::{Args, Parser, Subcommand};
use std::{fs, path::PathBuf, thread, time::Duration};

use ReportCreation as reportcreation;

const DEFAULT_TICK_RATE: u64 = 60;

/// Generate a PDF file from an existing Typst document.
#[derive(Parser)]
#[command(author, version, about, args_conflicts_with_subcommands = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Path to the Typst file that should be compiled (default command).
    #[arg(value_name = "INPUT.typ")]
    input: Option<PathBuf>,

    /// Output path for the generated PDF. Defaults to replacing the extension with `.pdf`.
    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile a Typst document into a PDF (default when no subcommand is given).
    Compile(CompileArgs),
    /// Start the dispatcher loop with a configurable tick rate.
    Start(StartArgs),
}

#[derive(Args)]
struct CompileArgs {
    /// Path to the Typst file that should be compiled.
    #[arg(value_name = "INPUT.typ")]
    input: PathBuf,

    /// Output path for the generated PDF. Defaults to replacing the extension with `.pdf`.
    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,
}

#[derive(Args)]
struct StartArgs {
    /// How many ticks should be processed per second.
    #[arg(long, default_value_t = DEFAULT_TICK_RATE, value_name = "HERTZ", value_parser = clap::value_parser!(u64).range(1..))]
    tick_rate: u64,
}

struct Dispatcher {
    tick_rate: u64,
    tick_duration: Duration,
}

impl Dispatcher {
    fn new(tick_rate: u64) -> Self {
        let tick_duration = Duration::from_nanos(1_000_000_000u64.saturating_div(tick_rate));
        Self {
            tick_rate,
            tick_duration,
        }
    }

    fn run_for_ticks(&self, ticks: u64) {
        for _ in 0..ticks {
            self.advance_tick();
        }
    }

    fn run_forever(&self) -> ! {
        loop {
            self.run_for_ticks(u64::MAX);
        }
    }

    fn advance_tick(&self) {
        thread::sleep(self.tick_duration);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Compile(args)) => compile(args),
        Some(Commands::Start(args)) => start(args),
        None => {
            let input = cli.input.ok_or_else(
                || "missing input Typst file; pass it directly or use the compile subcommand",
            )?;
            compile(CompileArgs {
                input,
                output: cli.output,
            })
        }
    }
}

fn compile(args: CompileArgs) -> Result<(), Box<dyn std::error::Error>> {
    let source = fs::read_to_string(&args.input)?;
    let output_path = args
        .output
        .clone()
        .unwrap_or_else(|| args.input.with_extension("pdf"));

    let pdf_bytes = reportcreation::compile_pdf(&source, &args.input);
    fs::write(&output_path, &pdf_bytes)?;

    println!("PDF written to {}", output_path.display());

    Ok(())
}

fn start(args: StartArgs) -> ! {
    let dispatcher = Dispatcher::new(args.tick_rate);

    println!(
        "Starting dispatcher at {} ticks/second",
        dispatcher.tick_rate
    );

    dispatcher.run_forever();
}
