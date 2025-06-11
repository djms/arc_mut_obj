use std::sync::{Arc, Mutex};
use tokio::task::JoinHandle;
use tokio::time::{Duration, sleep};

struct MyStruct {
    value: Arc<Mutex<i32>>,
}

impl MyStruct {
    fn new(initial: i32) -> Self {
        MyStruct {
            value: Arc::new(Mutex::new(initial)),
        }
    }

    fn get_value(&self) -> i32 {
        let guard = self.value.lock().unwrap();
        *guard
    }

    fn set_value(&self, new_value: i32) {
        let mut guard = self.value.lock().unwrap();
        *guard = new_value;
    }

    fn start_process_a(&self) -> JoinHandle<()> {
        let this = self.clone_for_task();
        tokio::spawn(async move {
            loop {
                sleep(Duration::from_secs(1)).await;
                let current = this.get_value();
                this.set_value(current + 1);
            }
        })
    }

    fn start_process_b(&self) -> JoinHandle<()> {
        let this = self.clone_for_task();
        tokio::spawn(async move {
            loop {
                sleep(Duration::from_secs(5)).await;
                let current = this.get_value();
                this.set_value(current * 2);
            }
        })
    }

    fn clone_for_task(&self) -> MyStruct {
        MyStruct {
            value: Arc::clone(&self.value),
        }
    }
}

#[tokio::main]
async fn main() {
    let my_obj = MyStruct::new(0);

    let _a_handle = my_obj.start_process_a();
    let _b_handle = my_obj.start_process_b();

    for i in 1..10 {
        sleep(Duration::from_secs(1)).await;
        dbg!(my_obj.get_value());
    }
}
