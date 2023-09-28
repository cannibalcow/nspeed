use std::{fmt::Display, str::FromStr, time::Duration};

use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

pub async fn read_data(socket: &mut TcpStream) -> io::Result<()> {
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

pub async fn send_data(socket: &mut TcpStream, size: usize) -> io::Result<()> {
    let chunk = vec![0; 1000 * 1024];
    let mut n = 0;
    while n < size {
        match socket.write(&chunk).await {
            Ok(_) => (),
            Err(e) => error!("helvete {:?}", e),
        };
        n += 1;
    }
    let _ = socket.write(b"\n").await.unwrap();
    Ok(())
}

#[derive(Debug)]
pub enum Cmd {
    Upload(usize),
    Download(usize),
}

#[derive(Debug)]
pub enum CmdParserError {
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
            Some(v) => match v.parse::<usize>() {
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

pub enum SpeedTest {
    Download(usize),
    Upload(usize),
}

impl SpeedTest {
    pub fn to_str(&self) -> String {
        match self {
            SpeedTest::Download(size) => format!("Download {}\n", size),
            SpeedTest::Upload(size) => format!("Upload {}\n", size),
        }
    }
}

pub fn calculate_mbits(duration: f64, size_mb: f64) -> f64 {
    let bits = (size_mb * 1000.0 * 1000.0) * 8.0;
    (bits / duration / 1000.0 / 1000.0).floor()
}

pub async fn send_command(socket: &mut TcpStream, cmd: &SpeedTest) -> Result<(), io::Error> {
    socket.write_all(cmd.to_str().as_bytes()).await
}

pub trait SpeedTestReport {
    fn to_string(&self) -> String;
}

pub struct SpeedTestResult {
    pub duration: Duration,
    pub speed_test: SpeedTest,
}

impl SpeedTestReport for SpeedTestResult {
    fn to_string(&self) -> String {
        let (name, size) = match self.speed_test {
            SpeedTest::Download(st) => ("Download", st),
            SpeedTest::Upload(st) => ("Upload", st),
        };

        let p1 = format!(
            "{}ed {} Mb in {} s ({} Mb/s) ({} ms)",
            name,
            size,
            self.duration.as_secs(),
            size as f64 / self.duration.as_secs() as f64,
            self.duration.as_millis()
        );

        let p2 = format!(
            "{} Speed: {} mbit",
            name,
            calculate_mbits(self.duration.as_secs_f64(), size as f64)
        );

        format!("{}\n{}", p1, p2)
    }
}

#[cfg(test)]
mod test {
    use crate::common::calculate_mbits;

    #[test]
    fn calc() {
        assert_eq!(calculate_mbits(8.0, 100.0), 100.0);
        assert_eq!(calculate_mbits(260.0, 3251.0), 100.0);
        assert_eq!(calculate_mbits(1.0, 1024.0), 8192.0);
    }
}
