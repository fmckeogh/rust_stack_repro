#![no_std]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_doc_comments)]
#![allow(non_upper_case_globals)]
//! BOREALIS GENERATED FILE
extern crate alloc;
use common::*;
pub fn IsEventRegisterSet<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_6623: (),
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_6623: (),
    }
    let fn_state = FunctionState {
        gs_6623,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_0_0: const #21968u : u32
        let s_0_0: u32 = 21968;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: bool = {
            let value = state.read_register::<bool>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: cast zx s_0_1 -> bv
        let s_0_2: Bits = Bits::new(s_0_1 as u128, 1u16);
        // C s_0_3: const #1u : u8
        let s_0_3: bool = true;
        // C s_0_4: cast zx s_0_3 -> bv
        let s_0_4: Bits = Bits::new(s_0_3 as u128, 1u16);
        // D s_0_5: cmp-eq s_0_2 s_0_4
        let s_0_5: bool = ((s_0_2) == (s_0_4));
        // N s_0_6: return s_0_5
        return s_0_5;
    }
}
