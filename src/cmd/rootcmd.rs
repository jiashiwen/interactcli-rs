use crate::cmd::requestsample::new_requestsample_cmd;
use crate::cmd::{new_config_cmd, new_multi_cmd, new_task_cmd, new_use_log_cmd};
use crate::commons::CommandCompleter;
use crate::commons::SubCmd;

use crate::configure::set_config_file_path;
use crate::configure::{self, get_config, get_config_file_path};
use crate::request::{req, ReqResult, Request, RequestTaskListAll};
use crate::{configure::set_config, interact};
use clap::{Arg, ArgMatches, Command as clap_Command};
use lazy_static::lazy_static;
use log::info;

use std::borrow::Borrow;
use std::{env, fs, thread};

use crate::cmd::cmdloop::new_loop_cmd;
use chrono::prelude::Local;
use fork::{daemon, Fork};
use std::fs::File;
use std::io::Read;
use std::process::Command;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Duration;
use sysinfo::{PidExt, System, SystemExt};

lazy_static! {
    static ref CLIAPP: clap::Command<'static> = clap::Command::new("interact-rs")
        .version("1.0")
        .author("Shiwen Jia. <jiashiwen@gmail.com>")
        .about("command line sample")
        .arg_required_else_help(true)
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true)
        )
        .arg(
            Arg::new("daemon")
                .short('d')
                .long("daemon")
                .help("run as daemon")
        )
        .arg(
            Arg::new("interact")
                .short('i')
                .long("interact")
                .conflicts_with("daemon")
                .help("run as interact mod")
        )
        .arg(
            Arg::new("v")
                .short('v')
                .multiple_occurrences(true)
                .takes_value(true)
                .help("Sets the level of verbosity")
        )
        .subcommand(new_requestsample_cmd())
        .subcommand(new_config_cmd())
        .subcommand(new_multi_cmd())
        .subcommand(new_task_cmd())
        .subcommand(new_loop_cmd())
        .subcommand(new_use_log_cmd())
        .subcommand(
            clap::Command::new("test")
                .about("controls testing features")
                .version("1.3")
                .author("Someone E. <someone_else@other.com>")
                .arg(
                    Arg::new("debug")
                        .short('d')
                        .help("print debug information verbosely")
                )
        );
    static ref SUBCMDS: Vec<SubCmd> = subcommands();
}

pub fn run_app() {
    let matches = CLIAPP.clone().get_matches();
    if let Some(c) = matches.value_of("config") {
        println!("config path is:{}", c);
        set_config_file_path(c.to_string());
    }
    set_config(&get_config_file_path());
    cmd_match(&matches);
}

pub fn run_from(args: Vec<String>) {
    match clap_Command::try_get_matches_from(CLIAPP.to_owned(), args.clone()) {
        Ok(matches) => {
            cmd_match(&matches);
        }
        Err(err) => {
            err.print().expect("Error writing Error");
        }
    };
}

// 获取全部子命令，用于构建commandcompleter
pub fn all_subcommand(app: &clap_Command, beginlevel: usize, input: &mut Vec<SubCmd>) {
    let nextlevel = beginlevel + 1;
    let mut subcmds = vec![];
    for iterm in app.get_subcommands() {
        subcmds.push(iterm.get_name().to_string());
        if iterm.has_subcommands() {
            all_subcommand(iterm, nextlevel, input);
        } else {
            if beginlevel == 0 {
                all_subcommand(iterm, nextlevel, input);
            }
        }
    }
    let subcommand = SubCmd {
        level: beginlevel,
        command_name: app.get_name().to_string(),
        subcommands: subcmds,
    };
    input.push(subcommand);
}

pub fn get_command_completer() -> CommandCompleter {
    CommandCompleter::new(SUBCMDS.to_vec())
}

fn subcommands() -> Vec<SubCmd> {
    let mut subcmds = vec![];
    all_subcommand(CLIAPP.clone().borrow(), 0, &mut subcmds);
    subcmds
}

pub fn process_exists(pid: &u32) -> bool {
    let mut sys = System::new_all();
    sys.refresh_all();
    for (syspid, _) in sys.processes() {
        if syspid.as_u32().eq(pid) {
            return true;
        }
    }
    return false;
}

