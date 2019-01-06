// Rust test file autogenerated with cargo build (build/spectests.rs).
// Please do NOT modify it by hand, as it will be reseted on next build.
// Test based on spectests/elem.wast
#![allow(
    warnings,
    dead_code
)]
use wabt::wat2wasm;

use crate::runtime::types::Value;
use crate::webassembly::{compile, instantiate, ImportObject, Instance, ResultObject};

use super::_common::{spectest_importobject, NaNCheck};

// Line 4
fn create_module_1() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (func (;0;) (type 0))
      (table (;0;) 10 anyfunc)
      (elem (;0;) (i32.const 0))
      (elem (;1;) (i32.const 0) 0 0)
      (elem (;2;) (i32.const 0))
      (elem (;3;) (i32.const 0) 0 0)
      (elem (;4;) (i32.const 0))
      (elem (;5;) (i32.const 0) 0 0)
      (elem (;6;) (i32.const 0))
      (elem (;7;) (i32.const 0) 0 0)
      (elem (;8;) (i32.const 0))
      (elem (;9;) (i32.const 0) 0 0)
      (elem (;10;) (i32.const 0))
      (elem (;11;) (i32.const 0) 0 0))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(&wasm_binary[..], &spectest_importobject(), None)
        .expect("WASM can't be instantiated")
}

fn start_module_1(result_object: &mut ResultObject) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //result_object.instance.start();
}

// Line 23

#[test]
fn test_module_1() {
    let mut result_object = create_module_1();
    // We group the calls together
    start_module_1(&mut result_object);
}
fn create_module_2() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (func (;0;) (type 0))
      (table (;0;) 10 anyfunc)
      (elem (;0;) (i32.const 0) 0))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(&wasm_binary[..], &spectest_importobject(), None)
        .expect("WASM can't be instantiated")
}

fn start_module_2(result_object: &mut ResultObject) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //result_object.instance.start();
}

// Line 28

#[test]
fn test_module_2() {
    let mut result_object = create_module_2();
    // We group the calls together
    start_module_2(&mut result_object);
}
fn create_module_3() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (import \"spectest\" \"table\" (table (;0;) 10 anyfunc))
      (func (;0;) (type 0))
      (elem (;0;) (i32.const 0) 0))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(&wasm_binary[..], &spectest_importobject(), None)
        .expect("WASM can't be instantiated")
}

fn start_module_3(result_object: &mut ResultObject) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //result_object.instance.start();
}

// Line 34

#[test]
fn test_module_3() {
    let mut result_object = create_module_3();
    // We group the calls together
    start_module_3(&mut result_object);
}
fn create_module_4() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (func (;0;) (type 0))
      (table (;0;) 10 anyfunc)
      (elem (;0;) (i32.const 0) 0)
      (elem (;1;) (i32.const 3) 0)
      (elem (;2;) (i32.const 7) 0)
      (elem (;3;) (i32.const 5) 0)
      (elem (;4;) (i32.const 3) 0))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(&wasm_binary[..], &spectest_importobject(), None)
        .expect("WASM can't be instantiated")
}

fn start_module_4(result_object: &mut ResultObject) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //result_object.instance.start();
}

// Line 43

#[test]
fn test_module_4() {
    let mut result_object = create_module_4();
    // We group the calls together
    start_module_4(&mut result_object);
}
fn create_module_5() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (import \"spectest\" \"table\" (table (;0;) 10 anyfunc))
      (func (;0;) (type 0))
      (elem (;0;) (i32.const 9) 0)
      (elem (;1;) (i32.const 3) 0)
      (elem (;2;) (i32.const 7) 0)
      (elem (;3;) (i32.const 3) 0)
      (elem (;4;) (i32.const 5) 0))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(&wasm_binary[..], &spectest_importobject(), None)
        .expect("WASM can't be instantiated")
}

fn start_module_5(result_object: &mut ResultObject) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //result_object.instance.start();
}

