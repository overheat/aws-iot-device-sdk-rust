use crate::common::*;

const API_CHANGED: &str = "notify";

/// Check if the given topic is one of the Device Defender topics.
///
/// # Example
/// ```
/// use aws_iot_device_sdk::{tunneling};
///
/// let tunnels = tunneling::match_topic("$aws/things/chloe/tunnels/notify");
/// assert_eq!(tunnels, Ok(()));
///
/// ```
pub fn match_topic(topic: &str) -> Result<(), Error> {
    // $aws/things/thing-name/tunnels/notify
    is_valid_mqtt_topic(topic)?;

    let s = is_valid_prefix(topic, AWS_THINGS_PREFIX)?;

    let mid = s.find('/').ok_or(Error::FAIL);
    let (thing_name, mut s) = s.split_at(mid?);
    is_valid_thing_name(thing_name)?;

    s = is_valid_bridge(s, TUNNELS_API_BRIDGE)?;

    if s == API_CHANGED {
        return Ok(());
    }
    Err(Error::NoMatch)
}

#[cfg(test)]
mod tests {
    use crate::tunneling;
    #[test]
    fn tunnels_match_topic() {
        let tunnels = tunneling::match_topic("$aws/things/chloe/tunnels/notify");
        assert_eq!(tunnels, Ok(()));
    }
}
