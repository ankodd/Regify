# Сервис аутентификации - Regify

## Cargo.toml
```
[package]
name = "regify"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
actix-web = "4.5.1"
chrono = { version = "0.4.38", features = ["serde"] }
diesel = { version = "2.1.6", features = ["postgres", "uuid", "chrono", "r2d2"] }
dotenvy = "0.15.7"
serde = { version = "1.0.199", features = ["derive"] }
uuid = { version = "1.8.0", features = ["v4", "serde"] }
r2d2 = "0.8.10"
env_logger = "0.11.3"
diesel-derive-enum = { version = "2.1.0", features = ["postgres"] }
```

actix-web - для запуска http сервера и создания маршрутов
chrono - для работы со временем
diesel - ORM
dotenvy - для того чтобы программа могла рабоать с .env файлом
serde - Сериализация/Десериализация
uuid - Генератор уникального id
r2d2 - для работы с пулом подключений к бд
env_logger - для логов
diesel_derive_enum - расширение для дизеля, позволяет подставлять enum как тип данных SQL
