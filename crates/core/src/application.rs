use std::error::Error;
use std::process;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use crossbeam_channel::{bounded, Receiver, select};

pub trait Application: Drop {
    fn run_in_background(&self) -> Result<Receiver<Box<dyn Error>>, Box<dyn Error>> ;
    fn wait_for_stop(&self);
}

pub fn run_application<T: Application>(application: T) -> Result<(), Box<dyn Error>> {
    let termination_channel = kill_handler()?;
    let application_channel = application.run_in_background()?;
    loop {
        select! {
            recv(termination_channel) -> _ => {
                /*
                 * right now, can't figure out how to get ctrlc to tell me what
                 *      the signal was... oh well
                 */
                application.wait_for_stop();
                return Ok(())
            }
            recv(application_channel) -> error => {
                return match error {
                    Ok(app_error) => Err(app_error),
                    Err(recv_error) => {
                        println!("Issue with crossbeam channel, exiting");
                        application.wait_for_stop();
                        Err(Box::new(recv_error))
                    }
                }
            }
        }
    }
}

pub fn kill_handler() -> Result<Receiver<()>, ctrlc::Error> {
    let (sender, receiver) = bounded(100);
    let running = Arc::new(AtomicUsize::new(0));
    ctrlc::set_handler(move || {
        let has_quit = running.fetch_add(1, Ordering::SeqCst);
        if has_quit == 0 {
            let _ = sender.send(());
        } else {
            process::exit(0);
        }
    })?;
    Ok(receiver)
}