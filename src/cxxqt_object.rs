/// The bridge definition for our QObject
#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "RustQt" {
        // The QObject definition
        // We tell CXX-Qt that we want a QObject class with the name MyObject
        // based on the Rust struct MyObjectRust.
        #[qobject]
        #[qml_element]
        type TestObject = super::TestRustObject;
    }

    unsafe extern "RustQt" {
        // Declare the invokable methods we want to expose on the QObject
        #[qinvokable]
        fn test_button(self: &TestObject);
    }
}

/// The Rust struct for the QObject
#[derive(Default)]
pub struct TestRustObject {}

impl qobject::TestObject {
    /// Print a log message
    pub fn test_button(&self) {
        println!("It works!");
    }
}
