use OpKind::*;
use std::io::{self};

fn intepreter(op: fn(i32, i32)->i32, a:i32, b:i32) -> i32 {
    op(a,b)
}

fn add(a:i32, b:i32) -> i32 {
    a+b
}

fn sub(a:i32, b:i32) -> i32 {
    a-b
}
    
#[derive(Debug)]
enum OpKind {
    ADD,
    SUB,
    MUL,
    DIV,
    NULL,
}

#[derive(Debug)]
struct Cmd {
    num1:i32,
    num2:i32,
    op: OpKind,
}

impl Cmd {
    fn new() -> Cmd {
        Cmd { num1: 0, num2: 0, op: ADD }
    }

    fn parse(&mut self, arg1:&str, arg2:&str, arg3:&str) {
        self.num1 = arg1.parse::<i32>().unwrap();
        self.num2 = arg2.parse::<i32>().unwrap();
        self.op = match arg3.trim() {
            "+" => ADD,
            "-" => SUB,
            "*" => MUL,
            "/" => DIV,
            _   => {println!("{}", arg3); NULL},
        };

        println!("num1 = {}, num2 = {}, op = {:?}", self.num1, self.num2, self.op)
    }

    fn run(&self) -> i32 {

        println!("num1 = {}  num2 = {}", self.num1, self.num2);

        let res = match self.op {
            ADD => self.num1 + self.num2,
            SUB => self.num1 - self.num2,
            MUL => self.num1 * self.num2,
            DIV => self.num1 / self.num2,
            _   => 0 as i32
        };
        
        res
    }





}




pub fn do_cmd() -> i32 {

    let mut cmd = Cmd::new();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    println!("input line is {}", buffer);
    println!("input length is {}", buffer.len());

    //divide buffer into  arg1 arg2 op 。。。
    let pos: Vec<&str> = buffer.split(" ").collect();

    println!("===============");
    for p in pos.iter() {
        println!("{}", p);
    }
    println!("===============");

    cmd.parse(pos[0], pos[1], pos[2]);

    let result = cmd.run();

    println!("result is {}", result);
    result
}