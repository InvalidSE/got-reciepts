use escpos_rs::{Printer, PrinterProfile};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let printer_details = PrinterProfile::usb_builder(0x4b43, 0x3830).build();

    let printer = match Printer::new(printer_details) {
        Ok(maybe_printer) => match maybe_printer {
            Some(printer) => printer,
            None => panic!("No printer was found :(")
        },
        Err(e) => panic!("Error: {}", e)
    };

    printer.println("Hello, world!")?;

    printer.cut()?;

    Ok(())
}