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
use UsingAArch32::*;
use ThrowSError::*;
use AArch32_PendingUnmaskedPhysicalInterrupts::*;
use AArch64_PendingUnmaskedPhysicalInterrupts::*;
use common::*;
pub fn TakeUnmaskedPhysicalSErrorInterrupts<T: Tracer>(
    state: &mut State,
    tracer: &T,
    iesb_req: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_367902: ProductTyped8f896a024a4e2cb,
        ga_367896: ProductTyped8f896a024a4e2cb,
        iesb_req: bool,
    }
    let fn_state = FunctionState {
        iesb_req,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call UsingAArch32(s_0_0)
        let s_0_1: bool = UsingAArch32(state, tracer, s_0_0);
        // N s_0_2: branch s_0_1 b4 b1
        if s_0_1 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #16968u : u32
        let s_1_0: u32 = 16968;
        // D s_1_1: read-reg s_1_0:u8
        let s_1_1: bool = {
            let value = state.read_register::<bool>(s_1_0 as isize);
            tracer.read_register(s_1_0 as isize, value);
            value
        };
        // C s_1_2: const #16979u : u32
        let s_1_2: u32 = 16979;
        // D s_1_3: read-reg s_1_2:u8
        let s_1_3: bool = {
            let value = state.read_register::<bool>(s_1_2 as isize);
            tracer.read_register(s_1_2 as isize, value);
            value
        };
        // C s_1_4: const #16977u : u32
        let s_1_4: u32 = 16977;
        // D s_1_5: read-reg s_1_4:u8
        let s_1_5: bool = {
            let value = state.read_register::<bool>(s_1_4 as isize);
            tracer.read_register(s_1_4 as isize, value);
            value
        };
        // D s_1_6: cast zx s_1_3 -> bv
        let s_1_6: Bits = Bits::new(s_1_3 as u128, 1u16);
        // D s_1_7: cast zx s_1_5 -> bv
        let s_1_7: Bits = Bits::new(s_1_5 as u128, 1u16);
        // D s_1_8: cast reint s_1_6 -> u128
        let s_1_8: u128 = (s_1_6.value() as u128);
        // D s_1_9: size-of s_1_6
        let s_1_9: u16 = s_1_6.length();
        // D s_1_10: cast reint s_1_7 -> u128
        let s_1_10: u128 = (s_1_7.value() as u128);
        // D s_1_11: size-of s_1_7
        let s_1_11: u16 = s_1_7.length();
        // D s_1_12: lsl s_1_8 s_1_11
        let s_1_12: u128 = s_1_8 << s_1_11;
        // D s_1_13: or s_1_12 s_1_10
        let s_1_13: u128 = ((s_1_12) | (s_1_10));
        // D s_1_14: add s_1_9 s_1_11
        let s_1_14: u16 = (s_1_9 + s_1_11);
        // D s_1_15: create-bits s_1_13 s_1_14
        let s_1_15: Bits = Bits::new(s_1_13, s_1_14);
        // D s_1_16: cast reint s_1_15 -> u8
        let s_1_16: u8 = (s_1_15.value() as u8);
        // D s_1_17: cast zx s_1_1 -> bv
        let s_1_17: Bits = Bits::new(s_1_1 as u128, 1u16);
        // D s_1_18: cast zx s_1_16 -> bv
        let s_1_18: Bits = Bits::new(s_1_16 as u128, 2u16);
        // D s_1_19: cast reint s_1_17 -> u128
        let s_1_19: u128 = (s_1_17.value() as u128);
        // D s_1_20: size-of s_1_17
        let s_1_20: u16 = s_1_17.length();
        // D s_1_21: cast reint s_1_18 -> u128
        let s_1_21: u128 = (s_1_18.value() as u128);
        // D s_1_22: size-of s_1_18
        let s_1_22: u16 = s_1_18.length();
        // D s_1_23: lsl s_1_19 s_1_22
        let s_1_23: u128 = s_1_19 << s_1_22;
        // D s_1_24: or s_1_23 s_1_21
        let s_1_24: u128 = ((s_1_23) | (s_1_21));
        // D s_1_25: add s_1_20 s_1_22
        let s_1_25: u16 = (s_1_20 + s_1_22);
        // D s_1_26: create-bits s_1_24 s_1_25
        let s_1_26: Bits = Bits::new(s_1_24, s_1_25);
        // D s_1_27: cast reint s_1_26 -> u8
        let s_1_27: u8 = (s_1_26.value() as u8);
        // D s_1_28: call AArch64_PendingUnmaskedPhysicalInterrupts(s_1_27)
        let s_1_28: ProductTyped8f896a024a4e2cb = AArch64_PendingUnmaskedPhysicalInterrupts(
            state,
            tracer,
            s_1_27,
        );
        // D s_1_29: write-var ga#367902 <= s_1_28
        fn_state.ga_367902 = s_1_28;
        // D s_1_30: read-var ga#367902.0:struct
        let s_1_30: bool = fn_state.ga_367902._0;
        // N s_1_31: branch s_1_30 b3 b2
        if s_1_30 {
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
        // N s_2_0: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var iesb_req:u8
        let s_3_0: bool = fn_state.iesb_req;
        // D s_3_1: call ThrowSError(s_3_0)
        let s_3_1: () = ThrowSError(state, tracer, s_3_0);
        // N s_3_2: return
        return;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #() : ()
        let s_4_0: () = ();
        // S s_4_1: call AArch32_PendingUnmaskedPhysicalInterrupts(s_4_0)
        let s_4_1: ProductTyped8f896a024a4e2cb = AArch32_PendingUnmaskedPhysicalInterrupts(
            state,
            tracer,
            s_4_0,
        );
        // D s_4_2: write-var ga#367896 <= s_4_1
        fn_state.ga_367896 = s_4_1;
        // D s_4_3: read-var ga#367896.0:struct
        let s_4_3: bool = fn_state.ga_367896._0;
        // N s_4_4: branch s_4_3 b6 b5
        if s_4_3 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var iesb_req:u8
        let s_6_0: bool = fn_state.iesb_req;
        // D s_6_1: call ThrowSError(s_6_0)
        let s_6_1: () = ThrowSError(state, tracer, s_6_0);
        // N s_6_2: return
        return;
    }
}
