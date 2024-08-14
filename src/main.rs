pub mod consts;
pub mod game;

use game::Sudoku;
use ratatui::{crossterm::execute, prelude::*, widgets::{Block, Borders}};

use std::{
    io::{self, stdout, Result},
    panic::{set_hook, take_hook},
    thread::sleep,
    time::Duration,
};

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    style::Stylize,
    widgets::Paragraph,
    Terminal,
};

fn draw_frame(frame: &mut Frame, sudoku: &Sudoku) -> () {
    let area = frame.size();
    let row_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Ratio(1, 9),
            Constraint::Ratio(1, 9),
            Constraint::Ratio(1, 9),
            Constraint::Ratio(1, 9),
            Constraint::Ratio(1, 9),
            Constraint::Ratio(1, 9),
            Constraint::Ratio(1, 9),
            Constraint::Ratio(1, 9),
            Constraint::Ratio(1, 9),
        ])
        .areas::<9>(area);

    let cell_layouts: Vec<Rect> = row_layout
        .iter()
        .flat_map(|rect| {
            Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Ratio(1, 9),
                    Constraint::Ratio(1, 9),
                    Constraint::Ratio(1, 9),
                    Constraint::Ratio(1, 9),
                    Constraint::Ratio(1, 9),
                    Constraint::Ratio(1, 9),
                    Constraint::Ratio(1, 9),
                    Constraint::Ratio(1, 9),
                    Constraint::Ratio(1, 9),
                ])
                .areas::<9>(*rect)
        })
        .into_iter()
        .collect();

    let blocks = cell_layouts.iter().zip(sudoku.board).map(|(cell, rect)| {
        frame.render_widget(Paragraph, area)
    })

    /* frame.render_widget(
        Paragraph::new("Hello Ratatui! (press 'q' to quit)")
            .white()
            .on_blue(),
        area,
    ); */
}

fn main_loop(sudoku: Sudoku) -> Result<()> {
    let mut terminal = init_tui()?;
    loop {
        terminal.draw(|frame| draw_frame(frame, &sudoku))?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }
    Ok(())
}

pub fn main() -> Result<()> {
    let sudoku = Sudoku::new();
    init_panic_hook();
    main_loop(sudoku)?;
    restore_tui()?;
    Ok(())
}

pub fn init_panic_hook() {
    let original_hook = take_hook();
    set_hook(Box::new(move |panic_info| {
        // intentionally ignore errors here since we're already in a panic
        let _ = restore_tui();
        original_hook(panic_info);
    }));
}

pub fn init_tui() -> Result<Terminal<impl Backend>> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

pub fn restore_tui() -> Result<()> {
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;
    Ok(())
}
