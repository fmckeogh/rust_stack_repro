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
use X_set::*;
use Mk_CNTHPS_TVAL_EL2_Type::*;
use u__UNKNOWN_bits::*;
use EL2Enabled::*;
use u_get_HCR_EL2_Type_NV::*;
use AArch64_SystemAccessTrap::*;
use u_get_SCR_EL3_Type_EEL2::*;
use PhysicalCountInt::*;
use IsCurrentSecurityState::*;
use u_get_CNTHPS_CTL_EL2_Type_ENABLE::*;
use u__get_CNTHPS_TVAL_EL2::*;
use common::*;
pub fn CNTHPS_TVAL_EL2_SysRegRead_893c5b3a9908faeb<T: Tracer>(
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
        u__CNTHPS_CTL_EL2_ENABLE: bool,
        ga_55644: ProductType5c790c8ef59cc8b2,
        gs_57792: bool,
        ga_55658: ProductType5c790c8ef59cc8b2,
        u__PSTATE_EL: u8,
        u__HCR_EL2_NV: bool,
        u__SCR_EL3_EEL2: bool,
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
        // C s_0_7: const #10504u : u32
        let s_0_7: u32 = 10504;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_CNTHPS_CTL_EL2_Type_ENABLE(s_0_8)
        let s_0_9: bool = u_get_CNTHPS_CTL_EL2_Type_ENABLE(state, tracer, s_0_8);
        // D s_0_10: write-var __CNTHPS_CTL_EL2_ENABLE <= s_0_9
        fn_state.u__CNTHPS_CTL_EL2_ENABLE = s_0_9;
        // C s_0_11: const #90704u : u32
        let s_0_11: u32 = 90704;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_SCR_EL3_Type_EEL2(s_0_12)
        let s_0_13: bool = u_get_SCR_EL3_Type_EEL2(state, tracer, s_0_12);
        // D s_0_14: write-var __SCR_EL3_EEL2 <= s_0_13
        fn_state.u__SCR_EL3_EEL2 = s_0_13;
        // D s_0_15: read-var __PSTATE_EL:u8
        let s_0_15: u8 = fn_state.u__PSTATE_EL;
        // D s_0_16: cast zx s_0_15 -> bv
        let s_0_16: Bits = Bits::new(s_0_15 as u128, 2u16);
        // C s_0_17: const #448u : u32
        let s_0_17: u32 = 448;
        // D s_0_18: read-reg s_0_17:u8
        let s_0_18: u8 = {
            let value = state.read_register::<u8>(s_0_17 as isize);
            tracer.read_register(s_0_17 as isize, value);
            value
        };
        // D s_0_19: cast zx s_0_18 -> bv
        let s_0_19: Bits = Bits::new(s_0_18 as u128, 2u16);
        // D s_0_20: cmp-eq s_0_16 s_0_19
        let s_0_20: bool = ((s_0_16) == (s_0_19));
        // N s_0_21: branch s_0_20 b23 b1
        if s_0_20 {
            return block_23(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b15 b2
        if s_1_5 {
            return block_15(state, tracer, fn_state);
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
        // D s_5_0: read-var __SCR_EL3_EEL2:u8
        let s_5_0: bool = fn_state.u__SCR_EL3_EEL2;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 1u16);
        // C s_5_2: const #0u : u8
        let s_5_2: bool = false;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 1u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // N s_5_5: branch s_5_4 b9 b6
        if s_5_4 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var __CNTHPS_CTL_EL2_ENABLE:u8
        let s_6_0: bool = fn_state.u__CNTHPS_CTL_EL2_ENABLE;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 1u16);
        // C s_6_2: const #0u : u8
        let s_6_2: bool = false;
        // C s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 1u16);
        // D s_6_4: cmp-eq s_6_1 s_6_3
        let s_6_4: bool = ((s_6_1) == (s_6_3));
        // N s_6_5: branch s_6_4 b8 b7
        if s_6_4 {
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
        // C s_7_0: const #64s : i64
        let s_7_0: i64 = 64;
        // C s_7_1: const #22672u : u32
        let s_7_1: u32 = 22672;
        // D s_7_2: read-reg s_7_1:u64
        let s_7_2: u64 = {
            let value = state.read_register::<u64>(s_7_1 as isize);
            tracer.read_register(s_7_1 as isize, value);
            value
        };
        // C s_7_3: const #() : ()
        let s_7_3: () = ();
        // S s_7_4: call PhysicalCountInt(s_7_3)
        let s_7_4: u64 = PhysicalCountInt(state, tracer, s_7_3);
        // D s_7_5: cast zx s_7_2 -> bv
        let s_7_5: Bits = Bits::new(s_7_2 as u128, 64u16);
        // S s_7_6: cast zx s_7_4 -> bv
        let s_7_6: Bits = Bits::new(s_7_4 as u128, 64u16);
        // D s_7_7: sub s_7_5 s_7_6
        let s_7_7: Bits = ((s_7_5) - (s_7_6));
        // D s_7_8: cast reint s_7_7 -> u64
        let s_7_8: u64 = (s_7_7.value() as u64);
        // D s_7_9: call Mk_CNTHPS_TVAL_EL2_Type(s_7_8)
        let s_7_9: ProductType5c790c8ef59cc8b2 = Mk_CNTHPS_TVAL_EL2_Type(
            state,
            tracer,
            s_7_8,
        );
        // D s_7_10: call __get_CNTHPS_TVAL_EL2(s_7_9)
        let s_7_10: ProductType5c790c8ef59cc8b2 = u__get_CNTHPS_TVAL_EL2(
            state,
            tracer,
            s_7_9,
        );
        // D s_7_11: write-var ga#55658 <= s_7_10
        fn_state.ga_55658 = s_7_10;
        // D s_7_12: read-var ga#55658.0:struct
        let s_7_12: u64 = fn_state.ga_55658._0;
        // D s_7_13: cast zx s_7_12 -> bv
        let s_7_13: Bits = Bits::new(s_7_12 as u128, 64u16);
        // D s_7_14: read-var t:i
        let s_7_14: i128 = fn_state.t;
        // D s_7_15: call X_set(s_7_14, s_7_0, s_7_13)
        let s_7_15: () = X_set(state, tracer, s_7_14, s_7_0, s_7_13);
        // N s_7_16: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #64s : i64
        let s_8_0: i64 = 64;
        // C s_8_1: const #64s : i64
        let s_8_1: i64 = 64;
        // C s_8_2: cast zx s_8_1 -> i
        let s_8_2: i128 = (i128::try_from(s_8_1).unwrap());
        // S s_8_3: call __UNKNOWN_bits(s_8_2)
        let s_8_3: Bits = u__UNKNOWN_bits(state, tracer, s_8_2);
        // S s_8_4: cast reint s_8_3 -> u64
        let s_8_4: u64 = (s_8_3.value() as u64);
        // S s_8_5: cast zx s_8_4 -> bv
        let s_8_5: Bits = Bits::new(s_8_4 as u128, 64u16);
        // D s_8_6: read-var t:i
        let s_8_6: i128 = fn_state.t;
        // D s_8_7: call X_set(s_8_6, s_8_0, s_8_5)
        let s_8_7: () = X_set(state, tracer, s_8_6, s_8_0, s_8_5);
        // N s_8_8: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: panic
        panic!("{:?}", ());
        // N s_9_1: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #3u : u32
        let s_10_0: u32 = 3;
        // S s_10_1: call IsCurrentSecurityState(s_10_0)
        let s_10_1: bool = IsCurrentSecurityState(state, tracer, s_10_0);
        // S s_10_2: not s_10_1
        let s_10_2: bool = !s_10_1;
        // N s_10_3: branch s_10_2 b14 b11
        if s_10_2 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var __CNTHPS_CTL_EL2_ENABLE:u8
        let s_11_0: bool = fn_state.u__CNTHPS_CTL_EL2_ENABLE;
        // D s_11_1: cast zx s_11_0 -> bv
        let s_11_1: Bits = Bits::new(s_11_0 as u128, 1u16);
        // C s_11_2: const #0u : u8
        let s_11_2: bool = false;
        // C s_11_3: cast zx s_11_2 -> bv
        let s_11_3: Bits = Bits::new(s_11_2 as u128, 1u16);
        // D s_11_4: cmp-eq s_11_1 s_11_3
        let s_11_4: bool = ((s_11_1) == (s_11_3));
        // N s_11_5: branch s_11_4 b13 b12
        if s_11_4 {
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
        // C s_12_0: const #64s : i64
        let s_12_0: i64 = 64;
        // C s_12_1: const #22672u : u32
        let s_12_1: u32 = 22672;
        // D s_12_2: read-reg s_12_1:u64
        let s_12_2: u64 = {
            let value = state.read_register::<u64>(s_12_1 as isize);
            tracer.read_register(s_12_1 as isize, value);
            value
        };
        // C s_12_3: const #() : ()
        let s_12_3: () = ();
        // S s_12_4: call PhysicalCountInt(s_12_3)
        let s_12_4: u64 = PhysicalCountInt(state, tracer, s_12_3);
        // D s_12_5: cast zx s_12_2 -> bv
        let s_12_5: Bits = Bits::new(s_12_2 as u128, 64u16);
        // S s_12_6: cast zx s_12_4 -> bv
        let s_12_6: Bits = Bits::new(s_12_4 as u128, 64u16);
        // D s_12_7: sub s_12_5 s_12_6
        let s_12_7: Bits = ((s_12_5) - (s_12_6));
        // D s_12_8: cast reint s_12_7 -> u64
        let s_12_8: u64 = (s_12_7.value() as u64);
        // D s_12_9: call Mk_CNTHPS_TVAL_EL2_Type(s_12_8)
        let s_12_9: ProductType5c790c8ef59cc8b2 = Mk_CNTHPS_TVAL_EL2_Type(
            state,
            tracer,
            s_12_8,
        );
        // D s_12_10: call __get_CNTHPS_TVAL_EL2(s_12_9)
        let s_12_10: ProductType5c790c8ef59cc8b2 = u__get_CNTHPS_TVAL_EL2(
            state,
            tracer,
            s_12_9,
        );
        // D s_12_11: write-var ga#55644 <= s_12_10
        fn_state.ga_55644 = s_12_10;
        // D s_12_12: read-var ga#55644.0:struct
        let s_12_12: u64 = fn_state.ga_55644._0;
        // D s_12_13: cast zx s_12_12 -> bv
        let s_12_13: Bits = Bits::new(s_12_12 as u128, 64u16);
        // D s_12_14: read-var t:i
        let s_12_14: i128 = fn_state.t;
        // D s_12_15: call X_set(s_12_14, s_12_0, s_12_13)
        let s_12_15: () = X_set(state, tracer, s_12_14, s_12_0, s_12_13);
        // N s_12_16: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #64s : i64
        let s_13_0: i64 = 64;
        // C s_13_1: const #64s : i64
        let s_13_1: i64 = 64;
        // C s_13_2: cast zx s_13_1 -> i
        let s_13_2: i128 = (i128::try_from(s_13_1).unwrap());
        // S s_13_3: call __UNKNOWN_bits(s_13_2)
        let s_13_3: Bits = u__UNKNOWN_bits(state, tracer, s_13_2);
        // S s_13_4: cast reint s_13_3 -> u64
        let s_13_4: u64 = (s_13_3.value() as u64);
        // S s_13_5: cast zx s_13_4 -> bv
        let s_13_5: Bits = Bits::new(s_13_4 as u128, 64u16);
        // D s_13_6: read-var t:i
        let s_13_6: i128 = fn_state.t;
        // D s_13_7: call X_set(s_13_6, s_13_0, s_13_5)
        let s_13_7: () = X_set(state, tracer, s_13_6, s_13_0, s_13_5);
        // N s_13_8: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_14_0: panic
        panic!("{:?}", ());
        // N s_14_1: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #3u : u32
        let s_15_0: u32 = 3;
        // S s_15_1: call IsCurrentSecurityState(s_15_0)
        let s_15_1: bool = IsCurrentSecurityState(state, tracer, s_15_0);
        // S s_15_2: not s_15_1
        let s_15_2: bool = !s_15_1;
        // N s_15_3: branch s_15_2 b22 b16
        if s_15_2 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #() : ()
        let s_16_0: () = ();
        // S s_16_1: call EL2Enabled(s_16_0)
        let s_16_1: bool = EL2Enabled(state, tracer, s_16_0);
        // N s_16_2: branch s_16_1 b21 b17
        if s_16_1 {
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
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var gs#57792 <= s_17_0
        fn_state.gs_57792 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#57792:u8
        let s_18_0: bool = fn_state.gs_57792;
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
        // N s_19_0: panic
        panic!("{:?}", ());
        // N s_19_1: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #24u : u8
        let s_20_0: u8 = 24;
        // C s_20_1: cast zx s_20_0 -> bv
        let s_20_1: Bits = Bits::new(s_20_0 as u128, 8u16);
        // C s_20_2: cast zx s_20_1 -> i
        let s_20_2: i128 = (s_20_1.value() as i128);
        // C s_20_3: cast reint s_20_2 -> i64
        let s_20_3: i64 = (s_20_2 as i64);
        // C s_20_4: cast zx s_20_3 -> i
        let s_20_4: i128 = (i128::try_from(s_20_3).unwrap());
        // C s_20_5: const #432u : u32
        let s_20_5: u32 = 432;
        // D s_20_6: read-reg s_20_5:u8
        let s_20_6: u8 = {
            let value = state.read_register::<u8>(s_20_5 as isize);
            tracer.read_register(s_20_5 as isize, value);
            value
        };
        // D s_20_7: call AArch64_SystemAccessTrap(s_20_6, s_20_4)
        let s_20_7: () = AArch64_SystemAccessTrap(state, tracer, s_20_6, s_20_4);
        // N s_20_8: return
        return;
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var __HCR_EL2_NV:u8
        let s_21_0: bool = fn_state.u__HCR_EL2_NV;
        // D s_21_1: cast zx s_21_0 -> bv
        let s_21_1: Bits = Bits::new(s_21_0 as u128, 1u16);
        // C s_21_2: const #1u : u8
        let s_21_2: bool = true;
        // C s_21_3: cast zx s_21_2 -> bv
        let s_21_3: Bits = Bits::new(s_21_2 as u128, 1u16);
        // D s_21_4: cmp-eq s_21_1 s_21_3
        let s_21_4: bool = ((s_21_1) == (s_21_3));
        // D s_21_5: write-var gs#57792 <= s_21_4
        fn_state.gs_57792 = s_21_4;
        // N s_21_6: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_22_0: panic
        panic!("{:?}", ());
        // N s_22_1: return
        return;
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_23_0: panic
        panic!("{:?}", ());
        // N s_23_1: return
        return;
    }
}
