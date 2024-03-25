mod servicehelper; // Note: Corrected the module name to lowercase 'servicehelper'
use servicehelper::ServiceHelper; // Corrected the module path

fn main() {
    // Create an instance of ServiceHelper
    let service_helper = ServiceHelper;

    // Call a method of ServiceHelper
    let menu_item_data = service_helper.get_menu_item_data();
    println!("{}", menu_item_data.to_string());
}
