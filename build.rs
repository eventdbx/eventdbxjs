fn main() {
  napi_build::setup();

  let target = std::env::var("TARGET").unwrap_or_default();
  if target.contains("apple") {
    // Allow unresolved N-API symbols at link time; Node provides them at runtime.
    println!("cargo:rustc-link-arg=-Wl,-undefined,dynamic_lookup");
    println!("cargo:rustc-cdylib-link-arg=-Wl");
    println!("cargo:rustc-cdylib-link-arg=-undefined");
    println!("cargo:rustc-cdylib-link-arg=dynamic_lookup");
  }

  let schema_dir = std::path::Path::new("src/plugin_api/schemas");
  let mut command = capnpc::CompilerCommand::new();
  command.src_prefix(schema_dir);
  command.import_path(schema_dir);
  command.default_parent_module(vec!["plugin_api".to_string()]);
  command.file(schema_dir.join("control.capnp"));
  command
    .run()
    .expect("failed to compile Cap'n Proto schemas for plugin API");

  println!("cargo:rerun-if-changed=src/plugin_api/schemas/control.capnp");
}