// Line 53

#[test]
fn test_module_5() {
    let mut result_object = create_module_5();
    // We group the calls together
    start_module_5(&mut result_object);
}
fn create_module_6() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (import \"spectest\" \"global_i32\" (global (;0;) i32))
      (func (;0;) (type 0))
      (table (;0;) 1000 anyfunc)
      (elem (;0;) (i32.const 0) 0))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(&wasm_binary[..], &spectest_importobject(), None)
        .expect("WASM can't be instantiated")
}

fn start_module_6(result_object: &mut ResultObject) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //result_object.instance.start();
}

// Line 60

#[test]
fn test_module_6() {
    let mut result_object = create_module_6();
    // We group the calls together
    start_module_6(&mut result_object);
}
fn create_module_7() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (import \"spectest\" \"global_i32\" (global (;0;) i32))
      (func (;0;) (type 0))
      (table (;0;) 1000 anyfunc)
      (elem (;0;) (get_global 0) 0))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(&wasm_binary[..], &spectest_importobject(), None)
        .expect("WASM can't be instantiated")
}

fn start_module_7(result_object: &mut ResultObject) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //result_object.instance.start();
}

// Line 67

#[test]
fn test_module_7() {
    let mut result_object = create_module_7();
    // We group the calls together
    start_module_7(&mut result_object);
}
fn create_module_8() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func (result i32)))
      (func (;0;) (type 0) (result i32)
        i32.const 65)
      (func (;1;) (type 0) (result i32)
        i32.const 66)
      (func (;2;) (type 0) (result i32)
        i32.const 7
        call_indirect (type 0))
      (func (;3;) (type 0) (result i32)
        i32.const 9
        call_indirect (type 0))
      (table (;0;) 10 anyfunc)
      (export \"call-7\" (func 2))
      (export \"call-9\" (func 3))
      (elem (;0;) (i32.const 7) 0)
      (elem (;1;) (i32.const 9) 1))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(&wasm_binary[..], &spectest_importobject(), None)
        .expect("WASM can't be instantiated")
}

fn start_module_8(result_object: &mut ResultObject) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //result_object.instance.start();
}

// Line 81
fn c8_l81_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c8_l81_action_invoke");
    let result = result_object
        .instance
        .call("call-7", &[])
        .expect("Missing result in c8_l81_action_invoke");
    assert_eq!(result, Some(Value::I32(65 as i32)));
}

// Line 82
fn c9_l82_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c9_l82_action_invoke");
    let result = result_object
        .instance
        .call("call-9", &[])
        .expect("Missing result in c9_l82_action_invoke");
    assert_eq!(result, Some(Value::I32(66 as i32)));
}

// Line 86

#[test]
fn test_module_8() {
    let mut result_object = create_module_8();
    // We group the calls together
    start_module_8(&mut result_object);
    c8_l81_action_invoke(&mut result_object);
    c9_l82_action_invoke(&mut result_object);
}
fn create_module_9() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (func (;0;) (type 0))
      (table (;0;) 10 anyfunc)
      (elem (;0;) (i32.const 9) 0))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(&wasm_binary[..], &spectest_importobject(), None)
        .expect("WASM can't be instantiated")
}

fn start_module_9(result_object: &mut ResultObject) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //result_object.instance.start();
}

// Line 91

#[test]
fn test_module_9() {
    let mut result_object = create_module_9();
    // We group the calls together
    start_module_9(&mut result_object);
}
fn create_module_10() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (import \"spectest\" \"table\" (table (;0;) 10 anyfunc))
      (func (;0;) (type 0))
      (elem (;0;) (i32.const 9) 0))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(&wasm_binary[..], &spectest_importobject(), None)
        .expect("WASM can't be instantiated")
}

fn start_module_10(result_object: &mut ResultObject) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //result_object.instance.start();
}

// Line 97

