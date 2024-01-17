use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::{BufRead, Read, Write};
use std::net::SocketAddr;
use std::net::{TcpListener, TcpStream};
use std::str::from_utf8;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

// fn generate_random_key(key_length: usize) -> Vec<u8> {
//     (0..key_length).map(|_| rand::thread_rng().gen()).collect()
// }

fn encrypt_decrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    if key.is_empty() {
        panic!("Key cannot be empty.");
    }
    data.iter()
        .zip(key.iter().cycle())
        .map(|(&byte, &key_byte)| byte ^ key_byte)
        .collect()
}

#[derive(Debug, PartialEq)]
enum CommandType {
    Help,
    Login,
    Register,
    Logout,
    Send,
    Inbox,
    Unknown,
    CreateLobby,
    JoinLobby,
    DeleteLobby,
    HistoryLobby,
    HistoryUser,
    Reply,
    Quit,
}
struct Commands {
    socket_addr: SocketAddr,
    command_type: CommandType,
    command_received: String,
    user: Option<String>,
    connection_manager: Option<Arc<Mutex<ConnectionManager>>>,
}

struct User {
    user: Option<String>,
}

struct ConnectionManager {
    connections: Vec<(TcpStream, Option<String>)>,
}

impl ConnectionManager {
    fn new() -> Self {
        ConnectionManager {
            connections: Vec::new(),
        }
    }
    fn add_connection(&mut self, stream: TcpStream) {
        self.connections.push((stream, None));
    }
}

impl Commands {
    fn new(
        socket_addr: std::net::SocketAddr,
        command_type: CommandType,
        command_received: String,
        user: Option<String>,
        connection_manager: Option<Arc<Mutex<ConnectionManager>>>,
    ) -> Self {
        Commands {
            socket_addr,
            command_type,
            command_received, // partea mesajului primit fara tipul comenzii specificat
            user,
            connection_manager,
        }
    }

    fn execute_command(&mut self, stream: &mut TcpStream) -> bool {
        if self.command_type == CommandType::Login {
            return self.login(stream);
        }
        if self.command_type == CommandType::Logout {
            return self.logout(stream);
        }
        if self.command_type == CommandType::JoinLobby {
            return self.joinlobby(stream);
        }
        match self.command_type {
            CommandType::Send => self.send(stream),
            CommandType::Register => self.register(stream),
            CommandType::Unknown => self.unknown(stream),
            CommandType::Help => self.help(stream),
            CommandType::Inbox => self.inbox(stream),
            CommandType::Login => return self.login(stream),
            CommandType::Logout => return self.logout(stream),
            CommandType::CreateLobby => self.createlobby(stream),
            CommandType::DeleteLobby => self.deletelobby(stream),
            CommandType::JoinLobby => return self.joinlobby(stream),
            CommandType::HistoryLobby => self.historylobby(stream),
            CommandType::HistoryUser => self.historyuser(stream),
            CommandType::Reply => self.reply(stream),
            CommandType::Quit => self.quit(stream),
        }
        true
    }
    fn quit(&mut self, stream: &mut TcpStream) {
        if let Err(er) = stream.write_all(b"\n Deconnected successfully from server!\n") {
            eprintln!("Error writing to client: {er}");
        }
    }
    fn reply(&mut self, stream: &mut TcpStream) {
        let mut part = self.command_received.split(|c| c == ' ' || c == '\0');
        let from_user = part.next().unwrap();
        part.next();
        let user = part.next();

        let mut result = String::new();
        for substring in part {
            result.push_str(substring);
            result.push(' ');
        }

        let path = format!(
            "history_of_lobbies/{}History.txt",
            self.user.clone().unwrap()
        );

        if let Ok(file) = File::open(path) {
            let reader = BufReader::new(file);
            let mut messages = Vec::new();

            for line in reader.lines() {
                if let Ok(line_content) = line {
                    messages.push(line_content);
                } else {
                    eprintln!("Error reading line");
                }
            }

            for line in messages.iter().rev() {
                if line.contains(user.unwrap_or("")) {
                    let final_messsage = format!(
                        "replied to:( {} ) =>> {} {}",
                        line.clone(),
                        from_user,
                        result
                    );
                    write_to_history_lobby(
                        self.user.clone().unwrap(),
                        final_messsage
                            .clone()
                            .trim_matches(|c| c == ' ' || c == '\0')
                            .to_string(),
                    );
                    send(
                        self.user.clone().unwrap(),
                        final_messsage.trim().to_string(),
                        self.connection_manager.clone().unwrap(),
                        self.socket_addr,
                    );
                    return;
                }
            }
            if let Err(err) = stream.write_all(b"\nUser not found!\n") {
                eprintln!("Error writing to client: {}", err);
            }
        } else if let Err(err) = stream.write_all(b"\nHistory unavailable!\n") {
            eprintln!("Error writing to client: {}", err);
        }
    }

