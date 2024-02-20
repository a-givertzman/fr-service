///
/// Creates Point name from parts, dividing parts with single "/" character
/// - path1 + "" = /path1
/// - path1 + path2 = /path1/path2
/// - path1/ + path2 = /path1/path2
/// - path1 + /path2 = /path1/path2
/// - path1/ + /path2 = /path1/path2
pub struct PointName {
    value: String
}
///
/// 
impl PointName {
    ///
    /// 
    pub fn new(left: &str, right: &str) ->Self {
        let left = match left.chars().next() {
            Some(left_first) => {
                if left_first == '/' {
                    format!("{}", left)
                } else {
                    format!("/{}", left)
                }
            },
            None => {
                format!("/")
            },
        };
        let value = match left.chars().last() {
            Some(left_last) => {
                match right.chars().next() {
                    Some(right_first) => {
                        if left_last == '/' && right_first == '/' {
                            format!("{}{}", left, &right[1..])
                        } else if left_last == '/' && right_first != '/' {
                            format!("{}{}", left, right)
                        } else if left_last != '/' && right_first == '/' {
                            format!("{}{}", left, right)
                        } else {
                            format!("{}/{}", left, right)
                        }
                    },
                    None => {
                        left
                    },
                }
            },
            None => {
                match right.chars().next() {
                    Some(name_first) => {
                        if name_first == '/' {
                            right.to_string()
                        } else {
                            format!("/{}", right)
                        }
                    },
                    None => {
                        panic!("PointName.new | Parent or name must not be empty")
                    },
                }
            },
        };
        Self {
            value,
        }
    }
    ///
    /// 
    pub fn full(&self) -> String {
        self.value.clone()
    }
}