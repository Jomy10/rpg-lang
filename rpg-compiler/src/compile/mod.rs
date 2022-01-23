use std::{fs, thread};
use std::cmp::max;
use std::sync::Arc;
use std::time::Duration;
use simple_colors::Color;
use spinner::{SpinnerBuilder, SpinnerHandle};
use spinners::utils::spinner_data::SpinnerData;
use crate::{Parser, rm_comments, Tokenizer};
use crate::generator::Generator;
use crate::type_checker::TypeChecker;
use crate::user_output::CompileError;

pub fn compile(file: &str) -> String {
    let sp = ColoredSpinner::new("Reading input...".to_string());
    let code = fs::read_to_string(file).expect_compile_error(&format!("{file} could not be found."));
    let code = rm_comments(&code);
    let code = code.trim();
    sp.stop(); println!();
    let sp = ColoredSpinner::new("Tokenizing...".to_string());
    let mut tokens = Tokenizer::new(&code).tokenize();
    sp.stop(); println!();
    let sp = ColoredSpinner::new("Parsing...".to_string());
    let parsed = Parser::new(&mut tokens).parse();
    let parsed = Arc::new(parsed);
    sp.stop(); println!();
    let thread_parsed = parsed.clone();
    let type_checker = thread::spawn(move || {
        TypeChecker::new(&thread_parsed).check_types();
    });
    let sp = ColoredSpinner::new("Generating...".to_string());
    let generated = Generator::new(&parsed).generate();
    sp.stop(); println!();
    type_checker.join().expect("Unable to join type-checker thread.");
    generated
}

pub struct Config {
    pub max_char: usize,
    pub verbose: bool
}

pub unsafe fn compile_with_config(file: &str, conf: Config) -> String {
    let sp = ColoredSpinner::new("Reading input...".to_string());
    let max_char = conf.max_char;
    let verbose = conf.verbose;
    if max_char > 10 { println!("Cheater :(") }
    crate::generator::MAX_CHAR = max_char;
    crate::user_output::VERBOSE = verbose;
    let code = fs::read_to_string(file).expect_compile_error(&format!("{file} could not be found."));
    let code = rm_comments(&code);
    let code = code.trim();
    sp.stop(); println!();
    let sp = ColoredSpinner::new("Tokenizing...".to_string());
    let mut tokens = Tokenizer::new(&code).tokenize();
    sp.stop(); println!();
    let sp = ColoredSpinner::new("Parsing...".to_string());
    let parsed = Parser::new(&mut tokens).parse();
    let parsed = Arc::new(parsed);
    sp.stop(); println!();
    // if let Some(handler) = &handler { &handler.stop_parsing; }
    // TODO: pb for type checker
    let thread_parsed = parsed.clone();
    let type_checker = thread::spawn(move || {
        TypeChecker::new(&thread_parsed).check_types();
    });
    let sp = ColoredSpinner::new("Generating...".to_string());
    let generated = Generator::new(&parsed).generate();
    sp.stop(); println!();
    // if let Some(handler) = &handler { &handler.stop_generating; }
    type_checker.join().expect("Unable to join type-checker thread.");
    generated
}

struct ColoredSpinner {
    handle: SpinnerHandle,
}

impl ColoredSpinner {
    /// Create a new spinner along with a message
    ///
    /// Returns a spinner
    fn new(message: String) -> Self {
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