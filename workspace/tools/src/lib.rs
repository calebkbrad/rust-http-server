use std::{
    fs,
    error::Error,
    env,
    path::Path,
};

use chrono::{
    offset::LocalResult, DateTime, Local, Utc
};

fn generate_200_response(file_path: &str) -> String {
    let content = fs::read_to_string(file_path).unwrap();
    let response_header = "HTTP/1.1 200 OK\r\n".to_owned();
    let generic_headers = generate_generic_headers().to_owned();
    let ok_headers = generate_200_headers(&content).to_owned();
    
    content + &response_header + &generic_headers + &generic_headers + &ok_headers
}


pub fn get_content_or_404(file_path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(file_path)
}



// fn generate_404_response() -> &[u8] {
//     b"HTTP/1.1 404 Not Found"
// }

fn generate_generic_headers() -> String {
    let mut headers = Vec::with_capacity(3);
    headers.push(generate_date_header(Utc::now()));
    headers.push(String::from("Server: Rust Server"));
    headers.push(String::from("Connection: close"));
    
    headers.join("\r\n")
}

fn generate_date_header(time: DateTime<Utc>) -> String {
    format!("Date: {}", time.format("%a, %d %b %Y %I:%M:%S GMT"))

}

fn generate_200_headers(content: &str) -> String{
    String::from("insert headers related to content here")
}




#[cfg(test)]
mod tests {
    use chrono::TimeZone;

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

    // #[test]
    // fn test_404() {

    // }

    #[test]
    fn test_date_header() {
        let test_time = Utc.with_ymd_and_hms(2020, 3, 20, 2, 23, 0).unwrap();
        let correct_string = String::from("Date: Fri, 20 Mar 2020 02:23:00 GMT");
        let test_string = generate_date_header(test_time);

        assert_eq!(correct_string, test_string);
    }
}
