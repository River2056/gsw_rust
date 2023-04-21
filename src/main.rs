use dialoguer::{console::Term, theme::ColorfulTheme, FuzzySelect};
use std::process::Command;
use std::str;

fn main() -> std::io::Result<()> {
    let branches = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "git branch"])
            .output()
            .expect("failed to process command")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("cd ~/excalibur && git branch")
            .output()
            .expect("failed to process command")
    };

    let branch_str = String::from_utf8(branches.stdout).unwrap();
    let branch_arr = branch_str.split("\n").collect::<Vec<&str>>();
    let mut initial_branch: String = String::new();
    for branch in &branch_arr {
        if branch.starts_with("*") {
            initial_branch = branch.replace("*", "").trim().to_owned();
        }
    }

    println!("pick a branch:");
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("pick a branch, current branch: {initial_branch}"))
        .items(&branch_arr)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    match selection {
        Some(index) => {
            let selected_branch = branch_arr[index].trim();
            println!("Selected branch: {}", selected_branch);
            let switch_branch = Command::new("sh")
                .arg("-c")
                .arg(format!(
                    "cd /root/excalibur && git checkout {selected_branch}"
                ))
                .output()
                .expect("Failed to checkout to branch!");
            // println!("{:?}", switch_branch);
            println!("{}", String::from_utf8(switch_branch.stdout).unwrap());
            println!("{}", String::from_utf8(switch_branch.stderr).unwrap());
        }
        None => println!("Did not select any branch!"),
    }

    Ok(())
}
