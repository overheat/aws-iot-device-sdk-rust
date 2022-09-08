//! # Overview
//!
//! say something.
//!
//!
//!
//!
// #![no_std]
pub mod backoff_algo;
pub mod common;
pub mod defender;
pub mod jobs;
pub mod shadow;
pub mod tunneling;

pub use common::Error;

pub use common::is_valid_bridge;
pub use common::is_valid_job_id;
pub use common::is_valid_mqtt_topic;
pub use common::is_valid_prefix;
pub use common::is_valid_shadow_name;
pub use common::is_valid_thing_name;

pub enum TopicType {
    Shadow = 0,
    Jobs,
    Defender,
    Tunneling,
}

pub fn match_topic<'a>(topic: &'a str) -> Result<TopicType, Error> {
    Ok(TopicType::Shadow)
}
