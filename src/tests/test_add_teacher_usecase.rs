#![cfg(test)]

use std::rc::Rc;

use crate::{
    application::AddTeacherUseCase, repository::create_repository,
    repository::Repository, tests::fixtures::highlight,
};

use super::fixtures::mock_persistence;

use pretty_assertions::assert_eq;

#[test]
fn test_add_teacher_usecase() {
    let repository = setup_repository();
    let usecase = AddTeacherUseCase {
        repository: Rc::clone(&repository),
    };
    assert_teachers_length_is_correct(&repository, 0);
    usecase.execute("John".to_string()).expect(&highlight(
        "Usecase should be executed without error".to_string(),
    ));
    assert_teachers_length_is_correct(&repository, 1);
}

fn setup_repository() -> Rc<Repository> {
    let mock_persistence = mock_persistence::create_void_mock_persistence();
    let repository = create_repository(Box::new(mock_persistence));
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
            )
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
