use ratatui::{
    layout::{Constraint, Direction, Layout, Rect, Alignment},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Row, Table},
    Frame,
};

use crate::shared::OperativeStatus;
use super::state::UssAppState;
use super::unit_info::{UNIT_INFO, ACTIVE_TARGETS, COVERT_OPS_LOG, TargetPriority, OpResult};
use super::roster::ROSTER;

// Standardized Colors for USS
const BORDER_DARK: Color = Color::DarkGray;
const ACCENT_RED: Color = Color::Red;
const TEXT_GOLD: Color = Color::Yellow;

fn uss_border() -> Style {
    Style::default().fg(Color::Red).add_modifier(Modifier::DIM)
}

fn block_uss(title: &str) -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .border_style(uss_border())
        .title(Span::styled(title.to_string(), Style::default().fg(ACCENT_RED).add_modifier(Modifier::BOLD)))
}

fn format_status(status: &OperativeStatus) -> Span<'static> {
    match status {
        OperativeStatus::Active => Span::styled("ACTIVE", Style::default().fg(Color::Green)),
        OperativeStatus::Kia => Span::styled("KIA", Style::default().fg(ACCENT_RED)),
        OperativeStatus::Mia => Span::styled("MIA", Style::default().fg(Color::DarkGray)),
        OperativeStatus::Retired => Span::styled("RETIRED", Style::default().fg(Color::Gray)),
    }
}

pub fn draw_uss(f: &mut Frame, area: Rect, state: &UssAppState) {
    if area.width < 80 || area.height < 36 {
        let text = Paragraph::new("TERMINAL TOO SMALL FOR CLASSIFIED FEED. PLEASE RESIZE.")
            .style(Style::default().fg(ACCENT_RED))
            .alignment(Alignment::Center);
        let block = Block::default().borders(Borders::ALL).border_style(uss_border());
        f.render_widget(block, area);
        let centered_rect = Rect { x: area.x, y: area.y + area.height / 2, width: area.width, height: 1 };
        f.render_widget(text, centered_rect);
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),      // Classified bar
            Constraint::Length(15),     // Top Zone (13-line logo + 2 borders)
            Constraint::Length(1),      // Spacer/Roster Label
            Constraint::Min(10),        // Mid Zone (Cards/Roster)
            Constraint::Length(8),      // Bot Zone
            Constraint::Length(1),      // Footer
        ])
        .split(area);

    // Classified Bar
    let classified_text = "██ USS — UMBRELLA SECURITY SERVICE — EYES ONLY — CLEARANCE LVL-5 REQUIRED ██";
    f.render_widget(
        Paragraph::new(classified_text)
            .style(Style::default().fg(Color::Red).add_modifier(Modifier::REVERSED))
            .alignment(Alignment::Center),
        chunks[0]
    );

    // Top Zone
    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(34), // Fixed length for 32-char Logo
            Constraint::Min(40),    // Unit Info
            Constraint::Length(30), // Ops Record
        ])
        .split(chunks[1]);

    draw_logo(f, top_chunks[0]);
    draw_unit_info(f, top_chunks[1]);
    draw_ops_record(f, top_chunks[2]);

    // Roster Label
    f.render_widget(
        Paragraph::new(Span::styled("■ COVERT OPERATIVES ROSTER", Style::default().fg(ACCENT_RED).add_modifier(Modifier::BOLD))),
        chunks[2]
    );

    // Mid Zone
    if state.show_detail {
        let mid_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
            .split(chunks[3]);
        draw_roster_table(f, mid_chunks[0], state);
        draw_operative_detail(f, mid_chunks[1], state);
    } else {
        draw_roster_table(f, chunks[3], state);
    }

    // Bot Zone
    let bot_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(45), // Active Targets
            Constraint::Percentage(35), // Covert Ops
            Constraint::Percentage(20), // Clearance
        ])
        .split(chunks[4]);

    draw_active_targets(f, bot_chunks[0]);
    draw_covert_ops(f, bot_chunks[1]);
    draw_clearance(f, bot_chunks[2]);

    // Footer
    let footer_text = " ↑↓ NAVIGATE   ENTER DETAIL   f FILTER SQUAD   s FILTER STATUS   p SORT PRICE   q QUIT ";
    f.render_widget(
        Paragraph::new(Span::styled(footer_text, Style::default().fg(Color::DarkGray).add_modifier(Modifier::BOLD)))
            .alignment(Alignment::Center),
        chunks[5]
    );
}

