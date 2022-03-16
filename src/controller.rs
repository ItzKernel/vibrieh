use hidapi::{HidDevice};

pub struct Controller(HidDevice);

impl Controller {

    pub fn new(dev: HidDevice) -> Self {
        return Controller(dev);
    } 

    pub fn rumble(&self, small_motor_level: u8, big_motor_level: u8) {
        let mut buf: Vec<u8> = Vec::new();
    
        for _ in 0..40 { buf.push(0x00); } //Default value is 0 for everything
    
        buf[0] = 0x05;
        buf[1] = 0x05;
    
        buf[4] = small_motor_level;  //smol one
        buf[5] = big_motor_level;  // da big one
    
        buf[9] = 0xff;
        buf[10] = 0x00;
        
        let _ = &self.0.write(&buf);
    }
}