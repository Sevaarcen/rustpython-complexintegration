use serde_json::json;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Sender, Receiver};
use uuid::Uuid;
use std::thread;
use log::{debug, info, warn, error};

use actix_web::web::Bytes;
use actix_web::http::StatusCode;
use actix_web::{HttpRequest, HttpResponse, HttpServer, Result, post};

use crate::model::DataObject;
use crate::plugin_handlers::Plugin;

mod plugin_handlers;
mod model;


fn manager_main(mut rx: Receiver<Bytes>) {
    info!("Manager main method is now running.");

    info!("Loading plugins...");
    let plugins = plugin_handlers::load_plugins();
    info!("Finished loading plugins, loaded a total of: {}", plugins.len());

    info!("Spawning tokio runtime");
    let runtime = tokio::runtime::Runtime::new().unwrap();

    // run logic loop until the transmitter is closed (no more jobs will be sent)
    loop {
        debug!("Top of Manager loop");
        let maybe_job = rx.blocking_recv();
        debug!("Manager received job");
        let job = match maybe_job {
            Some(job) => {
                job
            },
            None => {
                info!("Receiver is closed... shutting down manager");
                return;
            }
        };

        let task_uuid = Uuid::new_v4();
        let job_meta = json!({"uuid": task_uuid, "job_size": job.len()}).as_object().unwrap().to_owned();
        info!("Working job of size: {}", job_meta.get("job_size").unwrap());
        let data_object = DataObject{data: job.to_vec(), meta: job_meta};
        debug!("{:?}", data_object);
        for plugin in &plugins {
            debug!("POINTERS plugin: {:p},  plugins: {:p}", &plugin, &plugins);
            if plugin.check(&data_object) {
                info!("Plugin will run for job: {}", plugin);
                let plugin_copy = plugin.clone();
                let do_copy = data_object.clone();
                //let job_result = plugin.run(&do_copy);
                
                //info!("RESULTS OF PLUGIN JOB {}  ====>  {:?}", task_uuid, job_result);
                runtime.spawn(async move {
                    debug!("POINTERS plugin_copy: {:p}", &plugin_copy);
                    let job_result = plugin_copy.run(&do_copy);
                    //let job_result = model::PluginResults::None;
                    info!("Task {} finished and returned: {:?}", task_uuid, job_result);
                });
            }
        }
    }
}


#[post("/test")]
async fn dispatch(req: HttpRequest, body: Bytes) -> Result<HttpResponse> {
    debug!("Dispatching...");
    // take raw payload from POST request and transmit to worker manager for job dispatching.
    let tx = req.app_data::<Sender<Bytes>>().unwrap();
    tx.send(body).await.unwrap();
    
    debug!("Done dispatching");
    Ok(HttpResponse::build(StatusCode::CREATED)
        .body("Received")
    )
}


#[actix_web::main]
async fn main() {
    // initialize and setup logging
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    debug!("MAIN STARTING");

    // make channels for handling incoming/outgoing jobs
    let (manager_tx, manager_rx) = mpsc::channel::<Bytes>(25);
    
    // build and run Actix Web server
    debug!("Building and running Actix Web Server");
    // TODO remove below for testing, it should send two values which *should* execute in parallel and then end once they are worked to completion
    thread::spawn(move || {
        // send two jobs that should run in parallel
        manager_tx.blocking_send(Bytes::from("THIS IS A STRING WRITTEN BY A MAD MAN")).unwrap();
        manager_tx.blocking_send(Bytes::from("{\"WHY\": \"Is multiprocessing causing such undefined behavior... ?\"}")).unwrap();
        warn!("END OF SENDING");
    });
    /* 
    let server_future = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(manager_tx.clone())
            .service(dispatch)
    })
    .bind("127.0.0.1:8080").unwrap()
    .run();*/

    // Spawn worker manager in background
    debug!("Spawning worker manager");
    //let mut rt = actix_web::rt::System::new("workworkwork");
    //rt.block_on(manager_main(manager_rx));
    manager_main(manager_rx);
    //server_future.await.unwrap();

    debug!("END OF MAIN");
}