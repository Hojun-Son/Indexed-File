use std::fs::File;
use std::io::{self, BufReader, Read, BufRead};
use std::path::Path;
use sled::{Db, IVec};

fn main() -> sled::Result<()> {
    // sled크레이트로 indexed 파일 생성
    let tree: Db = sled::open("indexed_file")?;

    // key: value
    // key는 record 번호(int면 little-endian으로 저장), value는 nth record의 시작 오프셋 (8바이트 long 타입)
    // 인덱스 파일 사이즈를 8로 나누면 전체 레코드수가 나옴 (8바이트인 long타입으로 저장했으므로)

    // CSV 파일 읽기
    let file_path = "/mnt/c/Users/hjson96/Desktop/파수 교육/빅데이터처리(SystemThinking)/homework/1g_test.csv"; 
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);

    let mut buffer = String::new();
    let mut byte_offset: u64 = 0;
    let mut record_num: u16 = 0;

    // CSV 파일의 각 행을 읽어서 처리
    loop {
        buffer.clear();
        match reader.read_line(&mut buffer) {
            Ok(0) => break, // EOF reached
            Ok(_) => {
                // 현재 행의 바이트 크기를 계산
                let current_byte_size = buffer.as_bytes().len() as u64;

                // 키와 값을 indexed 파일에 저장
                let key = record_num.to_le_bytes();
                let value = byte_offset.to_le_bytes();
                tree.insert(key, &value)?;

                // 다음 행의 시작 바이트 오프셋 업데이트
                byte_offset += current_byte_size;
                record_num += 1;

                println!("record_num: {:?}, offset: {:?}", key, value);

            }
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                break;
            }
        }
    }

    Ok(())
}
