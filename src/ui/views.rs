use std::io::{self, stdout};

use ratatui::{
    crossterm::{
        event::{self, Event, KeyCode},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    layout::{Constraint, Layout, Rows},
    prelude::CrosstermBackend,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Row, Table, TableState},
    Terminal,
};

use crate::{services, tick_tick_api::Task};

pub struct TaskView {
    items: Vec<Task>,
    state: TableState,
}

impl TaskView {
    fn new(tasks: Vec<Task>) -> TaskView {
        TaskView {
            items: tasks,
            state: TableState::default(),
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
    }
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

pub fn render_tasks(tasks: Vec<Task>) -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;

    let mut view = TaskView::new(tasks);
    view.state.select(Some(0)); //Start by the first row

    loop {
        terminal.draw(|f| {
            let rects = Layout::default()
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(f.area());

            // Convert data rows to Table Rows
            let rows: Vec<Row> = view
                .items
                .iter()
                .map(|item| {
                    Row::new(vec![
                        item.title.clone(),
                        parse_priority(&item.priority),
                        item.due_date.clone().unwrap_or(String::from("")),
                    ])
                })
                .collect();

            let task_box = Block::new()
                .border_type(BorderType::Rounded)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White))
                .title("Test BOX");

            // Create the Table Widget
            let table = Table::new(rows, [Constraint::Length(20), Constraint::Length(10)])
                .header(
                    Row::new(vec!["Task", "Priority", "Due Date"])
                        .style(Style::default().fg(Color::Yellow)),
                )
                .block(Block::default().title("Tasks").borders(Borders::ALL))
                // THE KEY PART: How the selected row looks
                .row_highlight_style(Style::default().bg(Color::White).fg(Color::Black));

            // Render with State
            f.render_stateful_widget(table, rects[0], &mut view.state);
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => break,
                KeyCode::Down => view.next(),
                KeyCode::Up => view.previous(),
                _ => {}
            }
        }
    }

    // Restore Terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
