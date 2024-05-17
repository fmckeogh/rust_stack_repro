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
use ComputePACIMPDEF::*;
use UsePACIMP::*;
use ComputePACQARMA::*;
use UsePACQARMA3::*;
use UsePACQARMA5::*;
use common::*;
pub fn ComputePAC<T: Tracer>(
    state: &mut State,
    tracer: &T,
    data: u64,
    modifier: u64,
    key0: u64,
    key1: u64,
    isgeneric: bool,
) -> u64 {
    #[derive(Default)]
    struct FunctionState {
        ga_10775: u64,
        return_value: u64,
        data: u64,
        modifier: u64,
        key0: u64,
        key1: u64,
        isgeneric: bool,
    }
    let fn_state = FunctionState {
        data,
        modifier,
        key0,
        key1,
        isgeneric,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_0_0: read-var isgeneric:u8
        let s_0_0: bool = fn_state.isgeneric;
        // D s_0_1: call UsePACIMP(s_0_0)
        let s_0_1: bool = UsePACIMP(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b7 b1
        if s_0_1 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_1_0: read-var isgeneric:u8
        let s_1_0: bool = fn_state.isgeneric;
        // D s_1_1: call UsePACQARMA3(s_1_0)
        let s_1_1: bool = UsePACQARMA3(state, tracer, s_1_0);
        // N s_1_2: branch s_1_1 b6 b2
        if s_1_1 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_2_0: read-var isgeneric:u8
        let s_2_0: bool = fn_state.isgeneric;
        // D s_2_1: call UsePACQARMA5(s_2_0)
        let s_2_1: bool = UsePACQARMA5(state, tracer, s_2_0);
        // N s_2_2: branch s_2_1 b5 b3
        if s_2_1 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_3_0: read-var ga#10775:u64
        let s_3_0: u64 = fn_state.ga_10775;
        // D s_3_1: write-var return_value <= s_3_0
        fn_state.return_value = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_4_0: read-var return_value:u64
        let s_4_0: u64 = fn_state.return_value;
        // N s_4_1: return s_4_0
        return s_4_0;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: read-var data:u64
        let s_5_1: u64 = fn_state.data;
        // D s_5_2: read-var modifier:u64
        let s_5_2: u64 = fn_state.modifier;
        // D s_5_3: read-var key0:u64
        let s_5_3: u64 = fn_state.key0;
        // D s_5_4: read-var key1:u64
        let s_5_4: u64 = fn_state.key1;
        // D s_5_5: call ComputePACQARMA(s_5_1, s_5_2, s_5_3, s_5_4, s_5_0)
        let s_5_5: u64 = ComputePACQARMA(
            state,
            tracer,
            s_5_1,
            s_5_2,
            s_5_3,
            s_5_4,
            s_5_0,
        );
        // D s_5_6: write-var return_value <= s_5_5
        fn_state.return_value = s_5_5;
        // N s_5_7: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_6_0: const #1u : u8
        let s_6_0: bool = true;
        // D s_6_1: read-var data:u64
        let s_6_1: u64 = fn_state.data;
        // D s_6_2: read-var modifier:u64
        let s_6_2: u64 = fn_state.modifier;
        // D s_6_3: read-var key0:u64
        let s_6_3: u64 = fn_state.key0;
        // D s_6_4: read-var key1:u64
        let s_6_4: u64 = fn_state.key1;
        // D s_6_5: call ComputePACQARMA(s_6_1, s_6_2, s_6_3, s_6_4, s_6_0)
        let s_6_5: u64 = ComputePACQARMA(
            state,
            tracer,
            s_6_1,
            s_6_2,
            s_6_3,
            s_6_4,
            s_6_0,
        );
        // D s_6_6: write-var return_value <= s_6_5
        fn_state.return_value = s_6_5;
        // N s_6_7: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_7_0: read-var data:u64
        let s_7_0: u64 = fn_state.data;
        // D s_7_1: read-var modifier:u64
        let s_7_1: u64 = fn_state.modifier;
        // D s_7_2: read-var key0:u64
        let s_7_2: u64 = fn_state.key0;
        // D s_7_3: read-var key1:u64
        let s_7_3: u64 = fn_state.key1;
        // D s_7_4: call ComputePACIMPDEF(s_7_0, s_7_1, s_7_2, s_7_3)
        let s_7_4: u64 = ComputePACIMPDEF(state, tracer, s_7_0, s_7_1, s_7_2, s_7_3);
        // D s_7_5: write-var return_value <= s_7_4
        fn_state.return_value = s_7_4;
        // N s_7_6: jump b4
        return block_4(state, tracer, fn_state);
    }
}
