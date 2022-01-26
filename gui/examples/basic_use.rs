use gui::container::Container;
use gui::pivot::Pivot;
use gui::size::Size;
use gui::widget::Widget;

fn main() {
    let mut gui = Container::new("container");
    gui.set_position([0.0, 0.0]);
    gui.set_size(Size::Dynamic { width: 0.5, height: 0.5 });
    gui.set_pivot(Pivot::MiddleCenter);
    gui.add_widget(Widget::Text {
        label: "text".to_string(),
        text: "Hello World!".to_string(),
    });

    println!("{gui}");
}
