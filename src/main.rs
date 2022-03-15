use winrt::{init_apartment, ApartmentType, RtActivatable, windows::foundation::collections::StringMap};

fn main() {
    init_apartment(ApartmentType::STA);

    let af = StringMap::get_activation_factory();

    let instance = af.activate_instance();

    let name = instance.get_runtime_class_name();

    print!("{:}", name.to_string());
}