fn draw_logo(f: &mut Frame, area: Rect) {
    let logo = vec![
        r#"           :.           "#,
        r#"          :-:.          "#,
        r#"       ¸ .---:. ¸       "#,
        r#"     _/"..---: ."\_     "#,
        r#"    —" d@L.-:.d@L ¹.    "#,
        r#"  .:-:.'¹@\..d@¹".:-:.  "#,
        r#":-------:..::..:-------:"#,
        r#"  .:-:.:_q/..\@_:.:::.  "#,
        r#"    ]_ #@/.-:.@@* _.    "#,
        r#"     "\_..--::.._/"     "#,
        r#"       " :----. '       "#,
        r#"          :--.          "#,
        r#"           :.           "#,
    ];
    let mut lines = Vec::new();
    for l in logo {
        lines.push(Line::from(Span::styled(l, Style::default().fg(Color::DarkGray).add_modifier(Modifier::BOLD))));
    }
    
    let paragraph = Paragraph::new(lines).block(block_uss("").borders(Borders::ALL)).alignment(Alignment::Center);
    f.render_widget(paragraph, area);
}

fn draw_unit_info(f: &mut Frame, area: Rect) {
    let mut active = 0;
    let mut kia = 0;
    let mut mia = 0;
    let mut retired = 0;
    for op in ROSTER.iter() {
        match op.status {
            OperativeStatus::Active => active += 1,
            OperativeStatus::Kia => kia += 1,
            OperativeStatus::Mia => mia += 1,
            OperativeStatus::Retired => retired += 1,
        }
    }

    let mut lines = Vec::new();
    let dark_gray = Style::default().fg(Color::Gray);
    let label_style = Style::default().fg(Color::DarkGray);

    let mut add_pair = |key: &str, val: Span<'static>| {
        lines.push(Line::from(vec![
            Span::styled(format!("{:<14} ", key), label_style),
            val,
        ]));
    };

    add_pair("COMMANDER", Span::styled(UNIT_INFO.commander, dark_gray));
    add_pair("CODENAME", Span::styled(UNIT_INFO.codename, dark_gray));
    add_pair("DIVISION", Span::styled(UNIT_INFO.division, dark_gray));
    add_pair("FOUNDED", Span::styled(UNIT_INFO.founded, dark_gray));
    add_pair("MISSION", Span::styled(UNIT_INFO.mission, dark_gray));
    add_pair("CLEARANCE", Span::styled(UNIT_INFO.clearance, Style::default().fg(ACCENT_RED)));
    add_pair("DENIABILITY", Span::styled(UNIT_INFO.deniability, Style::default().fg(ACCENT_RED)));
    add_pair("CONTRACT", Span::styled(UNIT_INFO.contract, dark_gray));

    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled(format!("● ACTIVE {}   ", active), Style::default().fg(Color::Green)),
        Span::styled(format!("● KIA {}   ", kia), Style::default().fg(ACCENT_RED)),
        Span::styled(format!("● MIA {}   ", mia), Style::default().fg(Color::DarkGray)),
        Span::styled(format!("● RETIRED {}", retired), Style::default().fg(Color::Gray)),
    ]));

    f.render_widget(Paragraph::new(lines).block(block_uss("■ UNIT OVERVIEW — USS ALPHA")), area);
}

fn draw_ops_record(f: &mut Frame, area: Rect) {
    let mut lines = Vec::new();
    let label_style = Style::default().fg(Color::DarkGray);

    lines.push(Line::from(vec![Span::styled("TOTAL OPS:    ", label_style), Span::styled(UNIT_INFO.total_ops.to_string(), Style::default().fg(Color::Gray))]));
    lines.push(Line::from(vec![Span::styled("SUCCESS:      ", label_style), Span::styled(UNIT_INFO.ops_success.to_string(), Style::default().fg(Color::Green))]));
    lines.push(Line::from(vec![Span::styled("FAILED:       ", label_style), Span::styled(UNIT_INFO.ops_failed.to_string(), Style::default().fg(ACCENT_RED))]));
    lines.push(Line::from(vec![Span::styled("SURVIVAL AVG: ", label_style), Span::styled(UNIT_INFO.survival_avg, Style::default().fg(ACCENT_RED))]));
    lines.push(Line::from(vec![Span::styled("LAST OP:      ", label_style), Span::styled(UNIT_INFO.last_op, Style::default().fg(ACCENT_RED))]));
    lines.push(Line::from(""));
    lines.push(Line::from(vec![Span::styled("TARGETS:      ", label_style), Span::styled("████████", Style::default().fg(Color::DarkGray))]));
    
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(format!("{} — {}", UNIT_INFO.board_quote, UNIT_INFO.board_date), Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC))));

    f.render_widget(Paragraph::new(lines).block(block_uss("■ OPS RECORD")), area);
}

