use chrono::{DateTime, Utc};
use serenity::prelude::TypeMapKey;

pub struct UptimeData;

impl TypeMapKey for UptimeData {
    type Value = DateTime<Utc>;
}
