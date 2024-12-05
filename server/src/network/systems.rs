use bevy::prelude::*;
use std::io::{Read, Write};
use std::net::*;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::network::components::*;

/*   START_TCP_SERVER FUNCTION   */
///Function that handles TCP connections seperately
pub fn start_tcp_server(mut connections: &TcpResource) {
    //spawning thread to handle connections
    let tcp_listener = TcpListener::bind("127.0.0.1:8000").expect("TCP binding failed");
    println!("Server listening on 127.0.0.1:8000");

    tcp_listener
        .set_nonblocking(true)
        .expect("Nonblock toggle failed");

    //Accepting incoming connection
    for stream_result in tcp_listener.incoming() {
        match stream_result {
            Ok(stream) => {
                println!(
                    "Establishing a new TCP connection from {:?}",
                    stream.peer_addr()
                );

                // Handle the connection in a new thread
                connections.streams.lock().unwrap().add_connection(stream);
                break;
            }
            Err(e) => {}
        }
    }
}

/*   RECEIVE_UDP FUCNTION   */
/// Receives and handles UDP packets
pub fn receive_udp(socket: UdpSocket) {}

/*   HANDLE_TCP_CONNECTIONS FUNCTION   */
/// System to periodically check and handle active tcp connections
fn handle_connections(mut tcpconnections: ResMut<TcpConnections>) {
    tcpconnections.handle_connections();
}
