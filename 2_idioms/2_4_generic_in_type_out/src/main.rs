use http::status::StatusCode;
use std::net::SocketAddr;

fn main() {
    let err = ErrorBuilder::new()
        .code("NO_USER")
        .status(404.try_into().unwrap())
        .message("User not found")
        .build();
    dbg!(&err);
}

#[derive(Debug)]
pub struct Error {
    code: String,
    status: StatusCode,
    message: String,
}

impl Default for Error {
    #[inline]
    fn default() -> Self {
        Self {
            code: "UNKNOWN".to_string(),
            status: 500.try_into().unwrap(),
            message: "Unknown error has happened.".to_string(),
        }
    }
}

pub struct ErrorBuilder(Error);

impl ErrorBuilder {
    pub fn new() -> Self {
        let error = Error::default();
        Self(error)
    }

    pub fn build(self) -> Error {
        self.0
    }

    pub fn code<S: Into<String>>(mut self, code: S) -> Self {
        self.0.code = code.into();
        self
    }

    pub fn status(mut self, status: StatusCode) -> Self {
        self.0.status = status;
        self
    }

    pub fn message<S: Into<String>>(mut self, message: S) -> Self {
        self.0.message = message.into();
        self
    }
}

#[derive(Debug, Default)]
pub struct Server(Option<SocketAddr>);

impl Server {
    pub fn bind<A: Into<SocketAddr>>(&mut self, addr: A) {
        self.0 = Some(addr.into());
    }
}

#[cfg(test)]
mod server_spec {
    use super::*;

    mod bind {
        use std::net::{IpAddr, Ipv4Addr};

        use super::*;

        #[test]
        fn sets_provided_address_to_server() {
            let mut server = Server::default();

            let socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
            server.bind(socket_addr);
            assert_eq!(format!("{}", server.0.unwrap()), "127.0.0.1:8080");

            let socket_addr = SocketAddr::new("::1".parse::<IpAddr>().unwrap(), 9911);
            server.bind(socket_addr);
            assert_eq!(format!("{}", server.0.unwrap()), "[::1]:9911");
        }
    }
}
