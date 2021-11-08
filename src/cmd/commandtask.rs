use clap::App;
use clap::Arg;

pub fn new_task_cmd() -> App<'static> {
    clap::App::new("task")
        .about("command about task")
        .subcommand(cmd_task_create())
        .subcommand(cmd_task_start())
        .subcommand(cmd_task_stop())
        .subcommand(cmd_task_remove())
        .subcommand(cmd_task_list())
}

fn cmd_task_create() -> App<'static> {
    clap::App::new("create")
        .about("create task")
        .arg(Arg::from("<path> 'create task json file path'"))
}

fn cmd_task_start() -> App<'static> {
    clap::App::new("start")
        .about("start task")
        .arg(Arg::from("<taskid> 'input task id to stop'"))
}

fn cmd_task_stop() -> App<'static> {
    clap::App::new("stop")
        .about("stop task")
        .arg(Arg::from("<taskid> 'input task id to stop'"))
}
fn cmd_task_remove() -> App<'static> {
    clap::App::new("remove")
        .about("remove task")
        .arg(Arg::from("<taskid> 'input task id to stop'"))
}

fn cmd_task_list() -> App<'static> {
    clap::App::new("list")
        .about("list tasks")
        .subcommand(cmd_task_list_all())
        .subcommand(cmd_task_list_by_ids())
        .subcommand(cmd_task_list_by_names())
}

fn cmd_task_list_all() -> App<'static> {
    clap::App::new("all")
        .about("list tasks by task ids")
        .arg(Arg::from("[queryid] 'input queryid if have'"))
}

fn cmd_task_list_by_ids() -> App<'static> {
    clap::App::new("byid")
        .about("list tasks by task ids")
        .arg(Arg::from("<taskid> 'input taskid'"))
}

fn cmd_task_list_by_names() -> App<'static> {
    clap::App::new("bynames")
        .about("list tasks by task names")
        .arg(Arg::from(
            "<tasksname> 'input tasks name if multi use ',' to splite'",
        ))
}

// fn cmd_task_list_by_groupids() -> App<'static> {
//     clap::App::new("bygroupids")
//         .about("list tasks by task group ids")
//         .arg(Arg::from(
//             "<tasksgroupid> 'input tasks groupids if multi use ',' to splite'",
//         ))
// }
