fn refmut_string(s: &mut String) {
    // ここでsに対して変更を加えるなどの操作も可能。
    println!("{}", s);
}

fn main() {
    let mut s = "this is a resource".to_string();
    // ミュータブルな参照1つめ。
    let t = &mut s;
    // ミュータブルな参照2つめはエラー。
    // refmut_string(&s); // error[E0499]: cannot borrow `s` as mutable more than once at a time
    println!("{}", t)
}
