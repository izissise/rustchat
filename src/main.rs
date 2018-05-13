extern crate tokio;


use tokio::io;
use tokio::executor::thread_pool::ThreadPool;
use tokio::prelude::*;

struct Server {
    lst_socks: Vec<tokio::net::TcpListener>,
    clients: Vec<tokio::net::TcpStream>,
}

fn main() {
    // Create a thread pool with default configuration values
    let thread_pool = ThreadPool::new();

    let listen_addr = "0.0.0.0:2435".parse().unwrap();
    let mut serv = Server{
        lst_socks: Vec::new(),
        clients: Vec::new(),
    };
    serv.lst_socks.push(tokio::net::TcpListener::bind(&listen_addr).unwrap());

    for lst_sock in serv.lst_socks {
        thread_pool.spawn(lst_sock.incoming().for_each(|socket| {
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
        }));
    }

    // Gracefully shutdown the threadpool
    thread_pool.shutdown().wait().unwrap();
}
