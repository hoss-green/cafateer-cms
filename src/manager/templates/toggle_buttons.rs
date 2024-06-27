use askama::Template;

#[derive(Template)]
#[template(path = "manager/toggle_buttons/enable_button.html")]
pub struct EnableButton {
    pub post_url: String,
    pub button_text: String,
}

#[derive(Template)]
#[template(path = "manager/toggle_buttons/disable_button.html")]
pub struct DisableButton {
    pub post_url: String,
    pub button_text: String,
}
