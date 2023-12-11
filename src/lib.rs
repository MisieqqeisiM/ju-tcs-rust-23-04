pub mod backend {
    use std::fs::read_to_string;
    use std::path::PathBuf;

    pub fn head(path: PathBuf, n: usize) -> Result<Vec<String>, String> {
        Ok(
        read_to_string(path)
            .or(Err("Could not open file.".to_owned()))?
            .lines()
            .take(n)
            .map(|l| l.to_owned())
            .collect()
        )
    }

    pub fn tail(path: PathBuf, n: usize) -> Result<Vec<String>, String> {
        let lines = read_to_string(path)
            .or(Err("Could not open file.".to_owned()))?
            .lines()
            .map(|l| l.to_owned())
            .collect::<Vec<_>>();
        let len = lines.len();
        Ok(lines[len - n..].iter().map(|l| l.to_owned()).collect())
    }
}

#[cfg(test)]
mod tests {
    use crate::backend::*;
    #[test]
    fn test_head() {
        let lines = head("files/test.txt".into(), 3);
        assert_eq!(lines, Ok(vec!["1".to_owned(), "2".to_owned(), "3".to_owned()]));
    }

    #[test]
    fn test_tail() {
        let lines = tail("files/test.txt".into(), 3);
        assert_eq!(lines, Ok(vec!["8".to_owned(), "9".to_owned(), "10".to_owned()]));
    }
}