fn cmd_match(matches: &ArgMatches) {
    let config = get_config().unwrap();
    let server = &config["server"];
    let req = Request::new(server.clone());

    if matches.is_present("daemon") {
        let args: Vec<String> = env::args().collect();
        if let Ok(Fork::Child) = daemon(true, true) {
            // 启动子进程
            let mut cmd = Command::new(&args[0]);

            for idx in 1..args.len() {
                let arg = args.get(idx).expect("get cmd arg error!");
                // 去除后台启动参数,避免重复启动
                if arg.eq("-d") || arg.eq("-daemon") {
                    continue;
                }
                cmd.arg(arg);
            }

            let child = cmd.spawn().expect("Child process failed to start.");
            fs::write("pid", child.id().to_string()).unwrap();
            println!("process id is:{}", std::process::id());
            println!("child id is:{}", child.id());
        }
        println!("{}", "daemon mod");
        std::process::exit(0);
    }

    if matches.is_present("interact") {
        interact::run();
        return;
    }

    // 测试 log 写入不同文件
    if let Some(ref log) = matches.subcommand_matches("uselog") {
        println!("use log");
        if let Some(_) = log.subcommand_matches("syslog") {
            log::info!(target:"syslog","Input sys log");
        }

        if let Some(_) = log.subcommand_matches("businesslog") {
            log::info!(target:"businesslog","Input business log");
        }
    }

    if let Some(ref _matches) = matches.subcommand_matches("loop") {
        let term = Arc::new(AtomicBool::new(false));
        let sigint_2 = Arc::new(AtomicBool::new(false));
        signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&term)).unwrap();
        signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&sigint_2)).unwrap();
        loop {
            if sigint_2.load(Ordering::Relaxed) {
                println!("{}", "singint signal recived");
                break;
            }

            thread::sleep(Duration::from_millis(1000));
            if term.load(Ordering::Relaxed) {
                println!("{:?}", term);
                break;
            }
            let dt = Local::now();
            let _ = fs::write("timestamp", dt.timestamp_millis().to_string());
        }
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level app
    if let Some(ref matches) = matches.subcommand_matches("test") {
        if matches.is_present("debug") {
            println!("Printing debug info...");
        } else {
            println!("Printing normally...");
        }
    }

    if let Some(ref matches) = matches.subcommand_matches("requestsample") {
        if let Some(_) = matches.subcommand_matches("baidu") {
            let rt = tokio::runtime::Runtime::new().unwrap();
            let async_req = async {
                let result = req::get_baidu().await;
                println!("{:?}", result);
            };
            rt.block_on(async_req);
        };
    }

    if let Some(ref matches) = matches.subcommand_matches("task") {
        if let Some(create) = matches.subcommand_matches("create") {
            let file = File::open(create.value_of("path").unwrap());
            match file {
                Ok(mut f) => {
                    let mut data = String::new();
                    if let Err(e) = f.read_to_string(&mut data) {
                        println!("{}", e);
                        return;
                    };
                    let rt = tokio::runtime::Runtime::new().unwrap();
                    let async_req = async {
                        let resp = req.create_task(data).await;
                        let result = ReqResult::new(resp);
                        result.normal_parsor().await;
                    };
                    rt.block_on(async_req);
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
        }
        if let Some(start) = matches.subcommand_matches("start") {
            if let Some(taskid) = start.value_of("taskid") {
                let rt = tokio::runtime::Runtime::new().unwrap();
                let async_req = async {
                    let resp = req.task_start(taskid.to_string()).await;
                    let result = ReqResult::new(resp);
                    result.normal_parsor().await;
                };
                rt.block_on(async_req);
            };
        }
        if let Some(stop) = matches.subcommand_matches("stop") {
            if let Some(taskid) = stop.value_of("taskid") {
                let rt = tokio::runtime::Runtime::new().unwrap();
                let async_req = async {
                    let resp = req.task_stop(taskid.to_string()).await;
                    let result = ReqResult::new(resp);
                    result.normal_parsor().await;
                };
                rt.block_on(async_req);
            };
        }
        if let Some(remove) = matches.subcommand_matches("remove") {
            if let Some(taskid) = remove.value_of("taskid") {
                let rt = tokio::runtime::Runtime::new().unwrap();
                let async_req = async {
                    let resp = req.task_stop(taskid.to_string()).await;
                    let result = ReqResult::new(resp);
                    result.normal_parsor().await;
                };
                rt.block_on(async_req);
            };
        }
        if let Some(list) = matches.subcommand_matches("list") {
            match list.subcommand_name() {
                Some("all") => {
                    let queryid = list.subcommand_matches("all").unwrap().value_of("queryid");
                    let mut module = RequestTaskListAll::default();
                    let rt = tokio::runtime::Runtime::new().unwrap();
                    let async_req = async {
                        match queryid {
                            None => {
                                let resp = req.task_list_all(module).await;
                                let result = ReqResult::new(resp);
                                result.task_list_all_parsor().await;
                            }
                            Some(id) => {
                                module.set_query_id(id.to_string());
                                let resp = req.task_list_all(module).await;
                                let result = ReqResult::new(resp);
                                result.task_list_all_parsor().await;
                            }
                        }
                    };
                    rt.block_on(async_req);
                }
                Some("byid") => {
                    let queryid = list.subcommand_matches("byid").unwrap().value_of("taskid");
                    let rt = tokio::runtime::Runtime::new().unwrap();
                    let async_req = async {
                        let mut ids = vec![];
                        if let Some(id) = queryid {
                            ids.push(id.to_string());
                            let resp = req.task_list_by_ids(ids).await;
                            let result = ReqResult::new(resp);
                            result.task_list_byid_parsor().await;
                        }
                    };
                    rt.block_on(async_req);
                }
                Some("bynames") => {
                    let names = list
                        .subcommand_matches("bynames")
                        .unwrap()
                        .value_of("tasksname");
                    let rt = tokio::runtime::Runtime::new().unwrap();
                    let async_req = async {
                        // let mut namearry = names;
                        if let Some(namesstr) = names {
                            let namearry = namesstr.split(',').collect::<Vec<&str>>();

                            let resp = req.task_list_by_names(namearry).await;
                            let result = ReqResult::new(resp);
                            result.task_list_bynames_parsor().await;
                        }
                    };
                    rt.block_on(async_req);
                }

                _ => {}
            }
        }
    }

    if let Some(config) = matches.subcommand_matches("config") {
        if let Some(show) = config.subcommand_matches("show") {
            match show.subcommand_name() {
                Some("all") => {
                    println!("config show all");
                    info!("log show all");
                    configure::get_config_file_path();
                    println!("{:?}", configure::get_config());
                }
                Some("info") => {
                    println!("config show info");
                }
                _ => {}
            }
        }
    }
}
