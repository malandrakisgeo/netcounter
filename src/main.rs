use std::process::{Command, Stdio};
use std::{thread, time};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use chrono::{Timelike, Utc};


fn main() {
    let mut my_hash: HashMap<String, String> = HashMap::new();
    let mut counter: u32 = 1;
    loop {
        thread::sleep(time::Duration::from_millis(1000));
        let time = Utc::now();
        let hr = time.hour();
        let mn = time.minute();
        let str = String::from(hr.to_string() + ":" + &*mn.to_string());

        let hash = run_and_add_to_hash_set();
        hash.iter().for_each(|h| {
            my_hash.insert(h.clone(), str.clone());
        });

        if counter % 10 == 0 {
            println!("Connections so far: {:}", &my_hash.keys().len());
            println!("{:?}", &my_hash);
            write_to_file(&my_hash).unwrap();
        }

        counter += 1;
    }
}

fn write_to_file(myhash: &HashMap<String, String>) -> std::io::Result<()> {
    let mut file = File::create("connections.txt")?;
    file.write_all(format!("{:?}", myhash).as_ref())?;
    Ok(())
}

fn run_and_add_to_hash_set() -> Vec<String> {
    let mut returnable_vector: Vec<String> = Vec::new();

    let netstat = Command::new("netstat")
        .arg("-atupn") //The -n is needed because DNS resolution is slow, and can make netstat hang.
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let grep = Command::new("grep") 
        .arg("Virtualbox")
        .stdin(Stdio::from(netstat.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let awk = Command::new("awk")
        .arg(" { print $5, $1 } ")
        .stdin(Stdio::from(grep.stdout.unwrap())) //Replace grep with netstat in the VM script
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let output = awk.wait_with_output().unwrap();
    let str = String::from_utf8(output.stdout).unwrap();
    let spl = str.split("\n");
    let col: Vec<_> = spl.collect();
    col.iter().for_each(|e| { //TODO: Simplify
        returnable_vector.push(String::from(*e));
    }
    );

    return returnable_vector;
}
