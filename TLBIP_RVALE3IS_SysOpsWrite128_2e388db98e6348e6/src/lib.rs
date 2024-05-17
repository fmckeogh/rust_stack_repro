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
use SecurityStateAtEL::*;
use X_read::*;
use VMID_read::*;
use AArch64_TLBIP_RVA::*;
use common::*;
pub fn TLBIP_RVALE3IS_SysOpsWrite128_2e388db98e6348e6<T: Tracer>(
    state: &mut State,
    tracer: &T,
    el: u8,
    op0: u8,
    op1: u8,
    CRn: u8,
    op2: u8,
    CRm: u8,
    t: i128,
    t2: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        u__PSTATE_EL: u8,
        el: u8,
        op0: u8,
        op1: u8,
        CRn: u8,
        op2: u8,
        CRm: u8,
        t: i128,
        t2: i128,
    }
    let fn_state = FunctionState {
        el,
        op0,
        op1,
        CRn,
        op2,
        CRm,
        t,
        t2,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #16975u : u32
        let s_0_0: u32 = 16975;
        // D s_0_1: read-reg s_0_0:u8
        let s_0_1: u8 = {
            let value = state.read_register::<u8>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: write-var __PSTATE_EL <= s_0_1
        fn_state.u__PSTATE_EL = s_0_1;
        // D s_0_3: read-var __PSTATE_EL:u8
        let s_0_3: u8 = fn_state.u__PSTATE_EL;
        // D s_0_4: cast zx s_0_3 -> bv
        let s_0_4: Bits = Bits::new(s_0_3 as u128, 2u16);
        // C s_0_5: const #448u : u32
        let s_0_5: u32 = 448;
        // D s_0_6: read-reg s_0_5:u8
        let s_0_6: u8 = {
            let value = state.read_register::<u8>(s_0_5 as isize);
            tracer.read_register(s_0_5 as isize, value);
            value
        };
        // D s_0_7: cast zx s_0_6 -> bv
        let s_0_7: Bits = Bits::new(s_0_6 as u128, 2u16);
        // D s_0_8: cmp-eq s_0_4 s_0_7
        let s_0_8: bool = ((s_0_4) == (s_0_7));
        // N s_0_9: branch s_0_8 b8 b1
        if s_0_8 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var __PSTATE_EL:u8
        let s_1_0: u8 = fn_state.u__PSTATE_EL;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 2u16);
        // C s_1_2: const #440u : u32
        let s_1_2: u32 = 440;
        // D s_1_3: read-reg s_1_2:u8
        let s_1_3: u8 = {
            let value = state.read_register::<u8>(s_1_2 as isize);
            tracer.read_register(s_1_2 as isize, value);
            value
        };
        // D s_1_4: cast zx s_1_3 -> bv
        let s_1_4: Bits = Bits::new(s_1_3 as u128, 2u16);
        // D s_1_5: cmp-eq s_1_1 s_1_4
        let s_1_5: bool = ((s_1_1) == (s_1_4));
        // N s_1_6: branch s_1_5 b7 b2
        if s_1_5 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var __PSTATE_EL:u8
        let s_2_0: u8 = fn_state.u__PSTATE_EL;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 2u16);
        // C s_2_2: const #432u : u32
        let s_2_2: u32 = 432;
        // D s_2_3: read-reg s_2_2:u8
        let s_2_3: u8 = {
            let value = state.read_register::<u8>(s_2_2 as isize);
            tracer.read_register(s_2_2 as isize, value);
            value
        };
        // D s_2_4: cast zx s_2_3 -> bv
        let s_2_4: Bits = Bits::new(s_2_3 as u128, 2u16);
        // D s_2_5: cmp-eq s_2_1 s_2_4
        let s_2_5: bool = ((s_2_1) == (s_2_4));
        // N s_2_6: branch s_2_5 b6 b3
        if s_2_5 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var __PSTATE_EL:u8
        let s_3_0: u8 = fn_state.u__PSTATE_EL;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 2u16);
        // C s_3_2: const #424u : u32
        let s_3_2: u32 = 424;
        // D s_3_3: read-reg s_3_2:u8
        let s_3_3: u8 = {
            let value = state.read_register::<u8>(s_3_2 as isize);
            tracer.read_register(s_3_2 as isize, value);
            value
        };
        // D s_3_4: cast zx s_3_3 -> bv
        let s_3_4: Bits = Bits::new(s_3_3 as u128, 2u16);
        // D s_3_5: cmp-eq s_3_1 s_3_4
        let s_3_5: bool = ((s_3_1) == (s_3_4));
        // N s_3_6: branch s_3_5 b5 b4
        if s_3_5 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_4_0: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #424u : u32
        let s_5_0: u32 = 424;
        // D s_5_1: read-reg s_5_0:u8
        let s_5_1: u8 = {
            let value = state.read_register::<u8>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // D s_5_2: call SecurityStateAtEL(s_5_1)
        let s_5_2: u32 = SecurityStateAtEL(state, tracer, s_5_1);
        // C s_5_3: const #() : ()
        let s_5_3: () = ();
        // S s_5_4: call VMID_read(s_5_3)
        let s_5_4: u16 = VMID_read(state, tracer, s_5_3);
        // C s_5_5: const #64s : i64
        let s_5_5: i64 = 64;
        // D s_5_6: read-var t2:i
        let s_5_6: i128 = fn_state.t2;
        // D s_5_7: call X_read(s_5_6, s_5_5)
        let s_5_7: Bits = X_read(state, tracer, s_5_6, s_5_5);
        // D s_5_8: cast reint s_5_7 -> u64
        let s_5_8: u64 = (s_5_7.value() as u64);
        // C s_5_9: const #64s : i64
        let s_5_9: i64 = 64;
        // D s_5_10: read-var t:i
        let s_5_10: i128 = fn_state.t;
        // D s_5_11: call X_read(s_5_10, s_5_9)
        let s_5_11: Bits = X_read(state, tracer, s_5_10, s_5_9);
        // D s_5_12: cast reint s_5_11 -> u64
        let s_5_12: u64 = (s_5_11.value() as u64);
        // D s_5_13: cast zx s_5_8 -> bv
        let s_5_13: Bits = Bits::new(s_5_8 as u128, 64u16);
        // D s_5_14: cast zx s_5_12 -> bv
        let s_5_14: Bits = Bits::new(s_5_12 as u128, 64u16);
        // D s_5_15: cast reint s_5_13 -> u128
        let s_5_15: u128 = (s_5_13.value() as u128);
        // D s_5_16: size-of s_5_13
        let s_5_16: u16 = s_5_13.length();
        // D s_5_17: cast reint s_5_14 -> u128
        let s_5_17: u128 = (s_5_14.value() as u128);
        // D s_5_18: size-of s_5_14
        let s_5_18: u16 = s_5_14.length();
        // D s_5_19: lsl s_5_15 s_5_18
        let s_5_19: u128 = s_5_15 << s_5_18;
        // D s_5_20: or s_5_19 s_5_17
        let s_5_20: u128 = ((s_5_19) | (s_5_17));
        // D s_5_21: add s_5_16 s_5_18
        let s_5_21: u16 = (s_5_16 + s_5_18);
        // D s_5_22: create-bits s_5_20 s_5_21
        let s_5_22: Bits = Bits::new(s_5_20, s_5_21);
        // D s_5_23: cast reint s_5_22 -> u128
        let s_5_23: u128 = (s_5_22.value() as u128);
        // C s_5_24: const #0u : u32
        let s_5_24: u32 = 0;
        // C s_5_25: const #1u : u32
        let s_5_25: u32 = 1;
        // C s_5_26: const #1u : u32
        let s_5_26: u32 = 1;
        // C s_5_27: const #0u : u32
        let s_5_27: u32 = 0;
        // D s_5_28: call AArch64_TLBIP_RVA(s_5_2, s_5_24, s_5_4, s_5_25, s_5_26, s_5_27, s_5_23)
        let s_5_28: () = AArch64_TLBIP_RVA(
            state,
            tracer,
            s_5_2,
            s_5_24,
            s_5_4,
            s_5_25,
            s_5_26,
            s_5_27,
            s_5_23,
        );
        // N s_5_29: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_6_0: panic
        panic!("{:?}", ());
        // N s_6_1: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: panic
        panic!("{:?}", ());
        // N s_7_1: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: panic
        panic!("{:?}", ());
        // N s_8_1: return
        return;
    }
}
