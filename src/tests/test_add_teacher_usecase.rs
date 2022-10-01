#![cfg(test)]

use std::rc::Rc;

use crate::{application::AddTeacherUseCase, repository::Repository};

use {super::fixtures::mock_persistence, crate::repository::create_repository};

#[test]
fn test_add_teacher_usecase() {
    let mock_persistence = mock_persistence::create_void_mock_persistence();

    let repository = create_repository(Box::new(mock_persistence));
    repository.load_teachers_if_needed();
    let repository = Rc::new(repository);
    let usecase = AddTeacherUseCase {
        repository: Rc::clone(&repository),
    };
    assert_teachers_len(&repository, 0);
    let result = usecase.execute("John".to_string());
    assert_eq!(result, Ok(()), "Usecase was executed without error");
    assert_teachers_len(&repository, 2);
}

fn assert_teachers_len(repository: &Repository, expected_length: usize) {
    let teachers_len = repository
        .model
        .borrow()
        .teachers
        .as_ref()
        .unwrap()
        .clone()
        .len();
    assert_eq!(
        teachers_len,
        expected_length,
        "\n\n>>> After adding a teacher, there should be {} teacher(s) in model, but there are {}\n\n",
        expected_length,
        teachers_len
    );
}
