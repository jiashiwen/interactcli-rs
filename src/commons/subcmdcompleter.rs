use rustyline::completion::{Completer, Pair};
use rustyline::Context;
use rustyline::Result;

// const DOUBLE_QUOTES_ESCAPE_CHAR: Option<char> = Some('\\');

#[derive(Debug, Clone)]
pub struct SubCmd {
    pub level: usize,
    pub command_name: String,
    pub subcommands: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct CommandCompleter {
    subcommands: Vec<SubCmd>,
}

impl CommandCompleter {
    pub fn new(subcmds: Vec<SubCmd>) -> Self {
        Self {
            subcommands: subcmds,
        }
    }

    //获取level下所有可能的子命令
    pub fn level_possible_cmd(&self, level: usize) -> Vec<String> {
        let mut subcmds = vec![];
        let cmds = self.subcommands.clone();
        for iterm in cmds {
            if iterm.level == level {
                subcmds.push(iterm.command_name.clone());
            }
        }
        return subcmds;
    }
    //获取level下某字符串开头的子命令
    pub fn level_prefix_possible_cmd(&self, level: usize, prefix: &str) -> Vec<String> {
        let mut subcmds = vec![];
        let cmds = self.subcommands.clone();
        for iterm in cmds {
            if iterm.level == level && iterm.command_name.starts_with(prefix) {
                subcmds.push(iterm.command_name);
            }
        }
        return subcmds;
    }

    //获取某level 下某subcommand的所有子命令
    pub fn level_cmd_possible_sub_cmd(&self, level: usize, cmd: String) -> Vec<String> {
        let mut subcmds = vec![];
        let cmds = self.subcommands.clone();
        for iterm in cmds {
            if iterm.level == level && iterm.command_name == cmd {
                subcmds = iterm.subcommands.clone();
            }
        }
        return subcmds;
    }

    //获取某level 下某subcommand的所有prefix子命令
    pub fn level_cmd_possible_prefix_sub_cmd(
        &self,
        level: usize,
        cmd: String,
        prefix: &str,
    ) -> Vec<String> {
        let mut subcmds = vec![];
        let cmds = self.subcommands.clone();
        for iterm in cmds {
            if iterm.level == level && iterm.command_name == cmd {
                for i in iterm.subcommands {
                    if i.starts_with(prefix) {
                        subcmds.push(i);
                    }
                }
            }
        }
        return subcmds;
    }

    pub fn complete_cmd(&self, line: &str, pos: usize) -> Result<(usize, Vec<Pair>)> {
        let mut entries: Vec<Pair> = Vec::new();
        let d: Vec<_> = line.split(' ').collect();

        if d.len() == 1 {
            if d.last() == Some(&"") {
                for str in self.level_possible_cmd(1) {
                    let mut replace = str.clone();
                    replace.push_str(" ");
                    entries.push(Pair {
                        display: str.clone(),
                        replacement: replace,
                    });
                }
                return Ok((pos, entries));
            }

            if let Some(last) = d.last() {
                for str in self.level_prefix_possible_cmd(1, *last) {
                    let mut replace = str.clone();
                    replace.push_str(" ");
                    entries.push(Pair {
                        display: str.clone(),
                        replacement: replace,
                    });
                }
                return Ok((pos - last.len(), entries));
            }
        }

        if d.last() == Some(&"") {
            for str in self
                .level_cmd_possible_sub_cmd(d.len() - 1, d.get(d.len() - 2).unwrap().to_string())
            {
                let mut replace = str.clone();
                replace.push_str(" ");
                entries.push(Pair {
                    display: str.clone(),
                    replacement: replace,
                });
            }
            return Ok((pos, entries));
        }

        if let Some(last) = d.last() {
            for str in self.level_cmd_possible_prefix_sub_cmd(
                d.len() - 1,
                d.get(d.len() - 2).unwrap().to_string(),
                *last,
            ) {
                let mut replace = str.clone();
                replace.push_str(" ");
                entries.push(Pair {
                    display: str.clone(),
                    replacement: replace,
                });
            }
            return Ok((pos - last.len(), entries));
        }

        Ok((pos, entries))
    }
}

impl Completer for CommandCompleter {
    type Candidate = Pair;

    fn complete(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> Result<(usize, Vec<Pair>)> {
        self.complete_cmd(line, pos)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn command_completer_test() {
        let mut subcmds: Vec<SubCmd> = vec![];
        subcmds.push(SubCmd {
            level: 0,
            command_name: "level0_cmd0".to_string(),
            subcommands: vec!["level1_cmd1".to_string(), "level1_cmd2".to_string()],
        });
        subcmds.push(SubCmd {
            level: 1,
            command_name: "level1_cmd1".to_string(),
            subcommands: vec![
                "cmd11".to_string(),
                "level2_cmd1_1".to_string(),
                "level2_cmd1_2".to_string(),
            ],
        });
        subcmds.push(SubCmd {
            level: 1,
            command_name: "level1_cmd2".to_string(),
            subcommands: vec![
                "letcmd".to_string(),
                "level2_cmd2_1".to_string(),
                "level2_cmd2_2".to_string(),
            ],
        });

        let cmd_completer = CommandCompleter::new(subcmds);

        assert_eq!(
            Some(&"level0_cmd0".to_string()),
            cmd_completer.level_possible_cmd(0).get(0)
        );
        assert_eq!(
            Some(&"cmd11".to_string()),
            cmd_completer
                .level_cmd_possible_sub_cmd(1, String::from("level1_cmd1"))
                .get(0)
        );
        assert_eq!(
            Some(&"letcmd".to_string()),
            cmd_completer
                .level_cmd_possible_prefix_sub_cmd(1, String::from("level1_cmd2"), "let")
                .get(0)
        );
    }
}
