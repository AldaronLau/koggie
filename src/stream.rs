use futures::sync::mpsc;
use actix_web::web::Bytes;
use opus::{ Channels, Application };

pub fn stream() -> mpsc::UnboundedReceiver<Bytes> {
    let (send, recv) = mpsc::unbounded();

    std::thread::spawn(move || {
        let send = send;

	    let mut opus = opus::Encoder::new(48000, Channels::Stereo, Application::Audio).unwrap();
        let mut buffer = vec![];

	    loop {
            // Add new audio to buffer.
            let mut bytes = Bytes::new();
            buffer.clear();
            //
            for i in 0..(1920*2) {
                buffer.push((((i as f32 / 1920.0) - 1.0) * std::i16::MAX as f32) as i16);
            }
            std::thread::sleep(std::time::Duration::from_millis(1000 / 48));

        	let mut netbuf = [0u8; 1920 * 2];

            //
		    let nl = opus.encode(buffer.as_slice(), &mut netbuf).unwrap();
            bytes.extend_from_slice(&netbuf[..nl]);
            if send.unbounded_send(bytes).is_err() {
                return;
            }
	    }
    });

    recv
}
