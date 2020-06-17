#![feature(generator_trait)]

pub use crate::display::*;
use clap::{Parser, Subcommand};
use jupyter::{
    async_trait, Executed, ExecutionReply, ExecutionRequest, ExecutionResult, InstallAction, JupyterResult,
    JupyterServerProtocol, JupyterServerSockets, JupyterTheme, LanguageInfo, OpenAction, StartAction, UnboundedSender,
    UninstallAction,
};
use jupyter_derive::{include_png32, include_png64};
use serde_json::Value;
use std::path::PathBuf;
use valkyrie_parser::{repl::ValkyrieREPL, ThisParser};
mod display;
mod expression;

use valkyrie_types::{third_party::pex::ParseState, ValkyrieResult, ValkyrieValue};

pub struct ValkyrieExecutor {
    sockets: JupyterServerSockets,
    config: ValkyrieConfig,
}

pub struct ValkyrieConfig {
    running_time: bool,
}

impl Default for ValkyrieConfig {
    fn default() -> Self {
        ValkyrieConfig { running_time: false }
    }
}

impl Default for ValkyrieExecutor {
    fn default() -> Self {
        ValkyrieExecutor { sockets: Default::default(), config: ValkyrieConfig::default() }
    }
}

impl ValkyrieExecutor {
    pub(crate) async fn repl_parse_and_run(&mut self, code: &str) -> ValkyrieResult<()> {
        let (_, terms) = ValkyrieREPL::parse_many(ParseState::new(code))?;
        for i in terms {
            match self.execute_repl(i).await {
                Ok(v) => self.send_value(v).await,
                Err(e) => {
                    if e.is_fatal() {
                        return Err(e);
                    }
                    else {
                        self.sockets.send_executed(DisplayError::new(format!("Error: {}", e))).await;
                    }
                }
            }
        }
        Ok(())
    }

    pub async fn execute_repl(&mut self, tree: ValkyrieREPL) -> ValkyrieResult<ValkyrieValue> {
        match tree {
            ValkyrieREPL::Namespace(_) => Ok(ValkyrieValue::Nothing),
            ValkyrieREPL::Expression(e) => self.execute_expr(*e).await,
        }
    }

    pub(crate) async fn send_value(&self, value: ValkyrieValue) {
        match value {
            // never type never sends
            ValkyrieValue::Nothing => {}
            ValkyrieValue::Null => self.sockets.send_executed(DisplayKeywords::new("null")).await,
            ValkyrieValue::Unit => self.sockets.send_executed(DisplayKeywords::new("( )")).await,
            ValkyrieValue::Boolean(v) => self.sockets.send_executed(DisplayKeywords::new(v)).await,
            ValkyrieValue::Unsigned8(v) => self.sockets.send_executed(DisplayNumber::new(v)).await,
            ValkyrieValue::Unsigned16(v) => self.sockets.send_executed(DisplayNumber::new(v)).await,
            ValkyrieValue::Unsigned32(v) => self.sockets.send_executed(DisplayNumber::new(v)).await,
            ValkyrieValue::Unsigned64(v) => self.sockets.send_executed(DisplayNumber::new(v)).await,
            ValkyrieValue::Unsigned128(v) => self.sockets.send_executed(DisplayNumber::new(v)).await,
            ValkyrieValue::Integer8(v) => self.sockets.send_executed(DisplayNumber::new(v)).await,
            ValkyrieValue::Integer16(v) => self.sockets.send_executed(DisplayNumber::new(v)).await,
            ValkyrieValue::Integer32(v) => self.sockets.send_executed(DisplayNumber::new(v)).await,
            ValkyrieValue::Integer64(v) => self.sockets.send_executed(DisplayNumber::new(v)).await,
            ValkyrieValue::Integer128(v) => self.sockets.send_executed(DisplayNumber::new(v)).await,
            ValkyrieValue::Float32(v) => self.sockets.send_executed(DisplayNumber::new(v)).await,
            ValkyrieValue::Float64(v) => self.sockets.send_executed(DisplayNumber::new(v)).await,
            ValkyrieValue::UTF8String(v) => self.sockets.send_executed(Value::String(v.as_str().to_string())).await,
            ValkyrieValue::Buffer(_) => {
                todo!()
            }
            ValkyrieValue::Class(_) => {
                todo!()
            }
            ValkyrieValue::Variant(_) => {
                todo!()
            }
        }
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
        match self.repl_parse_and_run(&code.code).await {
            Ok(_) => code.as_reply(true, code.execution_count),
            Err(e) => {
                self.sockets.send_executed(DisplayError::new(e.to_string())).await;
                code.as_reply(false, code.execution_count)
            }
        }
    }

    fn running_time(&self, time: f64) -> String {
        if self.config.running_time { format!("<sub>Elapsed time: {:.2} seconds.</sub>", time) } else { String::new() }
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
