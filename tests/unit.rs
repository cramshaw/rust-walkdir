use readfiles;

#[cfg(test)]
mod tests {
    #[test]
    fn run() {
        let fi = readfiles::search();
        assert_eq!(fi.len(), 8);
    }
}
