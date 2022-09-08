use crate::common::*;
use arrayvec::{ArrayString, ArrayVec};

use self::Topic::*;

const API_JOBSCHANGED: &str = "notify";
const API_NEXTJOBCHANGED: &str = "notify-next";
const API_GETPENDING: &str = "get";
const API_STARTNEXT: &str = "start-next";
const API_DESCRIBE: &str = "get";
const API_UPDATE: &str = "update";
const API_JOBID_NEXT: &str = "$next";

/// The struct outputs which API the topic is for. It also outputs
/// the thing name in the given topic.
pub struct ThingJobs<'a> {
    pub thing_name: &'a str,
    pub api: Topic,
    pub id: Option<ArrayString<JOBID_MAX_LENGTH>>,
}

///
/// Topic values for subscription requests.
///
#[derive(Debug, PartialEq, PartialOrd)]
pub enum Topic {
    JobsChanged,
    NextJobChanged,
    GetPendingSuccess,
    GetPendingFailed,
    StartNextSuccess,
    StartNextFailed,
    /* Topics below use a job ID. */
    DescribeSuccess,
    DescribeFailed,
    UpdateSuccess,
    UpdateFailed,
}

/// Populate a topic string for a subscription request.
///
/// # Example
/// ```
/// use aws_iot_device_sdk::jobs::Topic::*;
/// use aws_iot_device_sdk::{jobs};
///
/// let jobs = jobs::match_topic("$aws/things/chloe/jobs/notify-next").unwrap();
/// assert_eq!(jobs.api, jobs::Topic::NextJobChanged);
/// assert_eq!(jobs.id, None);
/// ```
pub fn get_topic(
    thing_name: &str,
    api: Topic,
) -> Result<ArrayString<JOBS_TOPIC_MAX_LENGTH>, Error> {
    is_valid_thing_name(thing_name)?;
    let mut s = ArrayString::<JOBS_TOPIC_MAX_LENGTH>::new();
    s.push_str(AWS_THINGS_PREFIX);
    s.push_str(thing_name);
    s.push_str(JOBS_API_BRIDGE);
    s.push_str(id(&api));
    s.push_str(op(&api));
    s.push_str(suffix(&api));

    Ok(s)
}

fn id(api: &Topic) -> &str {
    match api {
        DescribeSuccess | DescribeFailed | UpdateSuccess | UpdateFailed => "+/",
        _ => "",
    }
}

fn op(api: &Topic) -> &str {
    match api {
        JobsChanged => API_JOBSCHANGED,
        NextJobChanged => API_NEXTJOBCHANGED,
        GetPendingSuccess => API_GETPENDING,
        GetPendingFailed => API_GETPENDING,
        StartNextSuccess => API_STARTNEXT,
        StartNextFailed => API_STARTNEXT,
        /* Topics below use a => job ID. */
        DescribeSuccess => API_DESCRIBE,
        DescribeFailed => API_DESCRIBE,
        UpdateSuccess => API_UPDATE,
        UpdateFailed => API_UPDATE,
    }
}

