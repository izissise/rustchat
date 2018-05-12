extern crate tokio;

use tokio::io;
use tokio::prelude::*;

fn main() {
    let listen_addr = "0.0.0.0:2435".parse().unwrap();
    let listener = tokio::net::TcpListener::bind(&listen_addr).unwrap();

    let server = listener.incoming().for_each(|socket| {
        println!("New connection: {:?}", socket.peer_addr().unwrap());
        let message = "Ok\n";
        let peer_write = io::write_all(socket, message)
            .then(|res| {
//                 println!("Wrote {:?} bytes", message.len());
                println!("Wrote {:?}", res.is_ok());
                Ok(())
            });
        tokio::spawn(peer_write);
        Ok(())
    }).map_err(|err| {
        println!("accept() error => {:?}", err);
    });
    tokio::run(server);
}
