use std::process::Command;

fn main() {
    // Reading the log

    let logcommand =
        Command::new("docker")
                .args(&["logs", "--tail", "100", "mc"])
                .output()
                .expect("docker logs failed");

    let log = String::from_utf8(logcommand.stdout).unwrap();
    let lines: Vec<&str> = log.split("\n").collect();

    for line in lines.into_iter() {

        // Analyzing

        let len = line.len();
        let start_bytes = line.find('<').unwrap_or(0);
        let end_bytes = line.find('>').unwrap_or(0);

        let user_slice = &line[start_bytes..end_bytes];
        let user: String = user_slice.chars().skip(1).collect();

        let line_sliced = &line[end_bytes..len];
        let line: String = line_sliced.chars().skip(2).collect();

        if end_bytes > start_bytes {
            println!("user: {}", user);
            println!("command: {}", line);
        }

        // Reactions

        let tp_keysave = "in the name of light i claim the land of ";
        if line.starts_with(tp_keysave)
        {
            let tpcommand = format!("tp {} {}", user, "~ ~ ~");
            println!("{}", tpcommand);

            Command::new("docker")
                .args(&["exec", "-i"])
                .args(&["mc", "rcon-cli"])
                .arg(&tpcommand)
                .status()
                .expect("Failed to execute process");
        }

        let tpkey = "i summon the light to ";
        if line.starts_with(tpkey)
        {
            let coords = line.replace(tpkey, "");
            println!("{}", coords);

            let tpcoords: Vec<&str> = coords.trim().split(" ").collect();
            let tpcommand = format!("tp {} {}", user, coords.trim());
            println!("{}", tpcommand);

            if tpcoords.len() == 3 {
                Command::new("docker")
                    .args(&["exec", "-i"])
                    .args(&["mc", "rcon-cli"])
                    .arg(&tpcommand)
                    .status()
                    .expect("Failed to execute process");
            }
        }

        if line.starts_with("please thunder a golden apple!")
        {
            // let rcon_command =
            //     Command::new("docker")
            //         .args(&["exec", "-i"])
            //         .args(&["mc", "rcon-cli"])
            //         .args(&["give", "adronomicon", "golden_apple"])
            //         .status()
            //         .expect("Failed to execute process");
        }
    }
}
