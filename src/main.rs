use std::process::{Command, Stdio};
use std::{env, process, thread, time};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use chrono::{Timelike, Utc};


static mut MY_HASH: Option<HashMap<String, String>> = None;


fn main() {
    // let mut MY_HASH: HashMap<String, String> = HashMap::new();
    unsafe { MY_HASH = Some(HashMap::new()); }
    let mut counter: u32 = 1;
    loop {
        thread::sleep(time::Duration::from_millis(1000));
        let time = Utc::now();
        let hr = time.hour();
        let mn = time.minute();
        let str = String::from(hr.to_string() + ":" + &*mn.to_string());

        let hash = run_and_add_to_hash_set();
        hash.iter().for_each(|h| unsafe {
            MY_HASH.as_mut().unwrap().insert(h.clone(), str.clone());
        });

        if counter % 10 == 0 {
            unsafe {
                println!("{:?}", MY_HASH.as_ref().unwrap());
                write_to_file(MY_HASH.as_ref().unwrap()).unwrap();
            }
        }

        counter += 1;
    }
}

unsafe fn write_to_file(myhash: &HashMap<String, String>) -> std::io::Result<()> {
    let mut file = File::create("connections.txt")?;
    file.write_all(format!("{:?}", myhash).as_ref())?;
    Ok(())
}

fn run_and_add_to_hash_set() -> Vec<String> {
    let mut returnable_vector: Vec<String> = Vec::new();

    let ps_child = Command::new("netstat")
        .arg("-atupn") //The -n is needed because DNS resolution is slow, and can make netstat hang.
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let grep_child_one = Command::new("grep")
        .arg("Virtualbox")
        .stdin(Stdio::from(ps_child.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let grep_child_two = Command::new("awk")
        .arg(" { print $5, $1 } ")
        .stdin(Stdio::from(grep_child_one.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let output = grep_child_two.wait_with_output().unwrap();
    let str = String::from_utf8(output.stdout).unwrap();
    let mut spl = str.split("\n");
    let mut col: Vec<_> = spl.collect();
    col.iter().for_each(|e| { //TODO: Simplify
        returnable_vector.push(String::from(*e));
    }
    );

    return returnable_vector;
}