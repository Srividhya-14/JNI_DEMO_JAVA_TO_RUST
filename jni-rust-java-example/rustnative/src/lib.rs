use jni::sys::{jclass, JNIEnv as RawJNIEnv};
use jni::JNIEnv;

#[no_mangle]
pub extern "system" fn Java_com_example_Hello_nativeFromRust(
    env: *mut RawJNIEnv,
    _class: jclass,
) {
    let mut env = unsafe { JNIEnv::from_raw(env).unwrap() };

    let (jvm_tid, os_tid) = get_java_thread_info(&mut env);

    println!(
        "4.[Rust callback] JVM-thread-id={} OS-tid={} RustThreadId={:?}",
        jvm_tid,
        os_tid,
        std::thread::current().id(),
    );
}


#[no_mangle]
pub extern "system" fn Java_com_example_Hello_getNativeTid(
    _env: *mut RawJNIEnv,
    _class: jclass,
) -> i64 {
    unsafe { libc::syscall(libc::SYS_gettid) }
}

fn get_java_thread_info(env: &mut JNIEnv) -> (i64, i64) {
    let class = env.find_class("com/example/Hello").unwrap();

    // Call Hello.getJvmThreadId()
    let jvm_tid = env
        .call_static_method(class, "getJvmThreadId", "()J", &[])
        .unwrap()
        .j()
        .unwrap();

    // Native OS tid
    let os_tid = unsafe { libc::syscall(libc::SYS_gettid) as i64 };

    (jvm_tid, os_tid)
}
