use gui::Container;
use gui::Layout;
use gui::LayoutPadding;
use gui::Size;
use gui::View;
use gui::Widget;

fn main() {
    let container = Container {
        screen_position: [0.0, 0.0],
        pixel_position: [15, 15],
        pivot: [0.0, 0.0],
        view: View::default(),
        layout: Layout::Free(Widget::Text { label: "text".into(), text: "Hello world!".into() }),
    };

    println!("{}", serde_json::to_string_pretty(&container).unwrap());

    let container = Container {
        screen_position: [0.5, 0.5],
        pixel_position: [0, 0],
        pivot: [0.5, 0.5],
        view: View::Simple {
            width: Size::Fixed(128),
            height: Size::Fixed(256),
            color: [0xff, 0xff, 0xff, 0xff],
        },
        layout: Layout::Vertical {
            widgets: vec![
                Widget::Text { label: "text".into(), text: "Hello...".into() },
                Widget::Text { label: "text".into(), text: "World!".into() },
                Widget::Button { label: "button".into() },
            ],
            padding: LayoutPadding::Static(12),
        },
    };

    println!("{}", serde_json::to_string_pretty(&container).unwrap());
}
