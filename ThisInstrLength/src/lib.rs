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
pub fn ThisInstrLength<T: Tracer>(state: &mut State, tracer: &T, gs_331129: ()) -> i128 {
    #[derive(Default)]
    struct FunctionState {
        ga_370266: i128,
        gs_331129: (),
    }
    let fn_state = FunctionState {
        gs_331129,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_0_0: const #13600u : u32
        let s_0_0: u32 = 13600;
        // D s_0_1: read-reg s_0_0:u32
        let s_0_1: u32 = {
            let value = state.read_register::<u32>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // C s_0_2: const #2u : u32
        let s_0_2: u32 = 2;
        // D s_0_3: cmp-eq s_0_1 s_0_2
        let s_0_3: bool = ((s_0_1) == (s_0_2));
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
    ) -> i128 {
        // C s_1_0: const #32s : i
        let s_1_0: i128 = 32;
        // D s_1_1: write-var ga#370266 <= s_1_0
        fn_state.ga_370266 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_2_0: read-var ga#370266:i
        let s_2_0: i128 = fn_state.ga_370266;
        // N s_2_1: return s_2_0
        return s_2_0;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_3_0: const #16s : i
        let s_3_0: i128 = 16;
        // D s_3_1: write-var ga#370266 <= s_3_0
        fn_state.ga_370266 = s_3_0;
        // N s_3_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
