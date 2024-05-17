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
pub fn Halted<T: Tracer>(state: &mut State, tracer: &T, gs_2160: ()) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_2161: bool,
        gs_2160: (),
    }
    let fn_state = FunctionState {
        gs_2160,
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
        // N s_0_7: branch s_0_6 b3 b1
        if s_0_6 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call EDSCR_read(s_1_0)
        let s_1_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_1_0);
        // S s_1_2: call _get_EDSCR_Type_STATUS(s_1_1)
        let s_1_2: u8 = u_get_EDSCR_Type_STATUS(state, tracer, s_1_1);
        // S s_1_3: cast zx s_1_2 -> bv
        let s_1_3: Bits = Bits::new(s_1_2 as u128, 6u16);
        // C s_1_4: const #2u : u8
        let s_1_4: u8 = 2;
        // C s_1_5: cast zx s_1_4 -> bv
        let s_1_5: Bits = Bits::new(s_1_4 as u128, 6u16);
        // S s_1_6: cmp-eq s_1_3 s_1_5
        let s_1_6: bool = ((s_1_3) == (s_1_5));
        // D s_1_7: write-var gs#2161 <= s_1_6
        fn_state.gs_2161 = s_1_6;
        // N s_1_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_2_0: read-var gs#2161:u8
        let s_2_0: bool = fn_state.gs_2161;
        // D s_2_1: not s_2_0
        let s_2_1: bool = !s_2_0;
        // N s_2_2: return s_2_1
        return s_2_1;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_3_0: const #1u : u8
        let s_3_0: bool = true;
        // D s_3_1: write-var gs#2161 <= s_3_0
        fn_state.gs_2161 = s_3_0;
        // N s_3_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
