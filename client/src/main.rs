use std::io::{self, ErrorKind, Read, Write};
use std::ip::{ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

const LOCAL: &str = "127.0..0.1:6000";
const MSG_SIZE: usize = 32;

fn main() {
    let server = TcpStream::connect(LOCAL).expect("Failed to connect");
}
