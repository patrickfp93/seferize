# 📜 SEFERIZE

> *"Revealing the written form of your Rust code."* ✨

A **procedural macro** that converts any Rust item (`struct`, `enum`, `trait`, `impl`, `fields`, `variants`, `methods`, etc.) into its **string representation** at compile time.

It can **automatically generate** a `&'static str` constant with the textual content of the item — useful for **reflection**, **documentation generation**, **code generation tests**,**code introspection**, or **debugging macro systems**.

---

## 🧩 Features

* ✅ Converts entire Rust items (structs, traits, impls, enums, fields, variants, methods, etc.) into strings.
* ✅ Removes self-invoking macros from within the processed item.
* ✅ Supports the attribute `#[ignore]` to **exclude specific items or blocks** from being stringified.
* 🧱 Optionally accepts a custom name for the generated string constant.
* 🔁 Supports **stringification of inner elements** (fields, variants, or methods) using `#[stringify]`.
* 💬 Accepts both **quoted and unquoted identifiers**: `#[stringify("TEXT")]` or `#[stringify(TEXT)]`.
* ⚡ Works at **compile time** — no runtime cost.
* 🕊️ 100% safe and pure Rust.
* 🧪 **New:** `#[expose_for_tests]` attribute to generate a **public version of private functions only for tests**, keeping the original function private.

---

## 🧠 How it works

When you annotate an item with `#[stringify]`, it generates a `&'static str` constant containing the **exact Rust source** of that item.
By default, the generated constant name follows the pattern `CODE_<ITEM_NAME>` unless another name is provided.

### Example

```rust
use seferize::stringify;

#[stringify]
pub struct User {
    id: u32,
    name: String,
}
```

Generates:

```rust
pub const CODE_USER: &str = "pub struct User { id: u32, name: String }";
```

---

## 🧱 Using inside structs, enums, and impl blocks

You can apply `#[stringify]` directly to **fields**, **enum variants**, or **methods** inside an item annotated with `#[stringify]`.
Each annotated element will generate its own `&'static str` constant alongside the main one.

### Example — inside a struct

```rust
use seferize::stringify;

#[stringify]
pub struct ExtractStruct {
    pub field_1: usize,
    #[stringify("FIELD_2")]
    pub field_2: String,
}
```

Produces:

```rust
pub const CODE_EXTRACT_STRUCT: &'static str =
    "pub struct ExtractStruct { pub field_1: usize, pub field_2: String, }";

pub const FIELD_2: &'static str = "pub field_2: String";

pub struct ExtractStruct {
    pub field_1: usize,
    pub field_2: String,
}
```

The same logic applies to **enum variants** and **impl methods**.

---

## 🧪 Exposing private functions for tests

Use the **`#[expose_for_tests]`** attribute to generate a **public version of a private function** that is only available in test builds (`#[cfg(test)]`).
The original function stays private and encapsulated.

### Example

```rust
use seferize::expose_for_tests;

struct MyStruct;

impl MyStruct {
    #[expose_for_tests]
    fn hidden(&self) -> i32 {
        42
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_call_hidden() {
        let s = MyStruct;
        // Call the public test version
        assert_eq!(s.test_hidden(), 42);
    }
}
```

---

## 💬 Attribute flexibility

The `#[stringify]` attribute accepts both **quoted and unquoted identifiers** for naming the generated constant:

```rust
#[stringify("CUSTOM_NAME")]
// or
#[stringify(CUSTOM_NAME)]
```

Both produce the same result:

```rust
pub const CUSTOM_NAME: &str = "...";
```

---

## 🚫 Ignoring code with `#[ignore]`

Mark any **item, block, or function** with `#[ignore]` to prevent it from being included in the generated string.

### Example — ignoring items

```rust
use seferize::stringify;

#[stringify]
mod my_module {
    pub struct Visible {
        field: i32,
    }

    #[ignore]
    pub fn hidden_fn() {
        println!("This function won't appear in CODE_MY_MODULE");
    }
}
```

Generates:

```rust
pub const CODE_MY_MODULE: &str = r#"
mod my_module {
    pub struct Visible {
        field: i32,
    }
}
"#;
```

---

## ⚙️ Advanced usage

### Custom constant name

Override the generated name by passing a custom identifier:

```rust
use seferize::stringify;

#[stringify("CUSTOM_NAME")]
pub enum Event {
    Created,
    Updated,
}
```

Produces:

```rust
pub const CUSTOM_NAME: &str = "pub enum Event { Created, Updated }";
```

---

## 🪶 Internal behavior

* Macros like `seferize::stringify`, `stringify`, `seferize::ignore`, and `ignore` are removed before string generation.
* Works recursively: modules, impls, or nested structures with ignored items are filtered out.
* Inner attributes marked with `#[stringify]` produce **individual constants**.
* `#[stringify(TEXT)]` and `#[stringify("TEXT")]` are both supported.
* Uses the Rust `syn` and `quote` crates for parsing and regenerating code.
* `#[expose_for_tests]` clones the original function, makes it `pub` and wraps it in `#[cfg(test)]`.

---

## 📦 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
seferize = "1.5.6"
```

---

## 📖 Example project structure

```
src/
├── main.rs
├── lib.rs
└── samples/
    ├── user.rs
    └── example.rs
```

---

## 🙌 Inspiration

The name **Seferize** comes from the Hebrew word *“Sefer”* (ספר), meaning *“book”* or *“scroll”*, symbolizing the act of **revealing written words**.

> *“The entrance of Thy words giveth light.”* — **Psalm 119:130**
