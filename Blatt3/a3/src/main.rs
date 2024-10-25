use std::time::Instant;
use std::time::SystemTime;
use std::io::{stdout, Read, Stdout, Write};
use termion::AsyncReader;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use ferris_says::say;
use rand::Rng;

enum GameEvent {
    KeyPress(Key),
    Quit,
    None,
}
struct App {
    stdin: AsyncReader,
    stdout: RawTerminal<Stdout>,
    game_event: GameEvent,
}

impl App {
    fn new() -> Self {
        let stdin = termion::async_stdin();
        let mut stdout = stdout().into_raw_mode().unwrap();
        let game_event = GameEvent::None;
        write!(stdout, "{}{}", termion::cursor::Goto(1, 1), termion::clear::All).unwrap();
        write!(stdout, "Welcome to the Ferris Reaction Timer Game!\n\r").unwrap();
        write!(stdout, "The game will show a random scary word after a delay of 1 to 5 seconds.\n\r").unwrap();
        write!(stdout, "Your task is to press any key as quickly as possible once you see the scary word.\n\r").unwrap();
        write!(stdout, "Press Ctrl+Q at any time to exit. Press any key to start the game...\n\r").unwrap();
        stdout.by_ref().flush().unwrap();
        App {
            stdin,
            stdout,
            game_event,
        }
    }

    fn get_event(&mut self) {
        if let Some(Ok(key)) = self.stdin.by_ref().keys().next() {
            match key {
                Key::Ctrl('q') => {
                    self.game_event = GameEvent::Quit;
                }
                other_key => {
                    self.game_event = GameEvent::KeyPress(other_key);
                }
            }
        } else {
            self.game_event = GameEvent::None;
        }
    }

    fn println(&mut self, s: &str) {
        write!(self.stdout, "{}\n\r", s).unwrap();
        self.stdout.flush().unwrap();
    }

    fn clean(&mut self) {
        write!(self.stdout, "{}{}", termion::cursor::Goto(1, 1), termion::clear::All).unwrap();
    }

    fn ferris_scare(&mut self) {
        let scare_words = vec!["Boo!", "Aaah!", "Ouch!", "Yikes!", "Eek!"];
        let scare_word = scare_words[rand::thread_rng().gen_range(0..scare_words.len())];

        let mut buffer = Vec::new();
        say(scare_word, scare_word.len(), &mut buffer).unwrap();

        let output = String::from_utf8(buffer).unwrap().replace("\n", "\n\r");

        self.stdout.write_all(output.as_bytes()).unwrap();
        self.stdout.flush().unwrap();
    }

    fn handle_scare_phase(&mut self) {
        let scare_time = std::time::Duration::from_millis(rand::random::<u64>() % 4000 + 1000);
        let end_time = SystemTime::now() + scare_time;

        self.println("Get ready...");

        while SystemTime::now() < end_time {
            self.process_frame();
            self.get_event();
            if let GameEvent::Quit = self.game_event {
                return;
            }
        }
    }

    fn handle_reaction_phase(&mut self) {
        self.clean();
        self.ferris_scare();
        self.println("Press any key now!");
        let reaction_time = Instant::now();

        loop {
            self.process_frame();
            self.get_event();
            match self.game_event {
                GameEvent::KeyPress(_) => {
                    let elapsed = reaction_time.elapsed().as_millis();
                    self.println(&format!("You took {} ms to react!", elapsed));
                    break;
                }
                GameEvent::Quit => return,
                _ => (),
            }
        }
    }

    fn process_frame(&mut self) {
        let frame_time = std::time::Duration::from_millis(1000 / 60);
        let start = Instant::now();
        std::thread::sleep(frame_time - start.elapsed());
    }

    fn run(&mut self) {
        loop {
            self.wait_for_start();
            self.handle_scare_phase();
            self.handle_reaction_phase();
            self.println("Press any key to restart the game or Ctrl+Q to exit...");

            self.get_event();
            if let GameEvent::Quit = self.game_event {
                break;
            }
        }
    }

    fn wait_for_start(&mut self) {
        loop {
            self.process_frame();
            self.get_event();
            match self.game_event {
                GameEvent::KeyPress(_) => break,
                GameEvent::Quit => return,
                _ => (),
            }
        }
    }

}

impl Drop for App {
    fn drop(&mut self) {
        self.clean();
        self.println("Thanks for playing! Goodbye!");
    }
}

fn main() {
    let mut app = App::new();
    app.run();
}
