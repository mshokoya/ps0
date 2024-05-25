use crate::actions::controllers::TaskType;
use async_std::task::{sleep, spawn};
use polodb_core::bson::Uuid;
use std::collections::VecDeque;
use std::sync::Mutex;
use std::time::Duration;
use tauri::{AppHandle, Manager};
// use String::String;

use super::types::{ActionData, Channels, Process, Task, TaskActionCTX, TaskEvent};

#[derive(Debug)]
pub struct TaskQueue {
    pub app_handle: AppHandle,
    pub wait_queue: Mutex<VecDeque<Task>>,
    pub process_queue: Mutex<VecDeque<Process>>,
    pub timeout_queue: Mutex<VecDeque<Task>>,
    pub max_processes: u8,
}

unsafe impl Send for TaskQueue {}
unsafe impl Sync for TaskQueue {}

// https://stackoverflow.com/questions/54971024/accessing-a-method-of-self-inside-a-thread-in-rust
// https://stackoverflow.com/questions/73551266/tauri-is-there-some-way-to-access-apphandler-or-window-in-regular-struct-or-sta
impl TaskQueue {
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            app_handle,
            wait_queue: Mutex::new(VecDeque::new()),
            process_queue: Mutex::new(VecDeque::new()),
            timeout_queue: Mutex::new(VecDeque::new()),
            max_processes: 2,
        }
    }

    pub fn w_enqueue(&self, task: Task) {
        let task_cln = task.clone();
        self.wait_queue.lock().unwrap().push_back(task_cln);
        self.app_handle
            .emit_all(
                Channels::WaitQueue.into(),
                TaskEvent {
                    task_id: &task.task_id,
                    message: "added new task to wait queue".to_string(),
                    ok: None,
                    task_type: TaskType::Enqueue,
                    metadata: &task.metadata,
                    action_data: ActionData {
                        task_group: &task.task_group,
                        task_type: &task.task_type,
                        metadata: None,
                    },
                },
            )
            .unwrap();
        self.exec();
    }

    fn w_dequeue(&self) -> Option<Task> {
        let task = match self.wait_queue.lock().unwrap().pop_front() {
            None => return None,
            Some(tsk) => tsk,
        };
        let task_cln = task.clone();
        self.app_handle.emit_all(
                Channels::WaitQueue.into(),
                TaskEvent {
                    task_id: &task_cln.task_id,
                    message: "removed task from wait queue".to_string(),
                    ok: None,
                    task_type: TaskType::Dequeue,
                    metadata: &task_cln.metadata,
                    action_data: ActionData {
                        task_group: &task_cln.task_group,
                        task_type: &task_cln.task_type,
                        metadata: None,
                    },
                },
            )
            .unwrap();
        Some(task)
    }

    fn p_enqueue(&self, process: Process) {
        let task = process.task.clone();
        self.process_queue.lock().unwrap().push_back(process);
        self.app_handle
            .emit_all(
                Channels::ProcessQueue.into(),
                TaskEvent {
                    task_id: &task.task_id,
                    message: "new task added to processing queue".to_string(),
                    ok: None,
                    task_type: TaskType::Enqueue,
                    metadata: &task.metadata,
                    action_data: ActionData {
                        task_group: &task.task_group,
                        task_type: &task.task_type,
                        metadata: None,
                    },
                },
            )
            .unwrap();
        self.exec();
    }

    pub fn p_dequeue(&self, task_id: &Uuid) {
        let ps = remove_process(&self.process_queue, task_id).unwrap();
        let mut task = ps.task.clone();

        if let Some(mut t_out) = task.timeout {
            t_out.rounds -= 1;
            task.timeout = Some(t_out);
            if t_out.rounds > 0 {
                self.t_enqueue(task);
            } else {
                // self.t_dequeue(task_id);
            }
        }

        self.app_handle
            .emit_all(
                Channels::ProcessQueue.into(),
                TaskEvent {
                    task_id,
                    message: "removed completed task from queue".to_string(),
                    task_type: TaskType::Dequeue,
                    ok: None,
                    metadata: &ps.task.metadata,
                    action_data: ActionData {
                        task_group: &ps.task.task_group,
                        task_type: &ps.task.task_type,
                        metadata: None,
                    },
                },
            )
            .unwrap();
    }

    fn t_enqueue(&self, task: Task) {
        let task_cln = task.clone();
        if let Some(timeout) = task_cln.timeout {
            let app_handle = self.app_handle.clone();
            spawn(async move {
                sleep(Duration::from_millis(timeout.time)).await;
                app_handle.state::<TaskQueue>().w_enqueue(task_cln.clone());
                app_handle
                    .emit_all(
                        Channels::TimeoutQueue.into(),
                        TaskEvent {
                            task_id: &task_cln.task_id,
                            message: "removed task from timeout queue".to_string(),
                            ok: None,
                            task_type: TaskType::Enqueue,
                            metadata: &task_cln.metadata,
                            action_data: ActionData {
                                task_group: &task_cln.task_group,
                                task_type: &task_cln.task_type,
                                metadata: None,
                            },
                        },
                    )
                    .unwrap();
            });
        }
        self.app_handle
            .emit_all(
                Channels::TimeoutQueue.into(),
                TaskEvent {
                    task_id: &task.task_id,
                    message: "added task to timeout queue".to_string(),
                    ok: None,
                    task_type: TaskType::Enqueue,
                    metadata: &task.metadata,
                    action_data: ActionData {
                        task_group: &task.task_group,
                        task_type: &task.task_type,
                        metadata: None,
                    },
                },
            )
            .unwrap();

        self.exec();
    }

    // fn t_dequeue(&self, task_id: String) {
    //     println!("{}", "t d 1");
    //     let mut pq = self.timeout_queue.lock().unwrap();
    //     let idx = pq
    //         .iter()
    //         .position(|t| t.task.task_id == task_id)
    //         .unwrap_or(225);
    //     println!("{}", "t d 2");
    //     let task = pq.swap_remove_back(idx).unwrap().task;
    //     println!("{}", "t d 3");
    //     self.app_handle
    //         .emit_all(
    //             Channels::TimeoutQueue.into(),
    //             TaskEvent {
    //                 task_id: task.task_id,
    //                 message: "removed task from timeout queue",
    //                 task_type: TaskType::Dequeue,
    //                 metadata: task.metadata,
    //                 action_data: ActionData {
    //                     task_group: task.task_group,
    //                     task_type: task.task_type,
    //                     metadata: None,
    //                 },
    //             },
    //         )
    //         .unwrap();
    // }

    fn exec(&self) {
        println!("WE IN DA TASK");
        let pq = self.process_queue.lock().unwrap();
        let pq_len = pq.len();
        if pq_len >= self.max_processes.into() {
            return;
        };
        let task = match self.w_dequeue() {
            None => return,
            Some(tsk) => tsk,
        };
        drop(pq);

        let handle = self.app_handle.clone();
        let tsk_cln = task.clone();

        let ps = spawn(async move {
            let mut ok: bool = false;
            let mut message = "removed completed task from queue".to_string();
            let metadata = match tsk_cln
                .task_type
                .exec(
                    TaskActionCTX {
                        handle: handle.clone(),
                        task_id: tsk_cln.task_id,
                        page: None,
                    },
                    tsk_cln.args,
                )
                .await
            {
                Ok(val) => {
                    ok = true;
                    val
                }
                Err(err) => {
                    message = err.to_string();
                    ok = false;
                    None
                }
            };

            handle.state::<TaskQueue>().p_dequeue(&tsk_cln.task_id);

            handle
                .emit_all(
                    Channels::ProcessQueue.into(),
                    TaskEvent {
                        task_id: &tsk_cln.task_id,
                        message,
                        ok: Some(ok),
                        task_type: TaskType::Enqueue,
                        metadata: &tsk_cln.metadata,
                        action_data: ActionData {
                            task_group: &tsk_cln.task_group,
                            task_type: &tsk_cln.task_type,
                            metadata,
                        },
                    },
                )
                .unwrap();
        });

        self.p_enqueue(Process { task, ps });
    }
}

// fn remove_task(queue: &Mutex<VecDeque<Task>>, task_id: &Uuid) -> Option<Task> {
//     let mut tq = queue.lock().unwrap();
//     let idx = tq.iter().position(|t| t.task_id == *task_id).unwrap_or(225);
//     tq.swap_remove_back(idx)
// }

fn remove_process(queue: &Mutex<VecDeque<Process>>, task_id: &Uuid) -> Option<Process> {
    let mut pq = queue.lock().unwrap();
    let idx = pq
        .iter()
        .position(|t| t.task.task_id == *task_id)
        .unwrap_or(225);
    pq.swap_remove_back(idx)
}
