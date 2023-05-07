#[allow(unused)]
#[allow(non_snake_case)]
pub mod FrontendInterface {
    use std::borrow::Cow::{self, Borrowed, Owned};

    use crate::define::*;
    use crate::RegexHandler::RegexHandler::RegexHandler;
    use rustyline::completion::FilenameCompleter;
    use rustyline::error::ReadlineError;
    use rustyline::highlight::{Highlighter, MatchingBracketHighlighter};
    use rustyline::hint::HistoryHinter;
    use rustyline::validate::MatchingBracketValidator;
    use rustyline::{Cmd, CompletionType, Config, EditMode, Editor, KeyEvent};
    use rustyline_derive::{Completer, Helper, Hinter, Validator};

    #[derive(Helper, Hinter, Completer, Validator)]
    struct MyHelper {
        completer: FilenameCompleter,
        highlighter: MatchingBracketHighlighter,
        validator: MatchingBracketValidator,
        hinter: HistoryHinter,
    }
    impl Highlighter for MyHelper {
        fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
            &'s self,
            prompt: &'p str,
            default: bool,
        ) -> Cow<'b, str> {
            if default {
                Borrowed(prompt)
            } else {
                Borrowed(prompt)
            }
        }

        fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
            Owned("\x1b[1m".to_owned() + hint + "\x1b[m")
        }

        fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
            self.highlighter.highlight(line, pos)
        }

        fn highlight_char(&self, line: &str, pos: usize) -> bool {
            self.highlighter.highlight_char(line, pos)
        }
    }
    pub fn handleFrontend(argc: usize, argv: &Vec<&str>) -> rustyline::Result<i32> {
        let mut rh: RegexHandler = RegexHandler::new();
        if argc == 3 && argv[1] == "run" {
            let run_command = "run ".to_owned() + argv[2];
            let ret = rh.handle(&run_command);
            match ret {
                Err(ErrorType::EXIT) => {
                    return Ok(0);
                }
                _ => {
                    return Ok(1);
                }
            };
        }
        let config = Config::builder()
            .history_ignore_space(true)
            .completion_type(CompletionType::List)
            .edit_mode(EditMode::Vi)
            .build();

        let h = MyHelper {
            completer: FilenameCompleter::new(),
            highlighter: MatchingBracketHighlighter::new(),
            hinter: HistoryHinter {},
            validator: MatchingBracketValidator::new(),
        };

        let mut rl = Editor::with_config(config)?;
        rl.set_helper(Some(h));
        rl.bind_sequence(
            KeyEvent::new('\t', rustyline::Modifiers::NONE),
            Cmd::Insert(1, "\t".into()),
        );

        let prompt: String = String::from("\x1b[1;32m#\x1b[0m");
        loop {
            let readline = rl.readline(prompt.as_str());
            match readline {
                Ok(line) => {
                    println!("{}", line);
                    if !line.is_empty() {
                        rl.add_history_entry(line.clone())?;
                    }
                    let ret = rh.handle(&line);
                    match ret {
                        Err(ErrorType::EXIT) => {
                            return Ok(0);
                        }
                        _ => {}
                    }
                }
                Err(_) => break,
            }
        }
        Ok(0)
    }
}
