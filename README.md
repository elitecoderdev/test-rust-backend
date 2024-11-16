# Rust API using Warp - Loan Management System

## Table of Contents
1. [Requirements](#requirements)
2. [Installation and Setup](#installation-and-setup)
3. [API Endpoints](#api-endpoints)
4. [Running the API](#running-the-api)
5. [Postman Integration](#postman-integration)
6. [Version Recap](#version-recap)

## Requirements
- **Rust Version:** 0.1.0
- **Cargo (Rust's Package Manager):** Included with Rust

## Installation and Setup

Follow these steps to install the project locally:

1. **Install Rust and Cargo:**
   - Install Rust by following the instructions on the official [Rust Installation page](https://www.rust-lang.org/tools/install).
   - Cargo will be installed automatically with Rust.

2. **Install Rust and Cargo For Linux:**
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
3. **Configure your current shell to include Rust's tools by running:**
    ```bash
    source $HOME/.cargo/env
    ```
4. **Clone the repository:**
   ```bash
   git clone https://github.com/elitecoderdev/test-rust-backend.git
   ```

5. **Navigate to the project directory:**
   ```bash
   cd test-rust-backend
   ```


6. **Run the API (see Running the API section below):**
   ```bash
   cargo run
   ``` 


This setup will compile and run your API locally for development and testing purposes.

## API Endpoints

The API consists of the following endpoints to manage portfolios, apply for loans, and check loan statuses:

| Method | Endpoint             | Description                                        |
|--------|----------------------|----------------------------------------------------|
| POST   | `/portfolio`         | Submits a new portfolio                            |
| POST   | `/apply-loan`        | Applies for a new loan                             |
| GET    | `/loan-status/{id}`  | Retrieves the status of a loan by its unique ID    |


## Running the API

To run the API, use the following command after building the project:

```bash
cargo run
```

This command will start the server and make the API available at `http://localhost:8000` (assuming the default Warp port).


## Postman Integration

To use Postman to interact with the API.

### 1. Retrieve Loan Status

#### Postman Request
- **Method**: GET
- **URL**: `http://localhost:8000/loan-status/CLIENT001`

#### cURL Command
```bash
curl --location 'http://localhost:8000/loan-status/CLIENT001'
```

### 2. Apply for a Loan

#### Postman Request
- **Method**: POST
- **URL**: `http://localhost:8000/apply-loan`
- **Headers**:
  - Content-Type: application/json
- **Body (JSON)**:
```json
{
    "client_id": "CLIENT001",
    "requested_amount": 30000
}
```

#### cURL Command
```bash
curl --location 'http://localhost:8000/apply-loan' \
--header 'Content-Type: application/json' \
--data '{"client_id": "CLIENT001", "requested_amount": 30000}'
```

### 3. Submit Portfolio

#### Postman Request
- **Method**: POST
- **URL**: `http://localhost:8000/portfolio`
- **Headers**:
  - Content-Type: multipart/form-data
- **Body**:
  - **Key**: `file`
  - **Type**: File
  - **Value**: Upload the file `portafolio_cliente_empresa_a.xlsx` from your local system.

#### How to Upload a File in Postman:
1. In the "Body" tab, select "form-data".
2. Enter the key as "file", set the type to File.
3. Use the file picker to select `portafolio_cliente_empresa_a.xlsx` from your local directory.

#### cURL Command
```bash
curl --location 'http://localhost:8000/portfolio' \
--header 'Content-Type: multipart/form-data' \
--form 'file=@"/path/to/your/portafolio_cliente_empresa_a.xlsx"'
```
#### Important Notes for cURL:
- Replace `"/path/to/your/portafolio_cliente_empresa_a.xlsx"` with the actual path to the file you want to upload.



## Version Recap
- **Rust Version:** 0.1.0
- **Cargo (Rust's Package Manager):** Included with Rust
---