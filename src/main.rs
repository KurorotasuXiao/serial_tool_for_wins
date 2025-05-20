use clap::Parser;
use anyhow::{Context, Result};
use serialport::SerialPort;
use std::time::Duration;
use std::io::{self, Read};

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

fn send_message(port: &mut Box<dyn SerialPort>, message: &str) -> Result<()> {
    // 转化为字节流
    let bytes = message.as_bytes();

    // 写入串口
    port.write_all(bytes)
        .context("写入串口失败")?;

    // 确保数据完全发送
    port.flush()
        .context("刷新缓冲区失败")?;

    Ok(())
}

/// 持续监听串口数据
fn monitor_port(port: &mut Box<dyn SerialPort>) -> Result<()> {
    let mut buffer = [0u8; 256]; // 固定大小缓冲区

    loop {
        match port.read(&mut buffer) {
            Ok(n) => {
                // 将字节转为字符串（宽松UTF-8处理）
                let text = String::from_utf8_lossy(&buffer[..n]);
                println!("{}", text); // 实时输出
            },
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => continue,
            Err(e) => return Err(e.into()),
        }
    }
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
            send_message(&mut port, &message)
                .context("发送消息失败")?;
            println!("消息已发送");
        }
        Action::Monitor => {
            println!("开始监听串口数据（按 Ctrl+C 退出）...");
            monitor_port(&mut port)
                .context("监听过程中发生错误")?;
        }
    }

    Ok(())
}