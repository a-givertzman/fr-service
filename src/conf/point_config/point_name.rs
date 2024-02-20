struct PointName {
    value: String
}
///
/// 
impl PointName {
    ///
    /// 
    pub fn new(parent: &str, name: &str) ->Self {
        let value = match parent.chars().last() {
            Some(last) => {
                if last == '/' {
                    format!("{}{}", parent, name)
                } else {
                    format!("{}/{}", parent, name)
                }
            },
            None => {
                format!("{}{}", parent, name)
            },
        };
        Self {
            value,
        }
    }
}