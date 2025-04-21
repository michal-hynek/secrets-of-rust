use anyhow::Ok;

use memo::{Memos, Memo, Status};
use clap::Parser;

/// Stores and manages simple reminders
#[derive(Parser)]
struct Args {
    /// Marks all matching memos as done
    #[arg(short, long)]
    done: bool,

    /// Deletes all done memos
    #[arg(short, long)]
    purge: bool,

    /// Text of the memo to store or mark as done
    text: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let mut memos = Memos::open("memos.json")?;
    let args = Args::parse();

    if args.done {
        let text = args.text.join(" ");
        for m in memos.find_all(&text) {
            m.status = Status::Done;
            println!("Marked \"{}\" as done", m.text);
        }

        memos.sync()?;
    } else if args.purge {
        memos.purge_done();
        memos.sync()?;
    } else if args.text.is_empty() {
        for memo in &memos.inner {
            println!("{memo}");
        }
    } else {
        let text = args.text.join(" ");
        memos.inner.push(
            Memo {
                text: text.clone(),
                status: Status::Pending,
            },
        );
        memos.sync()?;
        println!("Added \"{}\" as a new memo", &text);
    }

    Ok(())
}
