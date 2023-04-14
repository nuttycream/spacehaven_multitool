pub enum Input<'a> {
    Document(&'a sxd_document::dom::Document<'a>),
    Node(&'a sxd_xpath::nodeset::Node<'a>),
}

pub fn evaluate_nodeset<'a>(
    xpath: &'a str,
    factory: &'a sxd_xpath::Factory,
    context: &'a sxd_xpath::Context<'a>,
    input: Input<'a>,
) -> Result<sxd_xpath::nodeset::Nodeset<'a>, Box<dyn std::error::Error>> {
    let node = match input {
        Input::Document(document) => document.root().try_into()?,
        Input::Node(node) => *node,
    };

    let path = factory.build(xpath)?;

    let result = path
        .map(|path| path.evaluate(context, node))
        .transpose()
        .map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to evaluate XPath expression: '{}'", xpath),
            )
        })?;

    match result {
        Some(sxd_xpath::Value::Nodeset(nodes)) => Ok(nodes),

        _ => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to evaluate: Unknown Value type",
        ))),
    }
}

pub fn parse_attribute<T>(
    element: &sxd_document::dom::Element<'_>,
    attr_name: &str,
) -> Result<T, Box<dyn std::error::Error>>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    let value = element.attribute_value(attr_name).ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Attribute not found: '{}'", attr_name),
        )
    })?;

    value
        .parse::<T>()
        .map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to parse attribute '{}': {:?}", attr_name, e),
            )
        })
        .map_err(|e| e.into())
}

pub fn get_child_node<'a>(
    node: &'a sxd_xpath::nodeset::Node<'a>,
    child_name: &str,
) -> Result<sxd_xpath::nodeset::Node<'a>, Box<dyn std::error::Error>> {
    for child_node in node.children() {
        if let Some(name) = child_node.prefixed_name() {
            if name.as_str() == child_name {
                return Ok(child_node);
            }
        }
    }
    Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::Other,
        format!("Child node '{}' not found.", child_name),
    )))
}
