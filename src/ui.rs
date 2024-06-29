use crate::app::*;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

fn metric_ui(frame: &mut Frame, grid: Vec<Vec<Rect>>, app: &App) {
    let block_style = Style::default().fg(Color::Green);

    let metric_contents = [
        "Antenna",
        "Signal Strength",
        "Signal Noise",
        "Channel Freq",
        "Channel Flags",
        "Data Rate",
    ];
    for (idx, i) in metric_contents.iter().enumerate() {
        let metric_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default());
        let metric = Paragraph::new(Text::styled(*i, block_style))
            .block(metric_block)
            .alignment(Alignment::Center);
        frame.render_widget(metric, grid[idx / 3][idx % 3]);
        let metric_block = Block::default().style(Style::default());
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(50),
                    Constraint::Length(5),
                    Constraint::Percentage(50),
                ]
                .as_ref(),
            )
            .split(grid[idx / 3][idx % 3]);
        let center = layout[1];
        let metric_val = Paragraph::new(Text::styled(app.metrics[idx].clone(), block_style))
            .block(metric_block)
            .alignment(Alignment::Center);
        frame.render_widget(metric_val, center);
    }
}

pub fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(f.size());

    let outer_metric_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    let inner_metric_chunks_top = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
        ])
        .split(outer_metric_chunks[0]);

    let inner_metric_chunks_bottom = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
        ])
        .split(outer_metric_chunks[1]);

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled("GUSV-NET", Style::default().fg(Color::Green)))
        .block(title_block);

    let current_navigation_text = match app.current_screen {
        CurrentScreen::Main => {
            vec![
                Span::styled("q - quit", Style::default().fg(Color::LightYellow)),
                // TODO:  Add in once navigation is handled
                //
                // Span::styled(
                //     " navigation - arrow keys",
                //     Style::default().fg(Color::LightYellow),
                // ),
            ]
        }
        CurrentScreen::Exiting => vec![Span::styled(
            "press q again to exit",
            Style::default().fg(Color::LightRed),
        )],
    };

    let navigation_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);
    f.render_widget(navigation_footer, chunks[2]);
    metric_ui(
        f,
        vec![
            inner_metric_chunks_top.to_vec(),
            inner_metric_chunks_bottom.to_vec(),
        ],
        app,
    )
}
