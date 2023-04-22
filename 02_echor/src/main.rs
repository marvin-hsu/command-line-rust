use clap::{Arg, ArgAction, Command};

fn main() {
    let matches = Command::new("echor")
        .version("0.1.0")
        .author("marvinhsu")
        .about("Rust echo")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                // clap4後 參數數量統一使用num_args
                .num_args(1..),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .help("Do not print newline")
                // clap4後對於無參數命令可轉用action設定Arg
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let text: Vec<&str> = matches
        .get_many::<String>("text")
        .unwrap()
        .map(|s| s.as_str())
        .collect();

    // clap4將is_present改為contains_id
    // 有預設值情況下contains_id接回傳true
    // 如果設定num_args(0)也會回傳true
    // 改用ArgAction
    let omit_newline = matches.get_flag("omit_newline");

    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
}