    fn historyuser(&mut self, stream: &mut TcpStream) {
        let name_user: String = self
            .user
            .clone()
            .unwrap()
            .chars()
            .filter(|&c| c != '\0')
            .collect();
        let path = format!("history_of_conv_for_users/{}History.txt", name_user);
        if let Ok(file) = File::open(path) {
            let reader = BufReader::new(file);
            let mut ultimele50 = Vec::with_capacity(50);

            for line in reader.lines() {
                if let Ok(line_content) = line {
                    ultimele50.push(line_content);

                    if ultimele50.len() > 50 {
                        ultimele50.remove(0);
                    }
                } else {
                    eprintln!("Error reading line");
                }
            }
            if let Err(err) = stream.write_all(
                format!(
                    "      ---- History of inbox for < {} > ----\n",
                    self.user.clone().unwrap()
                )
                .as_bytes(),
            ) {
                eprintln!("Error writing to client: {}", err);
            }
            for line in ultimele50 {
                let line_bytes = line.as_bytes();
                if let Err(err) = stream.write_all(line_bytes) {
                    eprintln!("Error writing to client: {}", err);
                }
                thread::sleep(std::time::Duration::from_millis(1));
            }
            if let Err(err) = stream.write_all(b"\n      ------------------------------\n") {
                eprintln!("Error writing to client: {}", err);
            }
        } else if let Err(err) = stream.write_all(b"\nHistory unavailable!\n") {
            eprintln!("Error writing to client: {}", err);
        }
    }
    fn historylobby(&mut self, stream: &mut TcpStream) {
        let name_lobby: String = self
            .command_received
            .chars()
            .filter(|&c| c != '\0')
            .collect();
        let path = format!("history_of_lobbies/{}History.txt", name_lobby);
        if let Ok(file) = File::open(path) {
            let reader = BufReader::new(file);
            let mut ultimele50 = Vec::with_capacity(50);

            for line in reader.lines() {
                if let Ok(line_content) = line {
                    ultimele50.push(line_content);

                    if ultimele50.len() > 50 {
                        ultimele50.remove(0);
                    }
                } else {
                    eprintln!("Error reading line");
                }
            }
            if let Err(err) = stream.write_all(
                format!(
                    "    ---- History of conversation from < {} > ----\n",
                    self.command_received
                )
                .as_bytes(),
            ) {
                eprintln!("Error writing to client: {}", err);
            }
            for line in ultimele50 {
                let line_bytes = line.as_bytes();
                if let Err(err) = stream.write_all(line_bytes) {
                    eprintln!("Error writing to client: {}", err);
                }
                thread::sleep(std::time::Duration::from_millis(1));
            }
            if let Err(err) = stream.write_all(b"\n      ------------------------------\n") {
                eprintln!("Error writing to client: {}", err);
            }
        } else if let Err(err) = stream.write_all(b"\nHistory unavailable!\n") {
            eprintln!("Error writing to client: {}", err);
        }
    }
    fn deletelobby(&mut self, stream: &mut TcpStream) {
        let mut name_lobby = self.command_received.split(" \0");
        if let Some(name) = name_lobby.next() {
            let cleaned_name: String = name.chars().filter(|&c| c != '\0').collect();
            let filename = "lobbies.txt";
            let file_content = fs::read_to_string(filename).unwrap();

            let lines: Vec<&str> = file_content.lines().collect();
            let mut file = fs::File::create(filename).unwrap();
            let mut modified = false;
            for line in lines {
                if cleaned_name != line {
                    if let Err(err) = writeln!(file, "{}", line) {
                        eprintln!("Error writing on lobbies.txt: {err}");
                    }
                } else {
                    modified = true;
                }
            }
            if modified {
                if let Err(er) = stream.write_all(b"\nLobby deleted successfully!\n") {
                    eprintln!("Error writing to client: {er}");
                }
            } else if let Err(er) = stream.write_all(b"\nLobby doesn't exist!\n") {
                eprintln!("Error writing to client: {er}");
            }
        } else if let Err(er) =
            stream.write_all(b"\nName for lobby invalid! Try -delete <name_for_lobby>\n")
        {
            eprintln!("Error writing to client: {er}");
        }
    }

