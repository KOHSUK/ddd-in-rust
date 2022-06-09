use validator::Validate;

#[derive(Debug, Clone, Validate, PartialEq, Eq)]
pub struct UserIsPremium {
    value: bool,
}

impl UserIsPremium {
    pub fn new(value: bool) -> Self {
        Self { value }
    }

    pub fn to_inner(&self) -> bool {
        self.value
    }
}
