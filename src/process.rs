// 用于错误处理的 Result 类型
use anyhow::Result;
// 用于 CSV 文件读取的 Reader 类型
use csv::Reader;
// 用于序列化和反序列化的特性
use serde::{Deserialize, Serialize};
// 提供文件系统相关的功能
use std::fs;

// 定义 Record 结构体，用于存储 CSV 文件中的每一行数据
#[derive(Debug, Serialize, Deserialize)]
// 将所有字段名转换为 PascalCase 格式
#[serde(rename_all = "PascalCase")]
struct Record {
    name: String,
    position: String,
    // dob 字段重命名为 "DOB"
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    // 将字段 kit 重命名为 KitNumber
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(input: &str, output: &str) -> Result<()> {
    /* 创建 CSV 文件的 Reader 实例
    ? 操作符用于错误传播 */
    let mut reader = Reader::from_path(input)?;

    /* 初始化一个向量，用于存储处理后的 Record 结构体
    预分配内存以提高性能 */
    let mut results = Vec::with_capacity(128);

    // 遍历 CSV 文件中的每一行数据
    for result in reader.deserialize() {
        /* 将每一行数据反序列化为 Record 结构体
        ? 操作符用于错误传播 */
        let record: Record = result?;

        // 将 Record 结构体添加到结果向量中
        results.push(record);
    }
    // 将结果向量序列化为 JSON 格式的字符串
    let json = serde_json::to_string_pretty(&results)?;

    /* 将 JSON 数据写入到输出文件中
    ? 操作符用于错误传播 */
    fs::write(output, json)?;
    // 函数成功执行后返回 Ok(())
    Ok(())
}

// 数据模型：Record 结构体定义了 CSV 文件中每行数据的格式。
// CSV 文件读取：使用 csv::Reader 读取 CSV 文件，并通过 deserialize 方法将每一行转换为 Record 对象。
// 序列化和反序列化：使用 serde 特性将 Record 对象序列化为 JSON 格式。
// 错误处理：使用 anyhow::Result 来捕获和传播错误，简化错误处理逻辑。
// 文件操作：使用 std::fs 模块的 write 函数将 JSON 数据写入到输出文件中。
