#![warn(rust_2018_idioms)]
#![cfg(feature = "full")]

use tokio::runtime::Runtime;
use tokio::sync::oneshot;
use tokio::time;
use tokio_test::assert_ok;

use std::time::{Duration, Instant};

const MAX_THROTTLING: Duration = Duration::from_millis(50);

#[test]
fn delay_at_root_one_time_frame() {
    let mut rt = rt();

    let now = Instant::now();
    let dur = MAX_THROTTLING;

    rt.block_on(async move {
        time::delay_for(dur).await;
    });

    let elapsed = now.elapsed();
    assert!(elapsed >= dur);
    // delay is created during the first time frame
    // and must be fired at the beginning of the next time frame
    assert!(elapsed < MAX_THROTTLING + (MAX_THROTTLING / 2));
}

#[test]
fn delay_in_spawn_one_time_frame() {
    let mut rt = rt();

    let now = Instant::now();
    let dur = MAX_THROTTLING;

    rt.block_on(async move {
        let (tx, rx) = oneshot::channel();

        tokio::spawn(async move {
            time::delay_for(dur).await;
            assert_ok!(tx.send(()));
        });

        assert_ok!(rx.await);
    });

    let elapsed = now.elapsed();
    assert!(elapsed >= dur);
    // delay is created during the first time frame
    // and must be fired at the beginning of the next time frame
    assert!(elapsed < MAX_THROTTLING + (MAX_THROTTLING / 2));
}

#[test]
fn delay_at_root_two_time_frames() {
    let mut rt = rt();

    let now = Instant::now();
    let dur = MAX_THROTTLING * 2;

    rt.block_on(async move {
        time::delay_for(dur).await;
    });

    let elapsed = now.elapsed();
    assert!(elapsed >= dur);
    // delay is created during the first time frame
    // and must be fired after the end of next time frame
    assert!(elapsed < MAX_THROTTLING * 2 + (MAX_THROTTLING / 2));
}

#[test]
fn delay_in_spawn_two_time_frames() {
    let mut rt = rt();

    let now = Instant::now();
    let dur = MAX_THROTTLING * 2;

    rt.block_on(async move {
        let (tx, rx) = oneshot::channel();

        tokio::spawn(async move {
            time::delay_for(dur).await;
            assert_ok!(tx.send(()));
        });

        assert_ok!(rx.await);
    });

    let elapsed = now.elapsed();
    assert!(elapsed >= dur);
    // delay is created during the first time frame
    // and must be fired after the end of next time frame
    assert!(elapsed < MAX_THROTTLING * 2 + (MAX_THROTTLING / 2));
}

fn rt() -> Runtime {
    tokio::runtime::Builder::new()
        .basic_scheduler()
        .enable_all()
        .max_throttling(MAX_THROTTLING)
        .build()
        .unwrap()
}
