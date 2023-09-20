use tokio::net::TcpStream;

use std::time::Instant;

use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

extern crate log;
extern crate pretty_env_logger;

pub async fn client(host: &str, port: usize, data: usize) -> io::Result<()> {
    info!(
        r"
      - _            _   
     | (_)          | |  
  ___| |_  ___ _ __ | |_ 
 / __| | |/ _ \ '_ \| __|
| (__| | |  __/ | | | |_ 
 \___|_|_|\___|_| |_|\__|"
    );

    let socket = TcpStream::connect(format!("{}:{}", host, port))
        .await
        .unwrap();
    let (mut rd, mut wr) = io::split(socket);

    let download_cmd_str = format!("Download {}\n", data);
    let upload_cmd_str = format!("Upload {}\n", data);

    tokio::spawn(async move {
        info!("Sending: Hello");
        wr.write_all(download_cmd_str.as_bytes()).await?;
        Ok::<_, io::Error>(())
    });

    let mut buf = vec![0; 1024];

    let start = Instant::now();

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

    let elapsed = start.elapsed();

    info!(
        "Read {} mbytes in {}s ({}ms)",
        data,
        elapsed.as_secs(),
        elapsed.as_millis()
    );

    info!(
        "Speed: {} mbit",
        calculate_mbit_s(elapsed.as_secs_f64(), data as f64)
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
