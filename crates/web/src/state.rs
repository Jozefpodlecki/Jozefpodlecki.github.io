use crate::content::ContentError;

#[derive(Clone, PartialEq)]
pub enum Load<T> {
    Loading,
    Ready(T),
    Error(ContentError),
}
