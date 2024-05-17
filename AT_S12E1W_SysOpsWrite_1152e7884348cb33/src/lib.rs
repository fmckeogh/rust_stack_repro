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
use u_get_HCR_EL2_Type_DC::*;
use AArch64_AT::*;
use u_get_HCR_EL2_Type_E2H::*;
use u_get_HCR_EL2_Type_NV::*;
use X_read::*;
use AArch64_SystemAccessTrap::*;
use EL2Enabled::*;
use u_get_HCR_EL2_Type_TGE::*;
use u_get_HCR_EL2_Type_VM::*;
use common::*;
pub fn AT_S12E1W_SysOpsWrite_1152e7884348cb33<T: Tracer>(
    state: &mut State,
    tracer: &T,
    el: u8,
    op0: u8,
    op1: u8,
    CRn: u8,
    op2: u8,
    CRm: u8,
    t: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_101204: bool,
        gs_101201: bool,
        gs_101197: bool,
        u__PSTATE_EL: u8,
        u__HCR_EL2_NV: bool,
        gs_101196: bool,
        el: u8,
        op0: u8,
        op1: u8,
        CRn: u8,
        op2: u8,
        CRm: u8,
        t: i128,
    }
    let fn_state = FunctionState {
        el,
        op0,
        op1,
        CRn,
        op2,
        CRm,
        t,
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
        // N s_0_13: branch s_0_12 b28 b1
        if s_0_12 {
            return block_28(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b22 b2
        if s_1_5 {
            return block_22(state, tracer, fn_state);
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
        // N s_2_6: branch s_2_5 b16 b3
        if s_2_5 {
            return block_16(state, tracer, fn_state);
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
        // N s_5_3: branch s_5_2 b15 b6
        if s_5_2 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
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
        // N s_6_2: branch s_6_1 b11 b7
        if s_6_1 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#101197 <= s_7_0
        fn_state.gs_101197 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#101197:u8
        let s_8_0: bool = fn_state.gs_101197;
        // N s_8_1: branch s_8_0 b10 b9
        if s_8_0 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #64s : i64
        let s_9_0: i64 = 64;
        // D s_9_1: read-var t:i
        let s_9_1: i128 = fn_state.t;
        // D s_9_2: call X_read(s_9_1, s_9_0)
        let s_9_2: Bits = X_read(state, tracer, s_9_1, s_9_0);
        // D s_9_3: cast reint s_9_2 -> u64
        let s_9_3: u64 = (s_9_2.value() as u64);
        // C s_9_4: const #1u : u32
        let s_9_4: u32 = 1;
        // C s_9_5: const #440u : u32
        let s_9_5: u32 = 440;
        // D s_9_6: read-reg s_9_5:u8
        let s_9_6: u8 = {
            let value = state.read_register::<u8>(s_9_5 as isize);
            tracer.read_register(s_9_5 as isize, value);
            value
        };
        // C s_9_7: const #1u : u32
        let s_9_7: u32 = 1;
        // D s_9_8: call AArch64_AT(s_9_3, s_9_4, s_9_6, s_9_7)
        let s_9_8: () = AArch64_AT(state, tracer, s_9_3, s_9_4, s_9_6, s_9_7);
        // N s_9_9: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #64s : i64
        let s_10_0: i64 = 64;
        // D s_10_1: read-var t:i
        let s_10_1: i128 = fn_state.t;
        // D s_10_2: call X_read(s_10_1, s_10_0)
        let s_10_2: Bits = X_read(state, tracer, s_10_1, s_10_0);
        // D s_10_3: cast reint s_10_2 -> u64
        let s_10_3: u64 = (s_10_2.value() as u64);
        // C s_10_4: const #0u : u32
        let s_10_4: u32 = 0;
        // C s_10_5: const #440u : u32
        let s_10_5: u32 = 440;
        // D s_10_6: read-reg s_10_5:u8
        let s_10_6: u8 = {
            let value = state.read_register::<u8>(s_10_5 as isize);
            tracer.read_register(s_10_5 as isize, value);
            value
        };
        // C s_10_7: const #1u : u32
        let s_10_7: u32 = 1;
        // D s_10_8: call AArch64_AT(s_10_3, s_10_4, s_10_6, s_10_7)
        let s_10_8: () = AArch64_AT(state, tracer, s_10_3, s_10_4, s_10_6, s_10_7);
        // N s_10_9: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #102552u : u32
        let s_11_0: u32 = 102552;
        // D s_11_1: read-reg s_11_0:struct
        let s_11_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_11_0 as isize);
            tracer.read_register(s_11_0 as isize, value);
            value
        };
        // D s_11_2: call _get_HCR_EL2_Type_E2H(s_11_1)
        let s_11_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_11_1);
        // C s_11_3: const #102552u : u32
        let s_11_3: u32 = 102552;
        // D s_11_4: read-reg s_11_3:struct
        let s_11_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_11_3 as isize);
            tracer.read_register(s_11_3 as isize, value);
            value
        };
        // D s_11_5: call _get_HCR_EL2_Type_TGE(s_11_4)
        let s_11_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_11_4);
        // D s_11_6: cast zx s_11_2 -> bv
        let s_11_6: Bits = Bits::new(s_11_2 as u128, 1u16);
        // D s_11_7: cast zx s_11_5 -> bv
        let s_11_7: Bits = Bits::new(s_11_5 as u128, 1u16);
        // D s_11_8: cast reint s_11_6 -> u128
        let s_11_8: u128 = (s_11_6.value() as u128);
        // D s_11_9: size-of s_11_6
        let s_11_9: u16 = s_11_6.length();
        // D s_11_10: cast reint s_11_7 -> u128
        let s_11_10: u128 = (s_11_7.value() as u128);
        // D s_11_11: size-of s_11_7
        let s_11_11: u16 = s_11_7.length();
        // D s_11_12: lsl s_11_8 s_11_11
        let s_11_12: u128 = s_11_8 << s_11_11;
        // D s_11_13: or s_11_12 s_11_10
        let s_11_13: u128 = ((s_11_12) | (s_11_10));
        // D s_11_14: add s_11_9 s_11_11
        let s_11_14: u16 = (s_11_9 + s_11_11);
        // D s_11_15: create-bits s_11_13 s_11_14
        let s_11_15: Bits = Bits::new(s_11_13, s_11_14);
        // D s_11_16: cast reint s_11_15 -> u8
        let s_11_16: u8 = (s_11_15.value() as u8);
        // D s_11_17: cast zx s_11_16 -> bv
        let s_11_17: Bits = Bits::new(s_11_16 as u128, 2u16);
        // C s_11_18: const #3u : u8
        let s_11_18: u8 = 3;
        // C s_11_19: cast zx s_11_18 -> bv
        let s_11_19: Bits = Bits::new(s_11_18 as u128, 2u16);
        // D s_11_20: cmp-eq s_11_17 s_11_19
        let s_11_20: bool = ((s_11_17) == (s_11_19));
        // N s_11_21: branch s_11_20 b14 b12
        if s_11_20 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #102552u : u32
        let s_12_0: u32 = 102552;
        // D s_12_1: read-reg s_12_0:struct
        let s_12_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_0 as isize);
            tracer.read_register(s_12_0 as isize, value);
            value
        };
        // D s_12_2: call _get_HCR_EL2_Type_DC(s_12_1)
        let s_12_2: bool = u_get_HCR_EL2_Type_DC(state, tracer, s_12_1);
        // C s_12_3: const #102552u : u32
        let s_12_3: u32 = 102552;
        // D s_12_4: read-reg s_12_3:struct
        let s_12_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_3 as isize);
            tracer.read_register(s_12_3 as isize, value);
            value
        };
        // D s_12_5: call _get_HCR_EL2_Type_VM(s_12_4)
        let s_12_5: bool = u_get_HCR_EL2_Type_VM(state, tracer, s_12_4);
        // D s_12_6: cast zx s_12_2 -> bv
        let s_12_6: Bits = Bits::new(s_12_2 as u128, 1u16);
        // D s_12_7: cast zx s_12_5 -> bv
        let s_12_7: Bits = Bits::new(s_12_5 as u128, 1u16);
        // D s_12_8: cast reint s_12_6 -> u128
        let s_12_8: u128 = (s_12_6.value() as u128);
        // D s_12_9: size-of s_12_6
        let s_12_9: u16 = s_12_6.length();
        // D s_12_10: cast reint s_12_7 -> u128
        let s_12_10: u128 = (s_12_7.value() as u128);
        // D s_12_11: size-of s_12_7
        let s_12_11: u16 = s_12_7.length();
        // D s_12_12: lsl s_12_8 s_12_11
        let s_12_12: u128 = s_12_8 << s_12_11;
        // D s_12_13: or s_12_12 s_12_10
        let s_12_13: u128 = ((s_12_12) | (s_12_10));
        // D s_12_14: add s_12_9 s_12_11
        let s_12_14: u16 = (s_12_9 + s_12_11);
        // D s_12_15: create-bits s_12_13 s_12_14
        let s_12_15: Bits = Bits::new(s_12_13, s_12_14);
        // D s_12_16: cast reint s_12_15 -> u8
        let s_12_16: u8 = (s_12_15.value() as u8);
        // D s_12_17: cast zx s_12_16 -> bv
        let s_12_17: Bits = Bits::new(s_12_16 as u128, 2u16);
        // C s_12_18: const #0u : u8
        let s_12_18: u8 = 0;
        // C s_12_19: cast zx s_12_18 -> bv
        let s_12_19: Bits = Bits::new(s_12_18 as u128, 2u16);
        // D s_12_20: cmp-eq s_12_17 s_12_19
        let s_12_20: bool = ((s_12_17) == (s_12_19));
        // D s_12_21: write-var gs#101196 <= s_12_20
        fn_state.gs_101196 = s_12_20;
        // N s_12_22: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#101196:u8
        let s_13_0: bool = fn_state.gs_101196;
        // D s_13_1: write-var gs#101197 <= s_13_0
        fn_state.gs_101197 = s_13_0;
        // N s_13_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #1u : u8
        let s_14_0: bool = true;
        // D s_14_1: write-var gs#101196 <= s_14_0
        fn_state.gs_101196 = s_14_0;
        // N s_14_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #64s : i64
        let s_15_0: i64 = 64;
        // D s_15_1: read-var t:i
        let s_15_1: i128 = fn_state.t;
        // D s_15_2: call X_read(s_15_1, s_15_0)
        let s_15_2: Bits = X_read(state, tracer, s_15_1, s_15_0);
        // D s_15_3: cast reint s_15_2 -> u64
        let s_15_3: u64 = (s_15_2.value() as u64);
        // C s_15_4: const #0u : u32
        let s_15_4: u32 = 0;
        // C s_15_5: const #440u : u32
        let s_15_5: u32 = 440;
        // D s_15_6: read-reg s_15_5:u8
        let s_15_6: u8 = {
            let value = state.read_register::<u8>(s_15_5 as isize);
            tracer.read_register(s_15_5 as isize, value);
            value
        };
        // C s_15_7: const #1u : u32
        let s_15_7: u32 = 1;
        // D s_15_8: call AArch64_AT(s_15_3, s_15_4, s_15_6, s_15_7)
        let s_15_8: () = AArch64_AT(state, tracer, s_15_3, s_15_4, s_15_6, s_15_7);
        // N s_15_9: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #102552u : u32
        let s_16_0: u32 = 102552;
        // D s_16_1: read-reg s_16_0:struct
        let s_16_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_16_0 as isize);
            tracer.read_register(s_16_0 as isize, value);
            value
        };
        // D s_16_2: call _get_HCR_EL2_Type_E2H(s_16_1)
        let s_16_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_16_1);
        // C s_16_3: const #102552u : u32
        let s_16_3: u32 = 102552;
        // D s_16_4: read-reg s_16_3:struct
        let s_16_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_16_3 as isize);
            tracer.read_register(s_16_3 as isize, value);
            value
        };
        // D s_16_5: call _get_HCR_EL2_Type_TGE(s_16_4)
        let s_16_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_16_4);
        // D s_16_6: cast zx s_16_2 -> bv
        let s_16_6: Bits = Bits::new(s_16_2 as u128, 1u16);
        // D s_16_7: cast zx s_16_5 -> bv
        let s_16_7: Bits = Bits::new(s_16_5 as u128, 1u16);
        // D s_16_8: cast reint s_16_6 -> u128
        let s_16_8: u128 = (s_16_6.value() as u128);
        // D s_16_9: size-of s_16_6
        let s_16_9: u16 = s_16_6.length();
        // D s_16_10: cast reint s_16_7 -> u128
        let s_16_10: u128 = (s_16_7.value() as u128);
        // D s_16_11: size-of s_16_7
        let s_16_11: u16 = s_16_7.length();
        // D s_16_12: lsl s_16_8 s_16_11
        let s_16_12: u128 = s_16_8 << s_16_11;
        // D s_16_13: or s_16_12 s_16_10
        let s_16_13: u128 = ((s_16_12) | (s_16_10));
        // D s_16_14: add s_16_9 s_16_11
        let s_16_14: u16 = (s_16_9 + s_16_11);
        // D s_16_15: create-bits s_16_13 s_16_14
        let s_16_15: Bits = Bits::new(s_16_13, s_16_14);
        // D s_16_16: cast reint s_16_15 -> u8
        let s_16_16: u8 = (s_16_15.value() as u8);
        // D s_16_17: cast zx s_16_16 -> bv
        let s_16_17: Bits = Bits::new(s_16_16 as u128, 2u16);
        // C s_16_18: const #3u : u8
        let s_16_18: u8 = 3;
        // C s_16_19: cast zx s_16_18 -> bv
        let s_16_19: Bits = Bits::new(s_16_18 as u128, 2u16);
        // D s_16_20: cmp-eq s_16_17 s_16_19
        let s_16_20: bool = ((s_16_17) == (s_16_19));
        // N s_16_21: branch s_16_20 b21 b17
        if s_16_20 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #102552u : u32
        let s_17_0: u32 = 102552;
        // D s_17_1: read-reg s_17_0:struct
        let s_17_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_17_0 as isize);
            tracer.read_register(s_17_0 as isize, value);
            value
        };
        // D s_17_2: call _get_HCR_EL2_Type_DC(s_17_1)
        let s_17_2: bool = u_get_HCR_EL2_Type_DC(state, tracer, s_17_1);
        // C s_17_3: const #102552u : u32
        let s_17_3: u32 = 102552;
        // D s_17_4: read-reg s_17_3:struct
        let s_17_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_17_3 as isize);
            tracer.read_register(s_17_3 as isize, value);
            value
        };
        // D s_17_5: call _get_HCR_EL2_Type_VM(s_17_4)
        let s_17_5: bool = u_get_HCR_EL2_Type_VM(state, tracer, s_17_4);
        // D s_17_6: cast zx s_17_2 -> bv
        let s_17_6: Bits = Bits::new(s_17_2 as u128, 1u16);
        // D s_17_7: cast zx s_17_5 -> bv
        let s_17_7: Bits = Bits::new(s_17_5 as u128, 1u16);
        // D s_17_8: cast reint s_17_6 -> u128
        let s_17_8: u128 = (s_17_6.value() as u128);
        // D s_17_9: size-of s_17_6
        let s_17_9: u16 = s_17_6.length();
        // D s_17_10: cast reint s_17_7 -> u128
        let s_17_10: u128 = (s_17_7.value() as u128);
        // D s_17_11: size-of s_17_7
        let s_17_11: u16 = s_17_7.length();
        // D s_17_12: lsl s_17_8 s_17_11
        let s_17_12: u128 = s_17_8 << s_17_11;
        // D s_17_13: or s_17_12 s_17_10
        let s_17_13: u128 = ((s_17_12) | (s_17_10));
        // D s_17_14: add s_17_9 s_17_11
        let s_17_14: u16 = (s_17_9 + s_17_11);
        // D s_17_15: create-bits s_17_13 s_17_14
        let s_17_15: Bits = Bits::new(s_17_13, s_17_14);
        // D s_17_16: cast reint s_17_15 -> u8
        let s_17_16: u8 = (s_17_15.value() as u8);
        // D s_17_17: cast zx s_17_16 -> bv
        let s_17_17: Bits = Bits::new(s_17_16 as u128, 2u16);
        // C s_17_18: const #0u : u8
        let s_17_18: u8 = 0;
        // C s_17_19: cast zx s_17_18 -> bv
        let s_17_19: Bits = Bits::new(s_17_18 as u128, 2u16);
        // D s_17_20: cmp-eq s_17_17 s_17_19
        let s_17_20: bool = ((s_17_17) == (s_17_19));
        // D s_17_21: write-var gs#101201 <= s_17_20
        fn_state.gs_101201 = s_17_20;
        // N s_17_22: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#101201:u8
        let s_18_0: bool = fn_state.gs_101201;
        // N s_18_1: branch s_18_0 b20 b19
        if s_18_0 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #64s : i64
        let s_19_0: i64 = 64;
        // D s_19_1: read-var t:i
        let s_19_1: i128 = fn_state.t;
        // D s_19_2: call X_read(s_19_1, s_19_0)
        let s_19_2: Bits = X_read(state, tracer, s_19_1, s_19_0);
        // D s_19_3: cast reint s_19_2 -> u64
        let s_19_3: u64 = (s_19_2.value() as u64);
        // C s_19_4: const #1u : u32
        let s_19_4: u32 = 1;
        // C s_19_5: const #440u : u32
        let s_19_5: u32 = 440;
        // D s_19_6: read-reg s_19_5:u8
        let s_19_6: u8 = {
            let value = state.read_register::<u8>(s_19_5 as isize);
            tracer.read_register(s_19_5 as isize, value);
            value
        };
        // C s_19_7: const #1u : u32
        let s_19_7: u32 = 1;
        // D s_19_8: call AArch64_AT(s_19_3, s_19_4, s_19_6, s_19_7)
        let s_19_8: () = AArch64_AT(state, tracer, s_19_3, s_19_4, s_19_6, s_19_7);
        // N s_19_9: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #64s : i64
        let s_20_0: i64 = 64;
        // D s_20_1: read-var t:i
        let s_20_1: i128 = fn_state.t;
        // D s_20_2: call X_read(s_20_1, s_20_0)
        let s_20_2: Bits = X_read(state, tracer, s_20_1, s_20_0);
        // D s_20_3: cast reint s_20_2 -> u64
        let s_20_3: u64 = (s_20_2.value() as u64);
        // C s_20_4: const #0u : u32
        let s_20_4: u32 = 0;
        // C s_20_5: const #440u : u32
        let s_20_5: u32 = 440;
        // D s_20_6: read-reg s_20_5:u8
        let s_20_6: u8 = {
            let value = state.read_register::<u8>(s_20_5 as isize);
            tracer.read_register(s_20_5 as isize, value);
            value
        };
        // C s_20_7: const #1u : u32
        let s_20_7: u32 = 1;
        // D s_20_8: call AArch64_AT(s_20_3, s_20_4, s_20_6, s_20_7)
        let s_20_8: () = AArch64_AT(state, tracer, s_20_3, s_20_4, s_20_6, s_20_7);
        // N s_20_9: return
        return;
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #1u : u8
        let s_21_0: bool = true;
        // D s_21_1: write-var gs#101201 <= s_21_0
        fn_state.gs_101201 = s_21_0;
        // N s_21_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #() : ()
        let s_22_0: () = ();
        // S s_22_1: call EL2Enabled(s_22_0)
        let s_22_1: bool = EL2Enabled(state, tracer, s_22_0);
        // N s_22_2: branch s_22_1 b27 b23
        if s_22_1 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #0u : u8
        let s_23_0: bool = false;
        // D s_23_1: write-var gs#101204 <= s_23_0
        fn_state.gs_101204 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#101204:u8
        let s_24_0: bool = fn_state.gs_101204;
        // N s_24_1: branch s_24_0 b26 b25
        if s_24_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_25_0: panic
        panic!("{:?}", ());
        // N s_25_1: return
        return;
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #24u : u8
        let s_26_0: u8 = 24;
        // C s_26_1: cast zx s_26_0 -> bv
        let s_26_1: Bits = Bits::new(s_26_0 as u128, 8u16);
        // C s_26_2: cast zx s_26_1 -> i
        let s_26_2: i128 = (s_26_1.value() as i128);
        // C s_26_3: cast reint s_26_2 -> i64
        let s_26_3: i64 = (s_26_2 as i64);
        // C s_26_4: cast zx s_26_3 -> i
        let s_26_4: i128 = (i128::try_from(s_26_3).unwrap());
        // C s_26_5: const #432u : u32
        let s_26_5: u32 = 432;
        // D s_26_6: read-reg s_26_5:u8
        let s_26_6: u8 = {
            let value = state.read_register::<u8>(s_26_5 as isize);
            tracer.read_register(s_26_5 as isize, value);
            value
        };
        // D s_26_7: call AArch64_SystemAccessTrap(s_26_6, s_26_4)
        let s_26_7: () = AArch64_SystemAccessTrap(state, tracer, s_26_6, s_26_4);
        // N s_26_8: return
        return;
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var __HCR_EL2_NV:u8
        let s_27_0: bool = fn_state.u__HCR_EL2_NV;
        // D s_27_1: cast zx s_27_0 -> bv
        let s_27_1: Bits = Bits::new(s_27_0 as u128, 1u16);
        // C s_27_2: const #1u : u8
        let s_27_2: bool = true;
        // C s_27_3: cast zx s_27_2 -> bv
        let s_27_3: Bits = Bits::new(s_27_2 as u128, 1u16);
        // D s_27_4: cmp-eq s_27_1 s_27_3
        let s_27_4: bool = ((s_27_1) == (s_27_3));
        // D s_27_5: write-var gs#101204 <= s_27_4
        fn_state.gs_101204 = s_27_4;
        // N s_27_6: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_28_0: panic
        panic!("{:?}", ());
        // N s_28_1: return
        return;
    }
}
