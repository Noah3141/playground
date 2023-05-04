/* Write a struct to csv */

/*
?) Data passed to write_record has complex requirements, but those requirements are fairly broad:
*) wtr.write_record(&["field 1", "field 2", "etc"])

A slice of byte strings.
*) wtr.write_record(&[b"a", b"b", b"c"]);
A vector.
*) wtr.write_record(vec!["a", "b", "c"]);
A string record.
*) wtr.write_record(&csv::StringRecord::from(vec!["a", "b", "c"]));
A byte record.
*) wtr.write_record(&csv::ByteRecord::from(vec!["a", "b", "c"]));

 */

pub fn writefile_csv_test(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {

    let mut wtr = csv::Writer::from_path(file_path)?;

    wtr.write_record(&["City", "State", "Population", "Latitude", "Longitude"])?;
    wtr.write_record(&["Davidsons Landing", "AK", "", "65.2419444", "-165.2716667"])?;
    wtr.write_record(&["Kenai", "AK", "7610", "60.5544444", "-151.2583333"])?;
    wtr.write_record(&["Oakman", "AL", "", "33.7133333", "-87.3886111"])?;

    wtr.flush()?;

    Ok(())
}

/*
    csv::writing::writefile_csv_test("./Just dot slash"); // This will output to the crate folder when run with cargo, or to the folder in which the executable sits
    csv::writing::writefile_csv_test("./dot slash.csv"); // If you want it to be a file of a type, rather than unrecognized, give the ext

    csv::writing::writefile_csv_test("~/Just tilde"); // ! Can't do that
    csv::writing::writefile_csv_test("~/tilde.csv"); // ! Can't do that
    csv::writing::writefile_csv_test("./Attempt with spaces.csv") //? Spaces will work
    csv::writing::writefile_csv_test("./NewFolder/Attempt with spaces.csv") // ! Can't create directory with it
    csv::writing::writefile_csv_test("./Attempt with: things.csv"); // ! This will create the file but the :.. is chopped off
 */