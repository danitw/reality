fn main() {
  let a = 1;

  world_funcionally(a);
}

fn world_funcionally(a: i32) -> bool {
  if a == 1 {
    println!("The universe is operating normally.");
    return true;
  } else {
      loop {
        println!("The universe has a bug");
      }

    return false;
  }
}
