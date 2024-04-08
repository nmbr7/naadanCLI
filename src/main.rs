use std::{
    io::{self, Read, Write},
    net::{SocketAddr, TcpStream},
};

fn main() {
    // cli banner
    println!("-----------------------+");
    println!("naadan cli");
    println!("version: 0.1.0 \n");
    println!("-----------------------+\n");

    let mut new_prompt = true;
    let mut query: String = String::new();
    let mut buf: String = String::new();

    let addrs = [SocketAddr::from(([127, 0, 0, 1], 2222))];

    // prompt
    loop {
        buf.clear();
        if new_prompt {
            query.clear();
            print!("naadan-# ");
            io::stdout().flush().unwrap();
        }

        io::stdin().read_line(&mut buf).unwrap();

        if new_prompt && buf == "exit" {
            break;
        }

        if buf.ends_with("\\\n") {
            query.push_str(&buf.strip_suffix("\\\n").unwrap());
            query = query.trim().to_string();
            new_prompt = false;
        } else {
            query.push_str(&buf.strip_suffix("\n").unwrap());
            query = query.trim().to_string();

            // println!("Query: {:?}", query);
            if query.len() == 0 {
                continue;
            }

            new_prompt = true;

            let mut stream: TcpStream;
            if let Ok(l_stream) = TcpStream::connect(&addrs[..]) {
                stream = l_stream;
                //println!("Connected to the server!");
            } else {
                println!("Couldn't connect to server...");
                return;
            }

            let output_buf = query_db(&mut query, &mut stream);
            if output_buf.len() > 0 {
                println!("--------------------- Query result ---------------------");
                println!("{}", output_buf);
                println!("--------------------------------------------------------");
            }
        }
    }
}

fn query_db(query: &mut String, stream: &mut TcpStream) -> String {
    query.push_str("\nEOF\n");
    stream.write(query.as_bytes()).unwrap();
    stream.flush().unwrap();

    let mut output_buf = String::new();
    stream.read_to_string(&mut output_buf).unwrap();
    output_buf
}
