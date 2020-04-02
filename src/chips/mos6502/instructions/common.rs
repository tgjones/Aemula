use super::super::pins::Pins;
use super::super::registers::Registers;

pub(crate) fn rmw_cycle(r: &mut Registers, pins: &mut Pins) {
    r.ad.lo = pins.data;
    pins.rw = false;
}