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
use Mk_CNTV_TVAL_EL0_Type::*;
use X_set::*;
use u_get_HCR_EL2_Type_E2H::*;
use u__UNKNOWN_bits::*;
use u_get_HCR_EL2_Type_NV::*;
use ELUsingAArch32::*;
use u__get_CNTV_TVAL_EL0::*;
use u_get_CNTV_CTL_EL0_Type_ENABLE::*;
use AArch64_SystemAccessTrap::*;
use EL2Enabled::*;
use PhysicalCountInt::*;
use common::*;
pub fn CNTV_TVAL_EL0_SysRegRead_f682815b8c52312a<T: Tracer>(
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
        ga_57037: ProductType5c790c8ef59cc8b2,
        u__HCR_EL2_E2H: bool,
        gs_58162: bool,
        gs_58169: bool,
        u__PSTATE_EL: u8,
        u__CNTV_CTL_EL0_ENABLE: bool,
        u__HCR_EL2_NV: bool,
        ga_57055: ProductType5c790c8ef59cc8b2,
        gs_58161: bool,
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
        // C s_0_7: const #102552u : u32
        let s_0_7: u32 = 102552;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_HCR_EL2_Type_E2H(s_0_8)
        let s_0_9: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_0_8);
        // D s_0_10: write-var __HCR_EL2_E2H <= s_0_9
        fn_state.u__HCR_EL2_E2H = s_0_9;
        // C s_0_11: const #17200u : u32
        let s_0_11: u32 = 17200;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_CNTV_CTL_EL0_Type_ENABLE(s_0_12)
        let s_0_13: bool = u_get_CNTV_CTL_EL0_Type_ENABLE(state, tracer, s_0_12);
        // D s_0_14: write-var __CNTV_CTL_EL0_ENABLE <= s_0_13
        fn_state.u__CNTV_CTL_EL0_ENABLE = s_0_13;
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
        // N s_0_21: branch s_0_20 b27 b1
        if s_0_20 {
            return block_27(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b21 b2
        if s_1_5 {
            return block_21(state, tracer, fn_state);
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
        // N s_5_2: branch s_5_1 b15 b6
        if s_5_1 {
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
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#58161 <= s_6_0
        fn_state.gs_58161 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#58161:u8
        let s_7_0: bool = fn_state.gs_58161;
        // N s_7_1: branch s_7_0 b14 b8
        if s_7_0 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var gs#58162 <= s_8_0
        fn_state.gs_58162 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#58162:u8
        let s_9_0: bool = fn_state.gs_58162;
        // N s_9_1: branch s_9_0 b11 b10
        if s_9_0 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: panic
        panic!("{:?}", ());
        // N s_10_1: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var __CNTV_CTL_EL0_ENABLE:u8
        let s_11_0: bool = fn_state.u__CNTV_CTL_EL0_ENABLE;
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
        // C s_12_1: const #23632u : u32
        let s_12_1: u32 = 23632;
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
        // S s_12_5: cast zx s_12_4 -> bv
        let s_12_5: Bits = Bits::new(s_12_4 as u128, 64u16);
        // C s_12_6: const #22400u : u32
        let s_12_6: u32 = 22400;
        // D s_12_7: read-reg s_12_6:u64
        let s_12_7: u64 = {
            let value = state.read_register::<u64>(s_12_6 as isize);
            tracer.read_register(s_12_6 as isize, value);
            value
        };
        // D s_12_8: cast zx s_12_7 -> bv
        let s_12_8: Bits = Bits::new(s_12_7 as u128, 64u16);
        // D s_12_9: sub s_12_5 s_12_8
        let s_12_9: Bits = ((s_12_5) - (s_12_8));
        // D s_12_10: cast reint s_12_9 -> u64
        let s_12_10: u64 = (s_12_9.value() as u64);
        // D s_12_11: cast zx s_12_2 -> bv
        let s_12_11: Bits = Bits::new(s_12_2 as u128, 64u16);
        // D s_12_12: cast zx s_12_10 -> bv
        let s_12_12: Bits = Bits::new(s_12_10 as u128, 64u16);
        // D s_12_13: sub s_12_11 s_12_12
        let s_12_13: Bits = ((s_12_11) - (s_12_12));
        // D s_12_14: cast reint s_12_13 -> u64
        let s_12_14: u64 = (s_12_13.value() as u64);
        // D s_12_15: call Mk_CNTV_TVAL_EL0_Type(s_12_14)
        let s_12_15: ProductType5c790c8ef59cc8b2 = Mk_CNTV_TVAL_EL0_Type(
            state,
            tracer,
            s_12_14,
        );
        // D s_12_16: call __get_CNTV_TVAL_EL0(s_12_15)
        let s_12_16: ProductType5c790c8ef59cc8b2 = u__get_CNTV_TVAL_EL0(
            state,
            tracer,
            s_12_15,
        );
        // D s_12_17: write-var ga#57055 <= s_12_16
        fn_state.ga_57055 = s_12_16;
        // D s_12_18: read-var ga#57055.0:struct
        let s_12_18: u64 = fn_state.ga_57055._0;
        // D s_12_19: cast zx s_12_18 -> bv
        let s_12_19: Bits = Bits::new(s_12_18 as u128, 64u16);
        // D s_12_20: read-var t:i
        let s_12_20: i128 = fn_state.t;
        // D s_12_21: call X_set(s_12_20, s_12_0, s_12_19)
        let s_12_21: () = X_set(state, tracer, s_12_20, s_12_0, s_12_19);
        // N s_12_22: return
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
        // D s_14_0: read-var __HCR_EL2_E2H:u8
        let s_14_0: bool = fn_state.u__HCR_EL2_E2H;
        // D s_14_1: cast zx s_14_0 -> bv
        let s_14_1: Bits = Bits::new(s_14_0 as u128, 1u16);
        // C s_14_2: const #1u : u8
        let s_14_2: bool = true;
        // C s_14_3: cast zx s_14_2 -> bv
        let s_14_3: Bits = Bits::new(s_14_2 as u128, 1u16);
        // D s_14_4: cmp-eq s_14_1 s_14_3
        let s_14_4: bool = ((s_14_1) == (s_14_3));
        // D s_14_5: write-var gs#58162 <= s_14_4
        fn_state.gs_58162 = s_14_4;
        // N s_14_6: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #432u : u32
        let s_15_0: u32 = 432;
        // D s_15_1: read-reg s_15_0:u8
        let s_15_1: u8 = {
            let value = state.read_register::<u8>(s_15_0 as isize);
            tracer.read_register(s_15_0 as isize, value);
            value
        };
        // D s_15_2: call ELUsingAArch32(s_15_1)
        let s_15_2: bool = ELUsingAArch32(state, tracer, s_15_1);
        // D s_15_3: not s_15_2
        let s_15_3: bool = !s_15_2;
        // D s_15_4: write-var gs#58161 <= s_15_3
        fn_state.gs_58161 = s_15_3;
        // N s_15_5: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var __HCR_EL2_E2H:u8
        let s_16_0: bool = fn_state.u__HCR_EL2_E2H;
        // D s_16_1: cast zx s_16_0 -> bv
        let s_16_1: Bits = Bits::new(s_16_0 as u128, 1u16);
        // C s_16_2: const #1u : u8
        let s_16_2: bool = true;
        // C s_16_3: cast zx s_16_2 -> bv
        let s_16_3: Bits = Bits::new(s_16_2 as u128, 1u16);
        // D s_16_4: cmp-eq s_16_1 s_16_3
        let s_16_4: bool = ((s_16_1) == (s_16_3));
        // N s_16_5: branch s_16_4 b18 b17
        if s_16_4 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
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
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var __CNTV_CTL_EL0_ENABLE:u8
        let s_18_0: bool = fn_state.u__CNTV_CTL_EL0_ENABLE;
        // D s_18_1: cast zx s_18_0 -> bv
        let s_18_1: Bits = Bits::new(s_18_0 as u128, 1u16);
        // C s_18_2: const #0u : u8
        let s_18_2: bool = false;
        // C s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 1u16);
        // D s_18_4: cmp-eq s_18_1 s_18_3
        let s_18_4: bool = ((s_18_1) == (s_18_3));
        // N s_18_5: branch s_18_4 b20 b19
        if s_18_4 {
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
        // C s_19_1: const #23632u : u32
        let s_19_1: u32 = 23632;
        // D s_19_2: read-reg s_19_1:u64
        let s_19_2: u64 = {
            let value = state.read_register::<u64>(s_19_1 as isize);
            tracer.read_register(s_19_1 as isize, value);
            value
        };
        // C s_19_3: const #() : ()
        let s_19_3: () = ();
        // S s_19_4: call PhysicalCountInt(s_19_3)
        let s_19_4: u64 = PhysicalCountInt(state, tracer, s_19_3);
        // S s_19_5: cast zx s_19_4 -> bv
        let s_19_5: Bits = Bits::new(s_19_4 as u128, 64u16);
        // C s_19_6: const #22400u : u32
        let s_19_6: u32 = 22400;
        // D s_19_7: read-reg s_19_6:u64
        let s_19_7: u64 = {
            let value = state.read_register::<u64>(s_19_6 as isize);
            tracer.read_register(s_19_6 as isize, value);
            value
        };
        // D s_19_8: cast zx s_19_7 -> bv
        let s_19_8: Bits = Bits::new(s_19_7 as u128, 64u16);
        // D s_19_9: sub s_19_5 s_19_8
        let s_19_9: Bits = ((s_19_5) - (s_19_8));
        // D s_19_10: cast reint s_19_9 -> u64
        let s_19_10: u64 = (s_19_9.value() as u64);
        // D s_19_11: cast zx s_19_2 -> bv
        let s_19_11: Bits = Bits::new(s_19_2 as u128, 64u16);
        // D s_19_12: cast zx s_19_10 -> bv
        let s_19_12: Bits = Bits::new(s_19_10 as u128, 64u16);
        // D s_19_13: sub s_19_11 s_19_12
        let s_19_13: Bits = ((s_19_11) - (s_19_12));
        // D s_19_14: cast reint s_19_13 -> u64
        let s_19_14: u64 = (s_19_13.value() as u64);
        // D s_19_15: call Mk_CNTV_TVAL_EL0_Type(s_19_14)
        let s_19_15: ProductType5c790c8ef59cc8b2 = Mk_CNTV_TVAL_EL0_Type(
            state,
            tracer,
            s_19_14,
        );
        // D s_19_16: call __get_CNTV_TVAL_EL0(s_19_15)
        let s_19_16: ProductType5c790c8ef59cc8b2 = u__get_CNTV_TVAL_EL0(
            state,
            tracer,
            s_19_15,
        );
        // D s_19_17: write-var ga#57037 <= s_19_16
        fn_state.ga_57037 = s_19_16;
        // D s_19_18: read-var ga#57037.0:struct
        let s_19_18: u64 = fn_state.ga_57037._0;
        // D s_19_19: cast zx s_19_18 -> bv
        let s_19_19: Bits = Bits::new(s_19_18 as u128, 64u16);
        // D s_19_20: read-var t:i
        let s_19_20: i128 = fn_state.t;
        // D s_19_21: call X_set(s_19_20, s_19_0, s_19_19)
        let s_19_21: () = X_set(state, tracer, s_19_20, s_19_0, s_19_19);
        // N s_19_22: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #64s : i64
        let s_20_0: i64 = 64;
        // C s_20_1: const #64s : i64
        let s_20_1: i64 = 64;
        // C s_20_2: cast zx s_20_1 -> i
        let s_20_2: i128 = (i128::try_from(s_20_1).unwrap());
        // S s_20_3: call __UNKNOWN_bits(s_20_2)
        let s_20_3: Bits = u__UNKNOWN_bits(state, tracer, s_20_2);
        // S s_20_4: cast reint s_20_3 -> u64
        let s_20_4: u64 = (s_20_3.value() as u64);
        // S s_20_5: cast zx s_20_4 -> bv
        let s_20_5: Bits = Bits::new(s_20_4 as u128, 64u16);
        // D s_20_6: read-var t:i
        let s_20_6: i128 = fn_state.t;
        // D s_20_7: call X_set(s_20_6, s_20_0, s_20_5)
        let s_20_7: () = X_set(state, tracer, s_20_6, s_20_0, s_20_5);
        // N s_20_8: return
        return;
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #() : ()
        let s_21_0: () = ();
        // S s_21_1: call EL2Enabled(s_21_0)
        let s_21_1: bool = EL2Enabled(state, tracer, s_21_0);
        // N s_21_2: branch s_21_1 b26 b22
        if s_21_1 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #0u : u8
        let s_22_0: bool = false;
        // D s_22_1: write-var gs#58169 <= s_22_0
        fn_state.gs_58169 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#58169:u8
        let s_23_0: bool = fn_state.gs_58169;
        // N s_23_1: branch s_23_0 b25 b24
        if s_23_0 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_24_0: panic
        panic!("{:?}", ());
        // N s_24_1: return
        return;
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #24u : u8
        let s_25_0: u8 = 24;
        // C s_25_1: cast zx s_25_0 -> bv
        let s_25_1: Bits = Bits::new(s_25_0 as u128, 8u16);
        // C s_25_2: cast zx s_25_1 -> i
        let s_25_2: i128 = (s_25_1.value() as i128);
        // C s_25_3: cast reint s_25_2 -> i64
        let s_25_3: i64 = (s_25_2 as i64);
        // C s_25_4: cast zx s_25_3 -> i
        let s_25_4: i128 = (i128::try_from(s_25_3).unwrap());
        // C s_25_5: const #432u : u32
        let s_25_5: u32 = 432;
        // D s_25_6: read-reg s_25_5:u8
        let s_25_6: u8 = {
            let value = state.read_register::<u8>(s_25_5 as isize);
            tracer.read_register(s_25_5 as isize, value);
            value
        };
        // D s_25_7: call AArch64_SystemAccessTrap(s_25_6, s_25_4)
        let s_25_7: () = AArch64_SystemAccessTrap(state, tracer, s_25_6, s_25_4);
        // N s_25_8: return
        return;
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var __HCR_EL2_NV:u8
        let s_26_0: bool = fn_state.u__HCR_EL2_NV;
        // D s_26_1: cast zx s_26_0 -> bv
        let s_26_1: Bits = Bits::new(s_26_0 as u128, 1u16);
        // C s_26_2: const #1u : u8
        let s_26_2: bool = true;
        // C s_26_3: cast zx s_26_2 -> bv
        let s_26_3: Bits = Bits::new(s_26_2 as u128, 1u16);
        // D s_26_4: cmp-eq s_26_1 s_26_3
        let s_26_4: bool = ((s_26_1) == (s_26_3));
        // D s_26_5: write-var gs#58169 <= s_26_4
        fn_state.gs_58169 = s_26_4;
        // N s_26_6: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_27_0: panic
        panic!("{:?}", ());
        // N s_27_1: return
        return;
    }
}
