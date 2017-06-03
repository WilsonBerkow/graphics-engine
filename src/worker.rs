use std::thread::{ self, JoinHandle };
use std::sync::mpsc::Receiver;
use std::sync::{ Arc, Mutex };

use render::Color;
use ppm;

pub struct WorkerPool {
    pub threads: Vec<JoinHandle<()>>,
    rx: Arc<Mutex<Receiver<(String, Box<Vec<Vec<Color>>>)>>>,
}

impl WorkerPool {
    pub fn new(rx: Receiver<(String, Box<Vec<Vec<Color>>>)>, n: usize) -> WorkerPool {
        let mut w = WorkerPool { threads: vec![], rx: Arc::new(Mutex::new(rx)) };
        for _ in 0..n {
            w.add_worker();
        }
        w
    }

    pub fn add_worker(&mut self) {
        let amrx = self.rx.clone();
        let handle = thread::spawn(move || {
            let mrx = amrx.as_ref();
            loop {
                let lock = mrx.lock().unwrap();
                let next = (*lock).iter().next();
                drop(lock);
                if let Some((filename, screen)) = next {
                    ppm::save_png(&screen, &filename);
                } else {
                    break;
                }
            }
        });
        self.threads.push(handle);
    }

    pub fn join(self) -> thread::Result<()> {
        for handle in self.threads {
            handle.join()?;
        }
        Ok(())
    }
}
