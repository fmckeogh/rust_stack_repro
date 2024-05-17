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
use u_get_HCR_EL2_Type_NV2::*;
use u_get_SCR_EL3_Type_SCTLR2En::*;
use u_get_SCR_EL3_Type_FGTEn::*;
use u_get_HCR_EL2_Type_E2H::*;
use Halted::*;
use u_get_HCR_EL2_Type_TRVM::*;
use NVMem_read::*;
use AArch64_SystemAccessTrap::*;
use u__get_SCTLR2_EL1::*;
use u__IMPDEF_boolean::*;
use IsFeatureImplemented::*;
use u_get_HCR_EL2_Type_NV1::*;
use X_set::*;
use u_get_HCRX_EL2_Type_SCTLR2En::*;
use IsHCRXEL2Enabled::*;
use u_get_HFGRTR_EL2_Type_SCTLR_EL1::*;
use u_get_HCR_EL2_Type_NV::*;
use u_get_EDSCR_Type_SDD::*;
use EL2Enabled::*;
use EDSCR_read::*;
use u__get_SCTLR2_EL2::*;
use common::*;
pub fn SCTLR2_EL1_SysRegRead_e0ec1b448b49d808<T: Tracer>(
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
        gs_69167: bool,
        ga_84766: ProductType5c790c8ef59cc8b2,
        u__HCR_EL2_E2H: bool,
        ga_84769: ProductType5c790c8ef59cc8b2,
        gs_69153: bool,
        gs_69173: bool,
        u__EDSCR_SDD: bool,
        gs_69161: bool,
        gs_69155: bool,
        gs_69156: bool,
        u__SCR_EL3_SCTLR2En: bool,
        gs_69171: bool,
        u__SCR_EL3_FGTEn: bool,
        u__HFGRTR_EL2_SCTLR_EL1: bool,
        gs_69165: bool,
        u__HCR_EL2_TRVM: bool,
        gs_69168: bool,
        gs_69169: bool,
        gs_69157: bool,
        gs_69176: bool,
        gs_69166: bool,
        ga_84773: ProductType5c790c8ef59cc8b2,
        gs_69164: bool,
        gs_69160: bool,
        gs_69154: bool,
        ga_84749: ProductType5c790c8ef59cc8b2,
        gs_69170: bool,
        u__HCRX_EL2_SCTLR2En: bool,
        gs_69172: bool,
        gs_69162: bool,
        u__PSTATE_EL: u8,
        gs_69163: bool,
        ga_84748: u64,
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
        // C s_0_3: const #() : ()
        let s_0_3: () = ();
        // S s_0_4: call EDSCR_read(s_0_3)
        let s_0_4: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_0_3);
        // S s_0_5: call _get_EDSCR_Type_SDD(s_0_4)
        let s_0_5: bool = u_get_EDSCR_Type_SDD(state, tracer, s_0_4);
        // D s_0_6: write-var __EDSCR_SDD <= s_0_5
        fn_state.u__EDSCR_SDD = s_0_5;
        // C s_0_7: const #90704u : u32
        let s_0_7: u32 = 90704;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_SCR_EL3_Type_SCTLR2En(s_0_8)
        let s_0_9: bool = u_get_SCR_EL3_Type_SCTLR2En(state, tracer, s_0_8);
        // D s_0_10: write-var __SCR_EL3_SCTLR2En <= s_0_9
        fn_state.u__SCR_EL3_SCTLR2En = s_0_9;
        // C s_0_11: const #102552u : u32
        let s_0_11: u32 = 102552;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_HCR_EL2_Type_TRVM(s_0_12)
        let s_0_13: bool = u_get_HCR_EL2_Type_TRVM(state, tracer, s_0_12);
        // D s_0_14: write-var __HCR_EL2_TRVM <= s_0_13
        fn_state.u__HCR_EL2_TRVM = s_0_13;
        // C s_0_15: const #90704u : u32
        let s_0_15: u32 = 90704;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_SCR_EL3_Type_FGTEn(s_0_16)
        let s_0_17: bool = u_get_SCR_EL3_Type_FGTEn(state, tracer, s_0_16);
        // D s_0_18: write-var __SCR_EL3_FGTEn <= s_0_17
        fn_state.u__SCR_EL3_FGTEn = s_0_17;
        // C s_0_19: const #16592u : u32
        let s_0_19: u32 = 16592;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_HFGRTR_EL2_Type_SCTLR_EL1(s_0_20)
        let s_0_21: bool = u_get_HFGRTR_EL2_Type_SCTLR_EL1(state, tracer, s_0_20);
        // D s_0_22: write-var __HFGRTR_EL2_SCTLR_EL1 <= s_0_21
        fn_state.u__HFGRTR_EL2_SCTLR_EL1 = s_0_21;
        // C s_0_23: const #22528u : u32
        let s_0_23: u32 = 22528;
        // D s_0_24: read-reg s_0_23:struct
        let s_0_24: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_23 as isize);
            tracer.read_register(s_0_23 as isize, value);
            value
        };
        // D s_0_25: call _get_HCRX_EL2_Type_SCTLR2En(s_0_24)
        let s_0_25: bool = u_get_HCRX_EL2_Type_SCTLR2En(state, tracer, s_0_24);
        // D s_0_26: write-var __HCRX_EL2_SCTLR2En <= s_0_25
        fn_state.u__HCRX_EL2_SCTLR2En = s_0_25;
        // C s_0_27: const #102552u : u32
        let s_0_27: u32 = 102552;
        // D s_0_28: read-reg s_0_27:struct
        let s_0_28: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_27 as isize);
            tracer.read_register(s_0_27 as isize, value);
            value
        };
        // D s_0_29: call _get_HCR_EL2_Type_E2H(s_0_28)
        let s_0_29: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_0_28);
        // D s_0_30: write-var __HCR_EL2_E2H <= s_0_29
        fn_state.u__HCR_EL2_E2H = s_0_29;
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
        // N s_0_37: branch s_0_36 b91 b1
        if s_0_36 {
            return block_91(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b33 b2
        if s_1_5 {
            return block_33(state, tracer, fn_state);
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
        // C s_5_1: const #11720u : u32
        let s_5_1: u32 = 11720;
        // D s_5_2: read-reg s_5_1:struct
        let s_5_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_5_1 as isize);
            tracer.read_register(s_5_1 as isize, value);
            value
        };
        // D s_5_3: call __get_SCTLR2_EL1(s_5_2)
        let s_5_3: ProductType5c790c8ef59cc8b2 = u__get_SCTLR2_EL1(state, tracer, s_5_2);
        // D s_5_4: write-var ga#84773 <= s_5_3
        fn_state.ga_84773 = s_5_3;
        // D s_5_5: read-var ga#84773.0:struct
        let s_5_5: u64 = fn_state.ga_84773._0;
        // D s_5_6: cast zx s_5_5 -> bv
        let s_5_6: Bits = Bits::new(s_5_5 as u128, 64u16);
        // D s_5_7: read-var t:i
        let s_5_7: i128 = fn_state.t;
        // D s_5_8: call X_set(s_5_7, s_5_0, s_5_6)
        let s_5_8: () = X_set(state, tracer, s_5_7, s_5_0, s_5_6);
        // N s_5_9: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call Halted(s_6_0)
        let s_6_1: bool = Halted(state, tracer, s_6_0);
        // N s_6_2: branch s_6_1 b32 b7
        if s_6_1 {
            return block_32(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#69153 <= s_7_0
        fn_state.gs_69153 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#69153:u8
        let s_8_0: bool = fn_state.gs_69153;
        // N s_8_1: branch s_8_0 b31 b9
        if s_8_0 {
            return block_31(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#69154 <= s_9_0
        fn_state.gs_69154 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#69154:u8
        let s_10_0: bool = fn_state.gs_69154;
        // N s_10_1: branch s_10_0 b30 b11
        if s_10_0 {
            return block_30(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#69155 <= s_11_0
        fn_state.gs_69155 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#69155:u8
        let s_12_0: bool = fn_state.gs_69155;
        // N s_12_1: branch s_12_0 b29 b13
        if s_12_0 {
            return block_29(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#69156 <= s_13_0
        fn_state.gs_69156 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#69156:u8
        let s_14_0: bool = fn_state.gs_69156;
        // N s_14_1: branch s_14_0 b28 b15
        if s_14_0 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #424u : u32
        let s_15_0: u32 = 424;
        // D s_15_1: read-reg s_15_0:u8
        let s_15_1: u8 = {
            let value = state.read_register::<u8>(s_15_0 as isize);
            tracer.read_register(s_15_0 as isize, value);
            value
        };
        // C s_15_2: const #2u : u8
        let s_15_2: u8 = 2;
        // D s_15_3: cmp-lt s_15_1 s_15_2
        let s_15_3: bool = ((s_15_1) < (s_15_2));
        // N s_15_4: branch s_15_3 b27 b16
        if s_15_3 {
            return block_27(state, tracer, fn_state);
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
        // D s_16_1: write-var gs#69157 <= s_16_0
        fn_state.gs_69157 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#69157:u8
        let s_17_0: bool = fn_state.gs_69157;
        // N s_17_1: branch s_17_0 b21 b18
        if s_17_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var __HCR_EL2_E2H:u8
        let s_18_0: bool = fn_state.u__HCR_EL2_E2H;
        // D s_18_1: cast zx s_18_0 -> bv
        let s_18_1: Bits = Bits::new(s_18_0 as u128, 1u16);
        // C s_18_2: const #1u : u8
        let s_18_2: bool = true;
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
        // C s_19_1: const #11720u : u32
        let s_19_1: u32 = 11720;
        // D s_19_2: read-reg s_19_1:struct
        let s_19_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_19_1 as isize);
            tracer.read_register(s_19_1 as isize, value);
            value
        };
        // D s_19_3: call __get_SCTLR2_EL1(s_19_2)
        let s_19_3: ProductType5c790c8ef59cc8b2 = u__get_SCTLR2_EL1(
            state,
            tracer,
            s_19_2,
        );
        // D s_19_4: write-var ga#84769 <= s_19_3
        fn_state.ga_84769 = s_19_3;
        // D s_19_5: read-var ga#84769.0:struct
        let s_19_5: u64 = fn_state.ga_84769._0;
        // D s_19_6: cast zx s_19_5 -> bv
        let s_19_6: Bits = Bits::new(s_19_5 as u128, 64u16);
        // D s_19_7: read-var t:i
        let s_19_7: i128 = fn_state.t;
        // D s_19_8: call X_set(s_19_7, s_19_0, s_19_6)
        let s_19_8: () = X_set(state, tracer, s_19_7, s_19_0, s_19_6);
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
        // C s_20_1: const #102680u : u32
        let s_20_1: u32 = 102680;
        // D s_20_2: read-reg s_20_1:struct
        let s_20_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_20_1 as isize);
            tracer.read_register(s_20_1 as isize, value);
            value
        };
        // D s_20_3: call __get_SCTLR2_EL2(s_20_2)
        let s_20_3: ProductType5c790c8ef59cc8b2 = u__get_SCTLR2_EL2(
            state,
            tracer,
            s_20_2,
        );
        // D s_20_4: write-var ga#84766 <= s_20_3
        fn_state.ga_84766 = s_20_3;
        // D s_20_5: read-var ga#84766.0:struct
        let s_20_5: u64 = fn_state.ga_84766._0;
        // D s_20_6: cast zx s_20_5 -> bv
        let s_20_6: Bits = Bits::new(s_20_5 as u128, 64u16);
        // D s_20_7: read-var t:i
        let s_20_7: i128 = fn_state.t;
        // D s_20_8: call X_set(s_20_7, s_20_0, s_20_6)
        let s_20_8: () = X_set(state, tracer, s_20_7, s_20_0, s_20_6);
        // N s_20_9: return
        return;
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #() : ()
        let s_21_0: () = ();
        // S s_21_1: call Halted(s_21_0)
        let s_21_1: bool = Halted(state, tracer, s_21_0);
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
        // D s_22_1: write-var gs#69160 <= s_22_0
        fn_state.gs_69160 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#69160:u8
        let s_23_0: bool = fn_state.gs_69160;
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
        // C s_24_0: const #24u : u8
        let s_24_0: u8 = 24;
        // C s_24_1: cast zx s_24_0 -> bv
        let s_24_1: Bits = Bits::new(s_24_0 as u128, 8u16);
        // C s_24_2: cast zx s_24_1 -> i
        let s_24_2: i128 = (s_24_1.value() as i128);
        // C s_24_3: cast reint s_24_2 -> i64
        let s_24_3: i64 = (s_24_2 as i64);
        // C s_24_4: cast zx s_24_3 -> i
        let s_24_4: i128 = (i128::try_from(s_24_3).unwrap());
        // C s_24_5: const #424u : u32
        let s_24_5: u32 = 424;
        // D s_24_6: read-reg s_24_5:u8
        let s_24_6: u8 = {
            let value = state.read_register::<u8>(s_24_5 as isize);
            tracer.read_register(s_24_5 as isize, value);
            value
        };
        // D s_24_7: call AArch64_SystemAccessTrap(s_24_6, s_24_4)
        let s_24_7: () = AArch64_SystemAccessTrap(state, tracer, s_24_6, s_24_4);
        // N s_24_8: return
        return;
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
        // D s_26_0: read-var __EDSCR_SDD:u8
        let s_26_0: bool = fn_state.u__EDSCR_SDD;
        // D s_26_1: cast zx s_26_0 -> bv
        let s_26_1: Bits = Bits::new(s_26_0 as u128, 1u16);
        // C s_26_2: const #1u : u8
        let s_26_2: bool = true;
        // C s_26_3: cast zx s_26_2 -> bv
        let s_26_3: Bits = Bits::new(s_26_2 as u128, 1u16);
        // D s_26_4: cmp-eq s_26_1 s_26_3
        let s_26_4: bool = ((s_26_1) == (s_26_3));
        // D s_26_5: write-var gs#69160 <= s_26_4
        fn_state.gs_69160 = s_26_4;
        // N s_26_6: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var __SCR_EL3_SCTLR2En:u8
        let s_27_0: bool = fn_state.u__SCR_EL3_SCTLR2En;
        // D s_27_1: cast zx s_27_0 -> bv
        let s_27_1: Bits = Bits::new(s_27_0 as u128, 1u16);
        // C s_27_2: const #0u : u8
        let s_27_2: bool = false;
        // C s_27_3: cast zx s_27_2 -> bv
        let s_27_3: Bits = Bits::new(s_27_2 as u128, 1u16);
        // D s_27_4: cmp-eq s_27_1 s_27_3
        let s_27_4: bool = ((s_27_1) == (s_27_3));
        // D s_27_5: write-var gs#69157 <= s_27_4
        fn_state.gs_69157 = s_27_4;
        // N s_27_6: jump b17
        return block_17(state, tracer, fn_state);
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
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var __SCR_EL3_SCTLR2En:u8
        let s_29_0: bool = fn_state.u__SCR_EL3_SCTLR2En;
        // D s_29_1: cast zx s_29_0 -> bv
        let s_29_1: Bits = Bits::new(s_29_0 as u128, 1u16);
        // C s_29_2: const #0u : u8
        let s_29_2: bool = false;
        // C s_29_3: cast zx s_29_2 -> bv
        let s_29_3: Bits = Bits::new(s_29_2 as u128, 1u16);
        // D s_29_4: cmp-eq s_29_1 s_29_3
        let s_29_4: bool = ((s_29_1) == (s_29_3));
        // D s_29_5: write-var gs#69156 <= s_29_4
        fn_state.gs_69156 = s_29_4;
        // N s_29_6: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_30_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_30_1: call __IMPDEF_boolean(s_30_0)
        let s_30_1: bool = u__IMPDEF_boolean(state, tracer, s_30_0);
        // D s_30_2: write-var gs#69155 <= s_30_1
        fn_state.gs_69155 = s_30_1;
        // N s_30_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var __EDSCR_SDD:u8
        let s_31_0: bool = fn_state.u__EDSCR_SDD;
        // D s_31_1: cast zx s_31_0 -> bv
        let s_31_1: Bits = Bits::new(s_31_0 as u128, 1u16);
        // C s_31_2: const #1u : u8
        let s_31_2: bool = true;
        // C s_31_3: cast zx s_31_2 -> bv
        let s_31_3: Bits = Bits::new(s_31_2 as u128, 1u16);
        // D s_31_4: cmp-eq s_31_1 s_31_3
        let s_31_4: bool = ((s_31_1) == (s_31_3));
        // D s_31_5: write-var gs#69154 <= s_31_4
        fn_state.gs_69154 = s_31_4;
        // N s_31_6: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #424u : u32
        let s_32_0: u32 = 424;
        // D s_32_1: read-reg s_32_0:u8
        let s_32_1: u8 = {
            let value = state.read_register::<u8>(s_32_0 as isize);
            tracer.read_register(s_32_0 as isize, value);
            value
        };
        // C s_32_2: const #2u : u8
        let s_32_2: u8 = 2;
        // D s_32_3: cmp-lt s_32_1 s_32_2
        let s_32_3: bool = ((s_32_1) < (s_32_2));
        // D s_32_4: write-var gs#69153 <= s_32_3
        fn_state.gs_69153 = s_32_3;
        // N s_32_5: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #() : ()
        let s_33_0: () = ();
        // S s_33_1: call Halted(s_33_0)
        let s_33_1: bool = Halted(state, tracer, s_33_0);
        // N s_33_2: branch s_33_1 b90 b34
        if s_33_1 {
            return block_90(state, tracer, fn_state);
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
        // D s_34_1: write-var gs#69161 <= s_34_0
        fn_state.gs_69161 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#69161:u8
        let s_35_0: bool = fn_state.gs_69161;
        // N s_35_1: branch s_35_0 b89 b36
        if s_35_0 {
            return block_89(state, tracer, fn_state);
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
        // D s_36_1: write-var gs#69162 <= s_36_0
        fn_state.gs_69162 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#69162:u8
        let s_37_0: bool = fn_state.gs_69162;
        // N s_37_1: branch s_37_0 b88 b38
        if s_37_0 {
            return block_88(state, tracer, fn_state);
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
        // D s_38_1: write-var gs#69163 <= s_38_0
        fn_state.gs_69163 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#69163:u8
        let s_39_0: bool = fn_state.gs_69163;
        // N s_39_1: branch s_39_0 b87 b40
        if s_39_0 {
            return block_87(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #0u : u8
        let s_40_0: bool = false;
        // D s_40_1: write-var gs#69164 <= s_40_0
        fn_state.gs_69164 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var gs#69164:u8
        let s_41_0: bool = fn_state.gs_69164;
        // N s_41_1: branch s_41_0 b86 b42
        if s_41_0 {
            return block_86(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #() : ()
        let s_42_0: () = ();
        // S s_42_1: call EL2Enabled(s_42_0)
        let s_42_1: bool = EL2Enabled(state, tracer, s_42_0);
        // N s_42_2: branch s_42_1 b85 b43
        if s_42_1 {
            return block_85(state, tracer, fn_state);
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
        // D s_43_1: write-var gs#69165 <= s_43_0
        fn_state.gs_69165 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#69165:u8
        let s_44_0: bool = fn_state.gs_69165;
        // N s_44_1: branch s_44_0 b84 b45
        if s_44_0 {
            return block_84(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #() : ()
        let s_45_0: () = ();
        // S s_45_1: call EL2Enabled(s_45_0)
        let s_45_1: bool = EL2Enabled(state, tracer, s_45_0);
        // N s_45_2: branch s_45_1 b83 b46
        if s_45_1 {
            return block_83(state, tracer, fn_state);
        } else {
            return block_46(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #0u : u8
        let s_46_0: bool = false;
        // D s_46_1: write-var gs#69166 <= s_46_0
        fn_state.gs_69166 = s_46_0;
        // N s_46_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var gs#69166:u8
        let s_47_0: bool = fn_state.gs_69166;
        // N s_47_1: branch s_47_0 b79 b48
        if s_47_0 {
            return block_79(state, tracer, fn_state);
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
        // D s_48_1: write-var gs#69168 <= s_48_0
        fn_state.gs_69168 = s_48_0;
        // N s_48_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var gs#69168:u8
        let s_49_0: bool = fn_state.gs_69168;
        // N s_49_1: branch s_49_0 b78 b50
        if s_49_0 {
            return block_78(state, tracer, fn_state);
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
        // D s_50_1: write-var gs#69169 <= s_50_0
        fn_state.gs_69169 = s_50_0;
        // N s_50_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var gs#69169:u8
        let s_51_0: bool = fn_state.gs_69169;
        // N s_51_1: branch s_51_0 b77 b52
        if s_51_0 {
            return block_77(state, tracer, fn_state);
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
        // N s_52_2: branch s_52_1 b73 b53
        if s_52_1 {
            return block_73(state, tracer, fn_state);
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
        // D s_53_1: write-var gs#69171 <= s_53_0
        fn_state.gs_69171 = s_53_0;
        // N s_53_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var gs#69171:u8
        let s_54_0: bool = fn_state.gs_69171;
        // N s_54_1: branch s_54_0 b72 b55
        if s_54_0 {
            return block_72(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #424u : u32
        let s_55_0: u32 = 424;
        // D s_55_1: read-reg s_55_0:u8
        let s_55_1: u8 = {
            let value = state.read_register::<u8>(s_55_0 as isize);
            tracer.read_register(s_55_0 as isize, value);
            value
        };
        // C s_55_2: const #2u : u8
        let s_55_2: u8 = 2;
        // D s_55_3: cmp-lt s_55_1 s_55_2
        let s_55_3: bool = ((s_55_1) < (s_55_2));
        // N s_55_4: branch s_55_3 b71 b56
        if s_55_3 {
            return block_71(state, tracer, fn_state);
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
        // D s_56_1: write-var gs#69172 <= s_56_0
        fn_state.gs_69172 = s_56_0;
        // N s_56_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var gs#69172:u8
        let s_57_0: bool = fn_state.gs_69172;
        // N s_57_1: branch s_57_0 b65 b58
        if s_57_0 {
            return block_65(state, tracer, fn_state);
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
        // N s_58_2: branch s_58_1 b64 b59
        if s_58_1 {
            return block_64(state, tracer, fn_state);
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
        // D s_59_1: write-var gs#69173 <= s_59_0
        fn_state.gs_69173 = s_59_0;
        // N s_59_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var gs#69173:u8
        let s_60_0: bool = fn_state.gs_69173;
        // N s_60_1: branch s_60_0 b62 b61
        if s_60_0 {
            return block_62(state, tracer, fn_state);
        } else {
            return block_61(state, tracer, fn_state);
        };
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #64s : i64
        let s_61_0: i64 = 64;
        // C s_61_1: const #11720u : u32
        let s_61_1: u32 = 11720;
        // D s_61_2: read-reg s_61_1:struct
        let s_61_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_61_1 as isize);
            tracer.read_register(s_61_1 as isize, value);
            value
        };
        // D s_61_3: call __get_SCTLR2_EL1(s_61_2)
        let s_61_3: ProductType5c790c8ef59cc8b2 = u__get_SCTLR2_EL1(
            state,
            tracer,
            s_61_2,
        );
        // D s_61_4: write-var ga#84749 <= s_61_3
        fn_state.ga_84749 = s_61_3;
        // D s_61_5: read-var ga#84749.0:struct
        let s_61_5: u64 = fn_state.ga_84749._0;
        // D s_61_6: cast zx s_61_5 -> bv
        let s_61_6: Bits = Bits::new(s_61_5 as u128, 64u16);
        // D s_61_7: read-var t:i
        let s_61_7: i128 = fn_state.t;
        // D s_61_8: call X_set(s_61_7, s_61_0, s_61_6)
        let s_61_8: () = X_set(state, tracer, s_61_7, s_61_0, s_61_6);
        // N s_61_9: return
        return;
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #632u : u12
        let s_62_0: u16 = 632;
        // C s_62_1: cast zx s_62_0 -> bv
        let s_62_1: Bits = Bits::new(s_62_0 as u128, 12u16);
        // C s_62_2: cast zx s_62_1 -> i
        let s_62_2: i128 = (s_62_1.value() as i128);
        // C s_62_3: cast reint s_62_2 -> i64
        let s_62_3: i64 = (s_62_2 as i64);
        // C s_62_4: cast zx s_62_3 -> i
        let s_62_4: i128 = (i128::try_from(s_62_3).unwrap());
        // S s_62_5: call NVMem_read(s_62_4)
        let s_62_5: u64 = NVMem_read(state, tracer, s_62_4);
        // D s_62_6: write-var ga#84748 <= s_62_5
        fn_state.ga_84748 = s_62_5;
        // N s_62_7: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_63_0: read-var ga#84748:u64
        let s_63_0: u64 = fn_state.ga_84748;
        // D s_63_1: cast zx s_63_0 -> bv
        let s_63_1: Bits = Bits::new(s_63_0 as u128, 64u16);
        // D s_63_2: read-var t:i
        let s_63_2: i128 = fn_state.t;
        // C s_63_3: const #64s : i64
        let s_63_3: i64 = 64;
        // D s_63_4: call X_set(s_63_2, s_63_3, s_63_1)
        let s_63_4: () = X_set(state, tracer, s_63_2, s_63_3, s_63_1);
        // N s_63_5: return
        return;
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #102552u : u32
        let s_64_0: u32 = 102552;
        // D s_64_1: read-reg s_64_0:struct
        let s_64_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_64_0 as isize);
            tracer.read_register(s_64_0 as isize, value);
            value
        };
        // D s_64_2: call _get_HCR_EL2_Type_NV2(s_64_1)
        let s_64_2: bool = u_get_HCR_EL2_Type_NV2(state, tracer, s_64_1);
        // C s_64_3: const #102552u : u32
        let s_64_3: u32 = 102552;
        // D s_64_4: read-reg s_64_3:struct
        let s_64_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_64_3 as isize);
            tracer.read_register(s_64_3 as isize, value);
            value
        };
        // D s_64_5: call _get_HCR_EL2_Type_NV1(s_64_4)
        let s_64_5: bool = u_get_HCR_EL2_Type_NV1(state, tracer, s_64_4);
        // C s_64_6: const #102552u : u32
        let s_64_6: u32 = 102552;
        // D s_64_7: read-reg s_64_6:struct
        let s_64_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_64_6 as isize);
            tracer.read_register(s_64_6 as isize, value);
            value
        };
        // D s_64_8: call _get_HCR_EL2_Type_NV(s_64_7)
        let s_64_8: bool = u_get_HCR_EL2_Type_NV(state, tracer, s_64_7);
        // D s_64_9: cast zx s_64_5 -> bv
        let s_64_9: Bits = Bits::new(s_64_5 as u128, 1u16);
        // D s_64_10: cast zx s_64_8 -> bv
        let s_64_10: Bits = Bits::new(s_64_8 as u128, 1u16);
        // D s_64_11: cast reint s_64_9 -> u128
        let s_64_11: u128 = (s_64_9.value() as u128);
        // D s_64_12: size-of s_64_9
        let s_64_12: u16 = s_64_9.length();
        // D s_64_13: cast reint s_64_10 -> u128
        let s_64_13: u128 = (s_64_10.value() as u128);
        // D s_64_14: size-of s_64_10
        let s_64_14: u16 = s_64_10.length();
        // D s_64_15: lsl s_64_11 s_64_14
        let s_64_15: u128 = s_64_11 << s_64_14;
        // D s_64_16: or s_64_15 s_64_13
        let s_64_16: u128 = ((s_64_15) | (s_64_13));
        // D s_64_17: add s_64_12 s_64_14
        let s_64_17: u16 = (s_64_12 + s_64_14);
        // D s_64_18: create-bits s_64_16 s_64_17
        let s_64_18: Bits = Bits::new(s_64_16, s_64_17);
        // D s_64_19: cast reint s_64_18 -> u8
        let s_64_19: u8 = (s_64_18.value() as u8);
        // D s_64_20: cast zx s_64_2 -> bv
        let s_64_20: Bits = Bits::new(s_64_2 as u128, 1u16);
        // D s_64_21: cast zx s_64_19 -> bv
        let s_64_21: Bits = Bits::new(s_64_19 as u128, 2u16);
        // D s_64_22: cast reint s_64_20 -> u128
        let s_64_22: u128 = (s_64_20.value() as u128);
        // D s_64_23: size-of s_64_20
        let s_64_23: u16 = s_64_20.length();
        // D s_64_24: cast reint s_64_21 -> u128
        let s_64_24: u128 = (s_64_21.value() as u128);
        // D s_64_25: size-of s_64_21
        let s_64_25: u16 = s_64_21.length();
        // D s_64_26: lsl s_64_22 s_64_25
        let s_64_26: u128 = s_64_22 << s_64_25;
        // D s_64_27: or s_64_26 s_64_24
        let s_64_27: u128 = ((s_64_26) | (s_64_24));
        // D s_64_28: add s_64_23 s_64_25
        let s_64_28: u16 = (s_64_23 + s_64_25);
        // D s_64_29: create-bits s_64_27 s_64_28
        let s_64_29: Bits = Bits::new(s_64_27, s_64_28);
        // D s_64_30: cast reint s_64_29 -> u8
        let s_64_30: u8 = (s_64_29.value() as u8);
        // D s_64_31: cast zx s_64_30 -> bv
        let s_64_31: Bits = Bits::new(s_64_30 as u128, 3u16);
        // C s_64_32: const #7u : u8
        let s_64_32: u8 = 7;
        // C s_64_33: cast zx s_64_32 -> bv
        let s_64_33: Bits = Bits::new(s_64_32 as u128, 3u16);
        // D s_64_34: cmp-eq s_64_31 s_64_33
        let s_64_34: bool = ((s_64_31) == (s_64_33));
        // D s_64_35: write-var gs#69173 <= s_64_34
        fn_state.gs_69173 = s_64_34;
        // N s_64_36: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #() : ()
        let s_65_0: () = ();
        // S s_65_1: call Halted(s_65_0)
        let s_65_1: bool = Halted(state, tracer, s_65_0);
        // N s_65_2: branch s_65_1 b70 b66
        if s_65_1 {
            return block_70(state, tracer, fn_state);
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
        // D s_66_1: write-var gs#69176 <= s_66_0
        fn_state.gs_69176 = s_66_0;
        // N s_66_2: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var gs#69176:u8
        let s_67_0: bool = fn_state.gs_69176;
        // N s_67_1: branch s_67_0 b69 b68
        if s_67_0 {
            return block_69(state, tracer, fn_state);
        } else {
            return block_68(state, tracer, fn_state);
        };
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #24u : u8
        let s_68_0: u8 = 24;
        // C s_68_1: cast zx s_68_0 -> bv
        let s_68_1: Bits = Bits::new(s_68_0 as u128, 8u16);
        // C s_68_2: cast zx s_68_1 -> i
        let s_68_2: i128 = (s_68_1.value() as i128);
        // C s_68_3: cast reint s_68_2 -> i64
        let s_68_3: i64 = (s_68_2 as i64);
        // C s_68_4: cast zx s_68_3 -> i
        let s_68_4: i128 = (i128::try_from(s_68_3).unwrap());
        // C s_68_5: const #424u : u32
        let s_68_5: u32 = 424;
        // D s_68_6: read-reg s_68_5:u8
        let s_68_6: u8 = {
            let value = state.read_register::<u8>(s_68_5 as isize);
            tracer.read_register(s_68_5 as isize, value);
            value
        };
        // D s_68_7: call AArch64_SystemAccessTrap(s_68_6, s_68_4)
        let s_68_7: () = AArch64_SystemAccessTrap(state, tracer, s_68_6, s_68_4);
        // N s_68_8: return
        return;
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_69_0: panic
        panic!("{:?}", ());
        // N s_69_1: return
        return;
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_70_0: read-var __EDSCR_SDD:u8
        let s_70_0: bool = fn_state.u__EDSCR_SDD;
        // D s_70_1: cast zx s_70_0 -> bv
        let s_70_1: Bits = Bits::new(s_70_0 as u128, 1u16);
        // C s_70_2: const #1u : u8
        let s_70_2: bool = true;
        // C s_70_3: cast zx s_70_2 -> bv
        let s_70_3: Bits = Bits::new(s_70_2 as u128, 1u16);
        // D s_70_4: cmp-eq s_70_1 s_70_3
        let s_70_4: bool = ((s_70_1) == (s_70_3));
        // D s_70_5: write-var gs#69176 <= s_70_4
        fn_state.gs_69176 = s_70_4;
        // N s_70_6: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_71_0: read-var __SCR_EL3_SCTLR2En:u8
        let s_71_0: bool = fn_state.u__SCR_EL3_SCTLR2En;
        // D s_71_1: cast zx s_71_0 -> bv
        let s_71_1: Bits = Bits::new(s_71_0 as u128, 1u16);
        // C s_71_2: const #0u : u8
        let s_71_2: bool = false;
        // C s_71_3: cast zx s_71_2 -> bv
        let s_71_3: Bits = Bits::new(s_71_2 as u128, 1u16);
        // D s_71_4: cmp-eq s_71_1 s_71_3
        let s_71_4: bool = ((s_71_1) == (s_71_3));
        // D s_71_5: write-var gs#69172 <= s_71_4
        fn_state.gs_69172 = s_71_4;
        // N s_71_6: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #24u : u8
        let s_72_0: u8 = 24;
        // C s_72_1: cast zx s_72_0 -> bv
        let s_72_1: Bits = Bits::new(s_72_0 as u128, 8u16);
        // C s_72_2: cast zx s_72_1 -> i
        let s_72_2: i128 = (s_72_1.value() as i128);
        // C s_72_3: cast reint s_72_2 -> i64
        let s_72_3: i64 = (s_72_2 as i64);
        // C s_72_4: cast zx s_72_3 -> i
        let s_72_4: i128 = (i128::try_from(s_72_3).unwrap());
        // C s_72_5: const #432u : u32
        let s_72_5: u32 = 432;
        // D s_72_6: read-reg s_72_5:u8
        let s_72_6: u8 = {
            let value = state.read_register::<u8>(s_72_5 as isize);
            tracer.read_register(s_72_5 as isize, value);
            value
        };
        // D s_72_7: call AArch64_SystemAccessTrap(s_72_6, s_72_4)
        let s_72_7: () = AArch64_SystemAccessTrap(state, tracer, s_72_6, s_72_4);
        // N s_72_8: return
        return;
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #() : ()
        let s_73_0: () = ();
        // S s_73_1: call IsHCRXEL2Enabled(s_73_0)
        let s_73_1: bool = IsHCRXEL2Enabled(state, tracer, s_73_0);
        // S s_73_2: not s_73_1
        let s_73_2: bool = !s_73_1;
        // N s_73_3: branch s_73_2 b76 b74
        if s_73_2 {
            return block_76(state, tracer, fn_state);
        } else {
            return block_74(state, tracer, fn_state);
        };
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_74_0: read-var __HCRX_EL2_SCTLR2En:u8
        let s_74_0: bool = fn_state.u__HCRX_EL2_SCTLR2En;
        // D s_74_1: cast zx s_74_0 -> bv
        let s_74_1: Bits = Bits::new(s_74_0 as u128, 1u16);
        // C s_74_2: const #0u : u8
        let s_74_2: bool = false;
        // C s_74_3: cast zx s_74_2 -> bv
        let s_74_3: Bits = Bits::new(s_74_2 as u128, 1u16);
        // D s_74_4: cmp-eq s_74_1 s_74_3
        let s_74_4: bool = ((s_74_1) == (s_74_3));
        // D s_74_5: write-var gs#69170 <= s_74_4
        fn_state.gs_69170 = s_74_4;
        // N s_74_6: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_75_0: read-var gs#69170:u8
        let s_75_0: bool = fn_state.gs_69170;
        // D s_75_1: write-var gs#69171 <= s_75_0
        fn_state.gs_69171 = s_75_0;
        // N s_75_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #1u : u8
        let s_76_0: bool = true;
        // D s_76_1: write-var gs#69170 <= s_76_0
        fn_state.gs_69170 = s_76_0;
        // N s_76_2: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #24u : u8
        let s_77_0: u8 = 24;
        // C s_77_1: cast zx s_77_0 -> bv
        let s_77_1: Bits = Bits::new(s_77_0 as u128, 8u16);
        // C s_77_2: cast zx s_77_1 -> i
        let s_77_2: i128 = (s_77_1.value() as i128);
        // C s_77_3: cast reint s_77_2 -> i64
        let s_77_3: i64 = (s_77_2 as i64);
        // C s_77_4: cast zx s_77_3 -> i
        let s_77_4: i128 = (i128::try_from(s_77_3).unwrap());
        // C s_77_5: const #432u : u32
        let s_77_5: u32 = 432;
        // D s_77_6: read-reg s_77_5:u8
        let s_77_6: u8 = {
            let value = state.read_register::<u8>(s_77_5 as isize);
            tracer.read_register(s_77_5 as isize, value);
            value
        };
        // D s_77_7: call AArch64_SystemAccessTrap(s_77_6, s_77_4)
        let s_77_7: () = AArch64_SystemAccessTrap(state, tracer, s_77_6, s_77_4);
        // N s_77_8: return
        return;
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_78_0: read-var __HFGRTR_EL2_SCTLR_EL1:u8
        let s_78_0: bool = fn_state.u__HFGRTR_EL2_SCTLR_EL1;
        // D s_78_1: cast zx s_78_0 -> bv
        let s_78_1: Bits = Bits::new(s_78_0 as u128, 1u16);
        // C s_78_2: const #1u : u8
        let s_78_2: bool = true;
        // C s_78_3: cast zx s_78_2 -> bv
        let s_78_3: Bits = Bits::new(s_78_2 as u128, 1u16);
        // D s_78_4: cmp-eq s_78_1 s_78_3
        let s_78_4: bool = ((s_78_1) == (s_78_3));
        // D s_78_5: write-var gs#69169 <= s_78_4
        fn_state.gs_69169 = s_78_4;
        // N s_78_6: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #424u : u32
        let s_79_0: u32 = 424;
        // D s_79_1: read-reg s_79_0:u8
        let s_79_1: u8 = {
            let value = state.read_register::<u8>(s_79_0 as isize);
            tracer.read_register(s_79_0 as isize, value);
            value
        };
        // C s_79_2: const #2u : u8
        let s_79_2: u8 = 2;
        // D s_79_3: cmp-lt s_79_1 s_79_2
        let s_79_3: bool = ((s_79_1) < (s_79_2));
        // D s_79_4: not s_79_3
        let s_79_4: bool = !s_79_3;
        // N s_79_5: branch s_79_4 b82 b80
        if s_79_4 {
            return block_82(state, tracer, fn_state);
        } else {
            return block_80(state, tracer, fn_state);
        };
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_80_0: read-var __SCR_EL3_FGTEn:u8
        let s_80_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_80_1: cast zx s_80_0 -> bv
        let s_80_1: Bits = Bits::new(s_80_0 as u128, 1u16);
        // C s_80_2: const #1u : u8
        let s_80_2: bool = true;
        // C s_80_3: cast zx s_80_2 -> bv
        let s_80_3: Bits = Bits::new(s_80_2 as u128, 1u16);
        // D s_80_4: cmp-eq s_80_1 s_80_3
        let s_80_4: bool = ((s_80_1) == (s_80_3));
        // D s_80_5: write-var gs#69167 <= s_80_4
        fn_state.gs_69167 = s_80_4;
        // N s_80_6: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_81_0: read-var gs#69167:u8
        let s_81_0: bool = fn_state.gs_69167;
        // D s_81_1: write-var gs#69168 <= s_81_0
        fn_state.gs_69168 = s_81_0;
        // N s_81_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #1u : u8
        let s_82_0: bool = true;
        // D s_82_1: write-var gs#69167 <= s_82_0
        fn_state.gs_69167 = s_82_0;
        // N s_82_2: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #146u : u32
        let s_83_0: u32 = 146;
        // S s_83_1: call IsFeatureImplemented(s_83_0)
        let s_83_1: bool = IsFeatureImplemented(state, tracer, s_83_0);
        // D s_83_2: write-var gs#69166 <= s_83_1
        fn_state.gs_69166 = s_83_1;
        // N s_83_3: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_84_0: const #24u : u8
        let s_84_0: u8 = 24;
        // C s_84_1: cast zx s_84_0 -> bv
        let s_84_1: Bits = Bits::new(s_84_0 as u128, 8u16);
        // C s_84_2: cast zx s_84_1 -> i
        let s_84_2: i128 = (s_84_1.value() as i128);
        // C s_84_3: cast reint s_84_2 -> i64
        let s_84_3: i64 = (s_84_2 as i64);
        // C s_84_4: cast zx s_84_3 -> i
        let s_84_4: i128 = (i128::try_from(s_84_3).unwrap());
        // C s_84_5: const #432u : u32
        let s_84_5: u32 = 432;
        // D s_84_6: read-reg s_84_5:u8
        let s_84_6: u8 = {
            let value = state.read_register::<u8>(s_84_5 as isize);
            tracer.read_register(s_84_5 as isize, value);
            value
        };
        // D s_84_7: call AArch64_SystemAccessTrap(s_84_6, s_84_4)
        let s_84_7: () = AArch64_SystemAccessTrap(state, tracer, s_84_6, s_84_4);
        // N s_84_8: return
        return;
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_85_0: read-var __HCR_EL2_TRVM:u8
        let s_85_0: bool = fn_state.u__HCR_EL2_TRVM;
        // D s_85_1: cast zx s_85_0 -> bv
        let s_85_1: Bits = Bits::new(s_85_0 as u128, 1u16);
        // C s_85_2: const #1u : u8
        let s_85_2: bool = true;
        // C s_85_3: cast zx s_85_2 -> bv
        let s_85_3: Bits = Bits::new(s_85_2 as u128, 1u16);
        // D s_85_4: cmp-eq s_85_1 s_85_3
        let s_85_4: bool = ((s_85_1) == (s_85_3));
        // D s_85_5: write-var gs#69165 <= s_85_4
        fn_state.gs_69165 = s_85_4;
        // N s_85_6: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_86_0: panic
        panic!("{:?}", ());
        // N s_86_1: return
        return;
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_87_0: read-var __SCR_EL3_SCTLR2En:u8
        let s_87_0: bool = fn_state.u__SCR_EL3_SCTLR2En;
        // D s_87_1: cast zx s_87_0 -> bv
        let s_87_1: Bits = Bits::new(s_87_0 as u128, 1u16);
        // C s_87_2: const #0u : u8
        let s_87_2: bool = false;
        // C s_87_3: cast zx s_87_2 -> bv
        let s_87_3: Bits = Bits::new(s_87_2 as u128, 1u16);
        // D s_87_4: cmp-eq s_87_1 s_87_3
        let s_87_4: bool = ((s_87_1) == (s_87_3));
        // D s_87_5: write-var gs#69164 <= s_87_4
        fn_state.gs_69164 = s_87_4;
        // N s_87_6: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_88_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_88_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_88_1: call __IMPDEF_boolean(s_88_0)
        let s_88_1: bool = u__IMPDEF_boolean(state, tracer, s_88_0);
        // D s_88_2: write-var gs#69163 <= s_88_1
        fn_state.gs_69163 = s_88_1;
        // N s_88_3: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_89_0: read-var __EDSCR_SDD:u8
        let s_89_0: bool = fn_state.u__EDSCR_SDD;
        // D s_89_1: cast zx s_89_0 -> bv
        let s_89_1: Bits = Bits::new(s_89_0 as u128, 1u16);
        // C s_89_2: const #1u : u8
        let s_89_2: bool = true;
        // C s_89_3: cast zx s_89_2 -> bv
        let s_89_3: Bits = Bits::new(s_89_2 as u128, 1u16);
        // D s_89_4: cmp-eq s_89_1 s_89_3
        let s_89_4: bool = ((s_89_1) == (s_89_3));
        // D s_89_5: write-var gs#69162 <= s_89_4
        fn_state.gs_69162 = s_89_4;
        // N s_89_6: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_90_0: const #424u : u32
        let s_90_0: u32 = 424;
        // D s_90_1: read-reg s_90_0:u8
        let s_90_1: u8 = {
            let value = state.read_register::<u8>(s_90_0 as isize);
            tracer.read_register(s_90_0 as isize, value);
            value
        };
        // C s_90_2: const #2u : u8
        let s_90_2: u8 = 2;
        // D s_90_3: cmp-lt s_90_1 s_90_2
        let s_90_3: bool = ((s_90_1) < (s_90_2));
        // D s_90_4: write-var gs#69161 <= s_90_3
        fn_state.gs_69161 = s_90_3;
        // N s_90_5: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_91_0: panic
        panic!("{:?}", ());
        // N s_91_1: return
        return;
    }
}
