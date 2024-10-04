// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use chrono::Local;
use eframe::egui;
use std::fmt;

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
    show_ticket_form: bool,
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
            cur_ticket_message: String::from(""),
            cur_ticket_name: String::from(""),
            cur_ticket_priority: ItemPriority::NA,
            issues: Vec::new(),
            show_ticket_form: false,
        }
    }
}

// Defines the interface and interactions
impl eframe::App for ClientApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Rust Issue Tracker");
            ui.add(egui::Label::new(format!(
                "Currently logged in as: {}",
                self.users[self.logged_in_as].username
            )));

            let mut show_text = String::from("Raise new");

            if(self.show_ticket_form){show_text = String::from("Discard")}

            let create_ticket_button: egui::Response = ui.button(format!("{} ticket", show_text));
            if create_ticket_button.clicked() {
                self.show_ticket_form = !self.show_ticket_form;           
            }

            if (self.show_ticket_form) {
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

                if ticket_button.clicked() {
                    if self.cur_ticket_priority != ItemPriority::NA {
                        self.issues.push(Issue {
                            name: String::from(self.cur_ticket_name.clone()),
                            description: String::from(self.cur_ticket_message.clone()),
                            reporter: String::from(self.users[self.logged_in_as].email.clone()),
                            comment_thread: Vec::new(),
                            priority: self.cur_ticket_priority.clone(),
                            status: ItemStatus::New,
                            time_created: Local::now().to_string(),
                        });
                    }
                }
            }
        });
    }
}

// USE IMPL
// USE A COMPOSITE
