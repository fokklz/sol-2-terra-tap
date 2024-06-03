use modules::{SensorModule, WateringModule};
use once_cell::sync::{Lazy, OnceCell};
use std::{
    io,
    process::{Child, Command, Stdio},
};
use tokio::{
    signal,
    sync::{broadcast, Mutex},
};
use traits::ConfigFile;

mod core;
pub use core::*;

mod modules;
mod mqttc;
mod settings;
mod state;

pub use settings::Settings;
pub use state::State;

// Re-export the client for easy access
pub use mqttc::CLIENT;

/// Check if the program is running in release mode and dissallow it.
/// Currently only running from source is intended.
fn check_release_mode() {
    if cfg!(debug_assertions) {
        return;
    }
    eprintln!("Error: Running in release mode is not supported.");
    std::process::exit(1);
}

/// Spawn the MQTT broker as a child process
fn spawn_broker() -> io::Result<Child> {
    Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("mqttd")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
}

// Global handle to the MQTT broker
static BROKER: OnceCell<Mutex<Child>> = OnceCell::new();
// Global handle to the settings
static SETTINGS: OnceCell<Mutex<Settings>> = OnceCell::new();
// Global handle to the state
static STATE: OnceCell<Mutex<State>> = OnceCell::new();
// Global handle to the module manager
static MODULE_MANAGER: Lazy<Mutex<ModuleManager>> = Lazy::new(|| Mutex::new(ModuleManager::new()));

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "debug");
    check_release_mode();

    tracing_subscriber::fmt::init();

    // Shutdown channel to ensure graceful shutdown
    let (shutdown_tx, shutdown_rx) = broadcast::channel::<()>(1);

    // Spin up a task to listen for Ctrl+C
    let ctrl_c_task = tokio::spawn(async move {
        signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");
        // Send shutdown signal
        let _ = shutdown_tx.send(());
    });

    // #####################################################################
    // # Code above should not be modified, as it ensures the proper operation of the program.
    // # It also guarantees that the program will log anything it does

    // create settings and state handles
    let settings = Settings::load();
    let state = State::load();
    let _ = STATE.set(Mutex::new(state)).unwrap();
    let _ = SETTINGS.set(Mutex::new(settings.clone())).unwrap();

    // Register the modules
    let mut manager = MODULE_MANAGER.lock().await;
    manager.register_module(SensorModule::from(&settings));
    manager.register_module(WateringModule::from(&settings));
    // ensure the manager is available for the client
    drop(manager);

    tracing::info!("TerraTap running... Press Ctrl+C to exit.");
    tracing::info!("Starting MQTT broker... This may take a few seconds.");
    let broker = spawn_broker().expect("Failed to spawn broker");
    let _ = BROKER.set(Mutex::new(broker)).unwrap();

    let client_task = mqttc::run(shutdown_rx);

    // Wait for either Ctrl+C or the client task to finish
    // The client task should not finish
    // TODO: auto-restart mechanism instead of just exiting
    loop {
        tokio::select! {
            _ = ctrl_c_task => {
                break;
            }
            _ = client_task.await => {
                break;
            }
        }
    }
    // kill the broker to be sure all tasks are cleaned up
    let _ = BROKER.get().unwrap().lock().await.kill();
    // Save the state and settings before exiting
    STATE.get().unwrap().lock().await.save();
    SETTINGS.get().unwrap().lock().await.save();

    tracing::info!("Thank you for using TerraTap! Until next time!");
}
