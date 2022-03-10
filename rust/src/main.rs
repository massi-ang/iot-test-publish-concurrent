use aws_sdk_iotdataplane::{Client, Error, types::Blob, Endpoint};
use std::env;
// Ensure you add a dependency on the HTTP crate
use http::Uri;
use std::time::{Instant};
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use std::iter::Iterator;

use tracing_chrome::ChromeLayerBuilder;
use tracing_subscriber::{registry::Registry, prelude::*};

use std::{thread, time};

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Error> {
    let sleep = time::Duration::from_millis(2000);
    let (chrome_layer, _guard) = ChromeLayerBuilder::new().build();
    tracing_subscriber::registry().with(chrome_layer).init();

    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::DEBUG)
    //     .with_ansi(false)
    //     .init();

    let num_topics: i32;
    match env::var("NUM_TOPICS") {
        Ok(val) => num_topics = val.parse::<i32>().unwrap(),
        Err(_) => num_topics = 1000,
    }
    let config = aws_config::load_from_env().await;
    let conf = aws_sdk_iotdataplane::config::Builder::from(&config)
        .endpoint_resolver(Endpoint::immutable(Uri::from_static("https://acwprzq1nws9h-ats.iot.eu-west-1.amazonaws.com"))).build();
    let client = Client::from_conf(conf);
    println!("Publishing to {} topics", num_topics);
    let now = Instant::now();

    futures::future::join_all((0..num_topics).map(|i|
        client.publish()
            .topic(format!("test_{}", i))
            .payload(Blob::new("hello"))
            .send()
    )).await;

    println!("Published in {}ms", now.elapsed().as_millis());
    
    thread::sleep(sleep);
    println!("Re-Publishing to {} topics", num_topics);
    let now = Instant::now();

    futures::future::join_all((0..num_topics).map(|i|
        client.publish()
            .topic(format!("test_{}", i))
            .payload(Blob::new("hello"))
            .send()
    )).await;

    println!("Published in {}ms", now.elapsed().as_millis());
    thread::sleep(sleep);
    let start = Instant::now();
    let futs = (0..num_topics).map(|i|
        client.publish()
            .topic(format!("test_{}", i))
            .payload(Blob::new("hello"))
            .send()
    ).collect::<Vec<_>>();

    println!("Futures {}", futs.len());

    let mut futs_all: FuturesUnordered<_> = futs.into_iter().collect();

    loop {
        match futs_all.next().await {
            Some(_) => (),
            None => {println!("Published in {}ms", start.elapsed().as_millis());
            break;
        }
        }
    }

    
    Ok(())
}