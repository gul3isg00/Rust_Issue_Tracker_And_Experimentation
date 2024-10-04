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

struct TodoItem {
    name: String,
    description: String,
    reporter: String,
    reviewer: String,
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

fn main() {
    let default_admin = User {
        email: String::from("admin@test.com"),
        password: String::from("encrypted-password"),
        username: String::from("TheBeesKnees"),
        user_type: UserType::Admin,
    };

    let default_client = User {
        email: String::from("client@test.com"),
        password: String::from("encrypted-password"),
        username: String::from("BiteAtTheHand"),
        user_type: UserType::Client,
    };

    println!("{}", default_admin.email);
}

// USE A STRUCT
// USE IMPL
// USE A COMPOSITE
