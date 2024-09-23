use std::{
    io::{self, Read, Write},
    net::{SocketAddr, TcpStream},
    str::from_utf8,
};

fn main() {
    // cli banner
    println!("+*****************+\n");
    println!("- naadan cli 0.1.0\n");
    println!("+*****************+\n\n");

    let mut new_prompt = true;
    let mut query: String = String::new();
    let mut buf: String = String::new();

    let addrs = [SocketAddr::from(([127, 0, 0, 1], 2222))];

    let mut stream: TcpStream;
    if let Ok(l_stream) = TcpStream::connect(&addrs[..]) {
        stream = l_stream;
        //println!("Connected to the server!");
    } else {
        println!("Couldn't connect to server...");
        return;
    }

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
        } else if buf.strip_suffix("\n").unwrap() == "clear" {
            print!("\x1b[2J\x1b[H");
            continue;
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

    let mut out_str = String::new();
    let mut buffer = [0; 100];
    loop {
        let bytes = stream.read(&mut buffer).unwrap();

        let slice = from_utf8(&buffer[..(bytes)]).unwrap();
        out_str.push_str(slice);
        if bytes < 100 {
            break;
        }
    }

    out_str
}
