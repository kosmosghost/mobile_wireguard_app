mod wg_quick;
mod wg_conf;

pub fn connect_wg() -> String {
    //Todo: Check if a connection already exists.
    let interface = wg_conf::get_random_wg_config();
    wg_quick::run_wq_quick_up(interface.clone());
    interface
}

pub fn disconnect_wg() {
    let interface = wg_conf::get_interface();
    wg_quick::run_wq_quick_down(interface);
}

pub fn refresh_wg() -> String {
    let old_interface = wg_conf::get_interface();
    wg_quick::run_wq_quick_down(old_interface);
    let new_interface = wg_conf::get_random_wg_config();
    wg_quick::run_wq_quick_up(new_interface.clone());
    new_interface
}

