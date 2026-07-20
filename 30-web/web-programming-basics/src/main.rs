/*
 * Web Programming Basics
 */

use std::io::BufRead;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};
//use std::io::prelude;
use std::fs;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let listener: TcpListener = TcpListener::bind(/*addr:*/ "127.0.0.1:8000").unwrap();
    let mut active_requests/*: Arc<Mutex<i32>>*/ = Arc::new(Mutex::new(0));
    let stream/*: Result<(TcpStream, SocketAddr), ...>*/ = listener.accept();

    println!(
        "The stream {:?} \n The socket {:?}",
        stream.as_ref().unwrap().1,
        stream.as_ref().unwrap().0
    );

    //for i/*: i32*/ in 0..10 {
    //    match listener.accept() {
    //        Ok((socket/*: TcpStream*/, addr/*: SocketAddr*/)) => println!("The client info: {:?}", addr),
    //        Err(e/*: Error*/) => println!("Couldn't get client: {:?}", e),
    //    }
    //}

    for stream /*: Result<TcpStream, Error>*/ in listener.incoming() {
        let active_requests = Arc::clone(&active_requests);
        let stream: TcpStream = stream.unwrap();

        thread::spawn(move || {
            {
                let mut connection = active_requests.lock().unwrap();
                *connection += 1;
                if *connection >= 3 {
                    thread::sleep(Duration::from_secs(2));
                }
            } // Here, connection goes out of scope thereby releasing the lock.

            handle_connection(stream);

            {
                let mut connection = active_requests.lock().unwrap();
                *connection -= 1;
            }
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader: BufReader<&mut TcpStream> = BufReader::new(/*inner:*/ &mut stream);

    //let http_request = buf_reader
    //    .lines()
    //    .map(|result| result.unwrap())
    //    .take_while(|lines| !lines.is_empty())
    //    .collect::<Vec<String>>();

    //println!("Request: {:#?}", http_request);

    //let response: &str = "HTTP/1.1 200 OK\r\n\r\n";
    //stream.write(response.as_bytes()).unwrap();
    //stream.flush().unwrap(); // The flush method makes sure that all bytes reach their intented destination.

    //let status_line: &str = "HTTP/1.1 200 OK \r\n";
    //let contents = fs::read_to_string("index.html").unwrap();
    //let length: usize = contents.len();

    //let response: String = format!("{} Contents-Length: {}\r\n\r\n{}", status_line, length, contents);
    //stream.write(response.as_bytes()).unwrap();
    //stream.flush().unwrap();

    let mut request_line = buf_reader.lines().next();
    let (status_line, file_name) = match request_line.unwrap().unwrap().as_str() {
        "GET / HTTP/1.1" => (Some("HTTP/1.1 200 OK\r\n"), Some("index.html")),
        "GET /page1 HTTP/1.1" => {
            thread::sleep(Duration::from_secs(10));
            (Some("HTTP/1.1 200 OK\r\n"), Some("page1.html"))
        }
        "GET /page2 HTTP/1.1" => (Some("HTTP/1.1 200 OK\r\n"), Some("page2.html")),
        _ => (Some("HTTP/1.1 404 NOT FOUND\r\n"), Some("404.html")),
    };

    let contents = fs::read_to_string(file_name.unwrap()).unwrap();
    let length: usize = contents.len();

    let response: String = format!(
        "{} Contents-Length: {}\r\n\r\n{}",
        status_line.unwrap(),
        length,
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
