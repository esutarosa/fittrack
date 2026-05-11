use eframe::egui::{Button, CentralPanel, Context, RichText, SidePanel, Ui, vec2};

use crate::app::auth::Session;
use crate::shared::ui as theme;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Screen {
    Dashboard,
    Exercises,
    Workouts,
    Progress,
}

impl Screen {
    pub const NAV_ITEMS: [Self; 4] =
        [Self::Dashboard, Self::Exercises, Self::Workouts, Self::Progress];

    pub fn nav_label(self) -> &'static str {
        match self {
            Self::Dashboard => "Dashboard",
            Self::Exercises => "Exercises",
            Self::Workouts => "Workouts",
            Self::Progress => "Progress",
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            Self::Dashboard => "Dashboard",
            Self::Exercises => "Exercises",
            Self::Workouts => "Workouts",
            Self::Progress => "Progress",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::Dashboard => "Overview of the current training state.",
            Self::Exercises => "Manage the exercise library.",
            Self::Workouts => "Create and review workout sessions.",
            Self::Progress => "Track exercise progress over time.",
        }
    }

    pub fn tips(self) -> &'static [&'static str] {
        match self {
            Self::Dashboard => &[
                "Hook summary cards to real server data.",
                "Surface recent workouts and quick actions here.",
            ],
            Self::Exercises => {
                &["Add exercise creation and filtering.", "Group exercises by muscle group."]
            }
            Self::Workouts => &["List workout sessions by date.", "Open a workout to add sets."],
            Self::Progress => &[
                "Show max weight, reps, and total volume.",
                "Add a simple chart after the table works.",
            ],
        }
    }
}

pub fn render_shell(ctx: &Context, active_screen: &mut Screen, session: &Session) -> bool {
    let colors = theme::colors();
    let layout = theme::layout();
    let mut logout_clicked = false;

    SidePanel::left("fittrack_sidebar")
        .resizable(false)
        .default_width(layout.sidebar_width)
        .frame(theme::sidebar_frame())
        .show(ctx, |ui| {
            logout_clicked = render_sidebar(ui, active_screen, session, colors, layout);
        });

    CentralPanel::default()
        .frame(theme::page_frame())
        .show(ctx, |ui| render_main(ui, *active_screen, colors, layout));

    logout_clicked
}

fn render_sidebar(
    ui: &mut Ui,
    active_screen: &mut Screen,
    session: &Session,
    colors: theme::Colors,
    layout: theme::Layout,
) -> bool {
    let mut logout_clicked = false;

    ui.vertical(|ui| {
        ui.add_space(4.0);
        ui.label(RichText::new("FitTrack").size(24.0).strong());
        ui.label(RichText::new("Local workout tracker").size(13.0).color(colors.text_muted));
        ui.add_space(layout.section_gap);
        ui.separator();
        ui.add_space(layout.section_gap);

        for screen in Screen::NAV_ITEMS {
            let selected = *active_screen == screen;
            let button = Button::selectable(selected, screen.nav_label())
                .frame_when_inactive(true)
                .corner_radius(layout.button_radius)
                .min_size(vec2(ui.available_width(), layout.nav_item_height));

            if ui.add(button).clicked() {
                *active_screen = screen;
            }
        }

        ui.add_space(layout.section_gap);
        ui.separator();
        ui.add_space(layout.section_gap);

        theme::card_frame().show(ui, |ui| {
            ui.label(RichText::new("Signed in").size(12.0).color(colors.text_muted));
            ui.label(RichText::new(&session.user.username).size(16.0).strong());
            ui.add_space(8.0);

            if ui
                .add(
                    Button::new("Log out")
                        .frame_when_inactive(true)
                        .corner_radius(layout.button_radius)
                        .min_size(vec2(ui.available_width(), layout.nav_item_height)),
                )
                .clicked()
            {
                logout_clicked = true;
            }
        });
    });

    logout_clicked
}

fn render_main(ui: &mut Ui, screen: Screen, colors: theme::Colors, layout: theme::Layout) {
    ui.vertical(|ui| {
        ui.add_space(layout.page_padding);
        render_header(ui, screen, colors);
        ui.add_space(layout.section_gap);

        match screen {
            Screen::Dashboard => render_dashboard(ui, colors, layout),
            Screen::Exercises => render_placeholder(ui, screen, colors, layout),
            Screen::Workouts => render_placeholder(ui, screen, colors, layout),
            Screen::Progress => render_placeholder(ui, screen, colors, layout),
        }
    });
}

fn render_header(ui: &mut Ui, screen: Screen, colors: theme::Colors) {
    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            ui.label(RichText::new(screen.title()).size(30.0).strong());
            ui.label(RichText::new(screen.description()).size(13.0).color(colors.text_muted));
        });
    });
}

fn render_dashboard(ui: &mut Ui, colors: theme::Colors, layout: theme::Layout) {
    ui.columns(3, |columns| {
        metric_card(&mut columns[0], "Workouts", "0", "Current training sessions", colors, layout);
        metric_card(&mut columns[1], "Exercises", "0", "Exercise library entries", colors, layout);
        metric_card(&mut columns[2], "Progress", "0", "Tracked exercise records", colors, layout);
    });

    ui.add_space(layout.section_gap);

    theme::card_frame().show(ui, |ui| {
        ui.label(RichText::new("Shell ready").size(18.0).strong());
        ui.add_space(8.0);
        for tip in Screen::Dashboard.tips() {
            bullet_line(ui, tip, colors);
        }
    });
}

fn render_placeholder(ui: &mut Ui, screen: Screen, colors: theme::Colors, layout: theme::Layout) {
    theme::card_frame().show(ui, |ui| {
        ui.label(RichText::new(screen.title()).size(18.0).strong());
        ui.add_space(8.0);
        ui.label(
            RichText::new("This screen is reserved for the next implementation step.")
                .size(13.0)
                .color(colors.text_muted),
        );
        ui.add_space(layout.section_gap);

        for tip in screen.tips() {
            bullet_line(ui, tip, colors);
        }
    });
}

fn metric_card(
    ui: &mut Ui,
    title: &str,
    value: &str,
    detail: &str,
    colors: theme::Colors,
    layout: theme::Layout,
) {
    theme::card_frame().show(ui, |ui| {
        ui.label(RichText::new(title).size(13.0).color(colors.text_muted));
        ui.add_space(8.0);
        ui.label(RichText::new(value).size(28.0).strong());
        ui.add_space(4.0);
        ui.label(RichText::new(detail).size(12.0).color(colors.text_muted));
        ui.add_space(layout.section_gap);
    });
}

fn bullet_line(ui: &mut Ui, text: &str, colors: theme::Colors) {
    ui.horizontal(|ui| {
        ui.label(RichText::new("•").size(14.0).color(colors.accent));
        ui.label(RichText::new(text).size(13.0).color(colors.text_primary));
    });
}
