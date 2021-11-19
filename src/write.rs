use std::fs::File;
use std::io::Write;
use termcolor::Buffer;

/// Writes to tty from a receiving output.
/// Kinda like flush.
pub fn write_recv_out(recv_out: &mut Buffer, tty: &mut File) -> std::io::Result<()> {
    tty.write_all(recv_out.as_slice())?;
    Ok(())
}