use capnpc::CompilerCommand;

extern crate configure_me_codegen;

fn main() -> Result<(), configure_me_codegen::Error> {
    println!("cargo:rerun-if-changed=capnp");
    CompilerCommand::new()
        .src_prefix("capnp")
        .file("capnp/echo.capnp")
        .run()
        .unwrap();
    configure_me_codegen::build_script_auto()
}
