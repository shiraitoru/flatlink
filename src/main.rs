use clap::Parser;

/// flatlinkはsrcオプションで指定したディレクトリ内のすべてのファイル(サブディレクトリも含む)に対して、dstオプションで指定したディレクトリ内に階層を作らずにハードリンクを作成します。
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // リンク元となるファイルを含むディレクトリパス
    #[arg(short, long, help = "リンク元のディレクトリパス")]
    src: String,

    // リンク作成先のディレクトリパス
    #[arg(short, long, help = "リンク作成先のディレクトリパス")]
    dst: String,
}

fn main() {
    let args = Args::parse();

    flatlink::link(&args.src, &args.dst).unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        std::process::exit(1);
    });
}
