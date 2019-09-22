/*use redis;


pub fn conn() -> redis::RedisResult<()> {
    //the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `std::ops::Try`)
    let rh = redis::Client::open("redis://127.0.0.1:6379")?;

    let mut con = rh.get_connection()?;

    let ret = redis::cmd("auth").arg("83DUA4v0++").query::<String>(&mut con);
    println!("{:?}", ret);

    redis::cmd("set").arg("rust_key").arg(1233).execute(&mut con);
    let ret = redis::cmd("get").arg("rust_key").query::<i32>(&mut con);

//    con.set("rust_key", 1232);
//    let r = con.get("rust_key")?;
//
    println!("{:?}", ret);

    Ok(())
}*/

use std::collections::HashMap;
use simple_redis;

pub fn test() -> Result<(), simple_redis::RedisError> {

    let mut clients = simple_redis::create("redis://127.0.0.1:6379/0")?;

    let _ = clients.auth("****")?;

    let _ = clients.set("rust_key", "agc")?;

    let _ = clients.run_command("zadd", vec!["rust_zem", "1",  "me3"])?;
    let k = clients.run_command::<HashMap<String, String>>("zrange", vec!["rust_zem", "0", "-1", "WITHSCORES"])?;

    println!("{:?}", k);

    let _ = clients.quit();Ok(())

}