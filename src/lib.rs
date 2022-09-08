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


#[derive(Debug, PartialEq)]
pub enum TopicType {
    Shadow = 0,
    Jobs,
    Defender,
    Tunneling,
}
/// Given the topic string of an incoming message, determine whether it is
/// related to a device topic;
///
/// If it is, return the type of topic, like shadow ,jobs and so on.
///
/// # Example
/// ```
/// use aws_iot_device_sdk::{TopicType, match_topic_type};
///
/// let topic = "$aws/things/chloe/shadow/name/common/get/rejected";
/// let topic_type = match_topic_type(topic).unwrap();
///
/// assert_eq!(topic_type, TopicType::Shadow);
/// ```
pub fn match_topic_type<'a>(topic: &'a str) -> Result<TopicType, Error> {
    Ok(TopicType::Shadow)
}
