use anyhow::{self};
use portable_pty::{native_pty_system, CommandBuilder, PtyPair, PtySize};
use std::io::{stdin, stdout, Read, Write};
use std::thread;

fn main() -> anyhow::Result<()> {
    let pty_system = native_pty_system();
    let rows: u16 = 24;
    let cols: u16 = 80;

    let p2 = "/home/nathan/Documents/Personal/Projects/my_shell/target/debug/shell";

    let pair: PtyPair = pty_system.openpty(PtySize {
        rows,
        cols,
        pixel_width: 0,
        pixel_height: 0,
    })?;

    let cmd = CommandBuilder::new(p2);

    let mut child = pair.slave.spawn_command(cmd)?;

    let mut writer = pair.master.take_writer()?;
    let mut reader = pair.master.try_clone_reader()?;

    // FW stdin from master to slave
    thread::spawn(move || {
        let mut input = stdin();
        let mut buf = [0u8; 1024];
        loop {
            match input.read(&mut buf) {
                Ok(0) => break, // EOF on stdin
                Ok(n) => {
                    if writer.write_all(&buf[..n]).is_err() {
                        break;
                    }
                }
                Err(_) => break,
            }
        }
    });

    // FW stdout from slave to master
    thread::spawn(move || {
        let mut out = stdout();
        let mut buf = [0u8; 1024];
        loop {
            match reader.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    if out.write_all(&buf[..n]).is_err() {
                        break;
                    }
                    let _ = out.flush();
                }
                Err(_) => break,
            }
        }
    });

    child.wait()?;
    Ok(())
}
