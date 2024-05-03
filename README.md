# Сервис аутентификации - Regify

Regify - это сервис аутентификации, написанный на языке программирования Rust с использованием фреймворка Actix-web. Этот сервис предоставляет API для регистрации и входа пользователей, а также управления пользователями.

## Установка

Для установки и запуска проекта вам потребуется установить следующие зависимости:

- Rust и Cargo (https://www.rust-lang.org/)
- PostgreSQL (https://www.postgresql.org/)

После установки зависимостей, склонируйте репозиторий и выполните следующие команды:

```bash
git clone git@github.com:ankodd/Regify.git
cd regify
cargo run
```
## Cargo.toml
```rust
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
actix-cors = "0.7.0"
serde_json = "1.0.116"
```

- **actix-web** - для запуска http сервера и создания маршрутов  
- **chrono** - для работы со временем  
- **diesel** - ORM  
- **dotenvy** - для того чтобы программа могла рабоать с .env файлом  
- **serde** - Сериализация/Десериализация  
- **uuid** - Генератор уникального id  
- **r2d2** - для работы с пулом подключений к бд  
- **env_logger** - для логов в консоли  
- **diesel_derive_enum** - расширение для дизеля, позволяет подставлять enum как тип данных SQL
- **actix-cors** - работа с cors
- **serde_json** - создание json сообщения

## API
### Регистрация
Метод: Post  
Путь: /api/registration  
Пример запроса:  
```json
{
  "username": "user123",
  "password": "password123"
}
```  
Пример Ответа:  
```json
{
    "id": "4378f5f0-e474-48e6-8f82-cebce1133885",
    "username": "user123",
    "password": "password123",
    "privilege": "Free",
    "created_at": "2024-05-02T10:57:29.800243Z"
}
```
### Логин
Метод: Post  
Путь: /api/login    
Пример запроса:  
```json
{
  "username": "user123",
  "password": "password123"
}
```  
Пример Ответа:  
```json
{
    "id": "4378f5f0-e474-48e6-8f82-cebce1133885",
    "username": "user123",
    "password": "password123",
    "privilege": "Free",
    "created_at": "2024-05-02T10:57:29.800243Z"
}
```
### Список пользователей
Метод: Get  
Путь: /api/users  
Пример Ответа:  
```json
[
    {
        "id": "5fcab5b2-09d3-45f4-88e4-c0de4189d8d5",
        "username": "ruslan",
        "password": "qwerty123456",
        "privilege": "Vip",
        "created_at": "2024-05-01T18:15:44.527294Z"
    },
    {
        "id": "9be83393-6b02-42f0-b294-81d6c12eef0d",
        "username": "ivan5555",
        "password": "1234567890",
        "privilege": "Free",
        "created_at": "2024-05-01T18:20:57.266589Z"
    },
    {
        "id": "4378f5f0-e474-48e6-8f82-cebce1133885",
        "username": "user123",
        "password": "password123",
        "privilege": "Free",
        "created_at": "2024-05-02T10:57:29.800243Z"
    }
]
```
### Получить пользователя по id
Метод: Get  
Путь: /api/users/{id}  
Пример Ответа:  
```json
{
    "id": "5fcab5b2-09d3-45f4-88e4-c0de4189d8d5",
    "username": "ruslan",
    "password": "qwerty123456",
    "privilege": "Vip",
    "created_at": "2024-05-01T18:15:44.527294Z"
}
```
### Изменить данные у пользователя
Метод: Patch  
Путь: /api/users/{id}  
Пример запроса:  
```json
{
  "field": "password",
  "new_value": "abcdabcd"
}
```  
Пример Ответа:  
```json
{
    "id": "5fcab5b2-09d3-45f4-88e4-c0de4189d8d5",
    "username": "ruslan",
    "password": "abcdabcd",
    "privilege": "Vip",
    "created_at": "2024-05-01T18:15:44.527294Z"
}
```
### Удалить пользователя по id
Метод: Delete  
Путь: /api/users/{id}  
Пример Ответа:  
```json
{
    "id": "4378f5f0-e474-48e6-8f82-cebce1133885",
    "username": "user123",
    "password": "password123",
    "privilege": "Free",
    "created_at": "2024-05-02T10:57:29.800243Z"
}
```
