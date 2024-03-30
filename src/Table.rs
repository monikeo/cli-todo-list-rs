use crate::Task::Task;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;

pub fn create_header_cell(text: &str) -> Cell {
    Cell::new(text)
        .set_alignment(CellAlignment::Center)
        .add_attribute(Attribute::Bold)
        .fg(Color::Blue)
}

pub fn create_status_cell(status: &str) -> Cell {
    match status {
        "completed" => Cell::new("✅").fg(Color::Green),
        "cancelled" => Cell::new("❌").fg(Color::Red),
        "in progress" => Cell::new("⏳").fg(Color::Yellow),
        _ => Cell::new(status),
    }
    .set_alignment(CellAlignment::Center)
}

pub fn create_data_cell<T: std::fmt::Display>(data: T) -> Cell {
    Cell::new(format!("{}", data))
}

pub fn create_task_row(task: &Task) -> Vec<Cell> {
    vec![
        create_data_cell(task.id()),
        create_data_cell(task.name()),
        create_data_cell(task.description()),
        create_data_cell(task.created_date().format("%Y-%m-%d %H:%M:%S")),
        create_data_cell(task.updated_date().format("%Y-%m-%d %H:%M:%S")),
        create_data_cell(
            task.ended_date()
                .map(|date| date.format("%Y-%m-%d %H:%M:%S").to_string())
                .unwrap_or_else(|| "".to_string()),
        ),
        create_status_cell(task.status()),
    ]
}

pub fn create_task_table_header() -> Table {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            create_header_cell("ID"),
            create_header_cell("Name"),
            create_header_cell("Description"),
            create_header_cell("Created Date"),
            create_header_cell("Updated Date"),
            create_header_cell("Ended Date"),
            create_header_cell("Status"),
        ]);
    table
}
