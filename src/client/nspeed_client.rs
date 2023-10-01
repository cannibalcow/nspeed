use crate::common::nspeed_common::{send_command, SpeedTestReport};
use crate::common::SpeedTestResult;
use crate::common::{send_data, SpeedTest};
use std::time::Instant;
use tokio::io::{self};
use tokio::net::TcpStream;

extern crate log;
extern crate pretty_env_logger;

pub async fn client(host: &str, port: usize, size_mb: usize, loops: usize) -> io::Result<()> {
    println!(
        r"
 ▐ ▄ .▄▄ ·  ▄▄▄·▄▄▄ .▄▄▄ .·▄▄▄▄       ▄▄· ▄▄▌  ▪  ▄▄▄ . ▐ ▄ ▄▄▄▄▄
•█▌▐█▐█ ▀. ▐█ ▄█▀▄.▀·▀▄.▀·██▪ ██     ▐█ ▌▪██•  ██ ▀▄.▀·•█▌▐█•██  
▐█▐▐▌▄▀▀▀█▄ ██▀·▐▀▀▪▄▐▀▀▪▄▐█· ▐█▌    ██ ▄▄██▪  ▐█·▐▀▀▪▄▐█▐▐▌ ▐█.▪
██▐█▌▐█▄▪▐█▐█▪·•▐█▄▄▌▐█▄▄▌██. ██     ▐███▌▐█▌▐▌▐█▌▐█▄▄▌██▐█▌ ▐█▌·
▀▀ █▪ ▀▀▀▀ .▀    ▀▀▀  ▀▀▀ ▀▀▀▀▀•     ·▀▀▀ .▀▀▀ ▀▀▀ ▀▀▀ ▀▀ █▪ ▀▀▀ 
"
    );

    for test_it in 0..loops {
        info!("Test iteration: {}/{}", test_it + 1, loops);
        let up_result = upload_test(host, port, size_mb).await?;
        let down_result = downlod_test(host, port, size_mb).await?;

        println!("{}", down_result.to_string());
        println!("{}", up_result.to_string());
    }

    Ok(())
}

async fn upload_test(host: &str, port: usize, size_mb: usize) -> io::Result<SpeedTestResult> {
    info!("Starting upload speed test. Sending {} mb", size_mb);
    let mut socket = TcpStream::connect(format!("{}:{}", host, port))
        .await
        .unwrap();

    let cmd = SpeedTest::Upload(size_mb);

    send_command(&mut socket, &cmd).await?;

    // todo: kanske vara lite smart här och få ett svar från servern

    let upload_timer = Instant::now();

    send_data(&mut socket, size_mb).await?;

    Ok(SpeedTestResult {
        duration: upload_timer.elapsed(),
        speed_test: cmd,
    })
}

async fn downlod_test(host: &str, port: usize, data: usize) -> io::Result<SpeedTestResult> {
    info!("Starting download speed test. Reading {} mb", data);

    let mut socket = TcpStream::connect(format!("{}:{}", host, port))
        .await
        .unwrap();

    let download_start = Instant::now();
    let cmd = SpeedTest::Download(data);
    send_command(&mut socket, &cmd).await?;

    crate::common::read_data(&mut socket).await?;

    Ok(SpeedTestResult {
        duration: download_start.elapsed(),
        speed_test: cmd,
    })
}
