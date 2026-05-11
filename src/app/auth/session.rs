use crate::shared::contracts::{AuthResponse, UserDto};

#[derive(Clone, Debug)]
pub struct Session {
    pub token: String,
    pub user: UserDto,
}

impl Session {
    pub fn from_auth(auth: AuthResponse) -> Self {
        Self { token: auth.token, user: auth.user }
    }
}
