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
pub fn u__GIC_ClearActiveInterrupt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    intid: u32,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        intid: u32,
    }
    let fn_state = FunctionState {
        intid,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #90680u : u32
        let s_0_0: u32 = 90680;
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
    ) -> () {
        // C s_1_0: const #90680u : u32
        let s_1_0: u32 = 90680;
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
        // D s_1_3: call __GIC_InterruptID(s_1_2)
        let s_1_3: u32 = u__GIC_InterruptID(state, tracer, s_1_2);
        // D s_1_4: cast zx s_1_3 -> bv
        let s_1_4: Bits = Bits::new(s_1_3 as u128, 32u16);
        // D s_1_5: read-var intid:u32
        let s_1_5: u32 = fn_state.intid;
        // D s_1_6: cast zx s_1_5 -> bv
        let s_1_6: Bits = Bits::new(s_1_5 as u128, 32u16);
        // D s_1_7: cmp-eq s_1_4 s_1_6
        let s_1_7: bool = ((s_1_4) == (s_1_6));
        // D s_1_8: not s_1_7
        let s_1_8: bool = !s_1_7;
        // N s_1_9: branch s_1_8 b3 b2
        if s_1_8 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #() : ()
        let s_2_0: () = ();
        // D s_2_1: create-sum enum = 0:"s_2_0"
        let s_2_1: SumTypef8de2b264306e832 = SumTypef8de2b264306e832::_0(s_2_0);
        // C s_2_2: const #90680u : u32
        let s_2_2: u32 = 90680;
        // N s_2_3: write-reg s_2_2 <= s_2_1
        let s_2_3: () = {
            state.write_register::<SumTypef8de2b264306e832>(s_2_2 as isize, s_2_1);
            tracer.write_register(s_2_2 as isize, s_2_1);
        };
        // N s_2_4: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_3_0: return
        return;
    }
}
