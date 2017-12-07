extern crate capnpc;

fn main() {
    capnpc::CompilerCommand::new()
        .src_prefix("schema")
        .file("schema/example.capnp")
        .run().expect("schema compiler command");
}
