javac -d java/out java/src/com/example/Hello.java
cargo build -p rustnative --release
cargo build -p runner --release
#jdkpath
export JAVA_HOME=/home/jdk
export LD_LIBRARY_PATH="$JAVA_HOME/lib/server:$LD_LIBRARY_PATH"
./target/release/runner
