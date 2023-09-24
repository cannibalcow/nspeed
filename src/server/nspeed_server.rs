use std::{
    fmt::Display,
    str::{self, FromStr},
};
use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

#[derive(Debug)]
enum Cmd {
    Upload(u32),
    Download(u32),
}

#[derive(Debug)]
enum CmdParserError {
    InvalidCommand(String),
    InvalidValue(std::num::ParseIntError),
    MissingCommandName,
    MissingValue,
}

impl Display for CmdParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CmdParserError::InvalidCommand(v) => write!(f, "Invalid command: {}", v),
            CmdParserError::InvalidValue(v) => write!(f, "Invalid value: {}", v),
            CmdParserError::MissingCommandName => write!(f, "Missing command name"),
            CmdParserError::MissingValue => write!(f, "Missing value"),
        }
    }
}

impl FromStr for Cmd {
    type Err = CmdParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();

        let name = match parts.next() {
            Some(v) => v,
            None => return Err(CmdParserError::MissingCommandName),
        };

        let value = match parts.next() {
            Some(v) => match v.parse::<u32>() {
                Ok(i) => i,
                Err(e) => return Err(CmdParserError::InvalidValue(e)),
            },
            None => return Err(CmdParserError::MissingValue),
        };

        match name {
            "Download" => Ok(Cmd::Download(value)),
            "Upload" => Ok(Cmd::Upload(value)),
            _ => Err(CmdParserError::InvalidCommand(String::from(name))),
        }
    }
}

pub async fn server(bind: &str, port: usize) -> io::Result<()> {
    info!(
        r"
 ___  ___ _ ____   _____ _ __ 
/ __|/ _ \ '__\ \ / / _ \ '__|
\__ \  __/ |   \ V /  __/ |   
|___/\___|_|    \_/ \___|_|"
    );

    info!("Booting up server");

    let listener = TcpListener::bind(format!("{}:{}", bind, port))
        .await
        .unwrap();

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            info!("Connection esatblished: {}", socket.peer_addr().unwrap());

            let mut buf = vec![0; 1024];

            loop {
                let read_bytes = match socket.read(&mut buf).await {
                    Ok(0) => break,
                    Ok(n) => n,
                    Err(_) => return,
                };

                if char::from(buf[read_bytes - 1]) == '\n' {
                    break;
                }
            }

            let query = str::from_utf8(&buf).unwrap();

            match Cmd::from_str(query) {
                Ok(cmd) => match cmd {
                    Cmd::Upload(size) => {
                        info!("Client Upload: {}", size);
                        match read_data(&mut socket).await {
                            Ok(_) => (),
                            Err(e) => error!("{}", e),
                        }
                    }
                    Cmd::Download(size) => {
                        info!("Client Download: {}", size);
                        match send_data(&mut socket, size).await {
                            Ok(_) => (),
                            Err(e) => error!("{}", e),
                        }
                    }
                },
                Err(e) => error!("Client cmd error: {}: '{}'", e, query),
            };
        });
    }
}

async fn read_data(socket: &mut TcpStream) -> io::Result<()> {
    info!("Reading data:");
    let mut buf = vec![0; 1024];
    loop {
        let read_bytes = match socket.read(&mut buf).await {
            Ok(n) => n,
            Err(k) => return Err(k),
        };

        if read_bytes == 0 {
            break;
        }

        if char::from(buf[read_bytes - 1]) == '\n' {
            break;
        }
    }
    Ok(())
}

async fn send_data(socket: &mut TcpStream, size: u32) -> io::Result<()> {
    let chunk = vec![0; 1000 * 1024 * 1];
    let mut n = 0;
    while n < size {
        match socket.write(&chunk).await {
            Ok(_) => (),
            Err(e) => panic!("helvete {:?}", e),
        };
        n += 1;
    }
    socket.write(b"\n").await.unwrap();
    Ok(())
}
