use polars::prelude::*;

fn main() -> anyhow::Result<()> {
    let lf = LazyCsvReader::new("d1p1/input/input.txt")
        .with_separator(b' ')
        .with_has_header(false)
        .finish()?;

    let res: usize = lf
        .with_column(
            (col("column_1").sort(Default::default()) - col("column_4").sort(Default::default()))
                .abs()
                .alias("diff"),
        )
        .collect()?
        .column("diff")?
        .as_materialized_series()
        .sum()?;

    println!("res: {}", res);

    Ok(())
}
