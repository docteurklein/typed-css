
use typed_css::*;

#[test]
fn it_works() {
    assert_eq!(format!("{}", css! {
        color: blue;
    }), "color: blue;");

    assert_eq!(format!("{}", css! {
        background_color: green;
    }), "background-color: green;");
}
