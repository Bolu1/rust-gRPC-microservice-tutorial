# Rust Microservice Tutorial

This repository contains a tutorial on setting up a simple microservice architecture using Rust, gRPC, and Actix Web. The project demonstrates how to build a task manager application with a gRPC service for handling tasks and an HTTP API gateway for client interaction.

## Project Structure

```
rust-microservice-tutorial/
├── api-gateway/
│   └── ...
├── grpc-service/
│   └── ...
└── task-service/
    └── ...
```

### api-gateway

This project contains the HTTP API gateway built with Actix Web. It serves as the entry point for client requests and interacts with the task service via gRPC.

### grpc-service

This project contains the `.proto` files that define the gRPC service interfaces. It is packaged as a library and can be accessed from GitHub by the other services.

### task-service

This project contains the task service implemented using Rust and gRPC with Tonic. It manages a PostgreSQL database using Sqlx and provides gRPC functions for CRUD operations on tasks.

## How It Works

The task manager application is designed with the following flow:
1. A client sends an HTTP request to the API gateway.
2. The API gateway, built with Actix Web, receives the request and translates it into a gRPC call.
3. The gRPC call is sent to the task service, which interacts with a PostgreSQL database to perform the necessary operations (Create, Read, Update, Delete).
4. The task service responds with the result of the operation via gRPC.
5. The API gateway receives the gRPC response and sends an HTTP response back to the client.

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo (Rust package manager)
- PostgreSQL (for the task service database)

### Setup

1. **Clone the repository**:
   ```bash
   git clone https://github.com/Bolu1/rust-gRPC-microservice-tutorial.git
   cd rust-gRPC-microservice-tutorial
   ```

2. **Setup PostgreSQL**:
   Ensure you have PostgreSQL installed and running. Create a new database for the task service.

3. **Setup gRPC Service**:
   Navigate to the `grpc-service` directory and publish the package to your local or remote repository if needed.

4. **Setup Task Service**:
   Navigate to the `task-service` directory:
   ```bash
   cd task-service
   ```
   Add the necessary dependencies:
   ```bash
   cargo add tonic prost
   cargo add tonic-build --build
   ```
   Update the `Cargo.toml` to include the gRPC service package from GitHub.

   Configure the database connection settings in the `.env` file.

5. **Setup API Gateway**:
   Navigate to the `api-gateway` directory:
   ```bash
   cd api-gateway
   ```
   Add the necessary dependencies for Actix Web and gRPC:
   ```bash
   cargo add actix-web tonic
   ```
   Update the `Cargo.toml` to include the gRPC service package from GitHub.

### Running the Application

1. **Run the Task Service**:
   ```bash
   cd task-service
   cargo run
   ```

2. **Run the API Gateway**:
   ```bash
   cd api-gateway
   cargo run
   ```

### Usage

- Send HTTP requests to the API gateway to interact with the task service.
- The API gateway will handle the gRPC communication with the task service and return the appropriate responses.

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue if you have any suggestions or improvements.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

By following this tutorial, you will learn how to set up and run a microservice architecture in Rust, utilizing gRPC for efficient service communication and Actix Web for an HTTP API gateway.