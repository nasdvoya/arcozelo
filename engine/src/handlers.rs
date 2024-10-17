use askama::Template;
use axum::response::Html;

pub fn donors() -> Html<String> {
    let contacts = [
        Donor {
            name: "Alice",
            email: "alice@example.com",
        },
        Donor {
            name: "Bob",
            email: "bob@example.com",
        },
        Donor {
            name: "Charlie",
            email: "charlie@example.com",
        },
    ];
    let template = DonorsTemplate { donors: &contacts };
    let rendered_html = template.render().unwrap();
    println!("{}", rendered_html); // Print the rendered HTML
    Html(template.render().unwrap())
}

#[derive(Template)]
#[template(path = "donors.html")]
struct DonorsTemplate<'a> {
    donors: &'a [Donor<'a>],
}

struct Donor<'a> {
    name: &'a str,
    email: &'a str,
}
