# Migration Guide: Bevy 0.16 to 0.17

This guide will help you migrate your async_bevy_web project from Bevy 0.16 to Bevy 0.17.

## Overview

The upgrade from Bevy 0.16 to 0.17 is **straightforward** for async_bevy_web users. The library's minimal use of Bevy APIs means most applications will require **no code changes**.

## Breaking Changes Summary

### ✅ What DOESN'T Affect async_bevy_web

The most significant breaking change in Bevy 0.17 is the **Event → Message API split**. However, async_bevy_web and its core crates (`bevy-tokio-tasks`, `bevy-leptos`) **do not use Bevy's Event system**, so this change does not affect the library or your applications.

### 🔧 What You Need to Change

#### 1. Rust Toolchain (REQUIRED)

Bevy 0.17 requires **Rust nightly (1.88.0 or later)** due to its use of Rust Edition 2024 features.

**Action Required:**

The project already includes an updated `rust-toolchain.toml` file. Simply run:

```bash
rustup update
```

The toolchain will automatically switch to nightly when you work in the project directory.

#### 2. Update Dependencies

Update your `Cargo.toml` to use the new versions:

```toml
[dependencies]
async-bevy-web = "0.3.0"  # Now uses Bevy 0.17
bevy = "0.17.0"            # If you depend on Bevy directly
```

Then run:

```bash
cargo update
```

## Step-by-Step Migration

### For Library Users

If you're using async_bevy_web in your project:

1. **Update Rust toolchain:**
   ```bash
   rustup update
   ```

2. **Update your `Cargo.toml`:**
   ```toml
   [dependencies]
   async-bevy-web = "0.3.0"
   ```

3. **Update dependencies:**
   ```bash
   cargo update
   ```

4. **Build your project:**
   ```bash
   cargo build
   ```

5. **Test your application:**
   ```bash
   cargo run
   ```

That's it! Your code should work without any changes.

### For Contributors

If you're contributing to async_bevy_web:

1. **Pull the latest changes:**
   ```bash
   git pull origin main
   ```

2. **Update Rust toolchain:**
   ```bash
   rustup update
   ```

3. **Build the workspace:**
   ```bash
   cargo build --workspace
   ```

4. **Run tests (if any):**
   ```bash
   cargo test --workspace
   ```

## Code Changes

### No Changes Required

The following async_bevy_web APIs remain **unchanged**:

- ✅ `ABWConfigPlugin::new(frame_rate)`
- ✅ `ABWConfigPlugin::default()`
- ✅ `TokioTasksPlugin`
- ✅ `TokioTasksRuntime`
- ✅ `LeptosAppPlugin`
- ✅ All Bevy APIs used by the library (Plugin, App, Resource, Systems, Schedules)

### Example Code

Your existing code will continue to work:

```rust
use async_bevy_web::prelude::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(ABWConfigPlugin::new(60.0))
        .add_plugins(LeptosAppPlugin::new(my_leptos_app()))
        .add_systems(Update, my_system)
        .run();
}

fn my_system(runtime: Res<TokioTasksRuntime>) {
    // Your code here - no changes needed!
}
```

## Known Issues

### Leptos Compatibility

Some older versions of Leptos (e.g., 0.6.x) may have compatibility issues with Rust nightly. If you encounter compilation errors in Leptos macros, consider:

1. **Upgrading Leptos** to the latest version (0.8.x or later)
2. **Pinning a specific nightly version** that's known to work with your Leptos version

### Example Applications

The example applications in the repository may need updates to work with the latest dependencies. These are being updated separately and do not affect the core library functionality.

## Troubleshooting

### "error: package requires rustc 1.88.0 or newer"

**Solution:** Update your Rust toolchain:
```bash
rustup update
```

### "error: edition 2024 is unstable"

**Solution:** Ensure you're using Rust nightly:
```bash
rustup default nightly
# or
rustup override set nightly
```

### Compilation errors in dependencies

**Solution:** Update all dependencies:
```bash
cargo update
cargo clean
cargo build
```

## Bevy 0.17 Resources

For more information about Bevy 0.17:

- [Bevy 0.16 to 0.17 Migration Guide](https://bevyengine.org/learn/migration-guides/0-16-to-0-17/)
- [Bevy 0.17 Release Notes](https://bevyengine.org/news/bevy-0-17/)
- [Bevy Discord](https://discord.gg/bevy) - Get help from the community

## Rust Edition 2024 Resources

- [Rust Edition 2024 Guide](https://doc.rust-lang.org/edition-guide/rust-2024/index.html)
- [Rust Blog: Edition 2024](https://blog.rust-lang.org/2024/10/17/Rust-2024.html)

## Need Help?

If you encounter issues during migration:

1. Check the [GitHub Issues](https://github.com/vertec-io/async_bevy_web/issues)
2. Open a new issue with:
   - Your Rust version (`rustc --version`)
   - Your Cargo.toml dependencies
   - The full error message
   - Steps to reproduce

## Summary

✅ **No code changes required** for most users  
✅ **Rust nightly required** (automatic via rust-toolchain.toml)  
✅ **Update dependencies** with `cargo update`  
✅ **All async_bevy_web APIs unchanged**  

The migration is straightforward and should take less than 5 minutes for most projects!

