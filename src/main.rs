use std::{path::Path, process::Command, env};
use dll_syringe::{Syringe, process::OwnedProcess};
use std::{os::windows::prelude::OsStrExt, iter::once, ptr::null};
use windows_sys::Win32::{UI::WindowsAndMessaging::{FindWindowW, GetWindowRect}, Foundation::{HWND, RECT}};
use std::ffi::OsStr;

//Args:
//  [1]: Path to Battleborn exe
//  [2]: Path to ReBorn dll
fn main() {
    let args: Vec<String> = env::args().collect();

    let pid: u32 = Command::new(&args[1]).spawn().unwrap().id();

    let battleborn_process = OwnedProcess::from_pid(pid).unwrap();

    let mut can_break = false;

    /**
     * Scuffed as hell but here goes
     * We need to block until the resolution changes to the splash screen (160 x 28), then to a different resolution
     * We do this as if we inject too early, our injected process dies on engine startup
     */
    loop{
        let name: Vec<u16> = OsStr::new("Battleborn (64-bit, DX11)").encode_wide().chain(once(0)).collect();
                let ID = unsafe { FindWindowW(null(), name.as_ptr()) };
        let mut rect = RECT { left: 0, top: 0, right: 0, bottom: 0 };
        if ID != 0 && unsafe { GetWindowRect(ID, &mut rect) } != 0 {
            if(rect.right - rect.left == 160 && rect.bottom - rect.top == 28){
                can_break = true;
            }
            
            if(rect.right - rect.left != 160 && rect.bottom - rect.top != 28 && can_break){
                break;
            }
        }
    }

    let mut syringe = Syringe::for_process(battleborn_process);

    let _ = syringe.inject(&args[2]);
}