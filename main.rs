extern crate futures;
extern crate tokio;
extern crate tokio_stream;

use std::time::Duration;

use tokio::{
    sync::{Semaphore, SemaphorePermit},
    time::sleep,
};

pub struct ForkSys {
    pub semaphore: Semaphore,
}

impl ForkSys {
    #[inline]
    pub const fn new(forks: usize) -> Self {
        Self {
            semaphore: Semaphore::const_new(forks),
        }
    }
}

static FORK_SYS: ForkSys = ForkSys::new(5);

pub struct Philosopher<'a> {
    name: String,
    forks: usize,
    pub borrowed_forks: Vec<SemaphorePermit<'a>>,
}

impl Philosopher<'static> {
    #[inline]
    pub fn new(name: String) -> Self {
        Self {
            name,
            forks: 0,
            borrowed_forks: vec![],
        }
    }

    #[inline]
    pub fn thinking(&self) {
        println!("{} has started thinking", self.name)
    }

    #[inline]
    pub fn can_eat(&self) -> bool {
        self.forks == 2
    }

    #[inline]
    pub async fn eat(&mut self) {
        println!("{} has started eating", self.name);
        sleep(Duration::from_secs(1)).await;
        println!("{} has finished eating", self.name);
        self.return_forks();
    }

    #[inline]
    fn return_one_fork(&mut self) {
        self.forks -= 1;
        self.borrowed_forks.pop();
    }

    #[inline]
    fn return_forks(&mut self) {
        self.return_one_fork();
        self.return_one_fork();
        println!("{} has returned forks", self.name);
        self.thinking();
    }

    #[inline]
    async fn take_fork_unchecked(&mut self) {
        if let Ok(fork) = {
            let x = FORK_SYS.semaphore.acquire().await;
            x
        } {
            self.forks += 1;
            self.borrowed_forks.push(fork)
        }
    }

    #[inline]
    pub async fn take_forks(&mut self) {
        match self.forks {
            0 => match { FORK_SYS.semaphore.acquire().await } {
                Ok(fork) => {
                    self.forks += 1;
                    self.borrowed_forks.push(fork);
                    self.take_fork_unchecked().await;
                }

                Err(_) => return,
            },

            1 => self.take_fork_unchecked().await,
            _ => return,
        }

        println!("{} has taken forks (now: {})", self.name, self.forks);

        if self.can_eat() {
            self.eat().await;
        }
    }
}

#[tokio::main(flavor = "multi_thread", worker_threads = 5)]
async fn main() {
    futures::future::join_all((1..=5).map(move |number| {
        let mut philosopher = Philosopher::new(format!("Philosopher №{number}"));

        tokio::task::spawn(async move {
            loop {
                philosopher.take_forks().await;
            }
        })
    }))
    .await;
}
