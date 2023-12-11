mod backend {
    use std::path::PathBuf;
    use std::fs::{File, read_to_string};


    pub fn head(path: PathBuf, n: usize) -> Vec<String> {
        read_to_string(path).unwrap().lines().take(n).map(|l| l.to_owned()).collect()
    }

    pub fn tail(path: PathBuf, n: usize) -> Vec<String> {
        let lines = read_to_string(path).unwrap()
            .lines()
            .map(|l|l.to_owned())
            .collect::<Vec<_>>();
        let len = lines.len();
        lines[len - n..].into_iter().map(|l| l.to_owned()).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::backend::*;
    #[test]
    fn test_head() {
        let lines = head("files/test.txt".into(), 3);
        assert_eq!(lines, vec!["1", "2", "3"]);
    }

    #[test]
    fn test_tail() {
        let lines = tail("files/test.txt".into(), 3);
        assert_eq!(lines, vec!["8", "9", "10"]);
    }
}
