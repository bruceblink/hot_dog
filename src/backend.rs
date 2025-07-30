use dioxus::prelude::*;

#[cfg(feature = "server")]
thread_local! {
    pub static DB: rusqlite::Connection = {
        // 1. 先找到当前可执行文件的完整路径
        let exe_path = std::env::current_exe()
            .expect("获取可执行文件路径失败");
        // 2. 取其父目录（即可执行文件所在的目录）
        let exe_dir = exe_path.parent()
            .expect("无法获取可执行文件所在目录");
        // 3. 拼出 hotdog.db 的绝对路径
        let db_file = exe_dir.join("hotdog.db");

        // 打印一下路径，方便排查
        println!("打开数据库文件：{:?}", db_file);

        // 4. 用绝对路径打开（如果不存在，会自动创建）
        let conn = rusqlite::Connection::open(&db_file)
            .unwrap_or_else(|e| panic!("打开数据库失败 ({:?}): {}", db_file, e));

        // 5. 初始化表
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS dogs (
                id   INTEGER PRIMARY KEY,
                url  TEXT NOT NULL
            );"
        ).unwrap_or_else(|e| panic!("初始化数据表失败: {}", e));

        conn
    };
}

#[server(endpoint = "list_dogs")]
pub async fn list_dogs() -> Result<Vec<(usize, String)>, ServerFnError> {
    let dogs = DB.with(|f| {
        f.prepare("SELECT id, url FROM dogs ORDER BY id DESC LIMIT 10")
            .unwrap()
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
            .unwrap()
            .map(|r| r.unwrap())
            .collect()
    });

    Ok(dogs)
}

#[server(endpoint = "remove_dog")]
pub async fn remove_dog(id: usize) -> Result<(), ServerFnError> {
    DB.with(|f| f.execute("DELETE FROM dogs WHERE id = ?1", [&id]))?;
    Ok(())
}

#[server(endpoint = "save_dog")]
pub async fn save_dog(image: String) -> Result<(), ServerFnError> {
    println!("Saving dog image: {}", image);
    _ = DB.with(|f| f.execute("INSERT INTO dogs (url) VALUES (?1)", [&image]));
    Ok(())
}