package com.example;

public class Hello {
    private static native void nativeFromRust();
    private static native long getNativeTid();

    static {
        System.loadLibrary("rustnative");
    }

    // Returns OS tid (via Rust)
    public static long osThreadId() {
        return getNativeTid();
    }

    // Returns JVM logical thread id
    public static long getJvmThreadId() {
        return Thread.currentThread().getId();
    }

    public static void javaFunction() {
        System.out.println("2. Java thread: JVM-id=" + getJvmThreadId()
            + " OS-tid=" + osThreadId()
            + " javaFunction() called from Rust.");

        Thread t = new Thread(() -> {
            System.out.println("➡️3. Java thread: JVM-id=" + getJvmThreadId()
                + " OS-tid=" + osThreadId()
                + " calling nativeFromRust()");
            nativeFromRust();
        });
        t.start();
    }
}
