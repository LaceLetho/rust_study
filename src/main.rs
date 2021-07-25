use std::io::prelude::*;
use std::io::Result;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    //监听本地的7878端口的TCP连接请求，失败则panic
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    //轮询处理没个链接
    for stream in listener.incoming() {
        //获取TcpStream，失败则panic
        let mut stream = stream.unwrap();

        //调用处理TcpStream的函数
        match handle_connection(&stream){
            Ok(x) => {
                //本地打印
                println!("get user input: {}", x);
                //返回给用户
                stream.write(x.as_bytes()).unwrap();
                stream.flush().unwrap();
            }
            //打印错误
            _ => {println!("invalid input")}
        }
    }
}

//定一个处理TcpStream的函数，
fn handle_connection(mut stream: &TcpStream) -> Result<String> {
    //申请一个用于接收用户输入的数组
    let mut buffer = [0; 1024];

    //读取用户输入存放于buffer中，失败则panic
    stream.read(&mut buffer)?;

    //把字节转换为字符
    let user_input = String::from_utf8_lossy(&buffer[..]);

    //输入长度符合要求就返回
    match user_input.len() {
        0..=1024 => {return Ok(user_input.to_string())}
        _ => {panic!("stop here")}
    }
}