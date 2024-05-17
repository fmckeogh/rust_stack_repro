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
use u__GIC_InterruptID::*;
use common::*;
pub fn u__GIC_AcknowledgeInterrupt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    gs_331302: (),
) -> u32 {
    #[derive(Default)]
    struct FunctionState {
        gs_331303: u32,
        gs_331302: (),
    }
    let fn_state = FunctionState {
        gs_331302,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_0_0: const #20608u : u32
        let s_0_0: u32 = 20608;
        // D s_0_1: read-reg s_0_0:enum
        let s_0_1: SumTypef8de2b264306e832 = {
            let value = state.read_register::<SumTypef8de2b264306e832>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: matches-sum s_0_1 1
        let s_0_2: bool = matches!(s_0_1, SumTypef8de2b264306e832::_1(_));
        // N s_0_3: branch s_0_2 b3 b1
        if s_0_2 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_1_0: const #20608u : u32
        let s_1_0: u32 = 20608;
        // D s_1_1: read-reg s_1_0:enum
        let s_1_1: SumTypef8de2b264306e832 = {
            let value = state.read_register::<SumTypef8de2b264306e832>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // D s_1_2: unwrap-sum s_1_1 1
        let s_1_2: u32 = match s_1_1 {
            SumTypef8de2b264306e832::_1(inner) => inner,
            _ => panic!("unwrap sum failed"),
        };
        // C s_1_3: const #() : ()
        let s_1_3: () = ();
        // D s_1_4: create-sum enum = 0:"s_1_3"
        let s_1_4: SumTypef8de2b264306e832 = SumTypef8de2b264306e832::_0(s_1_3);
        // C s_1_5: const #20608u : u32
        let s_1_5: u32 = 20608;
        // N s_1_6: write-reg s_1_5 <= s_1_4
        let s_1_6: () = {
            state.write_register::<SumTypef8de2b264306e832>(s_1_5 as isize, s_1_4);
            tracer.write_register(s_1_5 as isize, s_1_4);
        };
        // D s_1_7: create-sum enum = 1:"s_1_2"
        let s_1_7: SumTypef8de2b264306e832 = SumTypef8de2b264306e832::_1(s_1_2);
        // C s_1_8: const #90680u : u32
        let s_1_8: u32 = 90680;
        // N s_1_9: write-reg s_1_8 <= s_1_7
        let s_1_9: () = {
            state.write_register::<SumTypef8de2b264306e832>(s_1_8 as isize, s_1_7);
            tracer.write_register(s_1_8 as isize, s_1_7);
        };
        // D s_1_10: call __GIC_InterruptID(s_1_2)
        let s_1_10: u32 = u__GIC_InterruptID(state, tracer, s_1_2);
        // D s_1_11: write-var gs#331303 <= s_1_10
        fn_state.gs_331303 = s_1_10;
        // N s_1_12: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // D s_2_0: read-var gs#331303:u32
        let s_2_0: u32 = fn_state.gs_331303;
        // N s_2_1: return s_2_0
        return s_2_0;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_3_0: const #20608u : u32
        let s_3_0: u32 = 20608;
        // D s_3_1: read-reg s_3_0:enum
        let s_3_1: SumTypef8de2b264306e832 = {
            let value = state.read_register::<SumTypef8de2b264306e832>(s_3_0 as isize);
            tracer.read_register(s_3_0 as isize, value);
            value
        };
        // D s_3_2: matches-sum s_3_1 0
        let s_3_2: bool = matches!(s_3_1, SumTypef8de2b264306e832::_0(_));
        // N s_3_3: branch s_3_2 b5 b4
        if s_3_2 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // C s_4_0: const #1384u : u32
        let s_4_0: u32 = 1384;
        // D s_4_1: read-reg s_4_0:u32
        let s_4_1: u32 = {
            let value = state.read_register::<u32>(s_4_0 as isize);
            tracer.read_register(s_4_0 as isize, value);
            value
        };
        // D s_4_2: write-var gs#331303 <= s_4_1
        fn_state.gs_331303 = s_4_1;
        // N s_4_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u32 {
        // N s_5_0: jump b2
        return block_2(state, tracer, fn_state);
    }
}
