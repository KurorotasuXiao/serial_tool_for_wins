use clap::Parser;

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

fn main() {
    let args = Args::parse();

    println!("端口: {}", args.port);
    println!("波特率: {}", args.baud);
    println!("操作: {:?}", args.action);
    
    match args.action {
        Action::Send { message } => {  // 直接解构 message
            println!("发送消息: {}", message);
        }
        Action::Monitor => {
            println!("开始监听...");
        }
    }
}