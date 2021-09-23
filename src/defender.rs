use arrayvec::{ArrayString, ArrayVec};

use self::Topic::*;
use crate::common::*;

const API_BRIDGE: &str = "/defender/metrics/";
const API_JSON_FORMAT: &str = "json";
const API_CBOR_FORMAT: &str = "cbor";

/// The struct outputs which API the topic is for. It also outputs
/// the thing name in the given topic.
pub struct ThingDefender<'a> {
    pub thing_name: &'a str,
    pub api: Topic,
}

/// 
/// A Defender topic has the following format:"<Prefix><Thing Name><Bridge><Report Format><Suffix>"
/// 
/// Where:
///     - Prefix = $aws/things/
///     - Thing Name = Name of the thing.
///     - Bridge = /defender/metrics/
///     - Report Format = json or cbor
///     - Suffix = /accepted or /rejected or empty
/// 
#[derive(Debug, PartialEq)]
pub enum Topic {
    JsonReportPublish,
    /* Topic for publishing a JSON report. */
    JsonReportAccepted,
    /* Topic for getting a JSON report accepted response. */
    JsonReportRejected,
    /* Topic for getting a JSON report rejected response. */
    CborReportPublish,
    /* Topic for publishing a CBOR report. */
    CborReportAccepted,
    /* Topic for getting a CBOR report accepted response. */
    CborReportRejected, /* Topic for getting a CBOR report rejected response. */
}

/// Populate the topic string for a Device Defender operation.
///
/// # Example
/// ```
/// use aws_iot_embedded_sdk_rust::defender::Topic::*;
/// use aws_iot_embedded_sdk_rust::{defender};
/// 
/// let topic = defender::get_topic("chloe", defender::Topic::JsonReportPublish).unwrap();
/// assert_eq!(&topic[..], "$aws/things/chloe/defender/metrics/json")
/// ```
pub fn get_topic(
    thing_name: &str,
    api: Topic,
) -> Result<ArrayString<DEFENDER_TOPIC_MAX_LENGTH>, Error> {
    is_valid_thing_name(thing_name)?;
    let mut s = ArrayString::<DEFENDER_TOPIC_MAX_LENGTH>::new();
    s.push_str(AWS_THINGS_PREFIX);
    s.push_str(thing_name);
    s.push_str(API_BRIDGE);
    s.push_str(op(&api));
    s.push_str(suffix(&api));

    Ok(s)
}

fn op(api: &Topic) -> &str {
    match api {
        JsonReportPublish | JsonReportAccepted | JsonReportRejected => API_JSON_FORMAT,
        CborReportPublish | CborReportAccepted | CborReportRejected => API_CBOR_FORMAT,
    }
}

fn suffix(topic_type: &Topic) -> &str {
    match topic_type {
        JsonReportAccepted | CborReportAccepted => SUFFIX_ACCEPTED,
        JsonReportRejected | CborReportRejected => SUFFIX_REJECTED,
        _ => "",
    }
}

/// Check if the given topic is one of the Device Defender topics.
///
/// # Example
/// ```
/// use aws_iot_embedded_sdk_rust::defender::Topic::*;
/// use aws_iot_embedded_sdk_rust::{defender};
/// 
/// let defender =
///     defender::match_topic("$aws/things/chloe/defender/metrics/json/accepted").unwrap();
///
/// assert_eq!(defender.thing_name, "chloe");
/// assert_eq!(defender.api, defender::Topic::JsonReportAccepted)
/// ```
pub fn match_topic(topic: &str) -> Result<ThingDefender, Error> {
    is_valid_mqtt_topic(topic)?;

    let s = is_valid_prefix(topic, AWS_THINGS_PREFIX)?;

    let mid = s.find('/').ok_or(Error::FAIL);
    let (thing_name, mut s) = s.split_at(mid?);
    is_valid_thing_name(thing_name)?;

    s = is_valid_bridge(s, API_BRIDGE)?;

    let v: ArrayVec<&str, 16> = s.split('/').collect();
    let api: Topic;
    match v[..] {
        // ~$aws/things/<thingName>/defender/metrics/~<format>/suffix
        [op, suffix] => {
            match (op, suffix) {
                (API_JSON_FORMAT, ACCEPTED) => api = JsonReportAccepted,
                (API_JSON_FORMAT, REJECTED) => api = JsonReportRejected,
                (API_CBOR_FORMAT, ACCEPTED) => api = CborReportAccepted,
                (API_CBOR_FORMAT, REJECTED) => api = CborReportRejected,
                _ => return Err(Error::NoMatch),
            }
            Ok(ThingDefender { thing_name, api })
        }
        // Not defender topic
        _ => Err(Error::NoMatch),
    }
}
#[cfg(test)]
mod tests {
    use crate::defender;
    #[test]
    fn get_topic_json() {
        let topic = defender::get_topic("chloe", defender::Topic::JsonReportPublish).unwrap();
        assert_eq!(&topic[..], "$aws/things/chloe/defender/metrics/json");
    }

    #[test]
    fn get_topic_cbor_rejected() {
        let topic = defender::get_topic("chloe", defender::Topic::CborReportRejected).unwrap();
        assert_eq!(
            &topic[..],
            "$aws/things/chloe/defender/metrics/cbor/rejected"
        );
    }
    #[test]
    fn test_match_topic_some_name() {
        let defender =
            defender::match_topic("$aws/things/chloe/defender/metrics/json/accepted").unwrap();

        assert_eq!(defender.thing_name, "chloe");
        assert_eq!(defender.api, defender::Topic::JsonReportAccepted);
    }
}
