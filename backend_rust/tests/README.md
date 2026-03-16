# Testing Guide

This directory contains comprehensive tests for the Social Network Backend.

## Test Structure

```
tests/
├── README.md                 # This file
├── common/                   # Shared test utilities
│   └── mod.rs               # Test helpers and fixtures
├── integration_tests.rs     # Integration tests for API endpoints
└── unit_tests.rs            # Entry point for unit tests
    └── unit/
        ├── error_tests.rs   # Error handling tests
        ├── models_tests.rs  # Data model tests
        ├── service_tests.rs # Service layer validation tests
        └── utils_tests.rs   # Utility function tests
```

## Running Tests

### Run All Tests

```bash
cargo test
```

### Run Only Unit Tests

```bash
cargo test --test unit_tests
```

### Run Only Integration Tests

```bash
cargo test --test integration_tests
```

### Run Tests with Output

```bash
cargo test -- --nocapture
```

### Run Specific Test

```bash
cargo test test_validate_username_valid
```

## Test Categories

### Unit Tests

Unit tests focus on individual components and don't require a database:

- **Utils Tests**: JWT token generation/validation, password hashing, input validators
- **Models Tests**: Data model serialization/deserialization, pagination
- **Service Tests**: Business logic validation (content length checks, slug generation)
- **Error Tests**: Error type conversions and HTTP status code mapping

### Integration Tests

Integration tests verify the interaction between components:

- **API Tests**: HTTP endpoint testing (requires database)
- **Auth Flow Tests**: Complete authentication workflows

## Setting Up for Integration Tests

Integration tests require a running PostgreSQL database:

1. Create a test database:
   ```bash
   createdb social_network_test
   ```

2. Set the environment variable:
   ```bash
   export DATABASE_URL="postgres://user:password@localhost:5432/social_network_test"
   ```

3. Run migrations:
   ```bash
   cargo sqlx migrate run
   ```

## Writing New Tests

### Unit Test Example

```rust
#[test]
fn test_my_feature() {
    // Arrange
    let input = "test";
    
    // Act
    let result = my_function(input);
    
    // Assert
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "expected");
}
```

### Async Test Example

```rust
#[actix_rt::test]
async fn test_async_feature() {
    let pool = create_test_pool().await.unwrap();
    let result = my_async_function(&pool).await;
    assert!(result.is_ok());
}
```

## Test Coverage

Current test coverage includes:

- ✅ JWT token generation and validation
- ✅ Password hashing and verification
- ✅ Username/password validation
- ✅ Error handling and conversions
- ✅ Pagination logic
- ✅ Model serialization
- ✅ Service validation logic
- ✅ Slug generation

## Best Practices

1. **Keep tests isolated**: Each test should be independent
2. **Use descriptive names**: `test_validate_username_too_short` is better than `test_1`
3. **Test edge cases**: Empty strings, max lengths, special characters
4. **Use common utilities**: Put shared setup code in `tests/common/mod.rs`
5. **Document test purpose**: Add comments for complex test scenarios
