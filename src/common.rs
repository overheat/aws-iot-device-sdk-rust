// Limit imposed by the mqtt spec
pub const MQTT_TOPIC_LENGTH_MAX: usize = 65535;

pub const THINGNAME_MAX_LENGTH: usize = 128;
pub const SHADOW_NAME_LENGTH_MAX: usize = 64;
pub const JOBID_MAX_LENGTH: usize = 64;

pub const TUNNEL_TOPIC_MAX_LENGTH: usize = THINGNAME_MAX_LENGTH + 32;
pub const DEFENDER_TOPIC_MAX_LENGTH: usize = THINGNAME_MAX_LENGTH + 32;
pub const JOBS_TOPIC_MAX_LENGTH: usize = THINGNAME_MAX_LENGTH + JOBID_MAX_LENGTH + 32;
pub const SHADOW_TOPIC_MAX_LENGTH: usize = THINGNAME_MAX_LENGTH + SHADOW_NAME_LENGTH_MAX + 32;

pub const AWS_THINGS_PREFIX: &str = "$aws/things/";

pub const SUFFIX_ACCEPTED: &str = "/accepted";
pub const SUFFIX_REJECTED: &str = "/rejected";

pub const ACCEPTED: &str = "accepted";
pub const REJECTED: &str = "rejected";

#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    FAIL,                   /* function encountered error. */
    MqttTopicFailed,        /* Input mqtt topic is invalid. */
    ThingnameParseFailed,   /* Could not parse the thing name. */
    MessageTypeParseFailed, /* Could not parse the type. */
    RootParseFailed,        /* Could not parse the root. */
    ShadownameParseFailed, /* Could not parse the shadow name (in the case of a named shadow topic). */
    JobsIdParseFailed,     /* Could not parse the job id. */
    NoMatch,               /* The provided topic does not match any defender topic. */
}

/// valid parameters?
///
/// # Example
/// ```
/// ```
fn is_valid_param(s: &str, max_len: usize) -> Result<(), Error> {
    if !s.is_empty() && s.len() < max_len {
        return Ok(());
    }
    Err(Error::FAIL)
}

///
/// valid mqtt topic?
/// # Example
/// ```
/// ```
pub fn is_valid_mqtt_topic(mqtt_topic: &str) -> Result<(), Error> {
    is_valid_param(mqtt_topic, MQTT_TOPIC_LENGTH_MAX).map_err(|_| Error::MqttTopicFailed)
}

///
/// valid aws thing prefix?
/// # Example
/// ```
/// ```
pub fn is_valid_prefix<'a>(s: &'a str, pre: &str) -> Result<&'a str, Error> {
    s.strip_prefix(pre).ok_or(Error::NoMatch)
}

///
/// valid name in aws iot?
/// # Example
/// ```
/// ```
fn is_valid_name(name: &str, len: usize) -> Result<(), Error> {
    is_valid_param(name, len)?;
    for a in name.chars() {
        match a {
            '-' | '_' | '0'..='9' | 'A'..='Z' | 'a'..='z' | ':' => continue,
            _ => return Err(Error::FAIL),
        }
    }
    Ok(())
}

///
/// valid aws iot thing name?
/// # Example
/// ```
/// ```
pub fn is_valid_thing_name(thing_name: &str) -> Result<(), Error> {
    is_valid_name(thing_name, THINGNAME_MAX_LENGTH).map_err(|_| Error::ThingnameParseFailed)
}

///
/// valid aws iot shadow name?
/// # Example
/// ```
/// ```
pub fn is_valid_shadow_name(shadow_name: &str) -> Result<(), Error> {
    is_valid_name(shadow_name, SHADOW_NAME_LENGTH_MAX).map_err(|_| Error::ShadownameParseFailed)
}

///
/// valid aws iot bridge?
/// Like, "/shadow/" or "/jobs?", etc.
/// # Example
/// ```
/// ```
pub fn is_valid_bridge<'a>(s: &'a str, bridge: &str) -> Result<&'a str, Error> {
    s.strip_prefix(bridge).ok_or(Error::RootParseFailed)
}

///
/// valid aws iot job id?
/// # Example
/// ```
/// ```
pub fn is_valid_job_id(job_id: &str) -> Result<(), Error> {
    // Thing thing_name cannot be empty or longer than JOBID_MAX_LENGTH
    is_valid_param(job_id, JOBID_MAX_LENGTH)?;
    for a in job_id.chars() {
        match a {
            '-' | '_' | '0'..='9' | 'A'..='Z' | 'a'..='z' => continue,
            _ => return Err(Error::JobsIdParseFailed),
        }
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use crate::common::*;
    #[test]
    fn valid_mqtt_topic() -> Result<(), Error> {
        is_valid_mqtt_topic("hello/world")?;
        Ok(())
    }
    #[test]
    fn valid_prefix() -> Result<(), Error> {
        is_valid_prefix("hello/world", "hello/")?;
        Ok(())
    }
    #[test]
    fn valid_thing_name() -> Result<(), Error> {
        is_valid_thing_name("-_09AZaz:")?;
        Ok(())
    }
    #[test]
    fn valid_shadow_name() -> Result<(), Error> {
        is_valid_shadow_name("common")?;
        Ok(())
    }
    #[test]
    fn valid_job_id() -> Result<(), Error> {
        is_valid_job_id("_-09AZaz")?;
        Ok(())
    }
}
