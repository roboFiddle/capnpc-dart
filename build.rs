fn main() {
    capnpc::CompilerCommand::new()
        .src_prefix("schemas")
        .file("schemas/schema.capnp")
        .run()
        .expect("failed to compile proto");
}
