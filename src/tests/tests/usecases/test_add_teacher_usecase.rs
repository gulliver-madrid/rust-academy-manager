#![cfg(test)]

use std::rc::Rc;

use crate::{application::AddTeacherUseCase, tests::mocks::mock_persistence};

use super::helpers::{
    assert_execution_without_err, // fmt
    assert_teachers_length_in_memory_is_correct,
    assert_teachers_length_saved_is_correct,
    setup_repository,
};

#[test]
fn add_teacher_usecase() {
    const TEACHER_NAME: &str = "John";
    const BEFORE: &str = "at the beginning";
    const AFTER: &str = "after adding 1 teacher";

    let mock_persistence = Rc::new(mock_persistence::create_void_mock_persistence());
    let repository = setup_repository(Rc::clone(&mock_persistence));

    assert_teachers_length_saved_is_correct(&mock_persistence, 0, BEFORE);
    assert_teachers_length_in_memory_is_correct(&repository, 0, BEFORE);

    let usecase = AddTeacherUseCase {
        repository: Rc::clone(&repository),
    };
    let result = usecase.execute(TEACHER_NAME.to_string());
    assert_execution_without_err(result);

    assert_teachers_length_in_memory_is_correct(&repository, 1, AFTER);
    assert_teachers_length_saved_is_correct(&mock_persistence, 1, AFTER);
}
