use std::{fmt::Display, str::FromStr, time::Duration};

use chrono::{DateTime, Utc};

use serde::Serialize;
use tokio::{
    io::{self, AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
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

pub fn calculate_mbits(duration: Duration, size_mb: usize) -> f64 {
    let bits = (size_mb as f64 * 1000.0 * 1000.0) * 8.0;
    (bits / duration.as_secs_f64() / 1000.0 / 1000.0).floor()
}

pub async fn send_command(socket: &mut TcpStream, cmd: &SpeedTest) -> Result<(), io::Error> {
    socket.write_all(cmd.to_str().as_bytes()).await
}

pub trait SpeedTestReport {
    fn to_string(&self) -> String;
}
// Todo: Yeah.. rewrite this to more generic result
pub struct SpeedTestResultOld {
    pub duration: Duration,
    pub speed_test: SpeedTest,
}

impl SpeedTestReport for SpeedTestResultOld {
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
            calculate_mbits(self.duration, size)
        );

        format!("{}\n{}", p1, p2)
    }
}

pub async fn read_command(stream: &mut TcpStream) -> io::Result<String> {
    let mut reader = BufReader::new(stream);
    let mut line = String::new();
    reader.read_line(&mut line).await?;
    Ok(line)
}

/**
*{
  "iterations": num, // number of iterations.. Don't know if I should use this.. cound result instead
  "data_size": num, // number mb data used in test
  "result" [
    {
      ts: datetime,
      down_speed: num, // always give in bytes/s ?
      up_speed: num,
    }
  ]
}
*/

#[derive(Serialize, Debug)]
pub struct TestResult {
    pub ts: DateTime<Utc>,
    pub down_speed: f64,
    pub up_speed: f64,
}

impl TestResult {
    pub fn new(down_speed: f64, up_speed: f64) -> Self {
        let ts = Utc::now();
        Self {
            ts,
            down_speed,
            up_speed,
        }
    }
}

// Todo name it to something smurt...
#[derive(Serialize, Debug)]
pub struct NetworkSpeedTestResult {
    pub iterations: usize,
    pub data_size: usize,
    pub result: Vec<TestResult>,
    pub date: DateTime<Utc>,
}

impl ToString for TestResult {
    fn to_string(&self) -> String {
        format!(
            "Date: {} Upload: {} mbps Download: {} mbps",
            self.ts, self.up_speed, self.down_speed
        )
    }
}

impl ToString for NetworkSpeedTestResult {
    fn to_string(&self) -> String {
        let meta = format!(
            "Test iterations: {}\nData size: {}",
            self.iterations, self.data_size
        );

        let rows: String = self
            .result
            .iter()
            .map(|r| r.to_string())
            .collect::<Vec<String>>()
            .join("\n");

        format!("{}\n---\n{}\n\n", meta, rows)
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use chrono::Utc;

    use crate::common::{calculate_mbits, TestResult};

    use super::NetworkSpeedTestResult;

    #[test]
    fn calc() {
        assert_eq!(calculate_mbits(Duration::new(8, 0), 100), 100.0);
        assert_eq!(calculate_mbits(Duration::new(260, 0), 3251), 100.0);
        assert_eq!(calculate_mbits(Duration::new(1, 0), 1024), 8192.0);
    }

    #[test]
    fn result_json() {
        let r1 = TestResult {
            up_speed: 123.0,
            down_speed: 321.0,
            ts: Utc::now(),
        };
        let res = vec![r1];

        let nsr = NetworkSpeedTestResult {
            iterations: 2,
            data_size: 102,
            result: res,
            date: Utc::now(),
        };
        let json = serde_json::to_string_pretty(&nsr).unwrap();

        println!("{}", json);
    }
}
