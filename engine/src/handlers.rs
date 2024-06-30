use askama::Template;
use axum::response::Html;

pub fn contacts() -> Html<String> {
    let contacts = [
        Contact {
            name: "Alice",
            email: "alice@example.com",
        },
        Contact {
            name: "Bob",
            email: "bob@example.com",
        },
        Contact {
            name: "Charlie",
            email: "charlie@example.com",
        },
    ];
    let template = ContactsTemplate {
        contacts: &contacts,
    };
    Html(template.render().unwrap())
}

#[derive(Template)]
#[template(path = "contacts.html")]
struct ContactsTemplate<'a> {
    contacts: &'a [Contact<'a>],
}

struct Contact<'a> {
    name: &'a str,
    email: &'a str,
}
