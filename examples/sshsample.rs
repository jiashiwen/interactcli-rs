// use ssh2::Session;
// use std::{io::Read, net::TcpStream};

// pub fn main() {
//     // Connect to the local SSH server
//     let tcp = TcpStream::connect("114.67.127.196:22").unwrap();
//     let mut sess = Session::new().unwrap();
//     sess.set_tcp_stream(tcp);
//     sess.handshake().unwrap();

//     sess.userauth_password("root", "Git785230").unwrap();

//     let mut channel = sess.channel_session().unwrap();

//     // channel
//     //     .exec("/root/interactcli-rs/target/release/interactcli-rs")
//     //     .unwrap();
//     let result = channel.exec("/root/interactcli-rs/target/release/interactcli-rs");
//     println!("{:?}", result);
//     let mut s = String::new();

//     channel.read_to_string(&mut s).unwrap();

//     println!("s is: {}", s);
//     channel.wait_close();
//     println!("{:?}", channel.exit_status());
// }

use core::time;
use futures::Future;
use std::io::Read;
use std::rc::Rc;
use std::sync::Arc;
use std::thread::{self, sleep};
use thrussh::server::{Auth, Session};
use thrussh::*;
use thrussh_keys::*;
use tokio::fs::remove_dir_all;
use tokio::task;

struct Client {}

impl client::Handler for Client {
    type Error = anyhow::Error;
    type FutureUnit = futures::future::Ready<Result<(Self, client::Session), anyhow::Error>>;
    type FutureBool = futures::future::Ready<Result<(Self, bool), anyhow::Error>>;

    fn finished_bool(self, b: bool) -> Self::FutureBool {
        futures::future::ready(Ok((self, b)))
    }
    fn finished(self, session: client::Session) -> Self::FutureUnit {
        futures::future::ready(Ok((self, session)))
    }
    fn check_server_key(self, server_public_key: &key::PublicKey) -> Self::FutureBool {
        println!("check_server_key: {:?}", server_public_key);
        self.finished_bool(true)
    }
    fn channel_open_confirmation(
        self,
        channel: ChannelId,
        max_packet_size: u32,
        window_size: u32,
        session: client::Session,
    ) -> Self::FutureUnit {
        println!("channel_open_confirmation: {:?}", channel);
        self.finished(session)
    }
    fn data(self, channel: ChannelId, data: &[u8], session: client::Session) -> Self::FutureUnit {
        println!(
            "data on channel {:?}: {:?}",
            channel,
            std::str::from_utf8(data)
        );
        self.finished(session)
    }
}

#[tokio::main]
async fn main() {
    let config = thrussh::client::Config::default();
    let config = Arc::new(config);
    let sh = Client {};

    // let key = thrussh_keys::key::KeyPair::generate_ed25519().unwrap();
    // let mut agent = thrussh_keys::agent::client::AgentClient::connect_env()
    //     .await
    //     .unwrap();
    // agent.add_identity(&key, &[]).await.unwrap();
    let mut session = thrussh::client::connect(config, "114.67.127.196:22", sh)
        .await
        .unwrap();
    let result = session
        .authenticate_password::<String, String>("root".to_string(), "Git785230".to_string())
        .await;

    // let mut channel = session.channel_open_session().await.unwrap();

    let channel = session.channel_open_session().await;

    match channel {
        Ok(c) => {
            println!("start chammel");
            let mut boxc = Box::new(c);
            for _ in 0..100 {
                thread::sleep(time::Duration::from_secs(1));
                boxc.as_mut().data(&b"Hello, world!"[..]).await.unwrap();
            }

            if let Some(msg) = boxc.as_mut().wait().await {
                println!("{:?}", msg)
            }
        }
        Err(e) => {
            println!("{}", e);
        }
    }

    // if let Some(msg) = channel.wait().await {
    //     println!("{:?}", msg)
    // }
    // for _ in 0..100 {
    //     thread::sleep(time::Duration::from_secs(1));
    //     channel.data(&b"Hello, world!"[..]).await.unwrap();
    // }

    // match channel {
    //     Ok(mut c) => {
    //         c.exec::<String>(
    //             false,
    //             "/root/interactcli-rs/target/release/interactcli-rs".to_string(),
    //         )
    //         .await
    //         .unwrap();
    //     }
    //     Err(e) => println!("error: {}", e),
    // }

    // channel
    //     .exec::<String>(
    //         false,
    //         "/root/interactcli-rs/target/release/interactcli-rs".to_string(),
    //     )
    //     .await
    //     .unwrap();
    // // channel.data(&b"Hello, world!"[..]).await.unwrap();
    // match channel.wait().await {
    //     Some(msg) => println!("{:?}", msg),
    //     None => {}
    // }
}
