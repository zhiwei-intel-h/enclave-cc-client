// Copyright (c) 2020 Ant Financial
//
// SPDX-License-Identifier: Apache-2.0
//

mod protocols;

use protocols::r#async::{image, image_ttrpc};
use ttrpc::context::{self, Context};
use ttrpc::r#async::Client;
use clap::{Arg, App};

pub const SOCK_ADDR: &str = "tcp://0.0.0.0:7788";

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut sockaddr = SOCK_ADDR;

    let matches = App::new("enclave-agent")
                    .author("Enclave-cc Team")
                    .arg(Arg::with_name("connect")
                        .short("c")
                        .long("connect")
                        .value_name("sockaddr")
                        .help("connect to server,  tcp://ip_addr:port")
                        .takes_value(true)
                    )
                    .get_matches();

    if matches.is_present("connect") {
        sockaddr = matches.value_of("connect").unwrap();
    }

    println!("Client connect to {}", sockaddr);
    let c = Client::connect(sockaddr).unwrap();
    let ic = image_ttrpc::ImageClient::new(c);

    let mut tic = ic.clone();

    let now = std::time::Instant::now();

    let t1 = tokio::spawn(async move {
        let mut req = image::PullImageRequest::new();
        println!(
            "Green Thread 1 - {} started: {:?}",
            "image.pull_image()",
            now.elapsed(),
        );

        //let image_tag = "docker.io/huaijin20191223/scratch-base:v1.8";
        //let image_tag = "docker.io/arronwang/busybox_kbs_encrypted";
        //let image_tag = "docker.io/huaijin20191223/ubuntu-base-enc";
        let image_tag = "docker.io/huaijin20191223/ubuntu-base:HelloWorld_v1.1";
        //let image_tag = "docker.io/ubuntu";
        //let image_tag = "docker.io/nginx";
        //let image_tag = "docker.io/httpd";
        //let image_tag = "docker.io/busybox";
        //let image_tag = "docker.io/centos";
        req.set_image(image_tag.to_string());
        //req.set_container_id(cid.to_string());
        println!(
            "Green Thread 1 - {} -> {:?} ended: {:?}",
            "pull_image",
            tic.pull_image(default_ctx(), &req)
                .await,
            now.elapsed(),
        );
        println!("pull_image - {}", image_tag);
    });

    let _ = tokio::join!(t1);
}

fn default_ctx() -> Context {
    let mut ctx = context::with_timeout(0);
    ctx.add("key-1".to_string(), "value-1-1".to_string());
    ctx.add("key-1".to_string(), "value-1-2".to_string());
    ctx.set("key-2".to_string(), vec!["value-2".to_string()]);

    ctx
}

