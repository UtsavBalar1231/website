/// Bash/Shell script language-specific tokenization rules

pub struct BashLanguage;

impl BashLanguage {
    /// Enhanced Bash keyword detection
    pub fn is_keyword(text: &str) -> bool {
        matches!(
            text,
            // Control structures
            "if" | "then" | "else" | "elif" | "fi" | "case" | "esac" | "for" | "select" |
            "while" | "until" | "do" | "done" | "function" | "time" | "coproc" |
            "in" | "break" | "continue" | "return" | "exit" | "trap" | "wait" |

            // Built-in commands
            "echo" | "printf" | "read" | "test" | "cd" | "pwd" | "pushd" | "popd" |
            "dirs" | "jobs" | "bg" | "fg" | "disown" | "kill" | "killall" |
            "export" | "unset" | "set" | "unalias" | "alias" | "source" | "eval" |
            "exec" | "shift" | "getopts" | "declare" | "local" | "readonly" |
            "typeset" | "let" | "compgen" | "complete" | "shopt" | "bind" |

            // Common external commands
            "ls" | "mkdir" | "rmdir" | "rm" | "cp" | "mv" | "ln" | "find" | "grep" |
            "sed" | "awk" | "sort" | "uniq" | "cut" | "tr" | "head" | "tail" |
            "cat" | "less" | "more" | "file" | "which" | "whereis" | "locate" |
            "chmod" | "chown" | "chgrp" | "umask" | "du" | "df" | "mount" | "umount" |
            "ps" | "top" | "htop" | "pgrep" | "pkill" | "nohup" | "screen" | "tmux" |
            "tar" | "gzip" | "gunzip" | "zip" | "unzip" | "curl" | "wget" | "ssh" |
            "scp" | "rsync" | "git" | "make" | "gcc" | "g++" | "clang" | "cargo" |
            "npm" | "yarn" | "pip" | "apt" | "yum" | "dnf" | "pacman" | "sudo" | "su" |

            // Miscellaneous
            "uname" | "uptime" | "who" | "whoami" | "last" | "history" |
            "env" | "setenv" | "printenv" | "lsmod" | "modprobe" | "insmod" |
            "rmmod" | "dmesg" | "lsof" | "strace" | "ltrace" | "vmstat" |
            "iostat" | "mpstat" | "free" | "iotop" | "atop" | "sar" | "watch" |
            "cron" | "crontab" | "systemctl" | "service" | "chkconfig" |
            "sysctl" | "journalctl" | "dstat" | "iftop" | "netstat" | "ss" |
            "ip" | "ifconfig" | "route" | "ping" | "traceroute" | "dig" |
            "nslookup" | "host" | "iptables" | "ufw" | "selinux" | "apparmor" |
            "auditd" | "systemd" | "sysvinit" | "upstart"
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bash_keywords() {
        assert!(BashLanguage::is_keyword("if"));
        assert!(BashLanguage::is_keyword("echo"));
        assert!(BashLanguage::is_keyword("sudo"));
        assert!(!BashLanguage::is_keyword("my_script"));
    }
}

