use sha2::{Digest, Sha256};

mod repository;

pub struct Cacher {
    repository: repository::RedisRepository,
}

impl Cacher {
    pub fn new(db_url: String) -> Self {
        Cacher {
            repository: repository::instantiate_repository(db_url),
        }
    }

    pub fn exists(&mut self, value: &Vec<u8>) -> bool {
        let hashed = self.hash(&value);
        self.repository.exists(&hashed)
    }

    pub fn exists_or_save(&mut self, value: &Vec<u8>) -> bool {
        let hashed = self.hash(&value);
        let exists = self.repository.exists(&hashed);

        if !exists {
            self.repository.save(&hashed, &b"ok".to_vec()).unwrap();
        }

        exists
    }

    pub fn find(&mut self, value: &Vec<u8>) -> Vec<u8> {
        let hashed = self.hash(&value);
        self.repository.get(&hashed)
    }

    pub fn hash(&mut self, value: &Vec<u8>) -> String {
        let mut hasher = Sha256::new();
        hasher.update(value);
        let result = hasher.finalize();

        format!("{:x}", result)
    }

    pub fn update(&mut self, value: &Vec<u8>) -> String {
        let hashed = self.hash(value);
        self.repository.save(&hashed, &b"ok".to_vec()).unwrap();

        hashed
    }


}
