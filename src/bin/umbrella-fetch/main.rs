//! Main entry point and CLI router for Umbrella Fetch.

use std::{io::{self, stdout}, time::Duration};
use crossterm::{
    event,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use ratatui::{prelude::*, TerminalOptions, Viewport};
use clap::{Parser, Subcommand, CommandFactory};

mod ascii;
mod config;
mod lore;
mod system_info;
mod ui;
mod virus;
mod shared;
mod ubcs;
mod uss;
mod minimal;

/// CLI arguments parser.
#[derive(Parser)]
#[command(name = "umbrella-fetch")]
#[command(about = "Umbrella Corporation Terminal Feed", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Watch feed continuously (seconds)
    #[arg(long, global = true)]
    watch: Option<Option<u64>>,
}

/// Available subcommands.
#[derive(Subcommand)]
enum Commands {
    /// Full secure feed
    Full,
    
    /// U.B.C.S. roster and status
    Ubcs {
        /// List roster
        #[arg(long)]
        list: bool,
        /// Filter by squad
        #[arg(long)]
        squad: Option<String>,
    },
    
    /// Virus strain simulation
    Virus {
        /// List strains
        #[arg(long)]
        list: bool,
        /// Select strain
        #[arg(long, default_value = "t-virus")]
        strain: String,
    },
    
    /// U.S.S. classified roster
    Uss {
        /// List classified roster
        #[arg(long)]
        list: bool,
        /// Filter by squad
        #[arg(long)]
        squad: Option<String>,
        /// Filter by status
        #[arg(long)]
        status: Option<String>,
    },
    
    /// Minimal system fetch
    Minimal,
    
    /// Generate shell completions
    #[command(hide = true)]
    Completions {
        #[arg(value_enum)]
        shell: clap_complete::Shell,
    },
}

/// Internal parsed command state.
enum AppCommand {
    Full,
    Ubcs { list: bool, squad: Option<String> },
    Virus { strain: String, list: bool },
    Uss { list: bool, squad: Option<String>, status: Option<String> },
    Minimal,
}

