use ratatui::{
    layout::{Constraint, Direction, Layout, Margin, Rect},
    style::{Style, Stylize},
    text::Line,
    widgets::{
        Block, Borders, List, Padding, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState,
    },
    Frame,
};

use crate::app::App;

pub(crate) fn ui(frame: &mut Frame, app: &mut App) {
    let items: Vec<_> = (1..=app.items_count).map(|n| n.to_string()).collect();

    let [list_area, scrollbar_area, paragraph_area, values_area] = {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(15),
                Constraint::Length(1),
                Constraint::Length(15),
                Constraint::Length(1),
                Constraint::Min(1),
            ])
            .split(frame.size());

        let scrollbar_area = chunks[1].inner(&Margin {
            horizontal: 0,
            vertical: 1,
        });

        [chunks[0], scrollbar_area, chunks[2], chunks[4]]
    };

    let list = List::new(items.clone())
        .highlight_style(Style::default().reversed())
        .block(
            Block::default()
                .title("List")
                .borders(Borders::ALL)
                .padding(Padding::horizontal(1)),
        );
    let viewport_len = list_area.height as usize - 2;
    app.page_size = viewport_len;
    frame.render_stateful_widget(list, list_area, &mut app.list_state);

    if let Some(mut scrollbar_state) = scrollbar_state_from_offset(
        app.items_count,
        list_area.height as usize - 2,
        app.list_state.offset(),
    ) {
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);
        frame.render_stateful_widget(scrollbar, scrollbar_area, &mut scrollbar_state);
    }

    let paragraph = Paragraph::new(items.into_iter().map(Line::from).collect::<Vec<_>>())
        .scroll((app.list_state.offset() as u16, 0))
        .block(
            Block::default()
                .title("Paragraph")
                .borders(Borders::ALL)
                .padding(Padding::horizontal(1)),
        );
    frame.render_widget(paragraph, paragraph_area);

    let values = [
        ("Count", app.items_count),
        ("Viewport", viewport_len),
        ("Offset", app.list_state.offset()),
    ];
    for (i, (label, value)) in values.iter().enumerate() {
        let value_area = Rect {
            y: i as u16 + 1,
            height: 1,
            ..values_area
        };
        frame.render_widget(Paragraph::new(format!("{label}: {value}")), value_area);
    }
}

fn scrollbar_state_from_offset(
    content_length: usize,
    viewport_content_length: usize,
    offset: usize,
) -> Option<ScrollbarState> {
    scrollbar_position_from_offset(content_length, viewport_content_length, offset).map(
        |position| {
            ScrollbarState::new(content_length)
                .viewport_content_length(viewport_content_length)
                .position(position)
        },
    )
}

fn scrollbar_position_from_offset(
    content_length: usize,
    viewport_content_length: usize,
    offset: usize,
) -> Option<usize> {
    let max_offset = content_length.saturating_sub(viewport_content_length);
    #[allow(clippy::unnecessary_lazy_evaluations)]
    (max_offset > 0).then(|| offset * (content_length - 1) / max_offset)
}
