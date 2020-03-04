use async_std::net::TcpListener;
use async_std::prelude::*;

use std::io::Result as IOResult;

use async_std::task;

const canned_response: &'static str = "HTTP/1.1 200 Ok\r\nContent-length: 0\r\n\r\n";

fn main() -> IOResult<()> {
  println!("spawning async thread");

  task::block_on(async {
    println!("thread open, connecting listener");
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    let mut incoming = listener.incoming();

    while let Some(attempt) = incoming.next().await {
      let mut connection = match attempt {
        Ok(connection) => connection,
        Err(e) => {
          println!("received invalid connection: {}", e);
          continue;
        }
      };

      println!("received connection from {:?}", connection.peer_addr());

      std::thread::sleep(std::time::Duration::from_secs(10));

      if let Err(e) = write!(connection, "{}", canned_response).await {
        println!("unable to write response: {}", e);
      }

      println!("main thread continuing on");
      drop(connection);
    }

    Ok(())
  })
}
