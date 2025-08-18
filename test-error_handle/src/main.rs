fn main() {
    // 可恢复错误
    test_error_handle::test_result::demo_result();

    // 可恢复错误
    // 如果想使一个可恢复错误按不可恢复错误处理，Result 类提供了两个办法：unwrap() 和 expect(message: &str)
    let file_name = "test-error_handle.d";
    let f1 = test_error_handle::test_panic::demo_result(file_name).unwrap();
    println!("文件内容:【{}】", f1);
    let f2 = test_error_handle::test_panic::demo_result(file_name).expect("打开失败~~~!");
    println!("文件内容:【{}】", f2);

    // 不可恢复错误
    test_error_handle::test_panic::demo_panic();
}
