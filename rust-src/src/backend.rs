// backend.rs

use crate::comm;

pub fn backend() {
    // This is the main game loop.
    loop {
        // In a real implementation, this loop would handle all aspects of the game,
        // including user input, game logic, and network communication.

        // For now, we'll just process any incoming messages.
        let mut buf = String::new();
        if comm::get_message(&mut buf) {
            println!("Received message: {}", buf);
        }

        // We'll also add a small delay to prevent the loop from spinning too fast.
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
