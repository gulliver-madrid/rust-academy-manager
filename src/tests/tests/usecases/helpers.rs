#![cfg(test)]

use pretty_assertions::assert_eq;
use std::rc::Rc;

use crate::{
    errors::SimpleResult,
    repository::{create_repository, Repository},
    tests::{helpers::highlight, mocks::mock_persistence::MockPersistence},
};

pub fn setup_repository(mock_persistence: Rc<MockPersistence>) -> Rc<Repository> {
    let repository = create_repository(mock_persistence);
    repository.load_teachers_if_needed();
    let repository = Rc::new(repository);
    repository
}

pub fn assert_execution_without_err(result: SimpleResult) {
    assert!(
        result.is_ok(),
        "{}",
        highlight("Usecase should be executed without error")
    );
}

pub fn assert_teachers_length_saved_is_correct(
    mock_persistence: &Rc<MockPersistence>,
    expected: usize,
    when: &str,
) {
    let actual = mock_persistence.mock_teachers.borrow().len();
    assert_teachers_length_is_correct(actual, expected, "persistence", when);
}

pub fn assert_teachers_length_in_memory_is_correct(
    repository: &Repository,
    expected: usize,
    when: &str,
) {
    let actual = get_teachers_length(repository);
    assert_teachers_length_is_correct(actual, expected, "memory", when);
}

fn assert_teachers_length_is_correct(
    actual: usize,
    expected: usize,
    place: &str,
    when: &str,
) {
    let msg = highlight(
        format!("There should be {expected} teacher(s) in {place} {when}, but there are {actual}").as_str()
    );
    assert_eq!(actual, expected, "{msg}");
}

fn get_teachers_length(repository: &Repository) -> usize {
    repository
        .model
        .borrow()
        .teachers
        .as_ref()
        .unwrap()
        .clone()
        .len()
}
