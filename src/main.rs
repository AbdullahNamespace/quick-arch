use anyhow::Result;
use clap::Parser;
mod cli;
mod config;
mod evaluator;
mod generator;
mod ui;
use cli::Cli;
use generator::ProjectGenerator;

fn main() -> Result<()> {
    let start_time = std::time::Instant::now();

    ui::print_header();

    let args = Cli::parse();
    args.validate_config_exists()?;

    let mut generator = ProjectGenerator::new(&args.config)?;

    let stats = generator.generate(args.output.as_deref())?;

    if let Some(ref scripts) = stats.scripts_to_run {
        ui::print_section("Running Post-Creation Scripts");
        generator.execute_scripts(scripts)?;
    }

    let duration = start_time.elapsed();
    ui::print_summary(&stats, duration);

    Ok(())
}
