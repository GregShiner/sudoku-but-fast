pub mod consts;
pub mod game;

use consts::SIZE;
use game::{check_possibility, Cell, CellIndex, Sudoku};
use ratatui::{
    crossterm::{
        event::{KeyEvent, KeyModifiers},
        execute,
    },
    prelude::*,
    text::Line,
    widgets::Block,
};

use std::{
    io::{stdout, Result},
    panic::{set_hook, take_hook},
};

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    widgets::Paragraph,
    Terminal,
};

impl StatefulWidget for Sudoku {
    type State = CellIndex;

    fn render(self, area: Rect, buf: &mut Buffer, selected_cell: &mut Self::State) {
        let col_constraints = (0..9).map(|_| Constraint::Length(9));
        let row_constraints = (0..9).map(|_| Constraint::Length(5));
        let horizontal = Layout::horizontal(col_constraints).spacing(0);
        let vertical = Layout::vertical(row_constraints).spacing(0);

        let rows = vertical.split(area);
        let cells = rows.iter().flat_map(|&row| horizontal.split(row).to_vec());

        for (i, (rect, cell)) in cells.zip(self.board).enumerate() {
            cell.render(rect, buf, &mut ((i == *selected_cell), i))
        }
    }
}

impl StatefulWidget for Cell {
    type State = (bool, usize);

    fn render(self, area: Rect, buf: &mut Buffer, highlighted: &mut Self::State) {
        let spans: Vec<Span<'static>> = (1..=9)
            .map(|n| {
                let style = match self {
                    Cell::Unknown(bits) => {
                        if check_possibility(bits, n) {
                            Style::default().fg(Color::White)
                        } else {
                            Style::default().fg(Color::DarkGray)
                        }
                    }
                    Cell::Solved(x) => {
                        if x == n {
                            Style::default().fg(Color::Green)
                        } else {
                            Style::default().fg(Color::DarkGray)
                        }
                    }
                };

                Span::styled(n.to_string(), style)
            })
            .collect();

        // Turn 9 spans into 3 lines of 3 spans
        let lines = vec![
            Line::from(spans[0..3].to_vec()),
            Line::from(spans[3..6].to_vec()),
            Line::from(spans[6..9].to_vec()),
        ];

        Paragraph::new(lines)
            .block(
                Block::bordered()
                    .border_style(if highlighted.0 {
                        Style::default().green()
                    } else {
                        Style::default().white()
                    })
                    .title(highlighted.1.to_string()),
            )
            .alignment(Alignment::Center)
            .render(area, buf);
    }
}

#[derive(Clone, Default)]
struct App {
    sudoku: Sudoku,
    selected_cell: CellIndex,
}

impl App {
    fn handle_key(&mut self, key_event: KeyEvent) {
        let row_start = (self.selected_cell / 9) * 9;
        use KeyCode::*;
        match key_event.code {
            // These move the selected box up, down, left, and right, wrapping to the opposite side
            // at the end of each row and column
            // Its necessaary to add the modulus first before subtracting 1
            // 0usize - 1 causes a panic
            Char('h' | 'a') | Left => {
                self.selected_cell = ((self.selected_cell + 9 - 1) % 9) + row_start
            }
            Char('l' | 'd') | Right => {
                self.selected_cell = (self.selected_cell + 1) % 9 + row_start
            }
            Char('j' | 's') | Down => self.selected_cell = (self.selected_cell + 9) % SIZE,
            Char('k' | 'w') | Up => self.selected_cell = (self.selected_cell + SIZE - 9) % SIZE,
            Char(num @ '1'..='9') => {
                let num: u8 = num.to_digit(10).unwrap().try_into().unwrap();
                if key_event.modifiers.intersects(KeyModifiers::ALT) {
                    match self.sudoku.board[self.selected_cell] {
                        Cell::Unknown(_) => {
                            let _ = self.sudoku.solve_cell(self.selected_cell, num);
                        }
                        Cell::Solved(_) => {
                            self.sudoku.unsolve_cell(self.selected_cell);
                        }
                    }
                } else {
                    self.sudoku
                        .toggle_poss(self.selected_cell, num)
                        .unwrap_or(()); // Already solved errors can be ignored
                }
            }
            _ => {}
        }
    }
}

impl From<Sudoku> for App {
    fn from(sudoku: Sudoku) -> Self {
        App {
            sudoku,
            selected_cell: 0,
        }
    }
}

impl Widget for App {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        self.sudoku.render(area, buf, &mut self.selected_cell);
    }
}

#[allow(dead_code)]
fn debug_key(key: KeyEvent, terminal: &mut Terminal<impl Backend>) {
    let debug = format!("{:?}", key);
    terminal
        .draw(|frame| frame.render_widget(Line::from(debug), frame.area()))
        .unwrap();
}

fn main_loop(app: &mut App) -> Result<()> {
    let mut terminal = init_tui()?;
    loop {
        if event::poll(std::time::Duration::from_millis(16))? {
            match event::read()? {
                event::Event::Key(key) if key.kind == KeyEventKind::Press => {
                    // debug_key(key, &mut terminal);
                    if let KeyCode::Char('q') = key.code {
                        break;
                    } else {
                        app.handle_key(key);
                    }
                }
                _ => {}
            }
        }
        terminal.draw(|frame| frame.render_widget(app.clone(), frame.area()))?;
    }
    Ok(())
}

pub fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let mut app: App = if args.len() == 1 {
        App::default()
    } else {
        Sudoku::from_file(&args[1]).unwrap().into()
    };
    init_panic_hook();
    main_loop(&mut app)?;
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
