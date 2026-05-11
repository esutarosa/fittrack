use super::AppLanguage;

#[derive(Clone, Copy)]
pub struct AuthStrings {
    pub title: &'static str,
    pub description: &'static str,
    pub switch_login: &'static str,
    pub switch_register: &'static str,
    pub username_label: &'static str,
    pub password_label: &'static str,
    pub confirm_password_label: &'static str,
    pub submit_label: &'static str,
    pub signed_in_prefix: &'static str,
}

pub fn auth_strings(language: AppLanguage, register_mode: bool) -> AuthStrings {
    match (language, register_mode) {
        (AppLanguage::English, false) => AuthStrings {
            title: "Sign in",
            description: "Sign in to continue working with workouts and progress.",
            switch_login: "Sign in",
            switch_register: "Register",
            username_label: "Username",
            password_label: "Password",
            confirm_password_label: "Confirm password",
            submit_label: "Sign in",
            signed_in_prefix: "Signed in:",
        },
        (AppLanguage::English, true) => AuthStrings {
            title: "Create account",
            description: "Create an account to save exercises, workouts, and progress history.",
            switch_login: "Sign in",
            switch_register: "Register",
            username_label: "Username",
            password_label: "Password",
            confirm_password_label: "Confirm password",
            submit_label: "Create account",
            signed_in_prefix: "Signed in:",
        },
        (AppLanguage::Ukrainian, false) => AuthStrings {
            title: "Вхід",
            description: "Увійдіть в акаунт, щоб продовжити роботу з тренуваннями та прогресом.",
            switch_login: "Увійти",
            switch_register: "Реєстрація",
            username_label: "Ім'я користувача",
            password_label: "Пароль",
            confirm_password_label: "Підтвердіть пароль",
            submit_label: "Увійти",
            signed_in_prefix: "Вхід виконано:",
        },
        (AppLanguage::Ukrainian, true) => AuthStrings {
            title: "Створення акаунта",
            description: "Створіть акаунт, щоб зберігати вправи, тренування та історію прогресу.",
            switch_login: "Увійти",
            switch_register: "Реєстрація",
            username_label: "Ім'я користувача",
            password_label: "Пароль",
            confirm_password_label: "Підтвердіть пароль",
            submit_label: "Створити акаунт",
            signed_in_prefix: "Вхід виконано:",
        },
    }
}
