use uuid::Uuid;

pub struct UserModel {
    pub id: Uuid,
    pub name: String,
}

impl UserModel {
    pub fn new(id: Uuid, name: String) -> Self {
        Self { id, name }
    }
}
