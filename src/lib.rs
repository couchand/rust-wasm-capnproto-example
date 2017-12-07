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

#[no_mangle]
pub fn make_point(x: f32, y: f32) -> *mut ::std::os::raw::c_void {
  let mut vec = Vec::new();

  match example::make_point(&mut vec, x, y) {
    Ok(_) => (),
    Err(_) => return 0 as *mut ::std::os::raw::c_void,
  }

  let len = vec.len();
  let ptr = vec.as_mut_ptr();
  ::std::mem::forget(vec);

  let mut slice = vec![ptr as u32, len as u32];
  let sliceptr = slice.as_mut_ptr();
  ::std::mem::forget(slice);
  sliceptr as *mut ::std::os::raw::c_void
}

#[no_mangle]
pub fn x(point: *mut ::std::os::raw::c_void) -> f32 {
  let slice = unsafe { Vec::from_raw_parts(point as *mut u32, 2, 2) };
  let ptr = slice[0];
  let size = slice[1];
  ::std::mem::forget(slice);

  let vec = unsafe { Vec::from_raw_parts(ptr as *mut u8, size as usize, size as usize) };

  let res = match example::read_x(&vec) {
    Ok(res) => res,
    Err(_) => -1.,
  };

  ::std::mem::forget(vec);

  res
}

#[no_mangle]
pub fn y(point: *mut ::std::os::raw::c_void) -> f32 {
  let slice = unsafe { Vec::from_raw_parts(point as *mut u32, 2, 2) };
  let ptr = slice[0];
  let size = slice[1];
  ::std::mem::forget(slice);

  let vec = unsafe { Vec::from_raw_parts(ptr as *mut u8, size as usize, size as usize) };

  let res = match example::read_y(&vec) {
    Ok(res) => res,
    Err(_) => -1.,
  };

  ::std::mem::forget(vec);

  res
}

// from https://github.com/killercup/wasm-experiments
#[no_mangle]
pub fn alloc(size: usize) -> *mut ::std::os::raw::c_void {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    ::std::mem::forget(buf);
    return ptr as *mut ::std::os::raw::c_void;
}

#[no_mangle]
pub fn dealloc(ptr: *mut ::std::os::raw::c_void, cap: usize) {
    unsafe  {
        let _buf = Vec::from_raw_parts(ptr, 0, cap);
    }
}
