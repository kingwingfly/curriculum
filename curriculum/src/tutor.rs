use bon::bon;
use getset::Getters;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Getters)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
