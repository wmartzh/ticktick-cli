use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Cell, Table};

use crate::tick_tick_api::Task;

pub fn render_tasks(tasks: &Vec<Task>) {
    let mut table = Table::new();

    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(comfy_table::ContentArrangement::Dynamic);

    table.set_header(vec!["Title", "Due Date"]);

    for task in tasks {
        table.add_row(vec![
            Cell::new(&task.title),
            Cell::new(task.due_date.clone().unwrap_or(String::from(""))),
        ]);
    }

    println!("{}", table);
}
