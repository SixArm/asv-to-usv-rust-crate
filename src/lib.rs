use std::convert::AsRef;
use usv::style::Style;

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

    #[test]
    fn asv_to_usv_test() {
        let asv = String::from("a\u{001F}b\u{001F}\u{001E}c\u{001F}d\u{001F}\u{001E}\u{001D}e\u{001F}f\u{001F}\u{001E}g\u{001F}h\u{001F}\u{001E}\u{001D}\u{001C}i\u{001F}j\u{001F}\u{001E}k\u{001F}l\u{001F}\u{001E}\u{001D}m\u{001F}n\u{001F}\u{001E}o\u{001F}p\u{001F}\u{001E}\u{001D}\u{001C}");
        let usv = "a␟b␟␞c␟d␟␞␝e␟f␟␞g␟h␟␞␝␜i␟j␟␞k␟l␟␞␝m␟n␟␞o␟p␟␞␝␜";
        let style = Style::default();
        assert_eq!(asv_to_usv(&asv, &style), usv);
    }

}
