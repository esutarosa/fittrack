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
            Self::Dashboard => "Overview of the shell and the next actions.",
            Self::Exercises => "Placeholder for the exercise catalog and muscle groups.",
            Self::Workouts => "Placeholder for workout creation and history.",
            Self::Progress => "Placeholder for progress tables and charts.",
        }
    }

    pub fn nav_label(self) -> &'static str {
        self.title()
    }

    pub fn tips(self) -> &'static [&'static str] {
        match self {
            Self::Dashboard => &[
                "Keep the main overview compact.",
                "Surface key actions and recent activity here.",
                "Use this screen as the entry point after login.",
            ],
            Self::Exercises => &[
                "Store exercise name and muscle group.",
                "Keep creation forms short and explicit.",
                "The list should support fast scanning.",
            ],
            Self::Workouts => &[
                "Create workouts for a selected date.",
                "Add notes and open workout details.",
                "Sets will be attached from the detail view.",
            ],
            Self::Progress => &[
                "Show progress by exercise.",
                "Support max weight, reps, and volume.",
                "A table is mandatory; a chart is optional.",
            ],
        }
    }
}
