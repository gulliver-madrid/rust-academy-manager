#![cfg(test)]

use std::rc::Rc;

use crate::{application::AddTeacherUseCase, tests::mocks::mock_persistence};

use super::helpers::{
    assert_execution_without_err, assert_teachers_length_in_memory_is_correct,
    assert_teachers_length_saved_is_correct, setup_repository,
};

#[test]
fn add_teacher_usecase() {
    let mock_persistence = Rc::new(mock_persistence::create_void_mock_persistence());
    let repository = setup_repository(Rc::clone(&mock_persistence));
    let before = "at the beginning";
    assert_teachers_length_saved_is_correct(&mock_persistence, 0, before);
    assert_teachers_length_in_memory_is_correct(&repository, 0, before);

    let usecase = AddTeacherUseCase {
        repository: Rc::clone(&repository),
    };
    let result = usecase.execute("John".to_string());
    assert_execution_without_err(result);

    let after = "after adding 1 teacher";
    assert_teachers_length_in_memory_is_correct(&repository, 1, after);
    assert_teachers_length_saved_is_correct(&mock_persistence, 1, after);
}
