use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum TimerData {
    Next(i64),
    Continue(i64),
}
