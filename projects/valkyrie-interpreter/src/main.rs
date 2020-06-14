use clap::Parser;
use jupyter::JupyterResult;
use valkyrie_interpreter::JupyterApplication;

fn main() -> JupyterResult<()> {
    JupyterApplication::parse().run()
}
