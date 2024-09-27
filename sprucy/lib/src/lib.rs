struct User;

enum AuthEvent {
    CheckAuth,
    LogIn { username: String, password: String },
    LogOut,
}

trait EventTarget<E>: State {
    fn tigger(&mut self, event: E) -> Result<(), ()>;
}

trait State: Default {
    fn init() -> Self {
        Self::default()
    }
}

enum AuthState {
    LoggedIn { current_user: User },
    LoggedOut,
}

impl State for AuthState {}

impl Default for AuthState {
    fn default() -> Self {
        Self::LoggedOut
    }
}

impl AuthState {
    pub fn check_auth(&self) -> Result<(), ()> {
        todo!()
    }
}

impl EventTarget<AuthEvent> for AuthState {
    fn tigger(&mut self, event: AuthEvent) -> Result<(), ()> {
        match event {
            AuthEvent::CheckAuth => {
                let _ = self.check_auth()?;
            }
            AuthEvent::LogIn { username, password } => todo!(),
            AuthEvent::LogOut => todo!(),
        }
        Ok(())
    }
}