/// Main application entry point.
fn main() -> io::Result<()> {
    let cli = Cli::parse();
    
    let watch_secs = match cli.watch {
        Some(Some(secs)) => Some(secs),
        Some(None) => Some(2),
        None => None,
    };

    let command = match cli.command.unwrap_or(Commands::Full) {
        Commands::Full => AppCommand::Full,
        Commands::Ubcs { list, squad } => AppCommand::Ubcs { list, squad },
        Commands::Virus { list, strain } => AppCommand::Virus { list, strain },
        Commands::Uss { list, squad, status } => AppCommand::Uss { list, squad, status },
        Commands::Minimal => AppCommand::Minimal,
        Commands::Completions { shell } => {
            let mut cmd = Cli::command();
            let bin_name = cmd.get_name().to_string();
            clap_complete::generate(shell, &mut cmd, bin_name, &mut std::io::stdout());
            return Ok(());
        }
    };

    match command {
        AppCommand::Ubcs { list, squad } => {
            if list {
                println!("\nU.B.C.S. ROSTER\n");
                let iter = ubcs::roster::ROSTER.iter().filter(|op| {
                    if let Some(sq) = &squad {
                        &op.squad.to_string() == sq
                    } else {
                        true
                    }
                });
                for op in iter {
                    let c = match op.status {
                        crate::shared::OperativeStatus::Active => crossterm::style::Color::Green,
                        crate::shared::OperativeStatus::Kia => crossterm::style::Color::Red,
                        crate::shared::OperativeStatus::Mia => crossterm::style::Color::DarkGrey,
                        crate::shared::OperativeStatus::Retired => crossterm::style::Color::DarkGrey,
                    };
                    let _ = crossterm::execute!(stdout(), crossterm::style::SetForegroundColor(c));
                    println!("{:<12} {:<20} {:<8} {}", op.rank, op.name, op.squad, op.speciality);
                }
                let _ = crossterm::execute!(stdout(), crossterm::style::ResetColor);
                println!("");
                return Ok(());
            }

            let mut state = ubcs::state::UbcsAppState::new();
            if let Some(sq) = &squad {
                state.filter_squad = match sq.as_str() {
                    "ALPHA" => Some("ALPHA"),
                    "BRAVO" => Some("BRAVO"),
                    "DELTA" => Some("DELTA"),
                    _ => None,
                };
            }

            enable_raw_mode()?;
            crossterm::execute!(stdout(), crossterm::terminal::EnterAlternateScreen)?;
            let mut terminal = Terminal::with_options(CrosstermBackend::new(stdout()), TerminalOptions { viewport: Viewport::Fullscreen })?;
            
            let tick_rate = Duration::from_millis(80);
            loop {
                terminal.draw(|f| {
                    ubcs::ui::draw_ubcs(f, f.area(), &state);
                })?;
                
                if event::poll(tick_rate)? {
                    if let event::Event::Key(key) = event::read()? {
                        match key.code {
                            event::KeyCode::Char('q') | event::KeyCode::Esc | event::KeyCode::Char('c') => break,
                            event::KeyCode::Up | event::KeyCode::Char('k') => state.previous(),
                            event::KeyCode::Down | event::KeyCode::Char('j') => state.next(),
                            event::KeyCode::Enter | event::KeyCode::Char(' ') => state.show_detail = !state.show_detail,
                            event::KeyCode::Char('f') => state.cycle_squad_filter(),
                            event::KeyCode::Char('s') => state.cycle_status_filter(),
                            event::KeyCode::Char('p') => state.toggle_price_sort(),
                            _ => {}
                        }
                    }
                }
            }
            
            crossterm::execute!(stdout(), crossterm::terminal::LeaveAlternateScreen)?;
            disable_raw_mode()?;
            return Ok(());
        }
        AppCommand::Virus { strain, list } => {
            if list {
                println!("\nAVAILABLE STRAINS — umbrella-fetch virus --strain <id>\n");
                for s in virus::strains::ALL_STRAINS {
                    let _ = crossterm::execute!(
                        stdout(),
                        crossterm::style::SetForegroundColor(crossterm::style::Color::Rgb { r: s.color.0, g: s.color.1, b: s.color.2 })
                    );
                    println!("{:<12} {:<12} {:<18} {}", s.arg_id, s.name, s.class, s.threat);
                }
                let _ = crossterm::execute!(stdout(), crossterm::style::ResetColor);
                println!("");
                return Ok(());
            }

            let profile = match virus::strains::find_strain(&strain) {
                Some(p) => p,
                None => {
                    eprintln!("ERROR: Unknown strain '{}'. Run 'umbrella-fetch virus --list' for available strains.", strain);
                    std::process::exit(1);
                }
            };
            
            enable_raw_mode()?;
            crossterm::execute!(stdout(), crossterm::terminal::EnterAlternateScreen)?;
            let mut terminal = Terminal::with_options(CrosstermBackend::new(stdout()), TerminalOptions { viewport: Viewport::Fullscreen })?;
            
            let mut state = virus::state::VirusAppState {
                sim: virus::simulation::ViralGrid::new(10, 10, profile), 
                tick: 0,
            };
            
            let tick_rate = Duration::from_millis(80);
            loop {
                let size = terminal.size()?;
                
                let target_width = ((size.width * 3) / 4).saturating_sub(2) as usize;
                let target_height = size.height.saturating_sub(9) as usize;
                
                let target_width = target_width.max(10);
                let target_height = target_height.max(10);
                
                if state.sim.cols != target_width || state.sim.rows != target_height {
                    state.sim = virus::simulation::ViralGrid::new(target_width, target_height, profile);
                }
                
                terminal.draw(|f| {
                    virus::ui::draw_virus(f, f.area(), &state);
                })?;
                
                if event::poll(tick_rate)? {
                    if let event::Event::Key(key) = event::read()? {
                        if key.code == event::KeyCode::Char('q') || key.code == event::KeyCode::Esc || key.code == event::KeyCode::Char('c') {
                            break;
                        }
                    }
                } else {
                    state.sim.step();
                    state.tick += 1;
                }
            }
            
            crossterm::execute!(stdout(), crossterm::terminal::LeaveAlternateScreen)?;
            disable_raw_mode()?;
            return Ok(());
        }
        AppCommand::Uss { list, squad, status } => {
            if list {
                println!("\nU.S.S. ROSTER — CLASSIFIED\n");
                let iter = uss::roster::ROSTER.iter().filter(|op| {
                    let mut ok = true;
                    if let Some(sq) = &squad {
                        if !op.alpha_id.starts_with(sq) { ok = false; }
                    }
                    if let Some(st_str) = &status {
                        let st = match st_str.as_str() {
                            "active" => Some(crate::shared::OperativeStatus::Active),
                            "kia" => Some(crate::shared::OperativeStatus::Kia),
                            "mia" => Some(crate::shared::OperativeStatus::Mia),
                            "retired" => Some(crate::shared::OperativeStatus::Retired),
                            _ => None,
                        };
                        if let Some(s) = st {
                            if op.status != s { ok = false; }
                        }
                    }
                    ok
                });
                for op in iter {
                    let c = match op.status {
                        crate::shared::OperativeStatus::Active => crossterm::style::Color::DarkGreen,
                        crate::shared::OperativeStatus::Kia => crossterm::style::Color::DarkRed,
                        crate::shared::OperativeStatus::Mia => crossterm::style::Color::DarkGrey,
                        crate::shared::OperativeStatus::Retired => crossterm::style::Color::Grey,
                    };
                    let _ = crossterm::execute!(stdout(), crossterm::style::SetForegroundColor(c));
                    println!("{:<10} {:<20} {:<20} {}", op.alpha_id, op.codename, op.speciality, if op.real_name.is_none() { "REDACTED" } else { op.real_name.unwrap() });
                }
                let _ = crossterm::execute!(stdout(), crossterm::style::ResetColor);
                println!("");
                return Ok(());
            }

            let mut state = uss::state::UssAppState::new();
            if let Some(sq) = &squad {
                state.filter_squad = match sq.as_str() {
                    "ALPHA" => Some("ALPHA"),
                    _ => None,
                };
            }
            if let Some(st_str) = &status {
                state.filter_status = match st_str.as_str() {
                    "active" => Some(crate::shared::OperativeStatus::Active),
                    "kia" => Some(crate::shared::OperativeStatus::Kia),
                    "mia" => Some(crate::shared::OperativeStatus::Mia),
                    "retired" => Some(crate::shared::OperativeStatus::Retired),
                    _ => None,
                };
            }

            enable_raw_mode()?;
            crossterm::execute!(stdout(), crossterm::terminal::EnterAlternateScreen)?;
            let mut terminal = Terminal::with_options(CrosstermBackend::new(stdout()), TerminalOptions { viewport: Viewport::Fullscreen })?;
            
            let tick_rate = Duration::from_millis(80);
            loop {
                terminal.draw(|f| {
                    uss::ui::draw_uss(f, f.area(), &state);
                })?;
                
                if event::poll(tick_rate)? {
                    if let event::Event::Key(key) = event::read()? {
                        match key.code {
                            event::KeyCode::Char('q') | event::KeyCode::Esc | event::KeyCode::Char('c') => break,
                            event::KeyCode::Up | event::KeyCode::Char('k') => state.previous(),
                            event::KeyCode::Down | event::KeyCode::Char('j') => state.next(),
                            event::KeyCode::Enter | event::KeyCode::Char(' ') => state.show_detail = !state.show_detail,
                            event::KeyCode::Char('f') => state.cycle_squad_filter(),
                            event::KeyCode::Char('s') => state.cycle_status_filter(),
                            event::KeyCode::Char('p') => state.toggle_price_sort(),
                            _ => {}
                        }
                    }
                }
            }
            
            crossterm::execute!(stdout(), crossterm::terminal::LeaveAlternateScreen)?;
            disable_raw_mode()?;
            return Ok(());
        }
        AppCommand::Minimal => {
            let config = config::Config::load();
            let info = system_info::SystemInfo::fetch();
            minimal::print_minimal_fetch(&info, &config)?;
            return Ok(());
        }
        _ => {}
    }

    let config = config::Config::load();
    let mut info = system_info::SystemInfo::fetch();

    enable_raw_mode()?;
    
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic| {
        let _ = disable_raw_mode();
        let _ = crossterm::execute!(stdout(), crossterm::terminal::LeaveAlternateScreen);
        original_hook(panic);
    }));

    crossterm::execute!(stdout(), crossterm::terminal::EnterAlternateScreen)?;

    let viewport = Viewport::Fullscreen;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::with_options(backend, TerminalOptions { viewport })?;

    let mut ticker_offset = 0;
    let mut tick_counter = 0;
    
    let tick_rate = Duration::from_millis(200);
    let mut last_fetch = std::time::Instant::now();
    
    loop {
        terminal.draw(|f| {
            match command {
                AppCommand::Full => ui::full::draw_full(f, &info, &config, watch_secs.is_some(), ticker_offset),
                _ => {}
            }
        })?;
        
        if event::poll(tick_rate)? {
            if let event::Event::Key(key) = event::read()? {
                if key.code == event::KeyCode::Char('q') || key.code == event::KeyCode::Esc || key.code == event::KeyCode::Char('c') {
                    break;
                }
            }
        }
        
        tick_counter += 1;
        if tick_counter >= 3 {
            ticker_offset += 1;
            tick_counter = 0;
        }
        
        if let Some(secs) = watch_secs {
            if last_fetch.elapsed().as_secs() >= secs {
                info = system_info::SystemInfo::fetch();
                last_fetch = std::time::Instant::now();
            }
        }
    }

    crossterm::execute!(stdout(), crossterm::terminal::LeaveAlternateScreen)?;
    disable_raw_mode()?;
    
    Ok(())
}
