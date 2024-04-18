
mod actions;
mod commands;
mod validators;
mod repositories;

trait Drawable {
    fn draw(&self);
}



fn main() {
    println!("Launching action!");
    let result = actions::domains::update();
    render(result);
}

fn draw_all(drawables: Vec<&dyn Drawable>) {
    for drawable in drawables {
        drawable.draw();
    }
}

fn render(result: Result<(), String>) {
    println!("Rendering result {:#?}", result);
}