use std::{
    fs,
    error::Error,
    env,
    path::Path
};
fn get_content_or_404(file_path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(file_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_content() {
        let root = Path::new("/home/caleb/rust-http-server/workspace");
        assert!(env::set_current_dir(&root).is_ok());
        let actual_content = "<h1>this is html!</h1>".as_bytes();
        
        let binding = get_content_or_404("test_files/content.html").expect("File was not found");
        let content = binding.as_bytes();

        assert_eq!(actual_content, content);
        
    }
}
