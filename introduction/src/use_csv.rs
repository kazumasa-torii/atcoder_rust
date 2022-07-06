pub fn csv() {
    let penguin_data: &str = "\
    common name,length (cm)
    Little penguin,33
    Yellow-eyed penguin,65
    Fiordland penguin,60
    Invalid,data
    ";

    let records = penguin_data.lines();
    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        // lambda関数とかクロージャーみたいなやつ
        // Vec<> はvectorというコレクション型の略称
        let fields: Vec<_> = record.split(',').map(|field| field.trim()).collect();

        // リリースビルドにはコンパイル時には反映されないやつ
        if cfg!(debug_assertions) {
            // 標準エラーに出力する際に使用する
            // {:?} は 「値を文字列として表現するメソッドを使え」 と指示している
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }

        let name = fields[0];
        // 型注釈
        // if let Ok(length) = fields[1].parse::<f32>()
        // これは、fields[1]を32ビットの浮動小数点数として解析できるか試み成功したらその値をlength変数に代入するって意味

        // parse()メソッドは文字列の解析に成功したOk(T)を返す Tは任意の型でOK
        // 失敗したら Err(E)を返す
        // このように書くとなにかエラーが起きた場合は、その処理をスキップすることができる
        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{} {}cm", name, length);
        }

        // 今回はほぼ型をrust側が推測して対応していたが推測できない場合は、きちんと型指定してあげる必要がある
    }
}
