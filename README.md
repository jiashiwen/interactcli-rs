# interactcli-rs

[简体中文](README_cn.md)

interactcli-rs is a command-line program framework used to solve the problem of the integration of command-line and interactive modes, including functions such as unification of command-line interactive modes and sub-command prompts. The framework integrates clap and shellwords.

## quick guide

The frame contains examples of visiting www.baidu.com

The quick start process is as follows:

* clone project

  ```shell
  git clone https://github.com/jiashiwen/interactcli-rs.git
  cd
  ```

* Command line mode

  ```shell
  cargo run requestsample baidu
  ```

* Interactive mode
  
  ```shell
  cargo run -- -i
  interact-rs> requestsample baidu
  ```

## Interactive mode

Use "Tab" key in interactive mode to prompt command

## Development steps

* Define commands
  The cmd module is used to define commands and related subcommands

  ```rust
  use clap::App;
  
  pub fn new_requestsample_cmd() -> App<'static> {
      clap::App::new("requestsample")
          .about("requestsample")
          .subcommand(get_baidu_cmd())
  }
  
  pub fn get_baidu_cmd() -> App<'static> {
      clap::App::new("baidu").about("request www.baidu.com")
  }

  ```

  The new_requestsample_cmd function defines the command "requestsample", and the get_baidu_cmd function defines the subcommand baidu of requestsample

* Register order
  The command tree is defined in the src/cmd/rootcmd.rs file, and the defined subcommands can be registered here

  ```rust
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

  ```

  The defined command does not need other processing, the framework will generate a sub-command tree when the system is running, for the support of the command prompt


* Parse command
  The cmd_match in src/cmd/rootcmd.rs is responsible for parsing commands, and the parsing logic can be written in this function

  ```rust
  fn cmd_match(matches: &ArgMatches) {   
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
  }
  ```
  
* Modify the command prompt in interactive mode
  The prompt can be defined in src/interact/cli.rs

  ```rust
  pub fn run() {
    
    ...

    loop {
        let p = format!("{}> ", "interact-rs");
        rl.helper_mut().expect("No helper").colored_prompt = format!("\x1b[1;32m{}\x1b[0m", p);

        ...
    }
    
    ...
  }

  ```
