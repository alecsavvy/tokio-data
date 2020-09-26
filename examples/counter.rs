#[macro_use]
extern crate tokio_data;
#[macro_use]
extern crate tokio;

#[derive(Debug)]
pub struct Counter {
    count: usize,
}

impl Counter {
    pub fn new() -> Self {
        Counter { count: 0 }
    }
    pub fn add(&mut self) {
        self.count = self.count + 1;
    }

    pub fn get(&self) -> usize {
        self.count
    }
}

tokio_data!(Counter, Counter::new());

#[tokio::main]
async fn main() {
    let add_two = spawn_data(|data| async move {
        let mut data = data.lock().await;
        data.add();
        data.add();
        println!("counter at {:?}!", data.get());
        data.get()
    });

    let add_one = spawn_data(|data| async move {
        tokio::time::delay_for(std::time::Duration::from_millis(10000)).await;
        let mut data = data.lock().await;
        data.add();
        println!("counter at {:?}!", data.get());
        data.get()
    });

    // two will complete first, then one will execute
    let (one, two) = join!(add_one, add_two);

    println!("one: {}", one.unwrap());
    println!("two: {}", two.unwrap())
}
