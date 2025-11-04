// 観察用の型。スコープを出る（または drop() する）とメッセージを出す
struct Guard(&'static str);

impl Drop for Guard {
    fn drop(&mut self) {
        // Dropは「その値が破棄される瞬間」に必ず呼ばれる
        println!("Drop: {}", self.0);
    }
}

// 見やすいように区切り線
fn sep(title: &str) { println!("\n=== {title} ==="); }

fn experiment_a_lifo_order() {
    sep("A: 同一スコープ内は後入れ先出し（LIFO）");
    let _a = Guard("A");
    let _b = Guard("B");
    let _c = Guard("C");
    // ここで関数末尾。_c → _b → _a の順に Drop が呼ばれる
}

fn experiment_b_nested_and_explicit_drop() {
    sep("B: ネスト + 明示的 drop()");
    let outer = Guard("outer");
    {
        let _inner1 = Guard("inner1");
        let inner2 = Guard("inner2");
        // inner2 をここで早めに解放（以後 inner2 は使えない）
        drop(inner2);
        println!("inner2 を drop() した直後");
        // スコープを抜けるので inner1 がここで Drop
    }
    println!("ネストを抜けた直後");
    // 最後に outer が Drop（この関数の末尾）
    drop(outer); // 明示的にここで落としてもOK（末尾まで待たない）
}

fn takes_ownership(_g: Guard) {
    // 何もしないが、関数末尾で _g が Drop される
}

fn experiment_c_move_and_function_return() {
    sep("C: ムーブと関数引き渡し");
    let a = Guard("move_src");
    let b = a; // 所有権が a → b へムーブ（a は以後使えない）
    // println!("{:?}", a); // ← これはコンパイルエラー（コメントのままに）
    takes_ownership(b); // b はここで関数にムーブ → takes_ownership末尾でDrop
    // ここでは b も a も存在しない（もうDrop済み）
}

fn experiment_d_vec_clear_and_scope() {
    sep("D: Vec の clear() と スコープ終端");
    let mut v: Vec<Guard> = Vec::new();
    v.push(Guard("v[0]"));
    v.push(Guard("v[1]"));
    v.push(Guard("v[2]"));

    println!("→ v.clear() を呼ぶと、中身(要素)がここで Drop");
    v.clear(); // ここで v の各要素が Drop される（順序は後ろから：v[2], v[1], v[0]）

    println!("→ その後に再度 push して、スコープ終端で Drop");
    v.push(Guard("v2[0]"));
    v.push(Guard("v2[1]"));
    // ここで関数末尾。残っている要素 v2[1], v2[0] が Drop → その後、Vec本体のバッファも解放
}

fn main() {
    experiment_a_lifo_order();
    experiment_b_nested_and_explicit_drop();
    experiment_c_move_and_function_return();
    experiment_d_vec_clear_and_scope();

    // 参考：mainの末尾で何かを落としたい時
    // let x = Guard("end");
    // drop(x); // 明示的にここで落とす
}
