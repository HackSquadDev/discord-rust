use redis::Client;

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
}
