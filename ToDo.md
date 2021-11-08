# ToDo

- [x] 关闭 rustyreadline 的log,将日志级别设为info，debug会输出rustyreadline 日志
- [ ] 集成类似golang viper 的配置文件管理组件,config-rs,目前config-rs还不支持多层配置，只支持kv模式
- [x] command autocomplete.思路：利用rustreadline的completer trate 自己实现