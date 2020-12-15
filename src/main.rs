#![allow(dead_code)]
#![allow(unused_imports)]

// mod errors;
// mod settings;
// mod stream;
// mod lexer;
// mod character_sets;
// mod tokens; 


// `block_on` blocks the current thread until the provided future has run to
// completion. Other executors provide more complex behavior, like scheduling
// multiple futures onto the same thread.
use futures::executor::block_on;
use std::time::Duration;
use std::thread;

struct Song {}

async fn learn_song() -> Song {
    println!("learning song");
    thread::sleep(Duration::from_secs(4));
    println!("song learned");
    return Song {};
 }
async fn sing_song(_song: Song) { 
    println!("singing song");
    thread::sleep(Duration::from_secs(4));
    println!("song sung");
 }
async fn dance() {
    println!("dancing song");
    thread::sleep(Duration::from_secs(4));
    println!("song danced");
 }


//  async fn learn_and_dance() {
//     let song = learn_song().await;
//     sing_song(song).await;
//  }

// async fn async_main() {
//     let f1 = learn_and_dance();
//     let f2 = dance();
//     futures::join!(f2, f1);
// }

// fn main() {
//     block_on(async_main());
// }
async fn learn_and_sing() {
    // Wait until the song has been learned before singing it.
    // We use `.await` here rather than `block_on` to prevent blocking the
    // thread, which makes it possible to `dance` at the same time.
    let song = learn_song().await;
    sing_song(song).await;
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();

    // `join!` is like `.await` but can wait for multiple futures concurrently.
    // If we're temporarily blocked in the `learn_and_sing` future, the `dance`
    // future will take over the current thread. If `dance` becomes blocked,
    // `learn_and_sing` can take back over. If both futures are blocked, then
    // `async_main` is blocked and will yield to the executor.
    futures::join!(f1, f2);
}

fn main() {
    block_on(async_main());
}