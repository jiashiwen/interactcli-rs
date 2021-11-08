use crate::cmd::requestsample::new_requestsample_cmd;
use crate::cmd::{new_config_cmd, new_multi_cmd, new_task_cmd};
use crate::commons::CommandCompleter;
use crate::commons::SubCmd;

use crate::configure::set_config_file_path;
use crate::configure::{self, get_config, get_config_file_path};
use crate::request::{req, ReqResult, Request, RequestTaskListAll};
use crate::{configure::set_config, interact};
use clap::{App, Arg, ArgMatches};
use lazy_static::lazy_static;
use log::info;

use std::borrow::Borrow;
use std::fs::File;
use std::io::Read;

lazy_static! {
    static ref CLIAPP: clap::App<'static> = App::new("interact-rs")
        .version("1.0")
        .author("Shiwen Jia. <jiashiwen@gmail.com>")
        .about("command line sample")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .about("Sets a custom config file")
                .takes_value(true)
        )
        .arg(
            Arg::new("interact")
                .short('i')
                .long("interact")
                .about("run as interact mod")
        )
        .arg(
            Arg::new("v")
                .short('v')
                .multiple_occurrences(true)
                .takes_value(true)
                .about("Sets the level of verbosity")
        )
        .subcommand(new_requestsample_cmd())
        .subcommand(new_config_cmd())
        .subcommand(new_multi_cmd())
        .subcommand(new_task_cmd())
        .subcommand(
            App::new("test")
                .about("controls testing features")
                .version("1.3")
                .author("Someone E. <someone_else@other.com>")
                .arg(
                    Arg::new("debug")
                        .short('d')
                        .about("print debug information verbosely")
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
    match App::try_get_matches_from(CLIAPP.to_owned(), args.clone()) {
        Ok(matches) => {
            cmd_match(&matches);
        }
        Err(err) => {
            err.print().expect("Error writing Error");
        }
    };
}

// 获取全部子命令，用于构建commandcompleter
pub fn all_subcommand(app: &App, beginlevel: usize, input: &mut Vec<SubCmd>) {
    let nextlevel = beginlevel + 1;
    let mut subcmds = vec![];
    for iterm in app.get_subcommands() {
        subcmds.push(iterm.get_name().to_string());
        if iterm.has_subcommands() {
            all_subcommand(iterm, nextlevel, input);
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

fn cmd_match(matches: &ArgMatches) {
    let config = get_config().unwrap();
    let server = &config["server"];
    // let req = Request::new("http://dev:8888".to_string());
    let req = Request::new(server.clone());
    if matches.is_present("interact") {
        interact::run();
        return;
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    // match matches.occurrences_of("v") {
    //     0 => println!("Verbose mode is off"),
    //     1 => println!("Verbose mode is kind of on"),
    //     _ => println!("Don't be crazy"),
    // }

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
                // Some("bygroupids") => {
                //     println!("{}", "bygroupids");
                //     if let Some(argmatches) = list.subcommand_matches("bygroupids") {
                //         if let Some(groupids) = argmatches.value_of("tasksgroupid") {
                //             let rt = tokio::runtime::Runtime::new().unwrap();
                //             let async_req = async {
                //             let groupidsarray = groupids.split(',').collect::<Vec<&str>>();
                //             let resp = req.task_list_by_groupids(groupidsarray).await;
                //             let result=ReqResult::new(resp);
                //         }
                //             rt.block_on(async_req);
                //         };
                //     };
                // }
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
