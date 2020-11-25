mod protos;
use protos::arithmetic::SumTask;
use protos::arithmetic::DiffTask;
use protos::arithmetic::ArithmeticTask;
use protos::arithmetic::ArithmeticResponse;
use protos::arithmetic::ArithmeticTask_oneof_subtask;

use protobuf::parse_from_bytes;
use protobuf::Message;

use mio::{Poll, Events,Interest, Token};
use mio::event::Event;
use mio::net::UnixDatagram;

use std::path::Path;
use std::fs;

use slab::Slab;

fn main() {
    let mut poll = Poll::new().unwrap();
    let mut events = Events::with_capacity(1024);
    let mut slab = Slab::new();
    let path = Path::new("/tmp/rust-ipc.server");

    let socket = UnixDatagram::bind(path).unwrap();

    // Store socket in slab
    let token = slab.insert(socket);
    poll.registry().register(&mut slab[token], Token(token), Interest::READABLE).unwrap();

    // Cleanup file
    ctrlc::set_handler(move || {
        println!("received Ctrl+C!");
        fs::remove_file(path).expect("Error removing file");
    })
    .expect("Error setting Ctrl-C handler");

    loop {
        println!("Polling...");
        poll.poll(&mut events, None).unwrap();
        for event in &events {
            handle_event(&event, &slab);
        }
    }
}

fn handle_event(event: &Event, slab: &Slab<UnixDatagram>) {
    let socket =  &slab[usize::from(event.token())];

    // Read until attempt fails
    loop {
        if !attempt_read(&socket) {
            break;
        }
    }
}

fn attempt_read(socket: &UnixDatagram) -> bool {
    let mut buff = [0u8; 1024];
    if let Ok((size, addr)) = socket.recv_from(&mut buff) {
        let vec = buff[0..size].to_vec();
        let task = parse_from_bytes::<ArithmeticTask>(&vec).unwrap();
        let response = match task.subtask.unwrap() {
            ArithmeticTask_oneof_subtask::sum_task(task) => handle_sum_task(task),
            ArithmeticTask_oneof_subtask::diff_task(task) => handle_diff_task(task),
        };

        println!("{:?}", response);
        let mut data = response.write_to_bytes().unwrap();
        data.append(&mut[0u8].to_vec()); // Append a null to indicate end of protobuf (REALLY???)
        socket.send_to(&data, &addr.as_pathname().unwrap()).unwrap();
        return true;
    } else {
        return false;
    }
}

fn handle_sum_task(task: SumTask) -> ArithmeticResponse {
    println!("SumTask: {:?}", task);
    let mut response = ArithmeticResponse::new();
    response.set_answer(task.val1 + task.val2);
    return response;
}

fn handle_diff_task(task: DiffTask) -> ArithmeticResponse {
    println!("DiffTask: {:?}", task);
    let mut response = ArithmeticResponse::new();
    response.set_answer(task.val1 - task.val2);
    return response;
}
