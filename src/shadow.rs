use crate::common::*;
use arrayvec::{ArrayString, ArrayVec};

use self::Topic::*;

const OP_GET: &str = "get";
const OP_DELETE: &str = "delete";
const OP_UPDATE: &str = "update";
const SUFFIX_DOCUMENTS: &str = "/documents";
const SUFFIX_DELTA: &str = "/delta";
/// A shadow topic string takes one of the two forms,
/// in the case of an unnamed ("Classic") shadow.
/// Or, in the case of a named shadow
/// The shadow_name part is None when unnamed shadow.
#[derive(Debug)]
pub struct ThingShadow<'a> {
    pub thing_name: &'a str,
    pub shadow_name: Option<&'a str>,
    pub shadow_op: Topic,
}

/// Each of these values describes the type of a shadow message.
/// https://docs.aws.amazon.com/iot/latest/developerguide/device-shadow-mqtt.html
#[derive(Debug, PartialEq)]
pub enum Topic {
    Get = 0,
    GetAccepted,
    GetRejected,
    Delete,
    DeleteAccepted,
    DeleteRejected,
    Update,
    UpdateAccepted,
    UpdateRejected,
    UpdateDocuments,
    UpdateDelta,
}

/// Assemble shadow topic string when Thing Name or Shadow Name is only known at run time.
///
/// # Example
/// ```
/// use aws_iot_device_sdk::shadow::Topic::*;
/// use aws_iot_device_sdk::{shadow};
/// use arrayvec::{ArrayString, ArrayVec};
///
/// let topic = shadow::assemble_topic(shadow::Topic::Get, "chloe", None).unwrap();
/// assert_eq!("$aws/things/chloe/shadow/get", topic.as_str())
/// ```

pub fn assemble_topic(
    topic_type: Topic,
    thing_name: &str,
    named: Option<&str>,
) -> Result<ArrayString<SHADOW_TOPIC_MAX_LENGTH>, Error> {
    is_valid_thing_name(thing_name)?;
    let mut s = ArrayString::<SHADOW_TOPIC_MAX_LENGTH>::new();
    s.push_str(AWS_THINGS_PREFIX);
    s.push_str(thing_name);
    match named {
        // Classic shadow topic
        None => {
            s.push_str(SHADOW_API_BRIDGE);
            s.push_str(op(&topic_type));
            s.push_str(suffix(&topic_type));
            Ok(s)
        }
        // Named shadow topic
        Some(shadow_name) => {
            is_valid_shadow_name(shadow_name)?;
            s.push_str(NAMED_SHADOW_API_BRIDGE);
            s.push_str(shadow_name);
            s.push_str("/");
            s.push_str(op(&topic_type));
            s.push_str(suffix(&topic_type));
            Ok(s)
        }
    }
}

fn op(topic_type: &Topic) -> &str {
    match topic_type {
        Get | GetAccepted | GetRejected => OP_GET,
        Delete | DeleteAccepted | DeleteRejected => OP_DELETE,
        Update | UpdateAccepted | UpdateRejected | UpdateDocuments | UpdateDelta => OP_UPDATE,
    }
}
fn suffix(topic_type: &Topic) -> &str {
    match topic_type {
        GetAccepted | DeleteAccepted | UpdateAccepted => SUFFIX_ACCEPTED,
        GetRejected | DeleteRejected | UpdateRejected => SUFFIX_REJECTED,
        UpdateDocuments => SUFFIX_DOCUMENTS,
        UpdateDelta => SUFFIX_DELTA,
        _ => "",
    }
}

