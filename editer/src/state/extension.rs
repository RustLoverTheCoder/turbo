#[derive(Debug)]
pub enum Extension {
    Single(ExtensionData),
    Array(Vec<ExtensionData>),
}

#[derive(Debug)]
struct ExtensionData {
    extension: String,
}

impl Extension {
    fn single(extension: String) -> Self {
        Extension::Single(ExtensionData { extension })
    }

    fn array(extensions: Vec<String>) -> Self {
        let data = extensions
            .into_iter()
            .map(|ext| ExtensionData { extension: ext })
            .collect();

        Extension::Array(data)
    }
}