fn draw_roster_table(f: &mut Frame, area: Rect, state: &UssAppState) {
    let header = Row::new(vec!["ID", "CODENAME", "REAL NAME", "SPECIALITY", "STATUS", "PRICE"])
        .style(Style::default().fg(Color::DarkGray).add_modifier(Modifier::BOLD))
        .bottom_margin(1);

    let filtered = state.get_filtered_roster();
    let mut rows = Vec::new();

    for (i, op) in filtered.iter().enumerate() {
        let mut row_style = Style::default().fg(Color::Gray);
        if i == state.selected_index {
            row_style = row_style.fg(Color::White).add_modifier(Modifier::BOLD).add_modifier(Modifier::REVERSED);
        }

        let real_name_span = match op.real_name {
            Some(name) => Span::styled(name, Style::default().fg(Color::DarkGray)),
            None => Span::styled("██████████", Style::default().fg(Color::DarkGray)),
        };

        let row = Row::new(vec![
            Span::styled(op.alpha_id, row_style),
            Span::styled(op.codename, row_style.fg(Color::White)),
            real_name_span,
            Span::styled(op.speciality, row_style),
            format_status(&op.status),
            Span::styled(format!("${}", op.price_usd), Style::default().fg(TEXT_GOLD)),
        ]).style(row_style);

        rows.push(row);
    }

    let title = format!("■ ROSTER [FILTER: {} | STATUS: {} | SORT: {}]", 
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
        Constraint::Percentage(10),
        Constraint::Percentage(20),
        Constraint::Percentage(20),
        Constraint::Percentage(20),
        Constraint::Percentage(15),
        Constraint::Percentage(15),
    ])
    .header(header)
    .block(block_uss(&title));

    f.render_widget(table, area);
}

fn draw_operative_detail(f: &mut Frame, area: Rect, state: &UssAppState) {
    let filtered = state.get_filtered_roster();
    if filtered.is_empty() { return; }

    let op = filtered[state.selected_index];
    let mut lines = Vec::new();
    let dark_gray = Style::default().fg(Color::DarkGray);
    let value_style = Style::default().fg(Color::Gray);

    lines.push(Line::from(vec![
        Span::styled(op.codename, Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        Span::styled(format!(" [{}]", op.alpha_id), Style::default().fg(ACCENT_RED)),
    ]));

    match op.real_name {
        Some(name) => lines.push(Line::from(vec![Span::styled("IDENTITY: ", dark_gray), Span::styled(name, value_style)])),
        None => lines.push(Line::from(vec![Span::styled("IDENTITY CLASSIFIED: ", dark_gray), Span::styled("████████████", Style::default().fg(Color::DarkGray))])),
    }

    lines.push(Line::from(Span::styled(format!("ORIGIN: {}", op.origin), dark_gray)));
    lines.push(Line::from(""));

    let draw_stat = |label: &str, val: u8| -> Line<'static> {
        let block_count = (val as usize * 10) / 100;
        let full = "█".repeat(block_count);
        let empty = "░".repeat(10 - block_count);
        Line::from(vec![
            Span::styled(format!("{:<8} [", label), dark_gray),
            Span::styled(full, Style::default().fg(ACCENT_RED)),
            Span::styled(empty, dark_gray),
            Span::styled(format!("] {:>3}", val), value_style),
        ])
    };

    lines.push(draw_stat("STR", op.stats.strength));
    lines.push(draw_stat("AGI", op.stats.agility));
    lines.push(draw_stat("INTEL", op.stats.intelligence));
    lines.push(draw_stat("ENDUR", op.stats.endurance));

    lines.push(Line::from(""));
    lines.push(Line::from(vec![Span::styled("SPECIALITY: ", dark_gray), Span::styled(op.speciality, value_style)]));
    lines.push(Line::from(vec![Span::styled("WEAPON: ", dark_gray), Span::styled(op.weapon, value_style)]));

    lines.push(Line::from(""));
    lines.push(Line::from(vec![Span::styled("STATUS: ", dark_gray), format_status(&op.status)]));
    lines.push(Line::from(vec![Span::styled("CONTRACT: ", dark_gray), Span::styled(format!("${}/OP", op.price_usd), Style::default().fg(TEXT_GOLD))]));
    lines.push(Line::from(vec![Span::styled("OPS RECORD: ", dark_gray), Span::styled(format!("{} ops ({} survived)", op.ops_total, op.ops_survived), value_style)]));

    f.render_widget(Paragraph::new(lines).block(block_uss("■ DOSSIER")), area);
}

