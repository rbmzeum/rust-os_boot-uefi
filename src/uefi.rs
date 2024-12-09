pub type ImageHandle = u64;

#[repr(C)]
pub struct SystemTable {
    header: [u8; 24],
    firmware_vendor: u64,
    firmware_revision: u32,
    input_handle: ImageHandle,
    input: u64,
    output_handle: ImageHandle,
    pub output: *const TextOutputProtocol,
    error_handle: ImageHandle,
    error: u64,
    runtime: u64,
    boot: u64,
    no_of_entries: usize,
    config_table: u64,
}

#[repr(C)]
pub struct TextOutputProtocol {
    reset: u64,
    pub output_string: OutputString,
    test_output: u64,
    query_mode: u64,
    set_mode: u64,
    pub set_attribute: fn(*const TextOutputProtocol, attributes: usize),
    pub clear_screen: fn(*const TextOutputProtocol),
    set_cursor_position: u64,
    enable_cursor: u64,
    mode: u64,
}

pub type OutputString =
    extern "efiapi" fn(output_protocol: *const TextOutputProtocol, string: *const u16) -> Status;

pub type Status = usize;
