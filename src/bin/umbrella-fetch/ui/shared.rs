//! Reusable UI components based on `ratatui`.

use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::config::Config;

use crate::ascii::UMBRELLA_LOGO;

/// Defines the standard style for block borders: dark/dim red.
pub fn dark_red_border() -> Style {
    Style::default().fg(Color::Red).add_modifier(Modifier::DIM)
}

/// Highlight style for block titles: bold red text.
pub fn title_style() -> Style {
    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
}

/// Generates a standard corporate `Block` widget with borders and title.
pub fn block_with_title(title: &str) -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .border_style(dark_red_border())
        .title(Span::styled(title.to_string(), title_style()))
}

/// Draws the central or lateral panel with the classic ASCII corporate logo.
pub fn draw_logo(f: &mut Frame, area: Rect, is_watch: bool) {
    let mut lines = Vec::new();
    for logo_line in UMBRELLA_LOGO {
        lines.push(Line::from(Span::styled(*logo_line, Style::default().fg(Color::Red))));
    }
    
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled("UMBRELLA", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))));
    lines.push(Line::from(Span::styled("CORP.", Style::default().fg(Color::DarkGray))));
    lines.push(Line::from(""));
    
    let dot = if is_watch { "● LIVE" } else { "● LIVE" };
    lines.push(Line::from(Span::styled(dot, Style::default().fg(Color::Red))));
    
    let paragraph = Paragraph::new(lines)
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_style(dark_red_border()));
        
    f.render_widget(paragraph, area);
}

/// Renders the descriptive table with the company's organizational structure.
pub fn draw_umbrella_info(f: &mut Frame, area: Rect, config: &Config) {
    let mut lines = Vec::new();
    let gray = Style::default().fg(Color::Gray);
    let red = Style::default().fg(Color::Red);
    
    let mut add_pair = |key: &str, val: &str, val_style: Style| {
        lines.push(Line::from(vec![
            Span::styled(format!("{:<15} → ", key), Style::default().fg(Color::DarkGray)),
            Span::styled(val.to_string(), val_style),
        ]));
    };

    add_pair("FOUNDED", "1968", gray);
    add_pair("HQ", "Europe", gray);
    
    add_pair("MISSION", "Bioweapons & Pharmaceuticals R&D", gray);
    add_pair("DIVISION", &config.app.division, gray);
    add_pair("FACILITY", &config.app.facility_id, gray);
    add_pair("PROJECT", &config.app.project_codename, gray);
    add_pair("PROTOCOL", &config.app.security_protocol, red);
    add_pair("DIRECTIVE", &config.app.current_directive, red);
    add_pair("AUTH", &config.app.auth_code, gray);
    
    add_pair("STATUS", &config.app.system_status, red);
    
    lines.push(Line::from(""));
    lines.push(Line::from(""));
    
    let quote1 = Span::styled("\"Obedience Breeds Discipline,", Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC));
    let quote2 = Span::styled(" Discipline Breeds Unity,", Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC));
    let quote3 = Span::styled(" Unity Breeds Power.\"", Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC));
    
    lines.push(Line::from(quote1));
    lines.push(Line::from(quote2));
    lines.push(Line::from(quote3));

    let paragraph = Paragraph::new(lines)
        .block(block_with_title("■ UMBRELLA CORPORATION"));
        
    f.render_widget(paragraph, area);
}

/// Prints a horizontal bar chart listing "biological assets" and simulated prices.
pub fn draw_bio_assets(f: &mut Frame, area: Rect, config: &Config) {
    let assets: Vec<_> = config.active_bio_assets.iter().collect();
    let max_price = assets.iter().map(|a| a.price).max().unwrap_or(1);
    
    let inner_width = area.width.saturating_sub(2) as usize;
    let bar_max_len = inner_width.saturating_sub(30).max(5);
    
    let mut lines = Vec::new();
    for asset in assets {
        let ratio = asset.price as f64 / max_price as f64;
        let bar_len = ((bar_max_len as f64) * ratio).round() as usize;
        let full = "█".repeat(bar_len);
        let empty = "░".repeat(bar_max_len.saturating_sub(bar_len));
        
        let price_str = format!("${:02},{:03}", asset.price / 1000, asset.price % 1000);
        
        let line = Line::from(vec![
            Span::styled(format!("{:<15} [", asset.name), Style::default().fg(Color::Gray)),
            Span::styled(full, Style::default().fg(Color::Red)),
            Span::styled(empty, Style::default().fg(Color::DarkGray)),
            Span::styled(format!("]   {:<8}", price_str), Style::default().fg(Color::Gray)),
        ]);
        lines.push(line);
        lines.push(Line::from(""));
    }
    
    lines.push(Line::from(Span::styled("PRICES IN USD — DARK MARKET INDEX", Style::default().fg(Color::DarkGray))));
    
    let paragraph = Paragraph::new(lines)
        .block(block_with_title("■ ACTIVE BIO-ASSETS — MARKET INDEX"));
        
    f.render_widget(paragraph, area);
}

/// Renders the global outbreak monitor.
pub fn draw_threat_monitor(f: &mut Frame, area: Rect, config: &Config) {
    let mut lines = Vec::new();
    
    for zone in &config.active_threat_zones {
        let level = zone.level as usize;
        let blocks = "█".repeat(level);
        let empty = "░".repeat(5 - level);
        
        let color = if level >= 4 { Color::Red } else { Color::Gray };
        
        lines.push(Line::from(vec![
            Span::styled(format!("{:<15} ", zone.name), Style::default().fg(Color::Gray)),
            Span::styled(blocks, Style::default().fg(color)),
            Span::styled(empty, Style::default().fg(Color::DarkGray)),
            Span::styled(format!(" LEVEL-{}", level), Style::default().fg(color)),
        ]));
    }
    
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled("LEVEL 5: CONTAINMENT BREACH | LEVEL 1: SECURE", Style::default().fg(Color::DarkGray))));
    
    let paragraph = Paragraph::new(lines).block(block_with_title("■ GLOBAL OUTBREAK INDEX"));
    f.render_widget(paragraph, area);
}
