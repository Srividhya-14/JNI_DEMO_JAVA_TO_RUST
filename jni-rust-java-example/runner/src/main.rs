use jni::InitArgsBuilder;
use jni::JavaVM;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    let class_path = "./java/out";
    let native_lib_dir = "/home/RUST_POC/jni-rust-java-example/target/release";// give the path to store the native library

    let class_path_opt = format!("-Djava.class.path={}", class_path);
    let lib_path_opt = format!("-Djava.library.path={}", native_lib_dir);

    let args = InitArgsBuilder::new()
        .option(&class_path_opt)
        .option(&lib_path_opt)
        .build()?;

    let jvm = JavaVM::new(args)?;
    let mut env = jvm.attach_current_thread()?;

    let os_tid = unsafe { libc::syscall(libc::SYS_gettid) };
    let rust_tid = std::thread::current().id();

    println!(
        "1.[Rust main] Running on Rust thread RustThreadId={:?} OStid={}",
        rust_tid, os_tid
    );

    let class = env.find_class("com/example/Hello")?;
    env.call_static_method(class, "javaFunction", "()V", &[])?;

    std::thread::sleep(Duration::from_secs(5));
    Ok(())
}
