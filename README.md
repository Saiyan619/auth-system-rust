# Rust Auth Backend

A professional, production-ready authentication service built with the **Rust ecosystem**. This project implements a secure and robust user management system using **Axum**, **PostgreSQL**, and **JWT**, following modern architectural patterns and best practices.

## Overview

This backend provides a complete user lifecycle management system, including secure registration, email verification, session handling via JWT, and password recovery. It is designed with a focus on performance, security, and clean code principles.

### Key Features

- **Secure Authentication**: JWT-based authentication with support for both `HttpOnly` cookies and `Authorization` headers.
- **User Lifecycle**: Registration, email verification, login/logout, and password reset flows.
- **Robust Security**: Password hashing using the **Argon2** algorithm.
- **Mailing System**: Integrated SMTP mailer with dynamic HTML templates for verification and welcome emails.
- **Clean Architecture**:
  - **Trait-based database abstraction** (Repository pattern).
  - **Modular routing** for scalability.
  - **Custom Middleware** for role-based access control (RBAC).
  - **DTO Pattern** for validated data transfer.

---

## Tech Stack

- **Language**: [Rust](https://www.rust-lang.org/)
- **Web Framework**: [Axum](https://github.com/tokio-rs/axum) (Layered on top of Tokio/Hyper)
- **Runtime**: [Tokio](https://tokio.rs/)
- **Database**: [PostgreSQL](https://www.postgresql.org/) with [SQLx](https://github.com/launchbadge/sqlx) (Async, compile-time checked queries)
- **Security**:
  - `argon2`: Password hashing.
  - `jsonwebtoken`: Stateless session management.
- **Data Validation**: `validator` crate for strict input sanitization.
- **Email**: `lettre` for SMTP delivery.

---

## Project Structure

```text
src/
├── handler/      # Request handlers (Business logic)
├── mail/         # Email service and HTML templates
├── utils/        # Cryptography, JWT, and common helpers
├── db.rs         # Database client and Repository traits
├── middleware.rs # JWT Authentication & Guard layers
├── models.rs     # Database entity definitions
├── dtos.rs       # Data Transfer Objects & Validation
└── route.rs      # API route definitions
└── config.rs      # Secret key definitions
└── error.rs      # Error Message definitions
```

---

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.70+)
- [Docker](https://www.docker.com/) (Optional, for database)
- PostgreSQL

### Installation

1. **Clone the repository**:
   ```bash
   git clone https://github.com/your-username/auth-backend.git
   cd auth-backend
   ```

2. **Configure Environment Variables**:
   Create a `.env` file in the root directory:
   ```env
   DATABASE_URL=postgres://user:password@localhost:5432/auth_db
   JWT_SECRET=your_jwt_secret
   SMTP_HOST=smtp.gmail.com
   SMTP_USER=your_email@gmail.com
   SMTP_PASS=your_app_password
   ```

3. **Run Migrations**:
   ```bash
   sqlx migrate run
   ```

4. **Run the Application**:
   ```bash
   cargo run
   ```

---

## API Endpoints

### Authentication (`/api/auth`)
- `POST /register`: Create a new user account.
- `GET /verify-email/:token`: Verify user email via token.
- `POST /login`: Authenticate and receive a JWT.
- `GET /logout`: Invalidate session (removes token).
- `POST /forgot-password`: Request a password reset link.
- `PATCH /reset-password/:token`: Update password using reset token.

### User (`/api/user`)
- `GET /me`: Retrieve details of the currently authenticated user (Protected).

---
