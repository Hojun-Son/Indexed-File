use sled::{Db, IVec};

fn main() -> sled::Result<()> {
    // sled 데이터베이스 생성
    let tree: Db = sled::open("indexed_file")?;

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
