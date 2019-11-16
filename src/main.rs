use futures::{future::try_join,TryFutureExt};
use tokio::net::{TcpStream, TcpListener};
use tokio::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:8081".parse()?;
    let mut listener = TcpListener::bind(&addr)?;

    loop {

        if let Ok((client, sock_addr)) = listener.accept().await {

            println!("client from {}", sock_addr);
            tokio::spawn(async move {
                let addr = "127.0.0.1:8080".parse().unwrap();

                if let Ok(server) = TcpStream::connect(&addr).await {
                    let (mut cr, mut cw) = client.split();
                    let (mut sr, mut sw) = server.split();

                    let client_to_server = cr
                        .copy(&mut sw)
                        .map(|bytes|{
                            if let Ok(n) = bytes {
                                println!("Copyed: {}", n)
                            }
                            bytes
                        })
                    .map_err(|err| {
                        println!("err {} for client {}", err, sock_addr);
                    });

                    let server_to_client = sr
                        .copy(&mut cw)
                        .map(|bytes|{
                            if let Ok(n) = bytes {
                                println!("Copyed: {}", n)
                            }
                            bytes
                        })
                    .map_err(|err| {
                        println!("err {} for client {}", err, sock_addr);
                    });
                    try_join(client_to_server, server_to_client).await;
                } else {
                    println!("fail connect to server");
                }
                //let (_client_to_server, _server_to_client) = try_join(client_to_server, server_to_client).await.();
            });
        }
    }
}
