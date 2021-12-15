use crate::domain::email::Email;
use crate::domain::name::Name;

#[derive(PartialEq, Eq, Debug)]
pub struct Subscriber {
    pub email: Email,
    pub name: Name,
}

impl Subscriber {
    pub fn new(email: Email, name: Name) -> Self {
        Self { email, name }
    }
}
