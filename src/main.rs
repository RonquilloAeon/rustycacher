use std::io;
use std::io::Read;

use cacher::Cacher;

fn cache_from_stdin(cacher: &mut Cacher) -> bool {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    while buffer.ends_with('\n') || buffer.ends_with('\r') {
        buffer.pop();
    }

    cacher.exists_or_save(&Vec::from(buffer))
}

fn main() {
    let db_url = std::env::var("DB_URL").ok().unwrap();

    let mut cacher = Cacher::new(db_url);
    let exists = cache_from_stdin(&mut cacher);


    print!("Exists: {:?}", exists);
}
