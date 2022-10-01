use std::cell::RefCell;

use crate::errors::SimpleResult;

use super::{
    subjects_app::SubjectsApp, teachers_app::TeachersApp,
    usecases::AssignTeacherToSubjectUseCase,
};

pub struct Application {
    assign_teacher_to_subject_usecase: RefCell<AssignTeacherToSubjectUseCase>,
    pub teachers_app: RefCell<TeachersApp>,
    pub subjects_app: RefCell<SubjectsApp>,
}

impl Application {
    pub fn new(
        assign_teacher_to_subject_usecase: RefCell<AssignTeacherToSubjectUseCase>,
        teachers_app: RefCell<TeachersApp>,
        subjects_app: RefCell<SubjectsApp>,
    ) -> Self {
        Self {
            assign_teacher_to_subject_usecase,
            teachers_app,
            subjects_app,
        }
    }
    /// Assign a teacher identified by his/her id to the subject with the specified index
    pub fn assign_teacher_to_subject(
        &self,
        subject_index: usize,
        teacher_id: u32,
    ) -> SimpleResult {
        self.assign_teacher_to_subject_usecase
            .borrow()
            .assign_teacher_to_subject(subject_index, teacher_id)
    }
}
