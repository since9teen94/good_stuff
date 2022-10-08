use chrono::Datelike;
use serde::Serialize;

pub const LOGIN_TITLE: &str = "Log In";
pub const LOGIN_URL: &str = "/login";
pub const LOGOUT_URL: &str = "/logout";
const REGISTER_TITLE: &str = "Register";
pub const REGISTER_URL: &str = "/register";
pub const HOME_URL: &str = "/home";
pub const HOME_TEMPLATE: &str = "home.html";
pub const LOG_REG_TEMPLATE: &str = "logReg.html";
pub const HOME_TITLE: &str = "Home";
pub const HOUSE_TITLE: &str = "House";
pub const HOUSE_URL: &str = "/house";
pub const TITLE: &str = "title";
const POST: &str = "POST";
pub const REGISTER: &str = "Register";
pub const INDEX_URL: &str = "/";
pub const DETAILS_URL: &str = "/details";

#[derive(Serialize)]
pub struct LogRegForm {
    title: String,
    action: String,
    method: String,
    fields: Vec<LogRegFormField>,
    year: i32,
    home: String,
}

#[derive(Serialize)]
struct LogRegFormField {
    id: String,
    text: String,
    field_type: String,
    placeholder: String,
}

impl LogRegFormField {
    pub fn new(id: &str, text: &str, field_type: &str, placeholder: &str) -> LogRegFormField {
        LogRegFormField {
            id: String::from(id),
            text: String::from(text),
            field_type: String::from(field_type),
            placeholder: String::from(placeholder),
        }
    }
}

impl LogRegForm {
    pub fn new(title: &str) -> LogRegForm {
        let action = match title {
            LOGIN_TITLE => String::from(LOGIN_URL),
            REGISTER_TITLE => String::from(REGISTER_URL),
            _ => String::from(""),
        };
        let method = String::from(POST);
        let home = String::from(HOME_URL);
        let mut form_fields = vec![
            LogRegFormField::new("email", "Email", "email", "Please enter a valid email."),
            LogRegFormField::new(
                "password",
                "Password",
                "password",
                "Please enter a valid password.",
            ),
        ];
        if title == "Register" {
            form_fields.insert(
                0,
                LogRegFormField::new(
                    "first_name",
                    "First Name",
                    "first_name",
                    "Please enter your first name.",
                ),
            );
            form_fields.insert(
                1,
                LogRegFormField::new(
                    "last_name",
                    "Last Name",
                    "last_name",
                    "Please enter your last name.",
                ),
            );
            form_fields.push(LogRegFormField::new(
                "confirm_password",
                "Confirm Password",
                "password",
                "Please confirm your password.",
            ));
        };
        let year = chrono::Utc::now().year();

        LogRegForm {
            title: title.to_string(),
            action,
            method,
            year,
            fields: form_fields,
            home,
        }
    }
}
