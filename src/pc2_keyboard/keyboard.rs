use crate::pc2_keyboard::scancode::decode_keys;
use crate::pc2_keyboard::pc2::wait_output_full;
use crate::pc2_keyboard::pc2::read_data;

pub unsafe fn keyboard () { 
    
    wait_output_full();

    let scancode = read_data();

    decode_keys(scancode);

}