use std::str::FromStr;

use tokio::{
    io::{self},
    net::TcpListener,
};

use crate::common::{read_command, read_data, send_data, Cmd};

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

    let listener = TcpListener::bind(format!("{}:{}", bind, port)).await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            info!("Connection esatblished: {}", socket.peer_addr().unwrap());

            let command_str = match read_command(&mut socket).await {
                Ok(v) => v,
                Err(e) => {
                    error!("Error reading line: {}", e);
                    return;
                }
            };

            match Cmd::from_str(&command_str) {
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
                Err(e) => error!("Client command error: {}: '{}'", e, command_str),
            };
        });
        info!("Closed connection");
    }
}
