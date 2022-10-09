use redis::{Client, Cmd, Commands, Connection, RedisError};

#[derive(Default)]
pub struct Database {
    client: Option<Client>,
}

impl Database {
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
}