    fn joinlobby(&mut self, stream: &mut TcpStream) -> bool {
        let mut name_lobby = self.command_received.split(" \0");
        if let Some(name) = name_lobby.next() {
            let cleaned_name: String = name.chars().filter(|&c| c != '\0').collect();
            OpenOptions::new()
                .create(true)
                .append(true)
                .open("lobbies.txt")
                .expect("Failed to open or create lobbies.txt");

            let read = File::open("lobbies.txt").unwrap();
            let reader = BufReader::new(read.try_clone().unwrap());
            for line in reader.lines() {
                if let Ok(line) = line {
                    if line.trim() == cleaned_name {
                        return true;
                    }
                } else {
                    eprintln!("Error reading line from file");
                    return false;
                }
            }
            if let Err(er) = stream.write_all(b"\nLobby not exists! Try first -create <name_for_lobby>, then connect to a lobby\n")
            {
                eprintln!("Error writing to client: {er}");
            }
            return false;
        } else if let Err(er) =
            stream.write_all(b"\nName for lobby invalid! Try -join <name_for_lobby>\n")
        {
            eprintln!("Error writing to client: {er}");
        }

        false
    }
    fn createlobby(&mut self, stream: &mut TcpStream) {
        let mut name_lobby = self.command_received.split(" \0");
        if let Some(name) = name_lobby.next() {
            let cleaned_name: String = name.chars().filter(|&c| c != '\0').collect();
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open("lobbies.txt")
                .expect("Failed to open or create lobbies.txt");

            let read = File::open("lobbies.txt").unwrap();
            let reader = BufReader::new(read.try_clone().unwrap());
            for line in reader.lines() {
                if let Ok(line) = line {
                    if line.trim() == cleaned_name {
                        if let Err(er) = stream.write_all(b"\nLobby already exists!\n") {
                            eprintln!("Error writing to client: {er}");
                        }
                        return;
                    }
                } else {
                    eprintln!("Error reading line from file");
                    return;
                }
            }
            writeln!(file, "{}", cleaned_name).expect("Can't write in lobbies.txt");
            if let Err(er) = stream.write_all(b"\nLobby created successfully!\n") {
                eprintln!("Error writing to client: {er}");
            }
        } else if let Err(er) =
            stream.write_all(b"\nName for lobby invalid! Try -create <name_for_lobby>\n")
        {
            eprintln!("Error writing to client: {er}");
        }
    }
    fn help(&mut self, stream: &mut TcpStream) {
        if let Err(err) = stream.write_all(b"       --- commands --- \n -help \n -register <username> \n -login <usermane> \n -logout \n -send <username> <message> \n -inbox \n -join <lobby \n -create <lobby> \n -delete <lobby> \n -historylobby <lobby> \n -historyuser \n -quitlobby \n -reply <user> <message> \n -quit\n") 
        {
            println!("Error writing to stream in help: {:?}", err);
        }
    }

    fn inbox(&mut self, stream: &mut TcpStream) {
        let user = self.user.clone();
        let file_path = format!("{}Inbox.txt", user.clone().unwrap_or_default());
        println!("{}", user.unwrap_or_default());
        if File::open(file_path.clone()).is_ok() {
            let reader = std::fs::read_to_string(file_path.clone());
            let reader2 = std::fs::read_to_string(file_path.clone());
            write_to_history_user(self.user.clone().unwrap(), reader.unwrap());
            let msg_to_send = format!("\nNew messages:\n {}", reader2.unwrap().clone());
            if let Err(err) = stream.write_all(msg_to_send.as_bytes()) {
                eprintln!("Error writing to stream: {}", err);
            }
            if let Err(err) = fs::remove_file(file_path) {
                eprintln!("Error deleting file: {}", err);
            }
        } else if let Err(err) = stream.write_all(b"\nNo new messages!\n") {
            eprintln!("Error writing to stream: {}", err);
        }
    }
    fn unknown(&mut self, stream: &mut TcpStream) {
        if let Err(err) =
            stream.write_all(b"\nCommand unknown! \n Use -help to see available commands!\n")
        {
            eprintln!("Error writing to stream in unknown: {:?}", err);
        }
    }

