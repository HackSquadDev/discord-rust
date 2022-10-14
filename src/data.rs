use chrono::{DateTime, Utc};
use serenity::prelude::TypeMapKey;

use crate::environment::Configuration;

pub struct UptimeData;

impl TypeMapKey for UptimeData {
    type Value = DateTime<Utc>;
}

impl TypeMapKey for Configuration {
    type Value = Configuration;
}
