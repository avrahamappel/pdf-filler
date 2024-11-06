use dialoguer::Input;
use lopdf::{Document, Object};

fn main() {
    // Load the PDF document
    let input_pdf_path: String = Input::new()
        .with_prompt("Enter the path to the PDF form")
        .interact_text()
        .unwrap();

    let mut document = Document::load(&input_pdf_path).expect("Failed to load PDF document");

    // Iterate over the objects in the PDF to find form fields
    for object in document.objects.values_mut() {
        if let Object::Dictionary(ref mut dict) = object {
            if dict.has(b"FT") {
                // Check if it's a form field
                let field_name = dict
                    .get(b"T")
                    .and_then(|name| name.as_str())
                    .unwrap_or(b"Unnamed Field");
                let field_value: String = Input::new()
                    .with_prompt(format!(
                        "Enter value for field '{}'",
                        String::from_utf8_lossy(field_name)
                    ))
                    .interact_text()
                    .unwrap();

                // Set the field value
                if let Ok(value) = dict.get_mut(b"V") {
                    *value = field_value.into();
                } else {
                    dict.set(b"V", field_value);
                }
            }
        }
    }

    // Check if a signature is required
    //if Confirm::new()
    //    .with_prompt("Is a signature required?")
    //    .interact()
    //    .unwrap()
    //{
    //    let signature_path = Input::new()
    //        .with_prompt("Enter the path to the signature image file")
    //        .interact_text()
    //        .unwrap();
    //
    //    // Load the signature image (this is a placeholder; actual implementation may vary)
    //    let signature_data = fs::read(&signature_path).expect("Failed to read signature file");
    //
    //    // Here you would need to add the signature to the PDF
    //    // This is a simplified example; actual implementation will depend on your PDF structure
    //    // You might need to create a new image object and add it to the PDF
    //    // For now, we will just print a message
    //    println!("Signature loaded from: {}", signature_path);
    //}

    // Save the filled PDF
    let output_pdf_path: String = Input::new()
        .with_prompt("Enter the path to save the filled PDF")
        .default(input_pdf_path)
        .interact_text()
        .unwrap();

    document
        .save(&output_pdf_path)
        .expect("Failed to save PDF document");

    println!("PDF form filled and saved to: {output_pdf_path}");
}
