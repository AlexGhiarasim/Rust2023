use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
//use rand::rngs::OsRng;
//use rsa::{RSAPrivateKey, PaddingScheme, PublicKey};

fn main() {
    let mut client = TcpStream::connect("127.0.0.1:2908").expect("Failed to connect");
    client
        .set_nonblocking(true)
        .expect("Failed to set non-blocking");

    let (sender, _receiver) = mpsc::channel::<String>();
    let (connection_sender, connection_receiver) = mpsc::channel::<String>();

    // let mut key_size_buf = [0; 4];
    // loop {
    //     match client.read(&mut key_size_buf) {
    //         Ok(0) => {
    //             println!("Server disconnected");
    //             break;
    //         }
    //         Ok(n) => {
    //             if n == 4 {
    //                 break;
    //             }
    //             // Wait for more data to be available
    //             thread::sleep(Duration::from_millis(100));
    //         }
    //         Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
    //             // Wait for more data to be available
    //             thread::sleep(Duration::from_millis(100));
    //         }
    //         Err(e) => {
    //             panic!("Failed to read key size: {}", e);
    //         }
    //     }
    // }

    // let key_size = u32::from_be_bytes(key_size_buf);

    // Primim cheia privată
    // let mut private_key_bytes = vec![0; key_size as usize];
    // client.read(&mut private_key_bytes).expect("Failed to read private key");

    // // Reconstruim cheia privată
    // let private_key = RSAPrivateKey::from_pkcs1(&private_key_bytes).expect("Failed to deserialize private key");
    // let public_key: rsa::RSAPublicKey = private_key.clone().into();
    // let mut rng = OsRng;

    let mut cloned_client = client.try_clone().expect("Failed to clone client");
    let mut connected = false;
    let mut lobby = false;
    thread::spawn(move || {
        let mut buf = [0; 1024];
        loop {
            if let Ok(bytes_read) = cloned_client.read(&mut buf) {
                if bytes_read == 0 {
                    println!("Server disconnected");
                    break;
                }
                let received_msg = String::from_utf8_lossy(&buf[..bytes_read]);
                let cleaned_message: String = received_msg.chars().filter(|&c| c != '\0').collect();
                if received_msg.contains("Login successfully!") {
                    if let Err(er) = connection_sender.send(String::from("login!")) {
                        println!("Failed to send connection message: {er}");
                        break;
                    }
                } else if received_msg.contains("Logout successfully!") {
                    if let Err(er) = connection_sender.send(String::from("logout!")) {
                        println!("Failed to send connection message: {er}");
                        break;
                    }
                } else if received_msg
                    .contains("\n--- CHAT GROUP ---\n For quit this session, use -quitlobby\n")
                {
                    if let Err(er) = connection_sender.send(String::from("lobby!")) {
                        println!("Failed to send connection message: {er}");
                        break;
                    }
                }
                else if received_msg.contains("Disconnected successfully from lobby!\n Continue with -help to see available commands!")
                {
                    if let Err(er) = connection_sender.send(String::from("quitlobby!")) {
                        println!("Failed to send connection message: {er}");
                        break;
                    }
                }
                println!("{}", cleaned_message);
            }
        }
    });

    println!();
    println!("            ~~~~Bine ati venit la Offline Messenger~~~~");
    println!(" **Scrie -login <<user>> pentru logare");
    println!(" **Scrie -register <<user>> pentru inregistrare");
    println!(" **Scrie -quit pentru a inchide sesiunea");
    println!();
    loop {
        if !lobby {
            if connected {
                print!("[login]: ");
            } else {
                print!("[logout]: ");
            }
        }
        thread::sleep(Duration::from_millis(30));
        let mut buff = String::new();
        buff.reserve(50);
        io::stdout().flush().expect("Failed to flush stdout");

        io::stdin().read_line(&mut buff).expect("Eroare la citire");
        let msg = buff.trim().to_string();

        if buff.contains("-login") && connected {
            println!("\nUser already connected!\n");
            continue;
        }
        if buff.contains("-logout") && !connected {
            println!("\nUser already disconnected!\n");
            continue;
        }
        if buff.contains("-historylobby") && !connected {
            println!("\nUser not connected!\n");
            continue;
        }
        if buff.contains("-historylobby") && connected && lobby {
            println!("\nYou must quit this lobby to see history of conversation from lobby!\n");
            continue;
        }
        if sender.send(msg.clone()).is_ok() {
            let mut buff = msg.to_string().into_bytes();
            buff.resize(50, 0);
            // !!! let encrypted_text = public_key.encrypt(&mut rng, PaddingScheme::new_pkcs1v15_encrypt(), &buff).expect("Criptarea a eșuat");
            if let Err(er) = client.write_all(&buff) {
                println!("Failed to send message to server: {er}");
                break;
            }
        }
        thread::sleep(Duration::from_millis(20));
        while let Ok(message) = connection_receiver.try_recv() {
            match message.as_str() {
                "login!" => {
                    connected = true;
                }
                "logout!" => {
                    connected = false;
                }
                "lobby!" => {
                    lobby = true;
                }
                "quitlobby!" => {
                    lobby = false;
                }
                _ => {
                    println!("Received unexpected message: {}", message);
                }
            }
        }
    }
}
