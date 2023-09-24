use tokio::net::TcpStream;

use std::time::Instant;

use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

extern crate log;
extern crate pretty_env_logger;

enum SpeedTest {
    Download(usize),
    Upload(usize),
}

impl SpeedTest {
    fn to_str(&self) -> String {
        match self {
            SpeedTest::Download(size) => format!("Download {}\n", size),
            SpeedTest::Upload(size) => format!("Upload {}\n", size),
        }
    }
}

pub async fn client(host: &str, port: usize, size_mb: usize) -> io::Result<()> {
    info!(
        r"
      - _            _   
     | (_)          | |  
  ___| |_  ___ _ __ | |_ 
 / __| | |/ _ \ '_ \| __|
| (__| | |  __/ | | | |_ 
 \___|_|_|\___|_| |_|\__|"
    );

    downlod_test(host, port, size_mb).await?;
    upload_test(host, port, size_mb).await?;

    Ok(())
}

async fn upload_test(host: &str, port: usize, size_mb: usize) -> io::Result<()> {
    let mut socket = TcpStream::connect(format!("{}:{}", host, port))
        .await
        .unwrap();

    let upload_cmd = SpeedTest::Upload(size_mb);

    socket.write_all(upload_cmd.to_str().as_bytes()).await?;

    let upload_start = Instant::now();

    let one_mb_chunk = vec![0; 1000 * 1024 * 1];

    let mut n = 0;

    while n < size_mb {
        socket.write_all(&one_mb_chunk).await?;
        n += 1;
    }

    let upload_elapsed = upload_start.elapsed();

    info!(
        "Uploaded {} mbytes in {}s ({}ms)",
        size_mb,
        upload_elapsed.as_secs(),
        upload_elapsed.as_millis()
    );

    info!(
        "Upload Speed: {} mbit",
        calculate_mbit_s(upload_elapsed.as_secs_f64(), size_mb as f64)
    );

    Ok(())
}

async fn downlod_test(host: &str, port: usize, data: usize) -> io::Result<()> {
    let socket = TcpStream::connect(format!("{}:{}", host, port))
        .await
        .unwrap();
    let (mut rd, mut wr) = io::split(socket);

    let download_cmd = SpeedTest::Download(data);

    wr.write_all(download_cmd.to_str().as_bytes()).await?;

    let mut buf = vec![0; 1024];

    let download_start = Instant::now();

    loop {
        let read_bytes = match rd.read(&mut buf).await {
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

    let download_elapsed = download_start.elapsed();

    info!(
        "Downloaded {} mbytes in {}s ({}ms)",
        data,
        download_elapsed.as_secs(),
        download_elapsed.as_millis()
    );

    info!(
        "Download Speed: {} mbit",
        calculate_mbit_s(download_elapsed.as_secs_f64(), data as f64)
    );

    Ok(())
}

fn calculate_mbit_s(duration: f64, size_mb: f64) -> f64 {
    let bits = (size_mb * 1000.0 * 1000.0) * 8.0;
    (bits / duration / 1000.0 / 1000.0).floor()
}

#[cfg(test)]
mod test {
    use crate::client::nspeed_client::calculate_mbit_s;

    #[test]
    fn calc() {
        assert_eq!(calculate_mbit_s(8.0, 100.0), 100.0);
        assert_eq!(calculate_mbit_s(260.0, 3251.0), 100.0);
        assert_eq!(calculate_mbit_s(1.0, 1024.0), 8192.0);
    }
}
