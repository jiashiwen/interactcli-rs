# interactcli-rs

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