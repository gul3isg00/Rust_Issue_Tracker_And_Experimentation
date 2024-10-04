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

fn main() {
    println!("Hello, world!");
}

// USE A STRUCT
// USE IMPL
// USE A COMPOSITE
