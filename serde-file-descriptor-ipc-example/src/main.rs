use std::env;
use std::process;
use unix_ipc::{channel, Bootstrapper, Receiver, Sender};
use serde::{Deserialize, Serialize};

const ENV_VAR: &str = "PROC_CONNECT_TO";

#[derive(Serialize, Deserialize, Debug)]
pub enum Task {
    Sum(Vec<i64>, Sender<i64>),
    Shutdown,
}

fn main() {
    match env::var(ENV_VAR) {
        Ok(path) => {
            let receiver = Receiver::<Task>::connect(path).unwrap();
            loop {
                match receiver.recv().unwrap() {
                    Task::Sum(values, tx) => {
                        tx.send(values.into_iter().sum::<i64>()).unwrap();
                    }
                    Task::Shutdown => break,
                }
            }
        }
        _ => {
            let bootstrapper = Bootstrapper::new().unwrap();
            println!("Spawning process A");
            process::Command::new(env::current_exe().unwrap())
                .env(ENV_VAR, bootstrapper.path())
                .spawn()
                .unwrap();

            let (tx, rx) = channel().unwrap();
            println!("Sending task to process A");
            bootstrapper.send(Task::Sum(vec![23, 42], tx)).unwrap();
            println!("Receiving result from process A");
            println!("sum: {}", rx.recv().unwrap());
            bootstrapper.send(Task::Shutdown).unwrap();
        }
    }
}
