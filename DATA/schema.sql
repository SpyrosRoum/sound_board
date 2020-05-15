CREATE TABLE IF NOT EXISTS settings (
    bot_token TEXT,
    CONSTRAINT token UNIQUE (bot_token)
);

CREATE TABLE IF NOT EXISTS words (
    g_id NUMERIC NOT NULL,
    chn_id NUMERIC NOT NULL,
    word TEXT,
    file_path TEXT NOT NULL,
    CONSTRAINT u_g_chn_word UNIQUE ( g_id, chn_id, word )
);