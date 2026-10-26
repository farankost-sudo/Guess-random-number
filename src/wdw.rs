use std::io;
use rand::Rng;


fn main()
{
  let apple = 10;
    {
        let apple = 5;
        println!("{apple}")
    }
    println!("{apple}")
}