use std::fs;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

///Ingests a file for usage
///
///# Arguments
///
///* `filename` - path to file
///
///# Returns
///
///* file contents split per line
///
pub fn ingest_file(filename: &str) -> Vec<String> {
    let bytes = fs::read(filename).unwrap();
    let s = String::from_utf8(bytes).unwrap();
    s.split('\n')
        .map(|x| x.trim())
        .map(String::from)
        .filter(|x| !x.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn ingest_file_works() {
        let result = ingest_file("src/test.txt");
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "Here is");
        assert_eq!(result[1], "some text");
        assert_eq!(result[2], "hooray!");
    }
}
