# interactcli-rs

[English](README.md)

interactcli-rs 是一个命令行程序框架，用于解决命令行与交互模式一体化问题，包括命令行交互模式统一、子命令提示等功能。该框架集成了clap和shellwords。

## 快速指南
框架中包含了访问www.baidu.com的例子

快速上手过程如下：

* clone 项目

  ```shell
  git clone https://github.com/jiashiwen/interactcli-rs.git
  cd 
  ```

* 命令行模式

  ```shell
  cargo run requestsample baidu
  ```

* 交互模式
  
  ```shell
  cargo run -- -i
  interact-rs> requestsample baidu
  ```

## 交互模式

交互模式下使用"Tab"键，进行命令提示

## 开发步骤

* 定义命令
  cmd 模块用于定义命令以及相关子命令

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

  new_requestsample_cmd 函数定义了命令 "requestsample",get_baidu_cmd 函数定义了 requestsample 的子命令 baidu

* 注册命令
  src/cmd/rootcmd.rs 文件中定义了命令树，可以在此注册定义好的子命令

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

  定义好的命令不需其他处理，框架会在系统运行时生成子命令树，用于命令提示的支持


* 命令解析
  src/cmd/rootcmd.rs 中的 cmd_match 负责解析命令，可以把解析逻辑写在该函数中

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
  
* 修改交互模式的命令提示
  提示符可以在src/interact/cli.rs 中定义

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
