use std::io::{self};

use ratatui::{
    crossterm::{
        event::{self, Event, KeyCode},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    layout::{Constraint, Direction, Layout},
    prelude::CrosstermBackend,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Row, Table, TableState, Wrap},
    Frame, Terminal,
};

use crate::tick_tick_api::Task;

pub struct TaskView {
    items: Vec<Task>,
    state: TableState,
    load_details: bool,
    selected: Option<Task>,
}

fn parse_priority(priority: &u32) -> String {
    let value = match priority {
        0 => String::from("―"),
        1 => String::from("🔵 Low"),
        3 => String::from("🟡 Medium"),
        5 => String::from("🔴 High"),
        _ => String::from(""),
    };
    value
}

impl TaskView {
    fn new(tasks: Vec<Task>) -> TaskView {
        TaskView {
            items: tasks,
            state: TableState::default(),
            selected: None,
            load_details: false,
        }
    }
    // Logic to move selection down
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        if self.load_details {
            self.selec_item();
        }
    }

    // Logic to move selection up
    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        if self.load_details {
            self.selec_item();
        }
    }

    pub fn selec_item(&mut self) {
        if let Some(current) = self.state.selected() {
            let item = self.items.get(current);
            self.selected = item.cloned();
            self.load_details = true;
        }
    }
}

fn draw_tasks(frame: &mut Frame, view: &mut TaskView) {
    let constraints = if view.load_details {
        vec![Constraint::Percentage(70), Constraint::Percentage(30)]
    } else {
        vec![Constraint::Percentage(100)]
    };

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(frame.area());

    let rows: Vec<Row> = view
        .items
        .iter()
        .map(|item| {
            Row::new(vec![
                item.title.clone(),
                parse_priority(&item.priority),
                item.due_date.clone().unwrap_or(String::from("_")),
            ])
        })
        .collect();

    let header: Row =
        Row::new(vec!["Title", "Priority", "Due"]).style(Style::default().fg(Color::Yellow));
    let table = Table::new(
        rows,
        [
            Constraint::Percentage(40),
            Constraint::Percentage(10),
            Constraint::Percentage(20),
        ],
    )
    .header(header)
    .block(Block::default().title("Tasks").borders(Borders::ALL))
    .row_highlight_style(Style::default().bg(Color::White).fg(Color::Black));

    frame.render_stateful_widget(table, chunks[0], &mut view.state);

    if view.load_details {
        let selected = view.selected.clone().unwrap();

        let details_widget = Paragraph::new(format!(
            "Title: {}\nPriority: {}\nDue Date:{}\nDescription: {}",
            selected.title,
            parse_priority(&selected.priority),
            selected.due_date.unwrap_or(String::from("-")),
            selected.content.unwrap_or(String::from(""))
        ))
        .block(Block::default().borders(Borders::ALL).title("Details"))
        .wrap(Wrap { trim: true });

        frame.render_widget(details_widget, chunks[1]);
    }
}
pub fn render_tasks(tasks: Vec<Task>) -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;

    let mut view = TaskView::new(tasks);
    view.state.select(Some(0)); //Start by the first row

    loop {
        terminal.draw(|f| draw_tasks(f, &mut view))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => break,
                KeyCode::Down | KeyCode::Char('j') => view.next(),
                KeyCode::Up | KeyCode::Char('k') => view.previous(),
                KeyCode::Char('s') => {
                    view.selec_item();
                }
                _ => {}
            }
        }
    }

    // Restore Terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
