//! Minimalist interactive fetch renderer.

use std::io::{self, stdout};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Terminal, TerminalOptions, Viewport, Frame,
};

use crate::system_info::SystemInfo;
use crate::config::Config;

use crossterm::{
    event,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::time::Duration;

/// Renders and starts the minimalist fetch interactive loop.
pub fn print_minimal_fetch(info: &SystemInfo, config: &Config) -> io::Result<()> {
    enable_raw_mode()?;
    crossterm::execute!(stdout(), EnterAlternateScreen)?;
    
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::with_options(backend, TerminalOptions { viewport: Viewport::Fullscreen })?;

    // Bucle principal interactivo (layout y eventos de teclado)
    loop {
        terminal.draw(|f| {
            let size = f.area();
            
            // Verificación de tamaño mínimo para evitar pánicos del motor de layout
            if size.width < 80 || size.height < 26 {
                let text = Paragraph::new("TERMINAL TOO SMALL FOR SECURE FEED. PLEASE RESIZE.")
                    .alignment(ratatui::layout::Alignment::Center)
                    .style(Style::default().fg(Color::Red).add_modifier(ratatui::style::Modifier::BOLD));
                let block = crate::ui::shared::block_with_title("■ UMBRELLA CORP.");
                f.render_widget(block, size);
                let text_area = Rect { x: size.x, y: size.y + size.height / 2, width: size.width, height: 1 };
                f.render_widget(text, text_area);
                return;
            }
            
            // Calculamos cuánto ancho real requiere el texto para centrar todo el bloque como una unidad
            let fetch_width = calculate_fetch_width(info, config, size.width);
            let fetch_height = 26;
            
            // Coordenadas absolutas centradas en el lienzo disponible
            let layout_x = size.x + size.width.saturating_sub(fetch_width) / 2;
            let layout_y = size.y + size.height.saturating_sub(fetch_height) / 2;
            let layout_area = Rect::new(layout_x, layout_y, fetch_width, fetch_height);
            
            draw_minimal_layout(f, layout_area, info, config);
        })?;

        if event::poll(Duration::from_millis(200))? {
            if let event::Event::Key(key) = event::read()? {
                if key.code == event::KeyCode::Char('q') || key.code == event::KeyCode::Esc || key.code == event::KeyCode::Char('c') {
                    break;
                }
            }
        }
    }

    crossterm::execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    
    // Final inline render to keep it in terminal history
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::with_options(backend, TerminalOptions { viewport: Viewport::Inline(27) })?;
    terminal.draw(|f| {
        let size = f.area();
        let fetch_width = calculate_fetch_width(info, config, size.width);
        let layout_area = Rect::new(size.x, size.y, fetch_width, size.height);
        draw_minimal_layout(f, layout_area, info, config);
    })?;

    Ok(())
}

/// Calculates the required dynamic width for the minimalist view.
fn calculate_fetch_width(info: &SystemInfo, config: &Config, max_width: u16) -> u16 {
    let max_val_len = [
        info.user.chars().count(),
        info.hostname.chars().count(),
        info.os.chars().count(),
        info.kernel.chars().count(),
        info.uptime.chars().count(),
        info.shell.chars().count(),
        info.cpu_model.chars().count() + 15,
        config.app.mission.chars().count(),
        config.app.threat.chars().count(),
        config.app.directive.chars().count(),
    ].into_iter().max().unwrap_or(20);
    
    let right_width = 18 + max_val_len as u16 + 4;
    std::cmp::min(max_width, 36 + right_width)
}

/// Renders both panels (Left: Logo, Right: Info).
fn draw_minimal_layout(f: &mut Frame, layout_area: Rect, info: &SystemInfo, config: &Config) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(36), // Left Panel (Logo + Lore)
            Constraint::Min(0),     // System Info (takes remaining space up to fetch_width)
        ])
        .split(layout_area);

    // Render Custom Left Panel
    draw_left_panel(f, chunks[0], info, config);
    
    // Render System Identification block on the right
    draw_sys_info(f, chunks[1], info, config);
}

