use sled::{Db, IVec};

fn main() -> sled::Result<()> {
    // sled 데이터베이스 생성
    let tree: Db = sled::open("indexed_file")?;

    let total records_num = 

    // key: value
    // key는 record 번호(int면 little-endian으로 저장), value는 nth record의 시작 오프셋 (8바이트 long 타입)
    // 인덱스 파일 사이즈를 8로 나누면 전체 레코드수가 나옴 (8바이트인 long타입으로 저장했으므로)

    // 키와 값 저장
    tree.insert(b"key1", b"value1")?;
    tree.insert(b"key2", b"value2")?;
    tree.insert(b"key3", b"value3")?;

    // 키를 사용하여 값 조회
    if let Some(value) = tree.get(b"key2")? {
        println!("Value for key2: {:?}", value);
    }

    Ok(())
}
