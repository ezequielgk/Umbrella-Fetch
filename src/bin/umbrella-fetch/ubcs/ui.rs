use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Row, Table},
    Frame,
};

use super::state::UbcsAppState;
use crate::shared::OperativeStatus;
use crate::ubcs::roster::ROSTER;
use crate::ascii::UMBRELLA_LOGO;

fn dark_red_border() -> Style {
    Style::default().fg(Color::Red).add_modifier(Modifier::DIM)
}

fn block_with_title(title: &str, color: Color) -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .border_style(dark_red_border())
        .title(Span::styled(title.to_string(), Style::default().fg(color).add_modifier(Modifier::BOLD)))
}

fn format_status(status: &OperativeStatus) -> Span<'static> {
    match status {
        OperativeStatus::Active => Span::styled("ACTIVE", Style::default().fg(Color::Rgb(0, 100, 0))),
        OperativeStatus::Kia => Span::styled("KIA", Style::default().fg(Color::Red)),
        OperativeStatus::Mia => Span::styled("MIA", Style::default().fg(Color::DarkGray)),
        OperativeStatus::Retired => Span::styled("RET", Style::default().fg(Color::DarkGray)),
    }
}

pub fn draw_ubcs(f: &mut Frame, area: Rect, state: &UbcsAppState) {
    if area.width < 80 || area.height < 30 {
        let text = Paragraph::new("TERMINAL TOO SMALL FOR UBCS FEED. PLEASE RESIZE.")
            .style(Style::default().fg(Color::Red))
            .alignment(ratatui::layout::Alignment::Center);
        let block = Block::default().borders(Borders::ALL).border_style(dark_red_border());
        f.render_widget(block, area);
        let centered_rect = Rect { x: area.x, y: area.y + area.height / 2, width: area.width, height: 1 };
        f.render_widget(text, centered_rect);
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(19),     // Top Zone
            Constraint::Min(10),        // Bottom Zone
            Constraint::Length(1),      // Footer
        ])
        .split(area);

    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(chunks[0]);

    draw_top_left(f, top_chunks[0]);
    draw_top_right(f, top_chunks[1]);

    if state.show_detail {
        let bottom_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
            .split(chunks[1]);
            
        draw_roster_table(f, bottom_chunks[0], state);
        draw_operative_detail(f, bottom_chunks[1], state);
    } else {
        draw_roster_table(f, chunks[1], state);
    }

    // Footer
    let footer_text = "↑↓ NAVIGATE   ENTER DETAIL   f FILTER SQUAD   s FILTER STATUS   p SORT PRICE   q QUIT";
    f.render_widget(Paragraph::new(Span::styled(footer_text, Style::default().fg(Color::DarkGray))).alignment(ratatui::layout::Alignment::Center), chunks[2]);
}

fn draw_top_left(f: &mut Frame, area: Rect) {
    let mut lines = Vec::new();
    
    for l in UMBRELLA_LOGO {
        lines.push(Line::from(Span::styled(*l, Style::default().fg(Color::Red))));
    }
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled("U B C S", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))));
    lines.push(Line::from(Span::styled("UMBRELLA BIOHAZARD COUNTERMEASURE SERVICE", Style::default().fg(Color::Gray))));
    lines.push(Line::from(Span::styled("EST. 1998 | COVERT OPS DIVISION", Style::default().fg(Color::DarkGray))));
    
    let paragraph = Paragraph::new(lines).block(block_with_title("■ U.B.C.S. DATABASE", Color::Red)).alignment(ratatui::layout::Alignment::Center);
    f.render_widget(paragraph, area);
}

fn draw_top_right(f: &mut Frame, area: Rect) {
    let mut active = 0;
    let mut kia = 0;
    let mut mia = 0;
    for op in ROSTER.iter() {
        match op.status {
            OperativeStatus::Active => active += 1,
            OperativeStatus::Kia => kia += 1,
            OperativeStatus::Mia => mia += 1,
            OperativeStatus::Retired => {}
        }
    }
    
    let mut lines = Vec::new();
    let gray = Style::default().fg(Color::Gray);
    let dark_gray = Style::default().fg(Color::DarkGray);
    
    let mut add_pair = |key: &str, val: Span<'static>| {
        lines.push(Line::from(vec![
            Span::styled(format!("{:<12} ", key), dark_gray),
            val,
        ]));
    };

    add_pair("COMMANDER", Span::styled("Col. Sergei Vladimir", gray));
    add_pair("DIVISION", Span::styled("Eastern European Ops", gray));
    add_pair("MISSION", Span::styled("Outbreak Containment & Asset Recovery", gray));
    add_pair("TOTAL OPS", Span::styled("47 registered operations", gray));
    add_pair("MORTALITY", Span::styled("38% avg.", Style::default().fg(Color::Red)));
    add_pair("CONTRACT", Span::styled("Mercenary — renewable 6mo", gray));
    
    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled(format!("● ACTIVE {}   ", active), Style::default().fg(Color::Rgb(0, 100, 0))),
        Span::styled(format!("● KIA {}   ", kia), Style::default().fg(Color::Red)),
        Span::styled(format!("● MIA {}", mia), Style::default().fg(Color::DarkGray)),
    ]));
    
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled("\"We are the expendable cleaners. The necessary evil.\"", Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC))));

    f.render_widget(Paragraph::new(lines).block(block_with_title("■ UNIT OVERVIEW", Color::Red)), area);
}

