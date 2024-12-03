// Tag docs with the required platform and features.
//
// https://doc.rust-lang.org/rustdoc/unstable-features.html
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(
    docsrs,
    feature(doc_auto_cfg),
    feature(doc_cfg_hide),
    doc(cfg_hide(doc))
)]

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(feature = "feature_1")]
pub fn feature_1() {}

#[cfg_attr(docsrs, doc(cfg(feature = "override_tag_feature_1")))]
#[cfg(feature = "feature_1")]
pub fn feature_1_override() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
