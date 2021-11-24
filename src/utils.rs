pub type Result<T> = std::result::Result<T, IdentityError>;

#[derive(Debug)]
pub enum IdentityError {
    Password(bcrypt::BcryptError),
}

impl std::fmt::Display for IdentityError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            IdentityError::Password(..) => write!(
                f,
                "failed to generate a bcrypt hash for the provided password"
            ),
        }
    }
}

impl std::error::Error for IdentityError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            IdentityError::Password(ref e) => Some(e),
        }
    }
}

impl From<bcrypt::BcryptError> for IdentityError {
    fn from(err: bcrypt::BcryptError) -> Self {
        IdentityError::Password(err)
    }
}
