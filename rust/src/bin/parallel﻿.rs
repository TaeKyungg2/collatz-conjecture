use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
	// 채널 생성 (송신자, 수신자)
	let (tx, rx) = mpsc::channel();

	// 스레드 생성 및 메시지 전송
	thread::spawn(move || {
		let messages = vec!["안녕", "러스트", "멀티스레드", "메시지"];
		for msg in messages {
			tx.send(msg).unwrap();
			thread::sleep(Duration::from_millis(500));
		}
	});

	// 메인 스레드에서 메시지 수신
	for received in rx {
		println!("받은 메시지: {}", received);
	}
}
