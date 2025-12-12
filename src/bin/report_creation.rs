use clap::{Args, Parser, Subcommand};
use std::{
    fs,
    io::{self, BufRead},
    path::PathBuf,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

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

enum Command {
    Pause,
    Resume,
    Step(u64),
    SetTickRate(u64),
    Custom(String),
    Terminate,
}

struct Dispatcher {
    tick_rate: u64,
    tick_duration: Duration,
    paused: bool,
}

impl Dispatcher {
    fn new(tick_rate: u64) -> Self {
        let tick_duration = Duration::from_nanos(1_000_000_000u64.saturating_div(tick_rate));
        Self {
            tick_rate,
            tick_duration,
            paused: false,
        }
    }

    fn update_tick_rate(&mut self, tick_rate: u64) {
        self.tick_rate = tick_rate;
        self.tick_duration = Duration::from_nanos(1_000_000_000u64.saturating_div(tick_rate));
        println!("Tick rate updated to {} ticks/second", self.tick_rate);
    }

    fn handle_command(&mut self, command: Command) -> bool {
        match command {
            Command::Pause => {
                self.paused = true;
                println!("Dispatcher paused");
            }
            Command::Resume => {
                self.paused = false;
                println!("Dispatcher resumed");
            }
            Command::Step(count) => {
                println!("Stepping dispatcher for {count} tick(s)");
                for _ in 0..count {
                    self.process_tick();
                }
            }
            Command::SetTickRate(rate) => self.update_tick_rate(rate),
            Command::Custom(message) => {
                println!("Received custom command: {message}");
            }
            Command::Terminate => {
                println!("Termination command received. Exiting dispatcher.");
                return true;
            }
        }

        false
    }

    fn process_tick(&self) {
        // Placeholder for simulator progression logic.
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
    let mut dispatcher = Dispatcher::new(args.tick_rate);
    let (tx, rx) = mpsc::channel();

    println!(
        "Starting dispatcher at {} ticks/second",
        dispatcher.tick_rate
    );

    spawn_stdin_listener(tx.clone());

    loop {
        let iteration_start = Instant::now();

        let mut should_terminate = false;
        while let Ok(command) = rx.try_recv() {
            if dispatcher.handle_command(command) {
                should_terminate = true;
                break;
            }
        }

        if should_terminate {
            break;
        }

        if !dispatcher.paused {
            dispatcher.process_tick();
        }

        let elapsed = iteration_start.elapsed();
        if elapsed < dispatcher.tick_duration {
            thread::sleep(dispatcher.tick_duration - elapsed);
        }
    }

    std::process::exit(0);
}

fn spawn_stdin_listener(tx: mpsc::Sender<Command>) {
    thread::spawn(move || {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            match line {
                Ok(text) => {
                    let trimmed = text.trim();
                    if trimmed.is_empty() {
                        continue;
                    }

                    match parse_command(trimmed) {
                        Ok(command) => {
                            if tx.send(command).is_err() {
                                break;
                            }
                        }
                        Err(err) => eprintln!("{err}"),
                    }
                }
                Err(err) => {
                    eprintln!("Failed to read from stdin: {err}");
                    break;
                }
            }
        }
    });
}

fn parse_command(input: &str) -> Result<Command, String> {
    let mut parts = input.split_whitespace();
    let verb = parts
        .next()
        .ok_or_else(|| "empty command received".to_string())?
        .to_lowercase();

    match verb.as_str() {
        "pause" => Ok(Command::Pause),
        "resume" => Ok(Command::Resume),
        "step" => {
            let count = if let Some(value) = parts.next() {
                value
                    .parse::<u64>()
                    .map_err(|_| format!("Invalid step count: {value}"))?
            } else {
                1
            };
            Ok(Command::Step(count))
        }
        "rate" | "tick_rate" => {
            let value = parts
                .next()
                .ok_or_else(|| "tick rate requires a numeric value".to_string())?;
            let rate = value
                .parse::<u64>()
                .map_err(|_| format!("Invalid tick rate: {value}"))?;
            if rate == 0 {
                return Err("Tick rate must be greater than zero".to_string());
            }
            Ok(Command::SetTickRate(rate))
        }
        "custom" => Ok(Command::Custom(parts.collect::<Vec<_>>().join(" "))),
        "quit" | "exit" | "terminate" => Ok(Command::Terminate),
        other => Err(format!("Unknown command: {other}")),
    }
}
