mod dashboard;
mod strings;

use dashboard::{render_dashboard, render_header};
use eframe::egui::{Align, Button, CentralPanel, Context, Layout, RichText, SidePanel, Ui, vec2};

use crate::app::auth::{AuthClient, Session};
use crate::app::exercises::ExercisesState;
use crate::app::progress::ProgressState;
use crate::app::workouts::WorkoutsState;
use crate::shared::i18n::AppLanguage;
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
}

pub struct ShellState<'a> {
    pub active_screen: &'a mut Screen,
    pub exercises_state: &'a mut ExercisesState,
    pub workouts_state: &'a mut WorkoutsState,
    pub progress_state: &'a mut ProgressState,
    pub language: &'a mut AppLanguage,
}

pub fn render_shell(
    ctx: &Context,
    session: &Session,
    client: &AuthClient,
    state: ShellState<'_>,
) -> bool {
    let colors = theme::colors();
    let layout = theme::layout();
    let mut logout_clicked = false;
    let ShellState { active_screen, exercises_state, workouts_state, progress_state, language } =
        state;

    SidePanel::left("fittrack_sidebar")
        .resizable(false)
        .default_width(layout.sidebar_width)
        .frame(theme::sidebar_frame())
        .show(ctx, |ui| {
            logout_clicked = render_sidebar(ui, active_screen, session, language, colors, layout);
        });

    CentralPanel::default().frame(theme::page_frame()).show(ctx, |ui| {
        let content_width = (ui.available_width() - layout.page_padding * 2.0).max(0.0);
        ui.horizontal(|ui| {
            ui.add_space(layout.page_padding);
            ui.allocate_ui_with_layout(
                vec2(content_width, ui.available_height()),
                Layout::top_down(Align::Min),
                |ui| {
                    ui.add_space(layout.page_padding);
                    render_header(ui, *active_screen, *language, colors);
                    ui.add_space(layout.section_gap);

                    match active_screen {
                        Screen::Dashboard => render_dashboard(
                            ui,
                            *language,
                            exercises_state.items().len(),
                            workouts_state.workout_count(),
                            progress_state.record_count(),
                            colors,
                            layout,
                        ),
                        Screen::Exercises => exercises_state.render(ui, client, session, *language),
                        Screen::Workouts => workouts_state.render(
                            ui,
                            client,
                            session,
                            exercises_state.items(),
                            *language,
                        ),
                        Screen::Progress => progress_state.render(
                            ui,
                            client,
                            session,
                            exercises_state.items(),
                            *language,
                        ),
                    }
                },
            );
            ui.add_space(layout.page_padding);
        });
    });

    logout_clicked
}

fn render_sidebar(
    ui: &mut Ui,
    active_screen: &mut Screen,
    session: &Session,
    language: &mut AppLanguage,
    colors: theme::Colors,
    layout: theme::Layout,
) -> bool {
    let mut logout_clicked = false;
    ui.vertical(|ui| {
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.with_layout(
                eframe::egui::Layout::right_to_left(eframe::egui::Align::Center),
                |ui| theme::language_toggle(ui, language),
            );
        });
        ui.add_space(12.0);
        ui.label(RichText::new("FitTrack").size(24.0).strong());
        ui.label(
            RichText::new(strings::app_tagline(*language)).size(13.0).color(colors.text_muted),
        );
        ui.add_space(layout.section_gap);
        ui.separator();
        ui.add_space(layout.section_gap);

        for screen in Screen::NAV_ITEMS {
            let selected = *active_screen == screen;
            let button = Button::selectable(selected, strings::nav_label(screen, *language))
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
            ui.label(
                RichText::new(strings::signed_in_label(*language))
                    .size(12.0)
                    .color(colors.text_muted),
            );
            ui.label(RichText::new(&session.user.username).size(16.0).strong());
            ui.add_space(8.0);
            if ui
                .add(
                    Button::new(strings::logout_label(*language))
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
