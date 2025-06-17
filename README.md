## 🗂 Структура проекта

```plaintext
grok-client/
├── src/
│   └── main.rs      
├── .env             # Конфигурация (API ключи)
├── Cargo.toml       
└── session.session  # Файл сессии (автогенерируется)

Пример работы
🚀 Starting with API_ID: 12345 and API_HASH: a1b2c3d4e5
🔑 Authorization required
Enter phone number: +1234567890
Enter Telegram code: 12345
🔒 Authorization successful!
🔍 Searching for bot GrokAI...
✅ Bot found: GrokAI (ID: 67890)
📤 Sending message with priority High
✅ Message sent
📥 Bot response: Принято!

API_ID=ваш_ид
API_HASH=ваш_хэш
