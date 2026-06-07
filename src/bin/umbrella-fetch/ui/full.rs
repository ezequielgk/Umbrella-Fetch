use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::system_info::SystemInfo;
use crate::config::Config;
use super::shared::{block_with_title, draw_logo, draw_umbrella_info, draw_bio_assets, draw_threat_monitor};

pub fn draw_full(f: &mut Frame, info: &SystemInfo, config: &Config, is_watch: bool, ticker_offset: usize) {
    let size = f.area();
    f.render_widget(ratatui::widgets::Clear, size);
    
    if size.width < 100 || size.height < 41 {
        let text = Paragraph::new("TERMINAL TOO SMALL FOR FULL SECURE FEED. PLEASE RESIZE.")
            .style(Style::default().fg(Color::Red))
            .alignment(ratatui::layout::Alignment::Center);
        let block = ratatui::widgets::Block::default().borders(ratatui::widgets::Borders::ALL).border_style(super::shared::dark_red_border());
        f.render_widget(block, size);
        let centered_rect = Rect { x: size.x, y: size.y + size.height / 2, width: size.width, height: 1 };
        f.render_widget(text, centered_rect);
        return;
    }
    
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(15),
            Constraint::Length(16),
            Constraint::Length(9),
            Constraint::Length(1)
        ])
        .split(size);
        
    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(22), Constraint::Percentage(39), Constraint::Percentage(39)])
        .split(main_chunks[0]);
        
    let mid_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_chunks[1]);
        
    let bot_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(25), Constraint::Percentage(50)])
        .split(main_chunks[2]);
        
    draw_logo(f, top_chunks[0], is_watch);
    draw_sys_info_full(f, top_chunks[1], info, config);
    draw_hardware_and_network(f, top_chunks[2], info);
    
    draw_umbrella_info(f, mid_chunks[0], config);
    draw_bio_assets(f, mid_chunks[1], config);
    
    draw_threat_monitor(f, bot_chunks[0], config);
    draw_storage(f, bot_chunks[1], info);
    draw_field_log(f, bot_chunks[2], config);
    
    draw_ticker(f, main_chunks[3], ticker_offset);
}

fn draw_sys_info_full(f: &mut Frame, area: Rect, info: &SystemInfo, config: &Config) {
    let mut lines = Vec::new();
    
    let mut add_pair = |key: &str, val: &str, val_style: Style| {
        lines.push(Line::from(vec![
            Span::styled(format!("{:<15} → ", key), Style::default().fg(Color::DarkGray)),
            Span::styled(val.to_string(), val_style),
        ]));
    };

    let gray = Style::default().fg(Color::Gray);
    let red = Style::default().fg(Color::Red);
    
    add_pair("USER", &info.user, gray);
    add_pair("CLEARANCE", &config.app.clearance, red);
    add_pair("HOSTNAME", &info.hostname, gray);
    add_pair("OS", &info.os, gray);
    add_pair("KERNEL", &info.kernel, gray);
    add_pair("UPTIME", &info.uptime, gray);
    add_pair("SHELL", &info.shell, gray);
    add_pair("PKGS", &info.pkgs, gray);
    add_pair("DISPLAY", &info.display, gray);
    add_pair("WM", &info.wm, gray);
    
    let paragraph = Paragraph::new(lines)
        .block(block_with_title("■ SYSTEM IDENTIFICATION"));
        
    f.render_widget(paragraph, area);
}

