-- Add migration script here
INSERT INTO users (user_id, username, password_hash)
VALUES (
    'd7a14cd5-9235-4f43-8884-b4e2f4410686',
    'admin',
    '$argon2id$v=19$m=15000,t=2,p=1$Zm5N/mUEoGMtYI3xKBGDaA$ByqVSzk+bDnGeTp7uyEpx3xhZuwvIftIB7xZIN3yCQM'
)