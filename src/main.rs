use clap::Parser;
use anyhow::{Context, Result};
use serialport::SerialPort;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 绑定的COM口
    #[arg(short, long, default_value = "COM3")]
    port: String,

    /// 波特率
    #[arg(short, long, default_value = "115200")]
    baud: u32,

    /// 要执行的操作类型
    #[command(subcommand)]
    action: Action,
}

#[derive(clap::Subcommand, Debug)]
enum Action {
    /// 发送消息到串口
    Send {
        /// 要发送的消息内容
        message: String,
    },
    /// 监听串口数据
    Monitor,
}

// 打开对应串口函数
fn open_serial(port_name: &str, baud_rate: u32) -> Result<Box<dyn SerialPort>> {
    let port = serialport::new(port_name, baud_rate)
        .data_bits(serialport::DataBits::Eight)
        .stop_bits(serialport::StopBits::One)
        .parity(serialport::Parity::None)
        .timeout(Duration::from_millis(100))
        .open()
        .with_context(|| format!("无法打开端口 {}", port_name))?;

    Ok(port)
}

// 检查串口是否存在
fn port_exists(port_name: &str) -> bool {
    serialport::available_ports()
        .map(|ports| ports.iter().any(|p| p.port_name == port_name))
        .unwrap_or(false)
}

fn main() -> Result<()> {
    env_logger::init(); // 初始化日志
    let args = Args::parse();

    if !port_exists(&args.port) {
        anyhow::bail!("端口 {} 不存在！可用端口：{:?}",
            args.port,
            serialport::available_ports()?
        );
    }

    // 打开串口（带错误上下文）
    let mut port = open_serial(&args.port, args.baud)
        .context("串口初始化失败，请检查端口是否存在或权限")?;

    match args.action {
        Action::Send { message } => {  // 直接解构 message
            println!("发送消息: {}", message);
            // To do:实现发送逻辑
        }
        Action::Monitor => {
            println!("开始监听...");
            // To do:实现监听逻辑
        }
    }

    Ok(())
}