    fn login(&mut self, stream: &mut TcpStream) -> bool {
        let mut user = self.command_received.split(" \0");
        if let Some(username) = user.next() {
            println!("Attempting login for user: {}", username);
            let cleaned_username: String = username.chars().filter(|&c| c != '\0').collect();
            // Deschide fișierul în modul de citire
            let file = match File::open("users.txt") {
                Ok(file) => file,
                Err(err) => {
                    eprintln!("Failed to open users.txt: {}", err);
                    return false;
                }
            };

            let reader = std::io::BufReader::new(file);
            if find_user_in_connected_users(cleaned_username.clone()) {
                if let Err(er) =
                    stream.write_all(b" Login failed! \n User connected on another system!")
                {
                    eprintln!("Error writing to client: {er}");
                }
                return false;
            }
            for line in reader.lines() {
                match line {
                    Ok(line_content) => {
                        for user in line_content.split(',') {
                            if user == cleaned_username {
                                println!("Login successfully!!");
                                if let Err(er) = stream.write_all(format!("\nLogin successfully!\n You have {} unread messages! Tap -inbox to see them.\n", count_nr_messages(user)).as_bytes()) {
                                    eprintln!("Failed to send response to the client: {er}");
                                }
                                if let Err(err) = add_new_login(self.socket_addr, cleaned_username)
                                {
                                    eprintln!("Error adding new login: {}", err);
                                }
                                return true;
                            }
                        }
                    }
                    Err(err) => {
                        eprintln!("Error reading line: {}", err);
                        return false;
                    }
                }
            }
            eprintln!("Invalid username for login");
            if stream
                .write_all(b" Login failed! \n User not specified or not registered!")
                .is_err()
            {
                eprintln!("Failed to send response to the client");
            };
        }
        false
    }

