use askama::Template;

#[derive(Template, Debug, Clone)]
#[template(path = "session/login.html")]
pub struct LoginPage<'a> {
    pub title: &'a str,
}

#[derive(Template, Debug, Clone)]
#[template(path = "session/sign_up.html")]
pub struct SignUpPage<'a> {
    pub title: &'a str,
}
#[derive(Template, Debug, Clone)]
#[template(path = "session/sign_up_success.html")]
pub struct SignUpSuccessPage<'a> {
    pub title: &'a str,
}
