use std::collections::HashMap;

pub struct Args {
    pub configs:        Vec<String>,
    pub subcommand:     Option<String>,
    pub long_options:   Vec<String>,
    pub short_options:  Vec<String>,
    pub config_options: HashMap<String, String>
}

impl Args {
    pub fn from(args: Vec<String>, config_options: &[&str]) -> Option<Args> {
        let args: Vec<&str> = args.iter().map(String::as_str).collect();
        let mut configs = Vec::<String>::new();
        let subcommand: Option<String>;
        let mut long_options = Vec::<String>::new();
        let mut short_options = Vec::<String>::new();
        let mut config_options_map = HashMap::<String, String>::new();
        if args.len() == 1 {
            return Option::None;
        }
        let retain: &[&str];
        if !args[1].starts_with("-") && !args[1].starts_with("--") {
            subcommand = Some(String::from(args[1]));
            retain = &args[2..];
        } else {
            subcommand = None;
            retain = &args[1..];
        }
        let mut prev_arg = Option::<&str>::None;
        for &arg in retain {
            if arg.starts_with("--") {
                long_options.push(String::from(arg));
            } else if arg.starts_with("-" ){
                short_options.push(String::from(arg));
            } else {
                if let Some(prev) = prev_arg {
                    if config_options.contains(&prev) {
                        short_options.pop();
                        config_options_map.insert(String::from(prev), String::from(arg));
                    } else {
                        configs.push(String::from(arg));
                    }
                } else {
                    configs.push(String::from(arg));
                }
            }
            prev_arg = Some(arg);
        }
        return Some(Args { configs, subcommand, long_options, short_options, config_options:config_options_map});
    }
}