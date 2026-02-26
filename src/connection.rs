use std::net::UdpSocket;

pub trait Connection {
    fn connect(&self);
    fn send(&self);
    fn recieve(&self);
}

pub struct UdpConnection {
    pub socket: UdpSocket,
}

impl Connection for UdpConnection {
    fn connect(&self) {
        todo!()
    }

    fn send(&self) {
        todo!()
    }

    fn recieve(&self) {
        todo!()
    }
}

pub fn init_and_send(conn: impl Connection) {
    conn.connect()
}
