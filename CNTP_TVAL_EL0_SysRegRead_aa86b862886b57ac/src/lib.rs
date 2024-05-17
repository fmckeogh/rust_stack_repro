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
use u_get_CNTHCTL_EL2_Type_EL1PTEN::*;
use u_get_CNTP_CTL_EL0_Type_ENABLE::*;
use Mk_CNTP_TVAL_EL0_Type::*;
use IsFeatureImplemented::*;
use AArch64_SystemAccessTrap::*;
use u_get_SCR_EL3_Type_NS::*;
use u__get_CNTP_TVAL_EL0::*;
use X_set::*;
use u__UNKNOWN_bits::*;
use u_get_CNTHCTL_EL2_Type_EL1PCEN::*;
use u_get_CNTHCTL_EL2_Type_EL0PTEN::*;
use u_get_SCR_EL3_Type_ECVEn::*;
use u_get_CNTHP_CTL_EL2_Type_ENABLE::*;
use PhysicalCountInt::*;
use u_get_CNTKCTL_EL1_Type_EL0PTEN::*;
use u_get_HCR_EL2_Type_TGE::*;
use u_get_CNTHPS_CTL_EL2_Type_ENABLE::*;
use common::*;
pub fn CNTP_TVAL_EL0_SysRegRead_aa86b862886b57ac<T: Tracer>(
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
        gs_58090: bool,
        gs_58075: bool,
        ga_56785: ProductType5c790c8ef59cc8b2,
        gs_58080: bool,
        u__CNTHP_CTL_EL2_ENABLE: bool,
        ga_56747: ProductType5c790c8ef59cc8b2,
        gs_58100: bool,
        gs_58092: bool,
        ga_56727: ProductType5c790c8ef59cc8b2,
        u__CNTHCTL_EL2_EL1PCEN: bool,
        gs_58093: bool,
        gs_58097: bool,
        ga_56758: ProductType5c790c8ef59cc8b2,
        gs_58116: bool,
        gs_58101: bool,
        u__HCR_EL2_TGE: bool,
        gs_58078: bool,
        gs_58077: bool,
        u__CNTHCTL_EL2_EL0PTEN: bool,
        gs_58062: bool,
        gs_58099: bool,
        gs_58102: bool,
        gs_58103: bool,
        gs_58087: bool,
        gs_58098: bool,
        ga_56828: ProductType5c790c8ef59cc8b2,
        u__CNTKCTL_EL1_EL0PTEN: bool,
        gs_58091: bool,
        gs_58064: bool,
        ga_56709: ProductType5c790c8ef59cc8b2,
        ga_56796: ProductType5c790c8ef59cc8b2,
        u__SCR_EL3_ECVEn: bool,
        gs_58089: bool,
        u__CNTHPS_CTL_EL2_ENABLE: bool,
        gs_58079: bool,
        u__CNTHCTL_EL2_ECV: bool,
        u__PSTATE_EL: u8,
        gs_58095: bool,
        u__CNTP_CTL_EL0_ENABLE: bool,
        gs_58076: bool,
        ga_56839: ProductType5c790c8ef59cc8b2,
        gs_58096: bool,
        gs_58088: bool,
        gs_58074: bool,
        ga_56851: ProductType5c790c8ef59cc8b2,
        gs_58063: bool,
        ga_56813: ProductType5c790c8ef59cc8b2,
        gs_58094: bool,
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
        // C s_0_23: const #10504u : u32
        let s_0_23: u32 = 10504;
        // D s_0_24: read-reg s_0_23:struct
        let s_0_24: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_23 as isize);
            tracer.read_register(s_0_23 as isize, value);
            value
        };
        // D s_0_25: call _get_CNTHPS_CTL_EL2_Type_ENABLE(s_0_24)
        let s_0_25: bool = u_get_CNTHPS_CTL_EL2_Type_ENABLE(state, tracer, s_0_24);
        // D s_0_26: write-var __CNTHPS_CTL_EL2_ENABLE <= s_0_25
        fn_state.u__CNTHPS_CTL_EL2_ENABLE = s_0_25;
        // C s_0_27: const #100912u : u32
        let s_0_27: u32 = 100912;
        // D s_0_28: read-reg s_0_27:struct
        let s_0_28: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_27 as isize);
            tracer.read_register(s_0_27 as isize, value);
            value
        };
        // D s_0_29: call _get_CNTHP_CTL_EL2_Type_ENABLE(s_0_28)
        let s_0_29: bool = u_get_CNTHP_CTL_EL2_Type_ENABLE(state, tracer, s_0_28);
        // D s_0_30: write-var __CNTHP_CTL_EL2_ENABLE <= s_0_29
        fn_state.u__CNTHP_CTL_EL2_ENABLE = s_0_29;
        // C s_0_31: const #90704u : u32
        let s_0_31: u32 = 90704;
        // D s_0_32: read-reg s_0_31:struct
        let s_0_32: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_31 as isize);
            tracer.read_register(s_0_31 as isize, value);
            value
        };
        // D s_0_33: call _get_SCR_EL3_Type_ECVEn(s_0_32)
        let s_0_33: bool = u_get_SCR_EL3_Type_ECVEn(state, tracer, s_0_32);
        // D s_0_34: write-var __SCR_EL3_ECVEn <= s_0_33
        fn_state.u__SCR_EL3_ECVEn = s_0_33;
        // C s_0_35: const #12808u : u32
        let s_0_35: u32 = 12808;
        // D s_0_36: read-reg s_0_35:struct
        let s_0_36: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_35 as isize);
            tracer.read_register(s_0_35 as isize, value);
            value
        };
        // D s_0_37: call _get_CNTHCTL_EL2_Type_ECV(s_0_36)
        let s_0_37: bool = u_get_CNTHCTL_EL2_Type_ECV(state, tracer, s_0_36);
        // D s_0_38: write-var __CNTHCTL_EL2_ECV <= s_0_37
        fn_state.u__CNTHCTL_EL2_ECV = s_0_37;
        // C s_0_39: const #90832u : u32
        let s_0_39: u32 = 90832;
        // D s_0_40: read-reg s_0_39:struct
        let s_0_40: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_39 as isize);
            tracer.read_register(s_0_39 as isize, value);
            value
        };
        // D s_0_41: call _get_CNTP_CTL_EL0_Type_ENABLE(s_0_40)
        let s_0_41: bool = u_get_CNTP_CTL_EL0_Type_ENABLE(state, tracer, s_0_40);
        // D s_0_42: write-var __CNTP_CTL_EL0_ENABLE <= s_0_41
        fn_state.u__CNTP_CTL_EL0_ENABLE = s_0_41;
        // D s_0_43: read-var __PSTATE_EL:u8
        let s_0_43: u8 = fn_state.u__PSTATE_EL;
        // D s_0_44: cast zx s_0_43 -> bv
        let s_0_44: Bits = Bits::new(s_0_43 as u128, 2u16);
        // C s_0_45: const #448u : u32
        let s_0_45: u32 = 448;
        // D s_0_46: read-reg s_0_45:u8
        let s_0_46: u8 = {
            let value = state.read_register::<u8>(s_0_45 as isize);
            tracer.read_register(s_0_45 as isize, value);
            value
        };
        // D s_0_47: cast zx s_0_46 -> bv
        let s_0_47: Bits = Bits::new(s_0_46 as u128, 2u16);
        // D s_0_48: cmp-eq s_0_44 s_0_47
        let s_0_48: bool = ((s_0_44) == (s_0_47));
        // N s_0_49: branch s_0_48 b60 b1
        if s_0_48 {
            return block_60(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b28 b2
        if s_1_5 {
            return block_28(state, tracer, fn_state);
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
        // D s_5_0: read-var __CNTP_CTL_EL0_ENABLE:u8
        let s_5_0: bool = fn_state.u__CNTP_CTL_EL0_ENABLE;
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
        // C s_6_1: const #20800u : u32
        let s_6_1: u32 = 20800;
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
        // D s_6_9: call Mk_CNTP_TVAL_EL0_Type(s_6_8)
        let s_6_9: ProductType5c790c8ef59cc8b2 = Mk_CNTP_TVAL_EL0_Type(
            state,
            tracer,
            s_6_8,
        );
        // D s_6_10: call __get_CNTP_TVAL_EL0(s_6_9)
        let s_6_10: ProductType5c790c8ef59cc8b2 = u__get_CNTP_TVAL_EL0(
            state,
            tracer,
            s_6_9,
        );
        // D s_6_11: write-var ga#56851 <= s_6_10
        fn_state.ga_56851 = s_6_10;
        // D s_6_12: read-var ga#56851.0:struct
        let s_6_12: u64 = fn_state.ga_56851._0;
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
        // C s_8_0: const #102552u : u32
        let s_8_0: u32 = 102552;
        // D s_8_1: read-reg s_8_0:struct
        let s_8_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_8_0 as isize);
            tracer.read_register(s_8_0 as isize, value);
            value
        };
        // D s_8_2: call _get_HCR_EL2_Type_E2H(s_8_1)
        let s_8_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_8_1);
        // D s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 1u16);
        // C s_8_4: const #1u : u8
        let s_8_4: bool = true;
        // C s_8_5: cast zx s_8_4 -> bv
        let s_8_5: Bits = Bits::new(s_8_4 as u128, 1u16);
        // D s_8_6: cmp-eq s_8_3 s_8_5
        let s_8_6: bool = ((s_8_3) == (s_8_5));
        // N s_8_7: branch s_8_6 b27 b9
        if s_8_6 {
            return block_27(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#58062 <= s_9_0
        fn_state.gs_58062 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#58062:u8
        let s_10_0: bool = fn_state.gs_58062;
        // N s_10_1: branch s_10_0 b26 b11
        if s_10_0 {
            return block_26(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#58063 <= s_11_0
        fn_state.gs_58063 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#58063:u8
        let s_12_0: bool = fn_state.gs_58063;
        // N s_12_1: branch s_12_0 b23 b13
        if s_12_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #102552u : u32
        let s_13_0: u32 = 102552;
        // D s_13_1: read-reg s_13_0:struct
        let s_13_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_13_0 as isize);
            tracer.read_register(s_13_0 as isize, value);
            value
        };
        // D s_13_2: call _get_HCR_EL2_Type_E2H(s_13_1)
        let s_13_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_13_1);
        // D s_13_3: cast zx s_13_2 -> bv
        let s_13_3: Bits = Bits::new(s_13_2 as u128, 1u16);
        // C s_13_4: const #1u : u8
        let s_13_4: bool = true;
        // C s_13_5: cast zx s_13_4 -> bv
        let s_13_5: Bits = Bits::new(s_13_4 as u128, 1u16);
        // D s_13_6: cmp-eq s_13_3 s_13_5
        let s_13_6: bool = ((s_13_3) == (s_13_5));
        // N s_13_7: branch s_13_6 b22 b14
        if s_13_6 {
            return block_22(state, tracer, fn_state);
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
        // D s_14_1: write-var gs#58064 <= s_14_0
        fn_state.gs_58064 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#58064:u8
        let s_15_0: bool = fn_state.gs_58064;
        // N s_15_1: branch s_15_0 b19 b16
        if s_15_0 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var __CNTP_CTL_EL0_ENABLE:u8
        let s_16_0: bool = fn_state.u__CNTP_CTL_EL0_ENABLE;
        // D s_16_1: cast zx s_16_0 -> bv
        let s_16_1: Bits = Bits::new(s_16_0 as u128, 1u16);
        // C s_16_2: const #0u : u8
        let s_16_2: bool = false;
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
        // C s_17_0: const #64s : i64
        let s_17_0: i64 = 64;
        // C s_17_1: const #20800u : u32
        let s_17_1: u32 = 20800;
        // D s_17_2: read-reg s_17_1:u64
        let s_17_2: u64 = {
            let value = state.read_register::<u64>(s_17_1 as isize);
            tracer.read_register(s_17_1 as isize, value);
            value
        };
        // C s_17_3: const #() : ()
        let s_17_3: () = ();
        // S s_17_4: call PhysicalCountInt(s_17_3)
        let s_17_4: u64 = PhysicalCountInt(state, tracer, s_17_3);
        // D s_17_5: cast zx s_17_2 -> bv
        let s_17_5: Bits = Bits::new(s_17_2 as u128, 64u16);
        // S s_17_6: cast zx s_17_4 -> bv
        let s_17_6: Bits = Bits::new(s_17_4 as u128, 64u16);
        // D s_17_7: sub s_17_5 s_17_6
        let s_17_7: Bits = ((s_17_5) - (s_17_6));
        // D s_17_8: cast reint s_17_7 -> u64
        let s_17_8: u64 = (s_17_7.value() as u64);
        // D s_17_9: call Mk_CNTP_TVAL_EL0_Type(s_17_8)
        let s_17_9: ProductType5c790c8ef59cc8b2 = Mk_CNTP_TVAL_EL0_Type(
            state,
            tracer,
            s_17_8,
        );
        // D s_17_10: call __get_CNTP_TVAL_EL0(s_17_9)
        let s_17_10: ProductType5c790c8ef59cc8b2 = u__get_CNTP_TVAL_EL0(
            state,
            tracer,
            s_17_9,
        );
        // D s_17_11: write-var ga#56839 <= s_17_10
        fn_state.ga_56839 = s_17_10;
        // D s_17_12: read-var ga#56839.0:struct
        let s_17_12: u64 = fn_state.ga_56839._0;
        // D s_17_13: cast zx s_17_12 -> bv
        let s_17_13: Bits = Bits::new(s_17_12 as u128, 64u16);
        // D s_17_14: read-var t:i
        let s_17_14: i128 = fn_state.t;
        // D s_17_15: call X_set(s_17_14, s_17_0, s_17_13)
        let s_17_15: () = X_set(state, tracer, s_17_14, s_17_0, s_17_13);
        // N s_17_16: return
        return;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #64s : i64
        let s_18_0: i64 = 64;
        // C s_18_1: const #64s : i64
        let s_18_1: i64 = 64;
        // C s_18_2: cast zx s_18_1 -> i
        let s_18_2: i128 = (i128::try_from(s_18_1).unwrap());
        // S s_18_3: call __UNKNOWN_bits(s_18_2)
        let s_18_3: Bits = u__UNKNOWN_bits(state, tracer, s_18_2);
        // S s_18_4: cast reint s_18_3 -> u64
        let s_18_4: u64 = (s_18_3.value() as u64);
        // S s_18_5: cast zx s_18_4 -> bv
        let s_18_5: Bits = Bits::new(s_18_4 as u128, 64u16);
        // D s_18_6: read-var t:i
        let s_18_6: i128 = fn_state.t;
        // D s_18_7: call X_set(s_18_6, s_18_0, s_18_5)
        let s_18_7: () = X_set(state, tracer, s_18_6, s_18_0, s_18_5);
        // N s_18_8: return
        return;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var __CNTHP_CTL_EL2_ENABLE:u8
        let s_19_0: bool = fn_state.u__CNTHP_CTL_EL2_ENABLE;
        // D s_19_1: cast zx s_19_0 -> bv
        let s_19_1: Bits = Bits::new(s_19_0 as u128, 1u16);
        // C s_19_2: const #0u : u8
        let s_19_2: bool = false;
        // C s_19_3: cast zx s_19_2 -> bv
        let s_19_3: Bits = Bits::new(s_19_2 as u128, 1u16);
        // D s_19_4: cmp-eq s_19_1 s_19_3
        let s_19_4: bool = ((s_19_1) == (s_19_3));
        // N s_19_5: branch s_19_4 b21 b20
        if s_19_4 {
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
        // C s_20_1: const #16640u : u32
        let s_20_1: u32 = 16640;
        // D s_20_2: read-reg s_20_1:u64
        let s_20_2: u64 = {
            let value = state.read_register::<u64>(s_20_1 as isize);
            tracer.read_register(s_20_1 as isize, value);
            value
        };
        // C s_20_3: const #() : ()
        let s_20_3: () = ();
        // S s_20_4: call PhysicalCountInt(s_20_3)
        let s_20_4: u64 = PhysicalCountInt(state, tracer, s_20_3);
        // D s_20_5: cast zx s_20_2 -> bv
        let s_20_5: Bits = Bits::new(s_20_2 as u128, 64u16);
        // S s_20_6: cast zx s_20_4 -> bv
        let s_20_6: Bits = Bits::new(s_20_4 as u128, 64u16);
        // D s_20_7: sub s_20_5 s_20_6
        let s_20_7: Bits = ((s_20_5) - (s_20_6));
        // D s_20_8: cast reint s_20_7 -> u64
        let s_20_8: u64 = (s_20_7.value() as u64);
        // D s_20_9: call Mk_CNTP_TVAL_EL0_Type(s_20_8)
        let s_20_9: ProductType5c790c8ef59cc8b2 = Mk_CNTP_TVAL_EL0_Type(
            state,
            tracer,
            s_20_8,
        );
        // D s_20_10: call __get_CNTP_TVAL_EL0(s_20_9)
        let s_20_10: ProductType5c790c8ef59cc8b2 = u__get_CNTP_TVAL_EL0(
            state,
            tracer,
            s_20_9,
        );
        // D s_20_11: write-var ga#56828 <= s_20_10
        fn_state.ga_56828 = s_20_10;
        // D s_20_12: read-var ga#56828.0:struct
        let s_20_12: u64 = fn_state.ga_56828._0;
        // D s_20_13: cast zx s_20_12 -> bv
        let s_20_13: Bits = Bits::new(s_20_12 as u128, 64u16);
        // D s_20_14: read-var t:i
        let s_20_14: i128 = fn_state.t;
        // D s_20_15: call X_set(s_20_14, s_20_0, s_20_13)
        let s_20_15: () = X_set(state, tracer, s_20_14, s_20_0, s_20_13);
        // N s_20_16: return
        return;
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #64s : i64
        let s_21_0: i64 = 64;
        // C s_21_1: const #64s : i64
        let s_21_1: i64 = 64;
        // C s_21_2: cast zx s_21_1 -> i
        let s_21_2: i128 = (i128::try_from(s_21_1).unwrap());
        // S s_21_3: call __UNKNOWN_bits(s_21_2)
        let s_21_3: Bits = u__UNKNOWN_bits(state, tracer, s_21_2);
        // S s_21_4: cast reint s_21_3 -> u64
        let s_21_4: u64 = (s_21_3.value() as u64);
        // S s_21_5: cast zx s_21_4 -> bv
        let s_21_5: Bits = Bits::new(s_21_4 as u128, 64u16);
        // D s_21_6: read-var t:i
        let s_21_6: i128 = fn_state.t;
        // D s_21_7: call X_set(s_21_6, s_21_0, s_21_5)
        let s_21_7: () = X_set(state, tracer, s_21_6, s_21_0, s_21_5);
        // N s_21_8: return
        return;
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #90704u : u32
        let s_22_0: u32 = 90704;
        // D s_22_1: read-reg s_22_0:struct
        let s_22_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_22_0 as isize);
            tracer.read_register(s_22_0 as isize, value);
            value
        };
        // D s_22_2: call _get_SCR_EL3_Type_NS(s_22_1)
        let s_22_2: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_22_1);
        // D s_22_3: cast zx s_22_2 -> bv
        let s_22_3: Bits = Bits::new(s_22_2 as u128, 1u16);
        // C s_22_4: const #1u : u8
        let s_22_4: bool = true;
        // C s_22_5: cast zx s_22_4 -> bv
        let s_22_5: Bits = Bits::new(s_22_4 as u128, 1u16);
        // D s_22_6: cmp-eq s_22_3 s_22_5
        let s_22_6: bool = ((s_22_3) == (s_22_5));
        // D s_22_7: write-var gs#58064 <= s_22_6
        fn_state.gs_58064 = s_22_6;
        // N s_22_8: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var __CNTHPS_CTL_EL2_ENABLE:u8
        let s_23_0: bool = fn_state.u__CNTHPS_CTL_EL2_ENABLE;
        // D s_23_1: cast zx s_23_0 -> bv
        let s_23_1: Bits = Bits::new(s_23_0 as u128, 1u16);
        // C s_23_2: const #0u : u8
        let s_23_2: bool = false;
        // C s_23_3: cast zx s_23_2 -> bv
        let s_23_3: Bits = Bits::new(s_23_2 as u128, 1u16);
        // D s_23_4: cmp-eq s_23_1 s_23_3
        let s_23_4: bool = ((s_23_1) == (s_23_3));
        // N s_23_5: branch s_23_4 b25 b24
        if s_23_4 {
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
        // C s_24_0: const #64s : i64
        let s_24_0: i64 = 64;
        // C s_24_1: const #22672u : u32
        let s_24_1: u32 = 22672;
        // D s_24_2: read-reg s_24_1:u64
        let s_24_2: u64 = {
            let value = state.read_register::<u64>(s_24_1 as isize);
            tracer.read_register(s_24_1 as isize, value);
            value
        };
        // C s_24_3: const #() : ()
        let s_24_3: () = ();
        // S s_24_4: call PhysicalCountInt(s_24_3)
        let s_24_4: u64 = PhysicalCountInt(state, tracer, s_24_3);
        // D s_24_5: cast zx s_24_2 -> bv
        let s_24_5: Bits = Bits::new(s_24_2 as u128, 64u16);
        // S s_24_6: cast zx s_24_4 -> bv
        let s_24_6: Bits = Bits::new(s_24_4 as u128, 64u16);
        // D s_24_7: sub s_24_5 s_24_6
        let s_24_7: Bits = ((s_24_5) - (s_24_6));
        // D s_24_8: cast reint s_24_7 -> u64
        let s_24_8: u64 = (s_24_7.value() as u64);
        // D s_24_9: call Mk_CNTP_TVAL_EL0_Type(s_24_8)
        let s_24_9: ProductType5c790c8ef59cc8b2 = Mk_CNTP_TVAL_EL0_Type(
            state,
            tracer,
            s_24_8,
        );
        // D s_24_10: call __get_CNTP_TVAL_EL0(s_24_9)
        let s_24_10: ProductType5c790c8ef59cc8b2 = u__get_CNTP_TVAL_EL0(
            state,
            tracer,
            s_24_9,
        );
        // D s_24_11: write-var ga#56813 <= s_24_10
        fn_state.ga_56813 = s_24_10;
        // D s_24_12: read-var ga#56813.0:struct
        let s_24_12: u64 = fn_state.ga_56813._0;
        // D s_24_13: cast zx s_24_12 -> bv
        let s_24_13: Bits = Bits::new(s_24_12 as u128, 64u16);
        // D s_24_14: read-var t:i
        let s_24_14: i128 = fn_state.t;
        // D s_24_15: call X_set(s_24_14, s_24_0, s_24_13)
        let s_24_15: () = X_set(state, tracer, s_24_14, s_24_0, s_24_13);
        // N s_24_16: return
        return;
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #64s : i64
        let s_25_0: i64 = 64;
        // C s_25_1: const #64s : i64
        let s_25_1: i64 = 64;
        // C s_25_2: cast zx s_25_1 -> i
        let s_25_2: i128 = (i128::try_from(s_25_1).unwrap());
        // S s_25_3: call __UNKNOWN_bits(s_25_2)
        let s_25_3: Bits = u__UNKNOWN_bits(state, tracer, s_25_2);
        // S s_25_4: cast reint s_25_3 -> u64
        let s_25_4: u64 = (s_25_3.value() as u64);
        // S s_25_5: cast zx s_25_4 -> bv
        let s_25_5: Bits = Bits::new(s_25_4 as u128, 64u16);
        // D s_25_6: read-var t:i
        let s_25_6: i128 = fn_state.t;
        // D s_25_7: call X_set(s_25_6, s_25_0, s_25_5)
        let s_25_7: () = X_set(state, tracer, s_25_6, s_25_0, s_25_5);
        // N s_25_8: return
        return;
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #117u : u32
        let s_26_0: u32 = 117;
        // S s_26_1: call IsFeatureImplemented(s_26_0)
        let s_26_1: bool = IsFeatureImplemented(state, tracer, s_26_0);
        // D s_26_2: write-var gs#58063 <= s_26_1
        fn_state.gs_58063 = s_26_1;
        // N s_26_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #90704u : u32
        let s_27_0: u32 = 90704;
        // D s_27_1: read-reg s_27_0:struct
        let s_27_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_27_0 as isize);
            tracer.read_register(s_27_0 as isize, value);
            value
        };
        // D s_27_2: call _get_SCR_EL3_Type_NS(s_27_1)
        let s_27_2: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_27_1);
        // D s_27_3: cast zx s_27_2 -> bv
        let s_27_3: Bits = Bits::new(s_27_2 as u128, 1u16);
        // C s_27_4: const #0u : u8
        let s_27_4: bool = false;
        // C s_27_5: cast zx s_27_4 -> bv
        let s_27_5: Bits = Bits::new(s_27_4 as u128, 1u16);
        // D s_27_6: cmp-eq s_27_3 s_27_5
        let s_27_6: bool = ((s_27_3) == (s_27_5));
        // D s_27_7: write-var gs#58062 <= s_27_6
        fn_state.gs_58062 = s_27_6;
        // N s_27_8: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #() : ()
        let s_28_0: () = ();
        // S s_28_1: call EL2Enabled(s_28_0)
        let s_28_1: bool = EL2Enabled(state, tracer, s_28_0);
        // N s_28_2: branch s_28_1 b59 b29
        if s_28_1 {
            return block_59(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #0u : u8
        let s_29_0: bool = false;
        // D s_29_1: write-var gs#58074 <= s_29_0
        fn_state.gs_58074 = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var gs#58074:u8
        let s_30_0: bool = fn_state.gs_58074;
        // N s_30_1: branch s_30_0 b58 b31
        if s_30_0 {
            return block_58(state, tracer, fn_state);
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
        // D s_31_1: write-var gs#58075 <= s_31_0
        fn_state.gs_58075 = s_31_0;
        // N s_31_2: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var gs#58075:u8
        let s_32_0: bool = fn_state.gs_58075;
        // N s_32_1: branch s_32_0 b57 b33
        if s_32_0 {
            return block_57(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #() : ()
        let s_33_0: () = ();
        // S s_33_1: call EL2Enabled(s_33_0)
        let s_33_1: bool = EL2Enabled(state, tracer, s_33_0);
        // N s_33_2: branch s_33_1 b56 b34
        if s_33_1 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #0u : u8
        let s_34_0: bool = false;
        // D s_34_1: write-var gs#58076 <= s_34_0
        fn_state.gs_58076 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#58076:u8
        let s_35_0: bool = fn_state.gs_58076;
        // N s_35_1: branch s_35_0 b55 b36
        if s_35_0 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #0u : u8
        let s_36_0: bool = false;
        // D s_36_1: write-var gs#58077 <= s_36_0
        fn_state.gs_58077 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#58077:u8
        let s_37_0: bool = fn_state.gs_58077;
        // N s_37_1: branch s_37_0 b54 b38
        if s_37_0 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #145u : u32
        let s_38_0: u32 = 145;
        // S s_38_1: call IsFeatureImplemented(s_38_0)
        let s_38_1: bool = IsFeatureImplemented(state, tracer, s_38_0);
        // N s_38_2: branch s_38_1 b53 b39
        if s_38_1 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #0u : u8
        let s_39_0: bool = false;
        // D s_39_1: write-var gs#58078 <= s_39_0
        fn_state.gs_58078 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#58078:u8
        let s_40_0: bool = fn_state.gs_58078;
        // N s_40_1: branch s_40_0 b52 b41
        if s_40_0 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #0u : u8
        let s_41_0: bool = false;
        // D s_41_1: write-var gs#58079 <= s_41_0
        fn_state.gs_58079 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#58079:u8
        let s_42_0: bool = fn_state.gs_58079;
        // N s_42_1: branch s_42_0 b51 b43
        if s_42_0 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #0u : u8
        let s_43_0: bool = false;
        // D s_43_1: write-var gs#58080 <= s_43_0
        fn_state.gs_58080 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#58080:u8
        let s_44_0: bool = fn_state.gs_58080;
        // N s_44_1: branch s_44_0 b48 b45
        if s_44_0 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var __CNTP_CTL_EL0_ENABLE:u8
        let s_45_0: bool = fn_state.u__CNTP_CTL_EL0_ENABLE;
        // D s_45_1: cast zx s_45_0 -> bv
        let s_45_1: Bits = Bits::new(s_45_0 as u128, 1u16);
        // C s_45_2: const #0u : u8
        let s_45_2: bool = false;
        // C s_45_3: cast zx s_45_2 -> bv
        let s_45_3: Bits = Bits::new(s_45_2 as u128, 1u16);
        // D s_45_4: cmp-eq s_45_1 s_45_3
        let s_45_4: bool = ((s_45_1) == (s_45_3));
        // N s_45_5: branch s_45_4 b47 b46
        if s_45_4 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_46(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #64s : i64
        let s_46_0: i64 = 64;
        // C s_46_1: const #20800u : u32
        let s_46_1: u32 = 20800;
        // D s_46_2: read-reg s_46_1:u64
        let s_46_2: u64 = {
            let value = state.read_register::<u64>(s_46_1 as isize);
            tracer.read_register(s_46_1 as isize, value);
            value
        };
        // C s_46_3: const #() : ()
        let s_46_3: () = ();
        // S s_46_4: call PhysicalCountInt(s_46_3)
        let s_46_4: u64 = PhysicalCountInt(state, tracer, s_46_3);
        // D s_46_5: cast zx s_46_2 -> bv
        let s_46_5: Bits = Bits::new(s_46_2 as u128, 64u16);
        // S s_46_6: cast zx s_46_4 -> bv
        let s_46_6: Bits = Bits::new(s_46_4 as u128, 64u16);
        // D s_46_7: sub s_46_5 s_46_6
        let s_46_7: Bits = ((s_46_5) - (s_46_6));
        // D s_46_8: cast reint s_46_7 -> u64
        let s_46_8: u64 = (s_46_7.value() as u64);
        // D s_46_9: call Mk_CNTP_TVAL_EL0_Type(s_46_8)
        let s_46_9: ProductType5c790c8ef59cc8b2 = Mk_CNTP_TVAL_EL0_Type(
            state,
            tracer,
            s_46_8,
        );
        // D s_46_10: call __get_CNTP_TVAL_EL0(s_46_9)
        let s_46_10: ProductType5c790c8ef59cc8b2 = u__get_CNTP_TVAL_EL0(
            state,
            tracer,
            s_46_9,
        );
        // D s_46_11: write-var ga#56796 <= s_46_10
        fn_state.ga_56796 = s_46_10;
        // D s_46_12: read-var ga#56796.0:struct
        let s_46_12: u64 = fn_state.ga_56796._0;
        // D s_46_13: cast zx s_46_12 -> bv
        let s_46_13: Bits = Bits::new(s_46_12 as u128, 64u16);
        // D s_46_14: read-var t:i
        let s_46_14: i128 = fn_state.t;
        // D s_46_15: call X_set(s_46_14, s_46_0, s_46_13)
        let s_46_15: () = X_set(state, tracer, s_46_14, s_46_0, s_46_13);
        // N s_46_16: return
        return;
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #64s : i64
        let s_47_0: i64 = 64;
        // C s_47_1: const #64s : i64
        let s_47_1: i64 = 64;
        // C s_47_2: cast zx s_47_1 -> i
        let s_47_2: i128 = (i128::try_from(s_47_1).unwrap());
        // S s_47_3: call __UNKNOWN_bits(s_47_2)
        let s_47_3: Bits = u__UNKNOWN_bits(state, tracer, s_47_2);
        // S s_47_4: cast reint s_47_3 -> u64
        let s_47_4: u64 = (s_47_3.value() as u64);
        // S s_47_5: cast zx s_47_4 -> bv
        let s_47_5: Bits = Bits::new(s_47_4 as u128, 64u16);
        // D s_47_6: read-var t:i
        let s_47_6: i128 = fn_state.t;
        // D s_47_7: call X_set(s_47_6, s_47_0, s_47_5)
        let s_47_7: () = X_set(state, tracer, s_47_6, s_47_0, s_47_5);
        // N s_47_8: return
        return;
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var __CNTP_CTL_EL0_ENABLE:u8
        let s_48_0: bool = fn_state.u__CNTP_CTL_EL0_ENABLE;
        // D s_48_1: cast zx s_48_0 -> bv
        let s_48_1: Bits = Bits::new(s_48_0 as u128, 1u16);
        // C s_48_2: const #0u : u8
        let s_48_2: bool = false;
        // C s_48_3: cast zx s_48_2 -> bv
        let s_48_3: Bits = Bits::new(s_48_2 as u128, 1u16);
        // D s_48_4: cmp-eq s_48_1 s_48_3
        let s_48_4: bool = ((s_48_1) == (s_48_3));
        // N s_48_5: branch s_48_4 b50 b49
        if s_48_4 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #64s : i64
        let s_49_0: i64 = 64;
        // C s_49_1: const #20800u : u32
        let s_49_1: u32 = 20800;
        // D s_49_2: read-reg s_49_1:u64
        let s_49_2: u64 = {
            let value = state.read_register::<u64>(s_49_1 as isize);
            tracer.read_register(s_49_1 as isize, value);
            value
        };
        // C s_49_3: const #() : ()
        let s_49_3: () = ();
        // S s_49_4: call PhysicalCountInt(s_49_3)
        let s_49_4: u64 = PhysicalCountInt(state, tracer, s_49_3);
        // S s_49_5: cast zx s_49_4 -> bv
        let s_49_5: Bits = Bits::new(s_49_4 as u128, 64u16);
        // C s_49_6: const #14584u : u32
        let s_49_6: u32 = 14584;
        // D s_49_7: read-reg s_49_6:u64
        let s_49_7: u64 = {
            let value = state.read_register::<u64>(s_49_6 as isize);
            tracer.read_register(s_49_6 as isize, value);
            value
        };
        // D s_49_8: cast zx s_49_7 -> bv
        let s_49_8: Bits = Bits::new(s_49_7 as u128, 64u16);
        // D s_49_9: sub s_49_5 s_49_8
        let s_49_9: Bits = ((s_49_5) - (s_49_8));
        // D s_49_10: cast reint s_49_9 -> u64
        let s_49_10: u64 = (s_49_9.value() as u64);
        // D s_49_11: cast zx s_49_2 -> bv
        let s_49_11: Bits = Bits::new(s_49_2 as u128, 64u16);
        // D s_49_12: cast zx s_49_10 -> bv
        let s_49_12: Bits = Bits::new(s_49_10 as u128, 64u16);
        // D s_49_13: sub s_49_11 s_49_12
        let s_49_13: Bits = ((s_49_11) - (s_49_12));
        // D s_49_14: cast reint s_49_13 -> u64
        let s_49_14: u64 = (s_49_13.value() as u64);
        // D s_49_15: call Mk_CNTP_TVAL_EL0_Type(s_49_14)
        let s_49_15: ProductType5c790c8ef59cc8b2 = Mk_CNTP_TVAL_EL0_Type(
            state,
            tracer,
            s_49_14,
        );
        // D s_49_16: call __get_CNTP_TVAL_EL0(s_49_15)
        let s_49_16: ProductType5c790c8ef59cc8b2 = u__get_CNTP_TVAL_EL0(
            state,
            tracer,
            s_49_15,
        );
        // D s_49_17: write-var ga#56785 <= s_49_16
        fn_state.ga_56785 = s_49_16;
        // D s_49_18: read-var ga#56785.0:struct
        let s_49_18: u64 = fn_state.ga_56785._0;
        // D s_49_19: cast zx s_49_18 -> bv
        let s_49_19: Bits = Bits::new(s_49_18 as u128, 64u16);
        // D s_49_20: read-var t:i
        let s_49_20: i128 = fn_state.t;
        // D s_49_21: call X_set(s_49_20, s_49_0, s_49_19)
        let s_49_21: () = X_set(state, tracer, s_49_20, s_49_0, s_49_19);
        // N s_49_22: return
        return;
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #64s : i64
        let s_50_0: i64 = 64;
        // C s_50_1: const #64s : i64
        let s_50_1: i64 = 64;
        // C s_50_2: cast zx s_50_1 -> i
        let s_50_2: i128 = (i128::try_from(s_50_1).unwrap());
        // S s_50_3: call __UNKNOWN_bits(s_50_2)
        let s_50_3: Bits = u__UNKNOWN_bits(state, tracer, s_50_2);
        // S s_50_4: cast reint s_50_3 -> u64
        let s_50_4: u64 = (s_50_3.value() as u64);
        // S s_50_5: cast zx s_50_4 -> bv
        let s_50_5: Bits = Bits::new(s_50_4 as u128, 64u16);
        // D s_50_6: read-var t:i
        let s_50_6: i128 = fn_state.t;
        // D s_50_7: call X_set(s_50_6, s_50_0, s_50_5)
        let s_50_7: () = X_set(state, tracer, s_50_6, s_50_0, s_50_5);
        // N s_50_8: return
        return;
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var __CNTHCTL_EL2_ECV:u8
        let s_51_0: bool = fn_state.u__CNTHCTL_EL2_ECV;
        // D s_51_1: cast zx s_51_0 -> bv
        let s_51_1: Bits = Bits::new(s_51_0 as u128, 1u16);
        // C s_51_2: const #1u : u8
        let s_51_2: bool = true;
        // C s_51_3: cast zx s_51_2 -> bv
        let s_51_3: Bits = Bits::new(s_51_2 as u128, 1u16);
        // D s_51_4: cmp-eq s_51_1 s_51_3
        let s_51_4: bool = ((s_51_1) == (s_51_3));
        // D s_51_5: write-var gs#58080 <= s_51_4
        fn_state.gs_58080 = s_51_4;
        // N s_51_6: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var __SCR_EL3_ECVEn:u8
        let s_52_0: bool = fn_state.u__SCR_EL3_ECVEn;
        // D s_52_1: cast zx s_52_0 -> bv
        let s_52_1: Bits = Bits::new(s_52_0 as u128, 1u16);
        // C s_52_2: const #1u : u8
        let s_52_2: bool = true;
        // C s_52_3: cast zx s_52_2 -> bv
        let s_52_3: Bits = Bits::new(s_52_2 as u128, 1u16);
        // D s_52_4: cmp-eq s_52_1 s_52_3
        let s_52_4: bool = ((s_52_1) == (s_52_3));
        // D s_52_5: write-var gs#58079 <= s_52_4
        fn_state.gs_58079 = s_52_4;
        // N s_52_6: jump b42
        return block_42(state, tracer, fn_state);
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
        // D s_53_2: write-var gs#58078 <= s_53_1
        fn_state.gs_58078 = s_53_1;
        // N s_53_3: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #24u : u8
        let s_54_0: u8 = 24;
        // C s_54_1: cast zx s_54_0 -> bv
        let s_54_1: Bits = Bits::new(s_54_0 as u128, 8u16);
        // C s_54_2: cast zx s_54_1 -> i
        let s_54_2: i128 = (s_54_1.value() as i128);
        // C s_54_3: cast reint s_54_2 -> i64
        let s_54_3: i64 = (s_54_2 as i64);
        // C s_54_4: cast zx s_54_3 -> i
        let s_54_4: i128 = (i128::try_from(s_54_3).unwrap());
        // C s_54_5: const #432u : u32
        let s_54_5: u32 = 432;
        // D s_54_6: read-reg s_54_5:u8
        let s_54_6: u8 = {
            let value = state.read_register::<u8>(s_54_5 as isize);
            tracer.read_register(s_54_5 as isize, value);
            value
        };
        // D s_54_7: call AArch64_SystemAccessTrap(s_54_6, s_54_4)
        let s_54_7: () = AArch64_SystemAccessTrap(state, tracer, s_54_6, s_54_4);
        // N s_54_8: return
        return;
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var __CNTHCTL_EL2_EL1PTEN:u8
        let s_55_0: bool = fn_state.u__CNTHCTL_EL2_EL1PTEN;
        // D s_55_1: cast zx s_55_0 -> bv
        let s_55_1: Bits = Bits::new(s_55_0 as u128, 1u16);
        // C s_55_2: const #0u : u8
        let s_55_2: bool = false;
        // C s_55_3: cast zx s_55_2 -> bv
        let s_55_3: Bits = Bits::new(s_55_2 as u128, 1u16);
        // D s_55_4: cmp-eq s_55_1 s_55_3
        let s_55_4: bool = ((s_55_1) == (s_55_3));
        // D s_55_5: write-var gs#58077 <= s_55_4
        fn_state.gs_58077 = s_55_4;
        // N s_55_6: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #102552u : u32
        let s_56_0: u32 = 102552;
        // D s_56_1: read-reg s_56_0:struct
        let s_56_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_56_0 as isize);
            tracer.read_register(s_56_0 as isize, value);
            value
        };
        // D s_56_2: call _get_HCR_EL2_Type_E2H(s_56_1)
        let s_56_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_56_1);
        // D s_56_3: cast zx s_56_2 -> bv
        let s_56_3: Bits = Bits::new(s_56_2 as u128, 1u16);
        // C s_56_4: const #1u : u8
        let s_56_4: bool = true;
        // C s_56_5: cast zx s_56_4 -> bv
        let s_56_5: Bits = Bits::new(s_56_4 as u128, 1u16);
        // D s_56_6: cmp-eq s_56_3 s_56_5
        let s_56_6: bool = ((s_56_3) == (s_56_5));
        // D s_56_7: write-var gs#58076 <= s_56_6
        fn_state.gs_58076 = s_56_6;
        // N s_56_8: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #24u : u8
        let s_57_0: u8 = 24;
        // C s_57_1: cast zx s_57_0 -> bv
        let s_57_1: Bits = Bits::new(s_57_0 as u128, 8u16);
        // C s_57_2: cast zx s_57_1 -> i
        let s_57_2: i128 = (s_57_1.value() as i128);
        // C s_57_3: cast reint s_57_2 -> i64
        let s_57_3: i64 = (s_57_2 as i64);
        // C s_57_4: cast zx s_57_3 -> i
        let s_57_4: i128 = (i128::try_from(s_57_3).unwrap());
        // C s_57_5: const #432u : u32
        let s_57_5: u32 = 432;
        // D s_57_6: read-reg s_57_5:u8
        let s_57_6: u8 = {
            let value = state.read_register::<u8>(s_57_5 as isize);
            tracer.read_register(s_57_5 as isize, value);
            value
        };
        // D s_57_7: call AArch64_SystemAccessTrap(s_57_6, s_57_4)
        let s_57_7: () = AArch64_SystemAccessTrap(state, tracer, s_57_6, s_57_4);
        // N s_57_8: return
        return;
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var __CNTHCTL_EL2_EL1PCEN:u8
        let s_58_0: bool = fn_state.u__CNTHCTL_EL2_EL1PCEN;
        // D s_58_1: cast zx s_58_0 -> bv
        let s_58_1: Bits = Bits::new(s_58_0 as u128, 1u16);
        // C s_58_2: const #0u : u8
        let s_58_2: bool = false;
        // C s_58_3: cast zx s_58_2 -> bv
        let s_58_3: Bits = Bits::new(s_58_2 as u128, 1u16);
        // D s_58_4: cmp-eq s_58_1 s_58_3
        let s_58_4: bool = ((s_58_1) == (s_58_3));
        // D s_58_5: write-var gs#58075 <= s_58_4
        fn_state.gs_58075 = s_58_4;
        // N s_58_6: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #102552u : u32
        let s_59_0: u32 = 102552;
        // D s_59_1: read-reg s_59_0:struct
        let s_59_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_59_0 as isize);
            tracer.read_register(s_59_0 as isize, value);
            value
        };
        // D s_59_2: call _get_HCR_EL2_Type_E2H(s_59_1)
        let s_59_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_59_1);
        // D s_59_3: cast zx s_59_2 -> bv
        let s_59_3: Bits = Bits::new(s_59_2 as u128, 1u16);
        // C s_59_4: const #0u : u8
        let s_59_4: bool = false;
        // C s_59_5: cast zx s_59_4 -> bv
        let s_59_5: Bits = Bits::new(s_59_4 as u128, 1u16);
        // D s_59_6: cmp-eq s_59_3 s_59_5
        let s_59_6: bool = ((s_59_3) == (s_59_5));
        // D s_59_7: write-var gs#58074 <= s_59_6
        fn_state.gs_58074 = s_59_6;
        // N s_59_8: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #() : ()
        let s_60_0: () = ();
        // S s_60_1: call EL2Enabled(s_60_0)
        let s_60_1: bool = EL2Enabled(state, tracer, s_60_0);
        // N s_60_2: branch s_60_1 b138 b61
        if s_60_1 {
            return block_138(state, tracer, fn_state);
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
        // D s_61_1: write-var gs#58087 <= s_61_0
        fn_state.gs_58087 = s_61_0;
        // N s_61_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var gs#58087:u8
        let s_62_0: bool = fn_state.gs_58087;
        // D s_62_1: not s_62_0
        let s_62_1: bool = !s_62_0;
        // N s_62_2: branch s_62_1 b137 b63
        if s_62_1 {
            return block_137(state, tracer, fn_state);
        } else {
            return block_63(state, tracer, fn_state);
        };
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #0u : u8
        let s_63_0: bool = false;
        // D s_63_1: write-var gs#58088 <= s_63_0
        fn_state.gs_58088 = s_63_0;
        // N s_63_2: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var gs#58088:u8
        let s_64_0: bool = fn_state.gs_58088;
        // N s_64_1: branch s_64_0 b131 b65
        if s_64_0 {
            return block_131(state, tracer, fn_state);
        } else {
            return block_65(state, tracer, fn_state);
        };
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #() : ()
        let s_65_0: () = ();
        // S s_65_1: call EL2Enabled(s_65_0)
        let s_65_1: bool = EL2Enabled(state, tracer, s_65_0);
        // N s_65_2: branch s_65_1 b130 b66
        if s_65_1 {
            return block_130(state, tracer, fn_state);
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
        // D s_66_1: write-var gs#58089 <= s_66_0
        fn_state.gs_58089 = s_66_0;
        // N s_66_2: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var gs#58089:u8
        let s_67_0: bool = fn_state.gs_58089;
        // N s_67_1: branch s_67_0 b129 b68
        if s_67_0 {
            return block_129(state, tracer, fn_state);
        } else {
            return block_68(state, tracer, fn_state);
        };
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #0u : u8
        let s_68_0: bool = false;
        // D s_68_1: write-var gs#58090 <= s_68_0
        fn_state.gs_58090 = s_68_0;
        // N s_68_2: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_69_0: read-var gs#58090:u8
        let s_69_0: bool = fn_state.gs_58090;
        // N s_69_1: branch s_69_0 b128 b70
        if s_69_0 {
            return block_128(state, tracer, fn_state);
        } else {
            return block_70(state, tracer, fn_state);
        };
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #() : ()
        let s_70_0: () = ();
        // S s_70_1: call EL2Enabled(s_70_0)
        let s_70_1: bool = EL2Enabled(state, tracer, s_70_0);
        // N s_70_2: branch s_70_1 b127 b71
        if s_70_1 {
            return block_127(state, tracer, fn_state);
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
        // D s_71_1: write-var gs#58091 <= s_71_0
        fn_state.gs_58091 = s_71_0;
        // N s_71_2: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_72_0: read-var gs#58091:u8
        let s_72_0: bool = fn_state.gs_58091;
        // N s_72_1: branch s_72_0 b126 b73
        if s_72_0 {
            return block_126(state, tracer, fn_state);
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
        // D s_73_1: write-var gs#58092 <= s_73_0
        fn_state.gs_58092 = s_73_0;
        // N s_73_2: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_74_0: read-var gs#58092:u8
        let s_74_0: bool = fn_state.gs_58092;
        // N s_74_1: branch s_74_0 b125 b75
        if s_74_0 {
            return block_125(state, tracer, fn_state);
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
        // N s_75_2: branch s_75_1 b124 b76
        if s_75_1 {
            return block_124(state, tracer, fn_state);
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
        // D s_76_1: write-var gs#58093 <= s_76_0
        fn_state.gs_58093 = s_76_0;
        // N s_76_2: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_77_0: read-var gs#58093:u8
        let s_77_0: bool = fn_state.gs_58093;
        // N s_77_1: branch s_77_0 b123 b78
        if s_77_0 {
            return block_123(state, tracer, fn_state);
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
        // D s_78_1: write-var gs#58094 <= s_78_0
        fn_state.gs_58094 = s_78_0;
        // N s_78_2: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_79_0: read-var gs#58094:u8
        let s_79_0: bool = fn_state.gs_58094;
        // N s_79_1: branch s_79_0 b122 b80
        if s_79_0 {
            return block_122(state, tracer, fn_state);
        } else {
            return block_80(state, tracer, fn_state);
        };
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #() : ()
        let s_80_0: () = ();
        // S s_80_1: call EL2Enabled(s_80_0)
        let s_80_1: bool = EL2Enabled(state, tracer, s_80_0);
        // N s_80_2: branch s_80_1 b121 b81
        if s_80_1 {
            return block_121(state, tracer, fn_state);
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
        // D s_81_1: write-var gs#58095 <= s_81_0
        fn_state.gs_58095 = s_81_0;
        // N s_81_2: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_82_0: read-var gs#58095:u8
        let s_82_0: bool = fn_state.gs_58095;
        // N s_82_1: branch s_82_0 b120 b83
        if s_82_0 {
            return block_120(state, tracer, fn_state);
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
        // D s_83_1: write-var gs#58096 <= s_83_0
        fn_state.gs_58096 = s_83_0;
        // N s_83_2: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_84_0: read-var gs#58096:u8
        let s_84_0: bool = fn_state.gs_58096;
        // N s_84_1: branch s_84_0 b119 b85
        if s_84_0 {
            return block_119(state, tracer, fn_state);
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
        // D s_85_1: write-var gs#58097 <= s_85_0
        fn_state.gs_58097 = s_85_0;
        // N s_85_2: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_86_0: read-var gs#58097:u8
        let s_86_0: bool = fn_state.gs_58097;
        // N s_86_1: branch s_86_0 b116 b87
        if s_86_0 {
            return block_116(state, tracer, fn_state);
        } else {
            return block_87(state, tracer, fn_state);
        };
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_87_0: const #() : ()
        let s_87_0: () = ();
        // S s_87_1: call EL2Enabled(s_87_0)
        let s_87_1: bool = EL2Enabled(state, tracer, s_87_0);
        // N s_87_2: branch s_87_1 b115 b88
        if s_87_1 {
            return block_115(state, tracer, fn_state);
        } else {
            return block_88(state, tracer, fn_state);
        };
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_88_0: const #0u : u8
        let s_88_0: bool = false;
        // D s_88_1: write-var gs#58098 <= s_88_0
        fn_state.gs_58098 = s_88_0;
        // N s_88_2: jump b89
        return block_89(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_89_0: read-var gs#58098:u8
        let s_89_0: bool = fn_state.gs_58098;
        // N s_89_1: branch s_89_0 b114 b90
        if s_89_0 {
            return block_114(state, tracer, fn_state);
        } else {
            return block_90(state, tracer, fn_state);
        };
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_90_0: const #0u : u8
        let s_90_0: bool = false;
        // D s_90_1: write-var gs#58099 <= s_90_0
        fn_state.gs_58099 = s_90_0;
        // N s_90_2: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_91_0: read-var gs#58099:u8
        let s_91_0: bool = fn_state.gs_58099;
        // N s_91_1: branch s_91_0 b111 b92
        if s_91_0 {
            return block_111(state, tracer, fn_state);
        } else {
            return block_92(state, tracer, fn_state);
        };
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_92_0: const #145u : u32
        let s_92_0: u32 = 145;
        // S s_92_1: call IsFeatureImplemented(s_92_0)
        let s_92_1: bool = IsFeatureImplemented(state, tracer, s_92_0);
        // N s_92_2: branch s_92_1 b110 b93
        if s_92_1 {
            return block_110(state, tracer, fn_state);
        } else {
            return block_93(state, tracer, fn_state);
        };
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_93_0: const #0u : u8
        let s_93_0: bool = false;
        // D s_93_1: write-var gs#58100 <= s_93_0
        fn_state.gs_58100 = s_93_0;
        // N s_93_2: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_94_0: read-var gs#58100:u8
        let s_94_0: bool = fn_state.gs_58100;
        // N s_94_1: branch s_94_0 b109 b95
        if s_94_0 {
            return block_109(state, tracer, fn_state);
        } else {
            return block_95(state, tracer, fn_state);
        };
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_95_0: const #0u : u8
        let s_95_0: bool = false;
        // D s_95_1: write-var gs#58101 <= s_95_0
        fn_state.gs_58101 = s_95_0;
        // N s_95_2: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_96_0: read-var gs#58101:u8
        let s_96_0: bool = fn_state.gs_58101;
        // N s_96_1: branch s_96_0 b108 b97
        if s_96_0 {
            return block_108(state, tracer, fn_state);
        } else {
            return block_97(state, tracer, fn_state);
        };
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_97_0: const #0u : u8
        let s_97_0: bool = false;
        // D s_97_1: write-var gs#58102 <= s_97_0
        fn_state.gs_58102 = s_97_0;
        // N s_97_2: jump b98
        return block_98(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_98_0: read-var gs#58102:u8
        let s_98_0: bool = fn_state.gs_58102;
        // N s_98_1: branch s_98_0 b107 b99
        if s_98_0 {
            return block_107(state, tracer, fn_state);
        } else {
            return block_99(state, tracer, fn_state);
        };
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_99_0: const #0u : u8
        let s_99_0: bool = false;
        // D s_99_1: write-var gs#58103 <= s_99_0
        fn_state.gs_58103 = s_99_0;
        // N s_99_2: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_100_0: read-var gs#58103:u8
        let s_100_0: bool = fn_state.gs_58103;
        // N s_100_1: branch s_100_0 b104 b101
        if s_100_0 {
            return block_104(state, tracer, fn_state);
        } else {
            return block_101(state, tracer, fn_state);
        };
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_101_0: read-var __CNTP_CTL_EL0_ENABLE:u8
        let s_101_0: bool = fn_state.u__CNTP_CTL_EL0_ENABLE;
        // D s_101_1: cast zx s_101_0 -> bv
        let s_101_1: Bits = Bits::new(s_101_0 as u128, 1u16);
        // C s_101_2: const #0u : u8
        let s_101_2: bool = false;
        // C s_101_3: cast zx s_101_2 -> bv
        let s_101_3: Bits = Bits::new(s_101_2 as u128, 1u16);
        // D s_101_4: cmp-eq s_101_1 s_101_3
        let s_101_4: bool = ((s_101_1) == (s_101_3));
        // N s_101_5: branch s_101_4 b103 b102
        if s_101_4 {
            return block_103(state, tracer, fn_state);
        } else {
            return block_102(state, tracer, fn_state);
        };
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_102_0: const #64s : i64
        let s_102_0: i64 = 64;
        // C s_102_1: const #20800u : u32
        let s_102_1: u32 = 20800;
        // D s_102_2: read-reg s_102_1:u64
        let s_102_2: u64 = {
            let value = state.read_register::<u64>(s_102_1 as isize);
            tracer.read_register(s_102_1 as isize, value);
            value
        };
        // C s_102_3: const #() : ()
        let s_102_3: () = ();
        // S s_102_4: call PhysicalCountInt(s_102_3)
        let s_102_4: u64 = PhysicalCountInt(state, tracer, s_102_3);
        // D s_102_5: cast zx s_102_2 -> bv
        let s_102_5: Bits = Bits::new(s_102_2 as u128, 64u16);
        // S s_102_6: cast zx s_102_4 -> bv
        let s_102_6: Bits = Bits::new(s_102_4 as u128, 64u16);
        // D s_102_7: sub s_102_5 s_102_6
        let s_102_7: Bits = ((s_102_5) - (s_102_6));
        // D s_102_8: cast reint s_102_7 -> u64
        let s_102_8: u64 = (s_102_7.value() as u64);
        // D s_102_9: call Mk_CNTP_TVAL_EL0_Type(s_102_8)
        let s_102_9: ProductType5c790c8ef59cc8b2 = Mk_CNTP_TVAL_EL0_Type(
            state,
            tracer,
            s_102_8,
        );
        // D s_102_10: call __get_CNTP_TVAL_EL0(s_102_9)
        let s_102_10: ProductType5c790c8ef59cc8b2 = u__get_CNTP_TVAL_EL0(
            state,
            tracer,
            s_102_9,
        );
        // D s_102_11: write-var ga#56758 <= s_102_10
        fn_state.ga_56758 = s_102_10;
        // D s_102_12: read-var ga#56758.0:struct
        let s_102_12: u64 = fn_state.ga_56758._0;
        // D s_102_13: cast zx s_102_12 -> bv
        let s_102_13: Bits = Bits::new(s_102_12 as u128, 64u16);
        // D s_102_14: read-var t:i
        let s_102_14: i128 = fn_state.t;
        // D s_102_15: call X_set(s_102_14, s_102_0, s_102_13)
        let s_102_15: () = X_set(state, tracer, s_102_14, s_102_0, s_102_13);
        // N s_102_16: return
        return;
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_103_0: const #64s : i64
        let s_103_0: i64 = 64;
        // C s_103_1: const #64s : i64
        let s_103_1: i64 = 64;
        // C s_103_2: cast zx s_103_1 -> i
        let s_103_2: i128 = (i128::try_from(s_103_1).unwrap());
        // S s_103_3: call __UNKNOWN_bits(s_103_2)
        let s_103_3: Bits = u__UNKNOWN_bits(state, tracer, s_103_2);
        // S s_103_4: cast reint s_103_3 -> u64
        let s_103_4: u64 = (s_103_3.value() as u64);
        // S s_103_5: cast zx s_103_4 -> bv
        let s_103_5: Bits = Bits::new(s_103_4 as u128, 64u16);
        // D s_103_6: read-var t:i
        let s_103_6: i128 = fn_state.t;
        // D s_103_7: call X_set(s_103_6, s_103_0, s_103_5)
        let s_103_7: () = X_set(state, tracer, s_103_6, s_103_0, s_103_5);
        // N s_103_8: return
        return;
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_104_0: read-var __CNTP_CTL_EL0_ENABLE:u8
        let s_104_0: bool = fn_state.u__CNTP_CTL_EL0_ENABLE;
        // D s_104_1: cast zx s_104_0 -> bv
        let s_104_1: Bits = Bits::new(s_104_0 as u128, 1u16);
        // C s_104_2: const #0u : u8
        let s_104_2: bool = false;
        // C s_104_3: cast zx s_104_2 -> bv
        let s_104_3: Bits = Bits::new(s_104_2 as u128, 1u16);
        // D s_104_4: cmp-eq s_104_1 s_104_3
        let s_104_4: bool = ((s_104_1) == (s_104_3));
        // N s_104_5: branch s_104_4 b106 b105
        if s_104_4 {
            return block_106(state, tracer, fn_state);
        } else {
            return block_105(state, tracer, fn_state);
        };
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_105_0: const #64s : i64
        let s_105_0: i64 = 64;
        // C s_105_1: const #20800u : u32
        let s_105_1: u32 = 20800;
        // D s_105_2: read-reg s_105_1:u64
        let s_105_2: u64 = {
            let value = state.read_register::<u64>(s_105_1 as isize);
            tracer.read_register(s_105_1 as isize, value);
            value
        };
        // C s_105_3: const #() : ()
        let s_105_3: () = ();
        // S s_105_4: call PhysicalCountInt(s_105_3)
        let s_105_4: u64 = PhysicalCountInt(state, tracer, s_105_3);
        // S s_105_5: cast zx s_105_4 -> bv
        let s_105_5: Bits = Bits::new(s_105_4 as u128, 64u16);
        // C s_105_6: const #14584u : u32
        let s_105_6: u32 = 14584;
        // D s_105_7: read-reg s_105_6:u64
        let s_105_7: u64 = {
            let value = state.read_register::<u64>(s_105_6 as isize);
            tracer.read_register(s_105_6 as isize, value);
            value
        };
        // D s_105_8: cast zx s_105_7 -> bv
        let s_105_8: Bits = Bits::new(s_105_7 as u128, 64u16);
        // D s_105_9: sub s_105_5 s_105_8
        let s_105_9: Bits = ((s_105_5) - (s_105_8));
        // D s_105_10: cast reint s_105_9 -> u64
        let s_105_10: u64 = (s_105_9.value() as u64);
        // D s_105_11: cast zx s_105_2 -> bv
        let s_105_11: Bits = Bits::new(s_105_2 as u128, 64u16);
        // D s_105_12: cast zx s_105_10 -> bv
        let s_105_12: Bits = Bits::new(s_105_10 as u128, 64u16);
        // D s_105_13: sub s_105_11 s_105_12
        let s_105_13: Bits = ((s_105_11) - (s_105_12));
        // D s_105_14: cast reint s_105_13 -> u64
        let s_105_14: u64 = (s_105_13.value() as u64);
        // D s_105_15: call Mk_CNTP_TVAL_EL0_Type(s_105_14)
        let s_105_15: ProductType5c790c8ef59cc8b2 = Mk_CNTP_TVAL_EL0_Type(
            state,
            tracer,
            s_105_14,
        );
        // D s_105_16: call __get_CNTP_TVAL_EL0(s_105_15)
        let s_105_16: ProductType5c790c8ef59cc8b2 = u__get_CNTP_TVAL_EL0(
            state,
            tracer,
            s_105_15,
        );
        // D s_105_17: write-var ga#56747 <= s_105_16
        fn_state.ga_56747 = s_105_16;
        // D s_105_18: read-var ga#56747.0:struct
        let s_105_18: u64 = fn_state.ga_56747._0;
        // D s_105_19: cast zx s_105_18 -> bv
        let s_105_19: Bits = Bits::new(s_105_18 as u128, 64u16);
        // D s_105_20: read-var t:i
        let s_105_20: i128 = fn_state.t;
        // D s_105_21: call X_set(s_105_20, s_105_0, s_105_19)
        let s_105_21: () = X_set(state, tracer, s_105_20, s_105_0, s_105_19);
        // N s_105_22: return
        return;
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_106_0: const #64s : i64
        let s_106_0: i64 = 64;
        // C s_106_1: const #64s : i64
        let s_106_1: i64 = 64;
        // C s_106_2: cast zx s_106_1 -> i
        let s_106_2: i128 = (i128::try_from(s_106_1).unwrap());
        // S s_106_3: call __UNKNOWN_bits(s_106_2)
        let s_106_3: Bits = u__UNKNOWN_bits(state, tracer, s_106_2);
        // S s_106_4: cast reint s_106_3 -> u64
        let s_106_4: u64 = (s_106_3.value() as u64);
        // S s_106_5: cast zx s_106_4 -> bv
        let s_106_5: Bits = Bits::new(s_106_4 as u128, 64u16);
        // D s_106_6: read-var t:i
        let s_106_6: i128 = fn_state.t;
        // D s_106_7: call X_set(s_106_6, s_106_0, s_106_5)
        let s_106_7: () = X_set(state, tracer, s_106_6, s_106_0, s_106_5);
        // N s_106_8: return
        return;
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
        // C s_107_18: const #3u : u8
        let s_107_18: u8 = 3;
        // C s_107_19: cast zx s_107_18 -> bv
        let s_107_19: Bits = Bits::new(s_107_18 as u128, 2u16);
        // D s_107_20: cmp-ne s_107_17 s_107_19
        let s_107_20: bool = ((s_107_17) != (s_107_19));
        // D s_107_21: write-var gs#58103 <= s_107_20
        fn_state.gs_58103 = s_107_20;
        // N s_107_22: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_108_0: read-var __CNTHCTL_EL2_ECV:u8
        let s_108_0: bool = fn_state.u__CNTHCTL_EL2_ECV;
        // D s_108_1: cast zx s_108_0 -> bv
        let s_108_1: Bits = Bits::new(s_108_0 as u128, 1u16);
        // C s_108_2: const #1u : u8
        let s_108_2: bool = true;
        // C s_108_3: cast zx s_108_2 -> bv
        let s_108_3: Bits = Bits::new(s_108_2 as u128, 1u16);
        // D s_108_4: cmp-eq s_108_1 s_108_3
        let s_108_4: bool = ((s_108_1) == (s_108_3));
        // D s_108_5: write-var gs#58102 <= s_108_4
        fn_state.gs_58102 = s_108_4;
        // N s_108_6: jump b98
        return block_98(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_109_0: read-var __SCR_EL3_ECVEn:u8
        let s_109_0: bool = fn_state.u__SCR_EL3_ECVEn;
        // D s_109_1: cast zx s_109_0 -> bv
        let s_109_1: Bits = Bits::new(s_109_0 as u128, 1u16);
        // C s_109_2: const #1u : u8
        let s_109_2: bool = true;
        // C s_109_3: cast zx s_109_2 -> bv
        let s_109_3: Bits = Bits::new(s_109_2 as u128, 1u16);
        // D s_109_4: cmp-eq s_109_1 s_109_3
        let s_109_4: bool = ((s_109_1) == (s_109_3));
        // D s_109_5: write-var gs#58101 <= s_109_4
        fn_state.gs_58101 = s_109_4;
        // N s_109_6: jump b96
        return block_96(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_110_0: const #() : ()
        let s_110_0: () = ();
        // S s_110_1: call EL2Enabled(s_110_0)
        let s_110_1: bool = EL2Enabled(state, tracer, s_110_0);
        // D s_110_2: write-var gs#58100 <= s_110_1
        fn_state.gs_58100 = s_110_1;
        // N s_110_3: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_111_0: read-var __CNTHP_CTL_EL2_ENABLE:u8
        let s_111_0: bool = fn_state.u__CNTHP_CTL_EL2_ENABLE;
        // D s_111_1: cast zx s_111_0 -> bv
        let s_111_1: Bits = Bits::new(s_111_0 as u128, 1u16);
        // C s_111_2: const #0u : u8
        let s_111_2: bool = false;
        // C s_111_3: cast zx s_111_2 -> bv
        let s_111_3: Bits = Bits::new(s_111_2 as u128, 1u16);
        // D s_111_4: cmp-eq s_111_1 s_111_3
        let s_111_4: bool = ((s_111_1) == (s_111_3));
        // N s_111_5: branch s_111_4 b113 b112
        if s_111_4 {
            return block_113(state, tracer, fn_state);
        } else {
            return block_112(state, tracer, fn_state);
        };
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_112_0: const #64s : i64
        let s_112_0: i64 = 64;
        // C s_112_1: const #16640u : u32
        let s_112_1: u32 = 16640;
        // D s_112_2: read-reg s_112_1:u64
        let s_112_2: u64 = {
            let value = state.read_register::<u64>(s_112_1 as isize);
            tracer.read_register(s_112_1 as isize, value);
            value
        };
        // C s_112_3: const #() : ()
        let s_112_3: () = ();
        // S s_112_4: call PhysicalCountInt(s_112_3)
        let s_112_4: u64 = PhysicalCountInt(state, tracer, s_112_3);
        // D s_112_5: cast zx s_112_2 -> bv
        let s_112_5: Bits = Bits::new(s_112_2 as u128, 64u16);
        // S s_112_6: cast zx s_112_4 -> bv
        let s_112_6: Bits = Bits::new(s_112_4 as u128, 64u16);
        // D s_112_7: sub s_112_5 s_112_6
        let s_112_7: Bits = ((s_112_5) - (s_112_6));
        // D s_112_8: cast reint s_112_7 -> u64
        let s_112_8: u64 = (s_112_7.value() as u64);
        // D s_112_9: call Mk_CNTP_TVAL_EL0_Type(s_112_8)
        let s_112_9: ProductType5c790c8ef59cc8b2 = Mk_CNTP_TVAL_EL0_Type(
            state,
            tracer,
            s_112_8,
        );
        // D s_112_10: call __get_CNTP_TVAL_EL0(s_112_9)
        let s_112_10: ProductType5c790c8ef59cc8b2 = u__get_CNTP_TVAL_EL0(
            state,
            tracer,
            s_112_9,
        );
        // D s_112_11: write-var ga#56727 <= s_112_10
        fn_state.ga_56727 = s_112_10;
        // D s_112_12: read-var ga#56727.0:struct
        let s_112_12: u64 = fn_state.ga_56727._0;
        // D s_112_13: cast zx s_112_12 -> bv
        let s_112_13: Bits = Bits::new(s_112_12 as u128, 64u16);
        // D s_112_14: read-var t:i
        let s_112_14: i128 = fn_state.t;
        // D s_112_15: call X_set(s_112_14, s_112_0, s_112_13)
        let s_112_15: () = X_set(state, tracer, s_112_14, s_112_0, s_112_13);
        // N s_112_16: return
        return;
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_113_0: const #64s : i64
        let s_113_0: i64 = 64;
        // C s_113_1: const #64s : i64
        let s_113_1: i64 = 64;
        // C s_113_2: cast zx s_113_1 -> i
        let s_113_2: i128 = (i128::try_from(s_113_1).unwrap());
        // S s_113_3: call __UNKNOWN_bits(s_113_2)
        let s_113_3: Bits = u__UNKNOWN_bits(state, tracer, s_113_2);
        // S s_113_4: cast reint s_113_3 -> u64
        let s_113_4: u64 = (s_113_3.value() as u64);
        // S s_113_5: cast zx s_113_4 -> bv
        let s_113_5: Bits = Bits::new(s_113_4 as u128, 64u16);
        // D s_113_6: read-var t:i
        let s_113_6: i128 = fn_state.t;
        // D s_113_7: call X_set(s_113_6, s_113_0, s_113_5)
        let s_113_7: () = X_set(state, tracer, s_113_6, s_113_0, s_113_5);
        // N s_113_8: return
        return;
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_114_0: const #90704u : u32
        let s_114_0: u32 = 90704;
        // D s_114_1: read-reg s_114_0:struct
        let s_114_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_114_0 as isize);
            tracer.read_register(s_114_0 as isize, value);
            value
        };
        // D s_114_2: call _get_SCR_EL3_Type_NS(s_114_1)
        let s_114_2: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_114_1);
        // D s_114_3: cast zx s_114_2 -> bv
        let s_114_3: Bits = Bits::new(s_114_2 as u128, 1u16);
        // C s_114_4: const #1u : u8
        let s_114_4: bool = true;
        // C s_114_5: cast zx s_114_4 -> bv
        let s_114_5: Bits = Bits::new(s_114_4 as u128, 1u16);
        // D s_114_6: cmp-eq s_114_3 s_114_5
        let s_114_6: bool = ((s_114_3) == (s_114_5));
        // D s_114_7: write-var gs#58099 <= s_114_6
        fn_state.gs_58099 = s_114_6;
        // N s_114_8: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_115_0: const #102552u : u32
        let s_115_0: u32 = 102552;
        // D s_115_1: read-reg s_115_0:struct
        let s_115_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_115_0 as isize);
            tracer.read_register(s_115_0 as isize, value);
            value
        };
        // D s_115_2: call _get_HCR_EL2_Type_E2H(s_115_1)
        let s_115_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_115_1);
        // C s_115_3: const #102552u : u32
        let s_115_3: u32 = 102552;
        // D s_115_4: read-reg s_115_3:struct
        let s_115_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_115_3 as isize);
            tracer.read_register(s_115_3 as isize, value);
            value
        };
        // D s_115_5: call _get_HCR_EL2_Type_TGE(s_115_4)
        let s_115_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_115_4);
        // D s_115_6: cast zx s_115_2 -> bv
        let s_115_6: Bits = Bits::new(s_115_2 as u128, 1u16);
        // D s_115_7: cast zx s_115_5 -> bv
        let s_115_7: Bits = Bits::new(s_115_5 as u128, 1u16);
        // D s_115_8: cast reint s_115_6 -> u128
        let s_115_8: u128 = (s_115_6.value() as u128);
        // D s_115_9: size-of s_115_6
        let s_115_9: u16 = s_115_6.length();
        // D s_115_10: cast reint s_115_7 -> u128
        let s_115_10: u128 = (s_115_7.value() as u128);
        // D s_115_11: size-of s_115_7
        let s_115_11: u16 = s_115_7.length();
        // D s_115_12: lsl s_115_8 s_115_11
        let s_115_12: u128 = s_115_8 << s_115_11;
        // D s_115_13: or s_115_12 s_115_10
        let s_115_13: u128 = ((s_115_12) | (s_115_10));
        // D s_115_14: add s_115_9 s_115_11
        let s_115_14: u16 = (s_115_9 + s_115_11);
        // D s_115_15: create-bits s_115_13 s_115_14
        let s_115_15: Bits = Bits::new(s_115_13, s_115_14);
        // D s_115_16: cast reint s_115_15 -> u8
        let s_115_16: u8 = (s_115_15.value() as u8);
        // D s_115_17: cast zx s_115_16 -> bv
        let s_115_17: Bits = Bits::new(s_115_16 as u128, 2u16);
        // C s_115_18: const #3u : u8
        let s_115_18: u8 = 3;
        // C s_115_19: cast zx s_115_18 -> bv
        let s_115_19: Bits = Bits::new(s_115_18 as u128, 2u16);
        // D s_115_20: cmp-eq s_115_17 s_115_19
        let s_115_20: bool = ((s_115_17) == (s_115_19));
        // D s_115_21: write-var gs#58098 <= s_115_20
        fn_state.gs_58098 = s_115_20;
        // N s_115_22: jump b89
        return block_89(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_116_0: read-var __CNTHPS_CTL_EL2_ENABLE:u8
        let s_116_0: bool = fn_state.u__CNTHPS_CTL_EL2_ENABLE;
        // D s_116_1: cast zx s_116_0 -> bv
        let s_116_1: Bits = Bits::new(s_116_0 as u128, 1u16);
        // C s_116_2: const #0u : u8
        let s_116_2: bool = false;
        // C s_116_3: cast zx s_116_2 -> bv
        let s_116_3: Bits = Bits::new(s_116_2 as u128, 1u16);
        // D s_116_4: cmp-eq s_116_1 s_116_3
        let s_116_4: bool = ((s_116_1) == (s_116_3));
        // N s_116_5: branch s_116_4 b118 b117
        if s_116_4 {
            return block_118(state, tracer, fn_state);
        } else {
            return block_117(state, tracer, fn_state);
        };
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_117_0: const #64s : i64
        let s_117_0: i64 = 64;
        // C s_117_1: const #22672u : u32
        let s_117_1: u32 = 22672;
        // D s_117_2: read-reg s_117_1:u64
        let s_117_2: u64 = {
            let value = state.read_register::<u64>(s_117_1 as isize);
            tracer.read_register(s_117_1 as isize, value);
            value
        };
        // C s_117_3: const #() : ()
        let s_117_3: () = ();
        // S s_117_4: call PhysicalCountInt(s_117_3)
        let s_117_4: u64 = PhysicalCountInt(state, tracer, s_117_3);
        // D s_117_5: cast zx s_117_2 -> bv
        let s_117_5: Bits = Bits::new(s_117_2 as u128, 64u16);
        // S s_117_6: cast zx s_117_4 -> bv
        let s_117_6: Bits = Bits::new(s_117_4 as u128, 64u16);
        // D s_117_7: sub s_117_5 s_117_6
        let s_117_7: Bits = ((s_117_5) - (s_117_6));
        // D s_117_8: cast reint s_117_7 -> u64
        let s_117_8: u64 = (s_117_7.value() as u64);
        // D s_117_9: call Mk_CNTP_TVAL_EL0_Type(s_117_8)
        let s_117_9: ProductType5c790c8ef59cc8b2 = Mk_CNTP_TVAL_EL0_Type(
            state,
            tracer,
            s_117_8,
        );
        // D s_117_10: call __get_CNTP_TVAL_EL0(s_117_9)
        let s_117_10: ProductType5c790c8ef59cc8b2 = u__get_CNTP_TVAL_EL0(
            state,
            tracer,
            s_117_9,
        );
        // D s_117_11: write-var ga#56709 <= s_117_10
        fn_state.ga_56709 = s_117_10;
        // D s_117_12: read-var ga#56709.0:struct
        let s_117_12: u64 = fn_state.ga_56709._0;
        // D s_117_13: cast zx s_117_12 -> bv
        let s_117_13: Bits = Bits::new(s_117_12 as u128, 64u16);
        // D s_117_14: read-var t:i
        let s_117_14: i128 = fn_state.t;
        // D s_117_15: call X_set(s_117_14, s_117_0, s_117_13)
        let s_117_15: () = X_set(state, tracer, s_117_14, s_117_0, s_117_13);
        // N s_117_16: return
        return;
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_118_0: const #64s : i64
        let s_118_0: i64 = 64;
        // C s_118_1: const #64s : i64
        let s_118_1: i64 = 64;
        // C s_118_2: cast zx s_118_1 -> i
        let s_118_2: i128 = (i128::try_from(s_118_1).unwrap());
        // S s_118_3: call __UNKNOWN_bits(s_118_2)
        let s_118_3: Bits = u__UNKNOWN_bits(state, tracer, s_118_2);
        // S s_118_4: cast reint s_118_3 -> u64
        let s_118_4: u64 = (s_118_3.value() as u64);
        // S s_118_5: cast zx s_118_4 -> bv
        let s_118_5: Bits = Bits::new(s_118_4 as u128, 64u16);
        // D s_118_6: read-var t:i
        let s_118_6: i128 = fn_state.t;
        // D s_118_7: call X_set(s_118_6, s_118_0, s_118_5)
        let s_118_7: () = X_set(state, tracer, s_118_6, s_118_0, s_118_5);
        // N s_118_8: return
        return;
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_119_0: const #117u : u32
        let s_119_0: u32 = 117;
        // S s_119_1: call IsFeatureImplemented(s_119_0)
        let s_119_1: bool = IsFeatureImplemented(state, tracer, s_119_0);
        // D s_119_2: write-var gs#58097 <= s_119_1
        fn_state.gs_58097 = s_119_1;
        // N s_119_3: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_120_0: const #90704u : u32
        let s_120_0: u32 = 90704;
        // D s_120_1: read-reg s_120_0:struct
        let s_120_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_120_0 as isize);
            tracer.read_register(s_120_0 as isize, value);
            value
        };
        // D s_120_2: call _get_SCR_EL3_Type_NS(s_120_1)
        let s_120_2: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_120_1);
        // D s_120_3: cast zx s_120_2 -> bv
        let s_120_3: Bits = Bits::new(s_120_2 as u128, 1u16);
        // C s_120_4: const #0u : u8
        let s_120_4: bool = false;
        // C s_120_5: cast zx s_120_4 -> bv
        let s_120_5: Bits = Bits::new(s_120_4 as u128, 1u16);
        // D s_120_6: cmp-eq s_120_3 s_120_5
        let s_120_6: bool = ((s_120_3) == (s_120_5));
        // D s_120_7: write-var gs#58096 <= s_120_6
        fn_state.gs_58096 = s_120_6;
        // N s_120_8: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_121_0: const #102552u : u32
        let s_121_0: u32 = 102552;
        // D s_121_1: read-reg s_121_0:struct
        let s_121_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_121_0 as isize);
            tracer.read_register(s_121_0 as isize, value);
            value
        };
        // D s_121_2: call _get_HCR_EL2_Type_E2H(s_121_1)
        let s_121_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_121_1);
        // C s_121_3: const #102552u : u32
        let s_121_3: u32 = 102552;
        // D s_121_4: read-reg s_121_3:struct
        let s_121_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_121_3 as isize);
            tracer.read_register(s_121_3 as isize, value);
            value
        };
        // D s_121_5: call _get_HCR_EL2_Type_TGE(s_121_4)
        let s_121_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_121_4);
        // D s_121_6: cast zx s_121_2 -> bv
        let s_121_6: Bits = Bits::new(s_121_2 as u128, 1u16);
        // D s_121_7: cast zx s_121_5 -> bv
        let s_121_7: Bits = Bits::new(s_121_5 as u128, 1u16);
        // D s_121_8: cast reint s_121_6 -> u128
        let s_121_8: u128 = (s_121_6.value() as u128);
        // D s_121_9: size-of s_121_6
        let s_121_9: u16 = s_121_6.length();
        // D s_121_10: cast reint s_121_7 -> u128
        let s_121_10: u128 = (s_121_7.value() as u128);
        // D s_121_11: size-of s_121_7
        let s_121_11: u16 = s_121_7.length();
        // D s_121_12: lsl s_121_8 s_121_11
        let s_121_12: u128 = s_121_8 << s_121_11;
        // D s_121_13: or s_121_12 s_121_10
        let s_121_13: u128 = ((s_121_12) | (s_121_10));
        // D s_121_14: add s_121_9 s_121_11
        let s_121_14: u16 = (s_121_9 + s_121_11);
        // D s_121_15: create-bits s_121_13 s_121_14
        let s_121_15: Bits = Bits::new(s_121_13, s_121_14);
        // D s_121_16: cast reint s_121_15 -> u8
        let s_121_16: u8 = (s_121_15.value() as u8);
        // D s_121_17: cast zx s_121_16 -> bv
        let s_121_17: Bits = Bits::new(s_121_16 as u128, 2u16);
        // C s_121_18: const #3u : u8
        let s_121_18: u8 = 3;
        // C s_121_19: cast zx s_121_18 -> bv
        let s_121_19: Bits = Bits::new(s_121_18 as u128, 2u16);
        // D s_121_20: cmp-eq s_121_17 s_121_19
        let s_121_20: bool = ((s_121_17) == (s_121_19));
        // D s_121_21: write-var gs#58095 <= s_121_20
        fn_state.gs_58095 = s_121_20;
        // N s_121_22: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_122_0: const #24u : u8
        let s_122_0: u8 = 24;
        // C s_122_1: cast zx s_122_0 -> bv
        let s_122_1: Bits = Bits::new(s_122_0 as u128, 8u16);
        // C s_122_2: cast zx s_122_1 -> i
        let s_122_2: i128 = (s_122_1.value() as i128);
        // C s_122_3: cast reint s_122_2 -> i64
        let s_122_3: i64 = (s_122_2 as i64);
        // C s_122_4: cast zx s_122_3 -> i
        let s_122_4: i128 = (i128::try_from(s_122_3).unwrap());
        // C s_122_5: const #432u : u32
        let s_122_5: u32 = 432;
        // D s_122_6: read-reg s_122_5:u8
        let s_122_6: u8 = {
            let value = state.read_register::<u8>(s_122_5 as isize);
            tracer.read_register(s_122_5 as isize, value);
            value
        };
        // D s_122_7: call AArch64_SystemAccessTrap(s_122_6, s_122_4)
        let s_122_7: () = AArch64_SystemAccessTrap(state, tracer, s_122_6, s_122_4);
        // N s_122_8: return
        return;
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_123_0: read-var __CNTHCTL_EL2_EL0PTEN:u8
        let s_123_0: bool = fn_state.u__CNTHCTL_EL2_EL0PTEN;
        // D s_123_1: cast zx s_123_0 -> bv
        let s_123_1: Bits = Bits::new(s_123_0 as u128, 1u16);
        // C s_123_2: const #0u : u8
        let s_123_2: bool = false;
        // C s_123_3: cast zx s_123_2 -> bv
        let s_123_3: Bits = Bits::new(s_123_2 as u128, 1u16);
        // D s_123_4: cmp-eq s_123_1 s_123_3
        let s_123_4: bool = ((s_123_1) == (s_123_3));
        // D s_123_5: write-var gs#58094 <= s_123_4
        fn_state.gs_58094 = s_123_4;
        // N s_123_6: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_124_0: const #102552u : u32
        let s_124_0: u32 = 102552;
        // D s_124_1: read-reg s_124_0:struct
        let s_124_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_124_0 as isize);
            tracer.read_register(s_124_0 as isize, value);
            value
        };
        // D s_124_2: call _get_HCR_EL2_Type_E2H(s_124_1)
        let s_124_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_124_1);
        // C s_124_3: const #102552u : u32
        let s_124_3: u32 = 102552;
        // D s_124_4: read-reg s_124_3:struct
        let s_124_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_124_3 as isize);
            tracer.read_register(s_124_3 as isize, value);
            value
        };
        // D s_124_5: call _get_HCR_EL2_Type_TGE(s_124_4)
        let s_124_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_124_4);
        // D s_124_6: cast zx s_124_2 -> bv
        let s_124_6: Bits = Bits::new(s_124_2 as u128, 1u16);
        // D s_124_7: cast zx s_124_5 -> bv
        let s_124_7: Bits = Bits::new(s_124_5 as u128, 1u16);
        // D s_124_8: cast reint s_124_6 -> u128
        let s_124_8: u128 = (s_124_6.value() as u128);
        // D s_124_9: size-of s_124_6
        let s_124_9: u16 = s_124_6.length();
        // D s_124_10: cast reint s_124_7 -> u128
        let s_124_10: u128 = (s_124_7.value() as u128);
        // D s_124_11: size-of s_124_7
        let s_124_11: u16 = s_124_7.length();
        // D s_124_12: lsl s_124_8 s_124_11
        let s_124_12: u128 = s_124_8 << s_124_11;
        // D s_124_13: or s_124_12 s_124_10
        let s_124_13: u128 = ((s_124_12) | (s_124_10));
        // D s_124_14: add s_124_9 s_124_11
        let s_124_14: u16 = (s_124_9 + s_124_11);
        // D s_124_15: create-bits s_124_13 s_124_14
        let s_124_15: Bits = Bits::new(s_124_13, s_124_14);
        // D s_124_16: cast reint s_124_15 -> u8
        let s_124_16: u8 = (s_124_15.value() as u8);
        // D s_124_17: cast zx s_124_16 -> bv
        let s_124_17: Bits = Bits::new(s_124_16 as u128, 2u16);
        // C s_124_18: const #3u : u8
        let s_124_18: u8 = 3;
        // C s_124_19: cast zx s_124_18 -> bv
        let s_124_19: Bits = Bits::new(s_124_18 as u128, 2u16);
        // D s_124_20: cmp-eq s_124_17 s_124_19
        let s_124_20: bool = ((s_124_17) == (s_124_19));
        // D s_124_21: write-var gs#58093 <= s_124_20
        fn_state.gs_58093 = s_124_20;
        // N s_124_22: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_125_0: const #24u : u8
        let s_125_0: u8 = 24;
        // C s_125_1: cast zx s_125_0 -> bv
        let s_125_1: Bits = Bits::new(s_125_0 as u128, 8u16);
        // C s_125_2: cast zx s_125_1 -> i
        let s_125_2: i128 = (s_125_1.value() as i128);
        // C s_125_3: cast reint s_125_2 -> i64
        let s_125_3: i64 = (s_125_2 as i64);
        // C s_125_4: cast zx s_125_3 -> i
        let s_125_4: i128 = (i128::try_from(s_125_3).unwrap());
        // C s_125_5: const #432u : u32
        let s_125_5: u32 = 432;
        // D s_125_6: read-reg s_125_5:u8
        let s_125_6: u8 = {
            let value = state.read_register::<u8>(s_125_5 as isize);
            tracer.read_register(s_125_5 as isize, value);
            value
        };
        // D s_125_7: call AArch64_SystemAccessTrap(s_125_6, s_125_4)
        let s_125_7: () = AArch64_SystemAccessTrap(state, tracer, s_125_6, s_125_4);
        // N s_125_8: return
        return;
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_126_0: read-var __CNTHCTL_EL2_EL1PTEN:u8
        let s_126_0: bool = fn_state.u__CNTHCTL_EL2_EL1PTEN;
        // D s_126_1: cast zx s_126_0 -> bv
        let s_126_1: Bits = Bits::new(s_126_0 as u128, 1u16);
        // C s_126_2: const #0u : u8
        let s_126_2: bool = false;
        // C s_126_3: cast zx s_126_2 -> bv
        let s_126_3: Bits = Bits::new(s_126_2 as u128, 1u16);
        // D s_126_4: cmp-eq s_126_1 s_126_3
        let s_126_4: bool = ((s_126_1) == (s_126_3));
        // D s_126_5: write-var gs#58092 <= s_126_4
        fn_state.gs_58092 = s_126_4;
        // N s_126_6: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_127_0: const #102552u : u32
        let s_127_0: u32 = 102552;
        // D s_127_1: read-reg s_127_0:struct
        let s_127_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_127_0 as isize);
            tracer.read_register(s_127_0 as isize, value);
            value
        };
        // D s_127_2: call _get_HCR_EL2_Type_E2H(s_127_1)
        let s_127_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_127_1);
        // C s_127_3: const #102552u : u32
        let s_127_3: u32 = 102552;
        // D s_127_4: read-reg s_127_3:struct
        let s_127_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_127_3 as isize);
            tracer.read_register(s_127_3 as isize, value);
            value
        };
        // D s_127_5: call _get_HCR_EL2_Type_TGE(s_127_4)
        let s_127_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_127_4);
        // D s_127_6: cast zx s_127_2 -> bv
        let s_127_6: Bits = Bits::new(s_127_2 as u128, 1u16);
        // D s_127_7: cast zx s_127_5 -> bv
        let s_127_7: Bits = Bits::new(s_127_5 as u128, 1u16);
        // D s_127_8: cast reint s_127_6 -> u128
        let s_127_8: u128 = (s_127_6.value() as u128);
        // D s_127_9: size-of s_127_6
        let s_127_9: u16 = s_127_6.length();
        // D s_127_10: cast reint s_127_7 -> u128
        let s_127_10: u128 = (s_127_7.value() as u128);
        // D s_127_11: size-of s_127_7
        let s_127_11: u16 = s_127_7.length();
        // D s_127_12: lsl s_127_8 s_127_11
        let s_127_12: u128 = s_127_8 << s_127_11;
        // D s_127_13: or s_127_12 s_127_10
        let s_127_13: u128 = ((s_127_12) | (s_127_10));
        // D s_127_14: add s_127_9 s_127_11
        let s_127_14: u16 = (s_127_9 + s_127_11);
        // D s_127_15: create-bits s_127_13 s_127_14
        let s_127_15: Bits = Bits::new(s_127_13, s_127_14);
        // D s_127_16: cast reint s_127_15 -> u8
        let s_127_16: u8 = (s_127_15.value() as u8);
        // D s_127_17: cast zx s_127_16 -> bv
        let s_127_17: Bits = Bits::new(s_127_16 as u128, 2u16);
        // C s_127_18: const #2u : u8
        let s_127_18: u8 = 2;
        // C s_127_19: cast zx s_127_18 -> bv
        let s_127_19: Bits = Bits::new(s_127_18 as u128, 2u16);
        // D s_127_20: cmp-eq s_127_17 s_127_19
        let s_127_20: bool = ((s_127_17) == (s_127_19));
        // D s_127_21: write-var gs#58091 <= s_127_20
        fn_state.gs_58091 = s_127_20;
        // N s_127_22: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_128_0: const #24u : u8
        let s_128_0: u8 = 24;
        // C s_128_1: cast zx s_128_0 -> bv
        let s_128_1: Bits = Bits::new(s_128_0 as u128, 8u16);
        // C s_128_2: cast zx s_128_1 -> i
        let s_128_2: i128 = (s_128_1.value() as i128);
        // C s_128_3: cast reint s_128_2 -> i64
        let s_128_3: i64 = (s_128_2 as i64);
        // C s_128_4: cast zx s_128_3 -> i
        let s_128_4: i128 = (i128::try_from(s_128_3).unwrap());
        // C s_128_5: const #432u : u32
        let s_128_5: u32 = 432;
        // D s_128_6: read-reg s_128_5:u8
        let s_128_6: u8 = {
            let value = state.read_register::<u8>(s_128_5 as isize);
            tracer.read_register(s_128_5 as isize, value);
            value
        };
        // D s_128_7: call AArch64_SystemAccessTrap(s_128_6, s_128_4)
        let s_128_7: () = AArch64_SystemAccessTrap(state, tracer, s_128_6, s_128_4);
        // N s_128_8: return
        return;
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_129_0: read-var __CNTHCTL_EL2_EL1PCEN:u8
        let s_129_0: bool = fn_state.u__CNTHCTL_EL2_EL1PCEN;
        // D s_129_1: cast zx s_129_0 -> bv
        let s_129_1: Bits = Bits::new(s_129_0 as u128, 1u16);
        // C s_129_2: const #0u : u8
        let s_129_2: bool = false;
        // C s_129_3: cast zx s_129_2 -> bv
        let s_129_3: Bits = Bits::new(s_129_2 as u128, 1u16);
        // D s_129_4: cmp-eq s_129_1 s_129_3
        let s_129_4: bool = ((s_129_1) == (s_129_3));
        // D s_129_5: write-var gs#58090 <= s_129_4
        fn_state.gs_58090 = s_129_4;
        // N s_129_6: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_130_0: const #102552u : u32
        let s_130_0: u32 = 102552;
        // D s_130_1: read-reg s_130_0:struct
        let s_130_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_130_0 as isize);
            tracer.read_register(s_130_0 as isize, value);
            value
        };
        // D s_130_2: call _get_HCR_EL2_Type_E2H(s_130_1)
        let s_130_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_130_1);
        // D s_130_3: cast zx s_130_2 -> bv
        let s_130_3: Bits = Bits::new(s_130_2 as u128, 1u16);
        // C s_130_4: const #0u : u8
        let s_130_4: bool = false;
        // C s_130_5: cast zx s_130_4 -> bv
        let s_130_5: Bits = Bits::new(s_130_4 as u128, 1u16);
        // D s_130_6: cmp-eq s_130_3 s_130_5
        let s_130_6: bool = ((s_130_3) == (s_130_5));
        // D s_130_7: write-var gs#58089 <= s_130_6
        fn_state.gs_58089 = s_130_6;
        // N s_130_8: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_131_0: const #() : ()
        let s_131_0: () = ();
        // S s_131_1: call EL2Enabled(s_131_0)
        let s_131_1: bool = EL2Enabled(state, tracer, s_131_0);
        // N s_131_2: branch s_131_1 b136 b132
        if s_131_1 {
            return block_136(state, tracer, fn_state);
        } else {
            return block_132(state, tracer, fn_state);
        };
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_132_0: const #0u : u8
        let s_132_0: bool = false;
        // D s_132_1: write-var gs#58116 <= s_132_0
        fn_state.gs_58116 = s_132_0;
        // N s_132_2: jump b133
        return block_133(state, tracer, fn_state);
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_133_0: read-var gs#58116:u8
        let s_133_0: bool = fn_state.gs_58116;
        // N s_133_1: branch s_133_0 b135 b134
        if s_133_0 {
            return block_135(state, tracer, fn_state);
        } else {
            return block_134(state, tracer, fn_state);
        };
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_134_0: const #24u : u8
        let s_134_0: u8 = 24;
        // C s_134_1: cast zx s_134_0 -> bv
        let s_134_1: Bits = Bits::new(s_134_0 as u128, 8u16);
        // C s_134_2: cast zx s_134_1 -> i
        let s_134_2: i128 = (s_134_1.value() as i128);
        // C s_134_3: cast reint s_134_2 -> i64
        let s_134_3: i64 = (s_134_2 as i64);
        // C s_134_4: cast zx s_134_3 -> i
        let s_134_4: i128 = (i128::try_from(s_134_3).unwrap());
        // C s_134_5: const #440u : u32
        let s_134_5: u32 = 440;
        // D s_134_6: read-reg s_134_5:u8
        let s_134_6: u8 = {
            let value = state.read_register::<u8>(s_134_5 as isize);
            tracer.read_register(s_134_5 as isize, value);
            value
        };
        // D s_134_7: call AArch64_SystemAccessTrap(s_134_6, s_134_4)
        let s_134_7: () = AArch64_SystemAccessTrap(state, tracer, s_134_6, s_134_4);
        // N s_134_8: return
        return;
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_135_0: const #24u : u8
        let s_135_0: u8 = 24;
        // C s_135_1: cast zx s_135_0 -> bv
        let s_135_1: Bits = Bits::new(s_135_0 as u128, 8u16);
        // C s_135_2: cast zx s_135_1 -> i
        let s_135_2: i128 = (s_135_1.value() as i128);
        // C s_135_3: cast reint s_135_2 -> i64
        let s_135_3: i64 = (s_135_2 as i64);
        // C s_135_4: cast zx s_135_3 -> i
        let s_135_4: i128 = (i128::try_from(s_135_3).unwrap());
        // C s_135_5: const #432u : u32
        let s_135_5: u32 = 432;
        // D s_135_6: read-reg s_135_5:u8
        let s_135_6: u8 = {
            let value = state.read_register::<u8>(s_135_5 as isize);
            tracer.read_register(s_135_5 as isize, value);
            value
        };
        // D s_135_7: call AArch64_SystemAccessTrap(s_135_6, s_135_4)
        let s_135_7: () = AArch64_SystemAccessTrap(state, tracer, s_135_6, s_135_4);
        // N s_135_8: return
        return;
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_136_0: read-var __HCR_EL2_TGE:u8
        let s_136_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_136_1: cast zx s_136_0 -> bv
        let s_136_1: Bits = Bits::new(s_136_0 as u128, 1u16);
        // C s_136_2: const #1u : u8
        let s_136_2: bool = true;
        // C s_136_3: cast zx s_136_2 -> bv
        let s_136_3: Bits = Bits::new(s_136_2 as u128, 1u16);
        // D s_136_4: cmp-eq s_136_1 s_136_3
        let s_136_4: bool = ((s_136_1) == (s_136_3));
        // D s_136_5: write-var gs#58116 <= s_136_4
        fn_state.gs_58116 = s_136_4;
        // N s_136_6: jump b133
        return block_133(state, tracer, fn_state);
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_137_0: read-var __CNTKCTL_EL1_EL0PTEN:u8
        let s_137_0: bool = fn_state.u__CNTKCTL_EL1_EL0PTEN;
        // D s_137_1: cast zx s_137_0 -> bv
        let s_137_1: Bits = Bits::new(s_137_0 as u128, 1u16);
        // C s_137_2: const #0u : u8
        let s_137_2: bool = false;
        // C s_137_3: cast zx s_137_2 -> bv
        let s_137_3: Bits = Bits::new(s_137_2 as u128, 1u16);
        // D s_137_4: cmp-eq s_137_1 s_137_3
        let s_137_4: bool = ((s_137_1) == (s_137_3));
        // D s_137_5: write-var gs#58088 <= s_137_4
        fn_state.gs_58088 = s_137_4;
        // N s_137_6: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_138_0: const #102552u : u32
        let s_138_0: u32 = 102552;
        // D s_138_1: read-reg s_138_0:struct
        let s_138_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_138_0 as isize);
            tracer.read_register(s_138_0 as isize, value);
            value
        };
        // D s_138_2: call _get_HCR_EL2_Type_E2H(s_138_1)
        let s_138_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_138_1);
        // C s_138_3: const #102552u : u32
        let s_138_3: u32 = 102552;
        // D s_138_4: read-reg s_138_3:struct
        let s_138_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_138_3 as isize);
            tracer.read_register(s_138_3 as isize, value);
            value
        };
        // D s_138_5: call _get_HCR_EL2_Type_TGE(s_138_4)
        let s_138_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_138_4);
        // D s_138_6: cast zx s_138_2 -> bv
        let s_138_6: Bits = Bits::new(s_138_2 as u128, 1u16);
        // D s_138_7: cast zx s_138_5 -> bv
        let s_138_7: Bits = Bits::new(s_138_5 as u128, 1u16);
        // D s_138_8: cast reint s_138_6 -> u128
        let s_138_8: u128 = (s_138_6.value() as u128);
        // D s_138_9: size-of s_138_6
        let s_138_9: u16 = s_138_6.length();
        // D s_138_10: cast reint s_138_7 -> u128
        let s_138_10: u128 = (s_138_7.value() as u128);
        // D s_138_11: size-of s_138_7
        let s_138_11: u16 = s_138_7.length();
        // D s_138_12: lsl s_138_8 s_138_11
        let s_138_12: u128 = s_138_8 << s_138_11;
        // D s_138_13: or s_138_12 s_138_10
        let s_138_13: u128 = ((s_138_12) | (s_138_10));
        // D s_138_14: add s_138_9 s_138_11
        let s_138_14: u16 = (s_138_9 + s_138_11);
        // D s_138_15: create-bits s_138_13 s_138_14
        let s_138_15: Bits = Bits::new(s_138_13, s_138_14);
        // D s_138_16: cast reint s_138_15 -> u8
        let s_138_16: u8 = (s_138_15.value() as u8);
        // D s_138_17: cast zx s_138_16 -> bv
        let s_138_17: Bits = Bits::new(s_138_16 as u128, 2u16);
        // C s_138_18: const #3u : u8
        let s_138_18: u8 = 3;
        // C s_138_19: cast zx s_138_18 -> bv
        let s_138_19: Bits = Bits::new(s_138_18 as u128, 2u16);
        // D s_138_20: cmp-eq s_138_17 s_138_19
        let s_138_20: bool = ((s_138_17) == (s_138_19));
        // D s_138_21: write-var gs#58087 <= s_138_20
        fn_state.gs_58087 = s_138_20;
        // N s_138_22: jump b62
        return block_62(state, tracer, fn_state);
    }
}
