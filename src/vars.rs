pub mod sub_a;
pub mod sub_b;

// 定数
const MAX_POINTS: u32 = 100_000; // メモリーの使用箇所

pub fn run() {
    println!("Here is vars module!");
    // sub_a::func_a();
    // sub_b::func_b();
    let mut x = 5; // Rustではすべての値は基本的にimmutable、:** で明示 or mut で mutable指定可能
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    // 型推論 値によるメモリの自動割当 _を変数前に置くことで変数非使用の警告を非表示にできる
    let _i1: i32 = 3;
    let _f1: f64 = 0.1;

    // 環境サイズの確認  {:p} &でメモリ番地
    println!("{}", usize::BITS);
    println!("Memory address of const is: {:p}", &MAX_POINTS);

    // スタックの確認 8byte分の差で格納
    let i2: i64 = 1;
    let i3: i64 = 2;
    println!("Stack address of i2 is: {:p}", &i2);
    println!("Stack address of i3 is: {:p}", &i3);

    // シャドーイング
    let y = 5;
    println!("Stack address of y is: {:p}", &y);
    let y = y + 1;
    println!("Stack address of y is: {:p}", &y);
    let y = y * 2;
    println!("Stack address of y is: {:p}", &y);
    println!("The value of y is: {}", y);

    {
        // スコープが効く
        let y = 0;
        println!("The value of y is: {}", y);
    }
    println!("The value of y is: {}", y);

    // タプル型
    let t1 = (500, 6.4, "dummy");
    let (x, y, z) = t1;
    println!("The value of t1 is: {} {} {}", t1.0, t1.1, t1.2);

    let mut t2 = ((0, 1), (2, 3));
    let ((ref mut x1_ptr, ref mut y1_ptr), _) = t2; // 指定しない（使用しない）場合は _

    // 参照外し * タプルへの値の代入
    *x1_ptr = 5;
    *y1_ptr = -5;
    // プリミティブ型でない場合（構造体やタプル等） {:?}
    println!("{:?}", t2);

    // 配列
    let a1 = [1, 2, 3, 4, 5];
    let a2 = [0; 10]; // 0が10含まれた配列を生成
    println!("{:?} {:?} {} {}", a1, a2, a1[2], a1[3]);

    // 文字列スライス構造
    let s1 = "helloこんにちは挨拶"; //26bytes
    let s2 = "hello"; // 5bytes
    println!("Stack address of s1 is: {:p}", &s1);
    println!("Stack address of s2 is: {:p}", &s2);
    // 静的値領域の取得(先頭アドレス pointer)
    println!("Static memory address of s1 is: {:?}", s1.as_ptr());
    println!("Static memory address of s2 is: {:?}", s2.as_ptr());
    // 実データのバイト数
    println!("Len of s1 is: {}", s1.len());
    println!("Len of s2 is: {}", s2.len());

    // 動的メモリ確保　String
    let mut s1 = String::from("hello");
    let mut s2 = String::from("helloworld");
    println!("Stack address of s1 is: {:p}", &s1);
    println!("Stack address of s2 is: {:p}", &s2);
    // ヒープ内のメモリ容量
    println!("Heap memory address of s1 is: {:?}", s1.as_ptr());
    println!("Heap memory address of s2 is: {:?}", s2.as_ptr());
    println!("Len of s1 is: {}", s1.len());
    println!("Len of s2 is: {}", s2.len());
    println!("Capacity of s1 is: {}", s1.capacity());
    println!("Capacity of s2 is: {}", s2.capacity());
    s1.push_str("_new1");
    s2.push_str("_new2");
    println!("{} {}", s1, s2);

    // memo
    // 文字列スライス型: 参照
    // String型: 所有 ※データ所有はメモリ解放（drop）する責任あり
    // 所有者はデータに対して必ず一人。所有者がメモリを解放する（解放はRustによって自動的に行われる）
    // 所有権者 = 変数名
    // 文字列スライスは例外的に所有ではなく、参照
    // 文字列リテラルから文字列スライスを作成の場合は、文字列データは静的領域にあるのでそもそも解放の必要がない
    // String型から文字列スライスを作成した場合はs1(String型)から移行しない

    // 参照と借用
    // 借用: 所有権を移動させず、参照する権利だけを貸し出す
}