    fn register(&mut self, stream: &mut TcpStream) {
        let mut user = self.command_received.split(" \0");
        if let Some(username) = user.next() {
            let trimmed_username = username.trim_matches(|c| c == ' ' || c == '\0');
            println!("{:?} {}", trimmed_username, trimmed_username.len());
            if (!trimmed_username.is_empty()
                && trimmed_username.chars().all(|c| c.is_alphanumeric()))
                && find_user_in_users(trimmed_username.to_string())
            {
                let mut file = match std::fs::OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open("users.txt")
                {
                    Ok(file) => file,
                    Err(err) => {
                        eprintln!("Failed to open/create users.txt: {}", err);
                        if stream.write_all(b"register_failed!").is_err() {
                            eprintln!("Error writing to stream in register");
                        }
                        return;
                    }
                };

                let cleaned_username: String = trimmed_username
                    .chars()
                    .take_while(|&c| c != '\0')
                    .collect();
                match file.write_all(cleaned_username.as_bytes()) {
                    Ok(_) => match file.write_all(b"\n") {
                        Ok(_) => {
                            println!("User registered successfully!");
                            if stream.write_all(b"User registered successfully!!").is_err() {
                                println!("Failed to send response to the client");
                            }
                        }
                        Err(err) => eprintln!("Failed to write newline: {}", err),
                    },
                    Err(err) => {
                        eprintln!("Failed to write to users.txt: {}", err);
                        if let Err(write_err) =
                            stream.write_all(b"User registration_failed! \n Can't register to DB!")
                        {
                            eprintln!("Error writing to stream in register: {:?}", write_err);
                        }
                    }
                }

                drop(file);
            } else if stream.write_all(b"User registration failed! \n Invalid username or name for user already used! \n Retry with another username!").is_err() {}
        } else if stream
            .write_all(
                b"User registration_failed! \n User not specified \n Use -register <username>",
            )
            .is_err()
        {
        }
    }
    fn logout(&mut self, stream: &mut TcpStream) -> bool {
        println!("Logout successfully!!");
        if stream.write_all(b"Logout successfully!!").is_err() {
            println!("Failed to send response to the client");
            return false;
        }
        if let Some(user) = self.user.as_ref() {
            if let Err(err) = remove_login(self.socket_addr, user.clone()) {
                eprintln!("Error at removing a user from the list: {:?}", err);
            }
        }
        true
    }
    fn send(&mut self, stream: &mut TcpStream) {
        match &self.user {
            Some(_) => {
                let user_for_send = self.command_received.clone();
                let mut user = user_for_send.split(' ');
                if let Some(username) = user.next() {
                    if find_user_in_users(username.to_string()) {
                        if let Err(er) = stream.write_all(b"\n User not registered! \n") {
                            eprintln!("Error writing to client: {er}");
                        }
                        return;
                    }
                    if username != self.user.clone().unwrap() {
                        let cale = format!("{}Inbox.txt", username);
                        let mut file = match std::fs::OpenOptions::new()
                            .append(true)
                            .create(true)
                            .open(cale)
                        {
                            Ok(file) => file,
                            Err(_) => {
                                if stream.write_all(b"Error writing on file!").is_err() {
                                    eprintln!("Error writing to client");
                                }
                                return;
                            }
                        };
                        let subtext = if let Some(space_index) = self.command_received.find(' ') {
                            self.command_received[space_index + 1..].to_string()
                        } else {
                            self.command_received.clone()
                        };
                        let cleaned_text: String = subtext.chars().filter(|&c| c != '\0').collect();

                        if let Err(err) = file.write_all(
                            format!("[From {}]: {}", self.user.clone().unwrap(), cleaned_text)
                                .as_bytes(),
                        ) {
                            eprintln!("Failed to write: {}", err);
                            if let Err(err) = stream.write_all(b"Send message failed!") {
                                eprintln!("Error writing to stream: {:?}", err);
                            }
                            return;
                        }

                        if let Err(err) = file.write_all(b"\n") {
                            eprintln!("Failed to write newline: {}", err);
                        }
                        if let Err(er) = stream.write_all(
                            format!("\nMessage sent successfully to < {} >!", username).as_bytes(),
                        ) {
                            eprintln!("Error writting message to client: {er}");
                        }
                    } else if let Err(er) = stream.write_all(
                        b"\n You can't send yourself a message!\n Send a message to another user!",
                    ) {
                        eprintln!("Error writting message to client: {er}");
                    }
                }
            }
            None => {
                if let Err(err) = stream.write_all(
                    b"User not connected! \n First try to login to send messages for other users\n",
                ) {
                    eprintln!("Not connected: {:?}", err);
                }
            }
        }
    }
}

fn add_new_client(socket_addr: SocketAddr) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(false)
        .open("clienti_conectati.txt")?;

    let client_info = format!("{:?}", socket_addr);
    writeln!(file, "{}", client_info)?;

    Ok(())
}
fn delete_client(socket_addr: std::net::SocketAddr) -> std::io::Result<()> {
    let filename = "clienti_conectati.txt";
    let file_content = fs::read_to_string(filename)?;

    let lines: Vec<&str> = file_content.lines().collect();
    let mut file = fs::File::create(filename)?;

    for line in lines {
        if let Ok(parsed_addr) = line.parse::<std::net::SocketAddr>() {
            if parsed_addr != socket_addr {
                writeln!(file, "{}", line)?;
            }
        } else {
            writeln!(file, "{}", line)?;
        }
    }
    Ok(())
}
fn add_new_login(socket_addr: SocketAddr, user: String) -> std::io::Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("clienti_conectati.txt")?;
    let reader = BufReader::new(&file);

    let mut linii_actualizate = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if line.trim() == socket_addr.to_string() {
            linii_actualizate.push(format!("{} - {}", line, user));
        } else {
            linii_actualizate.push(line);
        }
    }

    let mut file = File::create("clienti_conectati.txt")?;

    for line in linii_actualizate {
        writeln!(file, "{}", line)?;
    }
    Ok(())
}
fn remove_login(socket_addr: SocketAddr, user: String) -> std::io::Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("clienti_conectati.txt")?;
    let reader = BufReader::new(&file);

    let mut linii_actualizate = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let trimmed_line = line.trim();

        if trimmed_line.starts_with(&format!("{} - {}", socket_addr, user)) {
            linii_actualizate.push(format!("{}", socket_addr));
        } else {
            linii_actualizate.push(line);
        }
    }
    let mut file = File::create("clienti_conectati.txt")?;
    for line in linii_actualizate {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}
