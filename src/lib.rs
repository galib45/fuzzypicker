use std::io::{Stdout, stdout, Write};
use std::fmt::Display;
use std::clone::Clone;
use std::time::Duration;
use std::error::Error;
use crossterm::{
    QueueableCommand, 
    cursor::{MoveTo}, 
    style::{Stylize, Print, PrintStyledContent},
    terminal::{
        self, Clear, ClearType, 
        EnterAlternateScreen, LeaveAlternateScreen
    },
    event::{
        poll, read, Event, KeyCode, KeyEventKind, 
        EnableMouseCapture, DisableMouseCapture,
        MouseEventKind, MouseButton
    }
};
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

pub struct FuzzyPicker<T: Display + Clone> {
    stdout: Stdout, 
    matcher: SkimMatcherV2,
    items: Vec<T>, 
    display_items: Vec<String>, 
    num_of_items: usize,
    prompt: String, 
    selected: usize, 
    start_index: usize, 
    end_index: usize
}

impl<T: Display + Clone> FuzzyPicker<T> {
    pub fn new(items: &[T]) -> Self {
        let (_, h) = terminal::size().unwrap();
        Self {
            stdout: stdout(), 
            matcher: SkimMatcherV2::default(),
            items: items.to_vec(), 
            display_items: Vec::<String>::new(),
            num_of_items: 0,
            prompt: String::new(), 
            selected: 0,
            start_index: 0, 
            end_index: (h-1) as usize
        }
    }

    fn prev_item(&mut self) {
        self.selected = (self.selected + self.num_of_items - 1) % self.num_of_items
    }

    fn next_item(&mut self) {
        self.selected = (self.selected + 1) % self.num_of_items
    }

    fn reset_scroll(&mut self) {
        self.start_index = 0;
        self.selected = self.start_index;
    }

    pub fn pick(&mut self) -> Result<Option<T>, Box<dyn Error>> {
        let mut picked_item: Option<T> = None;
        self.stdout
        .queue(EnterAlternateScreen)?
        .queue(EnableMouseCapture)?;
        loop {
            if poll(Duration::from_millis(500))? {
                match read()? {
                    Event::Key(event) => {
                        if event.kind == KeyEventKind::Press {
                            match event.code {
                                KeyCode::Char(ch) => {
                                    self.prompt.push(ch);
                                    self.reset_scroll();
                                },
                                KeyCode::Backspace => {
                                    self.prompt.pop();
                                    self.reset_scroll();
                                }
                                KeyCode::Esc => {
                                    self.stdout
                                        .queue(LeaveAlternateScreen)?
                                        .queue(DisableMouseCapture)?;
                                    break;
                                },
                                KeyCode::Up | KeyCode::Left => {
                                    self.prev_item();
                                },
                                KeyCode::Down | KeyCode::Right => {
                                    self.next_item();
                                },
                                KeyCode::Enter => {
                                    self.stdout
                                        .queue(LeaveAlternateScreen)?
                                        .queue(DisableMouseCapture)?;
                                    picked_item = self.items.iter().find(
                                        |&item| format!("{item}") == self.display_items[self.selected]
                                    ).cloned();
                                    break;
                                },
                                _ => {}
                            }
                        }
                    },
                    Event::Mouse(event) => {
                        match event.kind { 
                            MouseEventKind::Down(MouseButton::Left) => {
                                if event.row < self.num_of_items as u16 +1 { // +1 for the row of prompt
                                    self.selected = (event.row-1) as usize + self.start_index;
                                }
                            },
                            MouseEventKind::ScrollUp => {
                                if self.start_index > 0
                                && self.end_index > 0 { 
                                    self.start_index -= 2;
                                    self.end_index -= 2;
                                    self.selected = self.start_index;
                                }
                            },
                            MouseEventKind::ScrollDown => {
                                if self.start_index < self.num_of_items 
                                && self.end_index < self.num_of_items { 
                                    self.start_index += 2;
                                    self.end_index += 2;
                                    self.selected = self.start_index;
                                }
                            },
                            _ => {}
                        }
                    },
                    Event::Resize(_, rows) => {
                        self.end_index = self.start_index + (rows-1) as usize;
                    },
                    _ => {}
                }
            }
            self.render_frame()?;	
        }
        Ok(picked_item)
    }

    fn render_frame(&mut self) -> Result<(), Box<dyn Error>> {
        self.stdout
            .queue(Clear(ClearType::All))?
            .queue(MoveTo(0, 0))?
            .queue(PrintStyledContent(format!("> {}", self.prompt).green().bold()))?;

        self.display_items = self.items.iter()
            .filter_map(|item| {
                let display_str = format!("{}", item);
                if self.prompt.is_empty() || self.matcher.fuzzy_match(
                    &display_str.to_lowercase(),
                    &self.prompt.to_lowercase(),
                ).unwrap_or_default() != 0 {
                    Some(display_str)
                } else {
                    None
                }
            })
            .collect();

        self.display_items.sort_by_key(|item| {
            -self.matcher.fuzzy_match(
                &item.to_lowercase(),
                &self.prompt.to_lowercase(),
            ).unwrap_or_default()
        });
        //self.display_items = <Vec<String> as Clone>::clone(&self.items).into_iter().filter(
        //    |s| self.prompt.is_empty() || self.matcher.fuzzy_match(
        //        s.to_lowercase().as_str(), self.prompt.to_lowercase().as_str()
        //    ).unwrap_or_default() != 0
        //).collect();
        //self.display_items.sort_by_key(
        //    |s| -self.matcher.fuzzy_match(
        //        s.to_lowercase().as_str(), self.prompt.to_lowercase().as_str()
        //    ).unwrap_or_default()
        //);
        self.num_of_items = self.display_items.len();
        let mut index = self.start_index;
        let mut row: u16 = 1; //row0 is for the prompt
        let mut item;
        while index < self.end_index && index < self.num_of_items {
            item = self.display_items[index].as_str();
            self.stdout
                .queue(MoveTo(0, row))?
                .queue(PrintStyledContent(" ".on_dark_grey()))?;
            if index == self.selected {
                self.stdout
                    .queue(PrintStyledContent(" ".on_dark_grey()))?
                    .queue(PrintStyledContent(
                        item.white().on_dark_grey()
                    ))?;
            } else {
                self.stdout.queue(Print(format!(" {}", item)))?;
            }
            index += 1; row += 1;
        }
        self.stdout.queue(MoveTo(self.prompt.len() as u16 + 2, 0))?;
        self.stdout.flush()?;
        Ok(())
    }
}
