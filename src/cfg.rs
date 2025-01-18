#[tokio::test]
async fn test() {
    use crate::*;
    let string: String = String::from("test");
    let number: i32 = 1;
    let func = async_func!(string, number, |data| {
        let tmp_string: String = String::from("test");
        assert_eq!(string, tmp_string);
        assert_eq!(data, 1);
        assert_eq!(number, 1);
    });
    func(1).await;
}
