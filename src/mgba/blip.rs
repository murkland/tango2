use super::c;

pub struct Blip<'a> {
    pub(crate) _core: &'a super::core::Core,
    pub(crate) ptr: *mut c::blip_t,
}

impl<'a> Blip<'a> {
    pub fn set_rates(&mut self, clock_rate: f64, sample_rate: f64) {
        unsafe { c::blip_set_rates(self.ptr, clock_rate, sample_rate) }
    }

    pub fn samples_avail(&self) -> i32 {
        unsafe { c::blip_samples_avail(self.ptr) }
    }

    pub fn read_samples(&self, out: &mut [i16], count: i32, stereo: bool) -> i32 {
        unsafe {
            c::blip_read_samples(
                self.ptr,
                out.as_mut_ptr(),
                count,
                if stereo { 1 } else { 0 },
            )
        }
    }
}