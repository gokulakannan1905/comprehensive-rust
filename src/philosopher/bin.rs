use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Fork;

struct Philosopher {
    name: String,
    left_fork: Arc<Mutex<Fork>>,
    right_fork: Arc<Mutex<Fork>>,
    thoughts: mpsc::SyncSender<String>
}

impl Philosopher {
    fn think(&self) {
        self.thoughts
            .send(format!("Eureka! {} has a new idea!", &self.name))
            .unwrap();
    }

    fn eat(&self) {
        println!("{} is eating...", &self.name);
        let _left_fork = self.left_fork.lock().expect("Cannot acquire the mutex guard");
        let _right_fork = self.right_fork.lock().expect("Cannot acquire the mutex guard");
        thread::sleep(Duration::from_millis(10));
        println!("{} finished eating", &self.name);
    }
}

static PHILOSOPHERS: &[&str] =
    &["Socrates", "Plato", "Aristotle", "Thales", "Pythagoras"];

fn main() {

    let (tx,rx) = mpsc::sync_channel(10);
    // Create forks
    let forks = (0..PHILOSOPHERS.len()).map(|_| Arc::new(Mutex::new(Fork))).collect::<Vec<_>>();

    for i in 0..forks.len(){
        let tx = tx.clone();
        let mut left_fork = forks[i].clone();
        let mut right_fork = forks[(i+1)%forks.len()].clone();

        if i == 2 {
            std::mem::swap(&mut left_fork, &mut right_fork);
        }
        
        // Create philosophers

        let philosopher = Philosopher{
            name: PHILOSOPHERS[i].to_string(),
            thoughts: tx,
            left_fork,
            right_fork
        };
        // Make them think and eat

        thread::spawn(move ||{
            for _ in 0..100{
                philosopher.eat();
                philosopher.think();
            }
        });
    }
    drop(tx);
    for thoughts in rx{
        println!("{thoughts}");
    }
    // Output their thoughts
}