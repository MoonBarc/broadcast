use std::fs::File;
use std::io::Write;
use termcolor::Buffer;

/// Writes to tty from a receiving output.
/// Kinda like flush.
pub fn write_recv_out(recv_out: &mut Buffer, tty: &mut File) {
    if let Err(_) = tty.write_all(recv_out.as_slice()) {
        println!("debug: failed to write!")
    }
}