extern crate futures_util;
extern crate tokio;

extern crate tokio_icmp;

use crate::futures_util::{future, StreamExt};
use std::net::Ipv4Addr;

#[tokio::main]
async fn main() {
    // let addr = std::env::args().nth(1).unwrap().parse().unwrap();

    let pinger = tokio_icmp::Pinger::new().await.unwrap();
    let stream = pinger
        .chain(Ipv4Addr::new(114, 114, 114, 114).into())
        .stream();
    stream
        .take(3)
        .for_each(|mb_time| {
            match mb_time {
                Ok(Some(time)) => println!("time={:?}", time),
                Ok(None) => println!("timeout"),
                Err(err) => println!("error: {:?}", err),
            }
            future::ready(())
        })
        .await;
}
