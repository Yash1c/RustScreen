use std::time::Duration;

fn main() {
	println!("[+] RustScreen starting. . .");
	
	loop {
		println!("[.] Waiting for frames. . .(Simulation)");
		std::thread::sleep(Duration::from_secs(5));

	}
}

