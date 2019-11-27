/// To run this benchmark:
///
/// > DENO_BUILD_MODE=release ./tools/build.py && \
///   ./target/release/deno_core_http_bench --multi-thread
extern crate deno;
extern crate futures;
extern crate libc;
extern crate tokio;

#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

use deno::*;
use futures::future::poll_fn;
use futures::future::Future;
use futures::future::FutureExt;
use std::env;
use std::io::Error;
use std::io::ErrorKind;
use std::net::SocketAddr;
use std::pin::Pin;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWrite;
use tokio::sync::Mutex;
use tokio::sync::MutexGuard;

static LOGGER: Logger = Logger;

struct Logger;

impl log::Log for Logger {
  fn enabled(&self, metadata: &log::Metadata) -> bool {
    metadata.level() <= log::max_level()
  }
  fn log(&self, record: &log::Record) {
    if self.enabled(record.metadata()) {
      println!("{} - {}", record.level(), record.args());
    }
  }
  fn flush(&self) {}
}

#[derive(Clone, Debug, PartialEq)]
pub struct Record {
  pub promise_id: i32,
  pub arg: i32,
  pub result: i32,
}

impl Into<Buf> for Record {
  fn into(self) -> Buf {
    let buf32 = vec![self.promise_id, self.arg, self.result].into_boxed_slice();
    let ptr = Box::into_raw(buf32) as *mut [u8; 3 * 4];
    unsafe { Box::from_raw(ptr) }
  }
}

impl From<&[u8]> for Record {
  fn from(s: &[u8]) -> Record {
    #[allow(clippy::cast_ptr_alignment)]
    let ptr = s.as_ptr() as *const i32;
    let ints = unsafe { std::slice::from_raw_parts(ptr, 3) };
    Record {
      promise_id: ints[0],
      arg: ints[1],
      result: ints[2],
    }
  }
}

impl From<Buf> for Record {
  fn from(buf: Buf) -> Record {
    assert_eq!(buf.len(), 3 * 4);
    #[allow(clippy::cast_ptr_alignment)]
    let ptr = Box::into_raw(buf) as *mut [i32; 3];
    let ints: Box<[i32]> = unsafe { Box::from_raw(ptr) };
    assert_eq!(ints.len(), 3);
    Record {
      promise_id: ints[0],
      arg: ints[1],
      result: ints[2],
    }
  }
}

#[test]
fn test_record_from() {
  let r = Record {
    promise_id: 1,
    arg: 3,
    result: 4,
  };
  let expected = r.clone();
  let buf: Buf = r.into();
  #[cfg(target_endian = "little")]
  assert_eq!(
    buf,
    vec![1u8, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0].into_boxed_slice()
  );
  let actual = Record::from(buf);
  assert_eq!(actual, expected);
  // TODO test From<&[u8]> for Record
}

pub type HttpOp = dyn Future<Output = Result<i32, std::io::Error>> + Send;

pub type HttpOpHandler =
  fn(record: Record, zero_copy_buf: Option<PinnedBuf>) -> Pin<Box<HttpOp>>;

