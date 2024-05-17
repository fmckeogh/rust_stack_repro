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
use Have56BitVAExt::*;
use Have52BitVAExt::*;
use common::*;
pub fn EncodeVARange<T: Tracer>(state: &mut State, tracer: &T, gs_327656: ()) -> u8 {
    #[derive(Default)]
    struct FunctionState {
        return_value: u8,
        ga_367917: u8,
        gs_327656: (),
    }
    let fn_state = FunctionState {
        gs_327656,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_0_0: const #14216u : u32
        let s_0_0: u32 = 14216;
        // D s_0_1: read-reg s_0_0:i
        let s_0_1: i128 = {
            let value = state.read_register::<i128>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // C s_0_2: const #48s : i
        let s_0_2: i128 = 48;
        // D s_0_3: cmp-eq s_0_1 s_0_2
        let s_0_3: bool = ((s_0_1) == (s_0_2));
        // D s_0_4: not s_0_3
        let s_0_4: bool = !s_0_3;
        // N s_0_5: branch s_0_4 b3 b1
        if s_0_4 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_1_0: const #0u : u8
        let s_1_0: u8 = 0;
        // D s_1_1: write-var return_value <= s_1_0
        fn_state.return_value = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_2_0: read-var return_value:u8
        let s_2_0: u8 = fn_state.return_value;
        // N s_2_1: return s_2_0
        return s_2_0;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_3_0: const #14216u : u32
        let s_3_0: u32 = 14216;
        // D s_3_1: read-reg s_3_0:i
        let s_3_1: i128 = {
            let value = state.read_register::<i128>(s_3_0 as isize);
            tracer.read_register(s_3_0 as isize, value);
            value
        };
        // C s_3_2: const #52s : i
        let s_3_2: i128 = 52;
        // D s_3_3: cmp-eq s_3_1 s_3_2
        let s_3_3: bool = ((s_3_1) == (s_3_2));
        // D s_3_4: not s_3_3
        let s_3_4: bool = !s_3_3;
        // N s_3_5: branch s_3_4 b5 b4
        if s_3_4 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_4_0: const #() : ()
        let s_4_0: () = ();
        // S s_4_1: call Have52BitVAExt(s_4_0)
        let s_4_1: bool = Have52BitVAExt(state, tracer, s_4_0);
        // N s_4_2: assert s_4_1
        let s_4_2: () = assert!(s_4_1);
        // C s_4_3: const #1u : u8
        let s_4_3: u8 = 1;
        // D s_4_4: write-var return_value <= s_4_3
        fn_state.return_value = s_4_3;
        // N s_4_5: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_5_0: const #14216u : u32
        let s_5_0: u32 = 14216;
        // D s_5_1: read-reg s_5_0:i
        let s_5_1: i128 = {
            let value = state.read_register::<i128>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // C s_5_2: const #56s : i
        let s_5_2: i128 = 56;
        // D s_5_3: cmp-eq s_5_1 s_5_2
        let s_5_3: bool = ((s_5_1) == (s_5_2));
        // D s_5_4: not s_5_3
        let s_5_4: bool = !s_5_3;
        // N s_5_5: branch s_5_4 b7 b6
        if s_5_4 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call Have56BitVAExt(s_6_0)
        let s_6_1: bool = Have56BitVAExt(state, tracer, s_6_0);
        // N s_6_2: assert s_6_1
        let s_6_2: () = assert!(s_6_1);
        // C s_6_3: const #1u : u8
        let s_6_3: u8 = 1;
        // D s_6_4: write-var return_value <= s_6_3
        fn_state.return_value = s_6_3;
        // N s_6_5: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u8 {
        // D s_7_0: read-var ga#367917:u8
        let s_7_0: u8 = fn_state.ga_367917;
        // D s_7_1: write-var return_value <= s_7_0
        fn_state.return_value = s_7_0;
        // N s_7_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
