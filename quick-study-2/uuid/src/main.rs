use std::sync::Mutex;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id(u32);

impl Id {
    pub fn new(id: u32) -> Self {
        Id(id)
    }
}

pub struct GenId {
    next: Mutex<u32>,
}

impl GenId {
    pub fn new() -> Self {
        GenId {
            next: Mutex::new(0),
        }
    }
    pub fn gen(&self) -> Id {
        let mut next = self.next.lock().unwrap();
        let id = Id::new(*next);
        *next += 1;
        id
    }
}

fn main() {
    let gen = GenId::new();
    let id1 = gen.gen();
    let id2 = gen.gen();
    println!("{:?}", id1);
    println!("{:?}", id2);
}
