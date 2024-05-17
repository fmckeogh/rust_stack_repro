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
use EDSCR_read::*;
use u_get_EDSCR_Type_STATUS::*;
use common::*;
pub fn Restarting<T: Tracer>(state: &mut State, tracer: &T, gs_5103: ()) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_5103: (),
    }
    let fn_state = FunctionState {
        gs_5103,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call EDSCR_read(s_0_0)
        let s_0_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_0_0);
        // S s_0_2: call _get_EDSCR_Type_STATUS(s_0_1)
        let s_0_2: u8 = u_get_EDSCR_Type_STATUS(state, tracer, s_0_1);
        // S s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 6u16);
        // C s_0_4: const #1u : u8
        let s_0_4: u8 = 1;
        // C s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 6u16);
        // S s_0_6: cmp-eq s_0_3 s_0_5
        let s_0_6: bool = ((s_0_3) == (s_0_5));
        // N s_0_7: return s_0_6
        return s_0_6;
    }
}
