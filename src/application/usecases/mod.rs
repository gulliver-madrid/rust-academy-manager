mod add_subject;
mod add_teacher;
mod assign_teacher_to_subject;
mod get_subject_index_by_name;
mod remove_subject;
mod remove_teacher;

pub use {
    add_subject::AddSubjectUseCase,
    assign_teacher_to_subject::AssignTeacherToSubjectUseCase,
    get_subject_index_by_name::GetSubjectIndexByNameUseCase,
    remove_subject::RemoveSubjectUseCase,
    add_teacher::AddTeacherUseCase, remove_teacher::RemoveTeacherUseCase
};
