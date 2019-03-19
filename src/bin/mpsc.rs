use std::process;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || loop {
        if let Ok(d) = rx.recv() {
            println!("{:?}", d);

            if d == "over" {
                process::exit(0);
            }
        }
    });

    for i in 0..10 {
        let tx = tx.clone();
        thread::sleep(Duration::from_millis(500));
        thread::spawn(move || {
            let msg = if i == 9 {
                "over".to_owned()
            } else {
                format!("message from thread: {}", i)
            };
            tx.send(msg).expect("error send message to channel");
        });
    }

    loop {}
}
