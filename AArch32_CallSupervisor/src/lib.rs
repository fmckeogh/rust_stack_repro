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
use AArch32_CurrentCond::*;
use AArch32_TakeSVCException::*;
use u__UNKNOWN_bits::*;
use AArch64_CallSupervisor::*;
use AArch32_GeneralExceptionsToAArch64::*;
use common::*;
pub fn AArch32_CallSupervisor<T: Tracer>(
    state: &mut State,
    tracer: &T,
    immediate_in: u16,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        immediate: u16,
        immediate_in: u16,
    }
    let fn_state = FunctionState {
        immediate_in,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var immediate_in:u16
        let s_0_0: u16 = fn_state.immediate_in;
        // D s_0_1: write-var immediate <= s_0_0
        fn_state.immediate = s_0_0;
        // C s_0_2: const #() : ()
        let s_0_2: () = ();
        // S s_0_3: call AArch32_CurrentCond(s_0_2)
        let s_0_3: u8 = AArch32_CurrentCond(state, tracer, s_0_2);
        // S s_0_4: cast zx s_0_3 -> bv
        let s_0_4: Bits = Bits::new(s_0_3 as u128, 4u16);
        // C s_0_5: const #14u : u8
        let s_0_5: u8 = 14;
        // C s_0_6: cast zx s_0_5 -> bv
        let s_0_6: Bits = Bits::new(s_0_5 as u128, 4u16);
        // S s_0_7: cmp-ne s_0_4 s_0_6
        let s_0_7: bool = ((s_0_4) != (s_0_6));
        // N s_0_8: branch s_0_7 b5 b1
        if s_0_7 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_1_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #() : ()
        let s_2_0: () = ();
        // S s_2_1: call AArch32_GeneralExceptionsToAArch64(s_2_0)
        let s_2_1: bool = AArch32_GeneralExceptionsToAArch64(state, tracer, s_2_0);
        // N s_2_2: branch s_2_1 b4 b3
        if s_2_1 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var immediate:u16
        let s_3_0: u16 = fn_state.immediate;
        // D s_3_1: call AArch32_TakeSVCException(s_3_0)
        let s_3_1: () = AArch32_TakeSVCException(state, tracer, s_3_0);
        // N s_3_2: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var immediate:u16
        let s_4_0: u16 = fn_state.immediate;
        // D s_4_1: call AArch64_CallSupervisor(s_4_0)
        let s_4_1: () = AArch64_CallSupervisor(state, tracer, s_4_0);
        // N s_4_2: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #16s : i64
        let s_5_0: i64 = 16;
        // C s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // S s_5_2: call __UNKNOWN_bits(s_5_1)
        let s_5_2: Bits = u__UNKNOWN_bits(state, tracer, s_5_1);
        // S s_5_3: cast reint s_5_2 -> u16
        let s_5_3: u16 = (s_5_2.value() as u16);
        // D s_5_4: write-var immediate <= s_5_3
        fn_state.immediate = s_5_3;
        // N s_5_5: jump b2
        return block_2(state, tracer, fn_state);
    }
}
