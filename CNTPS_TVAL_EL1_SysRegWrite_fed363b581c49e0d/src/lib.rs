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
use IsFeatureImplemented::*;
use EL2Enabled::*;
use AArch64_SystemAccessTrap::*;
use u_get_SCR_EL3_Type_ECVEn::*;
use Mk_CNTPS_CVAL_EL1_Type::*;
use u_get_SCR_EL3_Type_ST::*;
use u_get_SCR_EL3_Type_EEL2::*;
use PhysicalCountInt::*;
use u_get_SCR_EL3_Type_NS::*;
use X_read::*;
use u_get_CNTHCTL_EL2_Type_ECV::*;
use common::*;
pub fn CNTPS_TVAL_EL1_SysRegWrite_fed363b581c49e0d<T: Tracer>(
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
        u__SCR_EL3_ECVEn: bool,
        u__SCR_EL3_NS: bool,
        gs_81832: bool,
        gs_81831: bool,
        gs_81830: bool,
        u__CNTHCTL_EL2_ECV: bool,
        gs_81829: bool,
        u__PSTATE_EL: u8,
        u__SCR_EL3_ST: bool,
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
        // D s_0_23: read-var __PSTATE_EL:u8
        let s_0_23: u8 = fn_state.u__PSTATE_EL;
        // D s_0_24: cast zx s_0_23 -> bv
        let s_0_24: Bits = Bits::new(s_0_23 as u128, 2u16);
        // C s_0_25: const #448u : u32
        let s_0_25: u32 = 448;
        // D s_0_26: read-reg s_0_25:u8
        let s_0_26: u8 = {
            let value = state.read_register::<u8>(s_0_25 as isize);
            tracer.read_register(s_0_25 as isize, value);
            value
        };
        // D s_0_27: cast zx s_0_26 -> bv
        let s_0_27: Bits = Bits::new(s_0_26 as u128, 2u16);
        // D s_0_28: cmp-eq s_0_24 s_0_27
        let s_0_28: bool = ((s_0_24) == (s_0_27));
        // N s_0_29: branch s_0_28 b28 b1
        if s_0_28 {
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
        // C s_5_0: const #64s : i64
        let s_5_0: i64 = 64;
        // D s_5_1: read-var t:i
        let s_5_1: i128 = fn_state.t;
        // D s_5_2: call X_read(s_5_1, s_5_0)
        let s_5_2: Bits = X_read(state, tracer, s_5_1, s_5_0);
        // D s_5_3: cast reint s_5_2 -> u64
        let s_5_3: u64 = (s_5_2.value() as u64);
        // C s_5_4: const #0s : i
        let s_5_4: i128 = 0;
        // D s_5_5: cast zx s_5_3 -> bv
        let s_5_5: Bits = Bits::new(s_5_3 as u128, 64u16);
        // C s_5_6: const #1s : i64
        let s_5_6: i64 = 1;
        // C s_5_7: cast zx s_5_6 -> i
        let s_5_7: i128 = (i128::try_from(s_5_6).unwrap());
        // C s_5_8: const #31s : i
        let s_5_8: i128 = 31;
        // C s_5_9: add s_5_8 s_5_7
        let s_5_9: i128 = (s_5_8 + s_5_7);
        // D s_5_10: bit-extract s_5_5 s_5_4 s_5_9
        let s_5_10: Bits = (Bits::new(
            ((s_5_5) >> (s_5_4)).value(),
            u16::try_from(s_5_9).unwrap(),
        ));
        // D s_5_11: cast reint s_5_10 -> u32
        let s_5_11: u32 = (s_5_10.value() as u32);
        // C s_5_12: const #64s : i
        let s_5_12: i128 = 64;
        // D s_5_13: cast zx s_5_11 -> bv
        let s_5_13: Bits = Bits::new(s_5_11 as u128, 32u16);
        // D s_5_14: bits-cast sx s_5_13 -> bv length s_5_12
        let s_5_14: Bits = s_5_13.sign_extend(s_5_12);
        // D s_5_15: cast reint s_5_14 -> u64
        let s_5_15: u64 = (s_5_14.value() as u64);
        // C s_5_16: const #() : ()
        let s_5_16: () = ();
        // S s_5_17: call PhysicalCountInt(s_5_16)
        let s_5_17: u64 = PhysicalCountInt(state, tracer, s_5_16);
        // D s_5_18: cast zx s_5_15 -> bv
        let s_5_18: Bits = Bits::new(s_5_15 as u128, 64u16);
        // S s_5_19: cast zx s_5_17 -> bv
        let s_5_19: Bits = Bits::new(s_5_17 as u128, 64u16);
        // D s_5_20: add s_5_18 s_5_19
        let s_5_20: Bits = (s_5_18 + s_5_19);
        // D s_5_21: cast reint s_5_20 -> u64
        let s_5_21: u64 = (s_5_20.value() as u64);
        // D s_5_22: call Mk_CNTPS_CVAL_EL1_Type(s_5_21)
        let s_5_22: ProductType5c790c8ef59cc8b2 = Mk_CNTPS_CVAL_EL1_Type(
            state,
            tracer,
            s_5_21,
        );
        // C s_5_23: const #14160u : u32
        let s_5_23: u32 = 14160;
        // N s_5_24: write-reg s_5_23 <= s_5_22
        let s_5_24: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_5_23 as isize, s_5_22);
            tracer.write_register(s_5_23 as isize, s_5_22);
        };
        // N s_5_25: return
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
        // C s_7_0: const #424u : u32
        let s_7_0: u32 = 424;
        // D s_7_1: read-reg s_7_0:u8
        let s_7_1: u8 = {
            let value = state.read_register::<u8>(s_7_0 as isize);
            tracer.read_register(s_7_0 as isize, value);
            value
        };
        // C s_7_2: const #2u : u8
        let s_7_2: u8 = 2;
        // D s_7_3: cmp-lt s_7_1 s_7_2
        let s_7_3: bool = ((s_7_1) < (s_7_2));
        // N s_7_4: branch s_7_3 b27 b8
        if s_7_3 {
            return block_27(state, tracer, fn_state);
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
        // D s_8_1: write-var gs#81829 <= s_8_0
        fn_state.gs_81829 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#81829:u8
        let s_9_0: bool = fn_state.gs_81829;
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
        // D s_11_0: read-var __SCR_EL3_EEL2:u8
        let s_11_0: bool = fn_state.u__SCR_EL3_EEL2;
        // D s_11_1: cast zx s_11_0 -> bv
        let s_11_1: Bits = Bits::new(s_11_0 as u128, 1u16);
        // C s_11_2: const #1u : u8
        let s_11_2: bool = true;
        // C s_11_3: cast zx s_11_2 -> bv
        let s_11_3: Bits = Bits::new(s_11_2 as u128, 1u16);
        // D s_11_4: cmp-eq s_11_1 s_11_3
        let s_11_4: bool = ((s_11_1) == (s_11_3));
        // N s_11_5: branch s_11_4 b26 b12
        if s_11_4 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var __SCR_EL3_ST:u8
        let s_12_0: bool = fn_state.u__SCR_EL3_ST;
        // D s_12_1: cast zx s_12_0 -> bv
        let s_12_1: Bits = Bits::new(s_12_0 as u128, 1u16);
        // C s_12_2: const #0u : u8
        let s_12_2: bool = false;
        // C s_12_3: cast zx s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 1u16);
        // D s_12_4: cmp-eq s_12_1 s_12_3
        let s_12_4: bool = ((s_12_1) == (s_12_3));
        // N s_12_5: branch s_12_4 b25 b13
        if s_12_4 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #145u : u32
        let s_13_0: u32 = 145;
        // S s_13_1: call IsFeatureImplemented(s_13_0)
        let s_13_1: bool = IsFeatureImplemented(state, tracer, s_13_0);
        // N s_13_2: branch s_13_1 b24 b14
        if s_13_1 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // D s_14_1: write-var gs#81830 <= s_14_0
        fn_state.gs_81830 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#81830:u8
        let s_15_0: bool = fn_state.gs_81830;
        // N s_15_1: branch s_15_0 b23 b16
        if s_15_0 {
            return block_23(state, tracer, fn_state);
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
        // D s_16_1: write-var gs#81831 <= s_16_0
        fn_state.gs_81831 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#81831:u8
        let s_17_0: bool = fn_state.gs_81831;
        // N s_17_1: branch s_17_0 b22 b18
        if s_17_0 {
            return block_22(state, tracer, fn_state);
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
        // D s_18_1: write-var gs#81832 <= s_18_0
        fn_state.gs_81832 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#81832:u8
        let s_19_0: bool = fn_state.gs_81832;
        // N s_19_1: branch s_19_0 b21 b20
        if s_19_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
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
        // C s_20_4: const #0s : i
        let s_20_4: i128 = 0;
        // D s_20_5: cast zx s_20_3 -> bv
        let s_20_5: Bits = Bits::new(s_20_3 as u128, 64u16);
        // C s_20_6: const #1s : i64
        let s_20_6: i64 = 1;
        // C s_20_7: cast zx s_20_6 -> i
        let s_20_7: i128 = (i128::try_from(s_20_6).unwrap());
        // C s_20_8: const #31s : i
        let s_20_8: i128 = 31;
        // C s_20_9: add s_20_8 s_20_7
        let s_20_9: i128 = (s_20_8 + s_20_7);
        // D s_20_10: bit-extract s_20_5 s_20_4 s_20_9
        let s_20_10: Bits = (Bits::new(
            ((s_20_5) >> (s_20_4)).value(),
            u16::try_from(s_20_9).unwrap(),
        ));
        // D s_20_11: cast reint s_20_10 -> u32
        let s_20_11: u32 = (s_20_10.value() as u32);
        // C s_20_12: const #64s : i
        let s_20_12: i128 = 64;
        // D s_20_13: cast zx s_20_11 -> bv
        let s_20_13: Bits = Bits::new(s_20_11 as u128, 32u16);
        // D s_20_14: bits-cast sx s_20_13 -> bv length s_20_12
        let s_20_14: Bits = s_20_13.sign_extend(s_20_12);
        // D s_20_15: cast reint s_20_14 -> u64
        let s_20_15: u64 = (s_20_14.value() as u64);
        // C s_20_16: const #() : ()
        let s_20_16: () = ();
        // S s_20_17: call PhysicalCountInt(s_20_16)
        let s_20_17: u64 = PhysicalCountInt(state, tracer, s_20_16);
        // D s_20_18: cast zx s_20_15 -> bv
        let s_20_18: Bits = Bits::new(s_20_15 as u128, 64u16);
        // S s_20_19: cast zx s_20_17 -> bv
        let s_20_19: Bits = Bits::new(s_20_17 as u128, 64u16);
        // D s_20_20: add s_20_18 s_20_19
        let s_20_20: Bits = (s_20_18 + s_20_19);
        // D s_20_21: cast reint s_20_20 -> u64
        let s_20_21: u64 = (s_20_20.value() as u64);
        // D s_20_22: call Mk_CNTPS_CVAL_EL1_Type(s_20_21)
        let s_20_22: ProductType5c790c8ef59cc8b2 = Mk_CNTPS_CVAL_EL1_Type(
            state,
            tracer,
            s_20_21,
        );
        // C s_20_23: const #14160u : u32
        let s_20_23: u32 = 14160;
        // N s_20_24: write-reg s_20_23 <= s_20_22
        let s_20_24: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_20_23 as isize, s_20_22);
            tracer.write_register(s_20_23 as isize, s_20_22);
        };
        // N s_20_25: return
        return;
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #64s : i64
        let s_21_0: i64 = 64;
        // D s_21_1: read-var t:i
        let s_21_1: i128 = fn_state.t;
        // D s_21_2: call X_read(s_21_1, s_21_0)
        let s_21_2: Bits = X_read(state, tracer, s_21_1, s_21_0);
        // D s_21_3: cast reint s_21_2 -> u64
        let s_21_3: u64 = (s_21_2.value() as u64);
        // C s_21_4: const #0s : i
        let s_21_4: i128 = 0;
        // D s_21_5: cast zx s_21_3 -> bv
        let s_21_5: Bits = Bits::new(s_21_3 as u128, 64u16);
        // C s_21_6: const #1s : i64
        let s_21_6: i64 = 1;
        // C s_21_7: cast zx s_21_6 -> i
        let s_21_7: i128 = (i128::try_from(s_21_6).unwrap());
        // C s_21_8: const #31s : i
        let s_21_8: i128 = 31;
        // C s_21_9: add s_21_8 s_21_7
        let s_21_9: i128 = (s_21_8 + s_21_7);
        // D s_21_10: bit-extract s_21_5 s_21_4 s_21_9
        let s_21_10: Bits = (Bits::new(
            ((s_21_5) >> (s_21_4)).value(),
            u16::try_from(s_21_9).unwrap(),
        ));
        // D s_21_11: cast reint s_21_10 -> u32
        let s_21_11: u32 = (s_21_10.value() as u32);
        // C s_21_12: const #64s : i
        let s_21_12: i128 = 64;
        // D s_21_13: cast zx s_21_11 -> bv
        let s_21_13: Bits = Bits::new(s_21_11 as u128, 32u16);
        // D s_21_14: bits-cast sx s_21_13 -> bv length s_21_12
        let s_21_14: Bits = s_21_13.sign_extend(s_21_12);
        // D s_21_15: cast reint s_21_14 -> u64
        let s_21_15: u64 = (s_21_14.value() as u64);
        // C s_21_16: const #() : ()
        let s_21_16: () = ();
        // S s_21_17: call PhysicalCountInt(s_21_16)
        let s_21_17: u64 = PhysicalCountInt(state, tracer, s_21_16);
        // D s_21_18: cast zx s_21_15 -> bv
        let s_21_18: Bits = Bits::new(s_21_15 as u128, 64u16);
        // S s_21_19: cast zx s_21_17 -> bv
        let s_21_19: Bits = Bits::new(s_21_17 as u128, 64u16);
        // D s_21_20: add s_21_18 s_21_19
        let s_21_20: Bits = (s_21_18 + s_21_19);
        // D s_21_21: cast reint s_21_20 -> u64
        let s_21_21: u64 = (s_21_20.value() as u64);
        // D s_21_22: cast zx s_21_21 -> bv
        let s_21_22: Bits = Bits::new(s_21_21 as u128, 64u16);
        // C s_21_23: const #14584u : u32
        let s_21_23: u32 = 14584;
        // D s_21_24: read-reg s_21_23:u64
        let s_21_24: u64 = {
            let value = state.read_register::<u64>(s_21_23 as isize);
            tracer.read_register(s_21_23 as isize, value);
            value
        };
        // D s_21_25: cast zx s_21_24 -> bv
        let s_21_25: Bits = Bits::new(s_21_24 as u128, 64u16);
        // D s_21_26: sub s_21_22 s_21_25
        let s_21_26: Bits = ((s_21_22) - (s_21_25));
        // D s_21_27: cast reint s_21_26 -> u64
        let s_21_27: u64 = (s_21_26.value() as u64);
        // D s_21_28: call Mk_CNTPS_CVAL_EL1_Type(s_21_27)
        let s_21_28: ProductType5c790c8ef59cc8b2 = Mk_CNTPS_CVAL_EL1_Type(
            state,
            tracer,
            s_21_27,
        );
        // C s_21_29: const #14160u : u32
        let s_21_29: u32 = 14160;
        // N s_21_30: write-reg s_21_29 <= s_21_28
        let s_21_30: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_21_29 as isize, s_21_28);
            tracer.write_register(s_21_29 as isize, s_21_28);
        };
        // N s_21_31: return
        return;
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var __CNTHCTL_EL2_ECV:u8
        let s_22_0: bool = fn_state.u__CNTHCTL_EL2_ECV;
        // D s_22_1: cast zx s_22_0 -> bv
        let s_22_1: Bits = Bits::new(s_22_0 as u128, 1u16);
        // C s_22_2: const #1u : u8
        let s_22_2: bool = true;
        // C s_22_3: cast zx s_22_2 -> bv
        let s_22_3: Bits = Bits::new(s_22_2 as u128, 1u16);
        // D s_22_4: cmp-eq s_22_1 s_22_3
        let s_22_4: bool = ((s_22_1) == (s_22_3));
        // D s_22_5: write-var gs#81832 <= s_22_4
        fn_state.gs_81832 = s_22_4;
        // N s_22_6: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var __SCR_EL3_ECVEn:u8
        let s_23_0: bool = fn_state.u__SCR_EL3_ECVEn;
        // D s_23_1: cast zx s_23_0 -> bv
        let s_23_1: Bits = Bits::new(s_23_0 as u128, 1u16);
        // C s_23_2: const #1u : u8
        let s_23_2: bool = true;
        // C s_23_3: cast zx s_23_2 -> bv
        let s_23_3: Bits = Bits::new(s_23_2 as u128, 1u16);
        // D s_23_4: cmp-eq s_23_1 s_23_3
        let s_23_4: bool = ((s_23_1) == (s_23_3));
        // D s_23_5: write-var gs#81831 <= s_23_4
        fn_state.gs_81831 = s_23_4;
        // N s_23_6: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #() : ()
        let s_24_0: () = ();
        // S s_24_1: call EL2Enabled(s_24_0)
        let s_24_1: bool = EL2Enabled(state, tracer, s_24_0);
        // D s_24_2: write-var gs#81830 <= s_24_1
        fn_state.gs_81830 = s_24_1;
        // N s_24_3: jump b15
        return block_15(state, tracer, fn_state);
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
        // C s_25_5: const #424u : u32
        let s_25_5: u32 = 424;
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
        // N s_26_0: panic
        panic!("{:?}", ());
        // N s_26_1: return
        return;
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var __SCR_EL3_NS:u8
        let s_27_0: bool = fn_state.u__SCR_EL3_NS;
        // D s_27_1: cast zx s_27_0 -> bv
        let s_27_1: Bits = Bits::new(s_27_0 as u128, 1u16);
        // C s_27_2: const #0u : u8
        let s_27_2: bool = false;
        // C s_27_3: cast zx s_27_2 -> bv
        let s_27_3: Bits = Bits::new(s_27_2 as u128, 1u16);
        // D s_27_4: cmp-eq s_27_1 s_27_3
        let s_27_4: bool = ((s_27_1) == (s_27_3));
        // D s_27_5: write-var gs#81829 <= s_27_4
        fn_state.gs_81829 = s_27_4;
        // N s_27_6: jump b9
        return block_9(state, tracer, fn_state);
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
