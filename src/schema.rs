pub static SCHEMA: &str = r#"
CREATE TABLE IF NOT EXISTS settings (
    bot_token TEXT,
    CONSTRAINT token UNIQUE (bot_token)
);

CREATE TABLE IF NOT EXISTS words (
    chn_id NUMERIC NOT NULL,
    word TEXT,
    file_path TEXT NOT NULL,
    CONSTRAINT u_g_chn_word UNIQUE ( chn_id, word )
);
"#;