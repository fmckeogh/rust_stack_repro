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
use Mk_CNTPS_TVAL_EL1_Type::*;
use u__get_CNTPS_TVAL_EL1::*;
use X_set::*;
use u_get_SCR_EL3_Type_ECVEn::*;
use u__UNKNOWN_bits::*;
use EL2Enabled::*;
use AArch64_SystemAccessTrap::*;
use IsFeatureImplemented::*;
use u_get_SCR_EL3_Type_ST::*;
use u_get_SCR_EL3_Type_EEL2::*;
use PhysicalCountInt::*;
use u_get_SCR_EL3_Type_NS::*;
use u_get_CNTPS_CTL_EL1_Type_ENABLE::*;
use u_get_CNTHCTL_EL2_Type_ECV::*;
use common::*;
pub fn CNTPS_TVAL_EL1_SysRegRead_748a1753bd033377<T: Tracer>(
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
        gs_58022: bool,
        u__SCR_EL3_NS: bool,
        gs_58023: bool,
        ga_56530: ProductType5c790c8ef59cc8b2,
        ga_56519: ProductType5c790c8ef59cc8b2,
        ga_56545: ProductType5c790c8ef59cc8b2,
        u__CNTPS_CTL_EL1_ENABLE: bool,
        gs_58024: bool,
        u__SCR_EL3_ECVEn: bool,
        u__CNTHCTL_EL2_ECV: bool,
        u__PSTATE_EL: u8,
        u__SCR_EL3_ST: bool,
        gs_58025: bool,
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
        // C s_0_3: const #90704u : u32
        let s_0_3: u32 = 90704;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_SCR_EL3_Type_NS(s_0_4)
        let s_0_5: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_0_4);
        // D s_0_6: write-var __SCR_EL3_NS <= s_0_5
        fn_state.u__SCR_EL3_NS = s_0_5;
        // C s_0_7: const #90704u : u32
        let s_0_7: u32 = 90704;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_SCR_EL3_Type_EEL2(s_0_8)
        let s_0_9: bool = u_get_SCR_EL3_Type_EEL2(state, tracer, s_0_8);
        // D s_0_10: write-var __SCR_EL3_EEL2 <= s_0_9
        fn_state.u__SCR_EL3_EEL2 = s_0_9;
        // C s_0_11: const #90704u : u32
        let s_0_11: u32 = 90704;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_SCR_EL3_Type_ST(s_0_12)
        let s_0_13: bool = u_get_SCR_EL3_Type_ST(state, tracer, s_0_12);
        // D s_0_14: write-var __SCR_EL3_ST <= s_0_13
        fn_state.u__SCR_EL3_ST = s_0_13;
        // C s_0_15: const #90704u : u32
        let s_0_15: u32 = 90704;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_SCR_EL3_Type_ECVEn(s_0_16)
        let s_0_17: bool = u_get_SCR_EL3_Type_ECVEn(state, tracer, s_0_16);
        // D s_0_18: write-var __SCR_EL3_ECVEn <= s_0_17
        fn_state.u__SCR_EL3_ECVEn = s_0_17;
        // C s_0_19: const #12808u : u32
        let s_0_19: u32 = 12808;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_CNTHCTL_EL2_Type_ECV(s_0_20)
        let s_0_21: bool = u_get_CNTHCTL_EL2_Type_ECV(state, tracer, s_0_20);
        // D s_0_22: write-var __CNTHCTL_EL2_ECV <= s_0_21
        fn_state.u__CNTHCTL_EL2_ECV = s_0_21;
        // C s_0_23: const #20376u : u32
        let s_0_23: u32 = 20376;
        // D s_0_24: read-reg s_0_23:struct
        let s_0_24: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_23 as isize);
            tracer.read_register(s_0_23 as isize, value);
            value
        };
        // D s_0_25: call _get_CNTPS_CTL_EL1_Type_ENABLE(s_0_24)
        let s_0_25: bool = u_get_CNTPS_CTL_EL1_Type_ENABLE(state, tracer, s_0_24);
        // D s_0_26: write-var __CNTPS_CTL_EL1_ENABLE <= s_0_25
        fn_state.u__CNTPS_CTL_EL1_ENABLE = s_0_25;
        // D s_0_27: read-var __PSTATE_EL:u8
        let s_0_27: u8 = fn_state.u__PSTATE_EL;
        // D s_0_28: cast zx s_0_27 -> bv
        let s_0_28: Bits = Bits::new(s_0_27 as u128, 2u16);
        // C s_0_29: const #448u : u32
        let s_0_29: u32 = 448;
        // D s_0_30: read-reg s_0_29:u8
        let s_0_30: u8 = {
            let value = state.read_register::<u8>(s_0_29 as isize);
            tracer.read_register(s_0_29 as isize, value);
            value
        };
        // D s_0_31: cast zx s_0_30 -> bv
        let s_0_31: Bits = Bits::new(s_0_30 as u128, 2u16);
        // D s_0_32: cmp-eq s_0_28 s_0_31
        let s_0_32: bool = ((s_0_28) == (s_0_31));
        // N s_0_33: branch s_0_32 b34 b1
        if s_0_32 {
            return block_34(state, tracer, fn_state);
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
        // D s_5_0: read-var __CNTPS_CTL_EL1_ENABLE:u8
        let s_5_0: bool = fn_state.u__CNTPS_CTL_EL1_ENABLE;
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
        // C s_6_1: const #14160u : u32
        let s_6_1: u32 = 14160;
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
        // D s_6_9: call Mk_CNTPS_TVAL_EL1_Type(s_6_8)
        let s_6_9: ProductType5c790c8ef59cc8b2 = Mk_CNTPS_TVAL_EL1_Type(
            state,
            tracer,
            s_6_8,
        );
        // D s_6_10: call __get_CNTPS_TVAL_EL1(s_6_9)
        let s_6_10: ProductType5c790c8ef59cc8b2 = u__get_CNTPS_TVAL_EL1(
            state,
            tracer,
            s_6_9,
        );
        // D s_6_11: write-var ga#56545 <= s_6_10
        fn_state.ga_56545 = s_6_10;
        // D s_6_12: read-var ga#56545.0:struct
        let s_6_12: u64 = fn_state.ga_56545._0;
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
        // N s_8_0: panic
        panic!("{:?}", ());
        // N s_8_1: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #424u : u32
        let s_9_0: u32 = 424;
        // D s_9_1: read-reg s_9_0:u8
        let s_9_1: u8 = {
            let value = state.read_register::<u8>(s_9_0 as isize);
            tracer.read_register(s_9_0 as isize, value);
            value
        };
        // C s_9_2: const #2u : u8
        let s_9_2: u8 = 2;
        // D s_9_3: cmp-lt s_9_1 s_9_2
        let s_9_3: bool = ((s_9_1) < (s_9_2));
        // N s_9_4: branch s_9_3 b33 b10
        if s_9_3 {
            return block_33(state, tracer, fn_state);
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
        // D s_10_1: write-var gs#58022 <= s_10_0
        fn_state.gs_58022 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#58022:u8
        let s_11_0: bool = fn_state.gs_58022;
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
        // D s_13_0: read-var __SCR_EL3_EEL2:u8
        let s_13_0: bool = fn_state.u__SCR_EL3_EEL2;
        // D s_13_1: cast zx s_13_0 -> bv
        let s_13_1: Bits = Bits::new(s_13_0 as u128, 1u16);
        // C s_13_2: const #1u : u8
        let s_13_2: bool = true;
        // C s_13_3: cast zx s_13_2 -> bv
        let s_13_3: Bits = Bits::new(s_13_2 as u128, 1u16);
        // D s_13_4: cmp-eq s_13_1 s_13_3
        let s_13_4: bool = ((s_13_1) == (s_13_3));
        // N s_13_5: branch s_13_4 b32 b14
        if s_13_4 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var __SCR_EL3_ST:u8
        let s_14_0: bool = fn_state.u__SCR_EL3_ST;
        // D s_14_1: cast zx s_14_0 -> bv
        let s_14_1: Bits = Bits::new(s_14_0 as u128, 1u16);
        // C s_14_2: const #0u : u8
        let s_14_2: bool = false;
        // C s_14_3: cast zx s_14_2 -> bv
        let s_14_3: Bits = Bits::new(s_14_2 as u128, 1u16);
        // D s_14_4: cmp-eq s_14_1 s_14_3
        let s_14_4: bool = ((s_14_1) == (s_14_3));
        // N s_14_5: branch s_14_4 b31 b15
        if s_14_4 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #145u : u32
        let s_15_0: u32 = 145;
        // S s_15_1: call IsFeatureImplemented(s_15_0)
        let s_15_1: bool = IsFeatureImplemented(state, tracer, s_15_0);
        // N s_15_2: branch s_15_1 b30 b16
        if s_15_1 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #0u : u8
        let s_16_0: bool = false;
        // D s_16_1: write-var gs#58023 <= s_16_0
        fn_state.gs_58023 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#58023:u8
        let s_17_0: bool = fn_state.gs_58023;
        // N s_17_1: branch s_17_0 b29 b18
        if s_17_0 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #0u : u8
        let s_18_0: bool = false;
        // D s_18_1: write-var gs#58024 <= s_18_0
        fn_state.gs_58024 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#58024:u8
        let s_19_0: bool = fn_state.gs_58024;
        // N s_19_1: branch s_19_0 b28 b20
        if s_19_0 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #0u : u8
        let s_20_0: bool = false;
        // D s_20_1: write-var gs#58025 <= s_20_0
        fn_state.gs_58025 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#58025:u8
        let s_21_0: bool = fn_state.gs_58025;
        // N s_21_1: branch s_21_0 b25 b22
        if s_21_0 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var __CNTPS_CTL_EL1_ENABLE:u8
        let s_22_0: bool = fn_state.u__CNTPS_CTL_EL1_ENABLE;
        // D s_22_1: cast zx s_22_0 -> bv
        let s_22_1: Bits = Bits::new(s_22_0 as u128, 1u16);
        // C s_22_2: const #0u : u8
        let s_22_2: bool = false;
        // C s_22_3: cast zx s_22_2 -> bv
        let s_22_3: Bits = Bits::new(s_22_2 as u128, 1u16);
        // D s_22_4: cmp-eq s_22_1 s_22_3
        let s_22_4: bool = ((s_22_1) == (s_22_3));
        // N s_22_5: branch s_22_4 b24 b23
        if s_22_4 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #64s : i64
        let s_23_0: i64 = 64;
        // C s_23_1: const #14160u : u32
        let s_23_1: u32 = 14160;
        // D s_23_2: read-reg s_23_1:u64
        let s_23_2: u64 = {
            let value = state.read_register::<u64>(s_23_1 as isize);
            tracer.read_register(s_23_1 as isize, value);
            value
        };
        // C s_23_3: const #() : ()
        let s_23_3: () = ();
        // S s_23_4: call PhysicalCountInt(s_23_3)
        let s_23_4: u64 = PhysicalCountInt(state, tracer, s_23_3);
        // D s_23_5: cast zx s_23_2 -> bv
        let s_23_5: Bits = Bits::new(s_23_2 as u128, 64u16);
        // S s_23_6: cast zx s_23_4 -> bv
        let s_23_6: Bits = Bits::new(s_23_4 as u128, 64u16);
        // D s_23_7: sub s_23_5 s_23_6
        let s_23_7: Bits = ((s_23_5) - (s_23_6));
        // D s_23_8: cast reint s_23_7 -> u64
        let s_23_8: u64 = (s_23_7.value() as u64);
        // D s_23_9: call Mk_CNTPS_TVAL_EL1_Type(s_23_8)
        let s_23_9: ProductType5c790c8ef59cc8b2 = Mk_CNTPS_TVAL_EL1_Type(
            state,
            tracer,
            s_23_8,
        );
        // D s_23_10: call __get_CNTPS_TVAL_EL1(s_23_9)
        let s_23_10: ProductType5c790c8ef59cc8b2 = u__get_CNTPS_TVAL_EL1(
            state,
            tracer,
            s_23_9,
        );
        // D s_23_11: write-var ga#56530 <= s_23_10
        fn_state.ga_56530 = s_23_10;
        // D s_23_12: read-var ga#56530.0:struct
        let s_23_12: u64 = fn_state.ga_56530._0;
        // D s_23_13: cast zx s_23_12 -> bv
        let s_23_13: Bits = Bits::new(s_23_12 as u128, 64u16);
        // D s_23_14: read-var t:i
        let s_23_14: i128 = fn_state.t;
        // D s_23_15: call X_set(s_23_14, s_23_0, s_23_13)
        let s_23_15: () = X_set(state, tracer, s_23_14, s_23_0, s_23_13);
        // N s_23_16: return
        return;
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #64s : i64
        let s_24_0: i64 = 64;
        // C s_24_1: const #64s : i64
        let s_24_1: i64 = 64;
        // C s_24_2: cast zx s_24_1 -> i
        let s_24_2: i128 = (i128::try_from(s_24_1).unwrap());
        // S s_24_3: call __UNKNOWN_bits(s_24_2)
        let s_24_3: Bits = u__UNKNOWN_bits(state, tracer, s_24_2);
        // S s_24_4: cast reint s_24_3 -> u64
        let s_24_4: u64 = (s_24_3.value() as u64);
        // S s_24_5: cast zx s_24_4 -> bv
        let s_24_5: Bits = Bits::new(s_24_4 as u128, 64u16);
        // D s_24_6: read-var t:i
        let s_24_6: i128 = fn_state.t;
        // D s_24_7: call X_set(s_24_6, s_24_0, s_24_5)
        let s_24_7: () = X_set(state, tracer, s_24_6, s_24_0, s_24_5);
        // N s_24_8: return
        return;
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var __CNTPS_CTL_EL1_ENABLE:u8
        let s_25_0: bool = fn_state.u__CNTPS_CTL_EL1_ENABLE;
        // D s_25_1: cast zx s_25_0 -> bv
        let s_25_1: Bits = Bits::new(s_25_0 as u128, 1u16);
        // C s_25_2: const #0u : u8
        let s_25_2: bool = false;
        // C s_25_3: cast zx s_25_2 -> bv
        let s_25_3: Bits = Bits::new(s_25_2 as u128, 1u16);
        // D s_25_4: cmp-eq s_25_1 s_25_3
        let s_25_4: bool = ((s_25_1) == (s_25_3));
        // N s_25_5: branch s_25_4 b27 b26
        if s_25_4 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #64s : i64
        let s_26_0: i64 = 64;
        // C s_26_1: const #14160u : u32
        let s_26_1: u32 = 14160;
        // D s_26_2: read-reg s_26_1:u64
        let s_26_2: u64 = {
            let value = state.read_register::<u64>(s_26_1 as isize);
            tracer.read_register(s_26_1 as isize, value);
            value
        };
        // C s_26_3: const #() : ()
        let s_26_3: () = ();
        // S s_26_4: call PhysicalCountInt(s_26_3)
        let s_26_4: u64 = PhysicalCountInt(state, tracer, s_26_3);
        // S s_26_5: cast zx s_26_4 -> bv
        let s_26_5: Bits = Bits::new(s_26_4 as u128, 64u16);
        // C s_26_6: const #14584u : u32
        let s_26_6: u32 = 14584;
        // D s_26_7: read-reg s_26_6:u64
        let s_26_7: u64 = {
            let value = state.read_register::<u64>(s_26_6 as isize);
            tracer.read_register(s_26_6 as isize, value);
            value
        };
        // D s_26_8: cast zx s_26_7 -> bv
        let s_26_8: Bits = Bits::new(s_26_7 as u128, 64u16);
        // D s_26_9: sub s_26_5 s_26_8
        let s_26_9: Bits = ((s_26_5) - (s_26_8));
        // D s_26_10: cast reint s_26_9 -> u64
        let s_26_10: u64 = (s_26_9.value() as u64);
        // D s_26_11: cast zx s_26_2 -> bv
        let s_26_11: Bits = Bits::new(s_26_2 as u128, 64u16);
        // D s_26_12: cast zx s_26_10 -> bv
        let s_26_12: Bits = Bits::new(s_26_10 as u128, 64u16);
        // D s_26_13: sub s_26_11 s_26_12
        let s_26_13: Bits = ((s_26_11) - (s_26_12));
        // D s_26_14: cast reint s_26_13 -> u64
        let s_26_14: u64 = (s_26_13.value() as u64);
        // D s_26_15: call Mk_CNTPS_TVAL_EL1_Type(s_26_14)
        let s_26_15: ProductType5c790c8ef59cc8b2 = Mk_CNTPS_TVAL_EL1_Type(
            state,
            tracer,
            s_26_14,
        );
        // D s_26_16: call __get_CNTPS_TVAL_EL1(s_26_15)
        let s_26_16: ProductType5c790c8ef59cc8b2 = u__get_CNTPS_TVAL_EL1(
            state,
            tracer,
            s_26_15,
        );
        // D s_26_17: write-var ga#56519 <= s_26_16
        fn_state.ga_56519 = s_26_16;
        // D s_26_18: read-var ga#56519.0:struct
        let s_26_18: u64 = fn_state.ga_56519._0;
        // D s_26_19: cast zx s_26_18 -> bv
        let s_26_19: Bits = Bits::new(s_26_18 as u128, 64u16);
        // D s_26_20: read-var t:i
        let s_26_20: i128 = fn_state.t;
        // D s_26_21: call X_set(s_26_20, s_26_0, s_26_19)
        let s_26_21: () = X_set(state, tracer, s_26_20, s_26_0, s_26_19);
        // N s_26_22: return
        return;
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #64s : i64
        let s_27_0: i64 = 64;
        // C s_27_1: const #64s : i64
        let s_27_1: i64 = 64;
        // C s_27_2: cast zx s_27_1 -> i
        let s_27_2: i128 = (i128::try_from(s_27_1).unwrap());
        // S s_27_3: call __UNKNOWN_bits(s_27_2)
        let s_27_3: Bits = u__UNKNOWN_bits(state, tracer, s_27_2);
        // S s_27_4: cast reint s_27_3 -> u64
        let s_27_4: u64 = (s_27_3.value() as u64);
        // S s_27_5: cast zx s_27_4 -> bv
        let s_27_5: Bits = Bits::new(s_27_4 as u128, 64u16);
        // D s_27_6: read-var t:i
        let s_27_6: i128 = fn_state.t;
        // D s_27_7: call X_set(s_27_6, s_27_0, s_27_5)
        let s_27_7: () = X_set(state, tracer, s_27_6, s_27_0, s_27_5);
        // N s_27_8: return
        return;
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var __CNTHCTL_EL2_ECV:u8
        let s_28_0: bool = fn_state.u__CNTHCTL_EL2_ECV;
        // D s_28_1: cast zx s_28_0 -> bv
        let s_28_1: Bits = Bits::new(s_28_0 as u128, 1u16);
        // C s_28_2: const #1u : u8
        let s_28_2: bool = true;
        // C s_28_3: cast zx s_28_2 -> bv
        let s_28_3: Bits = Bits::new(s_28_2 as u128, 1u16);
        // D s_28_4: cmp-eq s_28_1 s_28_3
        let s_28_4: bool = ((s_28_1) == (s_28_3));
        // D s_28_5: write-var gs#58025 <= s_28_4
        fn_state.gs_58025 = s_28_4;
        // N s_28_6: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var __SCR_EL3_ECVEn:u8
        let s_29_0: bool = fn_state.u__SCR_EL3_ECVEn;
        // D s_29_1: cast zx s_29_0 -> bv
        let s_29_1: Bits = Bits::new(s_29_0 as u128, 1u16);
        // C s_29_2: const #1u : u8
        let s_29_2: bool = true;
        // C s_29_3: cast zx s_29_2 -> bv
        let s_29_3: Bits = Bits::new(s_29_2 as u128, 1u16);
        // D s_29_4: cmp-eq s_29_1 s_29_3
        let s_29_4: bool = ((s_29_1) == (s_29_3));
        // D s_29_5: write-var gs#58024 <= s_29_4
        fn_state.gs_58024 = s_29_4;
        // N s_29_6: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #() : ()
        let s_30_0: () = ();
        // S s_30_1: call EL2Enabled(s_30_0)
        let s_30_1: bool = EL2Enabled(state, tracer, s_30_0);
        // D s_30_2: write-var gs#58023 <= s_30_1
        fn_state.gs_58023 = s_30_1;
        // N s_30_3: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #24u : u8
        let s_31_0: u8 = 24;
        // C s_31_1: cast zx s_31_0 -> bv
        let s_31_1: Bits = Bits::new(s_31_0 as u128, 8u16);
        // C s_31_2: cast zx s_31_1 -> i
        let s_31_2: i128 = (s_31_1.value() as i128);
        // C s_31_3: cast reint s_31_2 -> i64
        let s_31_3: i64 = (s_31_2 as i64);
        // C s_31_4: cast zx s_31_3 -> i
        let s_31_4: i128 = (i128::try_from(s_31_3).unwrap());
        // C s_31_5: const #424u : u32
        let s_31_5: u32 = 424;
        // D s_31_6: read-reg s_31_5:u8
        let s_31_6: u8 = {
            let value = state.read_register::<u8>(s_31_5 as isize);
            tracer.read_register(s_31_5 as isize, value);
            value
        };
        // D s_31_7: call AArch64_SystemAccessTrap(s_31_6, s_31_4)
        let s_31_7: () = AArch64_SystemAccessTrap(state, tracer, s_31_6, s_31_4);
        // N s_31_8: return
        return;
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_32_0: panic
        panic!("{:?}", ());
        // N s_32_1: return
        return;
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var __SCR_EL3_NS:u8
        let s_33_0: bool = fn_state.u__SCR_EL3_NS;
        // D s_33_1: cast zx s_33_0 -> bv
        let s_33_1: Bits = Bits::new(s_33_0 as u128, 1u16);
        // C s_33_2: const #0u : u8
        let s_33_2: bool = false;
        // C s_33_3: cast zx s_33_2 -> bv
        let s_33_3: Bits = Bits::new(s_33_2 as u128, 1u16);
        // D s_33_4: cmp-eq s_33_1 s_33_3
        let s_33_4: bool = ((s_33_1) == (s_33_3));
        // D s_33_5: write-var gs#58022 <= s_33_4
        fn_state.gs_58022 = s_33_4;
        // N s_33_6: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_34_0: panic
        panic!("{:?}", ());
        // N s_34_1: return
        return;
    }
}
