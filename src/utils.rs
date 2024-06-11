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
