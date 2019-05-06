// KOGGIE SERVER

use cala;
use opus;

use opus::{ Channels, Application };

use std::net::UdpSocket;

fn main() {
    let mut app = cala::App::new(());

    // TODO: Make based on command line argument.
	let socket = UdpSocket::bind("10.0.0.159:8000").expect("Could not bind socket");

	// Wait for a connection.
	println!("Waiting for a connection....");
	let connection = {
		let mut buf = [0u8; 8];
		match socket.recv_from(&mut buf) {
			Ok((size, src)) => {
				if size == 8 && buf == *b"electric" {
					src
				} else {
					panic!("Error: size:{}, buf:{}",
						size, std::str::from_utf8(
							&buf).unwrap())
				}
			},
			Err(e) => {
				panic!("couldn't recieve a datagram: {}", e)
			},
		}
	};
	println!("Connected!");

	let mut netbuf = [0u8; 480*2];

	let mut opus = opus::Encoder::new(48000, Channels::Stereo, Application::Audio).unwrap();
    let mut buffer = vec![];

	loop {
        // Add new audio to buffer.
        buffer.clear();
		app.record(
            &mut |_mic_id, l, r| {
                buffer.push(l);
                buffer.push(r);
            }
        );

		let nl = opus.encode(buffer.as_slice(), &mut netbuf).unwrap();

		socket.send_to(&netbuf[..nl], &connection)
			.expect("Couldn't send");
	}
}
