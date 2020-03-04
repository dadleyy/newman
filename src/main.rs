use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use async_std::sync::Arc;

use std::io::Result as IOResult;

use async_std::task;

const CANNED_RESPONSE: &'static str = "HTTP/1.1 200 Ok\r\nContent-length: 0\r\n\r\n";

struct ResourcePool {
  #[allow(unused)]
  pools: u8,
}

impl ResourcePool {
  pub async fn send(&self) -> IOResult<()> {
    std::thread::sleep(std::time::Duration::from_secs(3));
    Ok(())
  }
}

async fn handle(local_pool: Arc<ResourcePool>, mut connection: TcpStream) -> IOResult<()> {
  let addr = match connection.peer_addr() {
    Ok(addr) => addr,
    Err(e) => {
      println!("unable to get peer addr: {}", e);
      return Err(e);
    }
  };

  println!("connection[{}] thread spawned, sleeping for 10 seconds first", addr);
  local_pool.send().await;
  println!("connection[{}] waking up, writing response", addr);
  let response = String::from(CANNED_RESPONSE);

  if let Err(e) = async_std::io::copy(&mut response.as_bytes(), &mut connection).await {
    println!("connection[{}] unable to write response: {}", addr, e);
  }

  println!("connection[{}] done", addr);
  drop(connection);
  Ok(())
}

fn main() -> IOResult<()> {
  println!("spawning async thread");

  task::block_on(async {
    println!("thread open, connecting listener");
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    let mut incoming = listener.incoming();
    let pool = Arc::new(ResourcePool { pools: 10u8 });

    while let Some(attempt) = incoming.next().await {
      let connection = match attempt {
        Ok(connection) => connection,
        Err(e) => {
          println!("received invalid connection: {}", e);
          continue;
        }
      };

      println!("received valid connection, spawning thread");
      let local_pool = pool.clone();

      task::spawn(async {
        handle(local_pool, connection).await;

        /*
        let local_pool = pool.clone();
        let addr = match connection.peer_addr() {
          Ok(addr) => addr,
          Err(e) => {
            println!("unable to get peer addr: {}", e);
            return;
          }
        };

        println!("connection[{}] thread spawned, sleeping for 10 seconds first", addr);
        local_pool.send().await;
        println!("connection[{}] waking up, writing response", addr);
        let response = String::from(CANNED_RESPONSE);

        if let Err(e) = async_std::io::copy(&mut response.as_bytes(), &mut connection).await {
          println!("connection[{}] unable to write response: {}", addr, e);
        }

        println!("connection[{}] done", addr);
        drop(connection);
        */
      });

      println!("main thread continuing");
    }

    Ok(())
  })
}
