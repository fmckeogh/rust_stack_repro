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
use u__UNKNOWN_bits::*;
use u_get_HCR_EL2_Type_NV::*;
use u_get_CNTHP_CTL_EL2_Type_ENABLE::*;
use AArch64_SystemAccessTrap::*;
use PhysicalCountInt::*;
use Mk_CNTHP_TVAL_EL2_Type::*;
use EL2Enabled::*;
use u__get_CNTHP_TVAL_EL2::*;
use common::*;
pub fn CNTHP_TVAL_EL2_SysRegRead_ddcf2e9dd3ff2611<T: Tracer>(
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
        ga_55799: ProductType5c790c8ef59cc8b2,
        ga_55811: ProductType5c790c8ef59cc8b2,
        u__PSTATE_EL: u8,
        u__HCR_EL2_NV: bool,
        gs_57836: bool,
        u__CNTHP_CTL_EL2_ENABLE: bool,
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
        // C s_0_7: const #100912u : u32
        let s_0_7: u32 = 100912;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_CNTHP_CTL_EL2_Type_ENABLE(s_0_8)
        let s_0_9: bool = u_get_CNTHP_CTL_EL2_Type_ENABLE(state, tracer, s_0_8);
        // D s_0_10: write-var __CNTHP_CTL_EL2_ENABLE <= s_0_9
        fn_state.u__CNTHP_CTL_EL2_ENABLE = s_0_9;
        // D s_0_11: read-var __PSTATE_EL:u8
        let s_0_11: u8 = fn_state.u__PSTATE_EL;
        // D s_0_12: cast zx s_0_11 -> bv
        let s_0_12: Bits = Bits::new(s_0_11 as u128, 2u16);
        // C s_0_13: const #448u : u32
        let s_0_13: u32 = 448;
        // D s_0_14: read-reg s_0_13:u8
        let s_0_14: u8 = {
            let value = state.read_register::<u8>(s_0_13 as isize);
            tracer.read_register(s_0_13 as isize, value);
            value
        };
        // D s_0_15: cast zx s_0_14 -> bv
        let s_0_15: Bits = Bits::new(s_0_14 as u128, 2u16);
        // D s_0_16: cmp-eq s_0_12 s_0_15
        let s_0_16: bool = ((s_0_12) == (s_0_15));
        // N s_0_17: branch s_0_16 b17 b1
        if s_0_16 {
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
        // N s_1_6: branch s_1_5 b11 b2
        if s_1_5 {
            return block_11(state, tracer, fn_state);
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
        // D s_5_0: read-var __CNTHP_CTL_EL2_ENABLE:u8
        let s_5_0: bool = fn_state.u__CNTHP_CTL_EL2_ENABLE;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 1u16);
        // C s_5_2: const #0u : u8
        let s_5_2: bool = false;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 1u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // N s_5_5: branch s_5_4 b7 b6
        if s_5_4 {
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
        // C s_6_0: const #64s : i64
        let s_6_0: i64 = 64;
        // C s_6_1: const #16640u : u32
        let s_6_1: u32 = 16640;
        // D s_6_2: read-reg s_6_1:u64
        let s_6_2: u64 = {
            let value = state.read_register::<u64>(s_6_1 as isize);
            tracer.read_register(s_6_1 as isize, value);
            value
        };
        // C s_6_3: const #() : ()
        let s_6_3: () = ();
        // S s_6_4: call PhysicalCountInt(s_6_3)
        let s_6_4: u64 = PhysicalCountInt(state, tracer, s_6_3);
        // D s_6_5: cast zx s_6_2 -> bv
        let s_6_5: Bits = Bits::new(s_6_2 as u128, 64u16);
        // S s_6_6: cast zx s_6_4 -> bv
        let s_6_6: Bits = Bits::new(s_6_4 as u128, 64u16);
        // D s_6_7: sub s_6_5 s_6_6
        let s_6_7: Bits = ((s_6_5) - (s_6_6));
        // D s_6_8: cast reint s_6_7 -> u64
        let s_6_8: u64 = (s_6_7.value() as u64);
        // D s_6_9: call Mk_CNTHP_TVAL_EL2_Type(s_6_8)
        let s_6_9: ProductType5c790c8ef59cc8b2 = Mk_CNTHP_TVAL_EL2_Type(
            state,
            tracer,
            s_6_8,
        );
        // D s_6_10: call __get_CNTHP_TVAL_EL2(s_6_9)
        let s_6_10: ProductType5c790c8ef59cc8b2 = u__get_CNTHP_TVAL_EL2(
            state,
            tracer,
            s_6_9,
        );
        // D s_6_11: write-var ga#55811 <= s_6_10
        fn_state.ga_55811 = s_6_10;
        // D s_6_12: read-var ga#55811.0:struct
        let s_6_12: u64 = fn_state.ga_55811._0;
        // D s_6_13: cast zx s_6_12 -> bv
        let s_6_13: Bits = Bits::new(s_6_12 as u128, 64u16);
        // D s_6_14: read-var t:i
        let s_6_14: i128 = fn_state.t;
        // D s_6_15: call X_set(s_6_14, s_6_0, s_6_13)
        let s_6_15: () = X_set(state, tracer, s_6_14, s_6_0, s_6_13);
        // N s_6_16: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #64s : i64
        let s_7_0: i64 = 64;
        // C s_7_1: const #64s : i64
        let s_7_1: i64 = 64;
        // C s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (i128::try_from(s_7_1).unwrap());
        // S s_7_3: call __UNKNOWN_bits(s_7_2)
        let s_7_3: Bits = u__UNKNOWN_bits(state, tracer, s_7_2);
        // S s_7_4: cast reint s_7_3 -> u64
        let s_7_4: u64 = (s_7_3.value() as u64);
        // S s_7_5: cast zx s_7_4 -> bv
        let s_7_5: Bits = Bits::new(s_7_4 as u128, 64u16);
        // D s_7_6: read-var t:i
        let s_7_6: i128 = fn_state.t;
        // D s_7_7: call X_set(s_7_6, s_7_0, s_7_5)
        let s_7_7: () = X_set(state, tracer, s_7_6, s_7_0, s_7_5);
        // N s_7_8: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var __CNTHP_CTL_EL2_ENABLE:u8
        let s_8_0: bool = fn_state.u__CNTHP_CTL_EL2_ENABLE;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 1u16);
        // C s_8_2: const #0u : u8
        let s_8_2: bool = false;
        // C s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 1u16);
        // D s_8_4: cmp-eq s_8_1 s_8_3
        let s_8_4: bool = ((s_8_1) == (s_8_3));
        // N s_8_5: branch s_8_4 b10 b9
        if s_8_4 {
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
        // C s_9_1: const #16640u : u32
        let s_9_1: u32 = 16640;
        // D s_9_2: read-reg s_9_1:u64
        let s_9_2: u64 = {
            let value = state.read_register::<u64>(s_9_1 as isize);
            tracer.read_register(s_9_1 as isize, value);
            value
        };
        // C s_9_3: const #() : ()
        let s_9_3: () = ();
        // S s_9_4: call PhysicalCountInt(s_9_3)
        let s_9_4: u64 = PhysicalCountInt(state, tracer, s_9_3);
        // D s_9_5: cast zx s_9_2 -> bv
        let s_9_5: Bits = Bits::new(s_9_2 as u128, 64u16);
        // S s_9_6: cast zx s_9_4 -> bv
        let s_9_6: Bits = Bits::new(s_9_4 as u128, 64u16);
        // D s_9_7: sub s_9_5 s_9_6
        let s_9_7: Bits = ((s_9_5) - (s_9_6));
        // D s_9_8: cast reint s_9_7 -> u64
        let s_9_8: u64 = (s_9_7.value() as u64);
        // D s_9_9: call Mk_CNTHP_TVAL_EL2_Type(s_9_8)
        let s_9_9: ProductType5c790c8ef59cc8b2 = Mk_CNTHP_TVAL_EL2_Type(
            state,
            tracer,
            s_9_8,
        );
        // D s_9_10: call __get_CNTHP_TVAL_EL2(s_9_9)
        let s_9_10: ProductType5c790c8ef59cc8b2 = u__get_CNTHP_TVAL_EL2(
            state,
            tracer,
            s_9_9,
        );
        // D s_9_11: write-var ga#55799 <= s_9_10
        fn_state.ga_55799 = s_9_10;
        // D s_9_12: read-var ga#55799.0:struct
        let s_9_12: u64 = fn_state.ga_55799._0;
        // D s_9_13: cast zx s_9_12 -> bv
        let s_9_13: Bits = Bits::new(s_9_12 as u128, 64u16);
        // D s_9_14: read-var t:i
        let s_9_14: i128 = fn_state.t;
        // D s_9_15: call X_set(s_9_14, s_9_0, s_9_13)
        let s_9_15: () = X_set(state, tracer, s_9_14, s_9_0, s_9_13);
        // N s_9_16: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #64s : i64
        let s_10_0: i64 = 64;
        // C s_10_1: const #64s : i64
        let s_10_1: i64 = 64;
        // C s_10_2: cast zx s_10_1 -> i
        let s_10_2: i128 = (i128::try_from(s_10_1).unwrap());
        // S s_10_3: call __UNKNOWN_bits(s_10_2)
        let s_10_3: Bits = u__UNKNOWN_bits(state, tracer, s_10_2);
        // S s_10_4: cast reint s_10_3 -> u64
        let s_10_4: u64 = (s_10_3.value() as u64);
        // S s_10_5: cast zx s_10_4 -> bv
        let s_10_5: Bits = Bits::new(s_10_4 as u128, 64u16);
        // D s_10_6: read-var t:i
        let s_10_6: i128 = fn_state.t;
        // D s_10_7: call X_set(s_10_6, s_10_0, s_10_5)
        let s_10_7: () = X_set(state, tracer, s_10_6, s_10_0, s_10_5);
        // N s_10_8: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call EL2Enabled(s_11_0)
        let s_11_1: bool = EL2Enabled(state, tracer, s_11_0);
        // N s_11_2: branch s_11_1 b16 b12
        if s_11_1 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var gs#57836 <= s_12_0
        fn_state.gs_57836 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#57836:u8
        let s_13_0: bool = fn_state.gs_57836;
        // N s_13_1: branch s_13_0 b15 b14
        if s_13_0 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
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
        // C s_15_0: const #24u : u8
        let s_15_0: u8 = 24;
        // C s_15_1: cast zx s_15_0 -> bv
        let s_15_1: Bits = Bits::new(s_15_0 as u128, 8u16);
        // C s_15_2: cast zx s_15_1 -> i
        let s_15_2: i128 = (s_15_1.value() as i128);
        // C s_15_3: cast reint s_15_2 -> i64
        let s_15_3: i64 = (s_15_2 as i64);
        // C s_15_4: cast zx s_15_3 -> i
        let s_15_4: i128 = (i128::try_from(s_15_3).unwrap());
        // C s_15_5: const #432u : u32
        let s_15_5: u32 = 432;
        // D s_15_6: read-reg s_15_5:u8
        let s_15_6: u8 = {
            let value = state.read_register::<u8>(s_15_5 as isize);
            tracer.read_register(s_15_5 as isize, value);
            value
        };
        // D s_15_7: call AArch64_SystemAccessTrap(s_15_6, s_15_4)
        let s_15_7: () = AArch64_SystemAccessTrap(state, tracer, s_15_6, s_15_4);
        // N s_15_8: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var __HCR_EL2_NV:u8
        let s_16_0: bool = fn_state.u__HCR_EL2_NV;
        // D s_16_1: cast zx s_16_0 -> bv
        let s_16_1: Bits = Bits::new(s_16_0 as u128, 1u16);
        // C s_16_2: const #1u : u8
        let s_16_2: bool = true;
        // C s_16_3: cast zx s_16_2 -> bv
        let s_16_3: Bits = Bits::new(s_16_2 as u128, 1u16);
        // D s_16_4: cmp-eq s_16_1 s_16_3
        let s_16_4: bool = ((s_16_1) == (s_16_3));
        // D s_16_5: write-var gs#57836 <= s_16_4
        fn_state.gs_57836 = s_16_4;
        // N s_16_6: jump b13
        return block_13(state, tracer, fn_state);
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
