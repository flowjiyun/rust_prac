use std::{
    fs,
    thread,
    time::Duration,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let socket = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in socket.incoming() {
        let stream = stream.unwrap();

        // stream 소유권을 handle_connection 함수로 넘김
        handle_connection(stream);
    }
}

// 소유권을 넘겨받고 함수 내부에서는 muttable하게 사용하겠다
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let request_line = buf_reader.lines().next().unwrap().unwrap();

    println!("Request line : {request_line}");

    let (status_line, file) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP1.1 200 OK", "hello.html")
        },
        _ => ("HTTP1.1 404 NOT FOUND", "error.html")
    };
    let contents = fs::read_to_string(file).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
