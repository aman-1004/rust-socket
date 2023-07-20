use std::process::Command;

fn main() {
    let output = Command::new("tcpdump")
        .arg("port 445")
        .output()
        .expect("Failed to execute command");
    let stdout = output.stdout;

    let text: String = stdout.into_iter().map(|x| x as char).collect();
    let texterr: String = output.stderr.into_iter().map(|x| x as char).collect();
    println!("{}", text);
    println!("{}", texterr);
}
