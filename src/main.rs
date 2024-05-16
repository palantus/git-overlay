#![windows_subsystem = "windows"]
use regex::Regex;
use std::path::Path;
use std::process;
use std::process::Command;

use config::{read_config, Config};
use iced::{
    widget::{button, column, row, text, Column, Container},
    Padding, Sandbox, Settings, Theme,
};
mod config;

#[derive(Debug, Clone)]
pub enum Message {
    Refresh,
    Quit,
    Lazygit(String),
}

fn main() -> iced::Result {
    RustUI::run(Settings::default())
}

#[derive(Default, Clone)]
pub struct Repo {
    pub name: String,
    pub branch: String,
    pub num_changes: i32,
}

struct RustUI {
    theme: Theme,
    config: Config,
    repos: Vec<Repo>,
}

impl RustUI {
    fn refresh_repos(&mut self) {
        self.repos = self
            .config
            .repos
            .iter()
            .map(|r| {
                let (branch, num_changes) = refresh_status(&self.config.rootpath, &r);
                return Repo {
                    name: r.into(),
                    branch,
                    num_changes,
                };
            })
            .collect();
    }

    fn open_lazygit(&mut self, repo: &str) {
        let path = Path::new(&self.config.rootpath);
        let path = path.join(repo);

        if cfg!(windows) {
            let mut command = Command::new("cmd");
            command.args(&["/C", "start", "lazygit"]);
            command.current_dir(path);
            let _ = command.spawn();
        } else {
            let _ = Command::new("lazygit").current_dir(path).spawn();
        }
    }
}


impl Sandbox for RustUI {
    type Message = Message;

    fn new() -> Self {
        let config = read_config().expect("No config file found");
        let repos = config
            .clone()
            .repos
            .into_iter()
            .map(|r| Repo {
                name: r,
                branch: String::new(),
                num_changes: 0,
            })
            .collect();

        let mut ui = Self {
            theme: Theme::Dark,
            config,
            repos,
        };

        ui.refresh_repos();

        return ui;
    }

    fn title(&self) -> String {
        String::from("Git overlay")
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Refresh => {
                self.refresh_repos();
            }
            Message::Quit => {
                process::exit(0);
            }
            Message::Lazygit(repo) => {
                self.open_lazygit(&repo);
            }
        }
    }

    fn view(&self) -> iced::Element<Message> {
        let padding = Padding {
            top: 0.0,
            bottom: 0.0,
            left: 0.0,
            right: 20.0,
        };

        let repos = row![
            // Name
            self.repos
                .iter()
                .fold(Column::new().padding(padding), |col, repo| {
                    col.push(text(&repo.name).size(25))
                }),
            // Branch
            self.repos
                .iter()
                .fold(Column::new().padding(padding), |col, repo| {
                    col.push(text(&repo.branch).size(25))
                }),
            // Num changes
            self.repos
                .iter()
                .fold(Column::new().padding(padding), |col, repo| {
                    col.push(text(&repo.num_changes.to_string()).size(25))
                }),
            // Lazygit button
            self.repos
                .iter()
                .fold(Column::new().padding(padding), |col, repo| {
                    col.push(button("Lazygit").on_press(Message::Lazygit(repo.name.clone())))
                }),
        ];

        column![
            repos,
            Container::new(row![
                Container::new(button("Refresh").on_press(Message::Refresh)).padding(padding),
                Container::new(button("Quit").on_press(Message::Quit)).padding(padding),
            ])
            .padding(Padding {
                top: 10.0,
                right: 0.0,
                bottom: 0.0,
                left: 0.0
            }),
        ]
        .into()
    }
}

fn refresh_status(root: &String, repo: &String) -> (String, i32) {
    let path = Path::new(&root);
    let path = path.join(&repo);
    let output = Command::new("git")
        .current_dir(path)
        .arg("status")
        .output()
        .expect("failed to execute process");

    let output = String::from_utf8_lossy(&output.stdout);

    let num_changes = output.matches("\t").count();

    let re = Regex::new(r"branch (?<branch>\w+)").unwrap();
    let branch = &re.captures(&output).unwrap()["branch"];

    (
        branch.to_owned(),
        num_changes.try_into().unwrap_or_default(),
    )
}
