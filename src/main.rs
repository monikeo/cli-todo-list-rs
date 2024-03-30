use clap::Parser;
use cli_todo_list::Cli::{Args, SubCommands};
use cli_todo_list::TaskJson::*;
use cli_todo_list::TodoList::TodoList;

fn run() {
    let args = Args::parse();

    let file_path = "first_json.json";
    let mut todo_list = TodoList::new();

    let tasks = read_tasks_from_json(file_path);
    todo_list.import(&tasks.unwrap().get_all_task());

    if !args.sub_command().is_none() {
        match args.sub_command().clone().unwrap() {
            SubCommands::All => {
                todo_list.print_tasks();
            }
            SubCommands::Done => {
                todo_list.print_filtered_tasks("completed");
            }
            SubCommands::Cancel => {
                todo_list.print_filtered_tasks("cancelled");
            }
            SubCommands::InProgress => {
                todo_list.print_filtered_tasks("in progress");
            }
            SubCommands::Todo => {
                todo_list.print_filtered_tasks("pending");
            }
            SubCommands::FileName => {
                println!("filename: not yet implement");
            }
            SubCommands::ChangeFileName => {
                println!("change file name: not yet implement");
            }
            SubCommands::UpdateName(update_name) => {
                let updated_task =
                    todo_list.update_task_name_by_id(update_name.id(), update_name.name());
                match updated_task {
                    Some(task) => {
                        let _ = update_task_in_json(&task, file_path);
                    }
                    _ => {}
                };
            }
            SubCommands::UpdateDescription(update_description) => {
                let updated_task = todo_list.update_task_description_by_id(
                    update_description.id(),
                    update_description.description(),
                );
                match updated_task {
                    Some(task) => {
                        let _ = update_task_in_json(&task, file_path);
                    }
                    _ => {}
                };
            }
            SubCommands::UpdateStatus(update_status) => {
                let updated_task =
                    todo_list.update_task_status_by_id(update_status.id(), update_status.status());
                match updated_task {
                    Some(task) => {
                        let _ = update_task_in_json(&task, file_path);
                    }
                    _ => {}
                };
            }
        }
    }

    if let Some(add) = args.add() {
        todo_list.add(add);
        let _ = write_tasks_to_json(todo_list.get_all_task(), file_path);
        todo_list.print_filtered_tasks("pending");
        //println!("execute add: {}", add);
    }

    if let Some(delete) = args.delete() {
        let _ = delete_task_from_json(*delete, file_path);
        //println!("execute delete: {}", delete);
    }

    if let Some(create) = args.create() {
        println!("execute create: {}", create);
    }

    if let Some(change_file_name) = args.change_file_name() {
        println!("execute change file name: {}", change_file_name);
    }
}

fn main() {
    run();
}
