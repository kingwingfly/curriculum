use bon::bon;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tutor {
    name: String,
}

#[bon]
impl Tutor {
    #[builder]
    pub fn new(name: impl AsRef<str>) -> Self {
        Self {
            name: name.as_ref().to_owned(),
        }
    }
}