#[test]
fn test_module_10() {
    let mut result_object = create_module_10();
    // We group the calls together
    start_module_10(&mut result_object);
}
fn create_module_11() -> ResultObject {
    let module_str = "(module
      (table (;0;) 0 anyfunc)
      (elem (;0;) (i32.const 0)))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(&wasm_binary[..], &spectest_importobject(), None)
        .expect("WASM can't be instantiated")
}

fn start_module_11(result_object: &mut ResultObject) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //result_object.instance.start();
}

// Line 101

#[test]
fn test_module_11() {
    let mut result_object = create_module_11();
    // We group the calls together
    start_module_11(&mut result_object);
}
fn create_module_12() -> ResultObject {
    let module_str = "(module
      (import \"spectest\" \"table\" (table (;0;) 0 anyfunc))
      (elem (;0;) (i32.const 0)))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(&wasm_binary[..], &spectest_importobject(), None)
        .expect("WASM can't be instantiated")
}

fn start_module_12(result_object: &mut ResultObject) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //result_object.instance.start();
}

// Line 106

#[test]
fn test_module_12() {
    let mut result_object = create_module_12();
    // We group the calls together
    start_module_12(&mut result_object);
}
fn create_module_13() -> ResultObject {
    let module_str = "(module
      (table (;0;) 0 0 anyfunc)
      (elem (;0;) (i32.const 0)))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(&wasm_binary[..], &spectest_importobject(), None)
        .expect("WASM can't be instantiated")
}

fn start_module_13(result_object: &mut ResultObject) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //result_object.instance.start();
}

// Line 111

#[test]
fn test_module_13() {
    let mut result_object = create_module_13();
    // We group the calls together
    start_module_13(&mut result_object);
}
fn create_module_14() -> ResultObject {
    let module_str = "(module
      (table (;0;) 20 anyfunc)
      (elem (;0;) (i32.const 20)))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(&wasm_binary[..], &spectest_importobject(), None)
        .expect("WASM can't be instantiated")
}

fn start_module_14(result_object: &mut ResultObject) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //result_object.instance.start();
}

// Line 116

#[test]
fn test_module_14() {
    let mut result_object = create_module_14();
    // We group the calls together
    start_module_14(&mut result_object);
}
fn create_module_15() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (import \"spectest\" \"table\" (table (;0;) 0 anyfunc))
      (func (;0;) (type 0))
      (elem (;0;) (i32.const 0) 0))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(&wasm_binary[..], &spectest_importobject(), None)
        .expect("WASM can't be instantiated")
}

fn start_module_15(result_object: &mut ResultObject) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //result_object.instance.start();
}

// Line 122

#[test]
fn test_module_15() {
    let mut result_object = create_module_15();
    // We group the calls together
    start_module_15(&mut result_object);
}
fn create_module_16() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (import \"spectest\" \"table\" (table (;0;) 0 100 anyfunc))
      (func (;0;) (type 0))
      (elem (;0;) (i32.const 0) 0))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(&wasm_binary[..], &spectest_importobject(), None)
        .expect("WASM can't be instantiated")
}

fn start_module_16(result_object: &mut ResultObject) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //result_object.instance.start();
}

// Line 128

#[test]
fn test_module_16() {
    let mut result_object = create_module_16();
    // We group the calls together
    start_module_16(&mut result_object);
}
fn create_module_17() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (import \"spectest\" \"table\" (table (;0;) 0 anyfunc))
      (func (;0;) (type 0))
      (elem (;0;) (i32.const 1) 0))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(&wasm_binary[..], &spectest_importobject(), None)
        .expect("WASM can't be instantiated")
}

fn start_module_17(result_object: &mut ResultObject) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //result_object.instance.start();
}

// Line 134

#[test]
fn test_module_17() {
    let mut result_object = create_module_17();
    // We group the calls together
    start_module_17(&mut result_object);
}
fn create_module_18() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (import \"spectest\" \"table\" (table (;0;) 0 30 anyfunc))
      (func (;0;) (type 0))
      (elem (;0;) (i32.const 1) 0))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(&wasm_binary[..], &spectest_importobject(), None)
        .expect("WASM can't be instantiated")
}

