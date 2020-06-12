#![feature(generator_trait)]

use clap::{Parser, Subcommand};
use jupyter::{
    async_trait, Executed, ExecutionReply, ExecutionRequest, ExecutionResult, InstallAction, JupyterResult,
    JupyterServerProtocol, JupyterServerSockets, JupyterTheme, LanguageInfo, OpenAction, StartAction, UnboundedSender,
    UninstallAction,
};
use jupyter_derive::{include_png32, include_png64};
use serde_json::Value;
use std::{ops::Generator, path::PathBuf};
use valkyrie_parser::{
    expression::ValkyrieExpression,
    repl::{parse_repl, ValkyrieREPL},
    ThisParser,
};
mod expression;

use valkyrie_types::{third_party::pex::ParseState, ValkyrieError, ValkyrieResult, ValkyrieValue};

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
                Err(e) => self.sockets.send_executed(DisplayError { text: format!("Error: {}", e) }).await,
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
            ValkyrieValue::Null => self.sockets.send_executed(DisplayKeywords { text: "null".to_string() }).await,
            ValkyrieValue::Unit => self.sockets.send_executed(DisplayKeywords { text: "()".to_string() }).await,
            ValkyrieValue::Boolean(v) => self.sockets.send_executed(DisplayKeywords { text: v.to_string() }).await,
            ValkyrieValue::Unsigned8(v) => self.sockets.send_executed(DisplayNumber { text: v.to_string() }).await,
            ValkyrieValue::Unsigned16(v) => self.sockets.send_executed(DisplayNumber { text: v.to_string() }).await,
            ValkyrieValue::Unsigned32(v) => self.sockets.send_executed(DisplayNumber { text: v.to_string() }).await,
            ValkyrieValue::Unsigned64(v) => self.sockets.send_executed(DisplayNumber { text: v.to_string() }).await,
            ValkyrieValue::Unsigned128(v) => self.sockets.send_executed(DisplayNumber { text: v.to_string() }).await,
            ValkyrieValue::Integer8(v) => self.sockets.send_executed(DisplayNumber { text: v.to_string() }).await,
            ValkyrieValue::Integer16(v) => self.sockets.send_executed(DisplayNumber { text: v.to_string() }).await,
            ValkyrieValue::Integer32(v) => self.sockets.send_executed(DisplayNumber { text: v.to_string() }).await,
            ValkyrieValue::Integer64(v) => self.sockets.send_executed(DisplayNumber { text: v.to_string() }).await,
            ValkyrieValue::Integer128(v) => self.sockets.send_executed(DisplayNumber { text: v.to_string() }).await,
            ValkyrieValue::Float32(v) => self.sockets.send_executed(DisplayNumber { text: v.to_string() }).await,
            ValkyrieValue::Float64(v) => self.sockets.send_executed(DisplayNumber { text: v.to_string() }).await,
            ValkyrieValue::String(_) => {
                todo!()
            }
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

pub struct DisplayKeywords {
    text: String,
}

impl Executed for DisplayKeywords {
    fn mime_type(&self) -> String {
        "text/html".to_string()
    }

    fn as_json(&self, _: JupyterTheme) -> Value {
        Value::String(format!(r#"<span style="color: pink">{}</span>"#, self.text))
    }
}

pub struct DisplayError {
    text: String,
}
impl Executed for DisplayError {
    fn mime_type(&self) -> String {
        "text/html".to_string()
    }

    fn as_json(&self, _: JupyterTheme) -> Value {
        Value::String(format!(r#"<span style="color: red">{}</span>"#, self.text))
    }
}

pub struct DisplayNumber {
    text: String,
}
impl Executed for DisplayNumber {
    fn mime_type(&self) -> String {
        "text/html".to_string()
    }

    fn as_json(&self, _: JupyterTheme) -> Value {
        Value::String(format!(r#"<span class="color: oriange">{}</span>"#, self.text))
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
                self.sockets.send_executed(DisplayError { text: e.to_string() }).await;
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

fn main() -> JupyterResult<()> {
    JupyterApplication::parse().run()
}
