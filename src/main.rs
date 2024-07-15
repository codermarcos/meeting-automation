use chrono::prelude::*;
use cron::Schedule;
use enigo::{
    Direction::{Press, Release},
    Enigo, Key, Keyboard, Settings,
};
use std::{env, str::FromStr};
use tokio::time::{sleep_until, Instant};

use std::{thread, time};

trait PageTrait {
    fn press(&mut self, key: Key, wait: bool);
}

impl PageTrait for Enigo {
    fn press(&mut self, key: Key, wait: bool) {
        let _ = self.key(key, Press);
        let _ = self.key(key, Release);
        if wait {
            thread::sleep(ONE_SECOND);
        }
    }
}

const TEN_SECONDS: time::Duration = time::Duration::from_secs(10 as u64);
const ONE_SECOND: time::Duration = time::Duration::from_secs(1 as u64);

fn join_to_metting(url: String) {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    enigo.press(Key::Meta, true);

    let _ = enigo.text("chrome");

    enigo.press(Key::Return, true);

    let _ = enigo.text(&url);

    enigo.press(Key::Return, true);

    thread::sleep(TEN_SECONDS);

    // press mute
    enigo.press(Key::F, true);
    enigo.press(Key::E, true);
    // --

    // press close camera
    enigo.press(Key::F, true);
    enigo.press(Key::F, true);
    // --

    // press confirm close camera
    enigo.press(Key::F, true);
    enigo.press(Key::S, false);
    enigo.press(Key::S, true);
    // --

    // press join meeting
    enigo.press(Key::F, true);
    enigo.press(Key::M, false);
    // --
}

#[tokio::main]
async fn main() {
    if let [_, cron_expression, url] = env::args().collect::<Vec<String>>().as_slice() {
        let schedule = Schedule::from_str(cron_expression).expect("Invalid cron expression");

        loop {
            let now = Utc::now();
            let upcoming = schedule.upcoming(Utc).take(1).next().unwrap();
            let duration_until_next = upcoming.signed_duration_since(now).to_std().unwrap();
            let next_instant = Instant::now() + duration_until_next;

            println!("⏱️ Scheduled to open!\n This meeting: {url}\n At: {}", upcoming);
            sleep_until(next_instant).await;

            join_to_metting(url.to_string());
        }
    } else {
        panic!("missing meeting url!");
    }
}