fn count_nr_messages(user: &str) -> i32 {
    let file_path = format!("{}Inbox.txt", user);

    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => return 0,
    };

    let reader = BufReader::new(file);

    reader.lines().count() as i32
}

fn find_user_in_users(user: String) -> bool {
    let file = File::open("users.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(line) = line {
            if line.trim() == user {
                return false;
            }
        } else {
            eprintln!("Error reading line from file");
            return false;
        }
    }

    true
}
fn find_user_in_connected_users(user: String) -> bool {
    let file = File::open("clienti_conectati.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(line) = line {
            if line.contains(user.as_str()) {
                return true;
            }
        } else {
            eprintln!("Error reading line from file");
            return false;
        }
    }
    false
}
fn connect_to_a_lobby(
    name: String,
    socket_addr: SocketAddr,
    connection_manager: Arc<Mutex<ConnectionManager>>,
) -> bool {
    let mut connection_manager = connection_manager.lock().unwrap();

    for connection in &mut connection_manager.connections {
        let (stream, lobby) = connection;

        if stream.peer_addr().unwrap() == socket_addr && lobby.is_none() {
            *lobby = Some(name.clone());
            return true;
        }
    }
    false
}
fn send(
    name_lobby: String,
    message_to_send: String,
    connection_manager: Arc<Mutex<ConnectionManager>>,
    socket_addr: SocketAddr,
) {
    let connection_manager = connection_manager.lock().unwrap();
    for (stream, lobby) in &connection_manager.connections {
        if let Some(lobby_name) = lobby {
            if name_lobby == *lobby_name && socket_addr != stream.peer_addr().unwrap() {
                let cleaned_message: String =
                    message_to_send.chars().filter(|&c| c != '\0').collect();
                if let Err(err) = stream
                    .try_clone()
                    .unwrap()
                    .write_all(cleaned_message.as_bytes())
                {
                    eprintln!("Error writing to client: {err}");
                }
            }
        }
    }
}
fn remove_user_from_lobby(
    client_address: SocketAddr,
    connection_manager: Arc<Mutex<ConnectionManager>>,
) -> bool {
    let mut connection_manager = connection_manager.lock().unwrap();

    for connection in &mut connection_manager.connections {
        let (addr, lobby) = connection;
        if addr
            .peer_addr()
            .unwrap()
            .to_string()
            .contains(&client_address.to_string())
        {
            *lobby = None;
            return true;
        }
    }
    false
}
fn write_to_history_lobby(lobby: String, text: String) {
    let path = format!("history_of_lobbies/{}History.txt", lobby);
    let file = OpenOptions::new().create(true).append(true).open(path);

    if let Err(er) = writeln!(file.unwrap(), "{}", text) {
        eprintln!("Error writing to file for history: {er}");
    }
}
fn write_to_history_user(user: String, text: String) {
    let path = format!("history_of_conv_for_users/{}History.txt", user);
    let file = OpenOptions::new().create(true).append(true).open(path);

    if let Err(er) = writeln!(file.unwrap(), "{}", text) {
        eprintln!("Error writing to file for history: {er}");
    }
}
fn handle_connection(
    mut stream: TcpStream,
    socket_addr: std::net::SocketAddr,
    connection_manager: Arc<Mutex<ConnectionManager>>,
) {
    let my_key: [u8; 30] = [
        245, 44, 154, 236, 202, 228, 72, 138, 13, 89, 221, 96, 6, 228, 241, 17, 100, 147, 7, 91,
        192, 15, 168, 238, 44, 58, 106, 209, 155, 162,
    ];

    let mut buffer = [0; 1024];
    let mut connected = false;
    let mut user_connected = User { user: None };
    let mut connected_to_a_lobby = false;
    let mut lobby: Option<String> = None;

    //let key = generate_random_key(30);
    loop {
        let _ = connected;
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    let _ = remove_login(socket_addr, user_connected.user.unwrap_or_default()); // !!!
                    let _ = delete_client(socket_addr); // !!!
                    println!("Client disconnected");
                    break;
                }
                let msg = encrypt_decrypt(buffer.as_slice(), &my_key);
                let msg = String::from_utf8_lossy(&msg[..bytes_read]).to_string();
                let msg_clone = msg.clone();
                println!("Received: {}, {}", bytes_read, msg);
                let mut parts = msg.split(|c| c == ' ' || c == '\0');
                if let Some(command_case) = parts.next() {
                    let command_type = match command_case {
                        "-register" => CommandType::Register,
                        "-login" => CommandType::Login,
                        "-help" => CommandType::Help,
                        "-logout" => CommandType::Logout,
                        "-send" => CommandType::Send,
                        "-inbox" => CommandType::Inbox,
                        "-create" => CommandType::CreateLobby,
                        "-delete" => CommandType::DeleteLobby,
                        "-join" => CommandType::JoinLobby,
                        "-historylobby" => CommandType::HistoryLobby,
                        "-historyuser" => CommandType::HistoryUser,
                        "-reply" => CommandType::Reply,
                        "-quit" => CommandType::Quit,
                        _ => CommandType::Unknown,
                    };

                    let rest_of_message = parts.next().unwrap().to_string();
                    let subtext = if let Some(space_index) = msg_clone.find(' ') {
                        msg_clone[space_index + 1..].to_string()
                    } else {
                        msg_clone.clone()
                    };
                    let rest_of_message_clone = rest_of_message.clone();
                    match command_type {
                        CommandType::Help => {
                            let mut command =
                                Commands::new(socket_addr, command_type, subtext, None, None);
                            command.execute_command(&mut stream);
                        }
                        CommandType::Quit => {
                            let mut command =
                                Commands::new(socket_addr, command_type, subtext, None, None);
                            command.execute_command(&mut stream);
                            let _ =
                                remove_login(socket_addr, user_connected.user.unwrap_or_default()); // !!!
                            let _ = delete_client(socket_addr); // !!!
                            println!("Client disconnected");
                            break;
                        }
                        CommandType::Inbox => {
                            let mut command = Commands::new(
                                socket_addr,
                                command_type,
                                subtext,
                                Some(user_connected.user.clone().unwrap_or_default()),
                                None,
                            );
                            command.execute_command(&mut stream);
                        }
                        CommandType::HistoryUser => {
                            let mut command = Commands::new(
                                socket_addr,
                                command_type,
                                subtext,
                                user_connected.user.clone(),
                                None,
                            );
                            command.execute_command(&mut stream);
                        }
                        CommandType::Logout => {
                            let mut command = Commands::new(
                                socket_addr,
                                command_type,
                                subtext,
                                Some(user_connected.user.clone().unwrap_or_default()),
                                None,
                            );
                            if command.execute_command(&mut stream) {
                                user_connected.user = None;
                                connected = false;
                            }
                        }
                        CommandType::Send => {
                            let mut command: Commands = Commands::new(
                                socket_addr,
                                command_type,
                                subtext,
                                user_connected.user.clone(),
                                None,
                            );
                            command.execute_command(&mut stream);
                        }
                        CommandType::HistoryLobby => {
                            let mut command: Commands = Commands::new(
                                socket_addr,
                                command_type,
                                subtext,
                                user_connected.user.clone(),
                                None,
                            );
                            command.execute_command(&mut stream);
                        }
                        CommandType::CreateLobby => {
                            let mut command: Commands = Commands::new(
                                socket_addr,
                                command_type,
                                subtext,
                                user_connected.user.clone(),
                                None,
                            );
                            if connected {
                                if command.execute_command(&mut stream) {}
                            } else if let Err(er) = stream.write_all(b"\nUser not connected\n ") {
                                eprintln!("Error writing to client: {er}");
                            }
                        }
                        CommandType::DeleteLobby => {
                            let mut command: Commands = Commands::new(
                                socket_addr,
                                command_type,
                                subtext,
                                user_connected.user.clone(),
                                None,
                            );
                            if connected {
                                command.execute_command(&mut stream);
                            } else if let Err(er) = stream.write_all(b"\nUser not connected\n ") {
                                eprintln!("Error writing to client: {er}");
                            }
                        }
                        CommandType::JoinLobby => {
                            let mut command: Commands = Commands::new(
                                socket_addr,
                                command_type,
                                subtext,
                                user_connected.user.clone(),
                                None,
                            );
                            if connected {
                                if command.execute_command(&mut stream) {
                                    let client_address = stream.peer_addr().unwrap();
                                    if connect_to_a_lobby(
                                        rest_of_message.clone(),
                                        client_address,
                                        connection_manager.clone(),
                                    ) {
                                        connected_to_a_lobby = true;
                                        lobby = Some(rest_of_message.clone());
                                        if let Err(er) = stream.write_all(b"\n--- CHAT GROUP ---\n For quit this session, use -quitlobby\n") {
                                            eprintln!("Error writing to client: {er}");
                                        }
                                    } else if let Err(er) = stream.write_all(
                                        b"\nYou are in a current lobby! Try -exit lobby\n ",
                                    ) {
                                        eprintln!("Error writing to client: {er}");
                                    }
                                }
                            } else if let Err(er) = stream.write_all(b"\nUser not connected\n ") {
                                eprintln!("Error writing to client: {er}");
                            }
                        }
                        CommandType::Login => {
                            let mut command = Commands::new(
                                socket_addr,
                                command_type,
                                subtext,
                                Some(rest_of_message_clone),
                                None,
                            );
                            if command.execute_command(&mut stream) {
                                user_connected.user = Some(rest_of_message.clone());
                                connected = true;
                            }
                        }
                        _ => {
                            let mut command =
                                Commands::new(socket_addr, command_type, subtext, None, None);
                            command.execute_command(&mut stream);
                        }
                    }
                    while connected_to_a_lobby {
                        match stream.read(&mut buffer) {
                            Ok(bytes_read) => {
                                if bytes_read == 0 {
                                    let _ = remove_login(
                                        socket_addr,
                                        user_connected.user.clone().unwrap_or_default(),
                                    );
                                    let _ = delete_client(socket_addr); // !!!
                                    remove_user_from_lobby(socket_addr, connection_manager.clone());
                                    println!("Client disconnected");
                                    break;
                                }
                                let command = format!(
                                    "[{}]: {}",
                                    user_connected.user.clone().unwrap(),
                                    from_utf8(&buffer).unwrap()
                                );
                                let message_to_send = command.clone();
                                let mut part = command.split(|c| c == ' ' || c == '\0');
                                part.next();
                                let comm = part.next().unwrap();
                                if comm == "-quitlobby" {
                                    remove_user_from_lobby(
                                        stream.peer_addr().unwrap(),
                                        connection_manager.clone(),
                                    );
                                    connected_to_a_lobby = false;
                                    if let Err(err) = stream.write_all(b"\nDisconnected successfully from lobby!\n Continue with -help to see available commands!\n"){
                                        eprintln!("Error writing to client: {err}");
                                    }
                                    lobby = None;
                                    break;
                                } else if comm == "-reply" {
                                    let mut command: Commands = Commands::new(
                                        socket_addr,
                                        CommandType::Reply,
                                        message_to_send.clone(),
                                        lobby.clone(),
                                        Some(connection_manager.clone()),
                                    );
                                    if connected {
                                        command.execute_command(&mut stream);
                                    } else if let Err(er) =
                                        stream.write_all(b"\nUser not connected\n ")
                                    {
                                        eprintln!("Error writing to client: {er}");
                                    }
                                } else {
                                    write_to_history_lobby(
                                        rest_of_message.clone(),
                                        message_to_send
                                            .clone()
                                            .trim_matches(|c| c == ' ' || c == '\0')
                                            .to_string(),
                                    );
                                    send(
                                        rest_of_message.clone(),
                                        message_to_send,
                                        connection_manager.clone(),
                                        socket_addr,
                                    );
                                }
                            }
                            Err(err) => {
                                println!("Error reading from the client: {}", err);
                                break;
                            }
                        }
                    }
                }
            }
            Err(err) => {
                println!("Error reading from the client: {}", err);
                break;
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:2908").expect("Failed to bind to address");
    println!("Server listening on port 2908");
    match OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("clienti_conectati.txt")
    {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error at creating file: {}", e);
        }
    }
    let connection_manager = Arc::new(Mutex::new(ConnectionManager::new()));
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let client_address = stream.peer_addr().unwrap();
                let connection_manager_clone = Arc::clone(&connection_manager);

                match add_new_client(client_address) {
                    Ok(()) => {}
                    Err(e) => eprintln!("Error at adding client {:?}", e),
                }

                connection_manager_clone
                    .lock()
                    .unwrap()
                    .add_connection(stream.try_clone().unwrap());
                thread::spawn(move || {
                    handle_connection(
                        stream.try_clone().unwrap(),
                        client_address,
                        connection_manager_clone,
                    );
                });
            }
            Err(e) => {
                println!("Error accepting connection: {}", e);
            }
        }
    }
}
