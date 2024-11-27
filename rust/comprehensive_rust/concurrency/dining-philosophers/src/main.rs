use std::sync::{mpsc, Arc, Mutex, MutexGuard};
use std::thread;
use std::time::Duration;

struct Fork;

struct Philosopher {
    name: String,
    left_fork: Arc<Mutex<Fork>>,
    right_fork: Arc<Mutex<Fork>>,
    thoughts: mpsc::SyncSender<String>,
}

impl Philosopher {
    fn think(&self) {
        self.thoughts
            .send(format!("Eureka! {} has a new idea!", &self.name))
            .unwrap();
    }

    fn pick_up_forks(&self) -> (MutexGuard<'_, Fork>, MutexGuard<'_, Fork>) {
        loop {
            let (fst, snd) = if rand::random::<bool>() {
                (self.left_fork.as_ref(), self.right_fork.as_ref())
            } else {
                (self.right_fork.as_ref(), self.left_fork.as_ref())
            };
            match fst
                .try_lock()
                .and_then(|fst| snd.try_lock().map(|snd| (fst, snd)))
            {
                Ok(x) => return x,
                Err(_) => thread::sleep(Duration::from_millis(1)),
            }
        }
    }

    fn eat(&self) {
        println!("{} is about to eat...", &self.name);
        let _forks = self.pick_up_forks();
        println!("{} is eating...", &self.name);
        thread::sleep(Duration::from_millis(10));
    }
}

static PHILOSOPHERS: &[&str] = &["Socrates", "Hypatia", "Plato", "Aristotle", "Pythagoras"];

fn main() {
    let (tx, rx) = mpsc::sync_channel(10);

    let forks = (0..PHILOSOPHERS.len())
        .map(|_| Arc::new(Mutex::new(Fork)))
        .collect::<Vec<_>>();

    for i in 0..PHILOSOPHERS.len() {
        let left_fork = Arc::clone(&forks[i]);
        let right_fork = Arc::clone(&forks[(i + 1) % PHILOSOPHERS.len()]);
        let thoughts = tx.clone();

        let philosopher = Philosopher {
            name: PHILOSOPHERS[i].to_string(),
            left_fork,
            right_fork,
            thoughts,
        };

        thread::spawn(move || {
            for _ in 0..100 {
                philosopher.eat();
                philosopher.think()
            }
        });
    }

    drop(tx);
    for thought in rx {
        println!("{thought}");
    }
}
