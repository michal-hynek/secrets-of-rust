use std::env;
use anyhow::Ok;

use memo::Memos;

fn main() -> anyhow::Result<()> {
    let mut memos = Memos::open("memos.txt")?;
    let args: Vec<_> = env::args().skip(1).collect();

    if args.is_empty() {
        for memo in &memos.inner {
            println!("{memo}");
        }
    } else {
        let new_memo = args.join(" ");
        memos.inner.push(new_memo);
        memos.sync()?;
    }

    Ok(())
}
