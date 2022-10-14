use std::{collections::HashMap, sync::Arc};

use chrono::{DateTime, Utc};
use serenity::{model::prelude::UserId, prelude::TypeMapKey};
use tokio::sync::Mutex;

use crate::{database::Database, environment::Configuration, pagination::Pagination};

pub struct UptimeData;
pub struct PaginationMap;

impl TypeMapKey for UptimeData {
    type Value = DateTime<Utc>;
}

impl TypeMapKey for Configuration {
    type Value = Configuration;
}

impl TypeMapKey for Database {
    type Value = Database;
}

impl TypeMapKey for PaginationMap {
    type Value = Arc<Mutex<HashMap<UserId, Pagination>>>;
}
