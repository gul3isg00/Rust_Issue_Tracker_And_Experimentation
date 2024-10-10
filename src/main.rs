// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use chrono::Local;
use eframe::egui;
use std::{fmt, usize};

#[derive(PartialEq, Eq, Clone)]
enum ItemPriority {
    NA,
    Low,
    Medium,
    High,
    Critical,
}

impl fmt::Display for ItemPriority {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ItemPriority::NA => write!(f, "NA"),
            ItemPriority::Low => write!(f, "Low"),
            ItemPriority::Medium => write!(f, "Medium"),
            ItemPriority::High => write!(f, "High"),
            ItemPriority::Critical => write!(f, "Critical"),
        }
    }
}

enum ItemStatus {
    New,
    Open,
    Closed,
}

impl fmt::Display for ItemStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ItemStatus::New => write!(f, "New"),
            ItemStatus::Open => write!(f, "Open"),
            ItemStatus::Closed => write!(f, "Closed"),
        }
    }
}

enum UserType {
    Client,
    Admin,
}

#[allow(clippy::upper_case_acronyms)]
type DateTime = String;

struct ItemComment {
    commenter: String,
    message: String,
    time_created: DateTime,
}

struct Issue {
    name: String,
    description: String,
    reporter: String,
    comment_thread: Vec<ItemComment>,
    priority: ItemPriority,
    status: ItemStatus,
    time_created: DateTime,
}

impl Default for Issue {
    fn default() -> Self {
        Self {
            name: String::from("NA"),
            description: String::from("NA"),
            reporter: String::from("NA"),
            comment_thread: Vec::new(),
            priority: ItemPriority::NA,
            status: ItemStatus::New,
            time_created: String::from(&(Local::now().to_string())[..10]),
        }
    }
}

struct User {
    email: String,
    password: String,
    username: String,
    user_type: UserType,
}

fn main() -> eframe::Result {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([500.0, 500.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Rust Issue Tracker",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<ClientApp>::default())
        }),
    )
}

// Defines the variables stored within the app
struct ClientApp {
    users: Vec<User>,
    issues: Vec<Issue>,
    logged_in_as: usize,
    cur_ticket_message: String,
    cur_ticket_priority: ItemPriority,
    cur_ticket_name: String,
    cur_comment: String,
    show_ticket_form: bool,
    adding_comment: bool,
    focus_issue: usize,
}

// Instantiates the default variables.
impl Default for ClientApp {
    fn default() -> Self {
        Self {
            users: Vec::from([
                User {
                    email: String::from("client@test.com"),
                    password: String::from("encrypted-password"),
                    username: String::from("BiteAtTheHand"),
                    user_type: UserType::Client,
                },
                User {
                    email: String::from("admin@test.com"),
                    password: String::from("encrypted-password"),
                    username: String::from("TheBeesKnees"),
                    user_type: UserType::Admin,
                },
            ]),
            logged_in_as: 0,
            focus_issue: usize::MAX,
            cur_ticket_message: String::from(""),
            cur_ticket_name: String::from(""),
            cur_ticket_priority: ItemPriority::NA,
            cur_comment: String::from(""),
            issues: Vec::new(),
            show_ticket_form: false,
            adding_comment: false,
        }
    }
}

