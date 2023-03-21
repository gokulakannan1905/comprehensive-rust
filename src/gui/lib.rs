// TODO: remove this when you're done with your implementation.
#![allow(unused_imports, unused_variables, dead_code)]

pub trait Widget {
    /// Natural width of `self`.
    fn width(&self) -> usize;

    /// Draw the widget into a buffer.
    fn draw_into(&self, buffer: &mut dyn std::fmt::Write);

    /// Draw the widget on standard output.
    fn draw(&self) {
        let mut buffer = String::new();
        self.draw_into(&mut buffer);
        println!("{buffer}");
    }
}

pub struct Label {
    label: String,
}

impl Label {
    fn new(label: &str) -> Label {
        Label {
            label: label.to_owned(),
        }
    }
}

pub struct Button {
    label: Label,
    callback: Box<dyn FnMut()>, //trait object --> to refer different types by using trait
}

impl Button {
    fn new(label: &str, callback: Box<dyn FnMut()>) -> Button {
        Button {
            label: Label::new(label),
            callback,
        }
    }
}

pub struct Window {
    title: String,
    widgets: Vec<Box<dyn Widget>>,
}

impl Window {
    fn new(title: &str) -> Window {
        Window {
            title: title.to_owned(),
            widgets: Vec::new(),
        }
    }

    fn add_widget(&mut self, widget: Box<dyn Widget>) {
        self.widgets.push(widget);
    }
}


impl Widget for Label {
    fn width(&self) -> usize {
        40
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        let s = format!("{}\n",&self.label);
        buffer.write_str(&s).unwrap();
    }
}

impl Widget for Button {
    fn width(&self) -> usize {
        15
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        let width = &self.width();
        let message = &self.label;
        buffer.write_str((  format!("+{:-<width$}+\n", "") +
                            format!("|{: ^width$}|\n", "click me!").as_str() +
                            format!("+{:-<width$}+\n", " ").as_str()).as_str()).unwrap();
    }
}

impl Widget for Window {
    fn width(&self) -> usize {
        40
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        let width = &self.width();
        let title = &self.title;
        let mut s = format!("+{:-<width$}+\n","") + 
                        format!("|{:^width$}|\n",title).as_str() +
                        format!("+{:=<width$}+\n","").as_str();        
        for widget in &self.widgets{
            let mut buf = String::new();
            widget.draw_into(&mut buf);
            for line in buf.lines(){
                s += format!("|{:^width$}|\n",line).as_str();
            }
        }
        s += format!("+{:-<width$}+","").as_str();
        buffer.write_str(&s).unwrap();
    }
}

fn lines(s:&str) -> u8{
    let mut counter:u8 = 0;
    for _ in s.lines(){
        counter += 1;
    }
    counter
}
fn main() {
    let mut window = Window::new("Rust GUI Demo 1.23");
    window.add_widget(Box::new(Label::new("This is a small text GUI demo.")));
    window.add_widget(Box::new(Button::new(
        "Click me!",
        Box::new(|| println!("You clicked the button!")),
    )));
    window.draw();
}