/// Given the topic string of an incoming message, determine whether it is
/// related to a device shadow;
///
/// If it is, return information about the type of device shadow message,
/// and the Thing Name and Shadow Name inside of the topic string.
///
/// # Example
/// ```
/// use aws_iot_device_sdk::shadow::Topic::*;
/// use aws_iot_device_sdk::{shadow};
///
/// let topic = "$aws/things/chloe/shadow/name/common/update";
/// let shadow = shadow::match_topic(topic).unwrap();
///
/// assert_eq!(shadow.thing_name, "chloe");
/// assert_eq!(shadow.shadow_name.unwrap(), "common");
/// assert_eq!(shadow.shadow_op, shadow::Topic::Update);
///
/// let topic = "$aws/things/chloe/shadow/name/common/update/delta";
/// let shadow = shadow::match_topic(topic).unwrap();
///
/// assert_eq!(shadow.thing_name, "chloe");
/// assert_eq!(shadow.shadow_name.unwrap(), "common");
/// assert_eq!(shadow.shadow_op, shadow::Topic::UpdateDelta);
/// ```
pub fn match_topic<'a>(topic: &'a str) -> Result<ThingShadow, Error> {
    is_valid_mqtt_topic(topic)?;

    let s = is_valid_prefix(topic, AWS_THINGS_PREFIX)?;

    let mid = s.find('/').ok_or(Error::NoMatch);
    let (thing_name, s) = s.split_at(mid?);
    is_valid_thing_name(thing_name)?;

    let s = is_valid_bridge(s, SHADOW_API_BRIDGE)?;

    let v: ArrayVec<&str, 16> = s.split('/').collect();
    match v[..] {
        // Named shadow topic
        [_named, shadow_name, op, suffix] => {
            is_valid_shadow_name(shadow_name)?;
            Ok(ThingShadow {
                thing_name,
                shadow_name: Some(shadow_name),
                shadow_op: find_message_type(op, Some(suffix))?,
            })
        }
        // Named shadow topic without suffix
        [_named, shadow_name, op] => {
            is_valid_shadow_name(shadow_name)?;
            Ok(ThingShadow {
                thing_name,
                shadow_name: Some(shadow_name),
                shadow_op: find_message_type(op, None)?,
            })
        }
        // Classic shadow topic
        [op, suffix] => Ok(ThingShadow {
            thing_name,
            shadow_name: None,
            shadow_op: find_message_type(op, Some(suffix))?,
        }),
        // Classic shadow topic without suffix
        [op] => Ok(ThingShadow {
            thing_name,
            shadow_name: None,
            shadow_op: find_message_type(op, None)?,
        }),
        // Not shadow topic
        _ => Err(Error::NoMatch),
    }
}

fn find_message_type(op: &str, suffix: Option<&str>) -> Result<Topic, Error> {
    match (op, suffix) {
        ("get", None) => Ok(Get),
        ("get", Some("accepted")) => Ok(GetAccepted),
        ("get", Some("rejected")) => Ok(GetRejected),
        ("delete", None) => Ok(Delete),
        ("delete", Some("accepted")) => Ok(DeleteAccepted),
        ("delete", Some("rejected")) => Ok(DeleteRejected),
        ("update", None) => Ok(Update),
        ("update", Some("accepted")) => Ok(UpdateAccepted),
        ("update", Some("rejected")) => Ok(UpdateRejected),
        ("update", Some("documents")) => Ok(UpdateDocuments),
        ("update", Some("delta")) => Ok(UpdateDelta),
        _ => Err(Error::MessageTypeParseFailed),
    }
}

#[cfg(test)]
mod tests {
    use crate::shadow;
    #[test]
    fn assemble_named_topic_string() {
        let topic = shadow::assemble_topic(shadow::Topic::Get, "chloe", Some("common")).unwrap();
        assert_eq!("$aws/things/chloe/shadow/name/common/get", topic.as_str());
    }
    #[test]
    fn assemble_classic_topic_string() {
        let topic = shadow::assemble_topic(shadow::Topic::Get, "chloe", None).unwrap();
        assert_eq!("$aws/things/chloe/shadow/get", topic.as_str());
    }
    #[test]
    fn assemble_classic_topic_string_suffix() {
        let topic = shadow::assemble_topic(shadow::Topic::GetAccepted, "chloe", None).unwrap();
        assert_eq!("$aws/things/chloe/shadow/get/accepted", topic.as_str());
    }
    #[test]
    fn match_classic_shadow_topic_string() {
        let topic = "$aws/things/chloe/shadow/get/accepted";
        let shadow = shadow::match_topic(topic).unwrap();
        assert_eq!(shadow.thing_name, "chloe");
        assert_eq!(shadow.shadow_name, None);
        assert_eq!(shadow.shadow_op, shadow::Topic::GetAccepted);
    }
    #[test]
    fn match_named_shadow_topic_string() {
        let topic = "$aws/things/chloe/shadow/name/common/get/rejected";
        let shadow = shadow::match_topic(topic).unwrap();
        assert_eq!(shadow.thing_name, "chloe");
        assert_eq!(shadow.shadow_name.unwrap(), "common");
        assert_eq!(shadow.shadow_op, shadow::Topic::GetRejected);
    }
}
