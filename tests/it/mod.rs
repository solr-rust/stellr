extern crate serde_json;
extern crate shiplift;
extern crate tokio;

use futures::StreamExt;
use serde_json::value::Value::{Object, String as Str};
use shiplift::rep::ContainerCreateInfo;
use shiplift::{tty::TtyChunk, ExecContainerOptions};
use shiplift::{BuildOptions, Container, ContainerOptions, Docker, RmContainerOptions};
use std::str::from_utf8;
// important that this uses the tokio v0.2 runtime, will panic with v0.3
use tokio::runtime::Runtime;

use std::error::Error;
use std::sync::Mutex;

const IMAGE: &str = "solr-stellr-it:Latest";
const DOCKER_DIR: &str = "Docker/it";
// a "prepopulated" dockerfile exists as well, but restoring the solr films collection takes 4+
// seconds from startup (tests happening too quickly would fail)

#[allow(dead_code)]
pub struct SolrContainer {
    pub details: Mutex<(u32, Option<String>)>,
}

impl SolrContainer {
    #[allow(dead_code)]
    pub fn new() -> SolrContainer {
        SolrContainer {
            details: Mutex::new((0, None)),
        }
    }

    #[allow(dead_code)]
    pub fn register(&self) {
        let mut guard = self
            .details
            .lock()
            .expect("Unwrapping the mutex (register)");
        let mut usage_counter = guard.0;
        if usage_counter == 0 {
            let new_container_id = start_container().expect("container initialisation failed");
            populate_container(&new_container_id).expect("Container population failed");
            guard.0 = 1;
            guard.1.replace(new_container_id);
        } else {
            usage_counter += 1;
            guard.0 = usage_counter;
        }
    }

    #[allow(dead_code)]
    pub fn deregister(&self) {
        let mut guard = self
            .details
            .lock()
            .expect("Unwrapping the mutex (deregister)");
        let mut usage_counter = guard.0;
        if usage_counter == 1 {
            guard.0 = 0;
            let container_id = guard.1.take();
            remove_container(&container_id.unwrap()).expect("Container removal failed");
        } else {
            usage_counter -= 1;
            guard.0 = usage_counter;
        }
    }
}

#[allow(dead_code)]
fn start_container() -> Result<String, Box<dyn Error>> {
    let mut rt = Runtime::new().unwrap();
    rt.block_on(async {
        let docker: Docker = Docker::new();
        build_container(&docker).await;
        let container_id: String = create_container(&docker).await.id;
        let solr_container: Container = Container::new(&docker, &container_id);
        match solr_container.start().await {
            Ok(()) => {}
            Err(e) => {
                eprintln!("Failed to start solr container.");
                panic!("Error: {}", e)
            }
        };

        Ok(container_id)
    })
}

#[allow(dead_code)]
fn remove_container(container_id: &str) -> Result<(), Box<dyn Error>> {
    let mut rt = Runtime::new().unwrap();
    rt.block_on(async {
        let docker = Docker::new();
        let solr_container = docker.containers().get(container_id);

        let remove_options = RmContainerOptions::builder().force(true).build();

        match solr_container.remove(remove_options).await {
            Ok(()) => {}
            Err(e) => {
                eprintln!("Failed to remove solr container.");
                panic!("Error: {}", e)
            }
        }
    });

    Ok(())
}

#[allow(dead_code)]
async fn build_container(docker: &Docker) {
    let images = docker.images();
    let options = BuildOptions::builder(DOCKER_DIR).tag(IMAGE).build();

    let mut stream = images.build(&options);
    while let Some(result) = stream.next().await {
        match result {
            Ok(output) => match output {
                Object(map) => match map.get("stream") {
                    Some(Str(message)) => println!("{}", message),
                    _ => {}
                },
                _ => {}
            },
            Err(e) => panic!("Error: {}", e),
        }
    }
}

#[allow(dead_code)]
async fn create_container(docker: &Docker) -> ContainerCreateInfo {
    let containers = docker.containers();
    let options = ContainerOptions::builder(IMAGE)
        .expose(8983, "tcp", 8983)
        .expose(9983, "tcp", 9983)
        .build();

    match containers.create(&options).await {
        Ok(info) => info,
        Err(e) => panic!("Error: {}", e),
    }
}

#[allow(dead_code)]
pub fn populate_container(container_id: &str) -> Result<(), Box<dyn Error>> {
    let mut rt = Runtime::new().unwrap();
    rt.block_on(async {
        populate_sample_data(container_id).await;
    });

    Ok(())
}

#[allow(dead_code)]
async fn populate_sample_data(container_id: &str) {
    let create_films = vec!["/home/solr/bin/solr", "create_collection", "-c", "films"];
    container_exec(container_id, create_films).await;

    let make_schema = vec!["curl", "http://localhost:8983/solr/films/schema", "-X", "POST", "-H", "Content-type:application/json", "--data-binary", "{ \"add-field\" : { \"name\":\"name\", \"type\":\"text_general\", \"multiValued\":false, \"stored\":true }, \"add-field\" : { \"name\":\"initial_release_date\", \"type\":\"pdate\", \"stored\":true } }"];
    container_exec(container_id, make_schema).await;

    let populate_data_json = vec![
        "/home/solr/bin/post",
        "-c",
        "films",
        "/home/solr/example/films/films.json",
    ];
    container_exec(container_id, populate_data_json).await;

    let populate_data_xml = vec![
        "/home/solr/bin/post",
        "-c",
        "films",
        "/home/solr/example/films/films.xml",
    ];
    container_exec(container_id, populate_data_xml).await;
}

#[allow(dead_code)]
async fn container_exec(container_id: &str, commands: Vec<&str>) {
    let docker: Docker = Docker::new();

    let exec_options = ExecContainerOptions::builder()
        .cmd(commands)
        .attach_stdout(true)
        .attach_stderr(true)
        .build();

    let my_container = docker.containers().get(container_id);
    let mut my_future_stream = my_container.exec(&exec_options);
    while let Some(exec_result) = my_future_stream.next().await {
        match exec_result {
            Ok(chunk) => print_chunk(chunk),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}

#[allow(dead_code)]
fn print_chunk(chunk: TtyChunk) {
    match chunk {
        TtyChunk::StdOut(bytes) => println!("Stdout: {}", from_utf8(&bytes).unwrap()),
        TtyChunk::StdErr(bytes) => eprintln!("Stderr: {}", from_utf8(&bytes).unwrap()),
        TtyChunk::StdIn(_) => unreachable!(),
    }
}
