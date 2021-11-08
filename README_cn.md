# interactcli-rs

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