fn suffix(topic_type: &Topic) -> &str {
    match topic_type {
        GetPendingSuccess | StartNextSuccess | DescribeSuccess | UpdateSuccess => SUFFIX_ACCEPTED,
        GetPendingFailed | StartNextFailed | DescribeFailed | UpdateFailed => SUFFIX_REJECTED,
        _ => "",
    }
}
/// Output a topic value if a Jobs API topic string is present.
/// Optionally, output a jobID and thing name within the topic.
///
/// # Example
/// ```
/// use aws_iot_device_sdk::jobs::Topic::*;
/// use aws_iot_device_sdk::{jobs};
///
/// let jobs = jobs::match_topic("$aws/things/chloe/jobs/$next/get/accepted").unwrap();
/// assert_eq!(jobs.api, jobs::Topic::DescribeSuccess);
/// let id = jobs.id.unwrap();
/// assert_eq!(&id[..], "$next")
///
/// ```
pub fn match_topic(topic: &str) -> Result<ThingJobs, Error> {
    is_valid_mqtt_topic(topic)?;

    let s = is_valid_prefix(topic, AWS_THINGS_PREFIX)?;

    let mid = s.find('/').ok_or(Error::FAIL);
    let (thing_name, mut s) = s.split_at(mid?);
    is_valid_thing_name(thing_name)?;

    s = is_valid_bridge(s, JOBS_API_BRIDGE)?;

    let v: ArrayVec<&str, 16> = s.split('/').collect();
    let api: Topic;
    let jobs_id;
    match v[..] {
        // ~$aws/things/MyThing/jobs/~<operation>
        // $aws/things/MyThing/jobs/notify (or $aws/things/MyThing/jobs/notify-next)
        [op] => {
            if op == API_JOBSCHANGED {
                api = JobsChanged;
            } else {
                api = NextJobChanged;
            }
            Ok(ThingJobs {
                thing_name,
                api,
                id: None,
            })
        }
        // $aws/things/MyThing/jobs/<operation>/<suffix>
        [op, suffix] => {
            match (op, suffix) {
                (API_GETPENDING, ACCEPTED) => api = GetPendingSuccess,
                (API_GETPENDING, REJECTED) => api = GetPendingFailed,
                (API_STARTNEXT, ACCEPTED) => api = StartNextSuccess,
                (API_STARTNEXT, REJECTED) => api = StartNextFailed,
                _ => return Err(Error::NoMatch),
            }
            Ok(ThingJobs {
                thing_name,
                api,
                id: None,
            })
        }
        // $aws/things/MyThing/jobs/<jobs-id>/<operation>/<suffix>
        [id, op, suffix] => {
            match (op, suffix) {
                (API_DESCRIBE, ACCEPTED) => api = DescribeSuccess,
                (API_DESCRIBE, REJECTED) => api = DescribeFailed,
                (API_UPDATE, ACCEPTED) => api = UpdateSuccess,
                (API_UPDATE, REJECTED) => api = UpdateFailed,
                _ => return Err(Error::NoMatch),
            }
            jobs_id = Some(ArrayString::<JOBID_MAX_LENGTH>::from(id).unwrap());
            Ok(ThingJobs {
                thing_name,
                api,
                id: jobs_id,
            })
        }
        // Not jobs topic
        _ => Err(Error::NoMatch),
    }
}
/// Populate a topic string for a GetPendingJobExecutions request.
///
pub fn get_pending(thing_name: &str) -> Result<ArrayString<THINGNAME_MAX_LENGTH>, Error> {
    is_valid_thing_name(thing_name)?;
    let mut s = ArrayString::<THINGNAME_MAX_LENGTH>::new();
    s.push_str(AWS_THINGS_PREFIX);
    s.push_str(thing_name);
    s.push_str(JOBS_API_BRIDGE);
    s.push_str(API_GETPENDING);

    Ok(s)
}
/// Populate a topic string for a StartNextPendingJobExecution request.
///
pub fn start_next(thing_name: &str) -> Result<ArrayString<THINGNAME_MAX_LENGTH>, Error> {
    is_valid_thing_name(thing_name)?;
    let mut s = ArrayString::<THINGNAME_MAX_LENGTH>::new();
    s.push_str(AWS_THINGS_PREFIX);
    s.push_str(thing_name);
    s.push_str(JOBS_API_BRIDGE);
    s.push_str(API_STARTNEXT);

    Ok(s)
}
/// Populate a topic string for a DescribeJobExecution request.
///
/// # Example
/// ```
/// use aws_iot_device_sdk::jobs::Topic::*;
/// use aws_iot_device_sdk::{jobs};
///
/// let topic = jobs::describe("chloe", "$next").unwrap();
/// assert_eq!(&topic[..], "$aws/things/chloe/jobs/$next/get")
///
/// ```
pub fn describe(thing_name: &str, id: &str) -> Result<ArrayString<THINGNAME_MAX_LENGTH>, Error> {
    is_valid_thing_name(thing_name)?;
    if id != API_JOBID_NEXT {
        is_valid_job_id(id)?
    };
    let mut s = ArrayString::<THINGNAME_MAX_LENGTH>::new();
    s.push_str(AWS_THINGS_PREFIX);
    s.push_str(thing_name);
    s.push_str(JOBS_API_BRIDGE);
    s.push_str(id);
    s.push_str("/");
    s.push_str(API_DESCRIBE);

    Ok(s)
}
/// Populate a topic string for an UpdateJobExecution request.
///
pub fn update(thing_name: &str, id: &str) -> Result<ArrayString<THINGNAME_MAX_LENGTH>, Error> {
    is_valid_thing_name(thing_name)?;
    is_valid_job_id(id)?;
    let mut s = ArrayString::<THINGNAME_MAX_LENGTH>::new();
    s.push_str(AWS_THINGS_PREFIX);
    s.push_str(thing_name);
    s.push_str(JOBS_API_BRIDGE);
    s.push_str(id);
    s.push_str("/");
    s.push_str(API_UPDATE);

    Ok(s)
}

#[cfg(test)]
mod tests {
    use crate::jobs;
    #[test]
    fn get_topic_notify_next() {
        let topic = jobs::get_topic("chloe", jobs::Topic::NextJobChanged).unwrap();
        assert_eq!(&topic[..], "$aws/things/chloe/jobs/notify-next");
    }

    #[test]
    fn get_topic_get_rejected() {
        let topic = jobs::get_topic("chloe", jobs::Topic::GetPendingFailed).unwrap();
        assert_eq!(&topic[..], "$aws/things/chloe/jobs/get/rejected");
    }

    #[test]
    fn get_topic_id_update_rejected() {
        let topic = jobs::get_topic("chloe", jobs::Topic::UpdateFailed).unwrap();
        assert_eq!(&topic[..], "$aws/things/chloe/jobs/+/update/rejected");
    }

    #[test]
    fn match_topic() {
        let jobs = jobs::match_topic("$aws/things/chloe/jobs/notify-next").unwrap();
        assert_eq!(jobs.api, jobs::Topic::NextJobChanged);
        assert_eq!(jobs.id, None);
    }
    #[test]
    fn match_topic_with_op() {
        let jobs = jobs::match_topic("$aws/things/chloe/jobs/get/rejected").unwrap();
        assert_eq!(jobs.api, jobs::Topic::GetPendingFailed);
        assert_eq!(jobs.id, None);
    }
    #[test]
    fn match_topic_with_id_op() {
        let jobs = jobs::match_topic("$aws/things/chloe/jobs/example-job-01/get/accepted").unwrap();
        assert_eq!(jobs.api, jobs::Topic::DescribeSuccess);
        let id = jobs.id.unwrap();
        assert_eq!(&id[..], "example-job-01");
    }
    #[test]
    fn get_pending() {
        let topic = jobs::get_pending("chloe").unwrap();
        assert_eq!(&topic[..], "$aws/things/chloe/jobs/get");
    }
    #[test]
    fn start_next() {
        let topic = jobs::start_next("chloe").unwrap();
        assert_eq!(&topic[..], "$aws/things/chloe/jobs/start-next");
    }
    #[test]
    fn update() {
        let topic = jobs::update("chloe", "example-job-01").unwrap();
        assert_eq!(&topic[..], "$aws/things/chloe/jobs/example-job-01/update");
    }
}
