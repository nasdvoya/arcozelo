use axum::{response::Html, Form};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CardData {
    name: String,
    address: String,
    parish: String,
    county: String,
    payment_location: String,
    donor: String,
    notes: String,
    email: String,
    hours: String,
    operator: String,
    postal_code: String,
    home_phone: String,
    work_phone: String,
    mobile_phone: String,
    tax_number: String,
    membership_number: String,
}

pub async fn submit_card(Form(card): Form<CardData>) -> Html<String> {
    println!("Received card submission");
    // let fields = [
    //     ("Name", &card.name),
    //     ("Address", &card.address),
    //     ("Parish", &card.parish),
    //     ("County", &card.county),
    //     ("Payment Location", &card.payment_location),
    //     ("Donor", &card.donor),
    //     ("Notes", &card.notes),
    //     ("Email", &card.email),
    //     ("Hours", &card.hours),
    //     ("Operator", &card.operator),
    //     ("Postal Code", &card.postal_code),
    //     ("Home Phone", &card.home_phone),
    //     ("Work Phone", &card.work_phone),
    //     ("Mobile Phone", &card.mobile_phone),
    //     ("Tax Number", &card.tax_number),
    //     ("Membership Number", &card.membership_number),
    // ];
    //
    // for (field_name, value) in fields.iter() {
    //     println!("{}: {}", field_name, value);
    // }

    Html("<div class=\"success-message\">Form submitted successfully!</div>".into())
}
