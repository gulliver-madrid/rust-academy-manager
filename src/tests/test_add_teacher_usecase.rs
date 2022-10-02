#![cfg(test)]

use std::rc::Rc;

use crate::{
    application::AddTeacherUseCase, errors::SimpleError, repository::create_repository,
    repository::Repository, tests::fixtures::highlight,
};

use super::fixtures::mock_persistence::{self, MockPersistence};

use pretty_assertions::assert_eq;

#[test]
fn add_teacher_usecase() {
    let mock_persistence = Rc::new(mock_persistence::create_void_mock_persistence());
    let mock_persistence_ref = Rc::clone(&mock_persistence);
    assert_eq!(mock_persistence_ref.mock_teachers.borrow().len(), 0);
    let repository = setup_repository(mock_persistence);
    let usecase = AddTeacherUseCase {
        repository: Rc::clone(&repository),
    };
    assert_teachers_length_is_correct(&repository, 0);
    usecase
        .execute("John".to_string())
        .unwrap_or_else(err_executing_usecase);
    // Testing behaviour on error:
    // Err(SimpleError::new("hola")).unwrap_or_else(err_executing_usecase);
    assert_teachers_length_is_correct(&repository, 1);
    assert_eq!(mock_persistence_ref.mock_teachers.borrow().len(), 1)
}

fn err_executing_usecase(_: SimpleError) {
    panic!("{}", highlight("Usecase should be executed without error"));
}

fn setup_repository(mock_persistence: Rc<MockPersistence>) -> Rc<Repository> {
    let repository = create_repository(mock_persistence);
    repository.load_teachers_if_needed();
    let repository = Rc::new(repository);
    repository
}

fn assert_teachers_length_is_correct(repository: &Repository, expected_length: usize) {
    let teachers_len = get_teachers_length(repository);
    assert_eq!(
        teachers_len,
        expected_length,
        "{}",
        highlight(
            format!(
                "After adding a teacher, there should be {} teacher(s) in model, but there are {}",
                expected_length,
                teachers_len
            ).as_str()
        )

    );
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
