use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

pub mod create_task;
pub mod create_task_extractor;
pub mod delete_task;
pub mod get_all_tasks;
pub mod get_one_task;
pub mod update_task;

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestTask {
    pub priority: Option<String>,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseTask {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<String>,
    pub completed_at: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseDataTask {
    pub data: ResponseTask,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseDataTasks {
    pub data: Vec<ResponseTask>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestUpdateTask {
    pub title: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub description: Option<Option<String>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub priority: Option<Option<String>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub completed_at: Option<Option<DateTime<FixedOffset>>>,
}
