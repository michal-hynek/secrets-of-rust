use std::env;
use anyhow::Ok;

use memo::{Memos, Memo, Status};

fn main() -> anyhow::Result<()> {
    let mut memos = Memos::open("memos.json")?;
    let args: Vec<_> = env::args().skip(1).collect();

    if args.is_empty() {
        for memo in &memos.inner {
            println!("{memo:?}");
        }
    } else {
        let text = args.join(" ");
        memos.inner.push(
            Memo {
                text,
                status: Status::Pending,
            },
        );
        memos.sync()?;
    }

    Ok(())
}
