// use std::array::IntoIter;
// use tabled::settings::style::HorizontalLine;
// use tabled::settings::style::HorizontalLine::HorizontalLineIter;
use tabled::settings::style::On;
// use tabled::settings::style::VerticalLine;
// use tabled::settings::style::VerticalLine::VerticalLineIter;
use tabled::settings::Style;

#[allow(clippy::type_complexity)]
pub(crate) fn modern_inline() -> Style<(), (), (), (), On, On, 0, 0> {
    Style::modern().remove_top().remove_left().remove_bottom().remove_right()
}
