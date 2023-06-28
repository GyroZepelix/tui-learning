use std::error::Error;
use std::io;

use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, LeaveAlternateScreen};
use crossterm::{execute, terminal::EnterAlternateScreen};

use tui::backend::{Backend, CrosstermBackend};
use tui::layout::Alignment;
use tui::style::Style;
use tui::widgets::{Block, BorderType, Borders};
use tui::{Frame, Terminal};

struct App {
    edit_mode: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            edit_mode: Default::default(),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::default();
    run_app(&mut terminal, app)?;

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Esc => app.edit_mode = false,
                KeyCode::Char('e') => app.edit_mode = true,
                _ => {}
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let size = f.size();

    let block = Block::default()
        .title(" I am a block ")
        .borders(Borders::ALL)
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded)
        .border_style(if app.edit_mode {
            Style::default().fg(tui::style::Color::Red)
        } else {
            Style::default().fg(tui::style::Color::Reset)
        });
    f.render_widget(block, size);
}