fn start_module_18(result_object: &mut ResultObject) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //result_object.instance.start();
}

// Line 143

// Line 152

// Line 161

// Line 170

// Line 178

// Line 186

// Line 195

// Line 203

// Line 212

// Line 220

// Line 229

// Line 237

// Line 248
#[test]
fn c32_l248_assert_invalid() {
    let wasm_binary = [
        0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 9, 7, 1, 0, 65, 0, 11, 1, 0,
        10, 4, 1, 2, 0, 11,
    ];
    let compilation = compile(&wasm_binary.to_vec());
    assert!(
        compilation.is_err(),
        "WASM should not compile as is invalid"
    );
}

// Line 258
#[test]
fn c33_l258_assert_invalid() {
    let wasm_binary = [
        0, 97, 115, 109, 1, 0, 0, 0, 4, 4, 1, 112, 0, 1, 9, 6, 1, 0, 66, 0, 11, 0,
    ];
    let compilation = compile(&wasm_binary.to_vec());
    assert!(
        compilation.is_err(),
        "WASM should not compile as is invalid"
    );
}

// Line 266
#[test]
fn c34_l266_assert_invalid() {
    let wasm_binary = [
        0, 97, 115, 109, 1, 0, 0, 0, 4, 4, 1, 112, 0, 1, 9, 7, 1, 0, 65, 0, 104, 11, 0,
    ];
    let compilation = compile(&wasm_binary.to_vec());
    assert!(
        compilation.is_err(),
        "WASM should not compile as is invalid"
    );
}

// Line 274
#[test]
fn c35_l274_assert_invalid() {
    let wasm_binary = [
        0, 97, 115, 109, 1, 0, 0, 0, 4, 4, 1, 112, 0, 1, 9, 5, 1, 0, 1, 11, 0,
    ];
    let compilation = compile(&wasm_binary.to_vec());
    assert!(
        compilation.is_err(),
        "WASM should not compile as is invalid"
    );
}

// Line 282
#[test]
fn c36_l282_assert_invalid() {
    let wasm_binary = [
        0, 97, 115, 109, 1, 0, 0, 0, 4, 4, 1, 112, 0, 1, 9, 7, 1, 0, 1, 65, 0, 11, 0,
    ];
    let compilation = compile(&wasm_binary.to_vec());
    assert!(
        compilation.is_err(),
        "WASM should not compile as is invalid"
    );
}

// Line 290
#[test]
fn c37_l290_assert_invalid() {
    let wasm_binary = [
        0, 97, 115, 109, 1, 0, 0, 0, 4, 4, 1, 112, 0, 1, 9, 7, 1, 0, 65, 0, 1, 11, 0,
    ];
    let compilation = compile(&wasm_binary.to_vec());
    assert!(
        compilation.is_err(),
        "WASM should not compile as is invalid"
    );
}

// Line 305

#[test]
fn test_module_18() {
    let mut result_object = create_module_18();
    // We group the calls together
    start_module_18(&mut result_object);
}
fn create_module_19() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func (result i32)))
      (func (;0;) (type 0) (result i32)
        i32.const 65)
      (func (;1;) (type 0) (result i32)
        i32.const 66)
      (func (;2;) (type 0) (result i32)
        i32.const 9
        call_indirect (type 0))
      (table (;0;) 10 anyfunc)
      (export \"call-overwritten\" (func 2))
      (elem (;0;) (i32.const 9) 0)
      (elem (;1;) (i32.const 9) 1))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(&wasm_binary[..], &spectest_importobject(), None)
        .expect("WASM can't be instantiated")
}

fn start_module_19(result_object: &mut ResultObject) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //result_object.instance.start();
}

