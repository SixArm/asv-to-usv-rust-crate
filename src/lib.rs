use std::convert::AsRef;
use usv::style::Style;

pub mod examples;

pub fn asv_to_usv<
    S: AsRef<str> + Sized
>(
    usv: S,
    style: &Style,
) -> String {
    usv.as_ref()
    .replace("\u{001F}", &style.unit_separator)
    .replace("\u{001E}", &style.record_separator)
    .replace("\u{001D}", &style.group_separator)
    .replace("\u{001C}", &style.file_separator)
}

#[cfg(test)]
mod tests {
    use super::*;
    use usv::style::Style;
    use crate::examples::*;

    #[test]
    fn asv_to_usv_test() {
        let asv = EXAMPLE_INPUT_FILES;
        let usv = usv::examples::EXAMPLE_FILES_STYLE_SYMBOLS;
        let style = Style::default();
        assert_eq!(asv_to_usv(&asv, &style), usv);
    }

}
