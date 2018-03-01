pub type WasmPointer = *mut ::std::os::raw::c_void;

/// Wrap a Cap'n'Proto message in a transparent header, giving
/// ownership to the JS caller.
///
/// To avoid a memory leak, make sure to call `use_message` at some
/// point in the future!
pub fn wrap_message(mut message: Vec<u8>) -> WasmPointer {
  // Find the length and raw pointer of the message.
  let size = message.len();
  let ptr = message.as_mut_ptr();

  // We're actually moving the message to the caller.
  ::std::mem::forget(message);

  // Construct the header.
  let mut slice = vec![ptr as usize, size as usize];
  let header = slice.as_mut_ptr();

  // We're actually moving the header to the caller.
  ::std::mem::forget(slice);

  header as WasmPointer
}

/// Unwrap a transparent header to use the Cap'n'Proto message
/// contained within it.  Borrows the header from the JS caller.
pub fn unwrap_message(header: WasmPointer) -> ::std::mem::ManuallyDrop<Vec<u8>> {
  // Read the pointer and length from the header.
  let slice = unsafe { Vec::from_raw_parts(header as *mut usize, 2, 2) };
  let ptr = slice[0];
  let size = slice[1];

  // We're actually borrowing the header from the caller.
  ::std::mem::forget(slice);

  // Reconstruct the vector of message bytes.
  let message = unsafe { Vec::from_raw_parts(ptr as *mut u8, size as usize, size as usize) };

  // We're actually borrowing the message from the caller.
  ::std::mem::ManuallyDrop::new(message)
}


/// Unwrap a transparent header to use the Cap'n'Proto message
/// contained within it.  Takes ownership from the JS caller.
pub fn use_message(header: WasmPointer) -> Vec<u8> {
  // Read the pointer and length from the header.
  let slice = unsafe { Vec::from_raw_parts(header as *mut usize, 2, 2) };
  let ptr = slice[0];
  let size = slice[1];

  // Reconstruct the vector of message bytes.
  unsafe { Vec::from_raw_parts(ptr as *mut u8, size as usize, size as usize) }
}
