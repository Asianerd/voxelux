pub fn reasonably_add_vec(target: f32, current: f32) -> f32 {
    // adding two values of the same axis
    if current.signum() == target.signum() {
        // acting in the same direction
        //    ^ t       ^ c
        //    |         |
        //    ^ c    -> |
        //    |         |
        //    X         X
        // 
        //    ^ c       ^ c
        //    |         |
        //    ^ t    -> |
        //    |         |
        //    X         X
        if current.signum() == 1f32 {
            f32::max(target, current)
        } else {
            f32::min(target, current)
        }
    } else {
        //    ^ t
        //    |
        //    |      -> ^ c
        //    X         X
        //    |         
        //    V c
        target + current
    }
}

pub fn slightly_round_floats(i: f32, custom_wiggle_room: Option<f32>) -> f32 {
    // 0.999999 -> 1.0
    let w = custom_wiggle_room.unwrap_or(0.000005);
    if (i + w) >= i.ceil() {
        return i.ceil();
    }
    if (i - w) <= i.floor() {
        return i.floor();
    }
    i
}