fn http_op(
  handler: HttpOpHandler,
) -> impl Fn(&[u8], Option<PinnedBuf>) -> CoreOp {
  move |control: &[u8], zero_copy_buf: Option<PinnedBuf>| -> CoreOp {
    let record = Record::from(control);
    let mut record_a = record.clone();
    let is_sync = record.promise_id == 0;
    let op = handler(record.clone(), zero_copy_buf);

    let fut = async move {
      match op.await {
        Ok(result) => record_a.result = result,
        Err(err) => {
          eprintln!("unexpected err {}", err);
          record_a.result = -1;
        }
      };
      Ok(record_a.into())
    };

    if is_sync {
      Op::Sync(futures::executor::block_on(fut).unwrap())
    } else {
      Op::Async(fut.boxed())
    }
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();
  // NOTE: `--help` arg will display V8 help and exit
  let args = deno::v8_set_flags(args);

  log::set_logger(&LOGGER).unwrap();
  log::set_max_level(if args.iter().any(|a| a == "-D") {
    log::LevelFilter::Debug
  } else {
    log::LevelFilter::Warn
  });

  let js_source = include_str!("http_bench.js");

  let startup_data = StartupData::Script(Script {
    source: js_source,
    filename: "http_bench.js",
  });

  let isolate = deno::Isolate::new(startup_data, false);
  isolate.register_op("listen", http_op(op_listen));
  isolate.register_op("accept", http_op(op_accept));
  isolate.register_op("read", http_op(op_read));
  isolate.register_op("write", http_op(op_write));
  isolate.register_op("close", http_op(op_close));

  let multi_thread = args.iter().any(|a| a == "--multi-thread");

  let mut builder = tokio::runtime::Builder::new();
  let builder = if multi_thread {
    println!("multi-thread");
    builder.threaded_scheduler()
  } else {
    println!("single-thread");
    builder.basic_scheduler()
  };

  let mut runtime = builder
    .enable_io()
    .build()
    .expect("Unable to create tokio runtime");
  let result = runtime.block_on(isolate.boxed());
  js_check(result);
}

pub fn bad_resource() -> Error {
  Error::new(ErrorKind::NotFound, "bad resource id")
}

struct TcpListener(tokio::net::TcpListener);

impl Resource for TcpListener {}

struct TcpStream(tokio::net::TcpStream);

impl Resource for TcpStream {}

lazy_static! {
  static ref RESOURCE_TABLE: Mutex<ResourceTable> =
    Mutex::new(ResourceTable::default());
}

async fn lock_resource_table<'a>() -> MutexGuard<'a, ResourceTable> {
  RESOURCE_TABLE.lock().await
}

fn op_accept(
  record: Record,
  _zero_copy_buf: Option<PinnedBuf>,
) -> Pin<Box<HttpOp>> {
  let rid = record.arg as u32;
  debug!("accept {}", rid);

  let fut = async move {
    let mut table = lock_resource_table().await;
    let listener =
      table.get_mut::<TcpListener>(rid).ok_or_else(bad_resource)?;
    let listener = &mut listener.0;
    let (stream, addr) = listener.accept().await?;
    debug!("accept success {}", addr);
    let mut table = lock_resource_table().await;
    let rid = table.add("tcpStream", Box::new(TcpStream(stream)));
    Ok(rid as i32)
  };

  fut.boxed()
}

fn op_listen(
  _record: Record,
  _zero_copy_buf: Option<PinnedBuf>,
) -> Pin<Box<HttpOp>> {
  debug!("listen");
  let fut = async {
    let addr = "127.0.0.1:4544".parse::<SocketAddr>().unwrap();
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    let mut table = lock_resource_table().await;
    let rid = table.add("tcpListener", Box::new(TcpListener(listener)));
    Ok(rid as i32)
  };

  fut.boxed()
}

fn op_close(
  record: Record,
  _zero_copy_buf: Option<PinnedBuf>,
) -> Pin<Box<HttpOp>> {
  debug!("close");
  let rid = record.arg as u32;

  let fut = async move {
    let mut table = lock_resource_table().await;
    match table.close(rid) {
      Some(_) => Ok(0),
      None => Err(bad_resource()),
    }
  };
  fut.boxed()
}

fn op_read(
  record: Record,
  zero_copy_buf: Option<PinnedBuf>,
) -> Pin<Box<HttpOp>> {
  let rid = record.arg as u32;
  debug!("read rid={}", rid);
  let mut zero_copy_buf = zero_copy_buf.unwrap();

  let fut = async move {
    let mut table = lock_resource_table().await;
    let stream_resource =
      table.get_mut::<TcpStream>(rid).ok_or_else(bad_resource)?;
    let stream = &mut stream_resource.0;
    let nread = stream.read(&mut zero_copy_buf).await?;
    debug!("read success {}", nread);
    Ok(nread as i32)
  };

  fut.boxed()
}

fn op_write(
  record: Record,
  zero_copy_buf: Option<PinnedBuf>,
) -> Pin<Box<HttpOp>> {
  let rid = record.arg as u32;
  debug!("write rid={}", rid);
  let zero_copy_buf = zero_copy_buf.unwrap();

  let fut = async move {
    let mut table = lock_resource_table().await;
    let stream = table.get_mut::<TcpStream>(rid).ok_or_else(bad_resource)?;
    // I'd prefer to do stream.0.write(&zero_copy_buf).await? but it seems
    // `PinnedBuf` does not implement Sync for its fields.
    // (error: `std::ptr::NonNull<u8>` cannot be shared between threads safely)
    // It's strange because `stream.read().await` works fine /shrug
    let write_fut = poll_fn(move |cx| {
      let pinned_stream = Pin::new(&mut stream.0);
      pinned_stream.poll_write(cx, &zero_copy_buf)
    });
    let nwritten = write_fut.await?;
    debug!("write success {}", nwritten);
    Ok(nwritten as i32)
  };

  fut.boxed()
}

fn js_check(r: Result<(), ErrBox>) {
  if let Err(e) = r {
    panic!(e.to_string());
  }
}
