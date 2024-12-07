mod external_sorting;

fn main() -> std::io::Result<()> {
    if let Err(error) = external_sorting::ProcessHugeFile("./huge.txt")
        .as_chunks()?
        .sort_each_chunk()
    {
        eprintln!("{error:#}");
        std::io::stdin().read_line(&mut String::new()).unwrap();
    }

    Ok(())
}
