use std::str::{from_utf8, FromStr};

use tokio::{
    io::{self, AsyncReadExt},
    net::TcpListener,
};

use crate::common::{read_data, send_data, Cmd};

pub async fn server(bind: &str, port: usize) -> io::Result<()> {
    println!(
        r"
 ▐ ▄ .▄▄ ·  ▄▄▄·▄▄▄ .▄▄▄ .·▄▄▄▄      .▄▄ · ▄▄▄ .▄▄▄   ▌ ▐·▄▄▄ .▄▄▄  
•█▌▐█▐█ ▀. ▐█ ▄█▀▄.▀·▀▄.▀·██▪ ██     ▐█ ▀. ▀▄.▀·▀▄ █·▪█·█▌▀▄.▀·▀▄ █·
▐█▐▐▌▄▀▀▀█▄ ██▀·▐▀▀▪▄▐▀▀▪▄▐█· ▐█▌    ▄▀▀▀█▄▐▀▀▪▄▐▀▀▄ ▐█▐█•▐▀▀▪▄▐▀▀▄ 
██▐█▌▐█▄▪▐█▐█▪·•▐█▄▄▌▐█▄▄▌██. ██     ▐█▄▪▐█▐█▄▄▌▐█•█▌ ███ ▐█▄▄▌▐█•█▌
▀▀ █▪ ▀▀▀▀ .▀    ▀▀▀  ▀▀▀ ▀▀▀▀▀•      ▀▀▀▀  ▀▀▀ .▀  ▀. ▀   ▀▀▀ .▀  ▀"
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

            let query = from_utf8(&buf).unwrap();

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