// Defines the interface and interactions
impl eframe::App for ClientApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // If nothing focussed draw home screen
            if (self.focus_issue == usize::MAX || self.issues.len() == 0) {
                ui.heading("Rust Issue Tracker");
                ui.add(egui::Label::new(format!(
                    "Currently logged in as: {}",
                    self.users[self.logged_in_as].username
                )));

                let mut show_text = String::from("Raise new");

                if (self.show_ticket_form) {
                    show_text = String::from("Discard")
                }

                let create_ticket_button: egui::Response =
                    ui.button(format!("{} ticket", show_text));
                if create_ticket_button.clicked() {
                    self.show_ticket_form = !self.show_ticket_form;
                }

                // Only show form if meant to be shown.
                if (self.show_ticket_form) {
                    // || {} == () => {}
                    ui.horizontal(|ui| {
                        let ticket_name_label = ui.label("Ticket title: ");
                        ui.text_edit_singleline(&mut self.cur_ticket_name)
                            .labelled_by(ticket_name_label.id);
                    });

                    ui.add(egui::TextEdit::multiline(&mut self.cur_ticket_message));

                    egui::ComboBox::from_label("Priority:")
                        .selected_text(self.cur_ticket_priority.to_string())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.cur_ticket_priority,
                                ItemPriority::Low,
                                "Low",
                            );
                            ui.selectable_value(
                                &mut self.cur_ticket_priority,
                                ItemPriority::Medium,
                                "Medium",
                            );
                            ui.selectable_value(
                                &mut self.cur_ticket_priority,
                                ItemPriority::High,
                                "High",
                            );
                            ui.selectable_value(
                                &mut self.cur_ticket_priority,
                                ItemPriority::Critical,
                                "Critical",
                            );
                        });

                    let ticket_button: egui::Response = ui.button("Submit ticket");

                    // If submit button clicked, create a new ticket,
                    if ticket_button.clicked() {
                        if self.cur_ticket_priority != ItemPriority::NA {
                            self.issues.push(Issue {
                                name: String::from(self.cur_ticket_name.clone()),
                                description: String::from(self.cur_ticket_message.clone()),
                                reporter: String::from(self.users[self.logged_in_as].email.clone()),
                                comment_thread: Vec::new(),
                                priority: self.cur_ticket_priority.clone(),
                                status: ItemStatus::New,
                                // Creating a substring using the &...[..10]
                                time_created: String::from(&(Local::now().to_string())[..10]),
                            });
                        }

                        // Reset all values.
                        self.cur_ticket_message = String::from("");
                        self.cur_ticket_name = String::from("");
                        self.cur_ticket_priority = ItemPriority::NA;
                        self.show_ticket_form = false;
                    }
                }

                let mut i: usize = 0;
                // Draw each existing ticket.
                for issue in self.issues.iter_mut() {
                    // Only draw ticket's you have raised, not other users!
                    if (issue.reporter == self.users[self.logged_in_as].email) {
                        ui.horizontal(|ui| {
                            ui.label(format!(
                                "{} | {} | {} | {} | {} ",
                                issue.name,
                                issue.reporter,
                                issue.priority,
                                issue.status,
                                issue.time_created
                            ));
                            let issue_button = ui.button("View ticket");
                            if issue_button.clicked() {
                                self.focus_issue = i;
                                self.cur_comment = String::from("");
                                self.cur_ticket_message = String::from("");
                                self.cur_ticket_name = String::from("");
                                self.cur_ticket_priority = ItemPriority::NA;
                            }
                        });
                    }
                    i += 1;
                }
            } else {
                let focused_issue: &Issue = &self.issues[self.focus_issue];
                ui.heading(format!("Issue: {}", focused_issue.name));
                ui.label(format!(
                    "Description: {}",
                    focused_issue.description.clone()
                ));
                ui.label(format!("Reported By: {}", focused_issue.reporter.clone()));
                ui.label(format!(
                    "Reported On: {}",
                    focused_issue.time_created.clone()
                ));
                ui.label(format!("Priority: {}", focused_issue.priority));
                ui.label(format!("Status: {}", focused_issue.status));

                ui.label(format!("Comments: "));

                // Only show this if comment button clicked

                let mut show_text = String::from("Add new");

                if (self.adding_comment) {
                    show_text = String::from("Discard")
                }

                let add_comment_button: egui::Response =
                    ui.button(format!("{} comment", show_text));
                if add_comment_button.clicked() {
                    self.adding_comment = !self.adding_comment;
                }
                if (self.adding_comment) {
                    ui.add(egui::TextEdit::multiline(&mut self.cur_ticket_message));

                    let post_comment_button: egui::Response = ui.button(format!("Post comment"));
                    if post_comment_button.clicked() {
                        // Post comment
                        // focused_issue.comment_thread.push(ItemComment {
                        //     commenter: self.users[self.logged_in_as].email,
                        //     message: self.cur_comment.clone(),
                        //     time_created: String::from(Local::now().to_string()),
                        // });
                    }
                }

                // Draw all comments

                let back_button = ui.button("Back");
                if back_button.clicked() {
                    self.focus_issue = usize::MAX;
                }
            }
        });
    }
}

// USE IMPL
// USE A COMPOSITE
