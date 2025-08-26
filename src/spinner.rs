use std::io::{self, Write};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::time::Duration;

/// Run any future while showing a spinner on stderr.
/// The spinner clears itself before returning.
pub async fn with_spinner<F, T>(msg: &'static str, fut: F) -> T
where
    F: std::future::Future<Output = T>,
{
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    // let frames = ["   ", ".  ", ".. ", "...", " ..", "  ."];
    let frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

    let handle = tokio::spawn(async move {
        let mut i = 0usize;
        while r.load(Ordering::Relaxed) {
            eprint!("\r{} {}", frames[i % frames.len()], msg);
            let _ = io::stderr().flush();
            i = i.wrapping_add(1);
            tokio::time::sleep(Duration::from_millis(80)).await;
        }
        // Clear the line
        eprint!("\r\x1b[2K");
        let _ = io::stderr().flush();
    });

    let out = fut.await;
    running.store(false, Ordering::Relaxed);
    let _ = handle.await;
    out
}
// use 
// let resp: serde_json::Value =
//        spinner::with_spinner("thinking…", client.chat().create_byot(req)).await?;
// to call the spinning animation 

