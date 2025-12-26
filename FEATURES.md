# Sentinel Features

Sentinel is a modular, full‑stack system development framework built in Rust.  
It provides a clean architecture with three major layers:

- **Frontend** — GUI, scripting, windows, events  
- **Backend** — authentication, data handling, cryptography, work engine  
- **Middlend** — security, middleware, validation, runtime, extensions  

Below is a full overview of Sentinel’s capabilities.

---

## Frontend Features

- **GUI System**
  - Reusable components
  - HTML/CSS injection
  - Dynamic props
  - Containers for complex layouts

- **Window Management**
  - WindowOptions + WindowBuilder
  - Tauri‑powered window creation

- **Event System**
  - Rust → JS events
  - JS → Rust events
  - Secure event bridge

- **Scripting Engine**
  - Safe JS execution
  - Script injection
  - Function calling

- **App Builder**
  - Mount components
  - Render GUI
  - Inject scripts
  - Launch Tauri app

---

## Backend Features

- **Authentication**
  - Argon2id + SHA‑256 hashing
  - Session tokens
  - Role checking
  - User serialization

- **Cryptography**
  - AES‑256 encryption/decryption
  - Developer‑controlled salts + keys

- **Data Handling**
  - JSON, YAML, XML CRUD
  - SQL query abstraction

- **Role-Based Views**
  - Map roles → GUI containers
  - Resolve views dynamically

- **Work Engine**
  - Unified backend orchestrator
  - Connects all backend modules

---

## Middlend Features

- **Security Layer**
  - Input validation + sanitization
  - Session enforcement
  - Role enforcement
  - Rate limiting
  - Intrusion detection

- **Middleware System**
  - Pre‑processing pipeline
  - Security policy enforcement

- **Loader**
  - Loads project structure
  - Loads templates + configs

- **Validator**
  - Deep project validation
  - Template integrity checks

- **Identity**
  - Project metadata loader

- **Extensions**
  - Developer‑defined function wrappers
  - Custom hooks and behaviors

- **Runtime Engine**
  - Central execution engine
  - Connects Frontend + Backend + Middlend

---

## Additional Features

- **Installer**
  - Downloads and installs Sentinel into any project

- **Modular Architecture**
  - Clean separation of concerns
  - Easy to extend and customize

- **Beginner-Friendly**
  - Clear APIs
  - Strong documentation
  - Predictable behavior

---

Sentinel is built to help developers create secure, modular, full‑stack systems with confidence and clarity.  
More features will be added as the framework evolves!

---
