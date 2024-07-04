use std::str::FromStr;
use std::env;
use std::time::Instant;
use solana_sdk::signature::Signer;
use solana_client::rpc_client::RpcClient;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::transaction::Transaction;
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::pubkey::Pubkey;
use solana_sdk::signer::keypair::read_keypair_file;

const RPC_ADDR: &str = "https://api.devnet.solana.com";

fn main() {
    // 读取命令行参数
    let args: Vec<String> = env::args().collect();
    println!("命令行参数: {:?}", args);
    if args.len() != 3 {
        eprintln!("用法: {} <最大可选整数> <期望总数>", args[0]);
        return;
    }

    let max_choosable_integer: i32 = args[1].parse().expect("无效的最大可选整数");
    println!("最大可选整数: {}", max_choosable_integer);
    let desired_total: i32 = args[2].parse().expect("无效的期望总数");
    println!("期望总数: {}", desired_total);

    // 将参数转换为字节数组
    let mut instruction_data = vec![];
    instruction_data.extend_from_slice(&max_choosable_integer.to_le_bytes());
    instruction_data.extend_from_slice(&desired_total.to_le_bytes());
    println!("指令数据: {:?}", instruction_data);

    // 目标程序的Pubkey
    let program_id = Pubkey::from_str("3N8i3UJNMBw5eyoQEKf8Qo14xh3yLKuXpK1jWVde8tFX").expect("无效的程序ID");
    println!("程序ID: {}", program_id);

    // 用户的密钥对
    let payer = read_keypair_file("/Users/mac/.config/solana/id.json").expect("读取密钥文件失败");
    println!("付款人公钥: {}", payer.pubkey());

    let client = RpcClient::new(RPC_ADDR);
    println!("RPC客户端已创建");

    // 检查付款人账户余额
    let balance = client.get_balance(&payer.pubkey()).expect("获取余额失败");
    println!("付款人账户余额: {}", balance);

    let account_metas = vec![
        AccountMeta::new(payer.pubkey(), true),
    ];
    println!("账户元数据: {:?}", account_metas);

    let instruction = Instruction::new_with_bytes(
        program_id,
        &instruction_data,
        account_metas,
    );
    println!("指令: {:?}", instruction);

    let ixs = vec![instruction];
    println!("指令列表: {:?}", ixs);

    let latest_blockhash = client.get_latest_blockhash().expect("获取最新区块哈希失败");
    println!("最新区块哈希: {:?}", latest_blockhash);

    let tx = Transaction::new_signed_with_payer(
        &ixs,
        Some(&payer.pubkey()),
        &[&payer],
        latest_blockhash,
    );
    println!("交易: {:?}", tx);

    // 模拟交易
    let simulation_result = client.simulate_transaction(&tx).expect("模拟交易失败");
    println!("交易模拟结果: {:?}", simulation_result);

    // 记录开始时间
    let start = Instant::now();

    match client.send_and_confirm_transaction(&tx) {
        Ok(sig) => {
            let duration = start.elapsed();
            println!("交易签名: {}", sig);
            println!("交易耗时: {:?}", duration);
        },
        Err(e) => {
            eprintln!("交易失败: {:?}", e);
            let duration = start.elapsed();
            println!("交易耗时: {:?}", duration);
        }
    }
}
