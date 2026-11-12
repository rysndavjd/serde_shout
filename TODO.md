### Serialization: 
  - dotenv format for docker/python-dotenv
  - create tests that dont require allocation

### Deserialization:
  - Implement array support
  - Check parse_value when `#[cfg(not(feature = "alloc"))]` as I think it can
    mistake input by splitting too early.