use std::process::Command;
use std::env;

fn get_packages() -> String {
    if let Ok(output) = Command::new("pacman").arg("-Q").output() {
        if output.status.success() {
            let count = String::from_utf8_lossy(&output.stdout).lines().count();
            if count > 0 { return format!("{} (pacman)", count); }
        }
    }
    if let Ok(output) = Command::new("dpkg").arg("-l").output() {
        if output.status.success() {
            let count = String::from_utf8_lossy(&output.stdout).lines().filter(|l| l.starts_with("ii")).count();
            if count > 0 { return format!("{} (dpkg)", count); }
        }
    }
    if let Ok(output) = Command::new("rpm").arg("-qa").output() {
        if output.status.success() {
            let count = String::from_utf8_lossy(&output.stdout).lines().count();
            if count > 0 { return format!("{} (rpm)", count); }
        }
    }
    "unknown".to_string()
}

fn get_wm(info: &SystemInfo) -> String {
    if let Ok(wm) = env::var("XDG_CURRENT_DESKTOP") {
        if !wm.is_empty() { return wm; }
    }
    if let Ok(wm) = env::var("DESKTOP_SESSION") {
        if !wm.is_empty() { return wm; }
    }
    if let Ok(wm) = env::var("WM") {
        if !wm.is_empty() { return wm; }
    }
    if !info.wm.is_empty() && info.wm != "Unknown" {
        return info.wm.clone();
    }
    "unknown".to_string()
}

fn get_terminal() -> String {
    if let Ok(term) = env::var("TERM_PROGRAM") {
        if !term.is_empty() { return term; }
    }
    if let Ok(term) = env::var("TERM") {
        if !term.is_empty() { return term; }
    }
    "unknown".to_string()
}

fn get_gpu(info: &SystemInfo) -> String {
    let mut raw = String::new();
    if let Ok(output) = Command::new("lspci").output() {
        if output.status.success() {
            let out = String::from_utf8_lossy(&output.stdout);
            for line in out.lines() {
                if line.to_lowercase().contains("vga") || line.to_lowercase().contains("3d controller") {
                    let parts: Vec<&str> = line.splitn(2, ": ").collect();
                    if parts.len() == 2 {
                        raw = parts[1].trim().to_string();
                        break;
                    }
                }
            }
        }
    }
    if raw.is_empty() && !info.gpu_info.is_empty() && info.gpu_info != "Unknown" {
        raw = info.gpu_info.clone();
    }
    
    if raw.is_empty() {
        return "unknown".to_string();
    }

    raw.split('[').last()
       .unwrap_or(&raw)
       .split(']').next()
       .unwrap_or(&raw)
       .trim()
       .to_string()
}

fn get_resolution() -> Option<String> {
    if let Ok(output) = Command::new("xrandr").output() {
        if output.status.success() {
            let out = String::from_utf8_lossy(&output.stdout);
            for line in out.lines() {
                if line.contains(" connected ") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    for p in parts {
                        if p.contains("x") && p.chars().next().unwrap().is_digit(10) {
                            let clean = p.split('+').next().unwrap_or(p);
                            return Some(clean.to_string());
                        }
                    }
                }
            }
        }
    }
    None
}

