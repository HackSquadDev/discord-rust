use redis::{Client, Commands, Connection, RedisError};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{environment::Configuration, CONFIG};

#[derive(Default)]
pub struct Database {
    client: Option<Client>,
    config: Configuration,
}

impl Database {
    pub async fn initialize(&mut self) {
        self.config = CONFIG.lock().await.clone();
        self.establish_connection(self.config.redis_uri.clone());
    }

    pub fn establish_connection(&mut self, connection_string: String) {
        let client = Client::open(connection_string);

        match client {
            Ok(client) => {
                println!("Connected to Redis!");
                self.client = Some(client)
            }
            Err(error) => todo!("{}", error),
        }
    }

    pub fn get_client(&self) -> &Client {
        let client = &self.client;

        match client {
            Some(client) => client,
            None => todo!("this is not good"),
        }
    }

    pub fn get_connection(&self) -> Connection {
        let con = self.get_client().get_connection();

        match con {
            Ok(con) => con,
            Err(_) => todo!("Did someone forgot to initialize connection?"),
        }
    }

    pub fn save(&self, key: &str, value: &str, ttl_seconds: usize) {
        let mut con = self.get_connection();

        redis::pipe()
            .cmd("SET")
            .arg(key)
            .arg(value)
            .cmd("EXPIRE")
            .arg(key)
            .arg(ttl_seconds)
            .execute(&mut con);
    }

    pub fn get(&self, key: &str) -> Result<String, RedisError> {
        let mut con = self.get_connection();

        let data: Result<String, RedisError> = con.get(key);

        data
    }

    pub async fn request<T: for<'a> Deserialize<'a> + Serialize>(
        &self,
        url: &str,
        key: &str,
        ttl_seconds: usize,
    ) -> Result<T, reqwest::Error> {
        let cached = self.get(key);

        match cached {
            Ok(data) => Ok(serde_json::from_str::<T>(&data).unwrap()),
            Err(_) => {
                let api_response: T = reqwest::get(url).await?.json().await?;

                self.save(key, &json!(api_response).to_string(), ttl_seconds);

                Ok(api_response)
            }
        }
    }
}