fn draw_active_targets(f: &mut Frame, area: Rect) {
    let mut lines = Vec::new();
    let dark_gray = Style::default().fg(Color::DarkGray);

    for target in ACTIVE_TARGETS {
        let prio_color = match target.priority {
            TargetPriority::High => ACCENT_RED,
            TargetPriority::Medium => TEXT_GOLD,
            TargetPriority::Low => Color::DarkGray,
        };

        let name_span = if target.redacted {
            Span::styled(target.name, Style::default().fg(Color::DarkGray))
        } else {
            Span::styled(target.name, Style::default().fg(Color::Gray))
        };

        lines.push(Line::from(vec![
            Span::styled(format!("{:02} | ", target.number), dark_gray),
            name_span,
            Span::styled(" | ", dark_gray),
            Span::styled(match target.priority {
                TargetPriority::High => "HIGH PRIO",
                TargetPriority::Medium => "MED PRIO",
                TargetPriority::Low => "LOW PRIO",
            }, Style::default().fg(prio_color)),
        ]));
        lines.push(Line::from(Span::styled("----------------------------------------", Style::default().fg(BORDER_DARK))));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled("TARGETS REDACTED PER BOARD ORDER — TS/SCI", Style::default().fg(Color::DarkGray))));

    f.render_widget(Paragraph::new(lines).block(block_uss("■ ACTIVE TARGETS — CLASSIFIED")), area);
}

fn draw_covert_ops(f: &mut Frame, area: Rect) {
    let mut lines = Vec::new();
    let dark_gray = Style::default().fg(Color::DarkGray);

    for op in COVERT_OPS_LOG {
        let (res_str, res_color, modifier) = match op.result {
            OpResult::Success => ("SUCCESS", Color::Green, Modifier::empty()),
            OpResult::Failed => ("FAILED ", ACCENT_RED, Modifier::empty()),
            OpResult::Ongoing => ("ONGOING", TEXT_GOLD, Modifier::RAPID_BLINK),
        };

        lines.push(Line::from(vec![
            Span::styled(format!("[{}] ", op.date), dark_gray),
            Span::styled(res_str, Style::default().fg(res_color).add_modifier(modifier)),
            Span::styled(format!("  {}", op.summary), Style::default().fg(Color::Gray)),
        ]));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled("FULL RECORD: ", dark_gray),
        Span::styled("████████████████████", Style::default().fg(Color::DarkGray)),
    ]));

    f.render_widget(Paragraph::new(lines).block(block_uss("■ COVERT OPS LOG")), area);
}

fn draw_clearance(f: &mut Frame, area: Rect) {
    let mut lines = Vec::new();
    lines.push(Line::from(Span::styled("LVL 1 - Public", Style::default().fg(Color::DarkGray))));
    lines.push(Line::from(Span::styled("LVL 2 - Corporate", Style::default().fg(Color::DarkGray))));
    lines.push(Line::from(Span::styled("LVL 3 - UBCS", Style::default().fg(Color::Gray))));
    lines.push(Line::from(Span::styled("LVL 4 - Research", Style::default().fg(Color::Gray))));
    lines.push(Line::from(Span::styled("LVL 5 - USS/Board", Style::default().fg(ACCENT_RED).add_modifier(Modifier::BOLD))));
    
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled("YOUR LEVEL:", Style::default().fg(Color::DarkGray))));
    lines.push(Line::from(Span::styled("LEVEL 5 ▓", Style::default().fg(ACCENT_RED).add_modifier(Modifier::RAPID_BLINK))));

    f.render_widget(Paragraph::new(lines).block(block_uss("■ CLEARANCE INDEX")), area);
}
