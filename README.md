## Student Registry Server
A lightweight REST API for managing a school registry which has an administrator, student and staff. It is built with `Rust` and `Axum`. Data is persited to a local JSON file and protected with JWT authentication 
for administrative actions.

### Features
- The registry is initialized by the admin. No other endpoint is called if registry is not initialized
- Ability to add, fetch, update, and remove a student
- Ability to add, fetch, update, and remove a staff
- JWT-authentication for administrative actions
- Storage of data in a local JSON file (No database used)

### Tech Stack
- Axum => Web framework
- tokio => Async runtime
- serde/serde_json => Serialization
- jsonwebtoken => Authentication
- Uuid => ID
- chrone => Timestamps
- tracing/tracing-subscriber => Logging
