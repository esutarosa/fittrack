use std::fmt::Display;

pub fn map_display_error<T, E, U>(
    result: Result<T, E>,
    map: impl FnOnce(String) -> U,
) -> Result<T, U>
where
    E: Display,
{
    result.map_err(|error| map(error.to_string()))
}
