use clap::{Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
#[clap(
    author,
    version,
    about,
    long_about = None,
    subcommand_required = false
)]
pub struct Args {
    #[command(subcommand)]
    sub_command: Option<SubCommands>,
    #[clap(
        short,
        long,
        required = false,
        help = "Adding new task",
        value_name = "new task name"
    )]
    add: Option<String>,
    #[clap(
        short,
        long,
        required = false,
        help = "delete a task by id",
        value_name = "ID"
    )]
    delete: Option<usize>,
    #[clap(
        short,
        long,
        required = false,
        help = "create a new todolist by new json file name",
        value_name = "new_file_name.json"
    )]
    create: Option<String>,
    #[clap(
        long,
        required = false,
        help = "change file name [filename.json]",
        value_name = "filename.json"
    )]
    change_file_name: Option<String>,
}

#[derive(Debug, Clone, Parser)]
pub struct UpdateNameArgs {
    #[clap(short, long, help = "task id")]
    id: usize,
    #[clap(short, long, help = "new task name")]
    name: String,
}

impl UpdateNameArgs {
    pub fn id(&self) -> usize {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone, Parser)]
pub struct UpdateDescriptionArgs {
    #[clap(short, long, help = "task id")]
    id: usize,
    #[clap(short, long, help = "new task description")]
    description: String,
}

impl UpdateDescriptionArgs {
    pub fn id(&self) -> usize {
        self.id
    }
    pub fn description(&self) -> &str {
        &self.description
    }
}

#[derive(Debug, Clone, Parser)]
pub struct UpdateStatusArgs {
    #[clap(short, long, help = "task is")]
    id: usize,
    #[clap(short, long, help = "new task status")]
    status: String,
}

impl UpdateStatusArgs {
    pub fn id(&self) -> usize {
        self.id
    }
    pub fn status(&self) -> &str {
        &self.status
    }
}

/*
#[derive(Debug, Clone, Parser)]
pub struct ShowTasksArgs {
    #[clap(short, long, value_parser = ["all", "done", "inprogress", "todo"])]
    status: Option<String>,
}
*/

#[derive(Subcommand, Debug, Clone)]
pub enum SubCommands {
    #[clap(
        name = "update-status",
        about = "Update status by id (id, status) [done, todo, inprogress, cancel]"
    )]
    UpdateStatus(UpdateStatusArgs),
    #[clap(
        name = "update-info",
        about = "Update description by id (id, description"
    )]
    UpdateDescription(UpdateDescriptionArgs),
    #[clap(name = "update-name", about = "Update name by id (id, name)")]
    UpdateName(UpdateNameArgs),
    #[clap(about = "Show all tasks")]
    All,
    #[clap(about = "Show completed tasks")]
    Done,
    #[clap(about = "Show Cancel tasks")]
    Cancel,
    #[clap(about = "Show tasks in progress")]
    InProgress,
    #[clap(about = "Show pending tasks")]
    Todo,
    #[clap(about = "Show current file name")]
    FileName,
    #[clap(about = "Change file name with this format [file_name.json]")]
    ChangeFileName,
}

impl Args {
    pub fn sub_command(&self) -> &Option<SubCommands> {
        &self.sub_command
    }

    pub fn add(&self) -> &Option<String> {
        &self.add
    }

    pub fn delete(&self) -> &Option<usize> {
        &self.delete
    }

    pub fn create(&self) -> &Option<String> {
        &self.create
    }

    pub fn change_file_name(&self) -> &Option<String> {
        &self.change_file_name
    }
}
