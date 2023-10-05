use std::time::{Duration, Instant};

use crate::cli::args::OutputFormat;
use crate::common::nspeed_common::send_command;
use crate::common::{calculate_mbits, send_data, NetworkSpeedTestResult, SpeedTest, TestResult};
use chrono::Utc;
use tokio::fs::File;
use tokio::io::{self, AsyncWriteExt};
use tokio::net::TcpStream;

extern crate log;
extern crate pretty_env_logger;

// Todo: take cmd instead
pub async fn client(
    host: &str,
    port: usize,
    size_mb: usize,
    iterations: usize,
    format: OutputFormat,
    output: Option<String>,
) -> io::Result<()> {
    println!(
        r"
 ▐ ▄ .▄▄ ·  ▄▄▄·▄▄▄ .▄▄▄ .·▄▄▄▄       ▄▄· ▄▄▌  ▪  ▄▄▄ . ▐ ▄ ▄▄▄▄▄
•█▌▐█▐█ ▀. ▐█ ▄█▀▄.▀·▀▄.▀·██▪ ██     ▐█ ▌▪██•  ██ ▀▄.▀·•█▌▐█•██  
▐█▐▐▌▄▀▀▀█▄ ██▀·▐▀▀▪▄▐▀▀▪▄▐█· ▐█▌    ██ ▄▄██▪  ▐█·▐▀▀▪▄▐█▐▐▌ ▐█.▪
██▐█▌▐█▄▪▐█▐█▪·•▐█▄▄▌▐█▄▄▌██. ██     ▐███▌▐█▌▐▌▐█▌▐█▄▄▌██▐█▌ ▐█▌·
▀▀ █▪ ▀▀▀▀ .▀    ▀▀▀  ▀▀▀ ▀▀▀▀▀•     ·▀▀▀ .▀▀▀ ▀▀▀ ▀▀▀ ▀▀ █▪ ▀▀▀ 
"
    );

    let mut nrs = NetworkSpeedTestResult {
        iterations,
        data_size: size_mb,
        result: vec![],
        date: Utc::now(),
        average_duration_ms: 0.0,
    };

    for test_it in 0..iterations {
        info!("Test iteration: {}/{}", test_it + 1, iterations);

        let mut result = TestResult::new(Duration::new(0, 0), 0.0, 0.0);

        let timer = Instant::now();

        let up_duration = upload_test(host, port, size_mb).await?;
        result.up_speed = calculate_mbits(up_duration, size_mb);

        let down_duration = downlod_test(host, port, size_mb).await?;
        result.down_speed = calculate_mbits(down_duration, size_mb);

        result.duration = timer.elapsed();

        info!("{}", result.to_string());

        nrs.result.push(result);
    }

    nrs.calc_average();

    let output_str = match format {
        OutputFormat::Console => {
            let console_str = nrs.to_string();
            if iterations > 1 {
                info!("{}", console_str);
            }
            console_str
        }
        OutputFormat::Json => {
            let json_str =
                serde_json::to_string_pretty(&nrs).unwrap_or(String::from("could not create json"));
            info!("{}", json_str);
            json_str
        }
    };

    if let Some(path) = output {
        write_to_file(path, output_str).await?
    }

    Ok(())
}

async fn write_to_file(path: String, output: String) -> io::Result<()> {
    let mut file = File::create(path).await?;
    file.write_all(output.as_bytes()).await?;
    file.flush().await?;
    Ok(())
}

async fn upload_test(host: &str, port: usize, size_mb: usize) -> io::Result<Duration> {
    info!("Starting upload speed test. Sending {} mb", size_mb);
    let mut socket = TcpStream::connect(format!("{}:{}", host, port)).await?;

    let cmd = SpeedTest::Upload(size_mb);

    send_command(&mut socket, &cmd).await?;

    // todo: kanske vara lite smart här och få ett svar från servern

    let upload_timer = Instant::now();

    send_data(&mut socket, size_mb).await?;

    Ok(upload_timer.elapsed())
}

async fn downlod_test(host: &str, port: usize, data: usize) -> io::Result<Duration> {
    info!("Starting download speed test. Reading {} mb", data);

    let mut socket = TcpStream::connect(format!("{}:{}", host, port))
        .await
        .unwrap();

    let download_start = Instant::now();
    let cmd = SpeedTest::Download(data);
    send_command(&mut socket, &cmd).await?;

    crate::common::read_data(&mut socket).await?;

    Ok(download_start.elapsed())
}
