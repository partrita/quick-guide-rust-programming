mod my_module;
mod second_mod;

use my_module::test_my_mod;

fn main() {
    test_my_mod();
    second_mod::sec_mod_file::second_module();
}
