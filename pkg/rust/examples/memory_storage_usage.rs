#[cfg(feature = "memory-storage")]
mod api_usage {
    use gluesql::{
        memory_storage::MemoryStorage,
        prelude::{Glue, Payload, Value},
    };

    pub async fn run() {
        let storage = MemoryStorage::default();
        let mut glue = Glue::new(storage);

        glue.execute("DROP TABLE IF EXISTS api_test").await.unwrap();

        glue.execute(
            "CREATE TABLE api_test (
                id INTEGER,
                name TEXT,
                nullable TEXT NULL,
                is BOOLEAN
            )",
        )
        .await
        .unwrap();

        glue.execute(
            "INSERT INTO api_test (
                id,
                name,
                nullable,
                is
            ) VALUES
                (1, 'test1', 'not null', TRUE),
                (2, 'test2', NULL, FALSE)",
        )
        .await
        .unwrap();

        let result = glue
            .execute("SELECT * FROM api_test ORDER BY id ASC")
            .await
            .unwrap();

        assert_eq!(
            result,
            vec![Payload::Select {
                labels: (vec![
                    String::from("id"),
                    String::from("name"),
                    String::from("nullable"),
                    String::from("is")
                ]),
                rows: (vec![
                    vec![
                        Value::I64(1),
                        Value::Str(String::from("test1")),
                        Value::Str(String::from("not null")),
                        Value::Bool(true),
                    ],
                    vec![
                        Value::I64(2),
                        Value::Str(String::from("test2")),
                        Value::Null,
                        Value::Bool(false),
                    ],
                ])
            }]
        );
    }
}

fn main() {
    #[cfg(feature = "memory-storage")]
    futures::executor::block_on(api_usage::run());
}
