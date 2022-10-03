#![cfg(test)]

use std::rc::Rc;

use crate::{
    application::RemoveTeacherUseCase, domain::Teacher, tests::mocks::mock_persistence,
};

use super::helpers::{
    assert_execution_without_err, // fmt
    assert_teachers_length_in_memory_is_correct,
    assert_teachers_length_saved_is_correct,
    setup_repository,
};

#[test]
fn remove_teacher_usecase() {
    const TEACHER_NAME: &str = "Manuel";
    const BEFORE: &str = "at the beginning";
    const AFTER: &str = "after remove the teacher";

    let mock_persistence = Rc::new(mock_persistence::create_void_mock_persistence());
    mock_persistence.mock_teachers.borrow_mut().push(Teacher {
        name: TEACHER_NAME.to_string(),
        id: 1033,
        phone_number: "555-555-555".to_string(),
    });
    let repository = setup_repository(Rc::clone(&mock_persistence));

    assert_teachers_length_saved_is_correct(&mock_persistence, 1, BEFORE);
    assert_teachers_length_in_memory_is_correct(&repository, 1, BEFORE);

    let usecase = RemoveTeacherUseCase {
        repository: Rc::clone(&repository),
    };
    let result = usecase.remove_teacher(TEACHER_NAME.to_string());
    assert_execution_without_err(result);

    assert_teachers_length_in_memory_is_correct(&repository, 0, AFTER);
    assert_teachers_length_saved_is_correct(&mock_persistence, 0, AFTER);
}
