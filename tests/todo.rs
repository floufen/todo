use todo::task;

#[test]
fn check_task_creation() {
    let task = task::Task::new("important task");
    assert_eq!(task.status, task::Status::Unchecked);
    assert_eq!(task.entry, "important task");
}

#[test]
fn check_task_mutability() {
    let mut task = task::Task::new("important task");
    assert_eq!(task.status, task::Status::Unchecked);
    task.check();
    assert_eq!(task.status, task::Status::Checked);
    task.uncheck();
    assert_eq!(task.status, task::Status::Unchecked);
}
