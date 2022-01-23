use std::{env, fs};
use std::path::Path;
use std::process::Command;
use std::time::Duration;
use clap::{App, arg};
use directories_next::ProjectDirs;
use rpg_compiler::{Config};
use rpg_compiler::user_output::CompileError;
use simple_colors::{blue, green};
use spinner::{SpinnerHandle, SpinnerBuilder};
use spinners::utils::spinner_data::SpinnerData;

pub struct ColoredSpinner {
    handle: SpinnerHandle,
}

impl ColoredSpinner {
    /// Create a new spinner along with a message
    ///
    /// Returns a spinner
    pub fn new(message: String) -> Self {
        // Dots3
        let spinner_data = SpinnerData {frames: vec![
            "\x1b[34m⠋\x1b[0m",
            "\x1b[34m⠙\x1b[0m",
            "\x1b[34m⠚\x1b[0m",
            "\x1b[34m⠞\x1b[0m",
            "\x1b[34m⠖\x1b[0m",
            "\x1b[34m⠦\x1b[0m",
            "\x1b[34m⠴\x1b[0m",
            "\x1b[34m⠲\x1b[0m",
            "\x1b[34m⠳\x1b[0m",
            "\x1b[34m⠓\x1b[0m"
        ], interval: 80};
        
        let handle = SpinnerBuilder::new(message)
            .spinner(spinner_data.frames.clone())
            .step(Duration::from_millis(spinner_data.interval.into()))
            .start();
        
        ColoredSpinner { handle }
    }
    
    /// Update spinner's message
    ///
    /// Returns the String that is put in in case the sender could not send.
    pub fn message(&self, message: String) -> Option<String> {
        self.handle.update(message)
    }
    
    /// Stop the spinner
    pub fn stop(self) {
        self.handle.close();
    }
}

fn main() {
    let dir = ProjectDirs::from("be", "jonaseveraert", "rpgc").expect("No valid home directory path could be retrieved from the operating system");
    let data_dir = dir.data_dir();
    let matches = App::new("RPG Compiler")
        .version("0.1.0")
        .author("Jonas Everaert <info@jonaseveraert.be>")
        .about("The official compiler for the RPG esoteric programming language")
        .arg(arg!([file] "The .rpg source file you wish to compile"))
        .arg(
            arg!([output_dir] "Sets the output directory of the compiled app")
        )
        .arg(
            arg!(-r --release "Compiles the program with optimizations")
        )
        .arg(
            arg!(-d --debug "Compiles the program without optimization (default)")
        )
        .arg(
            arg!(-m --max_char <VALUE> "Optionally sets the maximum amount of characters allowed in the program, setting it to more than 10 is considered cheating, though.")
                .required(false)
        )
        .arg(
            arg!(-v --verbose "Prints out more error messages")
        )
        .subcommand(
            App::new("clean")
                .about("Cleans the build folder"),
        )
        .get_matches();
    
    if let Some(file) = matches.value_of("file") {
        let debug = !matches.is_present("release");
        let verbose = matches.is_present("verbose");
        let max_char = matches.value_of("max_char");
        let _output_dir = matches.value_of("output_dir");
        let cd = env::current_dir().expect_compile_error("Could not find current working directory");
        let output_dir: &Path;
        if let Some(dir) = _output_dir {
            output_dir = Path::new(dir);
        } else {
            output_dir = cd.as_path()
        };
        let app_name = "rpg"; // TODO: argument for app name
        
        let compiled = if verbose == true || max_char.is_some() {
            unsafe {
                rpg_compiler::compile_with_config(
                    file,
                    Config {
                        max_char: if max_char.is_some() { max_char.unwrap().parse::<usize>().expect_compile_error("Did not specify a valid number for max_char") } else { 10 },
                        verbose
                    },
                )
            }
        } else {
            rpg_compiler::compile(file)
        };
        
        let compiled_path = Path::new(data_dir).join("tmp_compiled");
        
        if !compiled_path.exists() {
            fs::create_dir_all(&compiled_path).expect_compile_error("Couldn't create working directory.");
            fs::create_dir(&compiled_path.join("src")).expect_compile_error("Couldn't create working directory.");
            fs::write(&compiled_path.join("Cargo.toml"), CARGO_TOML).expect_compile_error("Couldn't create working directory.");
        }
        
        fs::write(&compiled_path.join("src").join("main.rs"), compiled).expect_compile_error("Couldn't write compiled source file.");
    
        let sp = ColoredSpinner::new("Compiling rust project...".to_string());
        let o = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", &format!("cd \"{}\" && cargo build{}", compiled_path.to_str().expect_compile_error(
                    "unable to convert compiled path to string."
                    ),
                    if debug {""} else {" --release"}
                ).trim()])
                .output()
                .expect_compile_error("Failed to execute rust compiler")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(&format!("cd \"{}\" && cargo build{}", compiled_path.to_str().expect_compile_error(
                    "unable to convert compiled path to string."
                    ),
                    if debug {""} else {" --release"}
                ).trim())
                .output()
                .expect_compile_error("Failed to execute rust compiler")
        };
        sp.stop();
    
        if verbose {
            let out = String::from_utf8(o.stdout).expect("Couldn't convert utf8.");
            let out = out.trim();
            let err = String::from_utf8(o.stderr).expect("Couldn't convert utf8.");
            let err = err.trim();
            println!("{out}");
            println!("{err}");
        }
        
        // Copy compiled to output_dir
        println!();
        let sp = ColoredSpinner::new("Copying...".to_string());
        let output_dir = Path::new(output_dir);
        let compiled_file_dir = compiled_path.join("target").join("debug").join(app_name);
        fs::write(output_dir.join(app_name), fs::read(compiled_file_dir).expect_compile_error("Compiled file not found or no read access.")).expect_compile_error("Couldn't write output file.");
    
        sp.message("Setting file permissions...".to_string());
        let o = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", &format!("icacls {app_name} /grant user:(gw,ge,d,wd,ra,rea)")])
                .output()
                .expect_compile_error("Failed to execute rust compiler")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(&format!("chmod +x {app_name}"))
                .output()
                .expect_compile_error("Failed to execute rust compiler")
        };
        sp.stop();
        
        if verbose {
            println!("{}", String::from_utf8(o.stdout).expect("Couldn't parse stdout").trim());
            println!("{}", String::from_utf8(o.stderr).expect("Couldn't parse stderr").trim());
        }
        println!("\n{}", blue!("Compilation successful."))
    }
    else if let Some(("clean", _)) = matches.subcommand() {
        fs::remove_dir_all(data_dir).expect("Couldn't remove working directory.");
        println!("{}", green!("Cleaned build folder"))
    } else {
        println!("Please specify a source file")
    }
}

/// The cargo.toml for the tmp_compiled dir
const CARGO_TOML: &str =
r#"[package]
name = "rpg"
version = "0.1.0"
edition = "2021""#;
