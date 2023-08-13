use chrono::Local;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::time::{Duration, Instant};
use tokio::time;

#[tokio::main]
async fn main() -> io::Result<()> {
    // 定义日志文件路径
    let log_file_path = "/var/log/caddy/fileserver.log";
    // 定义结果文件路径
    let result_file_path = "/root/log.txt";

    // 创建结果文件
    let result_file = File::create(&result_file_path)?;
    let mut writer = BufWriter::new(result_file);

    // 创建 HashSet 用于跟踪已经出现过的 remote_ip
    let mut seen_ips = HashSet::new();

    loop {
        let start_time = Instant::now();

        // 读取日志文件
        if let Ok(file) = File::open(&log_file_path) {
            let reader = BufReader::new(file);

            for line in reader.lines() {
                if let Ok(line) = line {
                    if line.contains("INFO") && line.contains(r#""user_id": "he""#) {
                        // 解析日志行获取 remote_ip
                        if let Some(remote_ip) = extract_remote_ip(&line) {
                            // 判断 remote_ip 是否已经出现过
                            if seen_ips.insert(remote_ip.clone()) {
                                // remote_ip 是新的值，写入结果文件
                                // 添加时间记录
                                let current_time = Local::now();
                                let formatted_time =
                                    current_time.format("%Y年%m月%d日 %H:%M:%S").to_string();
                                writeln!(writer, "{} {}", formatted_time, remote_ip)?;
                                writer.flush()?;
                                println!("Found matching log line. Remote IP: {}", remote_ip);
                            }
                        }
                    }
                }
            }
        }

        let elapsed_time = start_time.elapsed();
        if elapsed_time < Duration::from_secs(60) {
            // 如果读取日志文件的时间小于60秒，则等待剩余时间
            let sleep_duration = Duration::from_secs(60) - elapsed_time;
            time::sleep(sleep_duration).await;
        }
    }
}

fn extract_remote_ip(line: &str) -> Option<String> {
    if let Some(start_index) = line.find(r#"remote_ip": ""#) {
        let start_index = start_index + r#"remote_ip": ""#.len();
        if let Some(end_index) = line[start_index..].find('"') {
            let end_index = start_index + end_index;
            return Some(line[start_index..end_index].to_owned());
        }
    }
    None
}
