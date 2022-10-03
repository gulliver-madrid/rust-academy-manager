#![cfg(test)]

use pretty_assertions::assert_eq;
use std::rc::Rc;

use crate::{
    application::RemoveTeacherUseCase,
    domain::Teacher,
    errors::SimpleResult,
    repository::{create_repository, Repository},
    tests::{
        helpers::highlight,
        mocks::{
            mock_persistence, // fmt
            mock_persistence::MockPersistence,
        },
    },
};

#[test]
fn remove_teacher_usecase() {
    const TEACHER_NAME: &str = "Manuel";
    let mock_persistence = Rc::new(mock_persistence::create_void_mock_persistence());
    mock_persistence.mock_teachers.borrow_mut().push(Teacher {
        name: TEACHER_NAME.to_string(),
        id: 1033,
        phone_number: "555-555-555".to_string(),
    });
    let repository = setup_repository(Rc::clone(&mock_persistence));
    let before = "at the beginning";
    assert_teachers_length_saved_is_correct(&mock_persistence, 1, before);
    assert_teachers_length_in_memory_is_correct(&repository, 1, before);

    let usecase = RemoveTeacherUseCase {
        repository: Rc::clone(&repository),
    };
    let result = usecase.remove_teacher(TEACHER_NAME.to_string());
    assert_execution_without_err(result);

    let after = "after remove the teacher";
    assert_teachers_length_in_memory_is_correct(&repository, 0, after);
    assert_teachers_length_saved_is_correct(&mock_persistence, 0, after);
}

fn setup_repository(mock_persistence: Rc<MockPersistence>) -> Rc<Repository> {
    let repository = create_repository(mock_persistence);
    repository.load_teachers_if_needed();
    let repository = Rc::new(repository);
    repository
}

fn assert_execution_without_err(result: SimpleResult) {
    assert!(
        result.is_ok(),
        "{}",
        highlight("Usecase should be executed without error")
    );
}

fn assert_teachers_length_saved_is_correct(
    mock_persistence: &Rc<MockPersistence>,
    expected: usize,
    when: &str,
) {
    let actual = mock_persistence.mock_teachers.borrow().len();
    assert_teachers_length_is_correct(actual, expected, "persistence", when);
}

fn assert_teachers_length_in_memory_is_correct(
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
