use std::{path::Path, process::Command, env};
use dll_syringe::{Syringe, process::OwnedProcess};

//Args:
//  [1]: Path to Battleborn exe
//  [2]: Path to ReBorn dll
fn main() {
    let args: Vec<String> = env::args().collect();

    let pid: u32 = Command::new(&args[1]).spawn().unwrap().id();

    let battleborn_process = OwnedProcess::from_pid(pid).unwrap();

    let mut syringe = Syringe::for_process(battleborn_process);

    syringe.inject(&args[2]);
}