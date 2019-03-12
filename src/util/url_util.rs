
pub fn get_final_host(url: &str, host: &str) -> String{
    if url.starts_with("http://") || url.starts_with("https://") {
        return url.to_owned();
    }

    if url.starts_with("//"){
        return url.to_owned();
    }

    if url.starts_with("/"){
        return format!("{}{}", host, url);
    }



    let path = url.replace("https://","").replace("http://","");
    let index = path.find("/");

    String::from("");

    if (index.is_some()) {
        let i = index.unwrap();
        let b = &path[0 .. i];
    }
    return url.to_owned();
}

pub fn get_host(url: &str) -> &str{
    url
}