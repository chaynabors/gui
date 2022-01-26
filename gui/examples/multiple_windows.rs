use gui::Instance;
use gui::Widget;

fn main() {
    let mut instance = Instance::new();
    let label1 = "label1".to_string();
    instance.add_widget(Widget::Window {
        label: label1.to_string(),
        widgets: vec![
            Widget::Text { label: "text".to_string(), text: "Hello fixed!".to_string() }
        ],
    });

    let label2 = "label2".to_string();
    instance.add_widget(Widget::Window {
        label: label2.to_string(),
        widgets: vec![
            Widget::Text { label: "text".to_string(), text: "Hello dynamic!".to_string() }
        ],
    });

    println!("{instance}");

    instance.remove_widget(&label1);
    instance.remove_widget(&label2);

    println!("{instance}");
}
