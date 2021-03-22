// Importing crates
use fltk::{app, window::*, button::*, input, group, text::*};
use open;

// Backend
fn open_link(link: String) {
    open::that(link).unwrap();
}

fn colors() -> Vec<[u32; 4]> {
    // Different set of colors
    let all:Vec<[u32;4]> = vec![[0xff0000,0xe50000, 0xcc0000, 0xb2000],  [0x153043, 0x09324a, 0x31485c, 264963]];
    all
}

fn main() {



    //  Creating the App and all the necessary stuff
    let app = app::App::default().with_scheme(app::Scheme::Gleam);
    let mut wind = Window::new(100, 100, 400, 300, "Opener");
    let mut inp = input::Input::new(0,90, 400, 100, "");
    let mut btn = Button::new(50,200, 300, 50, "Search");


    // Styling the app
    let rand_color = colors();
    btn.set_color(Color::from_u32(rand_color[1][0]));
    btn.set_selection_color(Color::from_u32(rand_color[1][1]));
    btn.set_frame(FrameType::GtkUpBox);
    wind.set_color(Color::from_u32(rand_color[1][2]));
    inp.set_color(Color::White);
    btn.set_label_color(Color::White);

    // Windows shoudl be resizable
    wind.make_resizable(true);


    wind.end();
    wind.show();

    // Binding an event to button
    /*
    When ever the button is pressed you got to that link
    */
    btn.set_callback(move || open_link("https:\\".to_owned()+&inp.value()));

    // running app
    app.run().unwrap();
}
