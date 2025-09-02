use bon::bon;
use getset::Getters;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Getters)]
pub struct Tutor {
    #[getset(get = "pub")]
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
