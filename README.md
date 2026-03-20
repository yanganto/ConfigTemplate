# ConfigTemplate

A flexible configuration management approach using layered patches from multiple sources.

## Overview

ConfigTemplate demonstrates a clean, maintainable approach to handling complex configuration in Rust applications. It uses the **patch pattern** to layer configuration from multiple sources, allowing flexible overriding without tightly coupling to specific serialization formats, and it is flexible to use any deserialized tools.

### The Problem

When building Rust applications, configuration typically comes from multiple sources:
- Default values in code
- YAML/JSON/TOML config files
- Environment variables

Traditional approaches often mix deserialization with application logic, making code verbose and hard to maintain. Adding new configuration sources becomes increasingly difficult as the application grows.
This approach also makes it easy to work with multiple config files (e.g., `some.yaml`, `some.custom.yaml`).

### The Solution

This project uses [`struct-patch`](https://crates.io/crates/struct-patch) to decouple "patching" from "deserialization". Each configuration source is deserialized into a patch, then applied to the existing config. This creates a clean, composable architecture:

```
Default Values → YAML → JSON → TOML → Environment Variables
     (base)      ↑      ↑       ↑            ↑
                patch  patch   patch        patch
```

## Features

- **Layered Configuration** - Merge configs from multiple sources in priority order
- **Format Agnostic** - Support any serialization format (YAML, JSON, TOML, etc.)
- **Environment Variables** - Native support for env-based configuration
- **Type-Safe** - Full type safety with Rust's type system
- **Nested Structures** - Handle complex nested configurations with ease

## Demo Configuration Patch Flow

In the testcase, the configuration is built by layering patches from different sources in order:

1. **Default Values** - Start with sensible defaults in code
2. **YAML** - Override with YAML configuration file
3. **JSON** - Override with JSON configuration file
4. **TOML** - Override with TOML configuration file
5. **Environment Variables** - Final overrides from environment

Each layer only needs to specify values that change. Missing values are inherited from previous layers.

## Why This Approach?

1. **Maintainable** - Each config source is independent and testable
2. **Flexible** - Easily add or remove configuration sources
3. **Explicit** - Clear override order makes behavior predictable
4. **Testable** - Each patch can be tested in isolation

### Quick Example

```rust
let mut config = Config::default();
config.apply(serde_saphyr::from_str(yaml_content)?);                              // Apply YAML config
config.apply(serde_json::from_str(json_content)?);                                // Apply JSON config (overrides YAML)
config.apply(envious::Config::default().with_prefix("MY_").build_from_env()?);    // Apply env vars (highest priority)
```

This flow is clear and composable
