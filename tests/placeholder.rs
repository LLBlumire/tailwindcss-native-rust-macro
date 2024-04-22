use tailwindcss_native_rust_macro::include_tailwind;

#[test]
fn basic() {
    let tailwind = include_tailwind! {
        config: "tests/tailwind.config.js",
        input: "tests/input.css",
    };
    assert!(tailwind.contains("bg-blue-500")); // it will see the string here, and add it to the file
    assert!(!tailwind.contains(concat!("bg-red", "-500"))); // concat so it doesn't see the string
}
