use colored::*;
use std::time::Duration;

pub fn print_header() {
    println!(
        "{}",
        "╔════════════════════════════════════════════════════════════╗".cyan()
    );
    println!(
        "{}",
        "║   🚀 Universal Project Structure Generator (Rust)          ║".cyan()
    );
    println!(
        "{}",
        "╚════════════════════════════════════════════════════════════╝".cyan()
    );
    println!();
}

pub fn print_step(current: u32, total: u32, action: &str, item_name: &str, status: Status) {
    let icon = match status {
        Status::Success => "✅".green(),
        Status::Skip => "⏩".dimmed(),
    };

    let counter = format!("[{}/{}]", current, total).dimmed();
    let action_colored = action.yellow();
    let name_colored = item_name.cyan();

    println!("{} {} {}: {}", counter, action_colored, name_colored, icon);
}

pub fn print_section(title: &str) {
    println!();
    println!(
        "{}",
        format!("━━━━━━━━━━━━━━━━ {} ━━━━━━━━━━━━━━━━", title).cyan()
    );
    println!();
}

pub fn print_info(msg: &str) {
    println!("{}", format!("ℹ️  {}", msg).blue());
}

pub fn print_error(msg: &str) {
    eprintln!("{}", format!("❌ {}", msg).red());
}

pub enum Status {
    Success,
    Skip,
}

pub fn print_summary(stats: &crate::config::GenerationStats, duration: Duration) {
    println!();
    println!(
        "{}",
        "╔════════════════════════════════════════════════════════════╗".cyan()
    );
    println!(
        "{}",
        "║                   ✨ Creation Summary                      ║".cyan()
    );
    println!(
        "{}",
        "╠════════════════════════════════════════════════════════════╣".cyan()
    );
    println!(
        "{} {:<30} {}",
        "║".cyan(),
        "Project:",
        stats.project_name.bold()
    );
    println!(
        "{} {:<30} {}",
        "║".cyan(),
        "Location:",
        stats.output_dir.dimmed()
    );
    println!(
        "{} {:<30} {}",
        "║".cyan(),
        "Dirs Created:",
        stats.dirs_count.to_string().green()
    );
    println!(
        "{} {:<30} {}",
        "║".cyan(),
        "Files Created:",
        stats.files_count.to_string().green()
    );
    println!(
        "{} {:<30} {}",
        "║".cyan(),
        "Total Skipped:",
        stats.skipped_count.to_string().yellow()
    );

    let duration_str = format!("{:.2?}", duration);
    println!(
        "{} {:<30} {}",
        "║".cyan(),
        "Duration:",
        duration_str.magenta()
    );
    println!(
        "{}",
        "╚════════════════════════════════════════════════════════════╝".cyan()
    );

    println!();
    println!("{}", "🎉 Done! Happy coding!".bright_green().bold());
}