fn draw_left_panel(f: &mut Frame, area: Rect, info: &SystemInfo, config: &Config) {
    let block = crate::ui::shared::block_with_title("■ UMBRELLA CORP.");
    let inner = block.inner(area);
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(13), // logo
            Constraint::Length(1),  // UMBRELLA CORP.
            Constraint::Length(1),  // EST. 1968
            Constraint::Length(1),  // separador
            Constraint::Length(2),  // clearance + division
            Constraint::Length(1),  // separador
            Constraint::Length(1),  // ● LIVE
            Constraint::Length(1),  // separador
            Constraint::Min(0),     // lore directives
            Constraint::Length(4),  // quote
        ])
        .split(inner);

    // 0. Logo
    let mut logo_lines = Vec::new();
    for logo_line in crate::ascii::UMBRELLA_LOGO {
        logo_lines.push(Line::from(Span::styled(*logo_line, Style::default().fg(Color::Red))));
    }
    f.render_widget(Paragraph::new(logo_lines).alignment(ratatui::layout::Alignment::Center), chunks[0]);

    // 1 & 2. Subtitle
    f.render_widget(Paragraph::new(Line::from(Span::styled("UMBRELLA CORP.", Style::default().fg(Color::Red).add_modifier(ratatui::style::Modifier::BOLD)))).alignment(ratatui::layout::Alignment::Center), chunks[1]);
    f.render_widget(Paragraph::new(Line::from(Span::styled("EST. 1968 — CLASSIFIED", Style::default().fg(Color::DarkGray)))).alignment(ratatui::layout::Alignment::Center), chunks[2]);

    // 3. Separator
    f.render_widget(Paragraph::new(Line::from(Span::styled("────────────────────────", Style::default().fg(Color::DarkGray)))).alignment(ratatui::layout::Alignment::Center), chunks[3]);

    // 4. Clearance + Division
    let mut clearance_lines = Vec::new();
    clearance_lines.push(Line::from(vec![
        Span::styled("CLEARANCE ", Style::default().fg(Color::DarkGray)),
        Span::styled(&config.app.clearance, Style::default().fg(Color::Red).add_modifier(ratatui::style::Modifier::BOLD)),
    ]));
    clearance_lines.push(Line::from(vec![
        Span::styled("DIVISION  ", Style::default().fg(Color::DarkGray)),
        Span::styled(&config.app.division, Style::default().fg(Color::Gray)),
    ]));
    f.render_widget(Paragraph::new(clearance_lines).alignment(ratatui::layout::Alignment::Center), chunks[4]);

    // 5. Separator
    f.render_widget(Paragraph::new(Line::from(Span::styled("────────────────────────", Style::default().fg(Color::DarkGray)))).alignment(ratatui::layout::Alignment::Center), chunks[5]);

    // 6. LIVE
    let live_line = Line::from(vec![
        Span::styled("● LIVE  ", Style::default().fg(Color::Red)),
        Span::styled(&info.hostname, Style::default().fg(Color::Gray)),
    ]);
    f.render_widget(Paragraph::new(live_line).alignment(ratatui::layout::Alignment::Center), chunks[6]);

    // 7. Separator
    f.render_widget(Paragraph::new(Line::from(Span::styled("────────────────────────", Style::default().fg(Color::DarkGray)))).alignment(ratatui::layout::Alignment::Center), chunks[7]);

    // 8. Lore Directives
    let available_lines = chunks[8].height;
    let mut dir_lines = Vec::new();
    dir_lines.push(Line::from(Span::styled("▸ UMBRELLA DIRECTIVES", Style::default().fg(Color::Red))));
    
    if available_lines > 1 {
        let max_dirs = (available_lines - 1) as usize;
        for &dir in crate::config::DIRECTIVES.iter().take(max_dirs) {
            dir_lines.push(Line::from(vec![
                Span::styled("— ", Style::default().fg(Color::Red)),
                Span::styled(dir, Style::default().fg(Color::Gray)),
            ]));
        }
    }
    // Pad inside directives so quote is pushed to bottom properly by Paragraph auto height, but we use Constraint::Min(0), so ratatui handles pushing chunk 9 to bottom.
    f.render_widget(Paragraph::new(dir_lines).alignment(ratatui::layout::Alignment::Left), chunks[8]);

    // 9. Quote
    let quote = "\"Obedience Breeds Discipline,\nDiscipline Breeds Unity,\nUnity Breeds Power.\"";
    let quote_para = Paragraph::new(quote)
        .style(Style::default().fg(Color::DarkGray).add_modifier(ratatui::style::Modifier::ITALIC))
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(quote_para, chunks[9]);
}

