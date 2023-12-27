use keyberon::action::{k, Action, Action::*};
use keyberon::key_code::KeyCode::*;

use crate::{NUM_COLS, NUM_LAYERS, NUM_ROWS};
#[allow(unused_macros)]

// Shift + KeyCode
macro_rules! s {
    ($k:ident) => {
        m(&[LShift, $k])
    };
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CustomActions {
    Bootloader,
}

#[allow(dead_code)]
const BOOTLOADER: Action<CustomActions> = Action::Custom(CustomActions::Bootloader);

#[rustfmt::skip]
pub static LAYERS: keyberon::layout::Layers<NUM_COLS, NUM_ROWS, NUM_LAYERS, CustomActions> = [
    /* QWERTY */
    /* 
        All Trans keys are placeholders to even out the layout
        All k(No) keys are functional
    */
    [
        [k(Kb1),   k(Kb2),  k(Kb3), k(Kb4), k(Kb5),   k(Kb6), k(Kb7), k(Kb8),   k(Kb9),   k(Kb0),   k(Kb0),  k(Kb0)],
        [k(Q),     k(W),    k(E),   k(R),   k(T),     k(Y),   k(U),   k(I),     k(O),     k(P),     k(Kb0),  k(Kb0)],
        [k(A),     k(S),    k(D),   k(F),   k(G),     k(H),   k(J),   k(K),     k(L),     k(SColon),k(Kb0),  k(Kb0)],
        [k(Z),     k(X),    k(C),   k(V),   k(B),     k(N),   k(M),   k(Comma), k(Dot),   k(Slash), k(Kb0),  k(Kb0)],
        [k(Z),     k(X),    k(C),   k(V),   k(B),     k(N),   k(M),   k(Comma), k(Dot),   k(Slash), k(Kb1),k(Kb0),],
    ] 
];
