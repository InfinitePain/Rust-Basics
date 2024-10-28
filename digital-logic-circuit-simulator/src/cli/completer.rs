use rustyline::{
    completion::{Completer, Pair},
    error::ReadlineError,
    highlight::Highlighter,
    hint::{Hinter, HistoryHinter},
    validate::{ValidationContext, ValidationResult, Validator},
};

pub struct SimulatorHelper {
    pub hinter: HistoryHinter,
    pub colored_prompt: String,
    pub commands: Vec<String>,
}

impl rustyline::Helper for SimulatorHelper {}

impl Completer for SimulatorHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        _pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> Result<(usize, Vec<Pair>), ReadlineError> {
        let mut matches = Vec::new();

        // Find matching commands
        for command in &self.commands {
            if command.starts_with(line) {
                matches.push(Pair {
                    display: command.clone(),
                    replacement: command.clone(),
                });
            }
        }

        Ok((0, matches))
    }
}

impl Highlighter for SimulatorHelper {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        _prompt: &'p str,
        _default: bool,
    ) -> std::borrow::Cow<'b, str> {
        std::borrow::Cow::Borrowed(&self.colored_prompt)
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> std::borrow::Cow<'h, str> {
        std::borrow::Cow::Borrowed(hint)
    }
}

impl Hinter for SimulatorHelper {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, ctx: &rustyline::Context<'_>) -> Option<String> {
        self.hinter.hint(line, pos, ctx)
    }
}

impl Validator for SimulatorHelper {
    fn validate(
        &self,
        ctx: &mut ValidationContext,
    ) -> rustyline::Result<ValidationResult> {
        let input = ctx.input();
        if input.trim().is_empty() {
            return Ok(ValidationResult::Invalid(
                Some("Please enter a command".into())
            ));
        }
        Ok(ValidationResult::Valid(None))
    }
}
