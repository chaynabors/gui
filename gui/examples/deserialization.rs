use gui::Container;

fn main() {
    let data = r#"
        {
            "layout": {
                "Horizontal": {
                    "widgets": [
                        { "Text": { "label": "text", "text": "Hello..." }},
                        { "Text": { "label": "text", "text": "World!" }},
                        { "Button": { "label": "button" }},
                    ],
                    "padding": { "Static": 12 }
                }
            },
            "position": { "Dynamic": { "x": 0.5, "y": 0.5 }},
            "padding": { "Fixed": { "left": 10, "right": 10, "top": 10, "bottom": 10 }},
            "view": { "Unskinned": { "color": [255, 255, 255, 255] }}
        }"#;

    serde_json::from_str::<Container>(data).unwrap();
}
