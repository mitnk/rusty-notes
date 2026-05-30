use lazy_static::lazy_static;
use tera::Tera;

// Templates embedded in the binary.
const EMBEDDED_TEMPLATES: &[(&str, &str)] = &[
    (
        "notes/base.html",
        include_str!("../assets/templates/notes/base.html"),
    ),
    (
        "notes/home.html",
        include_str!("../assets/templates/notes/home.html"),
    ),
    (
        "notes/create.html",
        include_str!("../assets/templates/notes/create.html"),
    ),
    (
        "notes/detail.html",
        include_str!("../assets/templates/notes/detail.html"),
    ),
    (
        "notes/edit.html",
        include_str!("../assets/templates/notes/edit.html"),
    ),
];

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = Tera::default();
        if let Err(e) = tera.add_raw_templates(EMBEDDED_TEMPLATES.to_vec()) {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
        tera
    };
}