fn draw_roster_table(f: &mut Frame, area: Rect, state: &UbcsAppState) {
    let header = Row::new(vec!["RANK", "NAME", "SQUAD", "SPECIALITY", "STATUS", "PRICE/OP"])
        .style(Style::default().fg(Color::DarkGray).add_modifier(Modifier::BOLD))
        .bottom_margin(1);

    let filtered = state.get_filtered_roster();
    let mut rows = Vec::new();
    
    for (i, op) in filtered.iter().enumerate() {
        let mut row_style = Style::default().fg(Color::Gray);
        if i == state.selected_index {
            row_style = row_style.bg(Color::Rgb(26, 0, 0)).add_modifier(Modifier::BOLD);
        }

        let row = Row::new(vec![
            Span::styled(op.rank, row_style),
            Span::styled(op.name, row_style),
            Span::styled(op.squad, row_style),
            Span::styled(op.speciality, row_style),
            format_status(&op.status),
            Span::styled(format!("${}", op.price_usd), row_style),
        ]).style(row_style);
        
        rows.push(row);
    }

    let title = format!("■ ROSTER [SQUAD: {} | STATUS: {} | SORT: {}]", 
        state.filter_squad.unwrap_or("ALL"),
        match state.filter_status {
            Some(OperativeStatus::Active) => "ACTIVE",
            Some(OperativeStatus::Kia) => "KIA",
            Some(OperativeStatus::Mia) => "MIA",
            Some(OperativeStatus::Retired) => "RETIRED",
            None => "ALL"
        },
        match state.sort_price_asc {
            Some(true) => "PRICE ▲",
            Some(false) => "PRICE ▼",
            None => "DEFAULT"
        }
    );

    let table = Table::new(rows, [
        Constraint::Percentage(15),
        Constraint::Percentage(25),
        Constraint::Percentage(10),
        Constraint::Percentage(25),
        Constraint::Percentage(10),
        Constraint::Percentage(15),
    ])
    .header(header)
    .block(block_with_title(&title, Color::Red));

    f.render_widget(table, area);
}

fn draw_operative_detail(f: &mut Frame, area: Rect, state: &UbcsAppState) {
    let filtered = state.get_filtered_roster();
    if filtered.is_empty() { return; }
    
    let op = filtered[state.selected_index];
    let mut lines = Vec::new();
    let dark_gray = Style::default().fg(Color::DarkGray);
    let gray = Style::default().fg(Color::Gray);
    let red = Style::default().fg(Color::Red);

    lines.push(Line::from(Span::styled(op.name, Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))));
    lines.push(Line::from(vec![Span::styled(op.rank, gray), Span::styled(format!(" | SQUAD: {}", op.squad), dark_gray)]));
    lines.push(Line::from(Span::styled(format!("ORIGIN: {}", op.origin), dark_gray)));
    lines.push(Line::from(""));
    
    let draw_stat = |label: &str, val: u8| -> Line<'static> {
        let block_count = (val as usize * 10) / 100;
        let full = "█".repeat(block_count);
        let empty = "░".repeat(10 - block_count);
        Line::from(vec![
            Span::styled(format!("{:<8} [", label), dark_gray),
            Span::styled(full, red),
            Span::styled(empty, dark_gray),
            Span::styled(format!("] {:>3}", val), gray),
        ])
    };

    lines.push(draw_stat("STR", op.stats.strength));
    lines.push(draw_stat("AGI", op.stats.agility));
    lines.push(draw_stat("INTEL", op.stats.intelligence));
    lines.push(draw_stat("ENDUR", op.stats.endurance));
    
    lines.push(Line::from(""));
    lines.push(Line::from(vec![Span::styled("SPECIALITY: ", dark_gray), Span::styled(op.speciality, gray)]));
    lines.push(Line::from(vec![Span::styled("WEAPON: ", dark_gray), Span::styled(op.weapon, gray)]));
    
    lines.push(Line::from(""));
    lines.push(Line::from(vec![Span::styled("STATUS: ", dark_gray), format_status(&op.status)]));
    lines.push(Line::from(vec![Span::styled("CONTRACT: ", dark_gray), Span::styled(format!("${}/OP", op.price_usd), gray)]));
    
    let ops_count = (op.name.len() * 3 + (op.stats.strength as usize)) % 40;
    lines.push(Line::from(vec![Span::styled("MISSIONS COMPLETED: ", dark_gray), Span::styled(ops_count.to_string(), gray)]));

    f.render_widget(Paragraph::new(lines).block(block_with_title("■ DOSSIER", Color::Red)), area);
}
