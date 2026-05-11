use crate::app::auth::{AuthClient, Session};
use crate::app::workouts::{WorkoutsCopy, WorkoutsState};
use crate::shared::contracts::{CreateWorkoutSetRequest, UpdateWorkoutSetRequest};

pub(super) fn save_set(
    state: &mut WorkoutsState,
    client: &AuthClient,
    session: &Session,
    copy: WorkoutsCopy,
) {
    let weight = state.weight.trim().parse::<f64>();
    let repetitions = state.repetitions.trim().parse::<i32>();
    let editing = state.editing_set_id.is_some();
    let order = selected_set_order(state);
    if let (Some(workout_id), Some(exercise_id), Some(set_order), Ok(weight), Ok(repetitions)) =
        (state.selected_workout_id, state.selected_exercise_id, order, weight, repetitions)
    {
        let result = if let Some(workout_set_id) = state.editing_set_id {
            client.update_workout_set(
                &session.token,
                workout_id,
                workout_set_id,
                &UpdateWorkoutSetRequest { exercise_id, set_order, weight, repetitions },
            )
        } else {
            client.create_workout_set(
                &session.token,
                workout_id,
                &CreateWorkoutSetRequest {
                    exercise_id,
                    set_order: state.sets.len() as i32 + 1,
                    weight,
                    repetitions,
                },
            )
        };

        match result {
            Ok(_) => {
                state.editing_set_id = None;
                state.weight.clear();
                state.repetitions.clear();
                state.reload_sets(client, session);
                state.message = editing.then(|| copy.set_updated.to_owned());
            }
            Err(error) => state.message = Some(error.to_string()),
        }
    } else {
        state.message = Some(copy.invalid_set.to_owned());
    }
}

pub(super) fn delete_set(
    state: &mut WorkoutsState,
    client: &AuthClient,
    session: &Session,
    workout_set_id: i64,
    deleted_message: &str,
) {
    let Some(workout_id) = state.selected_workout_id else {
        return;
    };
    match client.delete_workout_set(&session.token, workout_id, workout_set_id) {
        Ok(_) => {
            if state.editing_set_id == Some(workout_set_id) {
                state.editing_set_id = None;
                state.weight.clear();
                state.repetitions.clear();
            }
            state.reload_sets(client, session);
            state.message = Some(deleted_message.to_owned());
        }
        Err(error) => state.message = Some(error.to_string()),
    }
}

fn selected_set_order(state: &WorkoutsState) -> Option<i32> {
    state
        .editing_set_id
        .and_then(|set_id| {
            state.sets.iter().find(|item| item.id == set_id).map(|item| item.set_order)
        })
        .or(Some(state.sets.len() as i32 + 1))
}
