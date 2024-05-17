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
use HaveSME::*;
use common::*;
pub fn InStreamingMode<T: Tracer>(state: &mut State, tracer: &T, gs_7297: ()) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_7298: bool,
        gs_7297: (),
    }
    let fn_state = FunctionState {
        gs_7297,
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
        // S s_0_1: call HaveSME(s_0_0)
        let s_0_1: bool = HaveSME(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b3 b1
        if s_0_1 {
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
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#7298 <= s_1_0
        fn_state.gs_7298 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_2_0: read-var gs#7298:u8
        let s_2_0: bool = fn_state.gs_7298;
        // N s_2_1: return s_2_0
        return s_2_0;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_3_0: const #16989u : u32
        let s_3_0: u32 = 16989;
        // D s_3_1: read-reg s_3_0:u8
        let s_3_1: bool = {
            let value = state.read_register::<bool>(s_3_0 as isize);
            tracer.read_register(s_3_0 as isize, value);
            value
        };
        // D s_3_2: cast zx s_3_1 -> bv
        let s_3_2: Bits = Bits::new(s_3_1 as u128, 1u16);
        // C s_3_3: const #1u : u8
        let s_3_3: bool = true;
        // C s_3_4: cast zx s_3_3 -> bv
        let s_3_4: Bits = Bits::new(s_3_3 as u128, 1u16);
        // D s_3_5: cmp-eq s_3_2 s_3_4
        let s_3_5: bool = ((s_3_2) == (s_3_4));
        // D s_3_6: write-var gs#7298 <= s_3_5
        fn_state.gs_7298 = s_3_5;
        // N s_3_7: jump b2
        return block_2(state, tracer, fn_state);
    }
}
