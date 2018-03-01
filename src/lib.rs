extern crate capnp;
pub mod example_capnp {
  include!(concat!(env!("OUT_DIR"), "/example_capnp.rs"));
}

pub mod example {
  use example_capnp::point;
  use capnp::serialize_packed;

  pub fn make_point(mut storage: &mut Vec<u8>, x: f32, y: f32) -> ::std::io::Result<()> {
    let mut message = ::capnp::message::Builder::new_default();
    {
      let mut point = message.init_root::<point::Builder>();

      point.set_x(x);
      point.set_y(y);
    }
    serialize_packed::write_message(&mut storage, &message)
  }

  pub fn read_x(storage: &Vec<u8>) -> ::capnp::Result<f32> {
    let bytes: &[u8] = &storage;
    let mut reader = ::std::io::BufReader::new(bytes);
    let message_reader = try!(serialize_packed::read_message(&mut reader, ::capnp::message::ReaderOptions::new()));

    let point = try!(message_reader.get_root::<point::Reader>());

    let x = point.get_x();

    Ok(x)
  }

  pub fn read_y(storage: &Vec<u8>) -> ::capnp::Result<f32> {
    let bytes: &[u8] = &storage;
    let mut reader = ::std::io::BufReader::new(bytes);
    let message_reader = try!(serialize_packed::read_message(&mut reader, ::capnp::message::ReaderOptions::new()));

    let point = try!(message_reader.get_root::<point::Reader>());

    let y = point.get_y();

    Ok(y)
  }
}

type WasmPointer = *mut ::std::os::raw::c_void;

fn wrap_message(mut message: Vec<u8>) -> WasmPointer {
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

fn unwrap_message<T: Sized>(header: WasmPointer, f: fn(&Vec<u8>) -> T) -> T {
  // Read the pointer and length from the header.
  let slice = unsafe { Vec::from_raw_parts(header as *mut usize, 2, 2) };
  let ptr = slice[0];
  let size = slice[1];

  // We're actually borrowing the header from the caller.
  ::std::mem::forget(slice);

  // Reconstruct the vector of message bytes.
  let message = unsafe { Vec::from_raw_parts(ptr as *mut u8, size as usize, size as usize) };

  // Do something with the reconstructed message.
  let result = f(&message);

  // We're actually borrowing the message from the caller.
  ::std::mem::forget(message);

  result
}

fn drop_message(header: WasmPointer) {
  // Read the pointer and length from the header.
  let slice = unsafe { Vec::from_raw_parts(header as *mut usize, 2, 2) };
  let ptr = slice[0];
  let size = slice[1];

  // Reconstruct the vector of message bytes.
  let _message = unsafe { Vec::from_raw_parts(ptr as *mut u8, size as usize, size as usize) };
}

#[no_mangle]
pub fn make_point(x: f32, y: f32) -> WasmPointer {
  let mut message = Vec::new();

  match example::make_point(&mut message, x, y) {
    Ok(_) => (),
    Err(_) => return 0 as WasmPointer,
  }

  wrap_message(message)
}

#[no_mangle]
pub fn x(point: WasmPointer) -> f32 {
  unwrap_message(point, |message| {
    match example::read_x(message) {
      Ok(res) => res,
      Err(_) => -1.,
    }
  })
}

#[no_mangle]
pub fn y(point: WasmPointer) -> f32 {
  unwrap_message(point, |message| {
    match example::read_y(message) {
      Ok(res) => res,
      Err(_) => -1.,
    }
  })
}

#[no_mangle]
pub fn destroy_point(point: WasmPointer) {
  drop_message(point);
}

// from https://github.com/killercup/wasm-experiments
#[no_mangle]
pub fn alloc(size: usize) -> WasmPointer {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    ::std::mem::forget(buf);
    return ptr as WasmPointer;
}

#[no_mangle]
pub fn dealloc(ptr: WasmPointer, cap: usize) {
    unsafe  {
        let _buf = Vec::from_raw_parts(ptr, 0, cap);
    }
}
