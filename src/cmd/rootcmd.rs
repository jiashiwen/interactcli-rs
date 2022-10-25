use crate::cmd::requestsample::new_requestsample_cmd;
use crate::cmd::{new_config_cmd, new_multi_cmd, new_server_cmd, new_task_cmd, new_use_log_cmd};
use crate::commons::CommandCompleter;
use crate::commons::SubCmd;

use crate::configure::{self, generate_default_config, get_config, get_config_file_path, Config};
use crate::configure::{set_config_file_path, set_config_from_file};
use crate::request::{req, ReqResult, Request, RequestTaskListAll};
use crate::server::start;
use crate::{configure::set_config, interact};
use clap::{Arg, ArgAction, ArgMatches, Command as clap_Command};
use daemonize::Daemonize;
use lazy_static::lazy_static;
use log::info;

use std::borrow::Borrow;
use std::env::args;
use std::{env, fs, process, thread};

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
    static ref CLIAPP: clap::Command = clap::Command::new("interact-rs")
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
                // .takes_value(true)
        )
        .arg(
            Arg::new("daemon")
                .short('d')
                .long("daemon")
                .action(ArgAction::SetTrue)
                .help("run as daemon")
        )
        .arg(
            Arg::new("interact")
                .short('i')
                .long("interact")
                .conflicts_with("daemon")
                .action(ArgAction::SetTrue)
                .help("run as interact mod")
        )
        .subcommand(new_requestsample_cmd())
        .subcommand(new_config_cmd())
        .subcommand(new_multi_cmd())
        .subcommand(new_task_cmd())
        .subcommand(new_loop_cmd())
        .subcommand(new_use_log_cmd())
        .subcommand(new_server_cmd())
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
    if let Some(c) = matches.get_one::<String>("config") {
        // if let Some(c) = matches.value_of("config") {
        println!("config path is:{}", c);
        set_config_file_path(c.to_string());
    }
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
    if let Some(c) = matches.get_one::<String>("config") {
        // if let Some(c) = matches.value_of("config") {
        set_config_file_path(c.to_string());
        set_config_from_file(&get_config_file_path());
    } else {
        set_config_from_file("");
    }
    let config = get_config().unwrap();
    let server = config.server;
    let req = Request::new(server.clone());

    if matches.get_flag("daemon") {
        // if matches.is_present("daemon") {
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
        process::exit(0);
    }

    if matches.get_flag("interact") {
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
        if matches.contains_id("debug") {
            // if matches.is_present("debug") {
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
            let file = File::open(create.get_one::<String>("path").unwrap());
            // let file = File::open(create.value_of("path").unwrap());
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
            if let Some(taskid) = start.get_one::<String>("taskid") {
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
            if let Some(taskid) = stop.get_one::<String>("taskid") {
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
            if let Some(taskid) = remove.get_one::<String>("taskid") {
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
                    let queryid = list
                        .subcommand_matches("all")
                        .unwrap()
                        .get_one::<String>("queryid");
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
                    let queryid = list
                        .subcommand_matches("byid")
                        .unwrap()
                        .get_one::<String>("taskid");
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
                        .get_one::<String>("tasksname");
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
                Some("current") => {
                    let current = configure::get_config().expect("get current configure error!");
                    let yml =
                        serde_yaml::to_string(&current).expect("pars configure to yaml error!");
                    println!("{}", yml);
                }
                Some("default") => {
                    let config = Config::default();
                    let yml = serde_yaml::to_string(&config);
                    match yml {
                        Ok(y) => {
                            println!("{}", y);
                        }
                        Err(e) => {
                            log::error!("{}", e);
                        }
                    }
                }
                _ => {}
            }
        }
        if let Some(gen_config) = config.subcommand_matches("gendefault") {
            let mut file = String::from("");
            if let Some(path) = gen_config.get_one::<String>("filepath") {
                file.push_str(path);
            } else {
                file.push_str("config_default.yml")
            }
            if let Err(e) = generate_default_config(file.as_str()) {
                log::error!("{}", e);
                return;
            };
            println!("{} created!", file);
        }
    }

    if let Some(server) = matches.subcommand_matches("server") {
        if let Some(startbyfork) = server.subcommand_matches("byfork") {
            println!("start by fork");
            if startbyfork.get_flag("daemon") {
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
                process::exit(0);
            }
            start("by_fork:".to_string());
        }
        if let Some(startbydaemonize) = server.subcommand_matches("bydaemonize") {
            println!("start by daemonize");
            let base_dir = env::current_dir().unwrap();
            if startbydaemonize.get_flag("daemon") {
                let stdout = File::create("/tmp/daemon.out").unwrap();
                let stderr = File::create("/tmp/daemon.err").unwrap();

                println!("{:?}", base_dir);

                let daemonize = Daemonize::new()
                    .pid_file("/tmp/test.pid") // Every method except `new` and `start`
                    .chown_pid_file(true) // is optional, see `Daemonize` documentation
                    .working_directory(base_dir.as_path()) // for default behaviour.
                    // .user("nobody")
                    // .group("daemon") // Group name
                    .umask(0o777) // Set umask, `0o027` by default.
                    .stdout(stdout) // Redirect stdout to `/tmp/daemon.out`.
                    .stderr(stderr) // Redirect stderr to `/tmp/daemon.err`.
                    .privileged_action(|| "Executed before drop privileges");

                match daemonize.start() {
                    Ok(_) => {
                        println!("Success, daemonized");
                    }
                    Err(e) => eprintln!("Error, {}", e),
                }
            }
            println!("pid is:{}", std::process::id());
            // let mut path = base_dir.clone();
            // path.push("pid");
            // fs::write(path, process::id().to_string()).unwrap();
            fs::write("pid", process::id().to_string()).unwrap();
            start("by_daemonize:".to_string());
        }
    }
}
