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
use EL2Enabled::*;
use u_get_HCR_EL2_Type_NV::*;
use AArch64_TLBIP_RIPAS2::*;
use AArch64_SystemAccessTrap::*;
use common::*;
pub fn TLBIP_RIPAS2LE1OS_SysOpsWrite128_1423ee9208f18247<T: Tracer>(
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
        gs_138207: bool,
        u__PSTATE_EL: u8,
        u__HCR_EL2_NV: bool,
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
        // C s_0_3: const #102552u : u32
        let s_0_3: u32 = 102552;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_HCR_EL2_Type_NV(s_0_4)
        let s_0_5: bool = u_get_HCR_EL2_Type_NV(state, tracer, s_0_4);
        // D s_0_6: write-var __HCR_EL2_NV <= s_0_5
        fn_state.u__HCR_EL2_NV = s_0_5;
        // D s_0_7: read-var __PSTATE_EL:u8
        let s_0_7: u8 = fn_state.u__PSTATE_EL;
        // D s_0_8: cast zx s_0_7 -> bv
        let s_0_8: Bits = Bits::new(s_0_7 as u128, 2u16);
        // C s_0_9: const #448u : u32
        let s_0_9: u32 = 448;
        // D s_0_10: read-reg s_0_9:u8
        let s_0_10: u8 = {
            let value = state.read_register::<u8>(s_0_9 as isize);
            tracer.read_register(s_0_9 as isize, value);
            value
        };
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 2u16);
        // D s_0_12: cmp-eq s_0_8 s_0_11
        let s_0_12: bool = ((s_0_8) == (s_0_11));
        // N s_0_13: branch s_0_12 b15 b1
        if s_0_12 {
            return block_15(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b9 b2
        if s_1_5 {
            return block_9(state, tracer, fn_state);
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
        // N s_2_6: branch s_2_5 b8 b3
        if s_2_5 {
            return block_8(state, tracer, fn_state);
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
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call EL2Enabled(s_5_0)
        let s_5_1: bool = EL2Enabled(state, tracer, s_5_0);
        // S s_5_2: not s_5_1
        let s_5_2: bool = !s_5_1;
        // N s_5_3: branch s_5_2 b7 b6
        if s_5_2 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #440u : u32
        let s_6_0: u32 = 440;
        // D s_6_1: read-reg s_6_0:u8
        let s_6_1: u8 = {
            let value = state.read_register::<u8>(s_6_0 as isize);
            tracer.read_register(s_6_0 as isize, value);
            value
        };
        // D s_6_2: call SecurityStateAtEL(s_6_1)
        let s_6_2: u32 = SecurityStateAtEL(state, tracer, s_6_1);
        // C s_6_3: const #() : ()
        let s_6_3: () = ();
        // S s_6_4: call VMID_read(s_6_3)
        let s_6_4: u16 = VMID_read(state, tracer, s_6_3);
        // C s_6_5: const #64s : i64
        let s_6_5: i64 = 64;
        // D s_6_6: read-var t2:i
        let s_6_6: i128 = fn_state.t2;
        // D s_6_7: call X_read(s_6_6, s_6_5)
        let s_6_7: Bits = X_read(state, tracer, s_6_6, s_6_5);
        // D s_6_8: cast reint s_6_7 -> u64
        let s_6_8: u64 = (s_6_7.value() as u64);
        // C s_6_9: const #64s : i64
        let s_6_9: i64 = 64;
        // D s_6_10: read-var t:i
        let s_6_10: i128 = fn_state.t;
        // D s_6_11: call X_read(s_6_10, s_6_9)
        let s_6_11: Bits = X_read(state, tracer, s_6_10, s_6_9);
        // D s_6_12: cast reint s_6_11 -> u64
        let s_6_12: u64 = (s_6_11.value() as u64);
        // D s_6_13: cast zx s_6_8 -> bv
        let s_6_13: Bits = Bits::new(s_6_8 as u128, 64u16);
        // D s_6_14: cast zx s_6_12 -> bv
        let s_6_14: Bits = Bits::new(s_6_12 as u128, 64u16);
        // D s_6_15: cast reint s_6_13 -> u128
        let s_6_15: u128 = (s_6_13.value() as u128);
        // D s_6_16: size-of s_6_13
        let s_6_16: u16 = s_6_13.length();
        // D s_6_17: cast reint s_6_14 -> u128
        let s_6_17: u128 = (s_6_14.value() as u128);
        // D s_6_18: size-of s_6_14
        let s_6_18: u16 = s_6_14.length();
        // D s_6_19: lsl s_6_15 s_6_18
        let s_6_19: u128 = s_6_15 << s_6_18;
        // D s_6_20: or s_6_19 s_6_17
        let s_6_20: u128 = ((s_6_19) | (s_6_17));
        // D s_6_21: add s_6_16 s_6_18
        let s_6_21: u16 = (s_6_16 + s_6_18);
        // D s_6_22: create-bits s_6_20 s_6_21
        let s_6_22: Bits = Bits::new(s_6_20, s_6_21);
        // D s_6_23: cast reint s_6_22 -> u128
        let s_6_23: u128 = (s_6_22.value() as u128);
        // C s_6_24: const #4u : u32
        let s_6_24: u32 = 4;
        // C s_6_25: const #2u : u32
        let s_6_25: u32 = 2;
        // C s_6_26: const #1u : u32
        let s_6_26: u32 = 1;
        // C s_6_27: const #0u : u32
        let s_6_27: u32 = 0;
        // D s_6_28: call AArch64_TLBIP_RIPAS2(s_6_2, s_6_24, s_6_4, s_6_25, s_6_26, s_6_27, s_6_23)
        let s_6_28: () = AArch64_TLBIP_RIPAS2(
            state,
            tracer,
            s_6_2,
            s_6_24,
            s_6_4,
            s_6_25,
            s_6_26,
            s_6_27,
            s_6_23,
        );
        // N s_6_29: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #440u : u32
        let s_8_0: u32 = 440;
        // D s_8_1: read-reg s_8_0:u8
        let s_8_1: u8 = {
            let value = state.read_register::<u8>(s_8_0 as isize);
            tracer.read_register(s_8_0 as isize, value);
            value
        };
        // D s_8_2: call SecurityStateAtEL(s_8_1)
        let s_8_2: u32 = SecurityStateAtEL(state, tracer, s_8_1);
        // C s_8_3: const #() : ()
        let s_8_3: () = ();
        // S s_8_4: call VMID_read(s_8_3)
        let s_8_4: u16 = VMID_read(state, tracer, s_8_3);
        // C s_8_5: const #64s : i64
        let s_8_5: i64 = 64;
        // D s_8_6: read-var t2:i
        let s_8_6: i128 = fn_state.t2;
        // D s_8_7: call X_read(s_8_6, s_8_5)
        let s_8_7: Bits = X_read(state, tracer, s_8_6, s_8_5);
        // D s_8_8: cast reint s_8_7 -> u64
        let s_8_8: u64 = (s_8_7.value() as u64);
        // C s_8_9: const #64s : i64
        let s_8_9: i64 = 64;
        // D s_8_10: read-var t:i
        let s_8_10: i128 = fn_state.t;
        // D s_8_11: call X_read(s_8_10, s_8_9)
        let s_8_11: Bits = X_read(state, tracer, s_8_10, s_8_9);
        // D s_8_12: cast reint s_8_11 -> u64
        let s_8_12: u64 = (s_8_11.value() as u64);
        // D s_8_13: cast zx s_8_8 -> bv
        let s_8_13: Bits = Bits::new(s_8_8 as u128, 64u16);
        // D s_8_14: cast zx s_8_12 -> bv
        let s_8_14: Bits = Bits::new(s_8_12 as u128, 64u16);
        // D s_8_15: cast reint s_8_13 -> u128
        let s_8_15: u128 = (s_8_13.value() as u128);
        // D s_8_16: size-of s_8_13
        let s_8_16: u16 = s_8_13.length();
        // D s_8_17: cast reint s_8_14 -> u128
        let s_8_17: u128 = (s_8_14.value() as u128);
        // D s_8_18: size-of s_8_14
        let s_8_18: u16 = s_8_14.length();
        // D s_8_19: lsl s_8_15 s_8_18
        let s_8_19: u128 = s_8_15 << s_8_18;
        // D s_8_20: or s_8_19 s_8_17
        let s_8_20: u128 = ((s_8_19) | (s_8_17));
        // D s_8_21: add s_8_16 s_8_18
        let s_8_21: u16 = (s_8_16 + s_8_18);
        // D s_8_22: create-bits s_8_20 s_8_21
        let s_8_22: Bits = Bits::new(s_8_20, s_8_21);
        // D s_8_23: cast reint s_8_22 -> u128
        let s_8_23: u128 = (s_8_22.value() as u128);
        // C s_8_24: const #4u : u32
        let s_8_24: u32 = 4;
        // C s_8_25: const #2u : u32
        let s_8_25: u32 = 2;
        // C s_8_26: const #1u : u32
        let s_8_26: u32 = 1;
        // C s_8_27: const #0u : u32
        let s_8_27: u32 = 0;
        // D s_8_28: call AArch64_TLBIP_RIPAS2(s_8_2, s_8_24, s_8_4, s_8_25, s_8_26, s_8_27, s_8_23)
        let s_8_28: () = AArch64_TLBIP_RIPAS2(
            state,
            tracer,
            s_8_2,
            s_8_24,
            s_8_4,
            s_8_25,
            s_8_26,
            s_8_27,
            s_8_23,
        );
        // N s_8_29: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #() : ()
        let s_9_0: () = ();
        // S s_9_1: call EL2Enabled(s_9_0)
        let s_9_1: bool = EL2Enabled(state, tracer, s_9_0);
        // N s_9_2: branch s_9_1 b14 b10
        if s_9_1 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #0u : u8
        let s_10_0: bool = false;
        // D s_10_1: write-var gs#138207 <= s_10_0
        fn_state.gs_138207 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#138207:u8
        let s_11_0: bool = fn_state.gs_138207;
        // N s_11_1: branch s_11_0 b13 b12
        if s_11_0 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: panic
        panic!("{:?}", ());
        // N s_12_1: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #20u : u8
        let s_13_0: u8 = 20;
        // C s_13_1: cast zx s_13_0 -> bv
        let s_13_1: Bits = Bits::new(s_13_0 as u128, 8u16);
        // C s_13_2: cast zx s_13_1 -> i
        let s_13_2: i128 = (s_13_1.value() as i128);
        // C s_13_3: cast reint s_13_2 -> i64
        let s_13_3: i64 = (s_13_2 as i64);
        // C s_13_4: cast zx s_13_3 -> i
        let s_13_4: i128 = (i128::try_from(s_13_3).unwrap());
        // C s_13_5: const #432u : u32
        let s_13_5: u32 = 432;
        // D s_13_6: read-reg s_13_5:u8
        let s_13_6: u8 = {
            let value = state.read_register::<u8>(s_13_5 as isize);
            tracer.read_register(s_13_5 as isize, value);
            value
        };
        // D s_13_7: call AArch64_SystemAccessTrap(s_13_6, s_13_4)
        let s_13_7: () = AArch64_SystemAccessTrap(state, tracer, s_13_6, s_13_4);
        // N s_13_8: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var __HCR_EL2_NV:u8
        let s_14_0: bool = fn_state.u__HCR_EL2_NV;
        // D s_14_1: cast zx s_14_0 -> bv
        let s_14_1: Bits = Bits::new(s_14_0 as u128, 1u16);
        // C s_14_2: const #1u : u8
        let s_14_2: bool = true;
        // C s_14_3: cast zx s_14_2 -> bv
        let s_14_3: Bits = Bits::new(s_14_2 as u128, 1u16);
        // D s_14_4: cmp-eq s_14_1 s_14_3
        let s_14_4: bool = ((s_14_1) == (s_14_3));
        // D s_14_5: write-var gs#138207 <= s_14_4
        fn_state.gs_138207 = s_14_4;
        // N s_14_6: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_15_0: panic
        panic!("{:?}", ());
        // N s_15_1: return
        return;
    }
}
