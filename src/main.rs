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

      println!("received valid connection");

      task::spawn(async {
        let addr = match connection.peer_addr() {
          Ok(addr) => addr,
          Err(e) => {
            println!("unable to get peer addr: {}", e);
            return;
          }
        };

        println!("connection[{}] thread spawned, sleeping for 10 seconds first", addr);
        std::thread::sleep(std::time::Duration::from_secs(10));

        println!("connection[{}] waking up, writing response", addr);
        let response = String::from(canned_response);

        if let Err(e) = async_std::io::copy(&mut response.as_bytes(), &mut connection).await {
          println!("connection[{}] unable to write response: {}", addr, e);
        }

        println!("connection[{}] done", addr);
        drop(connection);
      });

      println!("main thread continuing");
    }

    Ok(())
  })
}
