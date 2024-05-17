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
use u_get_CNTHCTL_EL2_Type_ECV::*;
use EL2Enabled::*;
use u_get_HCR_EL2_Type_E2H::*;
use Mk_CNTHPS_CVAL_EL2_Type::*;
use u_get_CNTHCTL_EL2_Type_EL1PTEN::*;
use IsFeatureImplemented::*;
use X_read::*;
use AArch64_SystemAccessTrap::*;
use u_get_SCR_EL3_Type_NS::*;
use u_get_SCR_EL3_Type_ECVEn::*;
use u_get_CNTHCTL_EL2_Type_EL1PCEN::*;
use u_get_CNTHCTL_EL2_Type_EL0PTEN::*;
use Mk_CNTP_CVAL_EL0_Type::*;
use u_get_CNTKCTL_EL1_Type_EL0PTEN::*;
use Mk_CNTHP_CVAL_EL2_Type::*;
use u_get_HCR_EL2_Type_TGE::*;
use PhysicalCountInt::*;
use common::*;
pub fn CNTP_TVAL_EL0_SysRegWrite_eccdee10d8c7f39e<T: Tracer>(
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
        gs_81876: bool,
        gs_81874: bool,
        gs_81910: bool,
        gs_81904: bool,
        gs_81889: bool,
        gs_81919: bool,
        u__SCR_EL3_ECVEn: bool,
        gs_81913: bool,
        gs_81906: bool,
        gs_81920: bool,
        gs_81875: bool,
        gs_81907: bool,
        gs_81892: bool,
        gs_81912: bool,
        gs_81895: bool,
        gs_81908: bool,
        gs_81905: bool,
        u__CNTHCTL_EL2_ECV: bool,
        u__PSTATE_EL: u8,
        u__CNTHCTL_EL2_EL1PCEN: bool,
        gs_81916: bool,
        gs_81890: bool,
        u__HCR_EL2_TGE: bool,
        gs_81894: bool,
        gs_81911: bool,
        gs_81937: bool,
        gs_81909: bool,
        gs_81891: bool,
        gs_81917: bool,
        u__CNTHCTL_EL2_EL0PTEN: bool,
        gs_81918: bool,
        gs_81914: bool,
        gs_81915: bool,
        u__CNTKCTL_EL1_EL0PTEN: bool,
        gs_81893: bool,
        u__CNTHCTL_EL2_EL1PTEN: bool,
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
        // C s_0_3: const #22056u : u32
        let s_0_3: u32 = 22056;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_CNTKCTL_EL1_Type_EL0PTEN(s_0_4)
        let s_0_5: bool = u_get_CNTKCTL_EL1_Type_EL0PTEN(state, tracer, s_0_4);
        // D s_0_6: write-var __CNTKCTL_EL1_EL0PTEN <= s_0_5
        fn_state.u__CNTKCTL_EL1_EL0PTEN = s_0_5;
        // C s_0_7: const #102552u : u32
        let s_0_7: u32 = 102552;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_HCR_EL2_Type_TGE(s_0_8)
        let s_0_9: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_0_8);
        // D s_0_10: write-var __HCR_EL2_TGE <= s_0_9
        fn_state.u__HCR_EL2_TGE = s_0_9;
        // C s_0_11: const #12808u : u32
        let s_0_11: u32 = 12808;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_CNTHCTL_EL2_Type_EL1PCEN(s_0_12)
        let s_0_13: bool = u_get_CNTHCTL_EL2_Type_EL1PCEN(state, tracer, s_0_12);
        // D s_0_14: write-var __CNTHCTL_EL2_EL1PCEN <= s_0_13
        fn_state.u__CNTHCTL_EL2_EL1PCEN = s_0_13;
        // C s_0_15: const #12808u : u32
        let s_0_15: u32 = 12808;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_CNTHCTL_EL2_Type_EL1PTEN(s_0_16)
        let s_0_17: bool = u_get_CNTHCTL_EL2_Type_EL1PTEN(state, tracer, s_0_16);
        // D s_0_18: write-var __CNTHCTL_EL2_EL1PTEN <= s_0_17
        fn_state.u__CNTHCTL_EL2_EL1PTEN = s_0_17;
        // C s_0_19: const #12808u : u32
        let s_0_19: u32 = 12808;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_CNTHCTL_EL2_Type_EL0PTEN(s_0_20)
        let s_0_21: bool = u_get_CNTHCTL_EL2_Type_EL0PTEN(state, tracer, s_0_20);
        // D s_0_22: write-var __CNTHCTL_EL2_EL0PTEN <= s_0_21
        fn_state.u__CNTHCTL_EL2_EL0PTEN = s_0_21;
        // C s_0_23: const #90704u : u32
        let s_0_23: u32 = 90704;
        // D s_0_24: read-reg s_0_23:struct
        let s_0_24: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_23 as isize);
            tracer.read_register(s_0_23 as isize, value);
            value
        };
        // D s_0_25: call _get_SCR_EL3_Type_ECVEn(s_0_24)
        let s_0_25: bool = u_get_SCR_EL3_Type_ECVEn(state, tracer, s_0_24);
        // D s_0_26: write-var __SCR_EL3_ECVEn <= s_0_25
        fn_state.u__SCR_EL3_ECVEn = s_0_25;
        // C s_0_27: const #12808u : u32
        let s_0_27: u32 = 12808;
        // D s_0_28: read-reg s_0_27:struct
        let s_0_28: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_27 as isize);
            tracer.read_register(s_0_27 as isize, value);
            value
        };
        // D s_0_29: call _get_CNTHCTL_EL2_Type_ECV(s_0_28)
        let s_0_29: bool = u_get_CNTHCTL_EL2_Type_ECV(state, tracer, s_0_28);
        // D s_0_30: write-var __CNTHCTL_EL2_ECV <= s_0_29
        fn_state.u__CNTHCTL_EL2_ECV = s_0_29;
        // D s_0_31: read-var __PSTATE_EL:u8
        let s_0_31: u8 = fn_state.u__PSTATE_EL;
        // D s_0_32: cast zx s_0_31 -> bv
        let s_0_32: Bits = Bits::new(s_0_31 as u128, 2u16);
        // C s_0_33: const #448u : u32
        let s_0_33: u32 = 448;
        // D s_0_34: read-reg s_0_33:u8
        let s_0_34: u8 = {
            let value = state.read_register::<u8>(s_0_33 as isize);
            tracer.read_register(s_0_33 as isize, value);
            value
        };
        // D s_0_35: cast zx s_0_34 -> bv
        let s_0_35: Bits = Bits::new(s_0_34 as u128, 2u16);
        // D s_0_36: cmp-eq s_0_32 s_0_35
        let s_0_36: bool = ((s_0_32) == (s_0_35));
        // N s_0_37: branch s_0_36 b48 b1
        if s_0_36 {
            return block_48(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b20 b2
        if s_1_5 {
            return block_20(state, tracer, fn_state);
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
        // D s_5_22: call Mk_CNTP_CVAL_EL0_Type(s_5_21)
        let s_5_22: ProductType5c790c8ef59cc8b2 = Mk_CNTP_CVAL_EL0_Type(
            state,
            tracer,
            s_5_21,
        );
        // C s_5_23: const #20800u : u32
        let s_5_23: u32 = 20800;
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
        // C s_6_0: const #102552u : u32
        let s_6_0: u32 = 102552;
        // D s_6_1: read-reg s_6_0:struct
        let s_6_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_6_0 as isize);
            tracer.read_register(s_6_0 as isize, value);
            value
        };
        // D s_6_2: call _get_HCR_EL2_Type_E2H(s_6_1)
        let s_6_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_6_1);
        // D s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 1u16);
        // C s_6_4: const #1u : u8
        let s_6_4: bool = true;
        // C s_6_5: cast zx s_6_4 -> bv
        let s_6_5: Bits = Bits::new(s_6_4 as u128, 1u16);
        // D s_6_6: cmp-eq s_6_3 s_6_5
        let s_6_6: bool = ((s_6_3) == (s_6_5));
        // N s_6_7: branch s_6_6 b19 b7
        if s_6_6 {
            return block_19(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#81874 <= s_7_0
        fn_state.gs_81874 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#81874:u8
        let s_8_0: bool = fn_state.gs_81874;
        // N s_8_1: branch s_8_0 b18 b9
        if s_8_0 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#81875 <= s_9_0
        fn_state.gs_81875 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#81875:u8
        let s_10_0: bool = fn_state.gs_81875;
        // N s_10_1: branch s_10_0 b17 b11
        if s_10_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
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
        // D s_11_3: cast zx s_11_2 -> bv
        let s_11_3: Bits = Bits::new(s_11_2 as u128, 1u16);
        // C s_11_4: const #1u : u8
        let s_11_4: bool = true;
        // C s_11_5: cast zx s_11_4 -> bv
        let s_11_5: Bits = Bits::new(s_11_4 as u128, 1u16);
        // D s_11_6: cmp-eq s_11_3 s_11_5
        let s_11_6: bool = ((s_11_3) == (s_11_5));
        // N s_11_7: branch s_11_6 b16 b12
        if s_11_6 {
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
        // D s_12_1: write-var gs#81876 <= s_12_0
        fn_state.gs_81876 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#81876:u8
        let s_13_0: bool = fn_state.gs_81876;
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
        // C s_14_0: const #64s : i64
        let s_14_0: i64 = 64;
        // D s_14_1: read-var t:i
        let s_14_1: i128 = fn_state.t;
        // D s_14_2: call X_read(s_14_1, s_14_0)
        let s_14_2: Bits = X_read(state, tracer, s_14_1, s_14_0);
        // D s_14_3: cast reint s_14_2 -> u64
        let s_14_3: u64 = (s_14_2.value() as u64);
        // C s_14_4: const #0s : i
        let s_14_4: i128 = 0;
        // D s_14_5: cast zx s_14_3 -> bv
        let s_14_5: Bits = Bits::new(s_14_3 as u128, 64u16);
        // C s_14_6: const #1s : i64
        let s_14_6: i64 = 1;
        // C s_14_7: cast zx s_14_6 -> i
        let s_14_7: i128 = (i128::try_from(s_14_6).unwrap());
        // C s_14_8: const #31s : i
        let s_14_8: i128 = 31;
        // C s_14_9: add s_14_8 s_14_7
        let s_14_9: i128 = (s_14_8 + s_14_7);
        // D s_14_10: bit-extract s_14_5 s_14_4 s_14_9
        let s_14_10: Bits = (Bits::new(
            ((s_14_5) >> (s_14_4)).value(),
            u16::try_from(s_14_9).unwrap(),
        ));
        // D s_14_11: cast reint s_14_10 -> u32
        let s_14_11: u32 = (s_14_10.value() as u32);
        // C s_14_12: const #64s : i
        let s_14_12: i128 = 64;
        // D s_14_13: cast zx s_14_11 -> bv
        let s_14_13: Bits = Bits::new(s_14_11 as u128, 32u16);
        // D s_14_14: bits-cast sx s_14_13 -> bv length s_14_12
        let s_14_14: Bits = s_14_13.sign_extend(s_14_12);
        // D s_14_15: cast reint s_14_14 -> u64
        let s_14_15: u64 = (s_14_14.value() as u64);
        // C s_14_16: const #() : ()
        let s_14_16: () = ();
        // S s_14_17: call PhysicalCountInt(s_14_16)
        let s_14_17: u64 = PhysicalCountInt(state, tracer, s_14_16);
        // D s_14_18: cast zx s_14_15 -> bv
        let s_14_18: Bits = Bits::new(s_14_15 as u128, 64u16);
        // S s_14_19: cast zx s_14_17 -> bv
        let s_14_19: Bits = Bits::new(s_14_17 as u128, 64u16);
        // D s_14_20: add s_14_18 s_14_19
        let s_14_20: Bits = (s_14_18 + s_14_19);
        // D s_14_21: cast reint s_14_20 -> u64
        let s_14_21: u64 = (s_14_20.value() as u64);
        // D s_14_22: call Mk_CNTP_CVAL_EL0_Type(s_14_21)
        let s_14_22: ProductType5c790c8ef59cc8b2 = Mk_CNTP_CVAL_EL0_Type(
            state,
            tracer,
            s_14_21,
        );
        // C s_14_23: const #20800u : u32
        let s_14_23: u32 = 20800;
        // N s_14_24: write-reg s_14_23 <= s_14_22
        let s_14_24: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_14_23 as isize, s_14_22);
            tracer.write_register(s_14_23 as isize, s_14_22);
        };
        // N s_14_25: return
        return;
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
        // C s_15_4: const #0s : i
        let s_15_4: i128 = 0;
        // D s_15_5: cast zx s_15_3 -> bv
        let s_15_5: Bits = Bits::new(s_15_3 as u128, 64u16);
        // C s_15_6: const #1s : i64
        let s_15_6: i64 = 1;
        // C s_15_7: cast zx s_15_6 -> i
        let s_15_7: i128 = (i128::try_from(s_15_6).unwrap());
        // C s_15_8: const #31s : i
        let s_15_8: i128 = 31;
        // C s_15_9: add s_15_8 s_15_7
        let s_15_9: i128 = (s_15_8 + s_15_7);
        // D s_15_10: bit-extract s_15_5 s_15_4 s_15_9
        let s_15_10: Bits = (Bits::new(
            ((s_15_5) >> (s_15_4)).value(),
            u16::try_from(s_15_9).unwrap(),
        ));
        // D s_15_11: cast reint s_15_10 -> u32
        let s_15_11: u32 = (s_15_10.value() as u32);
        // C s_15_12: const #64s : i
        let s_15_12: i128 = 64;
        // D s_15_13: cast zx s_15_11 -> bv
        let s_15_13: Bits = Bits::new(s_15_11 as u128, 32u16);
        // D s_15_14: bits-cast sx s_15_13 -> bv length s_15_12
        let s_15_14: Bits = s_15_13.sign_extend(s_15_12);
        // D s_15_15: cast reint s_15_14 -> u64
        let s_15_15: u64 = (s_15_14.value() as u64);
        // C s_15_16: const #() : ()
        let s_15_16: () = ();
        // S s_15_17: call PhysicalCountInt(s_15_16)
        let s_15_17: u64 = PhysicalCountInt(state, tracer, s_15_16);
        // D s_15_18: cast zx s_15_15 -> bv
        let s_15_18: Bits = Bits::new(s_15_15 as u128, 64u16);
        // S s_15_19: cast zx s_15_17 -> bv
        let s_15_19: Bits = Bits::new(s_15_17 as u128, 64u16);
        // D s_15_20: add s_15_18 s_15_19
        let s_15_20: Bits = (s_15_18 + s_15_19);
        // D s_15_21: cast reint s_15_20 -> u64
        let s_15_21: u64 = (s_15_20.value() as u64);
        // D s_15_22: call Mk_CNTHP_CVAL_EL2_Type(s_15_21)
        let s_15_22: ProductType5c790c8ef59cc8b2 = Mk_CNTHP_CVAL_EL2_Type(
            state,
            tracer,
            s_15_21,
        );
        // C s_15_23: const #16640u : u32
        let s_15_23: u32 = 16640;
        // N s_15_24: write-reg s_15_23 <= s_15_22
        let s_15_24: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_15_23 as isize, s_15_22);
            tracer.write_register(s_15_23 as isize, s_15_22);
        };
        // N s_15_25: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #90704u : u32
        let s_16_0: u32 = 90704;
        // D s_16_1: read-reg s_16_0:struct
        let s_16_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_16_0 as isize);
            tracer.read_register(s_16_0 as isize, value);
            value
        };
        // D s_16_2: call _get_SCR_EL3_Type_NS(s_16_1)
        let s_16_2: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_16_1);
        // D s_16_3: cast zx s_16_2 -> bv
        let s_16_3: Bits = Bits::new(s_16_2 as u128, 1u16);
        // C s_16_4: const #1u : u8
        let s_16_4: bool = true;
        // C s_16_5: cast zx s_16_4 -> bv
        let s_16_5: Bits = Bits::new(s_16_4 as u128, 1u16);
        // D s_16_6: cmp-eq s_16_3 s_16_5
        let s_16_6: bool = ((s_16_3) == (s_16_5));
        // D s_16_7: write-var gs#81876 <= s_16_6
        fn_state.gs_81876 = s_16_6;
        // N s_16_8: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #64s : i64
        let s_17_0: i64 = 64;
        // D s_17_1: read-var t:i
        let s_17_1: i128 = fn_state.t;
        // D s_17_2: call X_read(s_17_1, s_17_0)
        let s_17_2: Bits = X_read(state, tracer, s_17_1, s_17_0);
        // D s_17_3: cast reint s_17_2 -> u64
        let s_17_3: u64 = (s_17_2.value() as u64);
        // C s_17_4: const #0s : i
        let s_17_4: i128 = 0;
        // D s_17_5: cast zx s_17_3 -> bv
        let s_17_5: Bits = Bits::new(s_17_3 as u128, 64u16);
        // C s_17_6: const #1s : i64
        let s_17_6: i64 = 1;
        // C s_17_7: cast zx s_17_6 -> i
        let s_17_7: i128 = (i128::try_from(s_17_6).unwrap());
        // C s_17_8: const #31s : i
        let s_17_8: i128 = 31;
        // C s_17_9: add s_17_8 s_17_7
        let s_17_9: i128 = (s_17_8 + s_17_7);
        // D s_17_10: bit-extract s_17_5 s_17_4 s_17_9
        let s_17_10: Bits = (Bits::new(
            ((s_17_5) >> (s_17_4)).value(),
            u16::try_from(s_17_9).unwrap(),
        ));
        // D s_17_11: cast reint s_17_10 -> u32
        let s_17_11: u32 = (s_17_10.value() as u32);
        // C s_17_12: const #64s : i
        let s_17_12: i128 = 64;
        // D s_17_13: cast zx s_17_11 -> bv
        let s_17_13: Bits = Bits::new(s_17_11 as u128, 32u16);
        // D s_17_14: bits-cast sx s_17_13 -> bv length s_17_12
        let s_17_14: Bits = s_17_13.sign_extend(s_17_12);
        // D s_17_15: cast reint s_17_14 -> u64
        let s_17_15: u64 = (s_17_14.value() as u64);
        // C s_17_16: const #() : ()
        let s_17_16: () = ();
        // S s_17_17: call PhysicalCountInt(s_17_16)
        let s_17_17: u64 = PhysicalCountInt(state, tracer, s_17_16);
        // D s_17_18: cast zx s_17_15 -> bv
        let s_17_18: Bits = Bits::new(s_17_15 as u128, 64u16);
        // S s_17_19: cast zx s_17_17 -> bv
        let s_17_19: Bits = Bits::new(s_17_17 as u128, 64u16);
        // D s_17_20: add s_17_18 s_17_19
        let s_17_20: Bits = (s_17_18 + s_17_19);
        // D s_17_21: cast reint s_17_20 -> u64
        let s_17_21: u64 = (s_17_20.value() as u64);
        // D s_17_22: call Mk_CNTHPS_CVAL_EL2_Type(s_17_21)
        let s_17_22: ProductType5c790c8ef59cc8b2 = Mk_CNTHPS_CVAL_EL2_Type(
            state,
            tracer,
            s_17_21,
        );
        // C s_17_23: const #22672u : u32
        let s_17_23: u32 = 22672;
        // N s_17_24: write-reg s_17_23 <= s_17_22
        let s_17_24: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_17_23 as isize, s_17_22);
            tracer.write_register(s_17_23 as isize, s_17_22);
        };
        // N s_17_25: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #117u : u32
        let s_18_0: u32 = 117;
        // S s_18_1: call IsFeatureImplemented(s_18_0)
        let s_18_1: bool = IsFeatureImplemented(state, tracer, s_18_0);
        // D s_18_2: write-var gs#81875 <= s_18_1
        fn_state.gs_81875 = s_18_1;
        // N s_18_3: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #90704u : u32
        let s_19_0: u32 = 90704;
        // D s_19_1: read-reg s_19_0:struct
        let s_19_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_19_0 as isize);
            tracer.read_register(s_19_0 as isize, value);
            value
        };
        // D s_19_2: call _get_SCR_EL3_Type_NS(s_19_1)
        let s_19_2: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_19_1);
        // D s_19_3: cast zx s_19_2 -> bv
        let s_19_3: Bits = Bits::new(s_19_2 as u128, 1u16);
        // C s_19_4: const #0u : u8
        let s_19_4: bool = false;
        // C s_19_5: cast zx s_19_4 -> bv
        let s_19_5: Bits = Bits::new(s_19_4 as u128, 1u16);
        // D s_19_6: cmp-eq s_19_3 s_19_5
        let s_19_6: bool = ((s_19_3) == (s_19_5));
        // D s_19_7: write-var gs#81874 <= s_19_6
        fn_state.gs_81874 = s_19_6;
        // N s_19_8: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #() : ()
        let s_20_0: () = ();
        // S s_20_1: call EL2Enabled(s_20_0)
        let s_20_1: bool = EL2Enabled(state, tracer, s_20_0);
        // N s_20_2: branch s_20_1 b47 b21
        if s_20_1 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #0u : u8
        let s_21_0: bool = false;
        // D s_21_1: write-var gs#81889 <= s_21_0
        fn_state.gs_81889 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#81889:u8
        let s_22_0: bool = fn_state.gs_81889;
        // N s_22_1: branch s_22_0 b46 b23
        if s_22_0 {
            return block_46(state, tracer, fn_state);
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
        // D s_23_1: write-var gs#81890 <= s_23_0
        fn_state.gs_81890 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#81890:u8
        let s_24_0: bool = fn_state.gs_81890;
        // N s_24_1: branch s_24_0 b45 b25
        if s_24_0 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #() : ()
        let s_25_0: () = ();
        // S s_25_1: call EL2Enabled(s_25_0)
        let s_25_1: bool = EL2Enabled(state, tracer, s_25_0);
        // N s_25_2: branch s_25_1 b44 b26
        if s_25_1 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #0u : u8
        let s_26_0: bool = false;
        // D s_26_1: write-var gs#81891 <= s_26_0
        fn_state.gs_81891 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#81891:u8
        let s_27_0: bool = fn_state.gs_81891;
        // N s_27_1: branch s_27_0 b43 b28
        if s_27_0 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #0u : u8
        let s_28_0: bool = false;
        // D s_28_1: write-var gs#81892 <= s_28_0
        fn_state.gs_81892 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#81892:u8
        let s_29_0: bool = fn_state.gs_81892;
        // N s_29_1: branch s_29_0 b42 b30
        if s_29_0 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #145u : u32
        let s_30_0: u32 = 145;
        // S s_30_1: call IsFeatureImplemented(s_30_0)
        let s_30_1: bool = IsFeatureImplemented(state, tracer, s_30_0);
        // N s_30_2: branch s_30_1 b41 b31
        if s_30_1 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #0u : u8
        let s_31_0: bool = false;
        // D s_31_1: write-var gs#81893 <= s_31_0
        fn_state.gs_81893 = s_31_0;
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#81893:u8
        let s_32_0: bool = fn_state.gs_81893;
        // N s_32_1: branch s_32_0 b40 b33
        if s_32_0 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #0u : u8
        let s_33_0: bool = false;
        // D s_33_1: write-var gs#81894 <= s_33_0
        fn_state.gs_81894 = s_33_0;
        // N s_33_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var gs#81894:u8
        let s_34_0: bool = fn_state.gs_81894;
        // N s_34_1: branch s_34_0 b39 b35
        if s_34_0 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #0u : u8
        let s_35_0: bool = false;
        // D s_35_1: write-var gs#81895 <= s_35_0
        fn_state.gs_81895 = s_35_0;
        // N s_35_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var gs#81895:u8
        let s_36_0: bool = fn_state.gs_81895;
        // N s_36_1: branch s_36_0 b38 b37
        if s_36_0 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #64s : i64
        let s_37_0: i64 = 64;
        // D s_37_1: read-var t:i
        let s_37_1: i128 = fn_state.t;
        // D s_37_2: call X_read(s_37_1, s_37_0)
        let s_37_2: Bits = X_read(state, tracer, s_37_1, s_37_0);
        // D s_37_3: cast reint s_37_2 -> u64
        let s_37_3: u64 = (s_37_2.value() as u64);
        // C s_37_4: const #0s : i
        let s_37_4: i128 = 0;
        // D s_37_5: cast zx s_37_3 -> bv
        let s_37_5: Bits = Bits::new(s_37_3 as u128, 64u16);
        // C s_37_6: const #1s : i64
        let s_37_6: i64 = 1;
        // C s_37_7: cast zx s_37_6 -> i
        let s_37_7: i128 = (i128::try_from(s_37_6).unwrap());
        // C s_37_8: const #31s : i
        let s_37_8: i128 = 31;
        // C s_37_9: add s_37_8 s_37_7
        let s_37_9: i128 = (s_37_8 + s_37_7);
        // D s_37_10: bit-extract s_37_5 s_37_4 s_37_9
        let s_37_10: Bits = (Bits::new(
            ((s_37_5) >> (s_37_4)).value(),
            u16::try_from(s_37_9).unwrap(),
        ));
        // D s_37_11: cast reint s_37_10 -> u32
        let s_37_11: u32 = (s_37_10.value() as u32);
        // C s_37_12: const #64s : i
        let s_37_12: i128 = 64;
        // D s_37_13: cast zx s_37_11 -> bv
        let s_37_13: Bits = Bits::new(s_37_11 as u128, 32u16);
        // D s_37_14: bits-cast sx s_37_13 -> bv length s_37_12
        let s_37_14: Bits = s_37_13.sign_extend(s_37_12);
        // D s_37_15: cast reint s_37_14 -> u64
        let s_37_15: u64 = (s_37_14.value() as u64);
        // C s_37_16: const #() : ()
        let s_37_16: () = ();
        // S s_37_17: call PhysicalCountInt(s_37_16)
        let s_37_17: u64 = PhysicalCountInt(state, tracer, s_37_16);
        // D s_37_18: cast zx s_37_15 -> bv
        let s_37_18: Bits = Bits::new(s_37_15 as u128, 64u16);
        // S s_37_19: cast zx s_37_17 -> bv
        let s_37_19: Bits = Bits::new(s_37_17 as u128, 64u16);
        // D s_37_20: add s_37_18 s_37_19
        let s_37_20: Bits = (s_37_18 + s_37_19);
        // D s_37_21: cast reint s_37_20 -> u64
        let s_37_21: u64 = (s_37_20.value() as u64);
        // D s_37_22: call Mk_CNTP_CVAL_EL0_Type(s_37_21)
        let s_37_22: ProductType5c790c8ef59cc8b2 = Mk_CNTP_CVAL_EL0_Type(
            state,
            tracer,
            s_37_21,
        );
        // C s_37_23: const #20800u : u32
        let s_37_23: u32 = 20800;
        // N s_37_24: write-reg s_37_23 <= s_37_22
        let s_37_24: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_37_23 as isize, s_37_22);
            tracer.write_register(s_37_23 as isize, s_37_22);
        };
        // N s_37_25: return
        return;
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #64s : i64
        let s_38_0: i64 = 64;
        // D s_38_1: read-var t:i
        let s_38_1: i128 = fn_state.t;
        // D s_38_2: call X_read(s_38_1, s_38_0)
        let s_38_2: Bits = X_read(state, tracer, s_38_1, s_38_0);
        // D s_38_3: cast reint s_38_2 -> u64
        let s_38_3: u64 = (s_38_2.value() as u64);
        // C s_38_4: const #0s : i
        let s_38_4: i128 = 0;
        // D s_38_5: cast zx s_38_3 -> bv
        let s_38_5: Bits = Bits::new(s_38_3 as u128, 64u16);
        // C s_38_6: const #1s : i64
        let s_38_6: i64 = 1;
        // C s_38_7: cast zx s_38_6 -> i
        let s_38_7: i128 = (i128::try_from(s_38_6).unwrap());
        // C s_38_8: const #31s : i
        let s_38_8: i128 = 31;
        // C s_38_9: add s_38_8 s_38_7
        let s_38_9: i128 = (s_38_8 + s_38_7);
        // D s_38_10: bit-extract s_38_5 s_38_4 s_38_9
        let s_38_10: Bits = (Bits::new(
            ((s_38_5) >> (s_38_4)).value(),
            u16::try_from(s_38_9).unwrap(),
        ));
        // D s_38_11: cast reint s_38_10 -> u32
        let s_38_11: u32 = (s_38_10.value() as u32);
        // C s_38_12: const #64s : i
        let s_38_12: i128 = 64;
        // D s_38_13: cast zx s_38_11 -> bv
        let s_38_13: Bits = Bits::new(s_38_11 as u128, 32u16);
        // D s_38_14: bits-cast sx s_38_13 -> bv length s_38_12
        let s_38_14: Bits = s_38_13.sign_extend(s_38_12);
        // D s_38_15: cast reint s_38_14 -> u64
        let s_38_15: u64 = (s_38_14.value() as u64);
        // C s_38_16: const #() : ()
        let s_38_16: () = ();
        // S s_38_17: call PhysicalCountInt(s_38_16)
        let s_38_17: u64 = PhysicalCountInt(state, tracer, s_38_16);
        // D s_38_18: cast zx s_38_15 -> bv
        let s_38_18: Bits = Bits::new(s_38_15 as u128, 64u16);
        // S s_38_19: cast zx s_38_17 -> bv
        let s_38_19: Bits = Bits::new(s_38_17 as u128, 64u16);
        // D s_38_20: add s_38_18 s_38_19
        let s_38_20: Bits = (s_38_18 + s_38_19);
        // D s_38_21: cast reint s_38_20 -> u64
        let s_38_21: u64 = (s_38_20.value() as u64);
        // D s_38_22: cast zx s_38_21 -> bv
        let s_38_22: Bits = Bits::new(s_38_21 as u128, 64u16);
        // C s_38_23: const #14584u : u32
        let s_38_23: u32 = 14584;
        // D s_38_24: read-reg s_38_23:u64
        let s_38_24: u64 = {
            let value = state.read_register::<u64>(s_38_23 as isize);
            tracer.read_register(s_38_23 as isize, value);
            value
        };
        // D s_38_25: cast zx s_38_24 -> bv
        let s_38_25: Bits = Bits::new(s_38_24 as u128, 64u16);
        // D s_38_26: sub s_38_22 s_38_25
        let s_38_26: Bits = ((s_38_22) - (s_38_25));
        // D s_38_27: cast reint s_38_26 -> u64
        let s_38_27: u64 = (s_38_26.value() as u64);
        // D s_38_28: call Mk_CNTP_CVAL_EL0_Type(s_38_27)
        let s_38_28: ProductType5c790c8ef59cc8b2 = Mk_CNTP_CVAL_EL0_Type(
            state,
            tracer,
            s_38_27,
        );
        // C s_38_29: const #20800u : u32
        let s_38_29: u32 = 20800;
        // N s_38_30: write-reg s_38_29 <= s_38_28
        let s_38_30: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_38_29 as isize, s_38_28);
            tracer.write_register(s_38_29 as isize, s_38_28);
        };
        // N s_38_31: return
        return;
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var __CNTHCTL_EL2_ECV:u8
        let s_39_0: bool = fn_state.u__CNTHCTL_EL2_ECV;
        // D s_39_1: cast zx s_39_0 -> bv
        let s_39_1: Bits = Bits::new(s_39_0 as u128, 1u16);
        // C s_39_2: const #1u : u8
        let s_39_2: bool = true;
        // C s_39_3: cast zx s_39_2 -> bv
        let s_39_3: Bits = Bits::new(s_39_2 as u128, 1u16);
        // D s_39_4: cmp-eq s_39_1 s_39_3
        let s_39_4: bool = ((s_39_1) == (s_39_3));
        // D s_39_5: write-var gs#81895 <= s_39_4
        fn_state.gs_81895 = s_39_4;
        // N s_39_6: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var __SCR_EL3_ECVEn:u8
        let s_40_0: bool = fn_state.u__SCR_EL3_ECVEn;
        // D s_40_1: cast zx s_40_0 -> bv
        let s_40_1: Bits = Bits::new(s_40_0 as u128, 1u16);
        // C s_40_2: const #1u : u8
        let s_40_2: bool = true;
        // C s_40_3: cast zx s_40_2 -> bv
        let s_40_3: Bits = Bits::new(s_40_2 as u128, 1u16);
        // D s_40_4: cmp-eq s_40_1 s_40_3
        let s_40_4: bool = ((s_40_1) == (s_40_3));
        // D s_40_5: write-var gs#81894 <= s_40_4
        fn_state.gs_81894 = s_40_4;
        // N s_40_6: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #() : ()
        let s_41_0: () = ();
        // S s_41_1: call EL2Enabled(s_41_0)
        let s_41_1: bool = EL2Enabled(state, tracer, s_41_0);
        // D s_41_2: write-var gs#81893 <= s_41_1
        fn_state.gs_81893 = s_41_1;
        // N s_41_3: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #24u : u8
        let s_42_0: u8 = 24;
        // C s_42_1: cast zx s_42_0 -> bv
        let s_42_1: Bits = Bits::new(s_42_0 as u128, 8u16);
        // C s_42_2: cast zx s_42_1 -> i
        let s_42_2: i128 = (s_42_1.value() as i128);
        // C s_42_3: cast reint s_42_2 -> i64
        let s_42_3: i64 = (s_42_2 as i64);
        // C s_42_4: cast zx s_42_3 -> i
        let s_42_4: i128 = (i128::try_from(s_42_3).unwrap());
        // C s_42_5: const #432u : u32
        let s_42_5: u32 = 432;
        // D s_42_6: read-reg s_42_5:u8
        let s_42_6: u8 = {
            let value = state.read_register::<u8>(s_42_5 as isize);
            tracer.read_register(s_42_5 as isize, value);
            value
        };
        // D s_42_7: call AArch64_SystemAccessTrap(s_42_6, s_42_4)
        let s_42_7: () = AArch64_SystemAccessTrap(state, tracer, s_42_6, s_42_4);
        // N s_42_8: return
        return;
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var __CNTHCTL_EL2_EL1PTEN:u8
        let s_43_0: bool = fn_state.u__CNTHCTL_EL2_EL1PTEN;
        // D s_43_1: cast zx s_43_0 -> bv
        let s_43_1: Bits = Bits::new(s_43_0 as u128, 1u16);
        // C s_43_2: const #0u : u8
        let s_43_2: bool = false;
        // C s_43_3: cast zx s_43_2 -> bv
        let s_43_3: Bits = Bits::new(s_43_2 as u128, 1u16);
        // D s_43_4: cmp-eq s_43_1 s_43_3
        let s_43_4: bool = ((s_43_1) == (s_43_3));
        // D s_43_5: write-var gs#81892 <= s_43_4
        fn_state.gs_81892 = s_43_4;
        // N s_43_6: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #102552u : u32
        let s_44_0: u32 = 102552;
        // D s_44_1: read-reg s_44_0:struct
        let s_44_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_44_0 as isize);
            tracer.read_register(s_44_0 as isize, value);
            value
        };
        // D s_44_2: call _get_HCR_EL2_Type_E2H(s_44_1)
        let s_44_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_44_1);
        // D s_44_3: cast zx s_44_2 -> bv
        let s_44_3: Bits = Bits::new(s_44_2 as u128, 1u16);
        // C s_44_4: const #1u : u8
        let s_44_4: bool = true;
        // C s_44_5: cast zx s_44_4 -> bv
        let s_44_5: Bits = Bits::new(s_44_4 as u128, 1u16);
        // D s_44_6: cmp-eq s_44_3 s_44_5
        let s_44_6: bool = ((s_44_3) == (s_44_5));
        // D s_44_7: write-var gs#81891 <= s_44_6
        fn_state.gs_81891 = s_44_6;
        // N s_44_8: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #24u : u8
        let s_45_0: u8 = 24;
        // C s_45_1: cast zx s_45_0 -> bv
        let s_45_1: Bits = Bits::new(s_45_0 as u128, 8u16);
        // C s_45_2: cast zx s_45_1 -> i
        let s_45_2: i128 = (s_45_1.value() as i128);
        // C s_45_3: cast reint s_45_2 -> i64
        let s_45_3: i64 = (s_45_2 as i64);
        // C s_45_4: cast zx s_45_3 -> i
        let s_45_4: i128 = (i128::try_from(s_45_3).unwrap());
        // C s_45_5: const #432u : u32
        let s_45_5: u32 = 432;
        // D s_45_6: read-reg s_45_5:u8
        let s_45_6: u8 = {
            let value = state.read_register::<u8>(s_45_5 as isize);
            tracer.read_register(s_45_5 as isize, value);
            value
        };
        // D s_45_7: call AArch64_SystemAccessTrap(s_45_6, s_45_4)
        let s_45_7: () = AArch64_SystemAccessTrap(state, tracer, s_45_6, s_45_4);
        // N s_45_8: return
        return;
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var __CNTHCTL_EL2_EL1PCEN:u8
        let s_46_0: bool = fn_state.u__CNTHCTL_EL2_EL1PCEN;
        // D s_46_1: cast zx s_46_0 -> bv
        let s_46_1: Bits = Bits::new(s_46_0 as u128, 1u16);
        // C s_46_2: const #0u : u8
        let s_46_2: bool = false;
        // C s_46_3: cast zx s_46_2 -> bv
        let s_46_3: Bits = Bits::new(s_46_2 as u128, 1u16);
        // D s_46_4: cmp-eq s_46_1 s_46_3
        let s_46_4: bool = ((s_46_1) == (s_46_3));
        // D s_46_5: write-var gs#81890 <= s_46_4
        fn_state.gs_81890 = s_46_4;
        // N s_46_6: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #102552u : u32
        let s_47_0: u32 = 102552;
        // D s_47_1: read-reg s_47_0:struct
        let s_47_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_47_0 as isize);
            tracer.read_register(s_47_0 as isize, value);
            value
        };
        // D s_47_2: call _get_HCR_EL2_Type_E2H(s_47_1)
        let s_47_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_47_1);
        // D s_47_3: cast zx s_47_2 -> bv
        let s_47_3: Bits = Bits::new(s_47_2 as u128, 1u16);
        // C s_47_4: const #0u : u8
        let s_47_4: bool = false;
        // C s_47_5: cast zx s_47_4 -> bv
        let s_47_5: Bits = Bits::new(s_47_4 as u128, 1u16);
        // D s_47_6: cmp-eq s_47_3 s_47_5
        let s_47_6: bool = ((s_47_3) == (s_47_5));
        // D s_47_7: write-var gs#81889 <= s_47_6
        fn_state.gs_81889 = s_47_6;
        // N s_47_8: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #() : ()
        let s_48_0: () = ();
        // S s_48_1: call EL2Enabled(s_48_0)
        let s_48_1: bool = EL2Enabled(state, tracer, s_48_0);
        // N s_48_2: branch s_48_1 b118 b49
        if s_48_1 {
            return block_118(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #0u : u8
        let s_49_0: bool = false;
        // D s_49_1: write-var gs#81904 <= s_49_0
        fn_state.gs_81904 = s_49_0;
        // N s_49_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var gs#81904:u8
        let s_50_0: bool = fn_state.gs_81904;
        // D s_50_1: not s_50_0
        let s_50_1: bool = !s_50_0;
        // N s_50_2: branch s_50_1 b117 b51
        if s_50_1 {
            return block_117(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #0u : u8
        let s_51_0: bool = false;
        // D s_51_1: write-var gs#81905 <= s_51_0
        fn_state.gs_81905 = s_51_0;
        // N s_51_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var gs#81905:u8
        let s_52_0: bool = fn_state.gs_81905;
        // N s_52_1: branch s_52_0 b111 b53
        if s_52_0 {
            return block_111(state, tracer, fn_state);
        } else {
            return block_53(state, tracer, fn_state);
        };
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #() : ()
        let s_53_0: () = ();
        // S s_53_1: call EL2Enabled(s_53_0)
        let s_53_1: bool = EL2Enabled(state, tracer, s_53_0);
        // N s_53_2: branch s_53_1 b110 b54
        if s_53_1 {
            return block_110(state, tracer, fn_state);
        } else {
            return block_54(state, tracer, fn_state);
        };
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #0u : u8
        let s_54_0: bool = false;
        // D s_54_1: write-var gs#81906 <= s_54_0
        fn_state.gs_81906 = s_54_0;
        // N s_54_2: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var gs#81906:u8
        let s_55_0: bool = fn_state.gs_81906;
        // N s_55_1: branch s_55_0 b109 b56
        if s_55_0 {
            return block_109(state, tracer, fn_state);
        } else {
            return block_56(state, tracer, fn_state);
        };
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #0u : u8
        let s_56_0: bool = false;
        // D s_56_1: write-var gs#81907 <= s_56_0
        fn_state.gs_81907 = s_56_0;
        // N s_56_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var gs#81907:u8
        let s_57_0: bool = fn_state.gs_81907;
        // N s_57_1: branch s_57_0 b108 b58
        if s_57_0 {
            return block_108(state, tracer, fn_state);
        } else {
            return block_58(state, tracer, fn_state);
        };
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #() : ()
        let s_58_0: () = ();
        // S s_58_1: call EL2Enabled(s_58_0)
        let s_58_1: bool = EL2Enabled(state, tracer, s_58_0);
        // N s_58_2: branch s_58_1 b107 b59
        if s_58_1 {
            return block_107(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #0u : u8
        let s_59_0: bool = false;
        // D s_59_1: write-var gs#81908 <= s_59_0
        fn_state.gs_81908 = s_59_0;
        // N s_59_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var gs#81908:u8
        let s_60_0: bool = fn_state.gs_81908;
        // N s_60_1: branch s_60_0 b106 b61
        if s_60_0 {
            return block_106(state, tracer, fn_state);
        } else {
            return block_61(state, tracer, fn_state);
        };
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #0u : u8
        let s_61_0: bool = false;
        // D s_61_1: write-var gs#81909 <= s_61_0
        fn_state.gs_81909 = s_61_0;
        // N s_61_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var gs#81909:u8
        let s_62_0: bool = fn_state.gs_81909;
        // N s_62_1: branch s_62_0 b105 b63
        if s_62_0 {
            return block_105(state, tracer, fn_state);
        } else {
            return block_63(state, tracer, fn_state);
        };
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #() : ()
        let s_63_0: () = ();
        // S s_63_1: call EL2Enabled(s_63_0)
        let s_63_1: bool = EL2Enabled(state, tracer, s_63_0);
        // N s_63_2: branch s_63_1 b104 b64
        if s_63_1 {
            return block_104(state, tracer, fn_state);
        } else {
            return block_64(state, tracer, fn_state);
        };
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #0u : u8
        let s_64_0: bool = false;
        // D s_64_1: write-var gs#81910 <= s_64_0
        fn_state.gs_81910 = s_64_0;
        // N s_64_2: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_65_0: read-var gs#81910:u8
        let s_65_0: bool = fn_state.gs_81910;
        // N s_65_1: branch s_65_0 b103 b66
        if s_65_0 {
            return block_103(state, tracer, fn_state);
        } else {
            return block_66(state, tracer, fn_state);
        };
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #0u : u8
        let s_66_0: bool = false;
        // D s_66_1: write-var gs#81911 <= s_66_0
        fn_state.gs_81911 = s_66_0;
        // N s_66_2: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var gs#81911:u8
        let s_67_0: bool = fn_state.gs_81911;
        // N s_67_1: branch s_67_0 b102 b68
        if s_67_0 {
            return block_102(state, tracer, fn_state);
        } else {
            return block_68(state, tracer, fn_state);
        };
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #() : ()
        let s_68_0: () = ();
        // S s_68_1: call EL2Enabled(s_68_0)
        let s_68_1: bool = EL2Enabled(state, tracer, s_68_0);
        // N s_68_2: branch s_68_1 b101 b69
        if s_68_1 {
            return block_101(state, tracer, fn_state);
        } else {
            return block_69(state, tracer, fn_state);
        };
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #0u : u8
        let s_69_0: bool = false;
        // D s_69_1: write-var gs#81912 <= s_69_0
        fn_state.gs_81912 = s_69_0;
        // N s_69_2: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_70_0: read-var gs#81912:u8
        let s_70_0: bool = fn_state.gs_81912;
        // N s_70_1: branch s_70_0 b100 b71
        if s_70_0 {
            return block_100(state, tracer, fn_state);
        } else {
            return block_71(state, tracer, fn_state);
        };
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #0u : u8
        let s_71_0: bool = false;
        // D s_71_1: write-var gs#81913 <= s_71_0
        fn_state.gs_81913 = s_71_0;
        // N s_71_2: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_72_0: read-var gs#81913:u8
        let s_72_0: bool = fn_state.gs_81913;
        // N s_72_1: branch s_72_0 b99 b73
        if s_72_0 {
            return block_99(state, tracer, fn_state);
        } else {
            return block_73(state, tracer, fn_state);
        };
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #0u : u8
        let s_73_0: bool = false;
        // D s_73_1: write-var gs#81914 <= s_73_0
        fn_state.gs_81914 = s_73_0;
        // N s_73_2: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_74_0: read-var gs#81914:u8
        let s_74_0: bool = fn_state.gs_81914;
        // N s_74_1: branch s_74_0 b98 b75
        if s_74_0 {
            return block_98(state, tracer, fn_state);
        } else {
            return block_75(state, tracer, fn_state);
        };
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #() : ()
        let s_75_0: () = ();
        // S s_75_1: call EL2Enabled(s_75_0)
        let s_75_1: bool = EL2Enabled(state, tracer, s_75_0);
        // N s_75_2: branch s_75_1 b97 b76
        if s_75_1 {
            return block_97(state, tracer, fn_state);
        } else {
            return block_76(state, tracer, fn_state);
        };
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #0u : u8
        let s_76_0: bool = false;
        // D s_76_1: write-var gs#81915 <= s_76_0
        fn_state.gs_81915 = s_76_0;
        // N s_76_2: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_77_0: read-var gs#81915:u8
        let s_77_0: bool = fn_state.gs_81915;
        // N s_77_1: branch s_77_0 b96 b78
        if s_77_0 {
            return block_96(state, tracer, fn_state);
        } else {
            return block_78(state, tracer, fn_state);
        };
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_78_0: const #0u : u8
        let s_78_0: bool = false;
        // D s_78_1: write-var gs#81916 <= s_78_0
        fn_state.gs_81916 = s_78_0;
        // N s_78_2: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_79_0: read-var gs#81916:u8
        let s_79_0: bool = fn_state.gs_81916;
        // N s_79_1: branch s_79_0 b95 b80
        if s_79_0 {
            return block_95(state, tracer, fn_state);
        } else {
            return block_80(state, tracer, fn_state);
        };
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #145u : u32
        let s_80_0: u32 = 145;
        // S s_80_1: call IsFeatureImplemented(s_80_0)
        let s_80_1: bool = IsFeatureImplemented(state, tracer, s_80_0);
        // N s_80_2: branch s_80_1 b94 b81
        if s_80_1 {
            return block_94(state, tracer, fn_state);
        } else {
            return block_81(state, tracer, fn_state);
        };
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #0u : u8
        let s_81_0: bool = false;
        // D s_81_1: write-var gs#81917 <= s_81_0
        fn_state.gs_81917 = s_81_0;
        // N s_81_2: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_82_0: read-var gs#81917:u8
        let s_82_0: bool = fn_state.gs_81917;
        // N s_82_1: branch s_82_0 b93 b83
        if s_82_0 {
            return block_93(state, tracer, fn_state);
        } else {
            return block_83(state, tracer, fn_state);
        };
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #0u : u8
        let s_83_0: bool = false;
        // D s_83_1: write-var gs#81918 <= s_83_0
        fn_state.gs_81918 = s_83_0;
        // N s_83_2: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_84_0: read-var gs#81918:u8
        let s_84_0: bool = fn_state.gs_81918;
        // N s_84_1: branch s_84_0 b92 b85
        if s_84_0 {
            return block_92(state, tracer, fn_state);
        } else {
            return block_85(state, tracer, fn_state);
        };
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_85_0: const #0u : u8
        let s_85_0: bool = false;
        // D s_85_1: write-var gs#81919 <= s_85_0
        fn_state.gs_81919 = s_85_0;
        // N s_85_2: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_86_0: read-var gs#81919:u8
        let s_86_0: bool = fn_state.gs_81919;
        // N s_86_1: branch s_86_0 b91 b87
        if s_86_0 {
            return block_91(state, tracer, fn_state);
        } else {
            return block_87(state, tracer, fn_state);
        };
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_87_0: const #0u : u8
        let s_87_0: bool = false;
        // D s_87_1: write-var gs#81920 <= s_87_0
        fn_state.gs_81920 = s_87_0;
        // N s_87_2: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_88_0: read-var gs#81920:u8
        let s_88_0: bool = fn_state.gs_81920;
        // N s_88_1: branch s_88_0 b90 b89
        if s_88_0 {
            return block_90(state, tracer, fn_state);
        } else {
            return block_89(state, tracer, fn_state);
        };
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_89_0: const #64s : i64
        let s_89_0: i64 = 64;
        // D s_89_1: read-var t:i
        let s_89_1: i128 = fn_state.t;
        // D s_89_2: call X_read(s_89_1, s_89_0)
        let s_89_2: Bits = X_read(state, tracer, s_89_1, s_89_0);
        // D s_89_3: cast reint s_89_2 -> u64
        let s_89_3: u64 = (s_89_2.value() as u64);
        // C s_89_4: const #0s : i
        let s_89_4: i128 = 0;
        // D s_89_5: cast zx s_89_3 -> bv
        let s_89_5: Bits = Bits::new(s_89_3 as u128, 64u16);
        // C s_89_6: const #1s : i64
        let s_89_6: i64 = 1;
        // C s_89_7: cast zx s_89_6 -> i
        let s_89_7: i128 = (i128::try_from(s_89_6).unwrap());
        // C s_89_8: const #31s : i
        let s_89_8: i128 = 31;
        // C s_89_9: add s_89_8 s_89_7
        let s_89_9: i128 = (s_89_8 + s_89_7);
        // D s_89_10: bit-extract s_89_5 s_89_4 s_89_9
        let s_89_10: Bits = (Bits::new(
            ((s_89_5) >> (s_89_4)).value(),
            u16::try_from(s_89_9).unwrap(),
        ));
        // D s_89_11: cast reint s_89_10 -> u32
        let s_89_11: u32 = (s_89_10.value() as u32);
        // C s_89_12: const #64s : i
        let s_89_12: i128 = 64;
        // D s_89_13: cast zx s_89_11 -> bv
        let s_89_13: Bits = Bits::new(s_89_11 as u128, 32u16);
        // D s_89_14: bits-cast sx s_89_13 -> bv length s_89_12
        let s_89_14: Bits = s_89_13.sign_extend(s_89_12);
        // D s_89_15: cast reint s_89_14 -> u64
        let s_89_15: u64 = (s_89_14.value() as u64);
        // C s_89_16: const #() : ()
        let s_89_16: () = ();
        // S s_89_17: call PhysicalCountInt(s_89_16)
        let s_89_17: u64 = PhysicalCountInt(state, tracer, s_89_16);
        // D s_89_18: cast zx s_89_15 -> bv
        let s_89_18: Bits = Bits::new(s_89_15 as u128, 64u16);
        // S s_89_19: cast zx s_89_17 -> bv
        let s_89_19: Bits = Bits::new(s_89_17 as u128, 64u16);
        // D s_89_20: add s_89_18 s_89_19
        let s_89_20: Bits = (s_89_18 + s_89_19);
        // D s_89_21: cast reint s_89_20 -> u64
        let s_89_21: u64 = (s_89_20.value() as u64);
        // D s_89_22: call Mk_CNTP_CVAL_EL0_Type(s_89_21)
        let s_89_22: ProductType5c790c8ef59cc8b2 = Mk_CNTP_CVAL_EL0_Type(
            state,
            tracer,
            s_89_21,
        );
        // C s_89_23: const #20800u : u32
        let s_89_23: u32 = 20800;
        // N s_89_24: write-reg s_89_23 <= s_89_22
        let s_89_24: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_89_23 as isize, s_89_22);
            tracer.write_register(s_89_23 as isize, s_89_22);
        };
        // N s_89_25: return
        return;
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_90_0: const #64s : i64
        let s_90_0: i64 = 64;
        // D s_90_1: read-var t:i
        let s_90_1: i128 = fn_state.t;
        // D s_90_2: call X_read(s_90_1, s_90_0)
        let s_90_2: Bits = X_read(state, tracer, s_90_1, s_90_0);
        // D s_90_3: cast reint s_90_2 -> u64
        let s_90_3: u64 = (s_90_2.value() as u64);
        // C s_90_4: const #0s : i
        let s_90_4: i128 = 0;
        // D s_90_5: cast zx s_90_3 -> bv
        let s_90_5: Bits = Bits::new(s_90_3 as u128, 64u16);
        // C s_90_6: const #1s : i64
        let s_90_6: i64 = 1;
        // C s_90_7: cast zx s_90_6 -> i
        let s_90_7: i128 = (i128::try_from(s_90_6).unwrap());
        // C s_90_8: const #31s : i
        let s_90_8: i128 = 31;
        // C s_90_9: add s_90_8 s_90_7
        let s_90_9: i128 = (s_90_8 + s_90_7);
        // D s_90_10: bit-extract s_90_5 s_90_4 s_90_9
        let s_90_10: Bits = (Bits::new(
            ((s_90_5) >> (s_90_4)).value(),
            u16::try_from(s_90_9).unwrap(),
        ));
        // D s_90_11: cast reint s_90_10 -> u32
        let s_90_11: u32 = (s_90_10.value() as u32);
        // C s_90_12: const #64s : i
        let s_90_12: i128 = 64;
        // D s_90_13: cast zx s_90_11 -> bv
        let s_90_13: Bits = Bits::new(s_90_11 as u128, 32u16);
        // D s_90_14: bits-cast sx s_90_13 -> bv length s_90_12
        let s_90_14: Bits = s_90_13.sign_extend(s_90_12);
        // D s_90_15: cast reint s_90_14 -> u64
        let s_90_15: u64 = (s_90_14.value() as u64);
        // C s_90_16: const #() : ()
        let s_90_16: () = ();
        // S s_90_17: call PhysicalCountInt(s_90_16)
        let s_90_17: u64 = PhysicalCountInt(state, tracer, s_90_16);
        // D s_90_18: cast zx s_90_15 -> bv
        let s_90_18: Bits = Bits::new(s_90_15 as u128, 64u16);
        // S s_90_19: cast zx s_90_17 -> bv
        let s_90_19: Bits = Bits::new(s_90_17 as u128, 64u16);
        // D s_90_20: add s_90_18 s_90_19
        let s_90_20: Bits = (s_90_18 + s_90_19);
        // D s_90_21: cast reint s_90_20 -> u64
        let s_90_21: u64 = (s_90_20.value() as u64);
        // D s_90_22: cast zx s_90_21 -> bv
        let s_90_22: Bits = Bits::new(s_90_21 as u128, 64u16);
        // C s_90_23: const #14584u : u32
        let s_90_23: u32 = 14584;
        // D s_90_24: read-reg s_90_23:u64
        let s_90_24: u64 = {
            let value = state.read_register::<u64>(s_90_23 as isize);
            tracer.read_register(s_90_23 as isize, value);
            value
        };
        // D s_90_25: cast zx s_90_24 -> bv
        let s_90_25: Bits = Bits::new(s_90_24 as u128, 64u16);
        // D s_90_26: sub s_90_22 s_90_25
        let s_90_26: Bits = ((s_90_22) - (s_90_25));
        // D s_90_27: cast reint s_90_26 -> u64
        let s_90_27: u64 = (s_90_26.value() as u64);
        // D s_90_28: call Mk_CNTP_CVAL_EL0_Type(s_90_27)
        let s_90_28: ProductType5c790c8ef59cc8b2 = Mk_CNTP_CVAL_EL0_Type(
            state,
            tracer,
            s_90_27,
        );
        // C s_90_29: const #20800u : u32
        let s_90_29: u32 = 20800;
        // N s_90_30: write-reg s_90_29 <= s_90_28
        let s_90_30: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_90_29 as isize, s_90_28);
            tracer.write_register(s_90_29 as isize, s_90_28);
        };
        // N s_90_31: return
        return;
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_91_0: const #102552u : u32
        let s_91_0: u32 = 102552;
        // D s_91_1: read-reg s_91_0:struct
        let s_91_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_91_0 as isize);
            tracer.read_register(s_91_0 as isize, value);
            value
        };
        // D s_91_2: call _get_HCR_EL2_Type_E2H(s_91_1)
        let s_91_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_91_1);
        // C s_91_3: const #102552u : u32
        let s_91_3: u32 = 102552;
        // D s_91_4: read-reg s_91_3:struct
        let s_91_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_91_3 as isize);
            tracer.read_register(s_91_3 as isize, value);
            value
        };
        // D s_91_5: call _get_HCR_EL2_Type_TGE(s_91_4)
        let s_91_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_91_4);
        // D s_91_6: cast zx s_91_2 -> bv
        let s_91_6: Bits = Bits::new(s_91_2 as u128, 1u16);
        // D s_91_7: cast zx s_91_5 -> bv
        let s_91_7: Bits = Bits::new(s_91_5 as u128, 1u16);
        // D s_91_8: cast reint s_91_6 -> u128
        let s_91_8: u128 = (s_91_6.value() as u128);
        // D s_91_9: size-of s_91_6
        let s_91_9: u16 = s_91_6.length();
        // D s_91_10: cast reint s_91_7 -> u128
        let s_91_10: u128 = (s_91_7.value() as u128);
        // D s_91_11: size-of s_91_7
        let s_91_11: u16 = s_91_7.length();
        // D s_91_12: lsl s_91_8 s_91_11
        let s_91_12: u128 = s_91_8 << s_91_11;
        // D s_91_13: or s_91_12 s_91_10
        let s_91_13: u128 = ((s_91_12) | (s_91_10));
        // D s_91_14: add s_91_9 s_91_11
        let s_91_14: u16 = (s_91_9 + s_91_11);
        // D s_91_15: create-bits s_91_13 s_91_14
        let s_91_15: Bits = Bits::new(s_91_13, s_91_14);
        // D s_91_16: cast reint s_91_15 -> u8
        let s_91_16: u8 = (s_91_15.value() as u8);
        // D s_91_17: cast zx s_91_16 -> bv
        let s_91_17: Bits = Bits::new(s_91_16 as u128, 2u16);
        // C s_91_18: const #3u : u8
        let s_91_18: u8 = 3;
        // C s_91_19: cast zx s_91_18 -> bv
        let s_91_19: Bits = Bits::new(s_91_18 as u128, 2u16);
        // D s_91_20: cmp-ne s_91_17 s_91_19
        let s_91_20: bool = ((s_91_17) != (s_91_19));
        // D s_91_21: write-var gs#81920 <= s_91_20
        fn_state.gs_81920 = s_91_20;
        // N s_91_22: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_92_0: read-var __CNTHCTL_EL2_ECV:u8
        let s_92_0: bool = fn_state.u__CNTHCTL_EL2_ECV;
        // D s_92_1: cast zx s_92_0 -> bv
        let s_92_1: Bits = Bits::new(s_92_0 as u128, 1u16);
        // C s_92_2: const #1u : u8
        let s_92_2: bool = true;
        // C s_92_3: cast zx s_92_2 -> bv
        let s_92_3: Bits = Bits::new(s_92_2 as u128, 1u16);
        // D s_92_4: cmp-eq s_92_1 s_92_3
        let s_92_4: bool = ((s_92_1) == (s_92_3));
        // D s_92_5: write-var gs#81919 <= s_92_4
        fn_state.gs_81919 = s_92_4;
        // N s_92_6: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_93_0: read-var __SCR_EL3_ECVEn:u8
        let s_93_0: bool = fn_state.u__SCR_EL3_ECVEn;
        // D s_93_1: cast zx s_93_0 -> bv
        let s_93_1: Bits = Bits::new(s_93_0 as u128, 1u16);
        // C s_93_2: const #1u : u8
        let s_93_2: bool = true;
        // C s_93_3: cast zx s_93_2 -> bv
        let s_93_3: Bits = Bits::new(s_93_2 as u128, 1u16);
        // D s_93_4: cmp-eq s_93_1 s_93_3
        let s_93_4: bool = ((s_93_1) == (s_93_3));
        // D s_93_5: write-var gs#81918 <= s_93_4
        fn_state.gs_81918 = s_93_4;
        // N s_93_6: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_94_0: const #() : ()
        let s_94_0: () = ();
        // S s_94_1: call EL2Enabled(s_94_0)
        let s_94_1: bool = EL2Enabled(state, tracer, s_94_0);
        // D s_94_2: write-var gs#81917 <= s_94_1
        fn_state.gs_81917 = s_94_1;
        // N s_94_3: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_95_0: const #64s : i64
        let s_95_0: i64 = 64;
        // D s_95_1: read-var t:i
        let s_95_1: i128 = fn_state.t;
        // D s_95_2: call X_read(s_95_1, s_95_0)
        let s_95_2: Bits = X_read(state, tracer, s_95_1, s_95_0);
        // D s_95_3: cast reint s_95_2 -> u64
        let s_95_3: u64 = (s_95_2.value() as u64);
        // C s_95_4: const #0s : i
        let s_95_4: i128 = 0;
        // D s_95_5: cast zx s_95_3 -> bv
        let s_95_5: Bits = Bits::new(s_95_3 as u128, 64u16);
        // C s_95_6: const #1s : i64
        let s_95_6: i64 = 1;
        // C s_95_7: cast zx s_95_6 -> i
        let s_95_7: i128 = (i128::try_from(s_95_6).unwrap());
        // C s_95_8: const #31s : i
        let s_95_8: i128 = 31;
        // C s_95_9: add s_95_8 s_95_7
        let s_95_9: i128 = (s_95_8 + s_95_7);
        // D s_95_10: bit-extract s_95_5 s_95_4 s_95_9
        let s_95_10: Bits = (Bits::new(
            ((s_95_5) >> (s_95_4)).value(),
            u16::try_from(s_95_9).unwrap(),
        ));
        // D s_95_11: cast reint s_95_10 -> u32
        let s_95_11: u32 = (s_95_10.value() as u32);
        // C s_95_12: const #64s : i
        let s_95_12: i128 = 64;
        // D s_95_13: cast zx s_95_11 -> bv
        let s_95_13: Bits = Bits::new(s_95_11 as u128, 32u16);
        // D s_95_14: bits-cast sx s_95_13 -> bv length s_95_12
        let s_95_14: Bits = s_95_13.sign_extend(s_95_12);
        // D s_95_15: cast reint s_95_14 -> u64
        let s_95_15: u64 = (s_95_14.value() as u64);
        // C s_95_16: const #() : ()
        let s_95_16: () = ();
        // S s_95_17: call PhysicalCountInt(s_95_16)
        let s_95_17: u64 = PhysicalCountInt(state, tracer, s_95_16);
        // D s_95_18: cast zx s_95_15 -> bv
        let s_95_18: Bits = Bits::new(s_95_15 as u128, 64u16);
        // S s_95_19: cast zx s_95_17 -> bv
        let s_95_19: Bits = Bits::new(s_95_17 as u128, 64u16);
        // D s_95_20: add s_95_18 s_95_19
        let s_95_20: Bits = (s_95_18 + s_95_19);
        // D s_95_21: cast reint s_95_20 -> u64
        let s_95_21: u64 = (s_95_20.value() as u64);
        // D s_95_22: call Mk_CNTHP_CVAL_EL2_Type(s_95_21)
        let s_95_22: ProductType5c790c8ef59cc8b2 = Mk_CNTHP_CVAL_EL2_Type(
            state,
            tracer,
            s_95_21,
        );
        // C s_95_23: const #16640u : u32
        let s_95_23: u32 = 16640;
        // N s_95_24: write-reg s_95_23 <= s_95_22
        let s_95_24: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_95_23 as isize, s_95_22);
            tracer.write_register(s_95_23 as isize, s_95_22);
        };
        // N s_95_25: return
        return;
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_96_0: const #90704u : u32
        let s_96_0: u32 = 90704;
        // D s_96_1: read-reg s_96_0:struct
        let s_96_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_96_0 as isize);
            tracer.read_register(s_96_0 as isize, value);
            value
        };
        // D s_96_2: call _get_SCR_EL3_Type_NS(s_96_1)
        let s_96_2: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_96_1);
        // D s_96_3: cast zx s_96_2 -> bv
        let s_96_3: Bits = Bits::new(s_96_2 as u128, 1u16);
        // C s_96_4: const #1u : u8
        let s_96_4: bool = true;
        // C s_96_5: cast zx s_96_4 -> bv
        let s_96_5: Bits = Bits::new(s_96_4 as u128, 1u16);
        // D s_96_6: cmp-eq s_96_3 s_96_5
        let s_96_6: bool = ((s_96_3) == (s_96_5));
        // D s_96_7: write-var gs#81916 <= s_96_6
        fn_state.gs_81916 = s_96_6;
        // N s_96_8: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_97_0: const #102552u : u32
        let s_97_0: u32 = 102552;
        // D s_97_1: read-reg s_97_0:struct
        let s_97_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_97_0 as isize);
            tracer.read_register(s_97_0 as isize, value);
            value
        };
        // D s_97_2: call _get_HCR_EL2_Type_E2H(s_97_1)
        let s_97_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_97_1);
        // C s_97_3: const #102552u : u32
        let s_97_3: u32 = 102552;
        // D s_97_4: read-reg s_97_3:struct
        let s_97_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_97_3 as isize);
            tracer.read_register(s_97_3 as isize, value);
            value
        };
        // D s_97_5: call _get_HCR_EL2_Type_TGE(s_97_4)
        let s_97_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_97_4);
        // D s_97_6: cast zx s_97_2 -> bv
        let s_97_6: Bits = Bits::new(s_97_2 as u128, 1u16);
        // D s_97_7: cast zx s_97_5 -> bv
        let s_97_7: Bits = Bits::new(s_97_5 as u128, 1u16);
        // D s_97_8: cast reint s_97_6 -> u128
        let s_97_8: u128 = (s_97_6.value() as u128);
        // D s_97_9: size-of s_97_6
        let s_97_9: u16 = s_97_6.length();
        // D s_97_10: cast reint s_97_7 -> u128
        let s_97_10: u128 = (s_97_7.value() as u128);
        // D s_97_11: size-of s_97_7
        let s_97_11: u16 = s_97_7.length();
        // D s_97_12: lsl s_97_8 s_97_11
        let s_97_12: u128 = s_97_8 << s_97_11;
        // D s_97_13: or s_97_12 s_97_10
        let s_97_13: u128 = ((s_97_12) | (s_97_10));
        // D s_97_14: add s_97_9 s_97_11
        let s_97_14: u16 = (s_97_9 + s_97_11);
        // D s_97_15: create-bits s_97_13 s_97_14
        let s_97_15: Bits = Bits::new(s_97_13, s_97_14);
        // D s_97_16: cast reint s_97_15 -> u8
        let s_97_16: u8 = (s_97_15.value() as u8);
        // D s_97_17: cast zx s_97_16 -> bv
        let s_97_17: Bits = Bits::new(s_97_16 as u128, 2u16);
        // C s_97_18: const #3u : u8
        let s_97_18: u8 = 3;
        // C s_97_19: cast zx s_97_18 -> bv
        let s_97_19: Bits = Bits::new(s_97_18 as u128, 2u16);
        // D s_97_20: cmp-eq s_97_17 s_97_19
        let s_97_20: bool = ((s_97_17) == (s_97_19));
        // D s_97_21: write-var gs#81915 <= s_97_20
        fn_state.gs_81915 = s_97_20;
        // N s_97_22: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_98_0: const #64s : i64
        let s_98_0: i64 = 64;
        // D s_98_1: read-var t:i
        let s_98_1: i128 = fn_state.t;
        // D s_98_2: call X_read(s_98_1, s_98_0)
        let s_98_2: Bits = X_read(state, tracer, s_98_1, s_98_0);
        // D s_98_3: cast reint s_98_2 -> u64
        let s_98_3: u64 = (s_98_2.value() as u64);
        // C s_98_4: const #0s : i
        let s_98_4: i128 = 0;
        // D s_98_5: cast zx s_98_3 -> bv
        let s_98_5: Bits = Bits::new(s_98_3 as u128, 64u16);
        // C s_98_6: const #1s : i64
        let s_98_6: i64 = 1;
        // C s_98_7: cast zx s_98_6 -> i
        let s_98_7: i128 = (i128::try_from(s_98_6).unwrap());
        // C s_98_8: const #31s : i
        let s_98_8: i128 = 31;
        // C s_98_9: add s_98_8 s_98_7
        let s_98_9: i128 = (s_98_8 + s_98_7);
        // D s_98_10: bit-extract s_98_5 s_98_4 s_98_9
        let s_98_10: Bits = (Bits::new(
            ((s_98_5) >> (s_98_4)).value(),
            u16::try_from(s_98_9).unwrap(),
        ));
        // D s_98_11: cast reint s_98_10 -> u32
        let s_98_11: u32 = (s_98_10.value() as u32);
        // C s_98_12: const #64s : i
        let s_98_12: i128 = 64;
        // D s_98_13: cast zx s_98_11 -> bv
        let s_98_13: Bits = Bits::new(s_98_11 as u128, 32u16);
        // D s_98_14: bits-cast sx s_98_13 -> bv length s_98_12
        let s_98_14: Bits = s_98_13.sign_extend(s_98_12);
        // D s_98_15: cast reint s_98_14 -> u64
        let s_98_15: u64 = (s_98_14.value() as u64);
        // C s_98_16: const #() : ()
        let s_98_16: () = ();
        // S s_98_17: call PhysicalCountInt(s_98_16)
        let s_98_17: u64 = PhysicalCountInt(state, tracer, s_98_16);
        // D s_98_18: cast zx s_98_15 -> bv
        let s_98_18: Bits = Bits::new(s_98_15 as u128, 64u16);
        // S s_98_19: cast zx s_98_17 -> bv
        let s_98_19: Bits = Bits::new(s_98_17 as u128, 64u16);
        // D s_98_20: add s_98_18 s_98_19
        let s_98_20: Bits = (s_98_18 + s_98_19);
        // D s_98_21: cast reint s_98_20 -> u64
        let s_98_21: u64 = (s_98_20.value() as u64);
        // D s_98_22: call Mk_CNTHPS_CVAL_EL2_Type(s_98_21)
        let s_98_22: ProductType5c790c8ef59cc8b2 = Mk_CNTHPS_CVAL_EL2_Type(
            state,
            tracer,
            s_98_21,
        );
        // C s_98_23: const #22672u : u32
        let s_98_23: u32 = 22672;
        // N s_98_24: write-reg s_98_23 <= s_98_22
        let s_98_24: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_98_23 as isize, s_98_22);
            tracer.write_register(s_98_23 as isize, s_98_22);
        };
        // N s_98_25: return
        return;
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_99_0: const #117u : u32
        let s_99_0: u32 = 117;
        // S s_99_1: call IsFeatureImplemented(s_99_0)
        let s_99_1: bool = IsFeatureImplemented(state, tracer, s_99_0);
        // D s_99_2: write-var gs#81914 <= s_99_1
        fn_state.gs_81914 = s_99_1;
        // N s_99_3: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_100_0: const #90704u : u32
        let s_100_0: u32 = 90704;
        // D s_100_1: read-reg s_100_0:struct
        let s_100_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_100_0 as isize);
            tracer.read_register(s_100_0 as isize, value);
            value
        };
        // D s_100_2: call _get_SCR_EL3_Type_NS(s_100_1)
        let s_100_2: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_100_1);
        // D s_100_3: cast zx s_100_2 -> bv
        let s_100_3: Bits = Bits::new(s_100_2 as u128, 1u16);
        // C s_100_4: const #0u : u8
        let s_100_4: bool = false;
        // C s_100_5: cast zx s_100_4 -> bv
        let s_100_5: Bits = Bits::new(s_100_4 as u128, 1u16);
        // D s_100_6: cmp-eq s_100_3 s_100_5
        let s_100_6: bool = ((s_100_3) == (s_100_5));
        // D s_100_7: write-var gs#81913 <= s_100_6
        fn_state.gs_81913 = s_100_6;
        // N s_100_8: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_101_0: const #102552u : u32
        let s_101_0: u32 = 102552;
        // D s_101_1: read-reg s_101_0:struct
        let s_101_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_101_0 as isize);
            tracer.read_register(s_101_0 as isize, value);
            value
        };
        // D s_101_2: call _get_HCR_EL2_Type_E2H(s_101_1)
        let s_101_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_101_1);
        // C s_101_3: const #102552u : u32
        let s_101_3: u32 = 102552;
        // D s_101_4: read-reg s_101_3:struct
        let s_101_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_101_3 as isize);
            tracer.read_register(s_101_3 as isize, value);
            value
        };
        // D s_101_5: call _get_HCR_EL2_Type_TGE(s_101_4)
        let s_101_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_101_4);
        // D s_101_6: cast zx s_101_2 -> bv
        let s_101_6: Bits = Bits::new(s_101_2 as u128, 1u16);
        // D s_101_7: cast zx s_101_5 -> bv
        let s_101_7: Bits = Bits::new(s_101_5 as u128, 1u16);
        // D s_101_8: cast reint s_101_6 -> u128
        let s_101_8: u128 = (s_101_6.value() as u128);
        // D s_101_9: size-of s_101_6
        let s_101_9: u16 = s_101_6.length();
        // D s_101_10: cast reint s_101_7 -> u128
        let s_101_10: u128 = (s_101_7.value() as u128);
        // D s_101_11: size-of s_101_7
        let s_101_11: u16 = s_101_7.length();
        // D s_101_12: lsl s_101_8 s_101_11
        let s_101_12: u128 = s_101_8 << s_101_11;
        // D s_101_13: or s_101_12 s_101_10
        let s_101_13: u128 = ((s_101_12) | (s_101_10));
        // D s_101_14: add s_101_9 s_101_11
        let s_101_14: u16 = (s_101_9 + s_101_11);
        // D s_101_15: create-bits s_101_13 s_101_14
        let s_101_15: Bits = Bits::new(s_101_13, s_101_14);
        // D s_101_16: cast reint s_101_15 -> u8
        let s_101_16: u8 = (s_101_15.value() as u8);
        // D s_101_17: cast zx s_101_16 -> bv
        let s_101_17: Bits = Bits::new(s_101_16 as u128, 2u16);
        // C s_101_18: const #3u : u8
        let s_101_18: u8 = 3;
        // C s_101_19: cast zx s_101_18 -> bv
        let s_101_19: Bits = Bits::new(s_101_18 as u128, 2u16);
        // D s_101_20: cmp-eq s_101_17 s_101_19
        let s_101_20: bool = ((s_101_17) == (s_101_19));
        // D s_101_21: write-var gs#81912 <= s_101_20
        fn_state.gs_81912 = s_101_20;
        // N s_101_22: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_102_0: const #24u : u8
        let s_102_0: u8 = 24;
        // C s_102_1: cast zx s_102_0 -> bv
        let s_102_1: Bits = Bits::new(s_102_0 as u128, 8u16);
        // C s_102_2: cast zx s_102_1 -> i
        let s_102_2: i128 = (s_102_1.value() as i128);
        // C s_102_3: cast reint s_102_2 -> i64
        let s_102_3: i64 = (s_102_2 as i64);
        // C s_102_4: cast zx s_102_3 -> i
        let s_102_4: i128 = (i128::try_from(s_102_3).unwrap());
        // C s_102_5: const #432u : u32
        let s_102_5: u32 = 432;
        // D s_102_6: read-reg s_102_5:u8
        let s_102_6: u8 = {
            let value = state.read_register::<u8>(s_102_5 as isize);
            tracer.read_register(s_102_5 as isize, value);
            value
        };
        // D s_102_7: call AArch64_SystemAccessTrap(s_102_6, s_102_4)
        let s_102_7: () = AArch64_SystemAccessTrap(state, tracer, s_102_6, s_102_4);
        // N s_102_8: return
        return;
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_103_0: read-var __CNTHCTL_EL2_EL0PTEN:u8
        let s_103_0: bool = fn_state.u__CNTHCTL_EL2_EL0PTEN;
        // D s_103_1: cast zx s_103_0 -> bv
        let s_103_1: Bits = Bits::new(s_103_0 as u128, 1u16);
        // C s_103_2: const #0u : u8
        let s_103_2: bool = false;
        // C s_103_3: cast zx s_103_2 -> bv
        let s_103_3: Bits = Bits::new(s_103_2 as u128, 1u16);
        // D s_103_4: cmp-eq s_103_1 s_103_3
        let s_103_4: bool = ((s_103_1) == (s_103_3));
        // D s_103_5: write-var gs#81911 <= s_103_4
        fn_state.gs_81911 = s_103_4;
        // N s_103_6: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_104_0: const #102552u : u32
        let s_104_0: u32 = 102552;
        // D s_104_1: read-reg s_104_0:struct
        let s_104_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_104_0 as isize);
            tracer.read_register(s_104_0 as isize, value);
            value
        };
        // D s_104_2: call _get_HCR_EL2_Type_E2H(s_104_1)
        let s_104_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_104_1);
        // C s_104_3: const #102552u : u32
        let s_104_3: u32 = 102552;
        // D s_104_4: read-reg s_104_3:struct
        let s_104_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_104_3 as isize);
            tracer.read_register(s_104_3 as isize, value);
            value
        };
        // D s_104_5: call _get_HCR_EL2_Type_TGE(s_104_4)
        let s_104_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_104_4);
        // D s_104_6: cast zx s_104_2 -> bv
        let s_104_6: Bits = Bits::new(s_104_2 as u128, 1u16);
        // D s_104_7: cast zx s_104_5 -> bv
        let s_104_7: Bits = Bits::new(s_104_5 as u128, 1u16);
        // D s_104_8: cast reint s_104_6 -> u128
        let s_104_8: u128 = (s_104_6.value() as u128);
        // D s_104_9: size-of s_104_6
        let s_104_9: u16 = s_104_6.length();
        // D s_104_10: cast reint s_104_7 -> u128
        let s_104_10: u128 = (s_104_7.value() as u128);
        // D s_104_11: size-of s_104_7
        let s_104_11: u16 = s_104_7.length();
        // D s_104_12: lsl s_104_8 s_104_11
        let s_104_12: u128 = s_104_8 << s_104_11;
        // D s_104_13: or s_104_12 s_104_10
        let s_104_13: u128 = ((s_104_12) | (s_104_10));
        // D s_104_14: add s_104_9 s_104_11
        let s_104_14: u16 = (s_104_9 + s_104_11);
        // D s_104_15: create-bits s_104_13 s_104_14
        let s_104_15: Bits = Bits::new(s_104_13, s_104_14);
        // D s_104_16: cast reint s_104_15 -> u8
        let s_104_16: u8 = (s_104_15.value() as u8);
        // D s_104_17: cast zx s_104_16 -> bv
        let s_104_17: Bits = Bits::new(s_104_16 as u128, 2u16);
        // C s_104_18: const #3u : u8
        let s_104_18: u8 = 3;
        // C s_104_19: cast zx s_104_18 -> bv
        let s_104_19: Bits = Bits::new(s_104_18 as u128, 2u16);
        // D s_104_20: cmp-eq s_104_17 s_104_19
        let s_104_20: bool = ((s_104_17) == (s_104_19));
        // D s_104_21: write-var gs#81910 <= s_104_20
        fn_state.gs_81910 = s_104_20;
        // N s_104_22: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_105_0: const #24u : u8
        let s_105_0: u8 = 24;
        // C s_105_1: cast zx s_105_0 -> bv
        let s_105_1: Bits = Bits::new(s_105_0 as u128, 8u16);
        // C s_105_2: cast zx s_105_1 -> i
        let s_105_2: i128 = (s_105_1.value() as i128);
        // C s_105_3: cast reint s_105_2 -> i64
        let s_105_3: i64 = (s_105_2 as i64);
        // C s_105_4: cast zx s_105_3 -> i
        let s_105_4: i128 = (i128::try_from(s_105_3).unwrap());
        // C s_105_5: const #432u : u32
        let s_105_5: u32 = 432;
        // D s_105_6: read-reg s_105_5:u8
        let s_105_6: u8 = {
            let value = state.read_register::<u8>(s_105_5 as isize);
            tracer.read_register(s_105_5 as isize, value);
            value
        };
        // D s_105_7: call AArch64_SystemAccessTrap(s_105_6, s_105_4)
        let s_105_7: () = AArch64_SystemAccessTrap(state, tracer, s_105_6, s_105_4);
        // N s_105_8: return
        return;
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_106_0: read-var __CNTHCTL_EL2_EL1PTEN:u8
        let s_106_0: bool = fn_state.u__CNTHCTL_EL2_EL1PTEN;
        // D s_106_1: cast zx s_106_0 -> bv
        let s_106_1: Bits = Bits::new(s_106_0 as u128, 1u16);
        // C s_106_2: const #0u : u8
        let s_106_2: bool = false;
        // C s_106_3: cast zx s_106_2 -> bv
        let s_106_3: Bits = Bits::new(s_106_2 as u128, 1u16);
        // D s_106_4: cmp-eq s_106_1 s_106_3
        let s_106_4: bool = ((s_106_1) == (s_106_3));
        // D s_106_5: write-var gs#81909 <= s_106_4
        fn_state.gs_81909 = s_106_4;
        // N s_106_6: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_107_0: const #102552u : u32
        let s_107_0: u32 = 102552;
        // D s_107_1: read-reg s_107_0:struct
        let s_107_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_107_0 as isize);
            tracer.read_register(s_107_0 as isize, value);
            value
        };
        // D s_107_2: call _get_HCR_EL2_Type_E2H(s_107_1)
        let s_107_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_107_1);
        // C s_107_3: const #102552u : u32
        let s_107_3: u32 = 102552;
        // D s_107_4: read-reg s_107_3:struct
        let s_107_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_107_3 as isize);
            tracer.read_register(s_107_3 as isize, value);
            value
        };
        // D s_107_5: call _get_HCR_EL2_Type_TGE(s_107_4)
        let s_107_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_107_4);
        // D s_107_6: cast zx s_107_2 -> bv
        let s_107_6: Bits = Bits::new(s_107_2 as u128, 1u16);
        // D s_107_7: cast zx s_107_5 -> bv
        let s_107_7: Bits = Bits::new(s_107_5 as u128, 1u16);
        // D s_107_8: cast reint s_107_6 -> u128
        let s_107_8: u128 = (s_107_6.value() as u128);
        // D s_107_9: size-of s_107_6
        let s_107_9: u16 = s_107_6.length();
        // D s_107_10: cast reint s_107_7 -> u128
        let s_107_10: u128 = (s_107_7.value() as u128);
        // D s_107_11: size-of s_107_7
        let s_107_11: u16 = s_107_7.length();
        // D s_107_12: lsl s_107_8 s_107_11
        let s_107_12: u128 = s_107_8 << s_107_11;
        // D s_107_13: or s_107_12 s_107_10
        let s_107_13: u128 = ((s_107_12) | (s_107_10));
        // D s_107_14: add s_107_9 s_107_11
        let s_107_14: u16 = (s_107_9 + s_107_11);
        // D s_107_15: create-bits s_107_13 s_107_14
        let s_107_15: Bits = Bits::new(s_107_13, s_107_14);
        // D s_107_16: cast reint s_107_15 -> u8
        let s_107_16: u8 = (s_107_15.value() as u8);
        // D s_107_17: cast zx s_107_16 -> bv
        let s_107_17: Bits = Bits::new(s_107_16 as u128, 2u16);
        // C s_107_18: const #2u : u8
        let s_107_18: u8 = 2;
        // C s_107_19: cast zx s_107_18 -> bv
        let s_107_19: Bits = Bits::new(s_107_18 as u128, 2u16);
        // D s_107_20: cmp-eq s_107_17 s_107_19
        let s_107_20: bool = ((s_107_17) == (s_107_19));
        // D s_107_21: write-var gs#81908 <= s_107_20
        fn_state.gs_81908 = s_107_20;
        // N s_107_22: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_108_0: const #24u : u8
        let s_108_0: u8 = 24;
        // C s_108_1: cast zx s_108_0 -> bv
        let s_108_1: Bits = Bits::new(s_108_0 as u128, 8u16);
        // C s_108_2: cast zx s_108_1 -> i
        let s_108_2: i128 = (s_108_1.value() as i128);
        // C s_108_3: cast reint s_108_2 -> i64
        let s_108_3: i64 = (s_108_2 as i64);
        // C s_108_4: cast zx s_108_3 -> i
        let s_108_4: i128 = (i128::try_from(s_108_3).unwrap());
        // C s_108_5: const #432u : u32
        let s_108_5: u32 = 432;
        // D s_108_6: read-reg s_108_5:u8
        let s_108_6: u8 = {
            let value = state.read_register::<u8>(s_108_5 as isize);
            tracer.read_register(s_108_5 as isize, value);
            value
        };
        // D s_108_7: call AArch64_SystemAccessTrap(s_108_6, s_108_4)
        let s_108_7: () = AArch64_SystemAccessTrap(state, tracer, s_108_6, s_108_4);
        // N s_108_8: return
        return;
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_109_0: read-var __CNTHCTL_EL2_EL1PCEN:u8
        let s_109_0: bool = fn_state.u__CNTHCTL_EL2_EL1PCEN;
        // D s_109_1: cast zx s_109_0 -> bv
        let s_109_1: Bits = Bits::new(s_109_0 as u128, 1u16);
        // C s_109_2: const #0u : u8
        let s_109_2: bool = false;
        // C s_109_3: cast zx s_109_2 -> bv
        let s_109_3: Bits = Bits::new(s_109_2 as u128, 1u16);
        // D s_109_4: cmp-eq s_109_1 s_109_3
        let s_109_4: bool = ((s_109_1) == (s_109_3));
        // D s_109_5: write-var gs#81907 <= s_109_4
        fn_state.gs_81907 = s_109_4;
        // N s_109_6: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_110_0: const #102552u : u32
        let s_110_0: u32 = 102552;
        // D s_110_1: read-reg s_110_0:struct
        let s_110_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_110_0 as isize);
            tracer.read_register(s_110_0 as isize, value);
            value
        };
        // D s_110_2: call _get_HCR_EL2_Type_E2H(s_110_1)
        let s_110_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_110_1);
        // D s_110_3: cast zx s_110_2 -> bv
        let s_110_3: Bits = Bits::new(s_110_2 as u128, 1u16);
        // C s_110_4: const #0u : u8
        let s_110_4: bool = false;
        // C s_110_5: cast zx s_110_4 -> bv
        let s_110_5: Bits = Bits::new(s_110_4 as u128, 1u16);
        // D s_110_6: cmp-eq s_110_3 s_110_5
        let s_110_6: bool = ((s_110_3) == (s_110_5));
        // D s_110_7: write-var gs#81906 <= s_110_6
        fn_state.gs_81906 = s_110_6;
        // N s_110_8: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_111_0: const #() : ()
        let s_111_0: () = ();
        // S s_111_1: call EL2Enabled(s_111_0)
        let s_111_1: bool = EL2Enabled(state, tracer, s_111_0);
        // N s_111_2: branch s_111_1 b116 b112
        if s_111_1 {
            return block_116(state, tracer, fn_state);
        } else {
            return block_112(state, tracer, fn_state);
        };
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_112_0: const #0u : u8
        let s_112_0: bool = false;
        // D s_112_1: write-var gs#81937 <= s_112_0
        fn_state.gs_81937 = s_112_0;
        // N s_112_2: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_113_0: read-var gs#81937:u8
        let s_113_0: bool = fn_state.gs_81937;
        // N s_113_1: branch s_113_0 b115 b114
        if s_113_0 {
            return block_115(state, tracer, fn_state);
        } else {
            return block_114(state, tracer, fn_state);
        };
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_114_0: const #24u : u8
        let s_114_0: u8 = 24;
        // C s_114_1: cast zx s_114_0 -> bv
        let s_114_1: Bits = Bits::new(s_114_0 as u128, 8u16);
        // C s_114_2: cast zx s_114_1 -> i
        let s_114_2: i128 = (s_114_1.value() as i128);
        // C s_114_3: cast reint s_114_2 -> i64
        let s_114_3: i64 = (s_114_2 as i64);
        // C s_114_4: cast zx s_114_3 -> i
        let s_114_4: i128 = (i128::try_from(s_114_3).unwrap());
        // C s_114_5: const #440u : u32
        let s_114_5: u32 = 440;
        // D s_114_6: read-reg s_114_5:u8
        let s_114_6: u8 = {
            let value = state.read_register::<u8>(s_114_5 as isize);
            tracer.read_register(s_114_5 as isize, value);
            value
        };
        // D s_114_7: call AArch64_SystemAccessTrap(s_114_6, s_114_4)
        let s_114_7: () = AArch64_SystemAccessTrap(state, tracer, s_114_6, s_114_4);
        // N s_114_8: return
        return;
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_115_0: const #24u : u8
        let s_115_0: u8 = 24;
        // C s_115_1: cast zx s_115_0 -> bv
        let s_115_1: Bits = Bits::new(s_115_0 as u128, 8u16);
        // C s_115_2: cast zx s_115_1 -> i
        let s_115_2: i128 = (s_115_1.value() as i128);
        // C s_115_3: cast reint s_115_2 -> i64
        let s_115_3: i64 = (s_115_2 as i64);
        // C s_115_4: cast zx s_115_3 -> i
        let s_115_4: i128 = (i128::try_from(s_115_3).unwrap());
        // C s_115_5: const #432u : u32
        let s_115_5: u32 = 432;
        // D s_115_6: read-reg s_115_5:u8
        let s_115_6: u8 = {
            let value = state.read_register::<u8>(s_115_5 as isize);
            tracer.read_register(s_115_5 as isize, value);
            value
        };
        // D s_115_7: call AArch64_SystemAccessTrap(s_115_6, s_115_4)
        let s_115_7: () = AArch64_SystemAccessTrap(state, tracer, s_115_6, s_115_4);
        // N s_115_8: return
        return;
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_116_0: read-var __HCR_EL2_TGE:u8
        let s_116_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_116_1: cast zx s_116_0 -> bv
        let s_116_1: Bits = Bits::new(s_116_0 as u128, 1u16);
        // C s_116_2: const #1u : u8
        let s_116_2: bool = true;
        // C s_116_3: cast zx s_116_2 -> bv
        let s_116_3: Bits = Bits::new(s_116_2 as u128, 1u16);
        // D s_116_4: cmp-eq s_116_1 s_116_3
        let s_116_4: bool = ((s_116_1) == (s_116_3));
        // D s_116_5: write-var gs#81937 <= s_116_4
        fn_state.gs_81937 = s_116_4;
        // N s_116_6: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_117_0: read-var __CNTKCTL_EL1_EL0PTEN:u8
        let s_117_0: bool = fn_state.u__CNTKCTL_EL1_EL0PTEN;
        // D s_117_1: cast zx s_117_0 -> bv
        let s_117_1: Bits = Bits::new(s_117_0 as u128, 1u16);
        // C s_117_2: const #0u : u8
        let s_117_2: bool = false;
        // C s_117_3: cast zx s_117_2 -> bv
        let s_117_3: Bits = Bits::new(s_117_2 as u128, 1u16);
        // D s_117_4: cmp-eq s_117_1 s_117_3
        let s_117_4: bool = ((s_117_1) == (s_117_3));
        // D s_117_5: write-var gs#81905 <= s_117_4
        fn_state.gs_81905 = s_117_4;
        // N s_117_6: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_118_0: const #102552u : u32
        let s_118_0: u32 = 102552;
        // D s_118_1: read-reg s_118_0:struct
        let s_118_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_118_0 as isize);
            tracer.read_register(s_118_0 as isize, value);
            value
        };
        // D s_118_2: call _get_HCR_EL2_Type_E2H(s_118_1)
        let s_118_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_118_1);
        // C s_118_3: const #102552u : u32
        let s_118_3: u32 = 102552;
        // D s_118_4: read-reg s_118_3:struct
        let s_118_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_118_3 as isize);
            tracer.read_register(s_118_3 as isize, value);
            value
        };
        // D s_118_5: call _get_HCR_EL2_Type_TGE(s_118_4)
        let s_118_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_118_4);
        // D s_118_6: cast zx s_118_2 -> bv
        let s_118_6: Bits = Bits::new(s_118_2 as u128, 1u16);
        // D s_118_7: cast zx s_118_5 -> bv
        let s_118_7: Bits = Bits::new(s_118_5 as u128, 1u16);
        // D s_118_8: cast reint s_118_6 -> u128
        let s_118_8: u128 = (s_118_6.value() as u128);
        // D s_118_9: size-of s_118_6
        let s_118_9: u16 = s_118_6.length();
        // D s_118_10: cast reint s_118_7 -> u128
        let s_118_10: u128 = (s_118_7.value() as u128);
        // D s_118_11: size-of s_118_7
        let s_118_11: u16 = s_118_7.length();
        // D s_118_12: lsl s_118_8 s_118_11
        let s_118_12: u128 = s_118_8 << s_118_11;
        // D s_118_13: or s_118_12 s_118_10
        let s_118_13: u128 = ((s_118_12) | (s_118_10));
        // D s_118_14: add s_118_9 s_118_11
        let s_118_14: u16 = (s_118_9 + s_118_11);
        // D s_118_15: create-bits s_118_13 s_118_14
        let s_118_15: Bits = Bits::new(s_118_13, s_118_14);
        // D s_118_16: cast reint s_118_15 -> u8
        let s_118_16: u8 = (s_118_15.value() as u8);
        // D s_118_17: cast zx s_118_16 -> bv
        let s_118_17: Bits = Bits::new(s_118_16 as u128, 2u16);
        // C s_118_18: const #3u : u8
        let s_118_18: u8 = 3;
        // C s_118_19: cast zx s_118_18 -> bv
        let s_118_19: Bits = Bits::new(s_118_18 as u128, 2u16);
        // D s_118_20: cmp-eq s_118_17 s_118_19
        let s_118_20: bool = ((s_118_17) == (s_118_19));
        // D s_118_21: write-var gs#81904 <= s_118_20
        fn_state.gs_81904 = s_118_20;
        // N s_118_22: jump b50
        return block_50(state, tracer, fn_state);
    }
}
