extern crate redis;

use redis::Commands;

pub struct RedisRepository {
    connection: redis::Connection,
}

impl RedisRepository {
    pub fn new(connection: redis::Connection) -> Self {
        RedisRepository {
            connection,
        }
    }

    pub fn get(&mut self, key: &String) -> Vec<u8> {
        self.connection.get(key).unwrap()
    }

    pub fn exists(&mut self, key: &String) -> bool {
        self.connection.exists(key).unwrap()
    }

    pub fn save(&mut self, key: &String, value: &Vec<u8>) -> redis::RedisResult<()> {
        self.connection.set(key, value)
    }
}

pub fn instantiate_repository(db_url: String) -> RedisRepository {
    let conn = redis::Client::open(db_url).unwrap().get_connection().unwrap();

    RedisRepository::new(conn)
}
