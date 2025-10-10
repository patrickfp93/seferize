# 📜 SEFERIZE

> *"Revealing the written form of your Rust code."* ✨

A **procedural macro** that converts any Rust item (`struct`, `enum`, `trait`, `impl`, etc.) into its **string representation** at compile time.

It can **automatically generate** a `&'static str` constant with the textual content of the item — useful for **reflection**, **documentation generation**, **code introspection**, or **debugging macro systems**.

---

## 🧩 Features

- ✅ Converts entire Rust items (structs, traits, impls, enums, etc.) into strings.  
- 🧱 Optionally accepts a custom name for the generated string constant.  
- ⚡ Works at **compile time** — no runtime cost.  
- 💡 Easy integration with tools that require code serialization, logging, or reflection.  
- 🕊️ 100% safe and pure Rust.

---

## 📦 Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
seferize = "1.0.0"
