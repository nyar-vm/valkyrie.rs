use clap::{Parser, Subcommand};
use jupyter::{
    async_trait, ExecutionReply, ExecutionRequest, ExecutionResult, InstallAction, JupyterResult, JupyterServerProtocol,
    JupyterServerSockets, LanguageInfo, OpenAction, StartAction, UnboundedSender, UninstallAction,
};
use jupyter_derive::{include_png32, include_png64};
use std::path::PathBuf;

pub struct ValkyrieExecutor {
    sockets: JupyterServerSockets,
}

impl Default for ValkyrieExecutor {
    fn default() -> Self {
        ValkyrieExecutor { sockets: Default::default() }
    }
}

impl ValkyrieExecutor {
    pub async fn execute(&mut self, code: &str, id: usize) -> JupyterResult<()> {
        self.sockets.send_executed(code).await;
        self.sockets.send_executed(id as f64).await;
        Ok(())
    }
}

#[async_trait]
impl JupyterServerProtocol for ValkyrieExecutor {
    fn language_info(&self) -> LanguageInfo {
        LanguageInfo {
            language: "Valkyrie".to_string(),
            png_64: include_png64!(),
            png_32: include_png32!(),
            language_key: "valkyrie".to_string(),
            file_extensions: ".vk".to_string(),
        }
    }

    async fn running(&mut self, code: ExecutionRequest) -> ExecutionReply {
        match self.execute(&code.code, code.execution_count as usize).await {
            Ok(_) => code.as_reply(true, code.execution_count),
            Err(_) => code.as_reply(false, code.execution_count),
        }
    }

    async fn bind_execution_socket(&self, sender: UnboundedSender<ExecutionResult>) {
        self.sockets.bind_execution_socket(sender).await
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct JupyterApplication {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
    #[command(subcommand)]
    command: JupyterCommands,
}

#[derive(Subcommand)]
enum JupyterCommands {
    Open(Box<OpenAction>),
    Start(Box<StartAction>),
    Install(Box<InstallAction>),
    Uninstall(Box<UninstallAction>),
}

impl JupyterApplication {
    pub fn run(&self) -> JupyterResult<()> {
        let config = ValkyrieExecutor::default();
        match &self.command {
            JupyterCommands::Open(v) => v.run(),
            JupyterCommands::Start(v) => v.run(config),
            JupyterCommands::Install(v) => v.run(config),
            JupyterCommands::Uninstall(v) => v.run(config),
        }
    }
}

fn main() -> JupyterResult<()> {
    JupyterApplication::parse().run()
}
