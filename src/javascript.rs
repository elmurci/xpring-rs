use anyhow::{bail, Error};
use fehler::throws;
use serde::Serialize;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::{Ipv4Addr, Shutdown, SocketAddrV4, TcpListener, TcpStream};
use std::path::Path;
use std::process::Command;

macro_rules! js {
    ($context:ident . $pack:ident . $func:ident ( $($argument:tt),* )) => {
        js!($container.$context.$package.$func::<()>($($argument),*))
    };
    ($context:ident . $pack:ident . $func:ident :: < $ty:tt > ( $($argument:tt),* )) => {
        {
            let result: Result<$ty, anyhow::Error> = {
                let message = JsCall {
                    func: stringify!($pack.$func).to_owned(),
                    arguments: vec![$(serde_json::to_value($argument)?),*],
                };
                let result = $context.execute(message);
                match result {
                    Ok(value) => {
                        let ty: $ty = serde_json::from_value(value)?;
                        Ok(ty)
                    }
                    Err(err) => Err(err),
                }
            };
            result
        }
    }
}

#[derive(Serialize)]
pub struct JsCall {
    pub func: String,
    pub arguments: Vec<serde_json::Value>,
}

pub struct JavaScript {
    stream: TcpStream,
}

impl JavaScript {
    #[throws(_)]
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let loopback = Ipv4Addr::new(127, 0, 0, 1);
        // Request an unused port for ipc from the system.
        let socket = SocketAddrV4::new(loopback, 0);
        let listener = TcpListener::bind(socket)?;
        let address = listener.local_addr()?;
        Command::new("node")
            .arg(path.as_ref())
            .arg(address.port().to_string())
            .spawn()?;
        Self {
            // All communication will be done over the connection of the first client.
            stream: listener.accept()?.0,
        }
    }

    #[throws(_)]
    pub fn execute(&mut self, message: JsCall) -> serde_json::Value {
        serde_json::to_writer(&mut self.stream, &message)?;
        // Write message seperator.
        self.stream.write_all(&[0x04])?;
        let mut reader = BufReader::new(&mut self.stream);
        let mut buffer = vec![];
        // Read until message seperator.
        reader.read_until(0x04, &mut buffer)?;
        // Remove message seperator from buffer.
        buffer.pop();
        if buffer.is_empty() {
            serde_json::Value::Null
        } else {
            let mut buffer = std::io::Cursor::new(buffer);
            let (ok, err): (serde_json::Value, Option<String>) =
                serde_json::from_reader(&mut buffer)?;
            if let Some(err) = err {
                bail!(Error::msg(err));
            } else {
                serde_json::from_value(ok)?
            }
        }
    }
}

impl Drop for JavaScript {
    fn drop(&mut self) {
        // By closing the connection to the only client TcpListener will be dropped as well and
        // which shuts down the server. The node.js client also shuts itself down once the
        // connection was closed by the server.
        self.stream.shutdown(Shutdown::Both).unwrap();
    }
}
