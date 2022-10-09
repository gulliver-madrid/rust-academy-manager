#![cfg(test)]

pub mod teachers;

use std::rc::Rc;

use crate::{
    errors::SimpleResult,
    repository::{create_repository, Repository},
    tests::{
        helpers::highlight, // fmt
        mocks::mock_persistence::MockPersistence,
    },
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
