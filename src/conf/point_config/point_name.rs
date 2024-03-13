use concat_string::concat_string;

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
    pub fn new(left: &str, right: &str) -> Self {
        let left = match left.chars().next() {
            Some(left_first) => {
                if left_first == '/' {
                    left.to_owned()
                } else {
                    concat_string!("/", left)
                }
            },
            None => {
                String::from("/")
            },
        };
        let value = match left.chars().last() {
            Some(left_last) => {
                match right.chars().next() {
                    Some(right_first) => {
                        if left_last == '/' && right_first == '/' {
                            concat_string!(left, right[1..])
                        } else if left_last == '/' && right_first != '/' {
                            concat_string!(left, right)
                        } else if left_last != '/' && right_first == '/' {
                            concat_string!(left, right)
                        } else {
                            concat_string!(left, "/", right)
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
                            concat_string!("/{}", right)
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