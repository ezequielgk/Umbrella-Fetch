use std::{env, io::{self, stdout}, time::Duration};
use crossterm::{
    event,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use ratatui::{prelude::*, TerminalOptions, Viewport};

mod ascii;
mod config;
mod lore;
mod system_info;
mod ui;
mod virus;
mod ubcs;

struct UbcsArgs {
    list: bool,
    squad: Option<String>,
}

struct VirusArgs {
    strain: String,
    list: bool,
}

enum Command {
    Full,
    Ubcs(UbcsArgs),
    Virus(VirusArgs),
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    let mut command = Command::Full;
    let mut watch_secs = None;
    
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "full" => command = Command::Full,
            "ubcs" => {
                let mut u_args = UbcsArgs { list: false, squad: None };
                let mut j = i + 1;
                while j < args.len() {
                    if args[j] == "--list" {
                        u_args.list = true;
                    } else if args[j] == "--squad" && j + 1 < args.len() {
                        u_args.squad = Some(args[j+1].to_uppercase());
                        j += 1;
                    }
                    j += 1;
                }
                command = Command::Ubcs(u_args);
                break;
            }
            "virus" => {
                let mut v_args = VirusArgs { strain: "t-virus".to_string(), list: false };
                let mut j = i + 1;
                while j < args.len() {
                    if args[j] == "--list" {
                        v_args.list = true;
                    } else if args[j] == "--strain" && j + 1 < args.len() {
                        v_args.strain = args[j+1].clone();
                        j += 1;
                    }
                    j += 1;
                }
                command = Command::Virus(v_args);
                break;
            }
            "--watch" => {
                if i + 1 < args.len() {
                    if let Ok(secs) = args[i+1].parse::<u64>() {
                        watch_secs = Some(secs);
                        i += 1;
                    } else {
                        watch_secs = Some(2);
                    }
                } else {
                    watch_secs = Some(2);
                }
            }
            _ => {}
        }
        i += 1;
    }

    match command {
        Command::Ubcs(u_args) => {
            if u_args.list {
                println!("\nU.B.C.S. ROSTER\n");
                let iter = ubcs::roster::ROSTER.iter().filter(|op| {
                    if let Some(sq) = &u_args.squad {
                        &op.squad.to_string() == sq
                    } else {
                        true
                    }
                });
                for op in iter {
                    let c = match op.status {
                        ubcs::OperativeStatus::Active => crossterm::style::Color::Green,
                        ubcs::OperativeStatus::Kia => crossterm::style::Color::Red,
                        ubcs::OperativeStatus::Mia => crossterm::style::Color::DarkGrey,
                    };
                    let _ = crossterm::execute!(stdout(), crossterm::style::SetForegroundColor(c));
                    println!("{:<12} {:<20} {:<8} {}", op.rank, op.name, op.squad, op.speciality);
                }
                let _ = crossterm::execute!(stdout(), crossterm::style::ResetColor);
                println!("");
                return Ok(());
            }

            let mut state = ubcs::state::UbcsAppState::new();
            if let Some(sq) = &u_args.squad {
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
        Command::Virus(v_args) => {
            if v_args.list {
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

            let profile = match virus::strains::find_strain(&v_args.strain) {
                Some(p) => p,
                None => {
                    eprintln!("ERROR: Unknown strain '{}'. Run 'umbrella-fetch virus --list' for available strains.", v_args.strain);
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
                Command::Full => ui::full::draw_full(f, &info, &config, watch_secs.is_some(), ticker_offset),
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
