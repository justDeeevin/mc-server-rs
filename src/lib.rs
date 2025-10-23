use serde::{Deserialize, Serialize};

pub mod data;
pub mod text_component;
pub mod types;

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum OneOf<L, R> {
    Left(L),
    Right(R),
}

impl<L: Clone, R: Clone> Clone for OneOf<L, R> {
    fn clone(&self) -> Self {
        match self {
            Self::Left(value) => Self::Left(value.clone()),
            Self::Right(value) => Self::Right(value.clone()),
        }
    }
}

impl<T> From<OneOf<T, Vec<T>>> for Vec<T> {
    fn from(value: OneOf<T, Vec<T>>) -> Self {
        match value {
            OneOf::Left(value) => vec![value],
            OneOf::Right(value) => value,
        }
    }
}

impl<T> From<OneOf<Vec<T>, T>> for Vec<T> {
    fn from(value: OneOf<Vec<T>, T>) -> Self {
        match value {
            OneOf::Left(value) => value,
            OneOf::Right(value) => vec![value],
        }
    }
}

impl<T> From<Vec<T>> for OneOf<Vec<T>, T> {
    fn from(value: Vec<T>) -> Self {
        Self::Left(value)
    }
}

impl<T> From<Vec<T>> for OneOf<T, Vec<T>> {
    fn from(value: Vec<T>) -> Self {
        Self::Right(value)
    }
}
