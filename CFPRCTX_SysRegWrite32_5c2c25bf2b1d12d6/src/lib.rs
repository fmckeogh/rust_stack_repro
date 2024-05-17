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
use AArch32_RestrictPrediction::*;
use u_get_SCTLR_EL2_Type_EnRCTX::*;
use u_get_SCR_EL3_Type_FGTEn::*;
use AArch32_TakeHypTrapException::*;
use u_get_HCR_EL2_Type_E2H::*;
use u_get_SCTLR_Type_EnRCTX::*;
use HCR_read::*;
use IsFeatureImplemented::*;
use u_get_HSTR_EL2_Type_T7::*;
use AArch64_SystemAccessTrap::*;
use AArch64_AArch32SystemAccessTrap::*;
use HSTR_read::*;
use u_get_HCR_Type_TGE::*;
use SCTLR_read__2::*;
use R_read::*;
use u_get_SCTLR_EL1_Type_EnRCTX::*;
use u_get_HCR_EL2_Type_NV::*;
use u_get_HFGITR_EL2_Type_CFPRCTX::*;
use ELUsingAArch32::*;
use u_get_HCR_EL2_Type_TGE::*;
use EL2Enabled::*;
use u_get_HSTR_Type_T7::*;
use common::*;
pub fn CFPRCTX_SysRegWrite32_5c2c25bf2b1d12d6<T: Tracer>(
    state: &mut State,
    tracer: &T,
    el: u8,
    coproc: u8,
    opc1: u8,
    CRn: u8,
    opc2: u8,
    CRm: u8,
    t: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_124241: bool,
        gs_124237: bool,
        gs_124238: bool,
        gs_124243: bool,
        gs_124250: bool,
        u__HCR_EL2_TGE: bool,
        gs_124249: bool,
        gs_124236: bool,
        gs_124247: bool,
        u__SCR_EL3_FGTEn: bool,
        gs_124222: bool,
        gs_124226: bool,
        u__SCTLR_EL1_EnRCTX: bool,
        gs_124225: bool,
        gs_124221: bool,
        gs_124244: bool,
        gs_124224: bool,
        u__HSTR_EL2_T7: bool,
        u__SCTLR_EnRCTX: bool,
        u__HCR_TGE: bool,
        u__HFGITR_EL2_CFPRCTX: bool,
        u__HSTR_T7: bool,
        gs_124228: bool,
        gs_124239: bool,
        gs_124229: bool,
        gs_124232: bool,
        gs_124223: bool,
        gs_124235: bool,
        gs_124227: bool,
        gs_124242: bool,
        gs_124245: bool,
        gs_124231: bool,
        gs_124234: bool,
        u__SCTLR_EL2_EnRCTX: bool,
        u__PSTATE_EL: u8,
        gs_124248: bool,
        gs_124246: bool,
        gs_124230: bool,
        u__HCR_EL2_NV: bool,
        gs_124233: bool,
        gs_124240: bool,
        el: u8,
        coproc: u8,
        opc1: u8,
        CRn: u8,
        opc2: u8,
        CRm: u8,
        t: i128,
    }
    let fn_state = FunctionState {
        el,
        coproc,
        opc1,
        CRn,
        opc2,
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
        // C s_0_3: const #90272u : u32
        let s_0_3: u32 = 90272;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_SCTLR_EL1_Type_EnRCTX(s_0_4)
        let s_0_5: bool = u_get_SCTLR_EL1_Type_EnRCTX(state, tracer, s_0_4);
        // D s_0_6: write-var __SCTLR_EL1_EnRCTX <= s_0_5
        fn_state.u__SCTLR_EL1_EnRCTX = s_0_5;
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
        // C s_0_11: const #() : ()
        let s_0_11: () = ();
        // S s_0_12: call SCTLR_read__2(s_0_11)
        let s_0_12: ProductType700c18a878c5601b = SCTLR_read__2(state, tracer, s_0_11);
        // S s_0_13: call _get_SCTLR_Type_EnRCTX(s_0_12)
        let s_0_13: bool = u_get_SCTLR_Type_EnRCTX(state, tracer, s_0_12);
        // D s_0_14: write-var __SCTLR_EnRCTX <= s_0_13
        fn_state.u__SCTLR_EnRCTX = s_0_13;
        // C s_0_15: const #() : ()
        let s_0_15: () = ();
        // S s_0_16: call HCR_read(s_0_15)
        let s_0_16: ProductType700c18a878c5601b = HCR_read(state, tracer, s_0_15);
        // S s_0_17: call _get_HCR_Type_TGE(s_0_16)
        let s_0_17: bool = u_get_HCR_Type_TGE(state, tracer, s_0_16);
        // D s_0_18: write-var __HCR_TGE <= s_0_17
        fn_state.u__HCR_TGE = s_0_17;
        // C s_0_19: const #104936u : u32
        let s_0_19: u32 = 104936;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_HSTR_EL2_Type_T7(s_0_20)
        let s_0_21: bool = u_get_HSTR_EL2_Type_T7(state, tracer, s_0_20);
        // D s_0_22: write-var __HSTR_EL2_T7 <= s_0_21
        fn_state.u__HSTR_EL2_T7 = s_0_21;
        // C s_0_23: const #() : ()
        let s_0_23: () = ();
        // S s_0_24: call HSTR_read(s_0_23)
        let s_0_24: ProductType700c18a878c5601b = HSTR_read(state, tracer, s_0_23);
        // S s_0_25: call _get_HSTR_Type_T7(s_0_24)
        let s_0_25: bool = u_get_HSTR_Type_T7(state, tracer, s_0_24);
        // D s_0_26: write-var __HSTR_T7 <= s_0_25
        fn_state.u__HSTR_T7 = s_0_25;
        // C s_0_27: const #90704u : u32
        let s_0_27: u32 = 90704;
        // D s_0_28: read-reg s_0_27:struct
        let s_0_28: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_27 as isize);
            tracer.read_register(s_0_27 as isize, value);
            value
        };
        // D s_0_29: call _get_SCR_EL3_Type_FGTEn(s_0_28)
        let s_0_29: bool = u_get_SCR_EL3_Type_FGTEn(state, tracer, s_0_28);
        // D s_0_30: write-var __SCR_EL3_FGTEn <= s_0_29
        fn_state.u__SCR_EL3_FGTEn = s_0_29;
        // C s_0_31: const #13608u : u32
        let s_0_31: u32 = 13608;
        // D s_0_32: read-reg s_0_31:struct
        let s_0_32: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_31 as isize);
            tracer.read_register(s_0_31 as isize, value);
            value
        };
        // D s_0_33: call _get_HFGITR_EL2_Type_CFPRCTX(s_0_32)
        let s_0_33: bool = u_get_HFGITR_EL2_Type_CFPRCTX(state, tracer, s_0_32);
        // D s_0_34: write-var __HFGITR_EL2_CFPRCTX <= s_0_33
        fn_state.u__HFGITR_EL2_CFPRCTX = s_0_33;
        // C s_0_35: const #20784u : u32
        let s_0_35: u32 = 20784;
        // D s_0_36: read-reg s_0_35:struct
        let s_0_36: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_35 as isize);
            tracer.read_register(s_0_35 as isize, value);
            value
        };
        // D s_0_37: call _get_SCTLR_EL2_Type_EnRCTX(s_0_36)
        let s_0_37: bool = u_get_SCTLR_EL2_Type_EnRCTX(state, tracer, s_0_36);
        // D s_0_38: write-var __SCTLR_EL2_EnRCTX <= s_0_37
        fn_state.u__SCTLR_EL2_EnRCTX = s_0_37;
        // C s_0_39: const #102552u : u32
        let s_0_39: u32 = 102552;
        // D s_0_40: read-reg s_0_39:struct
        let s_0_40: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_39 as isize);
            tracer.read_register(s_0_39 as isize, value);
            value
        };
        // D s_0_41: call _get_HCR_EL2_Type_NV(s_0_40)
        let s_0_41: bool = u_get_HCR_EL2_Type_NV(state, tracer, s_0_40);
        // D s_0_42: write-var __HCR_EL2_NV <= s_0_41
        fn_state.u__HCR_EL2_NV = s_0_41;
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
        // N s_0_49: branch s_0_48 b32 b1
        if s_0_48 {
            return block_32(state, tracer, fn_state);
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
        // D s_5_0: read-var t:i
        let s_5_0: i128 = fn_state.t;
        // D s_5_1: call R_read(s_5_0)
        let s_5_1: u32 = R_read(state, tracer, s_5_0);
        // C s_5_2: const #1u : u32
        let s_5_2: u32 = 1;
        // D s_5_3: call AArch32_RestrictPrediction(s_5_1, s_5_2)
        let s_5_3: () = AArch32_RestrictPrediction(state, tracer, s_5_1, s_5_2);
        // N s_5_4: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var t:i
        let s_6_0: i128 = fn_state.t;
        // D s_6_1: call R_read(s_6_0)
        let s_6_1: u32 = R_read(state, tracer, s_6_0);
        // C s_6_2: const #1u : u32
        let s_6_2: u32 = 1;
        // D s_6_3: call AArch32_RestrictPrediction(s_6_1, s_6_2)
        let s_6_3: () = AArch32_RestrictPrediction(state, tracer, s_6_1, s_6_2);
        // N s_6_4: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #() : ()
        let s_7_0: () = ();
        // S s_7_1: call EL2Enabled(s_7_0)
        let s_7_1: bool = EL2Enabled(state, tracer, s_7_0);
        // N s_7_2: branch s_7_1 b31 b8
        if s_7_1 {
            return block_31(state, tracer, fn_state);
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
        // D s_8_1: write-var gs#124221 <= s_8_0
        fn_state.gs_124221 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#124221:u8
        let s_9_0: bool = fn_state.gs_124221;
        // N s_9_1: branch s_9_0 b30 b10
        if s_9_0 {
            return block_30(state, tracer, fn_state);
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
        // D s_10_1: write-var gs#124222 <= s_10_0
        fn_state.gs_124222 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#124222:u8
        let s_11_0: bool = fn_state.gs_124222;
        // N s_11_1: branch s_11_0 b29 b12
        if s_11_0 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #() : ()
        let s_12_0: () = ();
        // S s_12_1: call EL2Enabled(s_12_0)
        let s_12_1: bool = EL2Enabled(state, tracer, s_12_0);
        // N s_12_2: branch s_12_1 b28 b13
        if s_12_1 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #0u : u8
        let s_13_0: bool = false;
        // D s_13_1: write-var gs#124223 <= s_13_0
        fn_state.gs_124223 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#124223:u8
        let s_14_0: bool = fn_state.gs_124223;
        // N s_14_1: branch s_14_0 b27 b15
        if s_14_0 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // D s_15_1: write-var gs#124224 <= s_15_0
        fn_state.gs_124224 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#124224:u8
        let s_16_0: bool = fn_state.gs_124224;
        // N s_16_1: branch s_16_0 b26 b17
        if s_16_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #() : ()
        let s_17_0: () = ();
        // S s_17_1: call EL2Enabled(s_17_0)
        let s_17_1: bool = EL2Enabled(state, tracer, s_17_0);
        // N s_17_2: branch s_17_1 b25 b18
        if s_17_1 {
            return block_25(state, tracer, fn_state);
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
        // D s_18_1: write-var gs#124225 <= s_18_0
        fn_state.gs_124225 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#124225:u8
        let s_19_0: bool = fn_state.gs_124225;
        // N s_19_1: branch s_19_0 b24 b20
        if s_19_0 {
            return block_24(state, tracer, fn_state);
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
        // D s_20_1: write-var gs#124226 <= s_20_0
        fn_state.gs_124226 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#124226:u8
        let s_21_0: bool = fn_state.gs_124226;
        // N s_21_1: branch s_21_0 b23 b22
        if s_21_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var t:i
        let s_22_0: i128 = fn_state.t;
        // D s_22_1: call R_read(s_22_0)
        let s_22_1: u32 = R_read(state, tracer, s_22_0);
        // C s_22_2: const #1u : u32
        let s_22_2: u32 = 1;
        // D s_22_3: call AArch32_RestrictPrediction(s_22_1, s_22_2)
        let s_22_3: () = AArch32_RestrictPrediction(state, tracer, s_22_1, s_22_2);
        // N s_22_4: return
        return;
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #3u : u8
        let s_23_0: u8 = 3;
        // C s_23_1: cast zx s_23_0 -> bv
        let s_23_1: Bits = Bits::new(s_23_0 as u128, 8u16);
        // C s_23_2: cast zx s_23_1 -> i
        let s_23_2: i128 = (s_23_1.value() as i128);
        // C s_23_3: cast reint s_23_2 -> i64
        let s_23_3: i64 = (s_23_2 as i64);
        // C s_23_4: cast zx s_23_3 -> i
        let s_23_4: i128 = (i128::try_from(s_23_3).unwrap());
        // C s_23_5: const #432u : u32
        let s_23_5: u32 = 432;
        // D s_23_6: read-reg s_23_5:u8
        let s_23_6: u8 = {
            let value = state.read_register::<u8>(s_23_5 as isize);
            tracer.read_register(s_23_5 as isize, value);
            value
        };
        // D s_23_7: call AArch64_SystemAccessTrap(s_23_6, s_23_4)
        let s_23_7: () = AArch64_SystemAccessTrap(state, tracer, s_23_6, s_23_4);
        // N s_23_8: return
        return;
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var __HCR_EL2_NV:u8
        let s_24_0: bool = fn_state.u__HCR_EL2_NV;
        // D s_24_1: cast zx s_24_0 -> bv
        let s_24_1: Bits = Bits::new(s_24_0 as u128, 1u16);
        // C s_24_2: const #1u : u8
        let s_24_2: bool = true;
        // C s_24_3: cast zx s_24_2 -> bv
        let s_24_3: Bits = Bits::new(s_24_2 as u128, 1u16);
        // D s_24_4: cmp-eq s_24_1 s_24_3
        let s_24_4: bool = ((s_24_1) == (s_24_3));
        // D s_24_5: write-var gs#124226 <= s_24_4
        fn_state.gs_124226 = s_24_4;
        // N s_24_6: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #432u : u32
        let s_25_0: u32 = 432;
        // D s_25_1: read-reg s_25_0:u8
        let s_25_1: u8 = {
            let value = state.read_register::<u8>(s_25_0 as isize);
            tracer.read_register(s_25_0 as isize, value);
            value
        };
        // D s_25_2: call ELUsingAArch32(s_25_1)
        let s_25_2: bool = ELUsingAArch32(state, tracer, s_25_1);
        // D s_25_3: not s_25_2
        let s_25_3: bool = !s_25_2;
        // D s_25_4: write-var gs#124225 <= s_25_3
        fn_state.gs_124225 = s_25_3;
        // N s_25_5: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #3u : u8
        let s_26_0: u8 = 3;
        // C s_26_1: cast zx s_26_0 -> bv
        let s_26_1: Bits = Bits::new(s_26_0 as u128, 8u16);
        // C s_26_2: cast zx s_26_1 -> i
        let s_26_2: i128 = (s_26_1.value() as i128);
        // C s_26_3: cast reint s_26_2 -> i64
        let s_26_3: i64 = (s_26_2 as i64);
        // C s_26_4: cast zx s_26_3 -> i
        let s_26_4: i128 = (i128::try_from(s_26_3).unwrap());
        // S s_26_5: call AArch32_TakeHypTrapException(s_26_4)
        let s_26_5: () = AArch32_TakeHypTrapException(state, tracer, s_26_4);
        // N s_26_6: return
        return;
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var __HSTR_T7:u8
        let s_27_0: bool = fn_state.u__HSTR_T7;
        // D s_27_1: cast zx s_27_0 -> bv
        let s_27_1: Bits = Bits::new(s_27_0 as u128, 1u16);
        // C s_27_2: const #1u : u8
        let s_27_2: bool = true;
        // C s_27_3: cast zx s_27_2 -> bv
        let s_27_3: Bits = Bits::new(s_27_2 as u128, 1u16);
        // D s_27_4: cmp-eq s_27_1 s_27_3
        let s_27_4: bool = ((s_27_1) == (s_27_3));
        // D s_27_5: write-var gs#124224 <= s_27_4
        fn_state.gs_124224 = s_27_4;
        // N s_27_6: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #432u : u32
        let s_28_0: u32 = 432;
        // D s_28_1: read-reg s_28_0:u8
        let s_28_1: u8 = {
            let value = state.read_register::<u8>(s_28_0 as isize);
            tracer.read_register(s_28_0 as isize, value);
            value
        };
        // D s_28_2: call ELUsingAArch32(s_28_1)
        let s_28_2: bool = ELUsingAArch32(state, tracer, s_28_1);
        // D s_28_3: write-var gs#124223 <= s_28_2
        fn_state.gs_124223 = s_28_2;
        // N s_28_4: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #3u : u8
        let s_29_0: u8 = 3;
        // C s_29_1: cast zx s_29_0 -> bv
        let s_29_1: Bits = Bits::new(s_29_0 as u128, 8u16);
        // C s_29_2: cast zx s_29_1 -> i
        let s_29_2: i128 = (s_29_1.value() as i128);
        // C s_29_3: cast reint s_29_2 -> i64
        let s_29_3: i64 = (s_29_2 as i64);
        // C s_29_4: cast zx s_29_3 -> i
        let s_29_4: i128 = (i128::try_from(s_29_3).unwrap());
        // C s_29_5: const #432u : u32
        let s_29_5: u32 = 432;
        // D s_29_6: read-reg s_29_5:u8
        let s_29_6: u8 = {
            let value = state.read_register::<u8>(s_29_5 as isize);
            tracer.read_register(s_29_5 as isize, value);
            value
        };
        // D s_29_7: call AArch64_AArch32SystemAccessTrap(s_29_6, s_29_4)
        let s_29_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_29_6, s_29_4);
        // N s_29_8: return
        return;
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var __HSTR_EL2_T7:u8
        let s_30_0: bool = fn_state.u__HSTR_EL2_T7;
        // D s_30_1: cast zx s_30_0 -> bv
        let s_30_1: Bits = Bits::new(s_30_0 as u128, 1u16);
        // C s_30_2: const #1u : u8
        let s_30_2: bool = true;
        // C s_30_3: cast zx s_30_2 -> bv
        let s_30_3: Bits = Bits::new(s_30_2 as u128, 1u16);
        // D s_30_4: cmp-eq s_30_1 s_30_3
        let s_30_4: bool = ((s_30_1) == (s_30_3));
        // D s_30_5: write-var gs#124222 <= s_30_4
        fn_state.gs_124222 = s_30_4;
        // N s_30_6: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #432u : u32
        let s_31_0: u32 = 432;
        // D s_31_1: read-reg s_31_0:u8
        let s_31_1: u8 = {
            let value = state.read_register::<u8>(s_31_0 as isize);
            tracer.read_register(s_31_0 as isize, value);
            value
        };
        // D s_31_2: call ELUsingAArch32(s_31_1)
        let s_31_2: bool = ELUsingAArch32(state, tracer, s_31_1);
        // D s_31_3: not s_31_2
        let s_31_3: bool = !s_31_2;
        // D s_31_4: write-var gs#124221 <= s_31_3
        fn_state.gs_124221 = s_31_3;
        // N s_31_5: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #440u : u32
        let s_32_0: u32 = 440;
        // D s_32_1: read-reg s_32_0:u8
        let s_32_1: u8 = {
            let value = state.read_register::<u8>(s_32_0 as isize);
            tracer.read_register(s_32_0 as isize, value);
            value
        };
        // D s_32_2: call ELUsingAArch32(s_32_1)
        let s_32_2: bool = ELUsingAArch32(state, tracer, s_32_1);
        // D s_32_3: not s_32_2
        let s_32_3: bool = !s_32_2;
        // N s_32_4: branch s_32_3 b119 b33
        if s_32_3 {
            return block_119(state, tracer, fn_state);
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
        // D s_33_1: write-var gs#124228 <= s_33_0
        fn_state.gs_124228 = s_33_0;
        // N s_33_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var gs#124228:u8
        let s_34_0: bool = fn_state.gs_124228;
        // N s_34_1: branch s_34_0 b118 b35
        if s_34_0 {
            return block_118(state, tracer, fn_state);
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
        // D s_35_1: write-var gs#124229 <= s_35_0
        fn_state.gs_124229 = s_35_0;
        // N s_35_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var gs#124229:u8
        let s_36_0: bool = fn_state.gs_124229;
        // N s_36_1: branch s_36_0 b109 b37
        if s_36_0 {
            return block_109(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #440u : u32
        let s_37_0: u32 = 440;
        // D s_37_1: read-reg s_37_0:u8
        let s_37_1: u8 = {
            let value = state.read_register::<u8>(s_37_0 as isize);
            tracer.read_register(s_37_0 as isize, value);
            value
        };
        // D s_37_2: call ELUsingAArch32(s_37_1)
        let s_37_2: bool = ELUsingAArch32(state, tracer, s_37_1);
        // N s_37_3: branch s_37_2 b108 b38
        if s_37_2 {
            return block_108(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #0u : u8
        let s_38_0: bool = false;
        // D s_38_1: write-var gs#124230 <= s_38_0
        fn_state.gs_124230 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#124230:u8
        let s_39_0: bool = fn_state.gs_124230;
        // N s_39_1: branch s_39_0 b91 b40
        if s_39_0 {
            return block_91(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #() : ()
        let s_40_0: () = ();
        // S s_40_1: call EL2Enabled(s_40_0)
        let s_40_1: bool = EL2Enabled(state, tracer, s_40_0);
        // N s_40_2: branch s_40_1 b90 b41
        if s_40_1 {
            return block_90(state, tracer, fn_state);
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
        // D s_41_1: write-var gs#124231 <= s_41_0
        fn_state.gs_124231 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#124231:u8
        let s_42_0: bool = fn_state.gs_124231;
        // N s_42_1: branch s_42_0 b89 b43
        if s_42_0 {
            return block_89(state, tracer, fn_state);
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
        // D s_43_1: write-var gs#124232 <= s_43_0
        fn_state.gs_124232 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#124232:u8
        let s_44_0: bool = fn_state.gs_124232;
        // N s_44_1: branch s_44_0 b88 b45
        if s_44_0 {
            return block_88(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #0u : u8
        let s_45_0: bool = false;
        // D s_45_1: write-var gs#124233 <= s_45_0
        fn_state.gs_124233 = s_45_0;
        // N s_45_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var gs#124233:u8
        let s_46_0: bool = fn_state.gs_124233;
        // N s_46_1: branch s_46_0 b87 b47
        if s_46_0 {
            return block_87(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #() : ()
        let s_47_0: () = ();
        // S s_47_1: call EL2Enabled(s_47_0)
        let s_47_1: bool = EL2Enabled(state, tracer, s_47_0);
        // N s_47_2: branch s_47_1 b86 b48
        if s_47_1 {
            return block_86(state, tracer, fn_state);
        } else {
            return block_48(state, tracer, fn_state);
        };
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #0u : u8
        let s_48_0: bool = false;
        // D s_48_1: write-var gs#124234 <= s_48_0
        fn_state.gs_124234 = s_48_0;
        // N s_48_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var gs#124234:u8
        let s_49_0: bool = fn_state.gs_124234;
        // N s_49_1: branch s_49_0 b85 b50
        if s_49_0 {
            return block_85(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #0u : u8
        let s_50_0: bool = false;
        // D s_50_1: write-var gs#124235 <= s_50_0
        fn_state.gs_124235 = s_50_0;
        // N s_50_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var gs#124235:u8
        let s_51_0: bool = fn_state.gs_124235;
        // N s_51_1: branch s_51_0 b84 b52
        if s_51_0 {
            return block_84(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #() : ()
        let s_52_0: () = ();
        // S s_52_1: call EL2Enabled(s_52_0)
        let s_52_1: bool = EL2Enabled(state, tracer, s_52_0);
        // N s_52_2: branch s_52_1 b83 b53
        if s_52_1 {
            return block_83(state, tracer, fn_state);
        } else {
            return block_53(state, tracer, fn_state);
        };
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #0u : u8
        let s_53_0: bool = false;
        // D s_53_1: write-var gs#124236 <= s_53_0
        fn_state.gs_124236 = s_53_0;
        // N s_53_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var gs#124236:u8
        let s_54_0: bool = fn_state.gs_124236;
        // N s_54_1: branch s_54_0 b82 b55
        if s_54_0 {
            return block_82(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #0u : u8
        let s_55_0: bool = false;
        // D s_55_1: write-var gs#124237 <= s_55_0
        fn_state.gs_124237 = s_55_0;
        // N s_55_2: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var gs#124237:u8
        let s_56_0: bool = fn_state.gs_124237;
        // N s_56_1: branch s_56_0 b81 b57
        if s_56_0 {
            return block_81(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #0u : u8
        let s_57_0: bool = false;
        // D s_57_1: write-var gs#124238 <= s_57_0
        fn_state.gs_124238 = s_57_0;
        // N s_57_2: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var gs#124238:u8
        let s_58_0: bool = fn_state.gs_124238;
        // N s_58_1: branch s_58_0 b77 b59
        if s_58_0 {
            return block_77(state, tracer, fn_state);
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
        // D s_59_1: write-var gs#124240 <= s_59_0
        fn_state.gs_124240 = s_59_0;
        // N s_59_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var gs#124240:u8
        let s_60_0: bool = fn_state.gs_124240;
        // N s_60_1: branch s_60_0 b76 b61
        if s_60_0 {
            return block_76(state, tracer, fn_state);
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
        // D s_61_1: write-var gs#124241 <= s_61_0
        fn_state.gs_124241 = s_61_0;
        // N s_61_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var gs#124241:u8
        let s_62_0: bool = fn_state.gs_124241;
        // N s_62_1: branch s_62_0 b75 b63
        if s_62_0 {
            return block_75(state, tracer, fn_state);
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
        // N s_63_2: branch s_63_1 b74 b64
        if s_63_1 {
            return block_74(state, tracer, fn_state);
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
        // D s_64_1: write-var gs#124242 <= s_64_0
        fn_state.gs_124242 = s_64_0;
        // N s_64_2: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_65_0: read-var gs#124242:u8
        let s_65_0: bool = fn_state.gs_124242;
        // N s_65_1: branch s_65_0 b73 b66
        if s_65_0 {
            return block_73(state, tracer, fn_state);
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
        // D s_66_1: write-var gs#124243 <= s_66_0
        fn_state.gs_124243 = s_66_0;
        // N s_66_2: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var gs#124243:u8
        let s_67_0: bool = fn_state.gs_124243;
        // N s_67_1: branch s_67_0 b72 b68
        if s_67_0 {
            return block_72(state, tracer, fn_state);
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
        // D s_68_1: write-var gs#124244 <= s_68_0
        fn_state.gs_124244 = s_68_0;
        // N s_68_2: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_69_0: read-var gs#124244:u8
        let s_69_0: bool = fn_state.gs_124244;
        // N s_69_1: branch s_69_0 b71 b70
        if s_69_0 {
            return block_71(state, tracer, fn_state);
        } else {
            return block_70(state, tracer, fn_state);
        };
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_70_0: read-var t:i
        let s_70_0: i128 = fn_state.t;
        // D s_70_1: call R_read(s_70_0)
        let s_70_1: u32 = R_read(state, tracer, s_70_0);
        // C s_70_2: const #1u : u32
        let s_70_2: u32 = 1;
        // D s_70_3: call AArch32_RestrictPrediction(s_70_1, s_70_2)
        let s_70_3: () = AArch32_RestrictPrediction(state, tracer, s_70_1, s_70_2);
        // N s_70_4: return
        return;
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #3u : u8
        let s_71_0: u8 = 3;
        // C s_71_1: cast zx s_71_0 -> bv
        let s_71_1: Bits = Bits::new(s_71_0 as u128, 8u16);
        // C s_71_2: cast zx s_71_1 -> i
        let s_71_2: i128 = (s_71_1.value() as i128);
        // C s_71_3: cast reint s_71_2 -> i64
        let s_71_3: i64 = (s_71_2 as i64);
        // C s_71_4: cast zx s_71_3 -> i
        let s_71_4: i128 = (i128::try_from(s_71_3).unwrap());
        // C s_71_5: const #432u : u32
        let s_71_5: u32 = 432;
        // D s_71_6: read-reg s_71_5:u8
        let s_71_6: u8 = {
            let value = state.read_register::<u8>(s_71_5 as isize);
            tracer.read_register(s_71_5 as isize, value);
            value
        };
        // D s_71_7: call AArch64_AArch32SystemAccessTrap(s_71_6, s_71_4)
        let s_71_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_71_6, s_71_4);
        // N s_71_8: return
        return;
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_72_0: read-var __SCTLR_EL2_EnRCTX:u8
        let s_72_0: bool = fn_state.u__SCTLR_EL2_EnRCTX;
        // D s_72_1: cast zx s_72_0 -> bv
        let s_72_1: Bits = Bits::new(s_72_0 as u128, 1u16);
        // C s_72_2: const #0u : u8
        let s_72_2: bool = false;
        // C s_72_3: cast zx s_72_2 -> bv
        let s_72_3: Bits = Bits::new(s_72_2 as u128, 1u16);
        // D s_72_4: cmp-eq s_72_1 s_72_3
        let s_72_4: bool = ((s_72_1) == (s_72_3));
        // D s_72_5: write-var gs#124244 <= s_72_4
        fn_state.gs_124244 = s_72_4;
        // N s_72_6: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #102552u : u32
        let s_73_0: u32 = 102552;
        // D s_73_1: read-reg s_73_0:struct
        let s_73_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_73_0 as isize);
            tracer.read_register(s_73_0 as isize, value);
            value
        };
        // D s_73_2: call _get_HCR_EL2_Type_E2H(s_73_1)
        let s_73_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_73_1);
        // C s_73_3: const #102552u : u32
        let s_73_3: u32 = 102552;
        // D s_73_4: read-reg s_73_3:struct
        let s_73_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_73_3 as isize);
            tracer.read_register(s_73_3 as isize, value);
            value
        };
        // D s_73_5: call _get_HCR_EL2_Type_TGE(s_73_4)
        let s_73_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_73_4);
        // D s_73_6: cast zx s_73_2 -> bv
        let s_73_6: Bits = Bits::new(s_73_2 as u128, 1u16);
        // D s_73_7: cast zx s_73_5 -> bv
        let s_73_7: Bits = Bits::new(s_73_5 as u128, 1u16);
        // D s_73_8: cast reint s_73_6 -> u128
        let s_73_8: u128 = (s_73_6.value() as u128);
        // D s_73_9: size-of s_73_6
        let s_73_9: u16 = s_73_6.length();
        // D s_73_10: cast reint s_73_7 -> u128
        let s_73_10: u128 = (s_73_7.value() as u128);
        // D s_73_11: size-of s_73_7
        let s_73_11: u16 = s_73_7.length();
        // D s_73_12: lsl s_73_8 s_73_11
        let s_73_12: u128 = s_73_8 << s_73_11;
        // D s_73_13: or s_73_12 s_73_10
        let s_73_13: u128 = ((s_73_12) | (s_73_10));
        // D s_73_14: add s_73_9 s_73_11
        let s_73_14: u16 = (s_73_9 + s_73_11);
        // D s_73_15: create-bits s_73_13 s_73_14
        let s_73_15: Bits = Bits::new(s_73_13, s_73_14);
        // D s_73_16: cast reint s_73_15 -> u8
        let s_73_16: u8 = (s_73_15.value() as u8);
        // D s_73_17: cast zx s_73_16 -> bv
        let s_73_17: Bits = Bits::new(s_73_16 as u128, 2u16);
        // C s_73_18: const #3u : u8
        let s_73_18: u8 = 3;
        // C s_73_19: cast zx s_73_18 -> bv
        let s_73_19: Bits = Bits::new(s_73_18 as u128, 2u16);
        // D s_73_20: cmp-eq s_73_17 s_73_19
        let s_73_20: bool = ((s_73_17) == (s_73_19));
        // D s_73_21: write-var gs#124243 <= s_73_20
        fn_state.gs_124243 = s_73_20;
        // N s_73_22: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #432u : u32
        let s_74_0: u32 = 432;
        // D s_74_1: read-reg s_74_0:u8
        let s_74_1: u8 = {
            let value = state.read_register::<u8>(s_74_0 as isize);
            tracer.read_register(s_74_0 as isize, value);
            value
        };
        // D s_74_2: call ELUsingAArch32(s_74_1)
        let s_74_2: bool = ELUsingAArch32(state, tracer, s_74_1);
        // D s_74_3: not s_74_2
        let s_74_3: bool = !s_74_2;
        // D s_74_4: write-var gs#124242 <= s_74_3
        fn_state.gs_124242 = s_74_3;
        // N s_74_5: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #3u : u8
        let s_75_0: u8 = 3;
        // C s_75_1: cast zx s_75_0 -> bv
        let s_75_1: Bits = Bits::new(s_75_0 as u128, 8u16);
        // C s_75_2: cast zx s_75_1 -> i
        let s_75_2: i128 = (s_75_1.value() as i128);
        // C s_75_3: cast reint s_75_2 -> i64
        let s_75_3: i64 = (s_75_2 as i64);
        // C s_75_4: cast zx s_75_3 -> i
        let s_75_4: i128 = (i128::try_from(s_75_3).unwrap());
        // C s_75_5: const #432u : u32
        let s_75_5: u32 = 432;
        // D s_75_6: read-reg s_75_5:u8
        let s_75_6: u8 = {
            let value = state.read_register::<u8>(s_75_5 as isize);
            tracer.read_register(s_75_5 as isize, value);
            value
        };
        // D s_75_7: call AArch64_AArch32SystemAccessTrap(s_75_6, s_75_4)
        let s_75_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_75_6, s_75_4);
        // N s_75_8: return
        return;
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_76_0: read-var __HFGITR_EL2_CFPRCTX:u8
        let s_76_0: bool = fn_state.u__HFGITR_EL2_CFPRCTX;
        // D s_76_1: cast zx s_76_0 -> bv
        let s_76_1: Bits = Bits::new(s_76_0 as u128, 1u16);
        // C s_76_2: const #1u : u8
        let s_76_2: bool = true;
        // C s_76_3: cast zx s_76_2 -> bv
        let s_76_3: Bits = Bits::new(s_76_2 as u128, 1u16);
        // D s_76_4: cmp-eq s_76_1 s_76_3
        let s_76_4: bool = ((s_76_1) == (s_76_3));
        // D s_76_5: write-var gs#124241 <= s_76_4
        fn_state.gs_124241 = s_76_4;
        // N s_76_6: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #424u : u32
        let s_77_0: u32 = 424;
        // D s_77_1: read-reg s_77_0:u8
        let s_77_1: u8 = {
            let value = state.read_register::<u8>(s_77_0 as isize);
            tracer.read_register(s_77_0 as isize, value);
            value
        };
        // C s_77_2: const #2u : u8
        let s_77_2: u8 = 2;
        // D s_77_3: cmp-lt s_77_1 s_77_2
        let s_77_3: bool = ((s_77_1) < (s_77_2));
        // D s_77_4: not s_77_3
        let s_77_4: bool = !s_77_3;
        // N s_77_5: branch s_77_4 b80 b78
        if s_77_4 {
            return block_80(state, tracer, fn_state);
        } else {
            return block_78(state, tracer, fn_state);
        };
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_78_0: read-var __SCR_EL3_FGTEn:u8
        let s_78_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_78_1: cast zx s_78_0 -> bv
        let s_78_1: Bits = Bits::new(s_78_0 as u128, 1u16);
        // C s_78_2: const #1u : u8
        let s_78_2: bool = true;
        // C s_78_3: cast zx s_78_2 -> bv
        let s_78_3: Bits = Bits::new(s_78_2 as u128, 1u16);
        // D s_78_4: cmp-eq s_78_1 s_78_3
        let s_78_4: bool = ((s_78_1) == (s_78_3));
        // D s_78_5: write-var gs#124239 <= s_78_4
        fn_state.gs_124239 = s_78_4;
        // N s_78_6: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_79_0: read-var gs#124239:u8
        let s_79_0: bool = fn_state.gs_124239;
        // D s_79_1: write-var gs#124240 <= s_79_0
        fn_state.gs_124240 = s_79_0;
        // N s_79_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #1u : u8
        let s_80_0: bool = true;
        // D s_80_1: write-var gs#124239 <= s_80_0
        fn_state.gs_124239 = s_80_0;
        // N s_80_2: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #146u : u32
        let s_81_0: u32 = 146;
        // S s_81_1: call IsFeatureImplemented(s_81_0)
        let s_81_1: bool = IsFeatureImplemented(state, tracer, s_81_0);
        // D s_81_2: write-var gs#124238 <= s_81_1
        fn_state.gs_124238 = s_81_1;
        // N s_81_3: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #102552u : u32
        let s_82_0: u32 = 102552;
        // D s_82_1: read-reg s_82_0:struct
        let s_82_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_82_0 as isize);
            tracer.read_register(s_82_0 as isize, value);
            value
        };
        // D s_82_2: call _get_HCR_EL2_Type_E2H(s_82_1)
        let s_82_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_82_1);
        // C s_82_3: const #102552u : u32
        let s_82_3: u32 = 102552;
        // D s_82_4: read-reg s_82_3:struct
        let s_82_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_82_3 as isize);
            tracer.read_register(s_82_3 as isize, value);
            value
        };
        // D s_82_5: call _get_HCR_EL2_Type_TGE(s_82_4)
        let s_82_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_82_4);
        // D s_82_6: cast zx s_82_2 -> bv
        let s_82_6: Bits = Bits::new(s_82_2 as u128, 1u16);
        // D s_82_7: cast zx s_82_5 -> bv
        let s_82_7: Bits = Bits::new(s_82_5 as u128, 1u16);
        // D s_82_8: cast reint s_82_6 -> u128
        let s_82_8: u128 = (s_82_6.value() as u128);
        // D s_82_9: size-of s_82_6
        let s_82_9: u16 = s_82_6.length();
        // D s_82_10: cast reint s_82_7 -> u128
        let s_82_10: u128 = (s_82_7.value() as u128);
        // D s_82_11: size-of s_82_7
        let s_82_11: u16 = s_82_7.length();
        // D s_82_12: lsl s_82_8 s_82_11
        let s_82_12: u128 = s_82_8 << s_82_11;
        // D s_82_13: or s_82_12 s_82_10
        let s_82_13: u128 = ((s_82_12) | (s_82_10));
        // D s_82_14: add s_82_9 s_82_11
        let s_82_14: u16 = (s_82_9 + s_82_11);
        // D s_82_15: create-bits s_82_13 s_82_14
        let s_82_15: Bits = Bits::new(s_82_13, s_82_14);
        // D s_82_16: cast reint s_82_15 -> u8
        let s_82_16: u8 = (s_82_15.value() as u8);
        // D s_82_17: cast zx s_82_16 -> bv
        let s_82_17: Bits = Bits::new(s_82_16 as u128, 2u16);
        // C s_82_18: const #3u : u8
        let s_82_18: u8 = 3;
        // C s_82_19: cast zx s_82_18 -> bv
        let s_82_19: Bits = Bits::new(s_82_18 as u128, 2u16);
        // D s_82_20: cmp-ne s_82_17 s_82_19
        let s_82_20: bool = ((s_82_17) != (s_82_19));
        // D s_82_21: write-var gs#124237 <= s_82_20
        fn_state.gs_124237 = s_82_20;
        // N s_82_22: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #440u : u32
        let s_83_0: u32 = 440;
        // D s_83_1: read-reg s_83_0:u8
        let s_83_1: u8 = {
            let value = state.read_register::<u8>(s_83_0 as isize);
            tracer.read_register(s_83_0 as isize, value);
            value
        };
        // D s_83_2: call ELUsingAArch32(s_83_1)
        let s_83_2: bool = ELUsingAArch32(state, tracer, s_83_1);
        // D s_83_3: not s_83_2
        let s_83_3: bool = !s_83_2;
        // D s_83_4: write-var gs#124236 <= s_83_3
        fn_state.gs_124236 = s_83_3;
        // N s_83_5: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_84_0: const #3u : u8
        let s_84_0: u8 = 3;
        // C s_84_1: cast zx s_84_0 -> bv
        let s_84_1: Bits = Bits::new(s_84_0 as u128, 8u16);
        // C s_84_2: cast zx s_84_1 -> i
        let s_84_2: i128 = (s_84_1.value() as i128);
        // C s_84_3: cast reint s_84_2 -> i64
        let s_84_3: i64 = (s_84_2 as i64);
        // C s_84_4: cast zx s_84_3 -> i
        let s_84_4: i128 = (i128::try_from(s_84_3).unwrap());
        // S s_84_5: call AArch32_TakeHypTrapException(s_84_4)
        let s_84_5: () = AArch32_TakeHypTrapException(state, tracer, s_84_4);
        // N s_84_6: return
        return;
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_85_0: read-var __HSTR_T7:u8
        let s_85_0: bool = fn_state.u__HSTR_T7;
        // D s_85_1: cast zx s_85_0 -> bv
        let s_85_1: Bits = Bits::new(s_85_0 as u128, 1u16);
        // C s_85_2: const #1u : u8
        let s_85_2: bool = true;
        // C s_85_3: cast zx s_85_2 -> bv
        let s_85_3: Bits = Bits::new(s_85_2 as u128, 1u16);
        // D s_85_4: cmp-eq s_85_1 s_85_3
        let s_85_4: bool = ((s_85_1) == (s_85_3));
        // D s_85_5: write-var gs#124235 <= s_85_4
        fn_state.gs_124235 = s_85_4;
        // N s_85_6: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_86_0: const #432u : u32
        let s_86_0: u32 = 432;
        // D s_86_1: read-reg s_86_0:u8
        let s_86_1: u8 = {
            let value = state.read_register::<u8>(s_86_0 as isize);
            tracer.read_register(s_86_0 as isize, value);
            value
        };
        // D s_86_2: call ELUsingAArch32(s_86_1)
        let s_86_2: bool = ELUsingAArch32(state, tracer, s_86_1);
        // D s_86_3: write-var gs#124234 <= s_86_2
        fn_state.gs_124234 = s_86_2;
        // N s_86_4: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_87_0: const #3u : u8
        let s_87_0: u8 = 3;
        // C s_87_1: cast zx s_87_0 -> bv
        let s_87_1: Bits = Bits::new(s_87_0 as u128, 8u16);
        // C s_87_2: cast zx s_87_1 -> i
        let s_87_2: i128 = (s_87_1.value() as i128);
        // C s_87_3: cast reint s_87_2 -> i64
        let s_87_3: i64 = (s_87_2 as i64);
        // C s_87_4: cast zx s_87_3 -> i
        let s_87_4: i128 = (i128::try_from(s_87_3).unwrap());
        // C s_87_5: const #432u : u32
        let s_87_5: u32 = 432;
        // D s_87_6: read-reg s_87_5:u8
        let s_87_6: u8 = {
            let value = state.read_register::<u8>(s_87_5 as isize);
            tracer.read_register(s_87_5 as isize, value);
            value
        };
        // D s_87_7: call AArch64_AArch32SystemAccessTrap(s_87_6, s_87_4)
        let s_87_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_87_6, s_87_4);
        // N s_87_8: return
        return;
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_88_0: read-var __HSTR_EL2_T7:u8
        let s_88_0: bool = fn_state.u__HSTR_EL2_T7;
        // D s_88_1: cast zx s_88_0 -> bv
        let s_88_1: Bits = Bits::new(s_88_0 as u128, 1u16);
        // C s_88_2: const #1u : u8
        let s_88_2: bool = true;
        // C s_88_3: cast zx s_88_2 -> bv
        let s_88_3: Bits = Bits::new(s_88_2 as u128, 1u16);
        // D s_88_4: cmp-eq s_88_1 s_88_3
        let s_88_4: bool = ((s_88_1) == (s_88_3));
        // D s_88_5: write-var gs#124233 <= s_88_4
        fn_state.gs_124233 = s_88_4;
        // N s_88_6: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_89_0: const #102552u : u32
        let s_89_0: u32 = 102552;
        // D s_89_1: read-reg s_89_0:struct
        let s_89_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_89_0 as isize);
            tracer.read_register(s_89_0 as isize, value);
            value
        };
        // D s_89_2: call _get_HCR_EL2_Type_E2H(s_89_1)
        let s_89_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_89_1);
        // C s_89_3: const #102552u : u32
        let s_89_3: u32 = 102552;
        // D s_89_4: read-reg s_89_3:struct
        let s_89_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_89_3 as isize);
            tracer.read_register(s_89_3 as isize, value);
            value
        };
        // D s_89_5: call _get_HCR_EL2_Type_TGE(s_89_4)
        let s_89_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_89_4);
        // D s_89_6: cast zx s_89_2 -> bv
        let s_89_6: Bits = Bits::new(s_89_2 as u128, 1u16);
        // D s_89_7: cast zx s_89_5 -> bv
        let s_89_7: Bits = Bits::new(s_89_5 as u128, 1u16);
        // D s_89_8: cast reint s_89_6 -> u128
        let s_89_8: u128 = (s_89_6.value() as u128);
        // D s_89_9: size-of s_89_6
        let s_89_9: u16 = s_89_6.length();
        // D s_89_10: cast reint s_89_7 -> u128
        let s_89_10: u128 = (s_89_7.value() as u128);
        // D s_89_11: size-of s_89_7
        let s_89_11: u16 = s_89_7.length();
        // D s_89_12: lsl s_89_8 s_89_11
        let s_89_12: u128 = s_89_8 << s_89_11;
        // D s_89_13: or s_89_12 s_89_10
        let s_89_13: u128 = ((s_89_12) | (s_89_10));
        // D s_89_14: add s_89_9 s_89_11
        let s_89_14: u16 = (s_89_9 + s_89_11);
        // D s_89_15: create-bits s_89_13 s_89_14
        let s_89_15: Bits = Bits::new(s_89_13, s_89_14);
        // D s_89_16: cast reint s_89_15 -> u8
        let s_89_16: u8 = (s_89_15.value() as u8);
        // D s_89_17: cast zx s_89_16 -> bv
        let s_89_17: Bits = Bits::new(s_89_16 as u128, 2u16);
        // C s_89_18: const #3u : u8
        let s_89_18: u8 = 3;
        // C s_89_19: cast zx s_89_18 -> bv
        let s_89_19: Bits = Bits::new(s_89_18 as u128, 2u16);
        // D s_89_20: cmp-ne s_89_17 s_89_19
        let s_89_20: bool = ((s_89_17) != (s_89_19));
        // D s_89_21: write-var gs#124232 <= s_89_20
        fn_state.gs_124232 = s_89_20;
        // N s_89_22: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_90_0: const #432u : u32
        let s_90_0: u32 = 432;
        // D s_90_1: read-reg s_90_0:u8
        let s_90_1: u8 = {
            let value = state.read_register::<u8>(s_90_0 as isize);
            tracer.read_register(s_90_0 as isize, value);
            value
        };
        // D s_90_2: call ELUsingAArch32(s_90_1)
        let s_90_2: bool = ELUsingAArch32(state, tracer, s_90_1);
        // D s_90_3: not s_90_2
        let s_90_3: bool = !s_90_2;
        // D s_90_4: write-var gs#124231 <= s_90_3
        fn_state.gs_124231 = s_90_3;
        // N s_90_5: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_91_0: const #() : ()
        let s_91_0: () = ();
        // S s_91_1: call EL2Enabled(s_91_0)
        let s_91_1: bool = EL2Enabled(state, tracer, s_91_0);
        // N s_91_2: branch s_91_1 b107 b92
        if s_91_1 {
            return block_107(state, tracer, fn_state);
        } else {
            return block_92(state, tracer, fn_state);
        };
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_92_0: const #0u : u8
        let s_92_0: bool = false;
        // D s_92_1: write-var gs#124245 <= s_92_0
        fn_state.gs_124245 = s_92_0;
        // N s_92_2: jump b93
        return block_93(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_93_0: read-var gs#124245:u8
        let s_93_0: bool = fn_state.gs_124245;
        // N s_93_1: branch s_93_0 b106 b94
        if s_93_0 {
            return block_106(state, tracer, fn_state);
        } else {
            return block_94(state, tracer, fn_state);
        };
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_94_0: const #0u : u8
        let s_94_0: bool = false;
        // D s_94_1: write-var gs#124246 <= s_94_0
        fn_state.gs_124246 = s_94_0;
        // N s_94_2: jump b95
        return block_95(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_95_0: read-var gs#124246:u8
        let s_95_0: bool = fn_state.gs_124246;
        // N s_95_1: branch s_95_0 b105 b96
        if s_95_0 {
            return block_105(state, tracer, fn_state);
        } else {
            return block_96(state, tracer, fn_state);
        };
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_96_0: const #() : ()
        let s_96_0: () = ();
        // S s_96_1: call EL2Enabled(s_96_0)
        let s_96_1: bool = EL2Enabled(state, tracer, s_96_0);
        // N s_96_2: branch s_96_1 b104 b97
        if s_96_1 {
            return block_104(state, tracer, fn_state);
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
        // D s_97_1: write-var gs#124247 <= s_97_0
        fn_state.gs_124247 = s_97_0;
        // N s_97_2: jump b98
        return block_98(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_98_0: read-var gs#124247:u8
        let s_98_0: bool = fn_state.gs_124247;
        // N s_98_1: branch s_98_0 b103 b99
        if s_98_0 {
            return block_103(state, tracer, fn_state);
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
        // D s_99_1: write-var gs#124248 <= s_99_0
        fn_state.gs_124248 = s_99_0;
        // N s_99_2: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_100_0: read-var gs#124248:u8
        let s_100_0: bool = fn_state.gs_124248;
        // N s_100_1: branch s_100_0 b102 b101
        if s_100_0 {
            return block_102(state, tracer, fn_state);
        } else {
            return block_101(state, tracer, fn_state);
        };
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_101_0: panic
        panic!("{:?}", ());
        // N s_101_1: return
        return;
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_102_0: const #0u : u8
        let s_102_0: u8 = 0;
        // C s_102_1: cast zx s_102_0 -> bv
        let s_102_1: Bits = Bits::new(s_102_0 as u128, 8u16);
        // C s_102_2: cast zx s_102_1 -> i
        let s_102_2: i128 = (s_102_1.value() as i128);
        // C s_102_3: cast reint s_102_2 -> i64
        let s_102_3: i64 = (s_102_2 as i64);
        // C s_102_4: cast zx s_102_3 -> i
        let s_102_4: i128 = (i128::try_from(s_102_3).unwrap());
        // S s_102_5: call AArch32_TakeHypTrapException(s_102_4)
        let s_102_5: () = AArch32_TakeHypTrapException(state, tracer, s_102_4);
        // N s_102_6: return
        return;
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_103_0: read-var __HCR_TGE:u8
        let s_103_0: bool = fn_state.u__HCR_TGE;
        // D s_103_1: cast zx s_103_0 -> bv
        let s_103_1: Bits = Bits::new(s_103_0 as u128, 1u16);
        // C s_103_2: const #1u : u8
        let s_103_2: bool = true;
        // C s_103_3: cast zx s_103_2 -> bv
        let s_103_3: Bits = Bits::new(s_103_2 as u128, 1u16);
        // D s_103_4: cmp-eq s_103_1 s_103_3
        let s_103_4: bool = ((s_103_1) == (s_103_3));
        // D s_103_5: write-var gs#124248 <= s_103_4
        fn_state.gs_124248 = s_103_4;
        // N s_103_6: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_104_0: const #432u : u32
        let s_104_0: u32 = 432;
        // D s_104_1: read-reg s_104_0:u8
        let s_104_1: u8 = {
            let value = state.read_register::<u8>(s_104_0 as isize);
            tracer.read_register(s_104_0 as isize, value);
            value
        };
        // D s_104_2: call ELUsingAArch32(s_104_1)
        let s_104_2: bool = ELUsingAArch32(state, tracer, s_104_1);
        // D s_104_3: write-var gs#124247 <= s_104_2
        fn_state.gs_124247 = s_104_2;
        // N s_104_4: jump b98
        return block_98(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_105_0: const #3u : u8
        let s_105_0: u8 = 3;
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
        // D s_105_7: call AArch64_AArch32SystemAccessTrap(s_105_6, s_105_4)
        let s_105_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_105_6,
            s_105_4,
        );
        // N s_105_8: return
        return;
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_106_0: read-var __HCR_EL2_TGE:u8
        let s_106_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_106_1: cast zx s_106_0 -> bv
        let s_106_1: Bits = Bits::new(s_106_0 as u128, 1u16);
        // C s_106_2: const #1u : u8
        let s_106_2: bool = true;
        // C s_106_3: cast zx s_106_2 -> bv
        let s_106_3: Bits = Bits::new(s_106_2 as u128, 1u16);
        // D s_106_4: cmp-eq s_106_1 s_106_3
        let s_106_4: bool = ((s_106_1) == (s_106_3));
        // D s_106_5: write-var gs#124246 <= s_106_4
        fn_state.gs_124246 = s_106_4;
        // N s_106_6: jump b95
        return block_95(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_107_0: const #432u : u32
        let s_107_0: u32 = 432;
        // D s_107_1: read-reg s_107_0:u8
        let s_107_1: u8 = {
            let value = state.read_register::<u8>(s_107_0 as isize);
            tracer.read_register(s_107_0 as isize, value);
            value
        };
        // D s_107_2: call ELUsingAArch32(s_107_1)
        let s_107_2: bool = ELUsingAArch32(state, tracer, s_107_1);
        // D s_107_3: not s_107_2
        let s_107_3: bool = !s_107_2;
        // D s_107_4: write-var gs#124245 <= s_107_3
        fn_state.gs_124245 = s_107_3;
        // N s_107_5: jump b93
        return block_93(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_108_0: read-var __SCTLR_EnRCTX:u8
        let s_108_0: bool = fn_state.u__SCTLR_EnRCTX;
        // D s_108_1: cast zx s_108_0 -> bv
        let s_108_1: Bits = Bits::new(s_108_0 as u128, 1u16);
        // C s_108_2: const #0u : u8
        let s_108_2: bool = false;
        // C s_108_3: cast zx s_108_2 -> bv
        let s_108_3: Bits = Bits::new(s_108_2 as u128, 1u16);
        // D s_108_4: cmp-eq s_108_1 s_108_3
        let s_108_4: bool = ((s_108_1) == (s_108_3));
        // D s_108_5: write-var gs#124230 <= s_108_4
        fn_state.gs_124230 = s_108_4;
        // N s_108_6: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_109_0: const #() : ()
        let s_109_0: () = ();
        // S s_109_1: call EL2Enabled(s_109_0)
        let s_109_1: bool = EL2Enabled(state, tracer, s_109_0);
        // N s_109_2: branch s_109_1 b117 b110
        if s_109_1 {
            return block_117(state, tracer, fn_state);
        } else {
            return block_110(state, tracer, fn_state);
        };
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_110_0: const #0u : u8
        let s_110_0: bool = false;
        // D s_110_1: write-var gs#124249 <= s_110_0
        fn_state.gs_124249 = s_110_0;
        // N s_110_2: jump b111
        return block_111(state, tracer, fn_state);
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_111_0: read-var gs#124249:u8
        let s_111_0: bool = fn_state.gs_124249;
        // N s_111_1: branch s_111_0 b116 b112
        if s_111_0 {
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
        // D s_112_1: write-var gs#124250 <= s_112_0
        fn_state.gs_124250 = s_112_0;
        // N s_112_2: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_113_0: read-var gs#124250:u8
        let s_113_0: bool = fn_state.gs_124250;
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
        // C s_114_0: const #3u : u8
        let s_114_0: u8 = 3;
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
        // D s_114_7: call AArch64_AArch32SystemAccessTrap(s_114_6, s_114_4)
        let s_114_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_114_6,
            s_114_4,
        );
        // N s_114_8: return
        return;
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_115_0: const #3u : u8
        let s_115_0: u8 = 3;
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
        // D s_115_7: call AArch64_AArch32SystemAccessTrap(s_115_6, s_115_4)
        let s_115_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_115_6,
            s_115_4,
        );
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
        // D s_116_5: write-var gs#124250 <= s_116_4
        fn_state.gs_124250 = s_116_4;
        // N s_116_6: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_117_0: const #432u : u32
        let s_117_0: u32 = 432;
        // D s_117_1: read-reg s_117_0:u8
        let s_117_1: u8 = {
            let value = state.read_register::<u8>(s_117_0 as isize);
            tracer.read_register(s_117_0 as isize, value);
            value
        };
        // D s_117_2: call ELUsingAArch32(s_117_1)
        let s_117_2: bool = ELUsingAArch32(state, tracer, s_117_1);
        // D s_117_3: not s_117_2
        let s_117_3: bool = !s_117_2;
        // D s_117_4: write-var gs#124249 <= s_117_3
        fn_state.gs_124249 = s_117_3;
        // N s_117_5: jump b111
        return block_111(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_118_0: read-var __SCTLR_EL1_EnRCTX:u8
        let s_118_0: bool = fn_state.u__SCTLR_EL1_EnRCTX;
        // D s_118_1: cast zx s_118_0 -> bv
        let s_118_1: Bits = Bits::new(s_118_0 as u128, 1u16);
        // C s_118_2: const #0u : u8
        let s_118_2: bool = false;
        // C s_118_3: cast zx s_118_2 -> bv
        let s_118_3: Bits = Bits::new(s_118_2 as u128, 1u16);
        // D s_118_4: cmp-eq s_118_1 s_118_3
        let s_118_4: bool = ((s_118_1) == (s_118_3));
        // D s_118_5: write-var gs#124229 <= s_118_4
        fn_state.gs_124229 = s_118_4;
        // N s_118_6: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_119_0: const #() : ()
        let s_119_0: () = ();
        // S s_119_1: call EL2Enabled(s_119_0)
        let s_119_1: bool = EL2Enabled(state, tracer, s_119_0);
        // N s_119_2: branch s_119_1 b122 b120
        if s_119_1 {
            return block_122(state, tracer, fn_state);
        } else {
            return block_120(state, tracer, fn_state);
        };
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_120_0: const #0u : u8
        let s_120_0: bool = false;
        // D s_120_1: write-var gs#124227 <= s_120_0
        fn_state.gs_124227 = s_120_0;
        // N s_120_2: jump b121
        return block_121(state, tracer, fn_state);
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_121_0: read-var gs#124227:u8
        let s_121_0: bool = fn_state.gs_124227;
        // D s_121_1: not s_121_0
        let s_121_1: bool = !s_121_0;
        // D s_121_2: write-var gs#124228 <= s_121_1
        fn_state.gs_124228 = s_121_1;
        // N s_121_3: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_122_0: const #102552u : u32
        let s_122_0: u32 = 102552;
        // D s_122_1: read-reg s_122_0:struct
        let s_122_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_122_0 as isize);
            tracer.read_register(s_122_0 as isize, value);
            value
        };
        // D s_122_2: call _get_HCR_EL2_Type_E2H(s_122_1)
        let s_122_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_122_1);
        // C s_122_3: const #102552u : u32
        let s_122_3: u32 = 102552;
        // D s_122_4: read-reg s_122_3:struct
        let s_122_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_122_3 as isize);
            tracer.read_register(s_122_3 as isize, value);
            value
        };
        // D s_122_5: call _get_HCR_EL2_Type_TGE(s_122_4)
        let s_122_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_122_4);
        // D s_122_6: cast zx s_122_2 -> bv
        let s_122_6: Bits = Bits::new(s_122_2 as u128, 1u16);
        // D s_122_7: cast zx s_122_5 -> bv
        let s_122_7: Bits = Bits::new(s_122_5 as u128, 1u16);
        // D s_122_8: cast reint s_122_6 -> u128
        let s_122_8: u128 = (s_122_6.value() as u128);
        // D s_122_9: size-of s_122_6
        let s_122_9: u16 = s_122_6.length();
        // D s_122_10: cast reint s_122_7 -> u128
        let s_122_10: u128 = (s_122_7.value() as u128);
        // D s_122_11: size-of s_122_7
        let s_122_11: u16 = s_122_7.length();
        // D s_122_12: lsl s_122_8 s_122_11
        let s_122_12: u128 = s_122_8 << s_122_11;
        // D s_122_13: or s_122_12 s_122_10
        let s_122_13: u128 = ((s_122_12) | (s_122_10));
        // D s_122_14: add s_122_9 s_122_11
        let s_122_14: u16 = (s_122_9 + s_122_11);
        // D s_122_15: create-bits s_122_13 s_122_14
        let s_122_15: Bits = Bits::new(s_122_13, s_122_14);
        // D s_122_16: cast reint s_122_15 -> u8
        let s_122_16: u8 = (s_122_15.value() as u8);
        // D s_122_17: cast zx s_122_16 -> bv
        let s_122_17: Bits = Bits::new(s_122_16 as u128, 2u16);
        // C s_122_18: const #3u : u8
        let s_122_18: u8 = 3;
        // C s_122_19: cast zx s_122_18 -> bv
        let s_122_19: Bits = Bits::new(s_122_18 as u128, 2u16);
        // D s_122_20: cmp-eq s_122_17 s_122_19
        let s_122_20: bool = ((s_122_17) == (s_122_19));
        // D s_122_21: write-var gs#124227 <= s_122_20
        fn_state.gs_124227 = s_122_20;
        // N s_122_22: jump b121
        return block_121(state, tracer, fn_state);
    }
}
