mod external_sorting;

fn main() -> std::io::Result<()> {
    external_sorting::ProcessHugeFile("./example/example.txt")
        .as_chunks()
        .sort_each_chunk();

    Ok(())
}
