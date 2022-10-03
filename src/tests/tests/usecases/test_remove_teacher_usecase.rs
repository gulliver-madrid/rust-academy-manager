#![cfg(test)]

use std::rc::Rc;

use crate::{
    application::RemoveTeacherUseCase, domain::Teacher, tests::mocks::mock_persistence,
};

use super::helpers::{
    assert_execution_without_err, assert_teachers_length_in_memory_is_correct,
    assert_teachers_length_saved_is_correct, setup_repository,
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
