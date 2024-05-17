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
use u_get_SCR_EL3_Type_FGTEn::*;
use Halted::*;
use u_get_MDCR_EL3_Type_TPM::*;
use AArch32_GetNumEventCountersAccessible::*;
use u_get_PMUSERENR_Type_EN::*;
use u_get_EDSCR_Type_SDD::*;
use EL2Enabled::*;
use u_get_HCR_EL2_Type_TGE::*;
use PMEVCNTR_read::*;
use u_get_HDFGRTR_EL2_Type_PMEVCNTRn_EL0::*;
use u_get_MDCR_EL2_Type_TPM::*;
use u_get_PMUSERENR_EL0_Type_UEN::*;
use u_get_HCR_EL2_Type_E2H::*;
use HCR_read::*;
use HDCR_read::*;
use PMUSERENR_read::*;
use IsFeatureImplemented::*;
use u__IMPDEF_boolean::*;
use u_get_PMUSERENR_EL0_Type_ER::*;
use AArch64_AArch32SystemAccessTrap::*;
use u_get_HDCR_Type_TPM::*;
use u_get_HCR_Type_TGE::*;
use ConstrainUnpredictableProcedure::*;
use u_get_PMUSERENR_Type_ER::*;
use R_set::*;
use ELUsingAArch32::*;
use u_get_PMUSERENR_EL0_Type_EN::*;
use EDSCR_read::*;
use AArch32_TakeHypTrapException::*;
use common::*;
pub fn PMEVCNTR_SysRegRead32_63ff5bc2709c8d46<T: Tracer>(
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
        gs_114098: bool,
        gs_114086: bool,
        gs_114064: bool,
        gs_114060: bool,
        gs_114102: bool,
        gs_114083: bool,
        gs_114076: bool,
        u__MDCR_EL3_TPM: bool,
        gs_114089: bool,
        gs_114079: bool,
        gs_114097: bool,
        gs_114088: bool,
        gs_114063: bool,
        gs_114074: bool,
        gs_114087: bool,
        u__HCR_TGE: bool,
        gs_114080: bool,
        gs_114084: bool,
        gs_114099: bool,
        gs_114085: bool,
        gs_114066: bool,
        gs_114094: bool,
        gs_114075: bool,
        gs_114071: bool,
        gs_114047: bool,
        gs_114049: bool,
        gs_114052: bool,
        u__PSTATE_EL: u8,
        gs_114090: bool,
        gs_114061: bool,
        gs_114056: bool,
        u__MDCR_EL2_TPM: bool,
        gs_114077: bool,
        gs_114068: bool,
        gs_114082: bool,
        u__HCR_EL2_TGE: bool,
        gs_114058: bool,
        gs_114067: bool,
        gs_114100: bool,
        gs_114055: bool,
        gs_114051: bool,
        u__SCR_EL3_FGTEn: bool,
        gs_114092: bool,
        gs_114070: bool,
        gs_114101: bool,
        gs_114050: bool,
        gs_114078: bool,
        u__HDCR_TPM: bool,
        gs_114057: bool,
        gs_114072: bool,
        gs_114053: bool,
        gs_114062: bool,
        gs_114073: bool,
        u__HDFGRTR_EL2_PMEVCNTRn_EL0: bool,
        gs_114093: bool,
        gs_114059: bool,
        gs_114096: bool,
        gs_114081: bool,
        gs_114048: bool,
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
        // C s_0_3: const #22712u : u32
        let s_0_3: u32 = 22712;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_MDCR_EL3_Type_TPM(s_0_4)
        let s_0_5: bool = u_get_MDCR_EL3_Type_TPM(state, tracer, s_0_4);
        // D s_0_6: write-var __MDCR_EL3_TPM <= s_0_5
        fn_state.u__MDCR_EL3_TPM = s_0_5;
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
        // S s_0_12: call HCR_read(s_0_11)
        let s_0_12: ProductType700c18a878c5601b = HCR_read(state, tracer, s_0_11);
        // S s_0_13: call _get_HCR_Type_TGE(s_0_12)
        let s_0_13: bool = u_get_HCR_Type_TGE(state, tracer, s_0_12);
        // D s_0_14: write-var __HCR_TGE <= s_0_13
        fn_state.u__HCR_TGE = s_0_13;
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
        // C s_0_19: const #19144u : u32
        let s_0_19: u32 = 19144;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_HDFGRTR_EL2_Type_PMEVCNTRn_EL0(s_0_20)
        let s_0_21: bool = u_get_HDFGRTR_EL2_Type_PMEVCNTRn_EL0(state, tracer, s_0_20);
        // D s_0_22: write-var __HDFGRTR_EL2_PMEVCNTRn_EL0 <= s_0_21
        fn_state.u__HDFGRTR_EL2_PMEVCNTRn_EL0 = s_0_21;
        // C s_0_23: const #104880u : u32
        let s_0_23: u32 = 104880;
        // D s_0_24: read-reg s_0_23:struct
        let s_0_24: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_23 as isize);
            tracer.read_register(s_0_23 as isize, value);
            value
        };
        // D s_0_25: call _get_MDCR_EL2_Type_TPM(s_0_24)
        let s_0_25: bool = u_get_MDCR_EL2_Type_TPM(state, tracer, s_0_24);
        // D s_0_26: write-var __MDCR_EL2_TPM <= s_0_25
        fn_state.u__MDCR_EL2_TPM = s_0_25;
        // C s_0_27: const #() : ()
        let s_0_27: () = ();
        // S s_0_28: call HDCR_read(s_0_27)
        let s_0_28: ProductType700c18a878c5601b = HDCR_read(state, tracer, s_0_27);
        // S s_0_29: call _get_HDCR_Type_TPM(s_0_28)
        let s_0_29: bool = u_get_HDCR_Type_TPM(state, tracer, s_0_28);
        // D s_0_30: write-var __HDCR_TPM <= s_0_29
        fn_state.u__HDCR_TPM = s_0_29;
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
        // N s_0_37: branch s_0_36 b93 b1
        if s_0_36 {
            return block_93(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b37 b2
        if s_1_5 {
            return block_37(state, tracer, fn_state);
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
        // C s_5_0: const #21s : i64
        let s_5_0: i64 = 21;
        // S s_5_1: call PMEVCNTR_read(s_5_0)
        let s_5_1: u32 = PMEVCNTR_read(state, tracer, s_5_0);
        // D s_5_2: read-var t:i
        let s_5_2: i128 = fn_state.t;
        // D s_5_3: call R_set(s_5_2, s_5_1)
        let s_5_3: () = R_set(state, tracer, s_5_2, s_5_1);
        // N s_5_4: return
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
        // N s_6_2: branch s_6_1 b36 b7
        if s_6_1 {
            return block_36(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#114047 <= s_7_0
        fn_state.gs_114047 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#114047:u8
        let s_8_0: bool = fn_state.gs_114047;
        // N s_8_1: branch s_8_0 b35 b9
        if s_8_0 {
            return block_35(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#114048 <= s_9_0
        fn_state.gs_114048 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#114048:u8
        let s_10_0: bool = fn_state.gs_114048;
        // N s_10_1: branch s_10_0 b34 b11
        if s_10_0 {
            return block_34(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#114049 <= s_11_0
        fn_state.gs_114049 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#114049:u8
        let s_12_0: bool = fn_state.gs_114049;
        // N s_12_1: branch s_12_0 b33 b13
        if s_12_0 {
            return block_33(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#114050 <= s_13_0
        fn_state.gs_114050 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#114050:u8
        let s_14_0: bool = fn_state.gs_114050;
        // N s_14_1: branch s_14_0 b32 b15
        if s_14_0 {
            return block_32(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#114051 <= s_15_0
        fn_state.gs_114051 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#114051:u8
        let s_16_0: bool = fn_state.gs_114051;
        // N s_16_1: branch s_16_0 b31 b17
        if s_16_0 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #424u : u32
        let s_17_0: u32 = 424;
        // D s_17_1: read-reg s_17_0:u8
        let s_17_1: u8 = {
            let value = state.read_register::<u8>(s_17_0 as isize);
            tracer.read_register(s_17_0 as isize, value);
            value
        };
        // C s_17_2: const #2u : u8
        let s_17_2: u8 = 2;
        // D s_17_3: cmp-lt s_17_1 s_17_2
        let s_17_3: bool = ((s_17_1) < (s_17_2));
        // N s_17_4: branch s_17_3 b30 b18
        if s_17_3 {
            return block_30(state, tracer, fn_state);
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
        // D s_18_1: write-var gs#114052 <= s_18_0
        fn_state.gs_114052 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#114052:u8
        let s_19_0: bool = fn_state.gs_114052;
        // N s_19_1: branch s_19_0 b29 b20
        if s_19_0 {
            return block_29(state, tracer, fn_state);
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
        // D s_20_1: write-var gs#114053 <= s_20_0
        fn_state.gs_114053 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#114053:u8
        let s_21_0: bool = fn_state.gs_114053;
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
        // C s_22_0: const #21s : i64
        let s_22_0: i64 = 21;
        // S s_22_1: call PMEVCNTR_read(s_22_0)
        let s_22_1: u32 = PMEVCNTR_read(state, tracer, s_22_0);
        // D s_22_2: read-var t:i
        let s_22_2: i128 = fn_state.t;
        // D s_22_3: call R_set(s_22_2, s_22_1)
        let s_22_3: () = R_set(state, tracer, s_22_2, s_22_1);
        // N s_22_4: return
        return;
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #() : ()
        let s_23_0: () = ();
        // S s_23_1: call Halted(s_23_0)
        let s_23_1: bool = Halted(state, tracer, s_23_0);
        // N s_23_2: branch s_23_1 b28 b24
        if s_23_1 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #0u : u8
        let s_24_0: bool = false;
        // D s_24_1: write-var gs#114055 <= s_24_0
        fn_state.gs_114055 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#114055:u8
        let s_25_0: bool = fn_state.gs_114055;
        // N s_25_1: branch s_25_0 b27 b26
        if s_25_0 {
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
        // C s_26_5: const #424u : u32
        let s_26_5: u32 = 424;
        // D s_26_6: read-reg s_26_5:u8
        let s_26_6: u8 = {
            let value = state.read_register::<u8>(s_26_5 as isize);
            tracer.read_register(s_26_5 as isize, value);
            value
        };
        // D s_26_7: call AArch64_AArch32SystemAccessTrap(s_26_6, s_26_4)
        let s_26_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_26_6, s_26_4);
        // N s_26_8: return
        return;
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
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #() : ()
        let s_28_0: () = ();
        // S s_28_1: call EDSCR_read(s_28_0)
        let s_28_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_28_0);
        // S s_28_2: call _get_EDSCR_Type_SDD(s_28_1)
        let s_28_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_28_1);
        // S s_28_3: cast zx s_28_2 -> bv
        let s_28_3: Bits = Bits::new(s_28_2 as u128, 1u16);
        // C s_28_4: const #1u : u8
        let s_28_4: bool = true;
        // C s_28_5: cast zx s_28_4 -> bv
        let s_28_5: Bits = Bits::new(s_28_4 as u128, 1u16);
        // S s_28_6: cmp-eq s_28_3 s_28_5
        let s_28_6: bool = ((s_28_3) == (s_28_5));
        // D s_28_7: write-var gs#114055 <= s_28_6
        fn_state.gs_114055 = s_28_6;
        // N s_28_8: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var __MDCR_EL3_TPM:u8
        let s_29_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_29_1: cast zx s_29_0 -> bv
        let s_29_1: Bits = Bits::new(s_29_0 as u128, 1u16);
        // C s_29_2: const #1u : u8
        let s_29_2: bool = true;
        // C s_29_3: cast zx s_29_2 -> bv
        let s_29_3: Bits = Bits::new(s_29_2 as u128, 1u16);
        // D s_29_4: cmp-eq s_29_1 s_29_3
        let s_29_4: bool = ((s_29_1) == (s_29_3));
        // D s_29_5: write-var gs#114053 <= s_29_4
        fn_state.gs_114053 = s_29_4;
        // N s_29_6: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #424u : u32
        let s_30_0: u32 = 424;
        // D s_30_1: read-reg s_30_0:u8
        let s_30_1: u8 = {
            let value = state.read_register::<u8>(s_30_0 as isize);
            tracer.read_register(s_30_0 as isize, value);
            value
        };
        // D s_30_2: call ELUsingAArch32(s_30_1)
        let s_30_2: bool = ELUsingAArch32(state, tracer, s_30_1);
        // D s_30_3: not s_30_2
        let s_30_3: bool = !s_30_2;
        // D s_30_4: write-var gs#114052 <= s_30_3
        fn_state.gs_114052 = s_30_3;
        // N s_30_5: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_31_0: panic
        panic!("{:?}", ());
        // N s_31_1: return
        return;
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_32_0: read-var __MDCR_EL3_TPM:u8
        let s_32_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_32_1: cast zx s_32_0 -> bv
        let s_32_1: Bits = Bits::new(s_32_0 as u128, 1u16);
        // C s_32_2: const #1u : u8
        let s_32_2: bool = true;
        // C s_32_3: cast zx s_32_2 -> bv
        let s_32_3: Bits = Bits::new(s_32_2 as u128, 1u16);
        // D s_32_4: cmp-eq s_32_1 s_32_3
        let s_32_4: bool = ((s_32_1) == (s_32_3));
        // D s_32_5: write-var gs#114051 <= s_32_4
        fn_state.gs_114051 = s_32_4;
        // N s_32_6: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #424u : u32
        let s_33_0: u32 = 424;
        // D s_33_1: read-reg s_33_0:u8
        let s_33_1: u8 = {
            let value = state.read_register::<u8>(s_33_0 as isize);
            tracer.read_register(s_33_0 as isize, value);
            value
        };
        // D s_33_2: call ELUsingAArch32(s_33_1)
        let s_33_2: bool = ELUsingAArch32(state, tracer, s_33_1);
        // D s_33_3: not s_33_2
        let s_33_3: bool = !s_33_2;
        // D s_33_4: write-var gs#114050 <= s_33_3
        fn_state.gs_114050 = s_33_3;
        // N s_33_5: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_34_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_34_1: call __IMPDEF_boolean(s_34_0)
        let s_34_1: bool = u__IMPDEF_boolean(state, tracer, s_34_0);
        // D s_34_2: write-var gs#114049 <= s_34_1
        fn_state.gs_114049 = s_34_1;
        // N s_34_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #() : ()
        let s_35_0: () = ();
        // S s_35_1: call EDSCR_read(s_35_0)
        let s_35_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_35_0);
        // S s_35_2: call _get_EDSCR_Type_SDD(s_35_1)
        let s_35_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_35_1);
        // S s_35_3: cast zx s_35_2 -> bv
        let s_35_3: Bits = Bits::new(s_35_2 as u128, 1u16);
        // C s_35_4: const #1u : u8
        let s_35_4: bool = true;
        // C s_35_5: cast zx s_35_4 -> bv
        let s_35_5: Bits = Bits::new(s_35_4 as u128, 1u16);
        // S s_35_6: cmp-eq s_35_3 s_35_5
        let s_35_6: bool = ((s_35_3) == (s_35_5));
        // D s_35_7: write-var gs#114048 <= s_35_6
        fn_state.gs_114048 = s_35_6;
        // N s_35_8: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #424u : u32
        let s_36_0: u32 = 424;
        // D s_36_1: read-reg s_36_0:u8
        let s_36_1: u8 = {
            let value = state.read_register::<u8>(s_36_0 as isize);
            tracer.read_register(s_36_0 as isize, value);
            value
        };
        // C s_36_2: const #2u : u8
        let s_36_2: u8 = 2;
        // D s_36_3: cmp-lt s_36_1 s_36_2
        let s_36_3: bool = ((s_36_1) < (s_36_2));
        // D s_36_4: write-var gs#114047 <= s_36_3
        fn_state.gs_114047 = s_36_3;
        // N s_36_5: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #() : ()
        let s_37_0: () = ();
        // S s_37_1: call Halted(s_37_0)
        let s_37_1: bool = Halted(state, tracer, s_37_0);
        // N s_37_2: branch s_37_1 b92 b38
        if s_37_1 {
            return block_92(state, tracer, fn_state);
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
        // D s_38_1: write-var gs#114056 <= s_38_0
        fn_state.gs_114056 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#114056:u8
        let s_39_0: bool = fn_state.gs_114056;
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
        // C s_40_0: const #0u : u8
        let s_40_0: bool = false;
        // D s_40_1: write-var gs#114057 <= s_40_0
        fn_state.gs_114057 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var gs#114057:u8
        let s_41_0: bool = fn_state.gs_114057;
        // N s_41_1: branch s_41_0 b90 b42
        if s_41_0 {
            return block_90(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #0u : u8
        let s_42_0: bool = false;
        // D s_42_1: write-var gs#114058 <= s_42_0
        fn_state.gs_114058 = s_42_0;
        // N s_42_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var gs#114058:u8
        let s_43_0: bool = fn_state.gs_114058;
        // N s_43_1: branch s_43_0 b89 b44
        if s_43_0 {
            return block_89(state, tracer, fn_state);
        } else {
            return block_44(state, tracer, fn_state);
        };
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #0u : u8
        let s_44_0: bool = false;
        // D s_44_1: write-var gs#114059 <= s_44_0
        fn_state.gs_114059 = s_44_0;
        // N s_44_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var gs#114059:u8
        let s_45_0: bool = fn_state.gs_114059;
        // N s_45_1: branch s_45_0 b88 b46
        if s_45_0 {
            return block_88(state, tracer, fn_state);
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
        // D s_46_1: write-var gs#114060 <= s_46_0
        fn_state.gs_114060 = s_46_0;
        // N s_46_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var gs#114060:u8
        let s_47_0: bool = fn_state.gs_114060;
        // N s_47_1: branch s_47_0 b87 b48
        if s_47_0 {
            return block_87(state, tracer, fn_state);
        } else {
            return block_48(state, tracer, fn_state);
        };
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
        // N s_48_2: branch s_48_1 b86 b49
        if s_48_1 {
            return block_86(state, tracer, fn_state);
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
        // D s_49_1: write-var gs#114061 <= s_49_0
        fn_state.gs_114061 = s_49_0;
        // N s_49_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var gs#114061:u8
        let s_50_0: bool = fn_state.gs_114061;
        // N s_50_1: branch s_50_0 b85 b51
        if s_50_0 {
            return block_85(state, tracer, fn_state);
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
        // D s_51_1: write-var gs#114062 <= s_51_0
        fn_state.gs_114062 = s_51_0;
        // N s_51_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var gs#114062:u8
        let s_52_0: bool = fn_state.gs_114062;
        // N s_52_1: branch s_52_0 b84 b53
        if s_52_0 {
            return block_84(state, tracer, fn_state);
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
        // N s_53_2: branch s_53_1 b83 b54
        if s_53_1 {
            return block_83(state, tracer, fn_state);
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
        // D s_54_1: write-var gs#114063 <= s_54_0
        fn_state.gs_114063 = s_54_0;
        // N s_54_2: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var gs#114063:u8
        let s_55_0: bool = fn_state.gs_114063;
        // N s_55_1: branch s_55_0 b82 b56
        if s_55_0 {
            return block_82(state, tracer, fn_state);
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
        // D s_56_1: write-var gs#114064 <= s_56_0
        fn_state.gs_114064 = s_56_0;
        // N s_56_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var gs#114064:u8
        let s_57_0: bool = fn_state.gs_114064;
        // N s_57_1: branch s_57_0 b81 b58
        if s_57_0 {
            return block_81(state, tracer, fn_state);
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
        // N s_58_2: branch s_58_1 b80 b59
        if s_58_1 {
            return block_80(state, tracer, fn_state);
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
        // D s_59_1: write-var gs#114066 <= s_59_0
        fn_state.gs_114066 = s_59_0;
        // N s_59_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var gs#114066:u8
        let s_60_0: bool = fn_state.gs_114066;
        // N s_60_1: branch s_60_0 b75 b61
        if s_60_0 {
            return block_75(state, tracer, fn_state);
        } else {
            return block_61(state, tracer, fn_state);
        };
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #424u : u32
        let s_61_0: u32 = 424;
        // D s_61_1: read-reg s_61_0:u8
        let s_61_1: u8 = {
            let value = state.read_register::<u8>(s_61_0 as isize);
            tracer.read_register(s_61_0 as isize, value);
            value
        };
        // C s_61_2: const #2u : u8
        let s_61_2: u8 = 2;
        // D s_61_3: cmp-lt s_61_1 s_61_2
        let s_61_3: bool = ((s_61_1) < (s_61_2));
        // N s_61_4: branch s_61_3 b74 b62
        if s_61_3 {
            return block_74(state, tracer, fn_state);
        } else {
            return block_62(state, tracer, fn_state);
        };
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #0u : u8
        let s_62_0: bool = false;
        // D s_62_1: write-var gs#114067 <= s_62_0
        fn_state.gs_114067 = s_62_0;
        // N s_62_2: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_63_0: read-var gs#114067:u8
        let s_63_0: bool = fn_state.gs_114067;
        // N s_63_1: branch s_63_0 b73 b64
        if s_63_0 {
            return block_73(state, tracer, fn_state);
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
        // D s_64_1: write-var gs#114068 <= s_64_0
        fn_state.gs_114068 = s_64_0;
        // N s_64_2: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_65_0: read-var gs#114068:u8
        let s_65_0: bool = fn_state.gs_114068;
        // N s_65_1: branch s_65_0 b67 b66
        if s_65_0 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_66(state, tracer, fn_state);
        };
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #21s : i64
        let s_66_0: i64 = 21;
        // S s_66_1: call PMEVCNTR_read(s_66_0)
        let s_66_1: u32 = PMEVCNTR_read(state, tracer, s_66_0);
        // D s_66_2: read-var t:i
        let s_66_2: i128 = fn_state.t;
        // D s_66_3: call R_set(s_66_2, s_66_1)
        let s_66_3: () = R_set(state, tracer, s_66_2, s_66_1);
        // N s_66_4: return
        return;
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #() : ()
        let s_67_0: () = ();
        // S s_67_1: call Halted(s_67_0)
        let s_67_1: bool = Halted(state, tracer, s_67_0);
        // N s_67_2: branch s_67_1 b72 b68
        if s_67_1 {
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
        // D s_68_1: write-var gs#114070 <= s_68_0
        fn_state.gs_114070 = s_68_0;
        // N s_68_2: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_69_0: read-var gs#114070:u8
        let s_69_0: bool = fn_state.gs_114070;
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
        // C s_70_0: const #3u : u8
        let s_70_0: u8 = 3;
        // C s_70_1: cast zx s_70_0 -> bv
        let s_70_1: Bits = Bits::new(s_70_0 as u128, 8u16);
        // C s_70_2: cast zx s_70_1 -> i
        let s_70_2: i128 = (s_70_1.value() as i128);
        // C s_70_3: cast reint s_70_2 -> i64
        let s_70_3: i64 = (s_70_2 as i64);
        // C s_70_4: cast zx s_70_3 -> i
        let s_70_4: i128 = (i128::try_from(s_70_3).unwrap());
        // C s_70_5: const #424u : u32
        let s_70_5: u32 = 424;
        // D s_70_6: read-reg s_70_5:u8
        let s_70_6: u8 = {
            let value = state.read_register::<u8>(s_70_5 as isize);
            tracer.read_register(s_70_5 as isize, value);
            value
        };
        // D s_70_7: call AArch64_AArch32SystemAccessTrap(s_70_6, s_70_4)
        let s_70_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_70_6, s_70_4);
        // N s_70_8: return
        return;
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_71_0: panic
        panic!("{:?}", ());
        // N s_71_1: return
        return;
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #() : ()
        let s_72_0: () = ();
        // S s_72_1: call EDSCR_read(s_72_0)
        let s_72_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_72_0);
        // S s_72_2: call _get_EDSCR_Type_SDD(s_72_1)
        let s_72_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_72_1);
        // S s_72_3: cast zx s_72_2 -> bv
        let s_72_3: Bits = Bits::new(s_72_2 as u128, 1u16);
        // C s_72_4: const #1u : u8
        let s_72_4: bool = true;
        // C s_72_5: cast zx s_72_4 -> bv
        let s_72_5: Bits = Bits::new(s_72_4 as u128, 1u16);
        // S s_72_6: cmp-eq s_72_3 s_72_5
        let s_72_6: bool = ((s_72_3) == (s_72_5));
        // D s_72_7: write-var gs#114070 <= s_72_6
        fn_state.gs_114070 = s_72_6;
        // N s_72_8: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_73_0: read-var __MDCR_EL3_TPM:u8
        let s_73_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_73_1: cast zx s_73_0 -> bv
        let s_73_1: Bits = Bits::new(s_73_0 as u128, 1u16);
        // C s_73_2: const #1u : u8
        let s_73_2: bool = true;
        // C s_73_3: cast zx s_73_2 -> bv
        let s_73_3: Bits = Bits::new(s_73_2 as u128, 1u16);
        // D s_73_4: cmp-eq s_73_1 s_73_3
        let s_73_4: bool = ((s_73_1) == (s_73_3));
        // D s_73_5: write-var gs#114068 <= s_73_4
        fn_state.gs_114068 = s_73_4;
        // N s_73_6: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #424u : u32
        let s_74_0: u32 = 424;
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
        // D s_74_4: write-var gs#114067 <= s_74_3
        fn_state.gs_114067 = s_74_3;
        // N s_74_5: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #146u : u32
        let s_75_0: u32 = 146;
        // S s_75_1: call IsFeatureImplemented(s_75_0)
        let s_75_1: bool = IsFeatureImplemented(state, tracer, s_75_0);
        // S s_75_2: not s_75_1
        let s_75_2: bool = !s_75_1;
        // N s_75_3: branch s_75_2 b79 b76
        if s_75_2 {
            return block_79(state, tracer, fn_state);
        } else {
            return block_76(state, tracer, fn_state);
        };
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #432u : u32
        let s_76_0: u32 = 432;
        // D s_76_1: read-reg s_76_0:u8
        let s_76_1: u8 = {
            let value = state.read_register::<u8>(s_76_0 as isize);
            tracer.read_register(s_76_0 as isize, value);
            value
        };
        // D s_76_2: call ELUsingAArch32(s_76_1)
        let s_76_2: bool = ELUsingAArch32(state, tracer, s_76_1);
        // N s_76_3: branch s_76_2 b78 b77
        if s_76_2 {
            return block_78(state, tracer, fn_state);
        } else {
            return block_77(state, tracer, fn_state);
        };
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #3u : u8
        let s_77_0: u8 = 3;
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
        // D s_77_7: call AArch64_AArch32SystemAccessTrap(s_77_6, s_77_4)
        let s_77_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_77_6, s_77_4);
        // N s_77_8: return
        return;
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_78_0: const #3u : u8
        let s_78_0: u8 = 3;
        // C s_78_1: cast zx s_78_0 -> bv
        let s_78_1: Bits = Bits::new(s_78_0 as u128, 8u16);
        // C s_78_2: cast zx s_78_1 -> i
        let s_78_2: i128 = (s_78_1.value() as i128);
        // C s_78_3: cast reint s_78_2 -> i64
        let s_78_3: i64 = (s_78_2 as i64);
        // C s_78_4: cast zx s_78_3 -> i
        let s_78_4: i128 = (i128::try_from(s_78_3).unwrap());
        // S s_78_5: call AArch32_TakeHypTrapException(s_78_4)
        let s_78_5: () = AArch32_TakeHypTrapException(state, tracer, s_78_4);
        // N s_78_6: return
        return;
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #72u : u32
        let s_79_0: u32 = 72;
        // S s_79_1: call ConstrainUnpredictableProcedure(s_79_0)
        let s_79_1: () = ConstrainUnpredictableProcedure(state, tracer, s_79_0);
        // N s_79_2: return
        return;
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #() : ()
        let s_80_0: () = ();
        // S s_80_1: call AArch32_GetNumEventCountersAccessible(s_80_0)
        let s_80_1: i128 = AArch32_GetNumEventCountersAccessible(state, tracer, s_80_0);
        // C s_80_2: const #21s : i
        let s_80_2: i128 = 21;
        // S s_80_3: cmp-ge s_80_2 s_80_1
        let s_80_3: bool = ((s_80_2) >= (s_80_1));
        // D s_80_4: write-var gs#114066 <= s_80_3
        fn_state.gs_114066 = s_80_3;
        // N s_80_5: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #3u : u8
        let s_81_0: u8 = 3;
        // C s_81_1: cast zx s_81_0 -> bv
        let s_81_1: Bits = Bits::new(s_81_0 as u128, 8u16);
        // C s_81_2: cast zx s_81_1 -> i
        let s_81_2: i128 = (s_81_1.value() as i128);
        // C s_81_3: cast reint s_81_2 -> i64
        let s_81_3: i64 = (s_81_2 as i64);
        // C s_81_4: cast zx s_81_3 -> i
        let s_81_4: i128 = (i128::try_from(s_81_3).unwrap());
        // S s_81_5: call AArch32_TakeHypTrapException(s_81_4)
        let s_81_5: () = AArch32_TakeHypTrapException(state, tracer, s_81_4);
        // N s_81_6: return
        return;
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_82_0: read-var __HDCR_TPM:u8
        let s_82_0: bool = fn_state.u__HDCR_TPM;
        // D s_82_1: cast zx s_82_0 -> bv
        let s_82_1: Bits = Bits::new(s_82_0 as u128, 1u16);
        // C s_82_2: const #1u : u8
        let s_82_2: bool = true;
        // C s_82_3: cast zx s_82_2 -> bv
        let s_82_3: Bits = Bits::new(s_82_2 as u128, 1u16);
        // D s_82_4: cmp-eq s_82_1 s_82_3
        let s_82_4: bool = ((s_82_1) == (s_82_3));
        // D s_82_5: write-var gs#114064 <= s_82_4
        fn_state.gs_114064 = s_82_4;
        // N s_82_6: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #432u : u32
        let s_83_0: u32 = 432;
        // D s_83_1: read-reg s_83_0:u8
        let s_83_1: u8 = {
            let value = state.read_register::<u8>(s_83_0 as isize);
            tracer.read_register(s_83_0 as isize, value);
            value
        };
        // D s_83_2: call ELUsingAArch32(s_83_1)
        let s_83_2: bool = ELUsingAArch32(state, tracer, s_83_1);
        // D s_83_3: write-var gs#114063 <= s_83_2
        fn_state.gs_114063 = s_83_2;
        // N s_83_4: jump b55
        return block_55(state, tracer, fn_state);
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
        // C s_84_5: const #432u : u32
        let s_84_5: u32 = 432;
        // D s_84_6: read-reg s_84_5:u8
        let s_84_6: u8 = {
            let value = state.read_register::<u8>(s_84_5 as isize);
            tracer.read_register(s_84_5 as isize, value);
            value
        };
        // D s_84_7: call AArch64_AArch32SystemAccessTrap(s_84_6, s_84_4)
        let s_84_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_84_6, s_84_4);
        // N s_84_8: return
        return;
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_85_0: read-var __MDCR_EL2_TPM:u8
        let s_85_0: bool = fn_state.u__MDCR_EL2_TPM;
        // D s_85_1: cast zx s_85_0 -> bv
        let s_85_1: Bits = Bits::new(s_85_0 as u128, 1u16);
        // C s_85_2: const #1u : u8
        let s_85_2: bool = true;
        // C s_85_3: cast zx s_85_2 -> bv
        let s_85_3: Bits = Bits::new(s_85_2 as u128, 1u16);
        // D s_85_4: cmp-eq s_85_1 s_85_3
        let s_85_4: bool = ((s_85_1) == (s_85_3));
        // D s_85_5: write-var gs#114062 <= s_85_4
        fn_state.gs_114062 = s_85_4;
        // N s_85_6: jump b52
        return block_52(state, tracer, fn_state);
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
        // D s_86_3: not s_86_2
        let s_86_3: bool = !s_86_2;
        // D s_86_4: write-var gs#114061 <= s_86_3
        fn_state.gs_114061 = s_86_3;
        // N s_86_5: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_87_0: panic
        panic!("{:?}", ());
        // N s_87_1: return
        return;
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_88_0: read-var __MDCR_EL3_TPM:u8
        let s_88_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_88_1: cast zx s_88_0 -> bv
        let s_88_1: Bits = Bits::new(s_88_0 as u128, 1u16);
        // C s_88_2: const #1u : u8
        let s_88_2: bool = true;
        // C s_88_3: cast zx s_88_2 -> bv
        let s_88_3: Bits = Bits::new(s_88_2 as u128, 1u16);
        // D s_88_4: cmp-eq s_88_1 s_88_3
        let s_88_4: bool = ((s_88_1) == (s_88_3));
        // D s_88_5: write-var gs#114060 <= s_88_4
        fn_state.gs_114060 = s_88_4;
        // N s_88_6: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_89_0: const #424u : u32
        let s_89_0: u32 = 424;
        // D s_89_1: read-reg s_89_0:u8
        let s_89_1: u8 = {
            let value = state.read_register::<u8>(s_89_0 as isize);
            tracer.read_register(s_89_0 as isize, value);
            value
        };
        // D s_89_2: call ELUsingAArch32(s_89_1)
        let s_89_2: bool = ELUsingAArch32(state, tracer, s_89_1);
        // D s_89_3: not s_89_2
        let s_89_3: bool = !s_89_2;
        // D s_89_4: write-var gs#114059 <= s_89_3
        fn_state.gs_114059 = s_89_3;
        // N s_89_5: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_90_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_90_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_90_1: call __IMPDEF_boolean(s_90_0)
        let s_90_1: bool = u__IMPDEF_boolean(state, tracer, s_90_0);
        // D s_90_2: write-var gs#114058 <= s_90_1
        fn_state.gs_114058 = s_90_1;
        // N s_90_3: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_91_0: const #() : ()
        let s_91_0: () = ();
        // S s_91_1: call EDSCR_read(s_91_0)
        let s_91_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_91_0);
        // S s_91_2: call _get_EDSCR_Type_SDD(s_91_1)
        let s_91_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_91_1);
        // S s_91_3: cast zx s_91_2 -> bv
        let s_91_3: Bits = Bits::new(s_91_2 as u128, 1u16);
        // C s_91_4: const #1u : u8
        let s_91_4: bool = true;
        // C s_91_5: cast zx s_91_4 -> bv
        let s_91_5: Bits = Bits::new(s_91_4 as u128, 1u16);
        // S s_91_6: cmp-eq s_91_3 s_91_5
        let s_91_6: bool = ((s_91_3) == (s_91_5));
        // D s_91_7: write-var gs#114057 <= s_91_6
        fn_state.gs_114057 = s_91_6;
        // N s_91_8: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_92_0: const #424u : u32
        let s_92_0: u32 = 424;
        // D s_92_1: read-reg s_92_0:u8
        let s_92_1: u8 = {
            let value = state.read_register::<u8>(s_92_0 as isize);
            tracer.read_register(s_92_0 as isize, value);
            value
        };
        // C s_92_2: const #2u : u8
        let s_92_2: u8 = 2;
        // D s_92_3: cmp-lt s_92_1 s_92_2
        let s_92_3: bool = ((s_92_1) < (s_92_2));
        // D s_92_4: write-var gs#114056 <= s_92_3
        fn_state.gs_114056 = s_92_3;
        // N s_92_5: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_93_0: const #() : ()
        let s_93_0: () = ();
        // S s_93_1: call Halted(s_93_0)
        let s_93_1: bool = Halted(state, tracer, s_93_0);
        // N s_93_2: branch s_93_1 b211 b94
        if s_93_1 {
            return block_211(state, tracer, fn_state);
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
        // D s_94_1: write-var gs#114071 <= s_94_0
        fn_state.gs_114071 = s_94_0;
        // N s_94_2: jump b95
        return block_95(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_95_0: read-var gs#114071:u8
        let s_95_0: bool = fn_state.gs_114071;
        // N s_95_1: branch s_95_0 b210 b96
        if s_95_0 {
            return block_210(state, tracer, fn_state);
        } else {
            return block_96(state, tracer, fn_state);
        };
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_96_0: const #0u : u8
        let s_96_0: bool = false;
        // D s_96_1: write-var gs#114072 <= s_96_0
        fn_state.gs_114072 = s_96_0;
        // N s_96_2: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_97_0: read-var gs#114072:u8
        let s_97_0: bool = fn_state.gs_114072;
        // N s_97_1: branch s_97_0 b209 b98
        if s_97_0 {
            return block_209(state, tracer, fn_state);
        } else {
            return block_98(state, tracer, fn_state);
        };
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_98_0: const #0u : u8
        let s_98_0: bool = false;
        // D s_98_1: write-var gs#114073 <= s_98_0
        fn_state.gs_114073 = s_98_0;
        // N s_98_2: jump b99
        return block_99(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_99_0: read-var gs#114073:u8
        let s_99_0: bool = fn_state.gs_114073;
        // N s_99_1: branch s_99_0 b208 b100
        if s_99_0 {
            return block_208(state, tracer, fn_state);
        } else {
            return block_100(state, tracer, fn_state);
        };
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_100_0: const #0u : u8
        let s_100_0: bool = false;
        // D s_100_1: write-var gs#114074 <= s_100_0
        fn_state.gs_114074 = s_100_0;
        // N s_100_2: jump b101
        return block_101(state, tracer, fn_state);
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_101_0: read-var gs#114074:u8
        let s_101_0: bool = fn_state.gs_114074;
        // N s_101_1: branch s_101_0 b207 b102
        if s_101_0 {
            return block_207(state, tracer, fn_state);
        } else {
            return block_102(state, tracer, fn_state);
        };
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_102_0: const #0u : u8
        let s_102_0: bool = false;
        // D s_102_1: write-var gs#114075 <= s_102_0
        fn_state.gs_114075 = s_102_0;
        // N s_102_2: jump b103
        return block_103(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_103_0: read-var gs#114075:u8
        let s_103_0: bool = fn_state.gs_114075;
        // N s_103_1: branch s_103_0 b206 b104
        if s_103_0 {
            return block_206(state, tracer, fn_state);
        } else {
            return block_104(state, tracer, fn_state);
        };
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_104_0: const #440u : u32
        let s_104_0: u32 = 440;
        // D s_104_1: read-reg s_104_0:u8
        let s_104_1: u8 = {
            let value = state.read_register::<u8>(s_104_0 as isize);
            tracer.read_register(s_104_0 as isize, value);
            value
        };
        // D s_104_2: call ELUsingAArch32(s_104_1)
        let s_104_2: bool = ELUsingAArch32(state, tracer, s_104_1);
        // D s_104_3: not s_104_2
        let s_104_3: bool = !s_104_2;
        // N s_104_4: branch s_104_3 b196 b105
        if s_104_3 {
            return block_196(state, tracer, fn_state);
        } else {
            return block_105(state, tracer, fn_state);
        };
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_105_0: const #0u : u8
        let s_105_0: bool = false;
        // D s_105_1: write-var gs#114079 <= s_105_0
        fn_state.gs_114079 = s_105_0;
        // N s_105_2: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_106_0: read-var gs#114079:u8
        let s_106_0: bool = fn_state.gs_114079;
        // N s_106_1: branch s_106_0 b187 b107
        if s_106_0 {
            return block_187(state, tracer, fn_state);
        } else {
            return block_107(state, tracer, fn_state);
        };
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_107_0: const #440u : u32
        let s_107_0: u32 = 440;
        // D s_107_1: read-reg s_107_0:u8
        let s_107_1: u8 = {
            let value = state.read_register::<u8>(s_107_0 as isize);
            tracer.read_register(s_107_0 as isize, value);
            value
        };
        // D s_107_2: call ELUsingAArch32(s_107_1)
        let s_107_2: bool = ELUsingAArch32(state, tracer, s_107_1);
        // N s_107_3: branch s_107_2 b186 b108
        if s_107_2 {
            return block_186(state, tracer, fn_state);
        } else {
            return block_108(state, tracer, fn_state);
        };
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_108_0: const #0u : u8
        let s_108_0: bool = false;
        // D s_108_1: write-var gs#114080 <= s_108_0
        fn_state.gs_114080 = s_108_0;
        // N s_108_2: jump b109
        return block_109(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_109_0: read-var gs#114080:u8
        let s_109_0: bool = fn_state.gs_114080;
        // N s_109_1: branch s_109_0 b169 b110
        if s_109_0 {
            return block_169(state, tracer, fn_state);
        } else {
            return block_110(state, tracer, fn_state);
        };
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
        // N s_110_2: branch s_110_1 b168 b111
        if s_110_1 {
            return block_168(state, tracer, fn_state);
        } else {
            return block_111(state, tracer, fn_state);
        };
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_111_0: const #0u : u8
        let s_111_0: bool = false;
        // D s_111_1: write-var gs#114081 <= s_111_0
        fn_state.gs_114081 = s_111_0;
        // N s_111_2: jump b112
        return block_112(state, tracer, fn_state);
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_112_0: read-var gs#114081:u8
        let s_112_0: bool = fn_state.gs_114081;
        // N s_112_1: branch s_112_0 b167 b113
        if s_112_0 {
            return block_167(state, tracer, fn_state);
        } else {
            return block_113(state, tracer, fn_state);
        };
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_113_0: const #0u : u8
        let s_113_0: bool = false;
        // D s_113_1: write-var gs#114082 <= s_113_0
        fn_state.gs_114082 = s_113_0;
        // N s_113_2: jump b114
        return block_114(state, tracer, fn_state);
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_114_0: read-var gs#114082:u8
        let s_114_0: bool = fn_state.gs_114082;
        // N s_114_1: branch s_114_0 b166 b115
        if s_114_0 {
            return block_166(state, tracer, fn_state);
        } else {
            return block_115(state, tracer, fn_state);
        };
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_115_0: const #0u : u8
        let s_115_0: bool = false;
        // D s_115_1: write-var gs#114083 <= s_115_0
        fn_state.gs_114083 = s_115_0;
        // N s_115_2: jump b116
        return block_116(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_116_0: read-var gs#114083:u8
        let s_116_0: bool = fn_state.gs_114083;
        // N s_116_1: branch s_116_0 b162 b117
        if s_116_0 {
            return block_162(state, tracer, fn_state);
        } else {
            return block_117(state, tracer, fn_state);
        };
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_117_0: const #0u : u8
        let s_117_0: bool = false;
        // D s_117_1: write-var gs#114085 <= s_117_0
        fn_state.gs_114085 = s_117_0;
        // N s_117_2: jump b118
        return block_118(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_118_0: read-var gs#114085:u8
        let s_118_0: bool = fn_state.gs_114085;
        // N s_118_1: branch s_118_0 b161 b119
        if s_118_0 {
            return block_161(state, tracer, fn_state);
        } else {
            return block_119(state, tracer, fn_state);
        };
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_119_0: const #0u : u8
        let s_119_0: bool = false;
        // D s_119_1: write-var gs#114086 <= s_119_0
        fn_state.gs_114086 = s_119_0;
        // N s_119_2: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_120_0: read-var gs#114086:u8
        let s_120_0: bool = fn_state.gs_114086;
        // N s_120_1: branch s_120_0 b160 b121
        if s_120_0 {
            return block_160(state, tracer, fn_state);
        } else {
            return block_121(state, tracer, fn_state);
        };
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_121_0: const #() : ()
        let s_121_0: () = ();
        // S s_121_1: call EL2Enabled(s_121_0)
        let s_121_1: bool = EL2Enabled(state, tracer, s_121_0);
        // N s_121_2: branch s_121_1 b159 b122
        if s_121_1 {
            return block_159(state, tracer, fn_state);
        } else {
            return block_122(state, tracer, fn_state);
        };
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_122_0: const #0u : u8
        let s_122_0: bool = false;
        // D s_122_1: write-var gs#114087 <= s_122_0
        fn_state.gs_114087 = s_122_0;
        // N s_122_2: jump b123
        return block_123(state, tracer, fn_state);
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_123_0: read-var gs#114087:u8
        let s_123_0: bool = fn_state.gs_114087;
        // N s_123_1: branch s_123_0 b158 b124
        if s_123_0 {
            return block_158(state, tracer, fn_state);
        } else {
            return block_124(state, tracer, fn_state);
        };
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_124_0: const #0u : u8
        let s_124_0: bool = false;
        // D s_124_1: write-var gs#114088 <= s_124_0
        fn_state.gs_114088 = s_124_0;
        // N s_124_2: jump b125
        return block_125(state, tracer, fn_state);
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_125_0: read-var gs#114088:u8
        let s_125_0: bool = fn_state.gs_114088;
        // N s_125_1: branch s_125_0 b157 b126
        if s_125_0 {
            return block_157(state, tracer, fn_state);
        } else {
            return block_126(state, tracer, fn_state);
        };
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_126_0: const #() : ()
        let s_126_0: () = ();
        // S s_126_1: call EL2Enabled(s_126_0)
        let s_126_1: bool = EL2Enabled(state, tracer, s_126_0);
        // N s_126_2: branch s_126_1 b156 b127
        if s_126_1 {
            return block_156(state, tracer, fn_state);
        } else {
            return block_127(state, tracer, fn_state);
        };
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_127_0: const #0u : u8
        let s_127_0: bool = false;
        // D s_127_1: write-var gs#114089 <= s_127_0
        fn_state.gs_114089 = s_127_0;
        // N s_127_2: jump b128
        return block_128(state, tracer, fn_state);
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_128_0: read-var gs#114089:u8
        let s_128_0: bool = fn_state.gs_114089;
        // N s_128_1: branch s_128_0 b155 b129
        if s_128_0 {
            return block_155(state, tracer, fn_state);
        } else {
            return block_129(state, tracer, fn_state);
        };
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_129_0: const #0u : u8
        let s_129_0: bool = false;
        // D s_129_1: write-var gs#114090 <= s_129_0
        fn_state.gs_114090 = s_129_0;
        // N s_129_2: jump b130
        return block_130(state, tracer, fn_state);
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_130_0: read-var gs#114090:u8
        let s_130_0: bool = fn_state.gs_114090;
        // N s_130_1: branch s_130_0 b154 b131
        if s_130_0 {
            return block_154(state, tracer, fn_state);
        } else {
            return block_131(state, tracer, fn_state);
        };
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
        // N s_131_2: branch s_131_1 b153 b132
        if s_131_1 {
            return block_153(state, tracer, fn_state);
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
        // D s_132_1: write-var gs#114092 <= s_132_0
        fn_state.gs_114092 = s_132_0;
        // N s_132_2: jump b133
        return block_133(state, tracer, fn_state);
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_133_0: read-var gs#114092:u8
        let s_133_0: bool = fn_state.gs_114092;
        // N s_133_1: branch s_133_0 b148 b134
        if s_133_0 {
            return block_148(state, tracer, fn_state);
        } else {
            return block_134(state, tracer, fn_state);
        };
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_134_0: const #424u : u32
        let s_134_0: u32 = 424;
        // D s_134_1: read-reg s_134_0:u8
        let s_134_1: u8 = {
            let value = state.read_register::<u8>(s_134_0 as isize);
            tracer.read_register(s_134_0 as isize, value);
            value
        };
        // C s_134_2: const #2u : u8
        let s_134_2: u8 = 2;
        // D s_134_3: cmp-lt s_134_1 s_134_2
        let s_134_3: bool = ((s_134_1) < (s_134_2));
        // N s_134_4: branch s_134_3 b147 b135
        if s_134_3 {
            return block_147(state, tracer, fn_state);
        } else {
            return block_135(state, tracer, fn_state);
        };
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_135_0: const #0u : u8
        let s_135_0: bool = false;
        // D s_135_1: write-var gs#114093 <= s_135_0
        fn_state.gs_114093 = s_135_0;
        // N s_135_2: jump b136
        return block_136(state, tracer, fn_state);
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_136_0: read-var gs#114093:u8
        let s_136_0: bool = fn_state.gs_114093;
        // N s_136_1: branch s_136_0 b146 b137
        if s_136_0 {
            return block_146(state, tracer, fn_state);
        } else {
            return block_137(state, tracer, fn_state);
        };
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_137_0: const #0u : u8
        let s_137_0: bool = false;
        // D s_137_1: write-var gs#114094 <= s_137_0
        fn_state.gs_114094 = s_137_0;
        // N s_137_2: jump b138
        return block_138(state, tracer, fn_state);
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_138_0: read-var gs#114094:u8
        let s_138_0: bool = fn_state.gs_114094;
        // N s_138_1: branch s_138_0 b140 b139
        if s_138_0 {
            return block_140(state, tracer, fn_state);
        } else {
            return block_139(state, tracer, fn_state);
        };
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_139_0: const #21s : i64
        let s_139_0: i64 = 21;
        // S s_139_1: call PMEVCNTR_read(s_139_0)
        let s_139_1: u32 = PMEVCNTR_read(state, tracer, s_139_0);
        // D s_139_2: read-var t:i
        let s_139_2: i128 = fn_state.t;
        // D s_139_3: call R_set(s_139_2, s_139_1)
        let s_139_3: () = R_set(state, tracer, s_139_2, s_139_1);
        // N s_139_4: return
        return;
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_140_0: const #() : ()
        let s_140_0: () = ();
        // S s_140_1: call Halted(s_140_0)
        let s_140_1: bool = Halted(state, tracer, s_140_0);
        // N s_140_2: branch s_140_1 b145 b141
        if s_140_1 {
            return block_145(state, tracer, fn_state);
        } else {
            return block_141(state, tracer, fn_state);
        };
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_141_0: const #0u : u8
        let s_141_0: bool = false;
        // D s_141_1: write-var gs#114096 <= s_141_0
        fn_state.gs_114096 = s_141_0;
        // N s_141_2: jump b142
        return block_142(state, tracer, fn_state);
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_142_0: read-var gs#114096:u8
        let s_142_0: bool = fn_state.gs_114096;
        // N s_142_1: branch s_142_0 b144 b143
        if s_142_0 {
            return block_144(state, tracer, fn_state);
        } else {
            return block_143(state, tracer, fn_state);
        };
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_143_0: const #3u : u8
        let s_143_0: u8 = 3;
        // C s_143_1: cast zx s_143_0 -> bv
        let s_143_1: Bits = Bits::new(s_143_0 as u128, 8u16);
        // C s_143_2: cast zx s_143_1 -> i
        let s_143_2: i128 = (s_143_1.value() as i128);
        // C s_143_3: cast reint s_143_2 -> i64
        let s_143_3: i64 = (s_143_2 as i64);
        // C s_143_4: cast zx s_143_3 -> i
        let s_143_4: i128 = (i128::try_from(s_143_3).unwrap());
        // C s_143_5: const #424u : u32
        let s_143_5: u32 = 424;
        // D s_143_6: read-reg s_143_5:u8
        let s_143_6: u8 = {
            let value = state.read_register::<u8>(s_143_5 as isize);
            tracer.read_register(s_143_5 as isize, value);
            value
        };
        // D s_143_7: call AArch64_AArch32SystemAccessTrap(s_143_6, s_143_4)
        let s_143_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_143_6,
            s_143_4,
        );
        // N s_143_8: return
        return;
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_144_0: panic
        panic!("{:?}", ());
        // N s_144_1: return
        return;
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_145_0: const #() : ()
        let s_145_0: () = ();
        // S s_145_1: call EDSCR_read(s_145_0)
        let s_145_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_145_0);
        // S s_145_2: call _get_EDSCR_Type_SDD(s_145_1)
        let s_145_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_145_1);
        // S s_145_3: cast zx s_145_2 -> bv
        let s_145_3: Bits = Bits::new(s_145_2 as u128, 1u16);
        // C s_145_4: const #1u : u8
        let s_145_4: bool = true;
        // C s_145_5: cast zx s_145_4 -> bv
        let s_145_5: Bits = Bits::new(s_145_4 as u128, 1u16);
        // S s_145_6: cmp-eq s_145_3 s_145_5
        let s_145_6: bool = ((s_145_3) == (s_145_5));
        // D s_145_7: write-var gs#114096 <= s_145_6
        fn_state.gs_114096 = s_145_6;
        // N s_145_8: jump b142
        return block_142(state, tracer, fn_state);
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_146_0: read-var __MDCR_EL3_TPM:u8
        let s_146_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_146_1: cast zx s_146_0 -> bv
        let s_146_1: Bits = Bits::new(s_146_0 as u128, 1u16);
        // C s_146_2: const #1u : u8
        let s_146_2: bool = true;
        // C s_146_3: cast zx s_146_2 -> bv
        let s_146_3: Bits = Bits::new(s_146_2 as u128, 1u16);
        // D s_146_4: cmp-eq s_146_1 s_146_3
        let s_146_4: bool = ((s_146_1) == (s_146_3));
        // D s_146_5: write-var gs#114094 <= s_146_4
        fn_state.gs_114094 = s_146_4;
        // N s_146_6: jump b138
        return block_138(state, tracer, fn_state);
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_147_0: const #424u : u32
        let s_147_0: u32 = 424;
        // D s_147_1: read-reg s_147_0:u8
        let s_147_1: u8 = {
            let value = state.read_register::<u8>(s_147_0 as isize);
            tracer.read_register(s_147_0 as isize, value);
            value
        };
        // D s_147_2: call ELUsingAArch32(s_147_1)
        let s_147_2: bool = ELUsingAArch32(state, tracer, s_147_1);
        // D s_147_3: not s_147_2
        let s_147_3: bool = !s_147_2;
        // D s_147_4: write-var gs#114093 <= s_147_3
        fn_state.gs_114093 = s_147_3;
        // N s_147_5: jump b136
        return block_136(state, tracer, fn_state);
    }
    fn block_148<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_148_0: const #146u : u32
        let s_148_0: u32 = 146;
        // S s_148_1: call IsFeatureImplemented(s_148_0)
        let s_148_1: bool = IsFeatureImplemented(state, tracer, s_148_0);
        // S s_148_2: not s_148_1
        let s_148_2: bool = !s_148_1;
        // N s_148_3: branch s_148_2 b152 b149
        if s_148_2 {
            return block_152(state, tracer, fn_state);
        } else {
            return block_149(state, tracer, fn_state);
        };
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_149_0: const #440u : u32
        let s_149_0: u32 = 440;
        // D s_149_1: read-reg s_149_0:u8
        let s_149_1: u8 = {
            let value = state.read_register::<u8>(s_149_0 as isize);
            tracer.read_register(s_149_0 as isize, value);
            value
        };
        // D s_149_2: call ELUsingAArch32(s_149_1)
        let s_149_2: bool = ELUsingAArch32(state, tracer, s_149_1);
        // N s_149_3: branch s_149_2 b151 b150
        if s_149_2 {
            return block_151(state, tracer, fn_state);
        } else {
            return block_150(state, tracer, fn_state);
        };
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_150_0: const #3u : u8
        let s_150_0: u8 = 3;
        // C s_150_1: cast zx s_150_0 -> bv
        let s_150_1: Bits = Bits::new(s_150_0 as u128, 8u16);
        // C s_150_2: cast zx s_150_1 -> i
        let s_150_2: i128 = (s_150_1.value() as i128);
        // C s_150_3: cast reint s_150_2 -> i64
        let s_150_3: i64 = (s_150_2 as i64);
        // C s_150_4: cast zx s_150_3 -> i
        let s_150_4: i128 = (i128::try_from(s_150_3).unwrap());
        // C s_150_5: const #432u : u32
        let s_150_5: u32 = 432;
        // D s_150_6: read-reg s_150_5:u8
        let s_150_6: u8 = {
            let value = state.read_register::<u8>(s_150_5 as isize);
            tracer.read_register(s_150_5 as isize, value);
            value
        };
        // D s_150_7: call AArch64_AArch32SystemAccessTrap(s_150_6, s_150_4)
        let s_150_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_150_6,
            s_150_4,
        );
        // N s_150_8: return
        return;
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_151_0: const #3u : u8
        let s_151_0: u8 = 3;
        // C s_151_1: cast zx s_151_0 -> bv
        let s_151_1: Bits = Bits::new(s_151_0 as u128, 8u16);
        // C s_151_2: cast zx s_151_1 -> i
        let s_151_2: i128 = (s_151_1.value() as i128);
        // C s_151_3: cast reint s_151_2 -> i64
        let s_151_3: i64 = (s_151_2 as i64);
        // C s_151_4: cast zx s_151_3 -> i
        let s_151_4: i128 = (i128::try_from(s_151_3).unwrap());
        // S s_151_5: call AArch32_TakeHypTrapException(s_151_4)
        let s_151_5: () = AArch32_TakeHypTrapException(state, tracer, s_151_4);
        // N s_151_6: return
        return;
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_152_0: const #72u : u32
        let s_152_0: u32 = 72;
        // S s_152_1: call ConstrainUnpredictableProcedure(s_152_0)
        let s_152_1: () = ConstrainUnpredictableProcedure(state, tracer, s_152_0);
        // N s_152_2: return
        return;
    }
    fn block_153<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_153_0: const #() : ()
        let s_153_0: () = ();
        // S s_153_1: call AArch32_GetNumEventCountersAccessible(s_153_0)
        let s_153_1: i128 = AArch32_GetNumEventCountersAccessible(
            state,
            tracer,
            s_153_0,
        );
        // C s_153_2: const #21s : i
        let s_153_2: i128 = 21;
        // S s_153_3: cmp-ge s_153_2 s_153_1
        let s_153_3: bool = ((s_153_2) >= (s_153_1));
        // D s_153_4: write-var gs#114092 <= s_153_3
        fn_state.gs_114092 = s_153_3;
        // N s_153_5: jump b133
        return block_133(state, tracer, fn_state);
    }
    fn block_154<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_154_0: const #3u : u8
        let s_154_0: u8 = 3;
        // C s_154_1: cast zx s_154_0 -> bv
        let s_154_1: Bits = Bits::new(s_154_0 as u128, 8u16);
        // C s_154_2: cast zx s_154_1 -> i
        let s_154_2: i128 = (s_154_1.value() as i128);
        // C s_154_3: cast reint s_154_2 -> i64
        let s_154_3: i64 = (s_154_2 as i64);
        // C s_154_4: cast zx s_154_3 -> i
        let s_154_4: i128 = (i128::try_from(s_154_3).unwrap());
        // S s_154_5: call AArch32_TakeHypTrapException(s_154_4)
        let s_154_5: () = AArch32_TakeHypTrapException(state, tracer, s_154_4);
        // N s_154_6: return
        return;
    }
    fn block_155<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_155_0: read-var __HDCR_TPM:u8
        let s_155_0: bool = fn_state.u__HDCR_TPM;
        // D s_155_1: cast zx s_155_0 -> bv
        let s_155_1: Bits = Bits::new(s_155_0 as u128, 1u16);
        // C s_155_2: const #1u : u8
        let s_155_2: bool = true;
        // C s_155_3: cast zx s_155_2 -> bv
        let s_155_3: Bits = Bits::new(s_155_2 as u128, 1u16);
        // D s_155_4: cmp-eq s_155_1 s_155_3
        let s_155_4: bool = ((s_155_1) == (s_155_3));
        // D s_155_5: write-var gs#114090 <= s_155_4
        fn_state.gs_114090 = s_155_4;
        // N s_155_6: jump b130
        return block_130(state, tracer, fn_state);
    }
    fn block_156<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_156_0: const #432u : u32
        let s_156_0: u32 = 432;
        // D s_156_1: read-reg s_156_0:u8
        let s_156_1: u8 = {
            let value = state.read_register::<u8>(s_156_0 as isize);
            tracer.read_register(s_156_0 as isize, value);
            value
        };
        // D s_156_2: call ELUsingAArch32(s_156_1)
        let s_156_2: bool = ELUsingAArch32(state, tracer, s_156_1);
        // D s_156_3: write-var gs#114089 <= s_156_2
        fn_state.gs_114089 = s_156_2;
        // N s_156_4: jump b128
        return block_128(state, tracer, fn_state);
    }
    fn block_157<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_157_0: const #3u : u8
        let s_157_0: u8 = 3;
        // C s_157_1: cast zx s_157_0 -> bv
        let s_157_1: Bits = Bits::new(s_157_0 as u128, 8u16);
        // C s_157_2: cast zx s_157_1 -> i
        let s_157_2: i128 = (s_157_1.value() as i128);
        // C s_157_3: cast reint s_157_2 -> i64
        let s_157_3: i64 = (s_157_2 as i64);
        // C s_157_4: cast zx s_157_3 -> i
        let s_157_4: i128 = (i128::try_from(s_157_3).unwrap());
        // C s_157_5: const #432u : u32
        let s_157_5: u32 = 432;
        // D s_157_6: read-reg s_157_5:u8
        let s_157_6: u8 = {
            let value = state.read_register::<u8>(s_157_5 as isize);
            tracer.read_register(s_157_5 as isize, value);
            value
        };
        // D s_157_7: call AArch64_AArch32SystemAccessTrap(s_157_6, s_157_4)
        let s_157_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_157_6,
            s_157_4,
        );
        // N s_157_8: return
        return;
    }
    fn block_158<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_158_0: read-var __MDCR_EL2_TPM:u8
        let s_158_0: bool = fn_state.u__MDCR_EL2_TPM;
        // D s_158_1: cast zx s_158_0 -> bv
        let s_158_1: Bits = Bits::new(s_158_0 as u128, 1u16);
        // C s_158_2: const #1u : u8
        let s_158_2: bool = true;
        // C s_158_3: cast zx s_158_2 -> bv
        let s_158_3: Bits = Bits::new(s_158_2 as u128, 1u16);
        // D s_158_4: cmp-eq s_158_1 s_158_3
        let s_158_4: bool = ((s_158_1) == (s_158_3));
        // D s_158_5: write-var gs#114088 <= s_158_4
        fn_state.gs_114088 = s_158_4;
        // N s_158_6: jump b125
        return block_125(state, tracer, fn_state);
    }
    fn block_159<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_159_0: const #432u : u32
        let s_159_0: u32 = 432;
        // D s_159_1: read-reg s_159_0:u8
        let s_159_1: u8 = {
            let value = state.read_register::<u8>(s_159_0 as isize);
            tracer.read_register(s_159_0 as isize, value);
            value
        };
        // D s_159_2: call ELUsingAArch32(s_159_1)
        let s_159_2: bool = ELUsingAArch32(state, tracer, s_159_1);
        // D s_159_3: not s_159_2
        let s_159_3: bool = !s_159_2;
        // D s_159_4: write-var gs#114087 <= s_159_3
        fn_state.gs_114087 = s_159_3;
        // N s_159_5: jump b123
        return block_123(state, tracer, fn_state);
    }
    fn block_160<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_160_0: const #3u : u8
        let s_160_0: u8 = 3;
        // C s_160_1: cast zx s_160_0 -> bv
        let s_160_1: Bits = Bits::new(s_160_0 as u128, 8u16);
        // C s_160_2: cast zx s_160_1 -> i
        let s_160_2: i128 = (s_160_1.value() as i128);
        // C s_160_3: cast reint s_160_2 -> i64
        let s_160_3: i64 = (s_160_2 as i64);
        // C s_160_4: cast zx s_160_3 -> i
        let s_160_4: i128 = (i128::try_from(s_160_3).unwrap());
        // C s_160_5: const #432u : u32
        let s_160_5: u32 = 432;
        // D s_160_6: read-reg s_160_5:u8
        let s_160_6: u8 = {
            let value = state.read_register::<u8>(s_160_5 as isize);
            tracer.read_register(s_160_5 as isize, value);
            value
        };
        // D s_160_7: call AArch64_AArch32SystemAccessTrap(s_160_6, s_160_4)
        let s_160_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_160_6,
            s_160_4,
        );
        // N s_160_8: return
        return;
    }
    fn block_161<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_161_0: read-var __HDFGRTR_EL2_PMEVCNTRn_EL0:u8
        let s_161_0: bool = fn_state.u__HDFGRTR_EL2_PMEVCNTRn_EL0;
        // D s_161_1: cast zx s_161_0 -> bv
        let s_161_1: Bits = Bits::new(s_161_0 as u128, 1u16);
        // C s_161_2: const #1u : u8
        let s_161_2: bool = true;
        // C s_161_3: cast zx s_161_2 -> bv
        let s_161_3: Bits = Bits::new(s_161_2 as u128, 1u16);
        // D s_161_4: cmp-eq s_161_1 s_161_3
        let s_161_4: bool = ((s_161_1) == (s_161_3));
        // D s_161_5: write-var gs#114086 <= s_161_4
        fn_state.gs_114086 = s_161_4;
        // N s_161_6: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_162<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_162_0: const #424u : u32
        let s_162_0: u32 = 424;
        // D s_162_1: read-reg s_162_0:u8
        let s_162_1: u8 = {
            let value = state.read_register::<u8>(s_162_0 as isize);
            tracer.read_register(s_162_0 as isize, value);
            value
        };
        // C s_162_2: const #2u : u8
        let s_162_2: u8 = 2;
        // D s_162_3: cmp-lt s_162_1 s_162_2
        let s_162_3: bool = ((s_162_1) < (s_162_2));
        // D s_162_4: not s_162_3
        let s_162_4: bool = !s_162_3;
        // N s_162_5: branch s_162_4 b165 b163
        if s_162_4 {
            return block_165(state, tracer, fn_state);
        } else {
            return block_163(state, tracer, fn_state);
        };
    }
    fn block_163<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_163_0: read-var __SCR_EL3_FGTEn:u8
        let s_163_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_163_1: cast zx s_163_0 -> bv
        let s_163_1: Bits = Bits::new(s_163_0 as u128, 1u16);
        // C s_163_2: const #1u : u8
        let s_163_2: bool = true;
        // C s_163_3: cast zx s_163_2 -> bv
        let s_163_3: Bits = Bits::new(s_163_2 as u128, 1u16);
        // D s_163_4: cmp-eq s_163_1 s_163_3
        let s_163_4: bool = ((s_163_1) == (s_163_3));
        // D s_163_5: write-var gs#114084 <= s_163_4
        fn_state.gs_114084 = s_163_4;
        // N s_163_6: jump b164
        return block_164(state, tracer, fn_state);
    }
    fn block_164<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_164_0: read-var gs#114084:u8
        let s_164_0: bool = fn_state.gs_114084;
        // D s_164_1: write-var gs#114085 <= s_164_0
        fn_state.gs_114085 = s_164_0;
        // N s_164_2: jump b118
        return block_118(state, tracer, fn_state);
    }
    fn block_165<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_165_0: const #1u : u8
        let s_165_0: bool = true;
        // D s_165_1: write-var gs#114084 <= s_165_0
        fn_state.gs_114084 = s_165_0;
        // N s_165_2: jump b164
        return block_164(state, tracer, fn_state);
    }
    fn block_166<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_166_0: const #146u : u32
        let s_166_0: u32 = 146;
        // S s_166_1: call IsFeatureImplemented(s_166_0)
        let s_166_1: bool = IsFeatureImplemented(state, tracer, s_166_0);
        // D s_166_2: write-var gs#114083 <= s_166_1
        fn_state.gs_114083 = s_166_1;
        // N s_166_3: jump b116
        return block_116(state, tracer, fn_state);
    }
    fn block_167<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_167_0: const #102552u : u32
        let s_167_0: u32 = 102552;
        // D s_167_1: read-reg s_167_0:struct
        let s_167_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_167_0 as isize);
            tracer.read_register(s_167_0 as isize, value);
            value
        };
        // D s_167_2: call _get_HCR_EL2_Type_E2H(s_167_1)
        let s_167_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_167_1);
        // C s_167_3: const #102552u : u32
        let s_167_3: u32 = 102552;
        // D s_167_4: read-reg s_167_3:struct
        let s_167_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_167_3 as isize);
            tracer.read_register(s_167_3 as isize, value);
            value
        };
        // D s_167_5: call _get_HCR_EL2_Type_TGE(s_167_4)
        let s_167_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_167_4);
        // D s_167_6: cast zx s_167_2 -> bv
        let s_167_6: Bits = Bits::new(s_167_2 as u128, 1u16);
        // D s_167_7: cast zx s_167_5 -> bv
        let s_167_7: Bits = Bits::new(s_167_5 as u128, 1u16);
        // D s_167_8: cast reint s_167_6 -> u128
        let s_167_8: u128 = (s_167_6.value() as u128);
        // D s_167_9: size-of s_167_6
        let s_167_9: u16 = s_167_6.length();
        // D s_167_10: cast reint s_167_7 -> u128
        let s_167_10: u128 = (s_167_7.value() as u128);
        // D s_167_11: size-of s_167_7
        let s_167_11: u16 = s_167_7.length();
        // D s_167_12: lsl s_167_8 s_167_11
        let s_167_12: u128 = s_167_8 << s_167_11;
        // D s_167_13: or s_167_12 s_167_10
        let s_167_13: u128 = ((s_167_12) | (s_167_10));
        // D s_167_14: add s_167_9 s_167_11
        let s_167_14: u16 = (s_167_9 + s_167_11);
        // D s_167_15: create-bits s_167_13 s_167_14
        let s_167_15: Bits = Bits::new(s_167_13, s_167_14);
        // D s_167_16: cast reint s_167_15 -> u8
        let s_167_16: u8 = (s_167_15.value() as u8);
        // D s_167_17: cast zx s_167_16 -> bv
        let s_167_17: Bits = Bits::new(s_167_16 as u128, 2u16);
        // C s_167_18: const #3u : u8
        let s_167_18: u8 = 3;
        // C s_167_19: cast zx s_167_18 -> bv
        let s_167_19: Bits = Bits::new(s_167_18 as u128, 2u16);
        // D s_167_20: cmp-ne s_167_17 s_167_19
        let s_167_20: bool = ((s_167_17) != (s_167_19));
        // D s_167_21: write-var gs#114082 <= s_167_20
        fn_state.gs_114082 = s_167_20;
        // N s_167_22: jump b114
        return block_114(state, tracer, fn_state);
    }
    fn block_168<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_168_0: const #440u : u32
        let s_168_0: u32 = 440;
        // D s_168_1: read-reg s_168_0:u8
        let s_168_1: u8 = {
            let value = state.read_register::<u8>(s_168_0 as isize);
            tracer.read_register(s_168_0 as isize, value);
            value
        };
        // D s_168_2: call ELUsingAArch32(s_168_1)
        let s_168_2: bool = ELUsingAArch32(state, tracer, s_168_1);
        // D s_168_3: not s_168_2
        let s_168_3: bool = !s_168_2;
        // D s_168_4: write-var gs#114081 <= s_168_3
        fn_state.gs_114081 = s_168_3;
        // N s_168_5: jump b112
        return block_112(state, tracer, fn_state);
    }
    fn block_169<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_169_0: const #() : ()
        let s_169_0: () = ();
        // S s_169_1: call EL2Enabled(s_169_0)
        let s_169_1: bool = EL2Enabled(state, tracer, s_169_0);
        // N s_169_2: branch s_169_1 b185 b170
        if s_169_1 {
            return block_185(state, tracer, fn_state);
        } else {
            return block_170(state, tracer, fn_state);
        };
    }
    fn block_170<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_170_0: const #0u : u8
        let s_170_0: bool = false;
        // D s_170_1: write-var gs#114097 <= s_170_0
        fn_state.gs_114097 = s_170_0;
        // N s_170_2: jump b171
        return block_171(state, tracer, fn_state);
    }
    fn block_171<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_171_0: read-var gs#114097:u8
        let s_171_0: bool = fn_state.gs_114097;
        // N s_171_1: branch s_171_0 b184 b172
        if s_171_0 {
            return block_184(state, tracer, fn_state);
        } else {
            return block_172(state, tracer, fn_state);
        };
    }
    fn block_172<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_172_0: const #0u : u8
        let s_172_0: bool = false;
        // D s_172_1: write-var gs#114098 <= s_172_0
        fn_state.gs_114098 = s_172_0;
        // N s_172_2: jump b173
        return block_173(state, tracer, fn_state);
    }
    fn block_173<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_173_0: read-var gs#114098:u8
        let s_173_0: bool = fn_state.gs_114098;
        // N s_173_1: branch s_173_0 b183 b174
        if s_173_0 {
            return block_183(state, tracer, fn_state);
        } else {
            return block_174(state, tracer, fn_state);
        };
    }
    fn block_174<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_174_0: const #() : ()
        let s_174_0: () = ();
        // S s_174_1: call EL2Enabled(s_174_0)
        let s_174_1: bool = EL2Enabled(state, tracer, s_174_0);
        // N s_174_2: branch s_174_1 b182 b175
        if s_174_1 {
            return block_182(state, tracer, fn_state);
        } else {
            return block_175(state, tracer, fn_state);
        };
    }
    fn block_175<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_175_0: const #0u : u8
        let s_175_0: bool = false;
        // D s_175_1: write-var gs#114099 <= s_175_0
        fn_state.gs_114099 = s_175_0;
        // N s_175_2: jump b176
        return block_176(state, tracer, fn_state);
    }
    fn block_176<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_176_0: read-var gs#114099:u8
        let s_176_0: bool = fn_state.gs_114099;
        // N s_176_1: branch s_176_0 b181 b177
        if s_176_0 {
            return block_181(state, tracer, fn_state);
        } else {
            return block_177(state, tracer, fn_state);
        };
    }
    fn block_177<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_177_0: const #0u : u8
        let s_177_0: bool = false;
        // D s_177_1: write-var gs#114100 <= s_177_0
        fn_state.gs_114100 = s_177_0;
        // N s_177_2: jump b178
        return block_178(state, tracer, fn_state);
    }
    fn block_178<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_178_0: read-var gs#114100:u8
        let s_178_0: bool = fn_state.gs_114100;
        // N s_178_1: branch s_178_0 b180 b179
        if s_178_0 {
            return block_180(state, tracer, fn_state);
        } else {
            return block_179(state, tracer, fn_state);
        };
    }
    fn block_179<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_179_0: panic
        panic!("{:?}", ());
        // N s_179_1: return
        return;
    }
    fn block_180<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_180_0: const #0u : u8
        let s_180_0: u8 = 0;
        // C s_180_1: cast zx s_180_0 -> bv
        let s_180_1: Bits = Bits::new(s_180_0 as u128, 8u16);
        // C s_180_2: cast zx s_180_1 -> i
        let s_180_2: i128 = (s_180_1.value() as i128);
        // C s_180_3: cast reint s_180_2 -> i64
        let s_180_3: i64 = (s_180_2 as i64);
        // C s_180_4: cast zx s_180_3 -> i
        let s_180_4: i128 = (i128::try_from(s_180_3).unwrap());
        // S s_180_5: call AArch32_TakeHypTrapException(s_180_4)
        let s_180_5: () = AArch32_TakeHypTrapException(state, tracer, s_180_4);
        // N s_180_6: return
        return;
    }
    fn block_181<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_181_0: read-var __HCR_TGE:u8
        let s_181_0: bool = fn_state.u__HCR_TGE;
        // D s_181_1: cast zx s_181_0 -> bv
        let s_181_1: Bits = Bits::new(s_181_0 as u128, 1u16);
        // C s_181_2: const #1u : u8
        let s_181_2: bool = true;
        // C s_181_3: cast zx s_181_2 -> bv
        let s_181_3: Bits = Bits::new(s_181_2 as u128, 1u16);
        // D s_181_4: cmp-eq s_181_1 s_181_3
        let s_181_4: bool = ((s_181_1) == (s_181_3));
        // D s_181_5: write-var gs#114100 <= s_181_4
        fn_state.gs_114100 = s_181_4;
        // N s_181_6: jump b178
        return block_178(state, tracer, fn_state);
    }
    fn block_182<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_182_0: const #432u : u32
        let s_182_0: u32 = 432;
        // D s_182_1: read-reg s_182_0:u8
        let s_182_1: u8 = {
            let value = state.read_register::<u8>(s_182_0 as isize);
            tracer.read_register(s_182_0 as isize, value);
            value
        };
        // D s_182_2: call ELUsingAArch32(s_182_1)
        let s_182_2: bool = ELUsingAArch32(state, tracer, s_182_1);
        // D s_182_3: write-var gs#114099 <= s_182_2
        fn_state.gs_114099 = s_182_2;
        // N s_182_4: jump b176
        return block_176(state, tracer, fn_state);
    }
    fn block_183<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_183_0: const #3u : u8
        let s_183_0: u8 = 3;
        // C s_183_1: cast zx s_183_0 -> bv
        let s_183_1: Bits = Bits::new(s_183_0 as u128, 8u16);
        // C s_183_2: cast zx s_183_1 -> i
        let s_183_2: i128 = (s_183_1.value() as i128);
        // C s_183_3: cast reint s_183_2 -> i64
        let s_183_3: i64 = (s_183_2 as i64);
        // C s_183_4: cast zx s_183_3 -> i
        let s_183_4: i128 = (i128::try_from(s_183_3).unwrap());
        // C s_183_5: const #432u : u32
        let s_183_5: u32 = 432;
        // D s_183_6: read-reg s_183_5:u8
        let s_183_6: u8 = {
            let value = state.read_register::<u8>(s_183_5 as isize);
            tracer.read_register(s_183_5 as isize, value);
            value
        };
        // D s_183_7: call AArch64_AArch32SystemAccessTrap(s_183_6, s_183_4)
        let s_183_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_183_6,
            s_183_4,
        );
        // N s_183_8: return
        return;
    }
    fn block_184<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_184_0: read-var __HCR_EL2_TGE:u8
        let s_184_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_184_1: cast zx s_184_0 -> bv
        let s_184_1: Bits = Bits::new(s_184_0 as u128, 1u16);
        // C s_184_2: const #1u : u8
        let s_184_2: bool = true;
        // C s_184_3: cast zx s_184_2 -> bv
        let s_184_3: Bits = Bits::new(s_184_2 as u128, 1u16);
        // D s_184_4: cmp-eq s_184_1 s_184_3
        let s_184_4: bool = ((s_184_1) == (s_184_3));
        // D s_184_5: write-var gs#114098 <= s_184_4
        fn_state.gs_114098 = s_184_4;
        // N s_184_6: jump b173
        return block_173(state, tracer, fn_state);
    }
    fn block_185<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_185_0: const #432u : u32
        let s_185_0: u32 = 432;
        // D s_185_1: read-reg s_185_0:u8
        let s_185_1: u8 = {
            let value = state.read_register::<u8>(s_185_0 as isize);
            tracer.read_register(s_185_0 as isize, value);
            value
        };
        // D s_185_2: call ELUsingAArch32(s_185_1)
        let s_185_2: bool = ELUsingAArch32(state, tracer, s_185_1);
        // D s_185_3: not s_185_2
        let s_185_3: bool = !s_185_2;
        // D s_185_4: write-var gs#114097 <= s_185_3
        fn_state.gs_114097 = s_185_3;
        // N s_185_5: jump b171
        return block_171(state, tracer, fn_state);
    }
    fn block_186<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_186_0: const #() : ()
        let s_186_0: () = ();
        // S s_186_1: call PMUSERENR_read(s_186_0)
        let s_186_1: ProductType700c18a878c5601b = PMUSERENR_read(
            state,
            tracer,
            s_186_0,
        );
        // S s_186_2: call _get_PMUSERENR_Type_ER(s_186_1)
        let s_186_2: bool = u_get_PMUSERENR_Type_ER(state, tracer, s_186_1);
        // C s_186_3: const #() : ()
        let s_186_3: () = ();
        // S s_186_4: call PMUSERENR_read(s_186_3)
        let s_186_4: ProductType700c18a878c5601b = PMUSERENR_read(
            state,
            tracer,
            s_186_3,
        );
        // S s_186_5: call _get_PMUSERENR_Type_EN(s_186_4)
        let s_186_5: bool = u_get_PMUSERENR_Type_EN(state, tracer, s_186_4);
        // S s_186_6: cast zx s_186_2 -> bv
        let s_186_6: Bits = Bits::new(s_186_2 as u128, 1u16);
        // S s_186_7: cast zx s_186_5 -> bv
        let s_186_7: Bits = Bits::new(s_186_5 as u128, 1u16);
        // S s_186_8: cast reint s_186_6 -> u128
        let s_186_8: u128 = (s_186_6.value() as u128);
        // D s_186_9: size-of s_186_6
        let s_186_9: u16 = s_186_6.length();
        // S s_186_10: cast reint s_186_7 -> u128
        let s_186_10: u128 = (s_186_7.value() as u128);
        // D s_186_11: size-of s_186_7
        let s_186_11: u16 = s_186_7.length();
        // D s_186_12: lsl s_186_8 s_186_11
        let s_186_12: u128 = s_186_8 << s_186_11;
        // D s_186_13: or s_186_12 s_186_10
        let s_186_13: u128 = ((s_186_12) | (s_186_10));
        // D s_186_14: add s_186_9 s_186_11
        let s_186_14: u16 = (s_186_9 + s_186_11);
        // D s_186_15: create-bits s_186_13 s_186_14
        let s_186_15: Bits = Bits::new(s_186_13, s_186_14);
        // D s_186_16: cast reint s_186_15 -> u8
        let s_186_16: u8 = (s_186_15.value() as u8);
        // D s_186_17: cast zx s_186_16 -> bv
        let s_186_17: Bits = Bits::new(s_186_16 as u128, 2u16);
        // C s_186_18: const #0u : u8
        let s_186_18: u8 = 0;
        // C s_186_19: cast zx s_186_18 -> bv
        let s_186_19: Bits = Bits::new(s_186_18 as u128, 2u16);
        // D s_186_20: cmp-eq s_186_17 s_186_19
        let s_186_20: bool = ((s_186_17) == (s_186_19));
        // D s_186_21: write-var gs#114080 <= s_186_20
        fn_state.gs_114080 = s_186_20;
        // N s_186_22: jump b109
        return block_109(state, tracer, fn_state);
    }
    fn block_187<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_187_0: const #() : ()
        let s_187_0: () = ();
        // S s_187_1: call EL2Enabled(s_187_0)
        let s_187_1: bool = EL2Enabled(state, tracer, s_187_0);
        // N s_187_2: branch s_187_1 b195 b188
        if s_187_1 {
            return block_195(state, tracer, fn_state);
        } else {
            return block_188(state, tracer, fn_state);
        };
    }
    fn block_188<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_188_0: const #0u : u8
        let s_188_0: bool = false;
        // D s_188_1: write-var gs#114101 <= s_188_0
        fn_state.gs_114101 = s_188_0;
        // N s_188_2: jump b189
        return block_189(state, tracer, fn_state);
    }
    fn block_189<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_189_0: read-var gs#114101:u8
        let s_189_0: bool = fn_state.gs_114101;
        // N s_189_1: branch s_189_0 b194 b190
        if s_189_0 {
            return block_194(state, tracer, fn_state);
        } else {
            return block_190(state, tracer, fn_state);
        };
    }
    fn block_190<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_190_0: const #0u : u8
        let s_190_0: bool = false;
        // D s_190_1: write-var gs#114102 <= s_190_0
        fn_state.gs_114102 = s_190_0;
        // N s_190_2: jump b191
        return block_191(state, tracer, fn_state);
    }
    fn block_191<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_191_0: read-var gs#114102:u8
        let s_191_0: bool = fn_state.gs_114102;
        // N s_191_1: branch s_191_0 b193 b192
        if s_191_0 {
            return block_193(state, tracer, fn_state);
        } else {
            return block_192(state, tracer, fn_state);
        };
    }
    fn block_192<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_192_0: const #3u : u8
        let s_192_0: u8 = 3;
        // C s_192_1: cast zx s_192_0 -> bv
        let s_192_1: Bits = Bits::new(s_192_0 as u128, 8u16);
        // C s_192_2: cast zx s_192_1 -> i
        let s_192_2: i128 = (s_192_1.value() as i128);
        // C s_192_3: cast reint s_192_2 -> i64
        let s_192_3: i64 = (s_192_2 as i64);
        // C s_192_4: cast zx s_192_3 -> i
        let s_192_4: i128 = (i128::try_from(s_192_3).unwrap());
        // C s_192_5: const #440u : u32
        let s_192_5: u32 = 440;
        // D s_192_6: read-reg s_192_5:u8
        let s_192_6: u8 = {
            let value = state.read_register::<u8>(s_192_5 as isize);
            tracer.read_register(s_192_5 as isize, value);
            value
        };
        // D s_192_7: call AArch64_AArch32SystemAccessTrap(s_192_6, s_192_4)
        let s_192_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_192_6,
            s_192_4,
        );
        // N s_192_8: return
        return;
    }
    fn block_193<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_193_0: const #3u : u8
        let s_193_0: u8 = 3;
        // C s_193_1: cast zx s_193_0 -> bv
        let s_193_1: Bits = Bits::new(s_193_0 as u128, 8u16);
        // C s_193_2: cast zx s_193_1 -> i
        let s_193_2: i128 = (s_193_1.value() as i128);
        // C s_193_3: cast reint s_193_2 -> i64
        let s_193_3: i64 = (s_193_2 as i64);
        // C s_193_4: cast zx s_193_3 -> i
        let s_193_4: i128 = (i128::try_from(s_193_3).unwrap());
        // C s_193_5: const #432u : u32
        let s_193_5: u32 = 432;
        // D s_193_6: read-reg s_193_5:u8
        let s_193_6: u8 = {
            let value = state.read_register::<u8>(s_193_5 as isize);
            tracer.read_register(s_193_5 as isize, value);
            value
        };
        // D s_193_7: call AArch64_AArch32SystemAccessTrap(s_193_6, s_193_4)
        let s_193_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_193_6,
            s_193_4,
        );
        // N s_193_8: return
        return;
    }
    fn block_194<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_194_0: read-var __HCR_EL2_TGE:u8
        let s_194_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_194_1: cast zx s_194_0 -> bv
        let s_194_1: Bits = Bits::new(s_194_0 as u128, 1u16);
        // C s_194_2: const #1u : u8
        let s_194_2: bool = true;
        // C s_194_3: cast zx s_194_2 -> bv
        let s_194_3: Bits = Bits::new(s_194_2 as u128, 1u16);
        // D s_194_4: cmp-eq s_194_1 s_194_3
        let s_194_4: bool = ((s_194_1) == (s_194_3));
        // D s_194_5: write-var gs#114102 <= s_194_4
        fn_state.gs_114102 = s_194_4;
        // N s_194_6: jump b191
        return block_191(state, tracer, fn_state);
    }
    fn block_195<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_195_0: const #432u : u32
        let s_195_0: u32 = 432;
        // D s_195_1: read-reg s_195_0:u8
        let s_195_1: u8 = {
            let value = state.read_register::<u8>(s_195_0 as isize);
            tracer.read_register(s_195_0 as isize, value);
            value
        };
        // D s_195_2: call ELUsingAArch32(s_195_1)
        let s_195_2: bool = ELUsingAArch32(state, tracer, s_195_1);
        // D s_195_3: not s_195_2
        let s_195_3: bool = !s_195_2;
        // D s_195_4: write-var gs#114101 <= s_195_3
        fn_state.gs_114101 = s_195_3;
        // N s_195_5: jump b189
        return block_189(state, tracer, fn_state);
    }
    fn block_196<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_196_0: const #204u : u32
        let s_196_0: u32 = 204;
        // S s_196_1: call IsFeatureImplemented(s_196_0)
        let s_196_1: bool = IsFeatureImplemented(state, tracer, s_196_0);
        // N s_196_2: branch s_196_1 b205 b197
        if s_196_1 {
            return block_205(state, tracer, fn_state);
        } else {
            return block_197(state, tracer, fn_state);
        };
    }
    fn block_197<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_197_0: const #0u : u8
        let s_197_0: bool = false;
        // D s_197_1: write-var gs#114076 <= s_197_0
        fn_state.gs_114076 = s_197_0;
        // N s_197_2: jump b198
        return block_198(state, tracer, fn_state);
    }
    fn block_198<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_198_0: read-var gs#114076:u8
        let s_198_0: bool = fn_state.gs_114076;
        // N s_198_1: branch s_198_0 b204 b199
        if s_198_0 {
            return block_204(state, tracer, fn_state);
        } else {
            return block_199(state, tracer, fn_state);
        };
    }
    fn block_199<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_199_0: const #204u : u32
        let s_199_0: u32 = 204;
        // S s_199_1: call IsFeatureImplemented(s_199_0)
        let s_199_1: bool = IsFeatureImplemented(state, tracer, s_199_0);
        // S s_199_2: not s_199_1
        let s_199_2: bool = !s_199_1;
        // N s_199_3: branch s_199_2 b203 b200
        if s_199_2 {
            return block_203(state, tracer, fn_state);
        } else {
            return block_200(state, tracer, fn_state);
        };
    }
    fn block_200<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_200_0: const #0u : u8
        let s_200_0: bool = false;
        // D s_200_1: write-var gs#114077 <= s_200_0
        fn_state.gs_114077 = s_200_0;
        // N s_200_2: jump b201
        return block_201(state, tracer, fn_state);
    }
    fn block_201<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_201_0: read-var gs#114077:u8
        let s_201_0: bool = fn_state.gs_114077;
        // D s_201_1: write-var gs#114078 <= s_201_0
        fn_state.gs_114078 = s_201_0;
        // N s_201_2: jump b202
        return block_202(state, tracer, fn_state);
    }
    fn block_202<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_202_0: read-var gs#114078:u8
        let s_202_0: bool = fn_state.gs_114078;
        // D s_202_1: write-var gs#114079 <= s_202_0
        fn_state.gs_114079 = s_202_0;
        // N s_202_2: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_203<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_203_0: const #102624u : u32
        let s_203_0: u32 = 102624;
        // D s_203_1: read-reg s_203_0:struct
        let s_203_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_203_0 as isize);
            tracer.read_register(s_203_0 as isize, value);
            value
        };
        // D s_203_2: call _get_PMUSERENR_EL0_Type_ER(s_203_1)
        let s_203_2: bool = u_get_PMUSERENR_EL0_Type_ER(state, tracer, s_203_1);
        // C s_203_3: const #102624u : u32
        let s_203_3: u32 = 102624;
        // D s_203_4: read-reg s_203_3:struct
        let s_203_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_203_3 as isize);
            tracer.read_register(s_203_3 as isize, value);
            value
        };
        // D s_203_5: call _get_PMUSERENR_EL0_Type_EN(s_203_4)
        let s_203_5: bool = u_get_PMUSERENR_EL0_Type_EN(state, tracer, s_203_4);
        // D s_203_6: cast zx s_203_2 -> bv
        let s_203_6: Bits = Bits::new(s_203_2 as u128, 1u16);
        // D s_203_7: cast zx s_203_5 -> bv
        let s_203_7: Bits = Bits::new(s_203_5 as u128, 1u16);
        // D s_203_8: cast reint s_203_6 -> u128
        let s_203_8: u128 = (s_203_6.value() as u128);
        // D s_203_9: size-of s_203_6
        let s_203_9: u16 = s_203_6.length();
        // D s_203_10: cast reint s_203_7 -> u128
        let s_203_10: u128 = (s_203_7.value() as u128);
        // D s_203_11: size-of s_203_7
        let s_203_11: u16 = s_203_7.length();
        // D s_203_12: lsl s_203_8 s_203_11
        let s_203_12: u128 = s_203_8 << s_203_11;
        // D s_203_13: or s_203_12 s_203_10
        let s_203_13: u128 = ((s_203_12) | (s_203_10));
        // D s_203_14: add s_203_9 s_203_11
        let s_203_14: u16 = (s_203_9 + s_203_11);
        // D s_203_15: create-bits s_203_13 s_203_14
        let s_203_15: Bits = Bits::new(s_203_13, s_203_14);
        // D s_203_16: cast reint s_203_15 -> u8
        let s_203_16: u8 = (s_203_15.value() as u8);
        // D s_203_17: cast zx s_203_16 -> bv
        let s_203_17: Bits = Bits::new(s_203_16 as u128, 2u16);
        // C s_203_18: const #0u : u8
        let s_203_18: u8 = 0;
        // C s_203_19: cast zx s_203_18 -> bv
        let s_203_19: Bits = Bits::new(s_203_18 as u128, 2u16);
        // D s_203_20: cmp-eq s_203_17 s_203_19
        let s_203_20: bool = ((s_203_17) == (s_203_19));
        // D s_203_21: write-var gs#114077 <= s_203_20
        fn_state.gs_114077 = s_203_20;
        // N s_203_22: jump b201
        return block_201(state, tracer, fn_state);
    }
    fn block_204<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_204_0: const #1u : u8
        let s_204_0: bool = true;
        // D s_204_1: write-var gs#114078 <= s_204_0
        fn_state.gs_114078 = s_204_0;
        // N s_204_2: jump b202
        return block_202(state, tracer, fn_state);
    }
    fn block_205<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_205_0: const #102624u : u32
        let s_205_0: u32 = 102624;
        // D s_205_1: read-reg s_205_0:struct
        let s_205_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_205_0 as isize);
            tracer.read_register(s_205_0 as isize, value);
            value
        };
        // D s_205_2: call _get_PMUSERENR_EL0_Type_UEN(s_205_1)
        let s_205_2: bool = u_get_PMUSERENR_EL0_Type_UEN(state, tracer, s_205_1);
        // C s_205_3: const #102624u : u32
        let s_205_3: u32 = 102624;
        // D s_205_4: read-reg s_205_3:struct
        let s_205_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_205_3 as isize);
            tracer.read_register(s_205_3 as isize, value);
            value
        };
        // D s_205_5: call _get_PMUSERENR_EL0_Type_ER(s_205_4)
        let s_205_5: bool = u_get_PMUSERENR_EL0_Type_ER(state, tracer, s_205_4);
        // C s_205_6: const #102624u : u32
        let s_205_6: u32 = 102624;
        // D s_205_7: read-reg s_205_6:struct
        let s_205_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_205_6 as isize);
            tracer.read_register(s_205_6 as isize, value);
            value
        };
        // D s_205_8: call _get_PMUSERENR_EL0_Type_EN(s_205_7)
        let s_205_8: bool = u_get_PMUSERENR_EL0_Type_EN(state, tracer, s_205_7);
        // D s_205_9: cast zx s_205_5 -> bv
        let s_205_9: Bits = Bits::new(s_205_5 as u128, 1u16);
        // D s_205_10: cast zx s_205_8 -> bv
        let s_205_10: Bits = Bits::new(s_205_8 as u128, 1u16);
        // D s_205_11: cast reint s_205_9 -> u128
        let s_205_11: u128 = (s_205_9.value() as u128);
        // D s_205_12: size-of s_205_9
        let s_205_12: u16 = s_205_9.length();
        // D s_205_13: cast reint s_205_10 -> u128
        let s_205_13: u128 = (s_205_10.value() as u128);
        // D s_205_14: size-of s_205_10
        let s_205_14: u16 = s_205_10.length();
        // D s_205_15: lsl s_205_11 s_205_14
        let s_205_15: u128 = s_205_11 << s_205_14;
        // D s_205_16: or s_205_15 s_205_13
        let s_205_16: u128 = ((s_205_15) | (s_205_13));
        // D s_205_17: add s_205_12 s_205_14
        let s_205_17: u16 = (s_205_12 + s_205_14);
        // D s_205_18: create-bits s_205_16 s_205_17
        let s_205_18: Bits = Bits::new(s_205_16, s_205_17);
        // D s_205_19: cast reint s_205_18 -> u8
        let s_205_19: u8 = (s_205_18.value() as u8);
        // D s_205_20: cast zx s_205_2 -> bv
        let s_205_20: Bits = Bits::new(s_205_2 as u128, 1u16);
        // D s_205_21: cast zx s_205_19 -> bv
        let s_205_21: Bits = Bits::new(s_205_19 as u128, 2u16);
        // D s_205_22: cast reint s_205_20 -> u128
        let s_205_22: u128 = (s_205_20.value() as u128);
        // D s_205_23: size-of s_205_20
        let s_205_23: u16 = s_205_20.length();
        // D s_205_24: cast reint s_205_21 -> u128
        let s_205_24: u128 = (s_205_21.value() as u128);
        // D s_205_25: size-of s_205_21
        let s_205_25: u16 = s_205_21.length();
        // D s_205_26: lsl s_205_22 s_205_25
        let s_205_26: u128 = s_205_22 << s_205_25;
        // D s_205_27: or s_205_26 s_205_24
        let s_205_27: u128 = ((s_205_26) | (s_205_24));
        // D s_205_28: add s_205_23 s_205_25
        let s_205_28: u16 = (s_205_23 + s_205_25);
        // D s_205_29: create-bits s_205_27 s_205_28
        let s_205_29: Bits = Bits::new(s_205_27, s_205_28);
        // D s_205_30: cast reint s_205_29 -> u8
        let s_205_30: u8 = (s_205_29.value() as u8);
        // D s_205_31: cast zx s_205_30 -> bv
        let s_205_31: Bits = Bits::new(s_205_30 as u128, 3u16);
        // C s_205_32: const #0u : u8
        let s_205_32: u8 = 0;
        // C s_205_33: cast zx s_205_32 -> bv
        let s_205_33: Bits = Bits::new(s_205_32 as u128, 3u16);
        // D s_205_34: cmp-eq s_205_31 s_205_33
        let s_205_34: bool = ((s_205_31) == (s_205_33));
        // D s_205_35: write-var gs#114076 <= s_205_34
        fn_state.gs_114076 = s_205_34;
        // N s_205_36: jump b198
        return block_198(state, tracer, fn_state);
    }
    fn block_206<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_206_0: panic
        panic!("{:?}", ());
        // N s_206_1: return
        return;
    }
    fn block_207<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_207_0: read-var __MDCR_EL3_TPM:u8
        let s_207_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_207_1: cast zx s_207_0 -> bv
        let s_207_1: Bits = Bits::new(s_207_0 as u128, 1u16);
        // C s_207_2: const #1u : u8
        let s_207_2: bool = true;
        // C s_207_3: cast zx s_207_2 -> bv
        let s_207_3: Bits = Bits::new(s_207_2 as u128, 1u16);
        // D s_207_4: cmp-eq s_207_1 s_207_3
        let s_207_4: bool = ((s_207_1) == (s_207_3));
        // D s_207_5: write-var gs#114075 <= s_207_4
        fn_state.gs_114075 = s_207_4;
        // N s_207_6: jump b103
        return block_103(state, tracer, fn_state);
    }
    fn block_208<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_208_0: const #424u : u32
        let s_208_0: u32 = 424;
        // D s_208_1: read-reg s_208_0:u8
        let s_208_1: u8 = {
            let value = state.read_register::<u8>(s_208_0 as isize);
            tracer.read_register(s_208_0 as isize, value);
            value
        };
        // D s_208_2: call ELUsingAArch32(s_208_1)
        let s_208_2: bool = ELUsingAArch32(state, tracer, s_208_1);
        // D s_208_3: not s_208_2
        let s_208_3: bool = !s_208_2;
        // D s_208_4: write-var gs#114074 <= s_208_3
        fn_state.gs_114074 = s_208_3;
        // N s_208_5: jump b101
        return block_101(state, tracer, fn_state);
    }
    fn block_209<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_209_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_209_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_209_1: call __IMPDEF_boolean(s_209_0)
        let s_209_1: bool = u__IMPDEF_boolean(state, tracer, s_209_0);
        // D s_209_2: write-var gs#114073 <= s_209_1
        fn_state.gs_114073 = s_209_1;
        // N s_209_3: jump b99
        return block_99(state, tracer, fn_state);
    }
    fn block_210<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_210_0: const #() : ()
        let s_210_0: () = ();
        // S s_210_1: call EDSCR_read(s_210_0)
        let s_210_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_210_0);
        // S s_210_2: call _get_EDSCR_Type_SDD(s_210_1)
        let s_210_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_210_1);
        // S s_210_3: cast zx s_210_2 -> bv
        let s_210_3: Bits = Bits::new(s_210_2 as u128, 1u16);
        // C s_210_4: const #1u : u8
        let s_210_4: bool = true;
        // C s_210_5: cast zx s_210_4 -> bv
        let s_210_5: Bits = Bits::new(s_210_4 as u128, 1u16);
        // S s_210_6: cmp-eq s_210_3 s_210_5
        let s_210_6: bool = ((s_210_3) == (s_210_5));
        // D s_210_7: write-var gs#114072 <= s_210_6
        fn_state.gs_114072 = s_210_6;
        // N s_210_8: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_211<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_211_0: const #424u : u32
        let s_211_0: u32 = 424;
        // D s_211_1: read-reg s_211_0:u8
        let s_211_1: u8 = {
            let value = state.read_register::<u8>(s_211_0 as isize);
            tracer.read_register(s_211_0 as isize, value);
            value
        };
        // C s_211_2: const #2u : u8
        let s_211_2: u8 = 2;
        // D s_211_3: cmp-lt s_211_1 s_211_2
        let s_211_3: bool = ((s_211_1) < (s_211_2));
        // D s_211_4: write-var gs#114071 <= s_211_3
        fn_state.gs_114071 = s_211_3;
        // N s_211_5: jump b95
        return block_95(state, tracer, fn_state);
    }
}
