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
use u_get_HCR_EL2_Type_NV::*;
use IsFeatureImplemented::*;
use SecurityStateAtEL::*;
use VMID_read::*;
use EL2Enabled::*;
use X_read::*;
use AArch64_TLBIP_IPAS2::*;
use AArch64_SystemAccessTrap::*;
use common::*;
pub fn TLBIP_IPAS2E1IS_SysOpsWrite128_18df36996791819c<T: Tracer>(
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
        gs_138112: bool,
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
        // C s_0_7: const #166u : u32
        let s_0_7: u32 = 166;
        // S s_0_8: call IsFeatureImplemented(s_0_7)
        let s_0_8: bool = IsFeatureImplemented(state, tracer, s_0_7);
        // S s_0_9: not s_0_8
        let s_0_9: bool = !s_0_8;
        // N s_0_10: branch s_0_9 b17 b1
        if s_0_9 {
            return block_17(state, tracer, fn_state);
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
        // C s_1_2: const #448u : u32
        let s_1_2: u32 = 448;
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
        // N s_1_6: branch s_1_5 b16 b2
        if s_1_5 {
            return block_16(state, tracer, fn_state);
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
        // C s_2_2: const #440u : u32
        let s_2_2: u32 = 440;
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
        // N s_2_6: branch s_2_5 b10 b3
        if s_2_5 {
            return block_10(state, tracer, fn_state);
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
        // C s_3_2: const #432u : u32
        let s_3_2: u32 = 432;
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
        // N s_3_6: branch s_3_5 b9 b4
        if s_3_5 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var __PSTATE_EL:u8
        let s_4_0: u8 = fn_state.u__PSTATE_EL;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 2u16);
        // C s_4_2: const #424u : u32
        let s_4_2: u32 = 424;
        // D s_4_3: read-reg s_4_2:u8
        let s_4_3: u8 = {
            let value = state.read_register::<u8>(s_4_2 as isize);
            tracer.read_register(s_4_2 as isize, value);
            value
        };
        // D s_4_4: cast zx s_4_3 -> bv
        let s_4_4: Bits = Bits::new(s_4_3 as u128, 2u16);
        // D s_4_5: cmp-eq s_4_1 s_4_4
        let s_4_5: bool = ((s_4_1) == (s_4_4));
        // N s_4_6: branch s_4_5 b6 b5
        if s_4_5 {
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
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call EL2Enabled(s_6_0)
        let s_6_1: bool = EL2Enabled(state, tracer, s_6_0);
        // S s_6_2: not s_6_1
        let s_6_2: bool = !s_6_1;
        // N s_6_3: branch s_6_2 b8 b7
        if s_6_2 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #440u : u32
        let s_7_0: u32 = 440;
        // D s_7_1: read-reg s_7_0:u8
        let s_7_1: u8 = {
            let value = state.read_register::<u8>(s_7_0 as isize);
            tracer.read_register(s_7_0 as isize, value);
            value
        };
        // D s_7_2: call SecurityStateAtEL(s_7_1)
        let s_7_2: u32 = SecurityStateAtEL(state, tracer, s_7_1);
        // C s_7_3: const #() : ()
        let s_7_3: () = ();
        // S s_7_4: call VMID_read(s_7_3)
        let s_7_4: u16 = VMID_read(state, tracer, s_7_3);
        // C s_7_5: const #64s : i64
        let s_7_5: i64 = 64;
        // D s_7_6: read-var t2:i
        let s_7_6: i128 = fn_state.t2;
        // D s_7_7: call X_read(s_7_6, s_7_5)
        let s_7_7: Bits = X_read(state, tracer, s_7_6, s_7_5);
        // D s_7_8: cast reint s_7_7 -> u64
        let s_7_8: u64 = (s_7_7.value() as u64);
        // C s_7_9: const #64s : i64
        let s_7_9: i64 = 64;
        // D s_7_10: read-var t:i
        let s_7_10: i128 = fn_state.t;
        // D s_7_11: call X_read(s_7_10, s_7_9)
        let s_7_11: Bits = X_read(state, tracer, s_7_10, s_7_9);
        // D s_7_12: cast reint s_7_11 -> u64
        let s_7_12: u64 = (s_7_11.value() as u64);
        // D s_7_13: cast zx s_7_8 -> bv
        let s_7_13: Bits = Bits::new(s_7_8 as u128, 64u16);
        // D s_7_14: cast zx s_7_12 -> bv
        let s_7_14: Bits = Bits::new(s_7_12 as u128, 64u16);
        // D s_7_15: cast reint s_7_13 -> u128
        let s_7_15: u128 = (s_7_13.value() as u128);
        // D s_7_16: size-of s_7_13
        let s_7_16: u16 = s_7_13.length();
        // D s_7_17: cast reint s_7_14 -> u128
        let s_7_17: u128 = (s_7_14.value() as u128);
        // D s_7_18: size-of s_7_14
        let s_7_18: u16 = s_7_14.length();
        // D s_7_19: lsl s_7_15 s_7_18
        let s_7_19: u128 = s_7_15 << s_7_18;
        // D s_7_20: or s_7_19 s_7_17
        let s_7_20: u128 = ((s_7_19) | (s_7_17));
        // D s_7_21: add s_7_16 s_7_18
        let s_7_21: u16 = (s_7_16 + s_7_18);
        // D s_7_22: create-bits s_7_20 s_7_21
        let s_7_22: Bits = Bits::new(s_7_20, s_7_21);
        // D s_7_23: cast reint s_7_22 -> u128
        let s_7_23: u128 = (s_7_22.value() as u128);
        // C s_7_24: const #4u : u32
        let s_7_24: u32 = 4;
        // C s_7_25: const #1u : u32
        let s_7_25: u32 = 1;
        // C s_7_26: const #0u : u32
        let s_7_26: u32 = 0;
        // C s_7_27: const #1u : u32
        let s_7_27: u32 = 1;
        // D s_7_28: call AArch64_TLBIP_IPAS2(s_7_2, s_7_24, s_7_4, s_7_25, s_7_26, s_7_27, s_7_23)
        let s_7_28: () = AArch64_TLBIP_IPAS2(
            state,
            tracer,
            s_7_2,
            s_7_24,
            s_7_4,
            s_7_25,
            s_7_26,
            s_7_27,
            s_7_23,
        );
        // N s_7_29: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #440u : u32
        let s_9_0: u32 = 440;
        // D s_9_1: read-reg s_9_0:u8
        let s_9_1: u8 = {
            let value = state.read_register::<u8>(s_9_0 as isize);
            tracer.read_register(s_9_0 as isize, value);
            value
        };
        // D s_9_2: call SecurityStateAtEL(s_9_1)
        let s_9_2: u32 = SecurityStateAtEL(state, tracer, s_9_1);
        // C s_9_3: const #() : ()
        let s_9_3: () = ();
        // S s_9_4: call VMID_read(s_9_3)
        let s_9_4: u16 = VMID_read(state, tracer, s_9_3);
        // C s_9_5: const #64s : i64
        let s_9_5: i64 = 64;
        // D s_9_6: read-var t2:i
        let s_9_6: i128 = fn_state.t2;
        // D s_9_7: call X_read(s_9_6, s_9_5)
        let s_9_7: Bits = X_read(state, tracer, s_9_6, s_9_5);
        // D s_9_8: cast reint s_9_7 -> u64
        let s_9_8: u64 = (s_9_7.value() as u64);
        // C s_9_9: const #64s : i64
        let s_9_9: i64 = 64;
        // D s_9_10: read-var t:i
        let s_9_10: i128 = fn_state.t;
        // D s_9_11: call X_read(s_9_10, s_9_9)
        let s_9_11: Bits = X_read(state, tracer, s_9_10, s_9_9);
        // D s_9_12: cast reint s_9_11 -> u64
        let s_9_12: u64 = (s_9_11.value() as u64);
        // D s_9_13: cast zx s_9_8 -> bv
        let s_9_13: Bits = Bits::new(s_9_8 as u128, 64u16);
        // D s_9_14: cast zx s_9_12 -> bv
        let s_9_14: Bits = Bits::new(s_9_12 as u128, 64u16);
        // D s_9_15: cast reint s_9_13 -> u128
        let s_9_15: u128 = (s_9_13.value() as u128);
        // D s_9_16: size-of s_9_13
        let s_9_16: u16 = s_9_13.length();
        // D s_9_17: cast reint s_9_14 -> u128
        let s_9_17: u128 = (s_9_14.value() as u128);
        // D s_9_18: size-of s_9_14
        let s_9_18: u16 = s_9_14.length();
        // D s_9_19: lsl s_9_15 s_9_18
        let s_9_19: u128 = s_9_15 << s_9_18;
        // D s_9_20: or s_9_19 s_9_17
        let s_9_20: u128 = ((s_9_19) | (s_9_17));
        // D s_9_21: add s_9_16 s_9_18
        let s_9_21: u16 = (s_9_16 + s_9_18);
        // D s_9_22: create-bits s_9_20 s_9_21
        let s_9_22: Bits = Bits::new(s_9_20, s_9_21);
        // D s_9_23: cast reint s_9_22 -> u128
        let s_9_23: u128 = (s_9_22.value() as u128);
        // C s_9_24: const #4u : u32
        let s_9_24: u32 = 4;
        // C s_9_25: const #1u : u32
        let s_9_25: u32 = 1;
        // C s_9_26: const #0u : u32
        let s_9_26: u32 = 0;
        // C s_9_27: const #1u : u32
        let s_9_27: u32 = 1;
        // D s_9_28: call AArch64_TLBIP_IPAS2(s_9_2, s_9_24, s_9_4, s_9_25, s_9_26, s_9_27, s_9_23)
        let s_9_28: () = AArch64_TLBIP_IPAS2(
            state,
            tracer,
            s_9_2,
            s_9_24,
            s_9_4,
            s_9_25,
            s_9_26,
            s_9_27,
            s_9_23,
        );
        // N s_9_29: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call EL2Enabled(s_10_0)
        let s_10_1: bool = EL2Enabled(state, tracer, s_10_0);
        // N s_10_2: branch s_10_1 b15 b11
        if s_10_1 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // D s_11_1: write-var gs#138112 <= s_11_0
        fn_state.gs_138112 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#138112:u8
        let s_12_0: bool = fn_state.gs_138112;
        // N s_12_1: branch s_12_0 b14 b13
        if s_12_0 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: panic
        panic!("{:?}", ());
        // N s_13_1: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #20u : u8
        let s_14_0: u8 = 20;
        // C s_14_1: cast zx s_14_0 -> bv
        let s_14_1: Bits = Bits::new(s_14_0 as u128, 8u16);
        // C s_14_2: cast zx s_14_1 -> i
        let s_14_2: i128 = (s_14_1.value() as i128);
        // C s_14_3: cast reint s_14_2 -> i64
        let s_14_3: i64 = (s_14_2 as i64);
        // C s_14_4: cast zx s_14_3 -> i
        let s_14_4: i128 = (i128::try_from(s_14_3).unwrap());
        // C s_14_5: const #432u : u32
        let s_14_5: u32 = 432;
        // D s_14_6: read-reg s_14_5:u8
        let s_14_6: u8 = {
            let value = state.read_register::<u8>(s_14_5 as isize);
            tracer.read_register(s_14_5 as isize, value);
            value
        };
        // D s_14_7: call AArch64_SystemAccessTrap(s_14_6, s_14_4)
        let s_14_7: () = AArch64_SystemAccessTrap(state, tracer, s_14_6, s_14_4);
        // N s_14_8: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var __HCR_EL2_NV:u8
        let s_15_0: bool = fn_state.u__HCR_EL2_NV;
        // D s_15_1: cast zx s_15_0 -> bv
        let s_15_1: Bits = Bits::new(s_15_0 as u128, 1u16);
        // C s_15_2: const #1u : u8
        let s_15_2: bool = true;
        // C s_15_3: cast zx s_15_2 -> bv
        let s_15_3: Bits = Bits::new(s_15_2 as u128, 1u16);
        // D s_15_4: cmp-eq s_15_1 s_15_3
        let s_15_4: bool = ((s_15_1) == (s_15_3));
        // D s_15_5: write-var gs#138112 <= s_15_4
        fn_state.gs_138112 = s_15_4;
        // N s_15_6: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_16_0: panic
        panic!("{:?}", ());
        // N s_16_1: return
        return;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_17_0: panic
        panic!("{:?}", ());
        // N s_17_1: return
        return;
    }
}