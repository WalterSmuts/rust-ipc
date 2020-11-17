mod protos;
use protos::arithmetic::SumTask;
use protos::arithmetic::DiffTask;
use protos::arithmetic::ArithmeticTask;
use protos::arithmetic::ArithmeticResponse;
use protos::arithmetic::ArithmeticTask_oneof_subtask;

use protobuf::parse_from_bytes;
use protobuf::Message;

use nix::sys::socket::recvmsg;
use nix::sys::socket::MsgFlags;
use nix::sys::uio::IoVec;

use mio::{Poll, Events,Interest, Token};
use mio::net::UnixDatagram;

use std::path::Path;
use std::os::unix::io::{AsRawFd};

use slab::Slab;

fn main() {
    let mut poll = Poll::new().unwrap();
    let mut events = Events::with_capacity(1024);
    let mut slab = Slab::new();

    let path = Path::new("/tmp/rust-ipc.sock");
    let my_sock = UnixDatagram::bind(path).unwrap();
    let token = slab.insert(my_sock);
    poll.registry().register(&mut slab[token], Token(token), Interest::READABLE).unwrap();

    poll.poll(&mut events, None).unwrap();
    loop {
        for event in &events {
            let socket =  &slab[usize::from(event.token())];
            let mut buff = [0u8;  2048];
            let iov = [IoVec::from_mut_slice(&mut buff[..])];
            let msg = recvmsg(socket.as_raw_fd(), &iov, None, MsgFlags::empty()).unwrap();
            let vec = buff[0..msg.bytes].to_vec();
            let task = parse_from_bytes::<ArithmeticTask>(&vec).unwrap();
            let response = match task.subtask.unwrap() {
                ArithmeticTask_oneof_subtask::sum_task(task) => handleSumTask(task),
                ArithmeticTask_oneof_subtask::diff_task(task) => handleDiffTask(task),
            };
            println!("{:?}", response);
        }
    }
}

fn handleSumTask(task :SumTask) -> ArithmeticResponse {
    let mut response = ArithmeticResponse::new();
    response.set_answer(task.val1 + task.val2);
    return response;
}

fn handleDiffTask(task :DiffTask) -> ArithmeticResponse {
    let mut response = ArithmeticResponse::new();
    response.set_answer(task.val1 - task.val2);
    return response;
}