fn draw_sys_info(f: &mut Frame, area: Rect, info: &SystemInfo, config: &Config) {

    let gray = Style::default().fg(Color::Gray);
    
    let add_pair = |key: &str, val: &str, val_style: Style| -> Line<'static> {
        Line::from(vec![
            Span::styled(format!("{:<15}", key), Style::default().fg(Color::DarkGray)),
            Span::styled(" → ", Style::default().fg(Color::Red)),
            Span::styled(val.to_string(), val_style),
        ])
    };
    
    let mut sys_lines = Vec::new();
    sys_lines.push(add_pair("USER", &info.user, gray));
    sys_lines.push(add_pair("HOSTNAME", &info.hostname, gray));
    sys_lines.push(add_pair("OS", &info.os, gray));
    sys_lines.push(add_pair("KERNEL", &info.kernel, gray));
    sys_lines.push(add_pair("UPTIME", &info.uptime, gray));
    sys_lines.push(add_pair("PACKAGES", &get_packages(), gray));
    sys_lines.push(add_pair("SHELL", &info.shell, gray));
    
    if let Some(res) = get_resolution() {
        sys_lines.push(add_pair("RESOLUTION", &res, gray));
    }
    
    sys_lines.push(add_pair("WM / DE", &get_wm(info), gray));
    sys_lines.push(add_pair("TERMINAL", &get_terminal(), gray));
    
    let mut hw_lines = Vec::new();
    hw_lines.push(add_pair("CPU", &format!("{} ({} CORES)", info.cpu_model, info.cpu_cores), gray));
    hw_lines.push(add_pair("GPU", &get_gpu(info), gray));
    hw_lines.push(add_pair("MEMORY", &format!("{:.1} / {:.1} GB", info.ram_used_gb, info.ram_total_gb), gray));
    
    let disk_str = if let Some((_, used, total)) = info.partitions.first() {
        let pct = if *total > 0 { (*used as f64 / *total as f64 * 100.0) as u8 } else { 0 };
        format!("{}G / {}G ({}%)", used, total, pct)
    } else {
        "unknown".to_string()
    };
    hw_lines.push(add_pair("DISK", &disk_str, gray));

    let mut lore_lines = Vec::new();
    lore_lines.push(add_pair("STATUS", "▓ OPERATIONAL", Style::default().fg(Color::Gray)));
    lore_lines.push(add_pair("MISSION", &config.app.mission, Style::default().fg(Color::DarkGray)));
    lore_lines.push(add_pair("THREAT", &config.app.threat, Style::default().fg(Color::DarkGray)));
    lore_lines.push(add_pair("DIRECTIVE", &config.app.directive, Style::default().fg(Color::DarkGray)));
    lore_lines.push(add_pair("PROTOCOL", "OMEGA-7 SECURE", Style::default().fg(Color::DarkGray)));
    lore_lines.push(add_pair("ENCRYPTION", "AES-4096 MIL-SPEC", Style::default().fg(Color::DarkGray)));
    lore_lines.push(add_pair("UPLINK", "ESTABLISHED (74ms)", Style::default().fg(Color::DarkGray)));

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(sys_lines.len() as u16 + 2),
            Constraint::Length(hw_lines.len() as u16 + 2),
            Constraint::Min(0),
        ])
        .split(area);

    let sys_para = Paragraph::new(sys_lines).block(crate::ui::shared::block_with_title("■ SYSTEM IDENTIFICATION"));
    f.render_widget(sys_para, chunks[0]);

    let hw_para = Paragraph::new(hw_lines).block(crate::ui::shared::block_with_title("■ HARDWARE"));
    f.render_widget(hw_para, chunks[1]);
    
    let lore_para = Paragraph::new(lore_lines).block(crate::ui::shared::block_with_title("■ CLASSIFIED DATA"));
    f.render_widget(lore_para, chunks[2]);
}
