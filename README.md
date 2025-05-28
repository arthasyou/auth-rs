# Auth-RS - Login Service with JWT and SurrealDB

This project implements a login service using **SurrealDB** as the database and **JWT (JSON Web Tokens)** for authentication. It includes **user registration**, **login**, and **token refresh** functionality.

### Prerequisites

1. **Rust**: Ensure Rust is installed. If not, follow the installation instructions [here](https://www.rust-lang.org/tools/install).
2. **SurrealDB**: Install and start SurrealDB by following the instructions [here](https://surrealdb.com/docs/).

### Setup

1. **Clone the Repository**:

   ```bash
   git clone https://arthasyou/auth-rs.git
   cd auth-rs
   ```

2. **Install Dependencies**:
   Use the following command to install dependencies:

   ```bash
   cargo build
   ```

3. **Configure Database**:
   The configuration for **SurrealDB** and **JWT** settings is in the `config/services.toml` file. Modify it with your actual database credentials and JWT settings.

### Run the Application

1. **Start SurrealDB**:
   Ensure that SurrealDB is running. You can start it with:

   ```bash
   surrealdb start --bind 0.0.0.0:8000
   ```

2. **Run the Application**:
   Run the following command to start the server:

   ```bash
   cargo run
   ```

   The server will start on `localhost:9876` by default.

### API Endpoints

1. **Sign Up (`/signup`)**:

   - **POST** request to register a new user.
   - **Request**:

     ```json
     {
       "username": "john_doe",
       "password": "password123"
     }
     ```

   - **Response**: Success message.

2. **Login (`/login`)**:

   - **POST** request to log in and receive `access_token` and `refresh_token`.
   - **Request**:

     ```json
     {
       "username": "john_doe",
       "password": "password123"
     }
     ```

   - **Response**:

     ```json
     {
       "access_token": "your_access_token_here",
       "refresh_token": "your_refresh_token_here"
     }
     ```

3. **Refresh Token (`/refresh_token`)**:

   - **POST** request to refresh the `access_token` using a valid `refresh_token`.
   - **Request**:

     ```json
     {
       "refresh_token": "your_refresh_token_here"
     }
     ```

   - **Response**:

     ```json
     {
       "access_token": "new_access_token_here",
       "refresh_token": "new_refresh_token_here"
     }
     ```

### Directory Structure

```plaintext
auth-rs/
├── Cargo.toml        # Project dependencies
├── config/
│   └── services.toml # Configuration file for SurrealDB and JWT
├── run_db.sh         # Script to start SurrealDB
└── src/
    ├── database/
    │   └── user.rs   # User database operations
    ├── error/
    │   └── error_code.rs # Error handling
    ├── handlers/
    │   └── auth.rs   # Authentication logic (signup, login, refresh token)
    ├── main.rs       # Main entry point to start the server
    ├── models/
    │   └── auth.rs   # Data models (login, signup)
    ├── routes/
    │   └── auth.rs   # Route handlers for auth-related endpoints
    └── settings.rs    # Application settings and configuration loading
```

### Running the Database

To set up SurrealDB, you can use the provided `run_db.sh` script to start the database with the correct configuration.

```bash
bash run_db.sh
```

### Troubleshooting

- **Database connection errors**: Ensure that SurrealDB is running at `localhost:8000` and that the credentials in `config/services.toml` are correct.
- **Token generation issues**: Double-check the JWT secrets and configuration in `config/services.toml`.