fn draw_hardware_and_network(f: &mut Frame, area: Rect, info: &SystemInfo) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(6), Constraint::Min(0)])
        .split(area);

    let gray = Style::default().fg(Color::Gray);
    let dark_gray = Style::default().fg(Color::DarkGray);
    let red = Style::default().fg(Color::Red);
    
    // Hardware
    let mut hw_lines = Vec::new();
    hw_lines.push(Line::from(vec![
        Span::styled("CPU  ", dark_gray),
        Span::styled(format!("{} ({} CORES)", &info.cpu_model, info.cpu_cores), gray),
    ]));
    
    hw_lines.push(Line::from(vec![
        Span::styled("RAM  ", dark_gray),
        Span::styled(format!("{:.1} / {:.1} GB", info.ram_used_gb, info.ram_total_gb), gray),
    ]));
    
    hw_lines.push(Line::from(vec![
        Span::styled("SWAP ", dark_gray),
        Span::styled(format!("{:.1} / {:.1} GB", info.swap_used_gb, info.swap_total_gb), gray),
    ]));
    
    hw_lines.push(Line::from(vec![
        Span::styled("GPU  ", dark_gray),
        Span::styled(&info.gpu_info, gray),
    ]));
    
    let hw_para = Paragraph::new(hw_lines).block(block_with_title("■ HARDWARE"));
    f.render_widget(hw_para, chunks[0]);
    
    // Network
    let mut net_lines = Vec::new();
    net_lines.push(Line::from(vec![
        Span::styled("NET  ", dark_gray),
        Span::styled(crate::config::UMBRELLA_NETWORK, gray),
    ]));
    
    net_lines.push(Line::from(vec![
        Span::styled("IP   ", dark_gray),
        Span::styled(&info.net_ip, gray),
    ]));
    
    net_lines.push(Line::from(vec![
        Span::styled("DATA ", dark_gray),
        Span::styled(format!("TX: {:.1} MB | RX: {:.1} MB", info.net_tx_mb, info.net_rx_mb), gray),
    ]));
    
    net_lines.push(Line::from(vec![
        Span::styled("PING ", dark_gray),
        Span::styled(&info.net_ping, gray),
    ]));
    
    if info.vpn_active {
        net_lines.push(Line::from(Span::styled("▓ VPN ACTIVE", red)));
    } else {
        net_lines.push(Line::from(Span::styled("▓ NO VPN", dark_gray)));
    }

    let net_para = Paragraph::new(net_lines).block(block_with_title("■ NETWORK"));
    f.render_widget(net_para, chunks[1]);
}

fn draw_storage(f: &mut Frame, area: Rect, info: &SystemInfo) {
    let mut lines = Vec::new();
    
    for (mount, used, total) in &info.partitions {
        let ratio = if *total > 0 { *used as f64 / *total as f64 } else { 0.0 };
        let bar_len = (10.0 * ratio).round() as usize;
        let full = "█".repeat(bar_len);
        let empty = "░".repeat(10 - bar_len);
        
        lines.push(Line::from(Span::styled(format!("{}: {}G/{}G", mount, used, total), Style::default().fg(Color::Gray))));
        lines.push(Line::from(vec![
            Span::styled("[", Style::default().fg(Color::DarkGray)),
            Span::styled(full, Style::default().fg(Color::Red)),
            Span::styled(empty, Style::default().fg(Color::DarkGray)),
            Span::styled("]", Style::default().fg(Color::DarkGray)),
        ]));
    }
    
    let paragraph = Paragraph::new(lines)
        .block(block_with_title("■ STORAGE"));
        
    f.render_widget(paragraph, area);
}

fn draw_field_log(f: &mut Frame, area: Rect, config: &Config) {
    let mut lines = Vec::new();
    for entry in config.active_field_logs.iter() {
        lines.push(Line::from(vec![
            Span::styled(format!("[{}] ", entry.date), Style::default().fg(Color::Red)),
            Span::styled(entry.entry, Style::default().fg(Color::Gray)),
        ]));
    }
    
    let paragraph = Paragraph::new(lines)
        .block(block_with_title("■ FIELD LOG"));
        
    f.render_widget(paragraph, area);
}

fn draw_ticker(f: &mut Frame, area: Rect, offset: usize) {
    let full_text = crate::lore::TICKER_MESSAGES.join(" ");
    
    let repeated = format!("{} {} ", full_text, full_text);
    let chars: Vec<char> = repeated.chars().collect();
    
    if chars.is_empty() { return; }
    
    let safe_offset = offset % (chars.len() / 2);
    let view_text: String = chars[safe_offset..].iter().collect();
    
    let paragraph = Paragraph::new(Span::styled(view_text, Style::default().fg(Color::DarkGray).add_modifier(Modifier::DIM)));
    f.render_widget(paragraph, area);
}
