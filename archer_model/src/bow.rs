use crate::arrow::Arrow;

pub enum Error
{
    /// String contains reason for failure
    Failed( String ),
    Unknown
}

/// Archer's Bow will fires Arrows on the target
/// Bow executes commands in a virtual shell
/// It is stateful, meaning the environment is persisted until it is destroyed
pub trait Bow
{
    /// Shoot an Arrow / Execute a command 
    /// return the result of the command
    /// or an Error if failed
    fn shoot( arrow: &Arrow ) -> Result<String, Error>;
}

// future implementation
// use cmd_lib::*;
// use std::io::{BufRead, BufReader};

// fn main() -> CmdResult {
//     // use_builtin_cmd!(echo, info);

//     // // Read input from the user
//     // let mut input_line = String::new();
//     // println!("Enter your command:");

//     // io::stdin()
//     //     .read_line(&mut input_line)
//     //     .expect("Failed to read line");
//     // let user_cmd = input_line.trim();

//     // // Spawn the command and read its output
//     // spawn_with_output!(conan)?.wait_with_pipe(&mut |pipe| {
//     //     let reader = BufReader::new(pipe);
//     //     for line in reader.lines() {
//     //         match line {
//     //             Ok(line_contents) => println!("{}", line_contents),
//     //             Err(err) => eprintln!("Error reading line: {}", err),
//     //         }
//     //     }
//     // })?;

//     conan(Command::Version {})
// }

// enum Command {
//     Version,
//     // Help,
//     // Install { profile: String, update: bool },
//     // Create { profile: String },
// }

// fn execute_conan(args: String) -> CmdResult {
//     spawn_with_output!( conan $args )?.wait_with_pipe(&mut |pipe| {
//         let reader = BufReader::new(pipe);
//         for line in reader.lines() {
//             match line {
//                 Ok(line_contents) => println!("{}", line_contents),
//                 Err(err) => eprintln!("Error reading line: {}", err),
//             }
//         }
//     })?;

//     Ok(())
// }

// fn conan(cmd: Command) -> CmdResult {
//     match cmd {
//         Command::Version => execute_conan("--version".to_string()),
//     }
// }
