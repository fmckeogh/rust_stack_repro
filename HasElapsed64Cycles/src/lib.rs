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
pub fn HasElapsed64Cycles<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_2727: (),
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        return_value: bool,
        gs_2727: (),
    }
    let fn_state = FunctionState {
        gs_2727,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_0_0: const #63s : i
        let s_0_0: i128 = 63;
        // C s_0_1: const #15848u : u32
        let s_0_1: u32 = 15848;
        // D s_0_2: read-reg s_0_1:i
        let s_0_2: i128 = {
            let value = state.read_register::<i128>(s_0_1 as isize);
            tracer.read_register(s_0_1 as isize, value);
            value
        };
        // D s_0_3: cmp-eq s_0_2 s_0_0
        let s_0_3: bool = ((s_0_2) == (s_0_0));
        // N s_0_4: branch s_0_3 b3 b1
        if s_0_3 {
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
        // C s_1_0: const #1s : i
        let s_1_0: i128 = 1;
        // C s_1_1: const #15848u : u32
        let s_1_1: u32 = 15848;
        // D s_1_2: read-reg s_1_1:i
        let s_1_2: i128 = {
            let value = state.read_register::<i128>(s_1_1 as isize);
            tracer.read_register(s_1_1 as isize, value);
            value
        };
        // D s_1_3: add s_1_2 s_1_0
        let s_1_3: i128 = (s_1_2 + s_1_0);
        // C s_1_4: const #15848u : u32
        let s_1_4: u32 = 15848;
        // N s_1_5: write-reg s_1_4 <= s_1_3
        let s_1_5: () = {
            state.write_register::<i128>(s_1_4 as isize, s_1_3);
            tracer.write_register(s_1_4 as isize, s_1_3);
        };
        // C s_1_6: const #0u : u8
        let s_1_6: bool = false;
        // D s_1_7: write-var return_value <= s_1_6
        fn_state.return_value = s_1_6;
        // N s_1_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_2_0: read-var return_value:u8
        let s_2_0: bool = fn_state.return_value;
        // N s_2_1: return s_2_0
        return s_2_0;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_3_0: const #0s : i
        let s_3_0: i128 = 0;
        // C s_3_1: const #15848u : u32
        let s_3_1: u32 = 15848;
        // N s_3_2: write-reg s_3_1 <= s_3_0
        let s_3_2: () = {
            state.write_register::<i128>(s_3_1 as isize, s_3_0);
            tracer.write_register(s_3_1 as isize, s_3_0);
        };
        // C s_3_3: const #1u : u8
        let s_3_3: bool = true;
        // D s_3_4: write-var return_value <= s_3_3
        fn_state.return_value = s_3_3;
        // N s_3_5: jump b2
        return block_2(state, tracer, fn_state);
    }
}
