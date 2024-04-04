use axum::Form;

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscribe(Form(form): Form<FormData>) -> String {
    format!("Welcome {}! Will sent news to {}", form.name, form.email)
}
