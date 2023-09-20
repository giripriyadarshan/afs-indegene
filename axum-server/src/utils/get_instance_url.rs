pub fn get_instance_url(instance: impl Into<String>) -> String {
    let instance = instance.into();
    let instance_url = match instance.as_str() {
        "BI_US" => "https://bi.veevavault.com/",
        "prod" => "https://vault.veevavault.com",
        _ => panic!("Invalid instance"),
    };
    instance_url.to_owned()
}
