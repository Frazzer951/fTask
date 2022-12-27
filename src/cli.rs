use clap::{command, value_parser, Arg, ArgAction, Command};

pub fn cli() -> Command {
    command!()
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommands(vec![
            subcommand_list(),
            subcommand_new(),
            subcommand_reset(),
            subcommand_complete(),
            subcommand_remove(),
            subcommand_next(),
            subcommand_delete_all(),
        ])
}

fn subcommand_list() -> Command {
    Command::new("list")
        .about("List tasks, if no flags are given, only incomplete tasks will be displayed")
        .args(&[
            Arg::new("all")
                .short('a')
                .long("all")
                .help("Show all tasks")
                .action(ArgAction::SetTrue)
                .group("show"),
            Arg::new("completed")
                .short('c')
                .long("completed")
                .help("Show completed tasks")
                .action(ArgAction::SetTrue)
                .group("show"),
        ])
}

fn subcommand_new() -> Command {
    Command::new("new").about("Add a new Task").args(&[
        Arg::new("name").short('n').long("name"),
        Arg::new("desc").short('d').long("description"),
        Arg::new("priority")
            .short('p')
            .long("priority")
            .help("The priority of the task, the lower the number, the higher its priority")
            .value_parser(value_parser!(u32)),
    ])
}

fn subcommand_reset() -> Command {
    Command::new("reset")
        .about("Reset tasks by IDs. This will reset their completed state")
        .args(&[Arg::new("ids")
            .help("Give a list of IDs that will be reset")
            .num_args(1..)
            .required(true)
            .value_parser(value_parser!(u32))
            .action(ArgAction::Append)])
}

fn subcommand_complete() -> Command {
    Command::new("complete")
        .about("Complete tasks by IDs. This will set their states to completed")
        .args(&[Arg::new("ids")
            .help("Give a list of IDs that will be reset")
            .num_args(1..)
            .required(true)
            .value_parser(value_parser!(u32))
            .action(ArgAction::Append)])
}

fn subcommand_remove() -> Command {
    Command::new("remove")
        .about("Remove tasks by IDs. This will delete the tasks from the database")
        .args(&[
            Arg::new("ids")
                .help("Give a list of IDs that will be reset")
                .num_args(1..)
                .required(true)
                .value_parser(value_parser!(u32))
                .action(ArgAction::Append),
            Arg::new("force").short('f').long("force").action(ArgAction::SetTrue),
        ])
}

fn subcommand_next() -> Command {
    Command::new("next")
        .about("Display the next task(s) by priority")
        .args(&[Arg::new("num_tasks")
            .help("The number of tasks to display")
            .value_parser(value_parser!(usize))
            .default_value("1")])
}

fn subcommand_delete_all() -> Command {
    Command::new("delete_all")
        .about("Removes all tasks from the database")
        .args(&[Arg::new("force").short('f').long("force").action(ArgAction::SetTrue)])
}
