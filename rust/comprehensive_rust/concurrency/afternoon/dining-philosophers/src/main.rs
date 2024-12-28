use std::sync::Arc;
use tokio::sync::{mpsc, Mutex, MutexGuard, TryLockError};
use tokio::time;

struct Fork;

struct Philosopher {
    name: String,
    left_fork: Arc<Mutex<Fork>>,
    right_fork: Arc<Mutex<Fork>>,
    thoughts: mpsc::Sender<String>,
}

impl Philosopher {
    async fn think(&self) {
        self.thoughts
            .send(format!("Eureka! {} has a new idea!", &self.name))
            .await
            .unwrap();
    }

    async fn pick_up_forks(&self) -> (MutexGuard<'_, Fork>, MutexGuard<'_, Fork>) {
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
                Err(_) => time::sleep(time::Duration::from_millis(1)).await,
            }
        }
    }

    async fn eat(&self) {
        println!("{} is about to eat...", &self.name);
        let _forks = self.pick_up_forks().await;
        println!("{} is eating...", &self.name);
        time::sleep(time::Duration::from_millis(5)).await;
    }
}

static PHILOSOPHERS: &[&str] = &["Socrates", "Hypatia", "Plato", "Aristotle", "Pythagoras"];

#[tokio::main]
async fn main() {
    // Create forks

    // Create philosophers

    // Make them think and eat

    // Output their thoughts
}
