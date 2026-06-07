use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use super::strains::StrainProfile;
use super::state::VirusAppState;

fn age_to_color(age: u8, strain: &StrainProfile) -> Color {
    let t = (age as f32 / 12.0).min(1.0);
    let (r, g, b) = strain.color;
    Color::Rgb(
        (20.0 + t * (r as f32 - 20.0)) as u8,
        (g as f32 * t * 0.8) as u8,
        (b as f32 * t * 0.8) as u8,
    )
}

fn dark_red_border() -> Style {
    Style::default().fg(Color::Red).add_modifier(Modifier::DIM)
}

fn block_with_title(title: &str, color: Color) -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .border_style(dark_red_border())
        .title(Span::styled(title.to_string(), Style::default().fg(color).add_modifier(Modifier::BOLD)))
}

pub fn draw_virus(f: &mut Frame, area: Rect, state: &VirusAppState) {
    if area.width < 80 || area.height < 25 {
        let text = Paragraph::new("TERMINAL TOO SMALL FOR VIRUS SIMULATION. PLEASE RESIZE.")
            .style(Style::default().fg(Color::Red))
            .alignment(ratatui::layout::Alignment::Center);
        let block = Block::default().borders(Borders::ALL).border_style(dark_red_border());
        f.render_widget(block, area);
        let centered_rect = Rect { x: area.x, y: area.y + area.height / 2, width: area.width, height: 1 };
        f.render_widget(text, centered_rect);
        return;
    }

    let s = state.sim.strain;
    let base_color = Color::Rgb(s.color.0, s.color.1, s.color.2);
    
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), 
            Constraint::Min(0),   
            Constraint::Length(1), 
            Constraint::Length(5), 
        ])
        .split(area);
        
    f.render_widget(Paragraph::new(Span::styled(format!("■ {} SIMULATION MODULE", s.name), Style::default().fg(base_color).add_modifier(Modifier::BOLD))), main_chunks[0]);
    
    let mid_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(75), Constraint::Percentage(25)])
        .split(main_chunks[1]);
        
    let mut canvas_lines = Vec::new();
    for y in 0..state.sim.rows {
        let mut row_spans = Vec::new();
        for x in 0..state.sim.cols {
            let i = y * state.sim.cols + x;
            if state.sim.grid[i] {
                let color = age_to_color(state.sim.age[i], s);
                let symbol = match state.sim.age[i] {
                    0..=3 => "░",
                    4..=7 => "▒",
                    8..=11 => "▓",
                    _ => "█",
                };
                row_spans.push(Span::styled(symbol, Style::default().fg(color)));
            } else {
                row_spans.push(Span::raw(" "));
            }
        }
        canvas_lines.push(Line::from(row_spans));
    }
    
    let canvas_para = Paragraph::new(canvas_lines).block(Block::default().borders(Borders::ALL).border_style(dark_red_border()));
    f.render_widget(canvas_para, mid_chunks[0]);
    
    let mut stats_lines = Vec::new();

    let dark_gray = Style::default().fg(Color::DarkGray);
    
    let mut add_pair = |key: &str, val: &str| {
        stats_lines.push(Line::from(vec![
            Span::styled(format!("{:<12} ", key), dark_gray),
            Span::styled(val.to_string(), base_color),
        ]));
    };
    
    add_pair("CLASS", s.class);
    add_pair("HOST", s.host);
    add_pair("INCUBATION", s.incubation);
    add_pair("LETHALITY", s.lethality);
    add_pair("ORIGIN", s.origin);
    add_pair("CURE", s.cure);
    add_pair("THREAT", s.threat);
    
    let stats_para = Paragraph::new(stats_lines).block(block_with_title("■ VIRAL PROFILE", base_color));
    f.render_widget(stats_para, mid_chunks[1]);
    
    let blink = state.tick % 20 < 10;
    let mut info_lines = Vec::new();
    
    info_lines.push(Line::from(vec![
        Span::styled("PHASE ", dark_gray),
        Span::styled(format!("[{}]", state.sim.current_phase()), Style::default().fg(base_color).add_modifier(Modifier::BOLD)),
    ]));
    
    let mut_status = if state.sim.spread_pct() > 60 {
        if blink { "ACTIVE" } else { "      " }
    } else {
        "STABLE"
    };
    
    info_lines.push(Line::from(vec![
        Span::styled("HOST CELLS ", dark_gray),
        Span::styled(format!("{:<6}", state.sim.alive_count()), base_color),
        Span::styled(" | SPREAD ", dark_gray),
        Span::styled(format!("{:>3}%", state.sim.spread_pct()), base_color),
        Span::styled(" | MUTATION ", dark_gray),
        Span::styled(mut_status, base_color),
    ]));
    
    info_lines.push(Line::from(vec![
        Span::styled("CONTAINMENT ", dark_gray),
        Span::styled("[FAILED]", Style::default().fg(Color::Red)),
        Span::styled(" | LETHALITY ", dark_gray),
        Span::styled(s.lethality, base_color),
    ]));
    
    let info_para = Paragraph::new(info_lines).block(Block::default().borders(Borders::ALL).border_style(dark_red_border()));
    f.render_widget(info_para, main_chunks[3]);
}
