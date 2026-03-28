mod app;
mod ui;

use std::{env, io};

use anyhow::Result;
use app::App;
use crossterm::event::{self, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

fn main() -> Result<()> {
    let items_count: usize = {
        let mut args = env::args();
        args.next();
        args.next()
            .expect("Expected argument for number of items")
            .parse()?
    };

    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;
    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen)?;
    terminal.hide_cursor()?;
    terminal.clear()?;

    let var_name = App::new(items_count);
    let mut app = var_name;

    loop {
        terminal.draw(|frame| ui::ui(frame, &mut app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind != event::KeyEventKind::Press {
                continue;
            }

            match key.code {
                KeyCode::Up => app.select_up(1),
                KeyCode::Down => app.select_down(1),
                KeyCode::PageUp => app.select_up(app.page_size - 1),
                KeyCode::PageDown => app.select_down(app.page_size - 1),
                KeyCode::Home => app.select_first(),
                KeyCode::End => app.select_last(),
                KeyCode::Esc => break,
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
