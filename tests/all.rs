
use typed_css::*;

#[test]
fn it_works() {
    assert_eq!(format!("{}", css! {
        color: blue;
    }), "color: blue;");

    assert_eq!(format!("{}", css! {
        background_color: green;
    }), "background-color: green;");

    assert_eq!(format!("{}", css! {
        a {
            color: blue;
        }
    }), "a { color: blue; }");

    assert_eq!(format!("{}", css! {
        div {
            color: yellow;
            border: 1px solid red;
        }
    }), "a { color: blue; }");
}
