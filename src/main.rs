extern crate gtk;
use gtk::prelude::*;
use std::io::{self, Write};
use std::rc::Rc;
fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    // Create the main window
    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("LED Control");
    window.set_default_size(200, 150);

    // Create a vertical box layout
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
    window.add(&vbox);

    // Create the text view for logs
    let scrolled_window = gtk::ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
    let text_view = gtk::TextView::new();
    text_view.set_editable(false);
    text_view.set_wrap_mode(gtk::WrapMode::Word);
    scrolled_window.add(&text_view);
    vbox.add(&scrolled_window);

    // Create the buttons
    let high_button = gtk::Button::with_label("Turn On");
    let low_button = gtk::Button::with_label("Turn Off");
    let toggle_button = gtk::Button::with_label("Toggle");

    // add a serial port selector
    let serial_port_selector = gtk::ComboBoxText::new();
    for port in get_serial_ports() {
        serial_port_selector.append_text(&port);
    }
    // when a port gets selected, keep its name.
    // will be used to open the port when a command is sent
    let selected_port = Rc::new(std::cell::RefCell::new(String::new()));
    serial_port_selector.connect_changed(move |selector| {
        let port = selector.active_text().expect("Failed to get active text");
        *selected_port.borrow_mut() = port.to_string();
    });

    // Add buttons to the layout
    vbox.add(&high_button);
    vbox.add(&low_button);
    vbox.add(&toggle_button);

    // Connect button signals
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        gtk::glib::Propagation::Stop
    });

    let port = serialport::new(selected_port.borrow(), 9600)
        .open()
        .expect("Failed to open serial port");

    // protect access to the serial port
    let port = Rc::new(port);

    let log_buffer = text_view.buffer().expect("Failed to get buffer");

    high_button.connect_clicked(move |_| {
        send_command(port.clone(), "high", &log_buffer).expect("Failed to send command");
    });

    low_button.connect_clicked(move |_| {
        send_command(port.clone(), "low", &log_buffer).expect("Failed to send command");
    });

    toggle_button.connect_clicked(move |_| {
        send_command(port.clone(), "toggle", &log_buffer).expect("Failed to send command");
    });

    // Show everything
    window.show_all();

    // Run the GTK event loop
    gtk::main();
}

fn send_command(mut port: Rc<dyn serialport::SerialPort>, command: &str, log_buffer: &gtk::TextBuffer) -> io::Result<()> {
    port.write_all(command.as_bytes())?;
    port.write_all(b"\n")?;
    log_buffer.insert_at_cursor(&format!("Sent command: {}\n", command));
    Ok(())
}

fn get_serial_ports() -> Vec<String> {
    let ports = serialport::available_ports().expect("Failed to list serial ports");
    ports.iter().map(|port| port.port_name.clone()).collect()
}
