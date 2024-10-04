#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use eframe::egui;
use egui::Response;
use chrono::Local;

enum ItemPriority {
    Low,
    Medium,
    High,
    Critical,
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
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
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
    loggedInAs: usize,
    cur_ticket_message: String,
    cur_ticket_name: String
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
            loggedInAs: 0,
            cur_ticket_message: String::from(""),
            cur_ticket_name: String::from(""),
            issues: Vec::new(),
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
                self.users[self.loggedInAs].username
            )));
            let ticket_name_label = ui.label("Ticket title: ");
            let ticket_name_input: egui::Response = ui.add(egui::TextEdit::singleline(&mut self.cur_ticket_name)).labelled_by(ticket_name_label.id);
            let ticket_input: egui::Response = ui.add(egui::TextEdit::multiline(&mut self.cur_ticket_message));
            let ticket_button: egui::Response = ui.button("Raise new ticket");
            if ticket_button.clicked() {
                self.issues.push(Issue{
                    name: String::from(self.cur_ticket_name.clone()),
                    description: String::from(self.cur_ticket_message.clone()),
                    reporter: String::from(self.users[self.loggedInAs].email.clone()),
                    comment_thread: Vec::new(),
                    priority: ItemPriority::Critical,
                    status: ItemStatus::New,
                    time_created: Local::now().to_string(),
                });
            }

            // ui.image(egui::include_image!(
            // ));
        });
    }
}

// USE IMPL
// USE A COMPOSITE
