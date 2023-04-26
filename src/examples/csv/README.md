# Working with Excel Sheets in Rust

Technically speaking, we have almost no interest in "saving to an excel." Excel stores its files in a binary, and we don't want to deal with that. Instead we can use CSV (comma separated values) as much more friendly formate to _write_ to. Then anyone with the CSV file can just open it into excel.

Information on CSV, Excel, and Rust:

-   https://toggl.com/track/difference-between-csv-xls/
-   https://www.zupzup.org/xlsx-in-rust/

CSV files announce the column headers in the first row:

> name,email,balance,credit_card
> <br/>jim,jim@gmail.conn,40,yes
> <br/>...

To write to CSV, you basically just write in that structure.

See Rust's main CSV tutorial page:
https://docs.rs/csv/1.0.0/csv/tutorial/index.html
