use gui::Instance;
use gui::Widget;

fn main() {
    let mut instance = Instance::new();
    instance.add_widget(Widget::Window {
        label: "window".to_string(),
        widgets: vec![
            Widget::Text { label: "text".to_string(), text: "Hello world!".to_string() }
        ],
    });

    println!("{instance}");
}
