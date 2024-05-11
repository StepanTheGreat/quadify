use std::{io, task};

use miniquad::fs;
use oneshot::Receiver;

struct FileLoadingFuture(Receiver<Result<Vec<u8>, fs::Error>>);

impl std::future::Future for FileLoadingFuture {
	type Output = Result<Vec<u8>, fs::Error>;

	fn poll(self: std::pin::Pin<&mut Self>, _: &mut task::Context<'_>) -> task::Poll<Self::Output> {
		match self.0.try_recv() {
			Ok(res) => task::Poll::Ready(res),
			Err(oneshot::TryRecvError::Empty) => task::Poll::Pending,
			Err(oneshot::TryRecvError::Disconnected) => task::Poll::Ready(Err(fs::Error::IOError(io::Error::new(io::ErrorKind::Other, "File loading future was dropped")))),
		}
	}
}

/// Load a file from the filesystem or http on the web
pub async fn load_file(path: &str) -> Result<Vec<u8>, fs::Error> {
	let (sender, receiver) = oneshot::channel();
	fs::load_file(path, move |res| {
		let res = res.map(|mut data| {
			data.shrink_to_fit();
			data
		});
		sender.send(res).unwrap();
	});

	FileLoadingFuture(receiver).await
}

/// Load a file from the filesystem or http on the web, the parse as a string
pub async fn load_string(path: &str) -> Result<String, fs::Error> {
	let data = load_file(path).await?;
	Ok(String::from_utf8(data).unwrap())
}
