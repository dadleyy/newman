use std::io::Result as IOResult;

use async_std::task;

fn main() -> IOResult<()> {
  println!("spawning async thread");

  task::block_on(async {
    println!("thread open");
    Ok(())
  })
}
