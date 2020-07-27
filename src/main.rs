use std::process::Command;

fn main() {
    // Reading the log

    let log_command =
        Command::new("docker")
                .args(&["logs", "--tail", "100", "mc"])
                .output()
                .expect("docker logs failed");

    let log = String::from_utf8(log_command.stdout).unwrap();
    let lines: Vec<&str> = log.split("\n").collect();

    for line in lines.into_iter() {
        let len = line.len();
        let start_bytes = line.find('<').unwrap_or(0);
        let end_bytes = line.find('>').unwrap_or(0);

        let username_sliced = &line[start_bytes..end_bytes];
        let username: String = username_sliced.chars().skip(1).collect();

        let line_sliced = &line[end_bytes..len];
        let line_content: String = line_sliced.chars().skip(2).collect();

        if end_bytes > start_bytes {
            println!("Username: {}", username);
            println!("Command: {}", line_content);
        }

        if line_content.starts_with("thunder to ")
        {
            let coords = line_content.replace("thunder to ", "");
            println!("{}", coords);

            let mut tpcoords: Vec<&str> = coords.trim().split(" ").collect();
            let mut tpcommand = format!("tp {} {}", username, coords.trim());
            println!("{}", tpcommand);

            if tpcoords.len() == 3 {
                let rcon_command =
                    Command::new("docker")
                        .args(&["exec", "-i"])
                        .args(&["mc", "rcon-cli"])
                        .arg(&tpcommand)
                        .status()
                        .expect("Failed to execute process");
            }
        }

        if line_content.starts_with("please thunder a golden apple!")
        {
            let rcon_command =
                Command::new("docker")
                    .args(&["exec", "-i"])
                    .args(&["mc", "rcon-cli"])
                    .args(&["give", "adronomicon", "golden_apple"])
                    .status()
                    .expect("Failed to execute process");
        }
    }

    // Commands as reactions to the log

    // let rcon_command =
    //     Command::new("docker")
    //             // .args(&["ps", "-a"])
    //             .args(&["exec", "-i"])
    //             .args(&["mc", "rcon-cli"])
    //             .args(&["give", "adronomicon", "golden_apple"])
    //             .status()
    //             .expect("Failed to execute process");

    // println!("Result: {}", rcon_command);
}
