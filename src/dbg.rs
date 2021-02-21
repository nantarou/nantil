pub trait AllTypes<T> {
    // 型名を文字列で返す
    fn type_name( &self ) -> String {
        std::any::type_name::<Self>().to_string()
    }
}
impl<T> AllTypes<T> for T {}

#[test]
fn test_all_types_type_name() {
    let a : u8 = 0;
    assert_eq!( a.type_name(), "u8" );

    // 消費しない
    let s : String = "don".to_string();
    assert_eq!( s.type_name(), "alloc::string::String" );
    assert_eq!( s, "don" );
}
