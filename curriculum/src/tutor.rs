use bon::bon;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tutor {
    name: String,
}

#[bon]
impl Tutor {
    #[builder]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
        }
    }
}
