use crate::pricing::cost;

pub fn label(code: &str, weight: u32) -> String {
    format!("{code} costs {}", cost(weight))
}