// Line 316
fn c39_l316_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c39_l316_action_invoke");
    let result = result_object
        .instance
        .call("call-overwritten", &[])
        .expect("Missing result in c39_l316_action_invoke");
    assert_eq!(result, Some(Value::I32(66 as i32)));
}

// Line 318

#[test]
fn test_module_19() {
    let mut result_object = create_module_19();
    // We group the calls together
    start_module_19(&mut result_object);
    c39_l316_action_invoke(&mut result_object);
}
fn create_module_20() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func (result i32)))
      (import \"spectest\" \"table\" (table (;0;) 10 anyfunc))
      (func (;0;) (type 0) (result i32)
        i32.const 65)
      (func (;1;) (type 0) (result i32)
        i32.const 66)
      (func (;2;) (type 0) (result i32)
        i32.const 9
        call_indirect (type 0))
      (export \"call-overwritten-element\" (func 2))
      (elem (;0;) (i32.const 9) 0)
      (elem (;1;) (i32.const 9) 1))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(&wasm_binary[..], &spectest_importobject(), None)
        .expect("WASM can't be instantiated")
}

fn start_module_20(result_object: &mut ResultObject) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //result_object.instance.start();
}

// Line 329
fn c41_l329_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c41_l329_action_invoke");
    let result = result_object
        .instance
        .call("call-overwritten-element", &[])
        .expect("Missing result in c41_l329_action_invoke");
    assert_eq!(result, Some(Value::I32(66 as i32)));
}

// Line 333

#[test]
fn test_module_20() {
    let mut result_object = create_module_20();
    // We group the calls together
    start_module_20(&mut result_object);
    c41_l329_action_invoke(&mut result_object);
}
fn create_module_21() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func (result i32)))
      (func (;0;) (type 0) (result i32)
        i32.const 65)
      (func (;1;) (type 0) (result i32)
        i32.const 66)
      (func (;2;) (type 0) (result i32)
        i32.const 7
        call_indirect (type 0))
      (func (;3;) (type 0) (result i32)
        i32.const 8
        call_indirect (type 0))
      (func (;4;) (type 0) (result i32)
        i32.const 9
        call_indirect (type 0))
      (table (;0;) 10 anyfunc)
      (export \"shared-table\" (table 0))
      (export \"call-7\" (func 2))
      (export \"call-8\" (func 3))
      (export \"call-9\" (func 4))
      (elem (;0;) (i32.const 8) 0)
      (elem (;1;) (i32.const 9) 1))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(&wasm_binary[..], &spectest_importobject(), None)
        .expect("WASM can't be instantiated")
}

fn start_module_21(result_object: &mut ResultObject) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //result_object.instance.start();
}

// Line 351

// Line 353
fn c44_l353_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c44_l353_action_invoke");
    let result = result_object
        .instance
        .call("call-7", &[])
        .expect("Missing result in c44_l353_action_invoke");
}

#[test]
fn c44_l353_assert_trap() {
    let mut result_object = create_module_21();
    let result = call_protected!(c44_l353_action_invoke(&mut result_object));
    assert!(result.is_err());
}

// Line 354
fn c45_l354_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c45_l354_action_invoke");
    let result = result_object
        .instance
        .call("call-8", &[])
        .expect("Missing result in c45_l354_action_invoke");
    assert_eq!(result, Some(Value::I32(65 as i32)));
}

// Line 355
fn c46_l355_action_invoke(result_object: &mut ResultObject) {
    println!("Executing function {}", "c46_l355_action_invoke");
    let result = result_object
        .instance
        .call("call-9", &[])
        .expect("Missing result in c46_l355_action_invoke");
    assert_eq!(result, Some(Value::I32(66 as i32)));
}

#[test]
fn test_module_21() {
    let mut result_object = create_module_21();
    // We group the calls together
    start_module_21(&mut result_object);
    c45_l354_action_invoke(&mut result_object);
    c46_l355_action_invoke(&mut result_object);
}
