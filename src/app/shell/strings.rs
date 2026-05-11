use super::Screen;
use crate::shared::i18n::AppLanguage;

pub fn nav_label(screen: Screen, language: AppLanguage) -> &'static str {
    match (screen, language) {
        (Screen::Dashboard, AppLanguage::English) => "Dashboard",
        (Screen::Exercises, AppLanguage::English) => "Exercises",
        (Screen::Workouts, AppLanguage::English) => "Workouts",
        (Screen::Progress, AppLanguage::English) => "Progress",
        (Screen::Dashboard, AppLanguage::Ukrainian) => "Огляд",
        (Screen::Exercises, AppLanguage::Ukrainian) => "Вправи",
        (Screen::Workouts, AppLanguage::Ukrainian) => "Тренування",
        (Screen::Progress, AppLanguage::Ukrainian) => "Прогрес",
    }
}

pub fn screen_title(screen: Screen, language: AppLanguage) -> &'static str {
    nav_label(screen, language)
}

pub fn screen_description(screen: Screen, language: AppLanguage) -> &'static str {
    match (screen, language) {
        (Screen::Dashboard, AppLanguage::English) => "Overview of the current training state.",
        (Screen::Exercises, AppLanguage::English) => "Manage the exercise library.",
        (Screen::Workouts, AppLanguage::English) => "Create and review workout sessions.",
        (Screen::Progress, AppLanguage::English) => "Track exercise progress over time.",
        (Screen::Dashboard, AppLanguage::Ukrainian) => "Огляд поточного стану тренувань.",
        (Screen::Exercises, AppLanguage::Ukrainian) => "Керуйте бібліотекою вправ.",
        (Screen::Workouts, AppLanguage::Ukrainian) => "Створюйте та переглядайте тренування.",
        (Screen::Progress, AppLanguage::Ukrainian) => "Відстежуйте прогрес вправ у часі.",
    }
}

pub fn app_tagline(language: AppLanguage) -> &'static str {
    match language {
        AppLanguage::English => "Workout tracker",
        AppLanguage::Ukrainian => "Трекер тренувань",
    }
}

pub fn signed_in_label(language: AppLanguage) -> &'static str {
    match language {
        AppLanguage::English => "Signed in",
        AppLanguage::Ukrainian => "Виконано вхід",
    }
}

pub fn logout_label(language: AppLanguage) -> &'static str {
    match language {
        AppLanguage::English => "Log out",
        AppLanguage::Ukrainian => "Вийти",
    }
}

pub fn metric_copy(language: AppLanguage) -> [(&'static str, &'static str); 3] {
    match language {
        AppLanguage::English => [
            ("Workouts", "Current training sessions"),
            ("Exercises", "Exercise library entries"),
            ("Progress", "Tracked exercise records"),
        ],
        AppLanguage::Ukrainian => [
            ("Тренування", "Поточні тренувальні сесії"),
            ("Вправи", "Записи бібліотеки вправ"),
            ("Прогрес", "Відстежувані записи прогресу"),
        ],
    }
}
