#![feature(decl_macro)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use] 
extern crate rocket;
extern crate rocket_contrib;

mod system;
mod api;

use std::sync::{Arc,RwLock};
use std::thread;
use std::time::Duration;

use machine_uid;
use hostname;
use systemstat::{System, Platform, saturating_sub_bytes};

use system::State;

fn create_and_initialise_state() -> State {
    let mut initial_state = State::new();

    initial_state.system_uid = machine_uid::get().unwrap();
    match hostname::get() {
        Ok(hostname) => {
            let hn_ref = hostname.to_string_lossy();
            initial_state.hostname = String::from(hn_ref)
        },
        Err(err) => println!("Hostname Error: {0}", err),
    }

    initial_state
}

fn real_time_loop(rt_loop_period: usize, state: Arc<RwLock<State>>, system: Arc<System>) {
    loop {
        thread::sleep(Duration::from_secs(rt_loop_period as u64));
        println!("RT Tick");

        match system.memory() {
            Ok(mem) => state.write().unwrap().memory = saturating_sub_bytes(mem.total, mem.free).as_u64(),
            Err(_) => {}
        }
    }
}

fn near_real_time_loop(nrt_loop_period: usize, state: Arc<RwLock<State>>, system: Arc<System>) {
    loop {
        thread::sleep(Duration::from_secs(nrt_loop_period as u64));
        println!("NRT Tick");
    }
}

fn main() {
    // Quick threading tutorial...
    // Arc stands for "Atomically Reference Counted". This is used to be able to share data across threads without transferring ownership.
    // RwLock stand for "Read Write Lock". This gives us unblocked reads across threads but blocked/guarded writes across threads.
    
    let initial_state = create_and_initialise_state();
    let global_state: Arc<RwLock<State>> = Arc::new(RwLock::new(initial_state));
    let rt_global_state = global_state.clone();
    let nrt_global_state = global_state.clone();
    let api_global_state = global_state.clone();

    let system = Arc::new(System::new());
    let rt_loop_system = system.clone();
    let nrt_loop_system = system.clone();


    println!("GLOBAL_STATE: {:?}", &*global_state.read().unwrap());

    thread::spawn(|| {
        near_real_time_loop(5, nrt_global_state, nrt_loop_system);
    });

    thread::spawn(|| {
        real_time_loop(1, rt_global_state, rt_loop_system);
    });

    api::start_api(api_global_state);
}
