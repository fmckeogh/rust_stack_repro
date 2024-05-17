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
use u_get_HDCR_Type_TPMCR::*;
use Halted::*;
use u_get_HSTR_Type_T9::*;
use u__get_PMCR::*;
use u_get_MDCR_EL2_Type_TPMCR::*;
use u_get_MDCR_EL3_Type_TPM::*;
use u_get_PMUSERENR_Type_EN::*;
use u_get_EDSCR_Type_SDD::*;
use u_get_HSTR_EL2_Type_T9::*;
use u_get_HCR_EL2_Type_TGE::*;
use EL2Enabled::*;
use u_get_MDCR_EL2_Type_TPM::*;
use u_get_PMUSERENR_EL0_Type_UEN::*;
use u_get_HCR_EL2_Type_E2H::*;
use HCR_read::*;
use PMUSERENR_read::*;
use HDCR_read::*;
use IsFeatureImplemented::*;
use u__IMPDEF_boolean::*;
use AArch64_AArch32SystemAccessTrap::*;
use u_get_HDCR_Type_TPM::*;
use HSTR_read::*;
use u_get_HCR_Type_TGE::*;
use R_set::*;
use ELUsingAArch32::*;
use PMCR_read::*;
use u_get_PMUSERENR_EL0_Type_EN::*;
use EDSCR_read::*;
use AArch32_TakeHypTrapException::*;
use common::*;
pub fn PMCR_SysRegRead32_fb694f2a3eabe8b5<T: Tracer>(
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
        gs_113201: bool,
        u__MDCR_EL2_TPMCR: bool,
        gs_113214: bool,
        gs_113177: bool,
        gs_113164: bool,
        gs_113213: bool,
        gs_113196: bool,
        gs_113203: bool,
        gs_113216: bool,
        gs_113195: bool,
        gs_113217: bool,
        gs_113158: bool,
        gs_113162: bool,
        u__MDCR_EL3_TPM: bool,
        gs_113197: bool,
        gs_113183: bool,
        gs_113211: bool,
        u__HCR_TGE: bool,
        gs_113212: bool,
        gs_113172: bool,
        gs_113160: bool,
        gs_113163: bool,
        gs_113178: bool,
        ga_185827: ProductType700c18a878c5601b,
        gs_113198: bool,
        gs_113185: bool,
        gs_113208: bool,
        u__HDCR_TPMCR: bool,
        gs_113206: bool,
        gs_113168: bool,
        gs_113180: bool,
        gs_113188: bool,
        gs_113202: bool,
        u__PSTATE_EL: u8,
        gs_113187: bool,
        gs_113207: bool,
        gs_113184: bool,
        gs_113179: bool,
        gs_113191: bool,
        gs_113171: bool,
        u__MDCR_EL2_TPM: bool,
        u__PMUSERENR_EN: bool,
        gs_113189: bool,
        gs_113170: bool,
        gs_113176: bool,
        gs_113215: bool,
        ga_185752: ProductType700c18a878c5601b,
        gs_113186: bool,
        u__HCR_EL2_TGE: bool,
        gs_113194: bool,
        gs_113174: bool,
        u__PMUSERENR_EL0_EN: bool,
        u__HSTR_T9: bool,
        u__HDCR_TPM: bool,
        gs_113205: bool,
        gs_113193: bool,
        gs_113209: bool,
        gs_113166: bool,
        gs_113190: bool,
        gs_113175: bool,
        gs_113165: bool,
        gs_113200: bool,
        gs_113181: bool,
        gs_113192: bool,
        gs_113161: bool,
        gs_113210: bool,
        gs_113169: bool,
        ga_185803: ProductType700c18a878c5601b,
        gs_113204: bool,
        gs_113173: bool,
        gs_113199: bool,
        gs_113182: bool,
        gs_113167: bool,
        ga_185831: ProductType700c18a878c5601b,
        gs_113159: bool,
        u__HSTR_EL2_T9: bool,
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
        // C s_0_7: const #102624u : u32
        let s_0_7: u32 = 102624;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_PMUSERENR_EL0_Type_EN(s_0_8)
        let s_0_9: bool = u_get_PMUSERENR_EL0_Type_EN(state, tracer, s_0_8);
        // D s_0_10: write-var __PMUSERENR_EL0_EN <= s_0_9
        fn_state.u__PMUSERENR_EL0_EN = s_0_9;
        // C s_0_11: const #102552u : u32
        let s_0_11: u32 = 102552;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_HCR_EL2_Type_TGE(s_0_12)
        let s_0_13: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_0_12);
        // D s_0_14: write-var __HCR_EL2_TGE <= s_0_13
        fn_state.u__HCR_EL2_TGE = s_0_13;
        // C s_0_15: const #() : ()
        let s_0_15: () = ();
        // S s_0_16: call PMUSERENR_read(s_0_15)
        let s_0_16: ProductType700c18a878c5601b = PMUSERENR_read(state, tracer, s_0_15);
        // S s_0_17: call _get_PMUSERENR_Type_EN(s_0_16)
        let s_0_17: bool = u_get_PMUSERENR_Type_EN(state, tracer, s_0_16);
        // D s_0_18: write-var __PMUSERENR_EN <= s_0_17
        fn_state.u__PMUSERENR_EN = s_0_17;
        // C s_0_19: const #() : ()
        let s_0_19: () = ();
        // S s_0_20: call HCR_read(s_0_19)
        let s_0_20: ProductType700c18a878c5601b = HCR_read(state, tracer, s_0_19);
        // S s_0_21: call _get_HCR_Type_TGE(s_0_20)
        let s_0_21: bool = u_get_HCR_Type_TGE(state, tracer, s_0_20);
        // D s_0_22: write-var __HCR_TGE <= s_0_21
        fn_state.u__HCR_TGE = s_0_21;
        // C s_0_23: const #104936u : u32
        let s_0_23: u32 = 104936;
        // D s_0_24: read-reg s_0_23:struct
        let s_0_24: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_23 as isize);
            tracer.read_register(s_0_23 as isize, value);
            value
        };
        // D s_0_25: call _get_HSTR_EL2_Type_T9(s_0_24)
        let s_0_25: bool = u_get_HSTR_EL2_Type_T9(state, tracer, s_0_24);
        // D s_0_26: write-var __HSTR_EL2_T9 <= s_0_25
        fn_state.u__HSTR_EL2_T9 = s_0_25;
        // C s_0_27: const #() : ()
        let s_0_27: () = ();
        // S s_0_28: call HSTR_read(s_0_27)
        let s_0_28: ProductType700c18a878c5601b = HSTR_read(state, tracer, s_0_27);
        // S s_0_29: call _get_HSTR_Type_T9(s_0_28)
        let s_0_29: bool = u_get_HSTR_Type_T9(state, tracer, s_0_28);
        // D s_0_30: write-var __HSTR_T9 <= s_0_29
        fn_state.u__HSTR_T9 = s_0_29;
        // C s_0_31: const #104880u : u32
        let s_0_31: u32 = 104880;
        // D s_0_32: read-reg s_0_31:struct
        let s_0_32: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_31 as isize);
            tracer.read_register(s_0_31 as isize, value);
            value
        };
        // D s_0_33: call _get_MDCR_EL2_Type_TPM(s_0_32)
        let s_0_33: bool = u_get_MDCR_EL2_Type_TPM(state, tracer, s_0_32);
        // D s_0_34: write-var __MDCR_EL2_TPM <= s_0_33
        fn_state.u__MDCR_EL2_TPM = s_0_33;
        // C s_0_35: const #104880u : u32
        let s_0_35: u32 = 104880;
        // D s_0_36: read-reg s_0_35:struct
        let s_0_36: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_35 as isize);
            tracer.read_register(s_0_35 as isize, value);
            value
        };
        // D s_0_37: call _get_MDCR_EL2_Type_TPMCR(s_0_36)
        let s_0_37: bool = u_get_MDCR_EL2_Type_TPMCR(state, tracer, s_0_36);
        // D s_0_38: write-var __MDCR_EL2_TPMCR <= s_0_37
        fn_state.u__MDCR_EL2_TPMCR = s_0_37;
        // C s_0_39: const #() : ()
        let s_0_39: () = ();
        // S s_0_40: call HDCR_read(s_0_39)
        let s_0_40: ProductType700c18a878c5601b = HDCR_read(state, tracer, s_0_39);
        // S s_0_41: call _get_HDCR_Type_TPM(s_0_40)
        let s_0_41: bool = u_get_HDCR_Type_TPM(state, tracer, s_0_40);
        // D s_0_42: write-var __HDCR_TPM <= s_0_41
        fn_state.u__HDCR_TPM = s_0_41;
        // C s_0_43: const #() : ()
        let s_0_43: () = ();
        // S s_0_44: call HDCR_read(s_0_43)
        let s_0_44: ProductType700c18a878c5601b = HDCR_read(state, tracer, s_0_43);
        // S s_0_45: call _get_HDCR_Type_TPMCR(s_0_44)
        let s_0_45: bool = u_get_HDCR_Type_TPMCR(state, tracer, s_0_44);
        // D s_0_46: write-var __HDCR_TPMCR <= s_0_45
        fn_state.u__HDCR_TPMCR = s_0_45;
        // D s_0_47: read-var __PSTATE_EL:u8
        let s_0_47: u8 = fn_state.u__PSTATE_EL;
        // D s_0_48: cast zx s_0_47 -> bv
        let s_0_48: Bits = Bits::new(s_0_47 as u128, 2u16);
        // C s_0_49: const #448u : u32
        let s_0_49: u32 = 448;
        // D s_0_50: read-reg s_0_49:u8
        let s_0_50: u8 = {
            let value = state.read_register::<u8>(s_0_49 as isize);
            tracer.read_register(s_0_49 as isize, value);
            value
        };
        // D s_0_51: cast zx s_0_50 -> bv
        let s_0_51: Bits = Bits::new(s_0_50 as u128, 2u16);
        // D s_0_52: cmp-eq s_0_48 s_0_51
        let s_0_52: bool = ((s_0_48) == (s_0_51));
        // N s_0_53: branch s_0_52 b116 b1
        if s_0_52 {
            return block_116(state, tracer, fn_state);
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
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call PMCR_read(s_5_0)
        let s_5_1: ProductType700c18a878c5601b = PMCR_read(state, tracer, s_5_0);
        // S s_5_2: call __get_PMCR(s_5_1)
        let s_5_2: ProductType700c18a878c5601b = u__get_PMCR(state, tracer, s_5_1);
        // D s_5_3: write-var ga#185831 <= s_5_2
        fn_state.ga_185831 = s_5_2;
        // D s_5_4: read-var ga#185831.0:struct
        let s_5_4: u32 = fn_state.ga_185831._0;
        // D s_5_5: read-var t:i
        let s_5_5: i128 = fn_state.t;
        // D s_5_6: call R_set(s_5_5, s_5_4)
        let s_5_6: () = R_set(state, tracer, s_5_5, s_5_4);
        // N s_5_7: return
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
        // D s_7_1: write-var gs#113158 <= s_7_0
        fn_state.gs_113158 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#113158:u8
        let s_8_0: bool = fn_state.gs_113158;
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
        // D s_9_1: write-var gs#113159 <= s_9_0
        fn_state.gs_113159 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#113159:u8
        let s_10_0: bool = fn_state.gs_113159;
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
        // D s_11_1: write-var gs#113160 <= s_11_0
        fn_state.gs_113160 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#113160:u8
        let s_12_0: bool = fn_state.gs_113160;
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
        // D s_13_1: write-var gs#113161 <= s_13_0
        fn_state.gs_113161 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#113161:u8
        let s_14_0: bool = fn_state.gs_113161;
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
        // D s_15_1: write-var gs#113162 <= s_15_0
        fn_state.gs_113162 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#113162:u8
        let s_16_0: bool = fn_state.gs_113162;
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
        // D s_18_1: write-var gs#113163 <= s_18_0
        fn_state.gs_113163 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#113163:u8
        let s_19_0: bool = fn_state.gs_113163;
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
        // D s_20_1: write-var gs#113164 <= s_20_0
        fn_state.gs_113164 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#113164:u8
        let s_21_0: bool = fn_state.gs_113164;
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
        // C s_22_0: const #() : ()
        let s_22_0: () = ();
        // S s_22_1: call PMCR_read(s_22_0)
        let s_22_1: ProductType700c18a878c5601b = PMCR_read(state, tracer, s_22_0);
        // S s_22_2: call __get_PMCR(s_22_1)
        let s_22_2: ProductType700c18a878c5601b = u__get_PMCR(state, tracer, s_22_1);
        // D s_22_3: write-var ga#185827 <= s_22_2
        fn_state.ga_185827 = s_22_2;
        // D s_22_4: read-var ga#185827.0:struct
        let s_22_4: u32 = fn_state.ga_185827._0;
        // D s_22_5: read-var t:i
        let s_22_5: i128 = fn_state.t;
        // D s_22_6: call R_set(s_22_5, s_22_4)
        let s_22_6: () = R_set(state, tracer, s_22_5, s_22_4);
        // N s_22_7: return
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
        // D s_24_1: write-var gs#113165 <= s_24_0
        fn_state.gs_113165 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#113165:u8
        let s_25_0: bool = fn_state.gs_113165;
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
        // D s_28_7: write-var gs#113165 <= s_28_6
        fn_state.gs_113165 = s_28_6;
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
        // D s_29_5: write-var gs#113164 <= s_29_4
        fn_state.gs_113164 = s_29_4;
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
        // D s_30_4: write-var gs#113163 <= s_30_3
        fn_state.gs_113163 = s_30_3;
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
        // D s_32_5: write-var gs#113162 <= s_32_4
        fn_state.gs_113162 = s_32_4;
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
        // D s_33_4: write-var gs#113161 <= s_33_3
        fn_state.gs_113161 = s_33_3;
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
        // D s_34_2: write-var gs#113160 <= s_34_1
        fn_state.gs_113160 = s_34_1;
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
        // D s_35_7: write-var gs#113159 <= s_35_6
        fn_state.gs_113159 = s_35_6;
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
        // D s_36_4: write-var gs#113158 <= s_36_3
        fn_state.gs_113158 = s_36_3;
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
        // N s_37_2: branch s_37_1 b115 b38
        if s_37_1 {
            return block_115(state, tracer, fn_state);
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
        // D s_38_1: write-var gs#113166 <= s_38_0
        fn_state.gs_113166 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#113166:u8
        let s_39_0: bool = fn_state.gs_113166;
        // N s_39_1: branch s_39_0 b114 b40
        if s_39_0 {
            return block_114(state, tracer, fn_state);
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
        // D s_40_1: write-var gs#113167 <= s_40_0
        fn_state.gs_113167 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var gs#113167:u8
        let s_41_0: bool = fn_state.gs_113167;
        // N s_41_1: branch s_41_0 b113 b42
        if s_41_0 {
            return block_113(state, tracer, fn_state);
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
        // D s_42_1: write-var gs#113168 <= s_42_0
        fn_state.gs_113168 = s_42_0;
        // N s_42_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var gs#113168:u8
        let s_43_0: bool = fn_state.gs_113168;
        // N s_43_1: branch s_43_0 b112 b44
        if s_43_0 {
            return block_112(state, tracer, fn_state);
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
        // D s_44_1: write-var gs#113169 <= s_44_0
        fn_state.gs_113169 = s_44_0;
        // N s_44_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var gs#113169:u8
        let s_45_0: bool = fn_state.gs_113169;
        // N s_45_1: branch s_45_0 b111 b46
        if s_45_0 {
            return block_111(state, tracer, fn_state);
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
        // D s_46_1: write-var gs#113170 <= s_46_0
        fn_state.gs_113170 = s_46_0;
        // N s_46_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var gs#113170:u8
        let s_47_0: bool = fn_state.gs_113170;
        // N s_47_1: branch s_47_0 b110 b48
        if s_47_0 {
            return block_110(state, tracer, fn_state);
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
        // N s_48_2: branch s_48_1 b109 b49
        if s_48_1 {
            return block_109(state, tracer, fn_state);
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
        // D s_49_1: write-var gs#113171 <= s_49_0
        fn_state.gs_113171 = s_49_0;
        // N s_49_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var gs#113171:u8
        let s_50_0: bool = fn_state.gs_113171;
        // N s_50_1: branch s_50_0 b108 b51
        if s_50_0 {
            return block_108(state, tracer, fn_state);
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
        // D s_51_1: write-var gs#113172 <= s_51_0
        fn_state.gs_113172 = s_51_0;
        // N s_51_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var gs#113172:u8
        let s_52_0: bool = fn_state.gs_113172;
        // N s_52_1: branch s_52_0 b107 b53
        if s_52_0 {
            return block_107(state, tracer, fn_state);
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
        // N s_53_2: branch s_53_1 b106 b54
        if s_53_1 {
            return block_106(state, tracer, fn_state);
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
        // D s_54_1: write-var gs#113173 <= s_54_0
        fn_state.gs_113173 = s_54_0;
        // N s_54_2: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var gs#113173:u8
        let s_55_0: bool = fn_state.gs_113173;
        // N s_55_1: branch s_55_0 b105 b56
        if s_55_0 {
            return block_105(state, tracer, fn_state);
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
        // D s_56_1: write-var gs#113174 <= s_56_0
        fn_state.gs_113174 = s_56_0;
        // N s_56_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var gs#113174:u8
        let s_57_0: bool = fn_state.gs_113174;
        // N s_57_1: branch s_57_0 b104 b58
        if s_57_0 {
            return block_104(state, tracer, fn_state);
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
        // N s_58_2: branch s_58_1 b103 b59
        if s_58_1 {
            return block_103(state, tracer, fn_state);
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
        // D s_59_1: write-var gs#113175 <= s_59_0
        fn_state.gs_113175 = s_59_0;
        // N s_59_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var gs#113175:u8
        let s_60_0: bool = fn_state.gs_113175;
        // N s_60_1: branch s_60_0 b102 b61
        if s_60_0 {
            return block_102(state, tracer, fn_state);
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
        // D s_61_1: write-var gs#113176 <= s_61_0
        fn_state.gs_113176 = s_61_0;
        // N s_61_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var gs#113176:u8
        let s_62_0: bool = fn_state.gs_113176;
        // N s_62_1: branch s_62_0 b101 b63
        if s_62_0 {
            return block_101(state, tracer, fn_state);
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
        // N s_63_2: branch s_63_1 b100 b64
        if s_63_1 {
            return block_100(state, tracer, fn_state);
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
        // D s_64_1: write-var gs#113177 <= s_64_0
        fn_state.gs_113177 = s_64_0;
        // N s_64_2: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_65_0: read-var gs#113177:u8
        let s_65_0: bool = fn_state.gs_113177;
        // N s_65_1: branch s_65_0 b99 b66
        if s_65_0 {
            return block_99(state, tracer, fn_state);
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
        // D s_66_1: write-var gs#113178 <= s_66_0
        fn_state.gs_113178 = s_66_0;
        // N s_66_2: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var gs#113178:u8
        let s_67_0: bool = fn_state.gs_113178;
        // N s_67_1: branch s_67_0 b98 b68
        if s_67_0 {
            return block_98(state, tracer, fn_state);
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
        // N s_68_2: branch s_68_1 b97 b69
        if s_68_1 {
            return block_97(state, tracer, fn_state);
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
        // D s_69_1: write-var gs#113179 <= s_69_0
        fn_state.gs_113179 = s_69_0;
        // N s_69_2: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_70_0: read-var gs#113179:u8
        let s_70_0: bool = fn_state.gs_113179;
        // N s_70_1: branch s_70_0 b96 b71
        if s_70_0 {
            return block_96(state, tracer, fn_state);
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
        // D s_71_1: write-var gs#113180 <= s_71_0
        fn_state.gs_113180 = s_71_0;
        // N s_71_2: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_72_0: read-var gs#113180:u8
        let s_72_0: bool = fn_state.gs_113180;
        // N s_72_1: branch s_72_0 b95 b73
        if s_72_0 {
            return block_95(state, tracer, fn_state);
        } else {
            return block_73(state, tracer, fn_state);
        };
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #() : ()
        let s_73_0: () = ();
        // S s_73_1: call EL2Enabled(s_73_0)
        let s_73_1: bool = EL2Enabled(state, tracer, s_73_0);
        // N s_73_2: branch s_73_1 b94 b74
        if s_73_1 {
            return block_94(state, tracer, fn_state);
        } else {
            return block_74(state, tracer, fn_state);
        };
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #0u : u8
        let s_74_0: bool = false;
        // D s_74_1: write-var gs#113181 <= s_74_0
        fn_state.gs_113181 = s_74_0;
        // N s_74_2: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_75_0: read-var gs#113181:u8
        let s_75_0: bool = fn_state.gs_113181;
        // N s_75_1: branch s_75_0 b93 b76
        if s_75_0 {
            return block_93(state, tracer, fn_state);
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
        // D s_76_1: write-var gs#113182 <= s_76_0
        fn_state.gs_113182 = s_76_0;
        // N s_76_2: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_77_0: read-var gs#113182:u8
        let s_77_0: bool = fn_state.gs_113182;
        // N s_77_1: branch s_77_0 b92 b78
        if s_77_0 {
            return block_92(state, tracer, fn_state);
        } else {
            return block_78(state, tracer, fn_state);
        };
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_78_0: const #424u : u32
        let s_78_0: u32 = 424;
        // D s_78_1: read-reg s_78_0:u8
        let s_78_1: u8 = {
            let value = state.read_register::<u8>(s_78_0 as isize);
            tracer.read_register(s_78_0 as isize, value);
            value
        };
        // C s_78_2: const #2u : u8
        let s_78_2: u8 = 2;
        // D s_78_3: cmp-lt s_78_1 s_78_2
        let s_78_3: bool = ((s_78_1) < (s_78_2));
        // N s_78_4: branch s_78_3 b91 b79
        if s_78_3 {
            return block_91(state, tracer, fn_state);
        } else {
            return block_79(state, tracer, fn_state);
        };
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #0u : u8
        let s_79_0: bool = false;
        // D s_79_1: write-var gs#113183 <= s_79_0
        fn_state.gs_113183 = s_79_0;
        // N s_79_2: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_80_0: read-var gs#113183:u8
        let s_80_0: bool = fn_state.gs_113183;
        // N s_80_1: branch s_80_0 b90 b81
        if s_80_0 {
            return block_90(state, tracer, fn_state);
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
        // D s_81_1: write-var gs#113184 <= s_81_0
        fn_state.gs_113184 = s_81_0;
        // N s_81_2: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_82_0: read-var gs#113184:u8
        let s_82_0: bool = fn_state.gs_113184;
        // N s_82_1: branch s_82_0 b84 b83
        if s_82_0 {
            return block_84(state, tracer, fn_state);
        } else {
            return block_83(state, tracer, fn_state);
        };
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #() : ()
        let s_83_0: () = ();
        // S s_83_1: call PMCR_read(s_83_0)
        let s_83_1: ProductType700c18a878c5601b = PMCR_read(state, tracer, s_83_0);
        // S s_83_2: call __get_PMCR(s_83_1)
        let s_83_2: ProductType700c18a878c5601b = u__get_PMCR(state, tracer, s_83_1);
        // D s_83_3: write-var ga#185803 <= s_83_2
        fn_state.ga_185803 = s_83_2;
        // D s_83_4: read-var ga#185803.0:struct
        let s_83_4: u32 = fn_state.ga_185803._0;
        // D s_83_5: read-var t:i
        let s_83_5: i128 = fn_state.t;
        // D s_83_6: call R_set(s_83_5, s_83_4)
        let s_83_6: () = R_set(state, tracer, s_83_5, s_83_4);
        // N s_83_7: return
        return;
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_84_0: const #() : ()
        let s_84_0: () = ();
        // S s_84_1: call Halted(s_84_0)
        let s_84_1: bool = Halted(state, tracer, s_84_0);
        // N s_84_2: branch s_84_1 b89 b85
        if s_84_1 {
            return block_89(state, tracer, fn_state);
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
        // D s_85_1: write-var gs#113185 <= s_85_0
        fn_state.gs_113185 = s_85_0;
        // N s_85_2: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_86_0: read-var gs#113185:u8
        let s_86_0: bool = fn_state.gs_113185;
        // N s_86_1: branch s_86_0 b88 b87
        if s_86_0 {
            return block_88(state, tracer, fn_state);
        } else {
            return block_87(state, tracer, fn_state);
        };
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
        // C s_87_5: const #424u : u32
        let s_87_5: u32 = 424;
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
        // N s_88_0: panic
        panic!("{:?}", ());
        // N s_88_1: return
        return;
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_89_0: const #() : ()
        let s_89_0: () = ();
        // S s_89_1: call EDSCR_read(s_89_0)
        let s_89_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_89_0);
        // S s_89_2: call _get_EDSCR_Type_SDD(s_89_1)
        let s_89_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_89_1);
        // S s_89_3: cast zx s_89_2 -> bv
        let s_89_3: Bits = Bits::new(s_89_2 as u128, 1u16);
        // C s_89_4: const #1u : u8
        let s_89_4: bool = true;
        // C s_89_5: cast zx s_89_4 -> bv
        let s_89_5: Bits = Bits::new(s_89_4 as u128, 1u16);
        // S s_89_6: cmp-eq s_89_3 s_89_5
        let s_89_6: bool = ((s_89_3) == (s_89_5));
        // D s_89_7: write-var gs#113185 <= s_89_6
        fn_state.gs_113185 = s_89_6;
        // N s_89_8: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_90_0: read-var __MDCR_EL3_TPM:u8
        let s_90_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_90_1: cast zx s_90_0 -> bv
        let s_90_1: Bits = Bits::new(s_90_0 as u128, 1u16);
        // C s_90_2: const #1u : u8
        let s_90_2: bool = true;
        // C s_90_3: cast zx s_90_2 -> bv
        let s_90_3: Bits = Bits::new(s_90_2 as u128, 1u16);
        // D s_90_4: cmp-eq s_90_1 s_90_3
        let s_90_4: bool = ((s_90_1) == (s_90_3));
        // D s_90_5: write-var gs#113184 <= s_90_4
        fn_state.gs_113184 = s_90_4;
        // N s_90_6: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_91_0: const #424u : u32
        let s_91_0: u32 = 424;
        // D s_91_1: read-reg s_91_0:u8
        let s_91_1: u8 = {
            let value = state.read_register::<u8>(s_91_0 as isize);
            tracer.read_register(s_91_0 as isize, value);
            value
        };
        // D s_91_2: call ELUsingAArch32(s_91_1)
        let s_91_2: bool = ELUsingAArch32(state, tracer, s_91_1);
        // D s_91_3: not s_91_2
        let s_91_3: bool = !s_91_2;
        // D s_91_4: write-var gs#113183 <= s_91_3
        fn_state.gs_113183 = s_91_3;
        // N s_91_5: jump b80
        return block_80(state, tracer, fn_state);
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_92_0: const #3u : u8
        let s_92_0: u8 = 3;
        // C s_92_1: cast zx s_92_0 -> bv
        let s_92_1: Bits = Bits::new(s_92_0 as u128, 8u16);
        // C s_92_2: cast zx s_92_1 -> i
        let s_92_2: i128 = (s_92_1.value() as i128);
        // C s_92_3: cast reint s_92_2 -> i64
        let s_92_3: i64 = (s_92_2 as i64);
        // C s_92_4: cast zx s_92_3 -> i
        let s_92_4: i128 = (i128::try_from(s_92_3).unwrap());
        // S s_92_5: call AArch32_TakeHypTrapException(s_92_4)
        let s_92_5: () = AArch32_TakeHypTrapException(state, tracer, s_92_4);
        // N s_92_6: return
        return;
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_93_0: read-var __HDCR_TPMCR:u8
        let s_93_0: bool = fn_state.u__HDCR_TPMCR;
        // D s_93_1: cast zx s_93_0 -> bv
        let s_93_1: Bits = Bits::new(s_93_0 as u128, 1u16);
        // C s_93_2: const #1u : u8
        let s_93_2: bool = true;
        // C s_93_3: cast zx s_93_2 -> bv
        let s_93_3: Bits = Bits::new(s_93_2 as u128, 1u16);
        // D s_93_4: cmp-eq s_93_1 s_93_3
        let s_93_4: bool = ((s_93_1) == (s_93_3));
        // D s_93_5: write-var gs#113182 <= s_93_4
        fn_state.gs_113182 = s_93_4;
        // N s_93_6: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_94_0: const #432u : u32
        let s_94_0: u32 = 432;
        // D s_94_1: read-reg s_94_0:u8
        let s_94_1: u8 = {
            let value = state.read_register::<u8>(s_94_0 as isize);
            tracer.read_register(s_94_0 as isize, value);
            value
        };
        // D s_94_2: call ELUsingAArch32(s_94_1)
        let s_94_2: bool = ELUsingAArch32(state, tracer, s_94_1);
        // D s_94_3: write-var gs#113181 <= s_94_2
        fn_state.gs_113181 = s_94_2;
        // N s_94_4: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_95_0: const #3u : u8
        let s_95_0: u8 = 3;
        // C s_95_1: cast zx s_95_0 -> bv
        let s_95_1: Bits = Bits::new(s_95_0 as u128, 8u16);
        // C s_95_2: cast zx s_95_1 -> i
        let s_95_2: i128 = (s_95_1.value() as i128);
        // C s_95_3: cast reint s_95_2 -> i64
        let s_95_3: i64 = (s_95_2 as i64);
        // C s_95_4: cast zx s_95_3 -> i
        let s_95_4: i128 = (i128::try_from(s_95_3).unwrap());
        // S s_95_5: call AArch32_TakeHypTrapException(s_95_4)
        let s_95_5: () = AArch32_TakeHypTrapException(state, tracer, s_95_4);
        // N s_95_6: return
        return;
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_96_0: read-var __HDCR_TPM:u8
        let s_96_0: bool = fn_state.u__HDCR_TPM;
        // D s_96_1: cast zx s_96_0 -> bv
        let s_96_1: Bits = Bits::new(s_96_0 as u128, 1u16);
        // C s_96_2: const #1u : u8
        let s_96_2: bool = true;
        // C s_96_3: cast zx s_96_2 -> bv
        let s_96_3: Bits = Bits::new(s_96_2 as u128, 1u16);
        // D s_96_4: cmp-eq s_96_1 s_96_3
        let s_96_4: bool = ((s_96_1) == (s_96_3));
        // D s_96_5: write-var gs#113180 <= s_96_4
        fn_state.gs_113180 = s_96_4;
        // N s_96_6: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_97_0: const #432u : u32
        let s_97_0: u32 = 432;
        // D s_97_1: read-reg s_97_0:u8
        let s_97_1: u8 = {
            let value = state.read_register::<u8>(s_97_0 as isize);
            tracer.read_register(s_97_0 as isize, value);
            value
        };
        // D s_97_2: call ELUsingAArch32(s_97_1)
        let s_97_2: bool = ELUsingAArch32(state, tracer, s_97_1);
        // D s_97_3: write-var gs#113179 <= s_97_2
        fn_state.gs_113179 = s_97_2;
        // N s_97_4: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_98_0: const #3u : u8
        let s_98_0: u8 = 3;
        // C s_98_1: cast zx s_98_0 -> bv
        let s_98_1: Bits = Bits::new(s_98_0 as u128, 8u16);
        // C s_98_2: cast zx s_98_1 -> i
        let s_98_2: i128 = (s_98_1.value() as i128);
        // C s_98_3: cast reint s_98_2 -> i64
        let s_98_3: i64 = (s_98_2 as i64);
        // C s_98_4: cast zx s_98_3 -> i
        let s_98_4: i128 = (i128::try_from(s_98_3).unwrap());
        // C s_98_5: const #432u : u32
        let s_98_5: u32 = 432;
        // D s_98_6: read-reg s_98_5:u8
        let s_98_6: u8 = {
            let value = state.read_register::<u8>(s_98_5 as isize);
            tracer.read_register(s_98_5 as isize, value);
            value
        };
        // D s_98_7: call AArch64_AArch32SystemAccessTrap(s_98_6, s_98_4)
        let s_98_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_98_6, s_98_4);
        // N s_98_8: return
        return;
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_99_0: read-var __MDCR_EL2_TPMCR:u8
        let s_99_0: bool = fn_state.u__MDCR_EL2_TPMCR;
        // D s_99_1: cast zx s_99_0 -> bv
        let s_99_1: Bits = Bits::new(s_99_0 as u128, 1u16);
        // C s_99_2: const #1u : u8
        let s_99_2: bool = true;
        // C s_99_3: cast zx s_99_2 -> bv
        let s_99_3: Bits = Bits::new(s_99_2 as u128, 1u16);
        // D s_99_4: cmp-eq s_99_1 s_99_3
        let s_99_4: bool = ((s_99_1) == (s_99_3));
        // D s_99_5: write-var gs#113178 <= s_99_4
        fn_state.gs_113178 = s_99_4;
        // N s_99_6: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_100_0: const #432u : u32
        let s_100_0: u32 = 432;
        // D s_100_1: read-reg s_100_0:u8
        let s_100_1: u8 = {
            let value = state.read_register::<u8>(s_100_0 as isize);
            tracer.read_register(s_100_0 as isize, value);
            value
        };
        // D s_100_2: call ELUsingAArch32(s_100_1)
        let s_100_2: bool = ELUsingAArch32(state, tracer, s_100_1);
        // D s_100_3: not s_100_2
        let s_100_3: bool = !s_100_2;
        // D s_100_4: write-var gs#113177 <= s_100_3
        fn_state.gs_113177 = s_100_3;
        // N s_100_5: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_101_0: const #3u : u8
        let s_101_0: u8 = 3;
        // C s_101_1: cast zx s_101_0 -> bv
        let s_101_1: Bits = Bits::new(s_101_0 as u128, 8u16);
        // C s_101_2: cast zx s_101_1 -> i
        let s_101_2: i128 = (s_101_1.value() as i128);
        // C s_101_3: cast reint s_101_2 -> i64
        let s_101_3: i64 = (s_101_2 as i64);
        // C s_101_4: cast zx s_101_3 -> i
        let s_101_4: i128 = (i128::try_from(s_101_3).unwrap());
        // C s_101_5: const #432u : u32
        let s_101_5: u32 = 432;
        // D s_101_6: read-reg s_101_5:u8
        let s_101_6: u8 = {
            let value = state.read_register::<u8>(s_101_5 as isize);
            tracer.read_register(s_101_5 as isize, value);
            value
        };
        // D s_101_7: call AArch64_AArch32SystemAccessTrap(s_101_6, s_101_4)
        let s_101_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_101_6,
            s_101_4,
        );
        // N s_101_8: return
        return;
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_102_0: read-var __MDCR_EL2_TPM:u8
        let s_102_0: bool = fn_state.u__MDCR_EL2_TPM;
        // D s_102_1: cast zx s_102_0 -> bv
        let s_102_1: Bits = Bits::new(s_102_0 as u128, 1u16);
        // C s_102_2: const #1u : u8
        let s_102_2: bool = true;
        // C s_102_3: cast zx s_102_2 -> bv
        let s_102_3: Bits = Bits::new(s_102_2 as u128, 1u16);
        // D s_102_4: cmp-eq s_102_1 s_102_3
        let s_102_4: bool = ((s_102_1) == (s_102_3));
        // D s_102_5: write-var gs#113176 <= s_102_4
        fn_state.gs_113176 = s_102_4;
        // N s_102_6: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_103_0: const #432u : u32
        let s_103_0: u32 = 432;
        // D s_103_1: read-reg s_103_0:u8
        let s_103_1: u8 = {
            let value = state.read_register::<u8>(s_103_0 as isize);
            tracer.read_register(s_103_0 as isize, value);
            value
        };
        // D s_103_2: call ELUsingAArch32(s_103_1)
        let s_103_2: bool = ELUsingAArch32(state, tracer, s_103_1);
        // D s_103_3: not s_103_2
        let s_103_3: bool = !s_103_2;
        // D s_103_4: write-var gs#113175 <= s_103_3
        fn_state.gs_113175 = s_103_3;
        // N s_103_5: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_104_0: const #3u : u8
        let s_104_0: u8 = 3;
        // C s_104_1: cast zx s_104_0 -> bv
        let s_104_1: Bits = Bits::new(s_104_0 as u128, 8u16);
        // C s_104_2: cast zx s_104_1 -> i
        let s_104_2: i128 = (s_104_1.value() as i128);
        // C s_104_3: cast reint s_104_2 -> i64
        let s_104_3: i64 = (s_104_2 as i64);
        // C s_104_4: cast zx s_104_3 -> i
        let s_104_4: i128 = (i128::try_from(s_104_3).unwrap());
        // S s_104_5: call AArch32_TakeHypTrapException(s_104_4)
        let s_104_5: () = AArch32_TakeHypTrapException(state, tracer, s_104_4);
        // N s_104_6: return
        return;
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_105_0: read-var __HSTR_T9:u8
        let s_105_0: bool = fn_state.u__HSTR_T9;
        // D s_105_1: cast zx s_105_0 -> bv
        let s_105_1: Bits = Bits::new(s_105_0 as u128, 1u16);
        // C s_105_2: const #1u : u8
        let s_105_2: bool = true;
        // C s_105_3: cast zx s_105_2 -> bv
        let s_105_3: Bits = Bits::new(s_105_2 as u128, 1u16);
        // D s_105_4: cmp-eq s_105_1 s_105_3
        let s_105_4: bool = ((s_105_1) == (s_105_3));
        // D s_105_5: write-var gs#113174 <= s_105_4
        fn_state.gs_113174 = s_105_4;
        // N s_105_6: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_106_0: const #432u : u32
        let s_106_0: u32 = 432;
        // D s_106_1: read-reg s_106_0:u8
        let s_106_1: u8 = {
            let value = state.read_register::<u8>(s_106_0 as isize);
            tracer.read_register(s_106_0 as isize, value);
            value
        };
        // D s_106_2: call ELUsingAArch32(s_106_1)
        let s_106_2: bool = ELUsingAArch32(state, tracer, s_106_1);
        // D s_106_3: write-var gs#113173 <= s_106_2
        fn_state.gs_113173 = s_106_2;
        // N s_106_4: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_107_0: const #3u : u8
        let s_107_0: u8 = 3;
        // C s_107_1: cast zx s_107_0 -> bv
        let s_107_1: Bits = Bits::new(s_107_0 as u128, 8u16);
        // C s_107_2: cast zx s_107_1 -> i
        let s_107_2: i128 = (s_107_1.value() as i128);
        // C s_107_3: cast reint s_107_2 -> i64
        let s_107_3: i64 = (s_107_2 as i64);
        // C s_107_4: cast zx s_107_3 -> i
        let s_107_4: i128 = (i128::try_from(s_107_3).unwrap());
        // C s_107_5: const #432u : u32
        let s_107_5: u32 = 432;
        // D s_107_6: read-reg s_107_5:u8
        let s_107_6: u8 = {
            let value = state.read_register::<u8>(s_107_5 as isize);
            tracer.read_register(s_107_5 as isize, value);
            value
        };
        // D s_107_7: call AArch64_AArch32SystemAccessTrap(s_107_6, s_107_4)
        let s_107_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_107_6,
            s_107_4,
        );
        // N s_107_8: return
        return;
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_108_0: read-var __HSTR_EL2_T9:u8
        let s_108_0: bool = fn_state.u__HSTR_EL2_T9;
        // D s_108_1: cast zx s_108_0 -> bv
        let s_108_1: Bits = Bits::new(s_108_0 as u128, 1u16);
        // C s_108_2: const #1u : u8
        let s_108_2: bool = true;
        // C s_108_3: cast zx s_108_2 -> bv
        let s_108_3: Bits = Bits::new(s_108_2 as u128, 1u16);
        // D s_108_4: cmp-eq s_108_1 s_108_3
        let s_108_4: bool = ((s_108_1) == (s_108_3));
        // D s_108_5: write-var gs#113172 <= s_108_4
        fn_state.gs_113172 = s_108_4;
        // N s_108_6: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_109_0: const #432u : u32
        let s_109_0: u32 = 432;
        // D s_109_1: read-reg s_109_0:u8
        let s_109_1: u8 = {
            let value = state.read_register::<u8>(s_109_0 as isize);
            tracer.read_register(s_109_0 as isize, value);
            value
        };
        // D s_109_2: call ELUsingAArch32(s_109_1)
        let s_109_2: bool = ELUsingAArch32(state, tracer, s_109_1);
        // D s_109_3: not s_109_2
        let s_109_3: bool = !s_109_2;
        // D s_109_4: write-var gs#113171 <= s_109_3
        fn_state.gs_113171 = s_109_3;
        // N s_109_5: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_110_0: panic
        panic!("{:?}", ());
        // N s_110_1: return
        return;
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_111_0: read-var __MDCR_EL3_TPM:u8
        let s_111_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_111_1: cast zx s_111_0 -> bv
        let s_111_1: Bits = Bits::new(s_111_0 as u128, 1u16);
        // C s_111_2: const #1u : u8
        let s_111_2: bool = true;
        // C s_111_3: cast zx s_111_2 -> bv
        let s_111_3: Bits = Bits::new(s_111_2 as u128, 1u16);
        // D s_111_4: cmp-eq s_111_1 s_111_3
        let s_111_4: bool = ((s_111_1) == (s_111_3));
        // D s_111_5: write-var gs#113170 <= s_111_4
        fn_state.gs_113170 = s_111_4;
        // N s_111_6: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_112_0: const #424u : u32
        let s_112_0: u32 = 424;
        // D s_112_1: read-reg s_112_0:u8
        let s_112_1: u8 = {
            let value = state.read_register::<u8>(s_112_0 as isize);
            tracer.read_register(s_112_0 as isize, value);
            value
        };
        // D s_112_2: call ELUsingAArch32(s_112_1)
        let s_112_2: bool = ELUsingAArch32(state, tracer, s_112_1);
        // D s_112_3: not s_112_2
        let s_112_3: bool = !s_112_2;
        // D s_112_4: write-var gs#113169 <= s_112_3
        fn_state.gs_113169 = s_112_3;
        // N s_112_5: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_113_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_113_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_113_1: call __IMPDEF_boolean(s_113_0)
        let s_113_1: bool = u__IMPDEF_boolean(state, tracer, s_113_0);
        // D s_113_2: write-var gs#113168 <= s_113_1
        fn_state.gs_113168 = s_113_1;
        // N s_113_3: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_114_0: const #() : ()
        let s_114_0: () = ();
        // S s_114_1: call EDSCR_read(s_114_0)
        let s_114_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_114_0);
        // S s_114_2: call _get_EDSCR_Type_SDD(s_114_1)
        let s_114_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_114_1);
        // S s_114_3: cast zx s_114_2 -> bv
        let s_114_3: Bits = Bits::new(s_114_2 as u128, 1u16);
        // C s_114_4: const #1u : u8
        let s_114_4: bool = true;
        // C s_114_5: cast zx s_114_4 -> bv
        let s_114_5: Bits = Bits::new(s_114_4 as u128, 1u16);
        // S s_114_6: cmp-eq s_114_3 s_114_5
        let s_114_6: bool = ((s_114_3) == (s_114_5));
        // D s_114_7: write-var gs#113167 <= s_114_6
        fn_state.gs_113167 = s_114_6;
        // N s_114_8: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_115_0: const #424u : u32
        let s_115_0: u32 = 424;
        // D s_115_1: read-reg s_115_0:u8
        let s_115_1: u8 = {
            let value = state.read_register::<u8>(s_115_0 as isize);
            tracer.read_register(s_115_0 as isize, value);
            value
        };
        // C s_115_2: const #2u : u8
        let s_115_2: u8 = 2;
        // D s_115_3: cmp-lt s_115_1 s_115_2
        let s_115_3: bool = ((s_115_1) < (s_115_2));
        // D s_115_4: write-var gs#113166 <= s_115_3
        fn_state.gs_113166 = s_115_3;
        // N s_115_5: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_116_0: const #() : ()
        let s_116_0: () = ();
        // S s_116_1: call Halted(s_116_0)
        let s_116_1: bool = Halted(state, tracer, s_116_0);
        // N s_116_2: branch s_116_1 b240 b117
        if s_116_1 {
            return block_240(state, tracer, fn_state);
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
        // D s_117_1: write-var gs#113186 <= s_117_0
        fn_state.gs_113186 = s_117_0;
        // N s_117_2: jump b118
        return block_118(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_118_0: read-var gs#113186:u8
        let s_118_0: bool = fn_state.gs_113186;
        // N s_118_1: branch s_118_0 b239 b119
        if s_118_0 {
            return block_239(state, tracer, fn_state);
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
        // D s_119_1: write-var gs#113187 <= s_119_0
        fn_state.gs_113187 = s_119_0;
        // N s_119_2: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_120_0: read-var gs#113187:u8
        let s_120_0: bool = fn_state.gs_113187;
        // N s_120_1: branch s_120_0 b238 b121
        if s_120_0 {
            return block_238(state, tracer, fn_state);
        } else {
            return block_121(state, tracer, fn_state);
        };
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_121_0: const #0u : u8
        let s_121_0: bool = false;
        // D s_121_1: write-var gs#113188 <= s_121_0
        fn_state.gs_113188 = s_121_0;
        // N s_121_2: jump b122
        return block_122(state, tracer, fn_state);
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_122_0: read-var gs#113188:u8
        let s_122_0: bool = fn_state.gs_113188;
        // N s_122_1: branch s_122_0 b237 b123
        if s_122_0 {
            return block_237(state, tracer, fn_state);
        } else {
            return block_123(state, tracer, fn_state);
        };
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_123_0: const #0u : u8
        let s_123_0: bool = false;
        // D s_123_1: write-var gs#113189 <= s_123_0
        fn_state.gs_113189 = s_123_0;
        // N s_123_2: jump b124
        return block_124(state, tracer, fn_state);
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_124_0: read-var gs#113189:u8
        let s_124_0: bool = fn_state.gs_113189;
        // N s_124_1: branch s_124_0 b236 b125
        if s_124_0 {
            return block_236(state, tracer, fn_state);
        } else {
            return block_125(state, tracer, fn_state);
        };
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_125_0: const #0u : u8
        let s_125_0: bool = false;
        // D s_125_1: write-var gs#113190 <= s_125_0
        fn_state.gs_113190 = s_125_0;
        // N s_125_2: jump b126
        return block_126(state, tracer, fn_state);
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_126_0: read-var gs#113190:u8
        let s_126_0: bool = fn_state.gs_113190;
        // N s_126_1: branch s_126_0 b235 b127
        if s_126_0 {
            return block_235(state, tracer, fn_state);
        } else {
            return block_127(state, tracer, fn_state);
        };
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_127_0: const #440u : u32
        let s_127_0: u32 = 440;
        // D s_127_1: read-reg s_127_0:u8
        let s_127_1: u8 = {
            let value = state.read_register::<u8>(s_127_0 as isize);
            tracer.read_register(s_127_0 as isize, value);
            value
        };
        // D s_127_2: call ELUsingAArch32(s_127_1)
        let s_127_2: bool = ELUsingAArch32(state, tracer, s_127_1);
        // D s_127_3: not s_127_2
        let s_127_3: bool = !s_127_2;
        // N s_127_4: branch s_127_3 b225 b128
        if s_127_3 {
            return block_225(state, tracer, fn_state);
        } else {
            return block_128(state, tracer, fn_state);
        };
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_128_0: const #0u : u8
        let s_128_0: bool = false;
        // D s_128_1: write-var gs#113194 <= s_128_0
        fn_state.gs_113194 = s_128_0;
        // N s_128_2: jump b129
        return block_129(state, tracer, fn_state);
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_129_0: read-var gs#113194:u8
        let s_129_0: bool = fn_state.gs_113194;
        // N s_129_1: branch s_129_0 b216 b130
        if s_129_0 {
            return block_216(state, tracer, fn_state);
        } else {
            return block_130(state, tracer, fn_state);
        };
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_130_0: const #440u : u32
        let s_130_0: u32 = 440;
        // D s_130_1: read-reg s_130_0:u8
        let s_130_1: u8 = {
            let value = state.read_register::<u8>(s_130_0 as isize);
            tracer.read_register(s_130_0 as isize, value);
            value
        };
        // D s_130_2: call ELUsingAArch32(s_130_1)
        let s_130_2: bool = ELUsingAArch32(state, tracer, s_130_1);
        // N s_130_3: branch s_130_2 b215 b131
        if s_130_2 {
            return block_215(state, tracer, fn_state);
        } else {
            return block_131(state, tracer, fn_state);
        };
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_131_0: const #0u : u8
        let s_131_0: bool = false;
        // D s_131_1: write-var gs#113195 <= s_131_0
        fn_state.gs_113195 = s_131_0;
        // N s_131_2: jump b132
        return block_132(state, tracer, fn_state);
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_132_0: read-var gs#113195:u8
        let s_132_0: bool = fn_state.gs_113195;
        // N s_132_1: branch s_132_0 b198 b133
        if s_132_0 {
            return block_198(state, tracer, fn_state);
        } else {
            return block_133(state, tracer, fn_state);
        };
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_133_0: const #() : ()
        let s_133_0: () = ();
        // S s_133_1: call EL2Enabled(s_133_0)
        let s_133_1: bool = EL2Enabled(state, tracer, s_133_0);
        // N s_133_2: branch s_133_1 b197 b134
        if s_133_1 {
            return block_197(state, tracer, fn_state);
        } else {
            return block_134(state, tracer, fn_state);
        };
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_134_0: const #0u : u8
        let s_134_0: bool = false;
        // D s_134_1: write-var gs#113196 <= s_134_0
        fn_state.gs_113196 = s_134_0;
        // N s_134_2: jump b135
        return block_135(state, tracer, fn_state);
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_135_0: read-var gs#113196:u8
        let s_135_0: bool = fn_state.gs_113196;
        // N s_135_1: branch s_135_0 b196 b136
        if s_135_0 {
            return block_196(state, tracer, fn_state);
        } else {
            return block_136(state, tracer, fn_state);
        };
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_136_0: const #0u : u8
        let s_136_0: bool = false;
        // D s_136_1: write-var gs#113197 <= s_136_0
        fn_state.gs_113197 = s_136_0;
        // N s_136_2: jump b137
        return block_137(state, tracer, fn_state);
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_137_0: read-var gs#113197:u8
        let s_137_0: bool = fn_state.gs_113197;
        // N s_137_1: branch s_137_0 b195 b138
        if s_137_0 {
            return block_195(state, tracer, fn_state);
        } else {
            return block_138(state, tracer, fn_state);
        };
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_138_0: const #0u : u8
        let s_138_0: bool = false;
        // D s_138_1: write-var gs#113198 <= s_138_0
        fn_state.gs_113198 = s_138_0;
        // N s_138_2: jump b139
        return block_139(state, tracer, fn_state);
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_139_0: read-var gs#113198:u8
        let s_139_0: bool = fn_state.gs_113198;
        // N s_139_1: branch s_139_0 b194 b140
        if s_139_0 {
            return block_194(state, tracer, fn_state);
        } else {
            return block_140(state, tracer, fn_state);
        };
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_140_0: const #() : ()
        let s_140_0: () = ();
        // S s_140_1: call EL2Enabled(s_140_0)
        let s_140_1: bool = EL2Enabled(state, tracer, s_140_0);
        // N s_140_2: branch s_140_1 b193 b141
        if s_140_1 {
            return block_193(state, tracer, fn_state);
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
        // D s_141_1: write-var gs#113199 <= s_141_0
        fn_state.gs_113199 = s_141_0;
        // N s_141_2: jump b142
        return block_142(state, tracer, fn_state);
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_142_0: read-var gs#113199:u8
        let s_142_0: bool = fn_state.gs_113199;
        // N s_142_1: branch s_142_0 b192 b143
        if s_142_0 {
            return block_192(state, tracer, fn_state);
        } else {
            return block_143(state, tracer, fn_state);
        };
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_143_0: const #0u : u8
        let s_143_0: bool = false;
        // D s_143_1: write-var gs#113200 <= s_143_0
        fn_state.gs_113200 = s_143_0;
        // N s_143_2: jump b144
        return block_144(state, tracer, fn_state);
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_144_0: read-var gs#113200:u8
        let s_144_0: bool = fn_state.gs_113200;
        // N s_144_1: branch s_144_0 b191 b145
        if s_144_0 {
            return block_191(state, tracer, fn_state);
        } else {
            return block_145(state, tracer, fn_state);
        };
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_145_0: const #() : ()
        let s_145_0: () = ();
        // S s_145_1: call EL2Enabled(s_145_0)
        let s_145_1: bool = EL2Enabled(state, tracer, s_145_0);
        // N s_145_2: branch s_145_1 b190 b146
        if s_145_1 {
            return block_190(state, tracer, fn_state);
        } else {
            return block_146(state, tracer, fn_state);
        };
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_146_0: const #0u : u8
        let s_146_0: bool = false;
        // D s_146_1: write-var gs#113201 <= s_146_0
        fn_state.gs_113201 = s_146_0;
        // N s_146_2: jump b147
        return block_147(state, tracer, fn_state);
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_147_0: read-var gs#113201:u8
        let s_147_0: bool = fn_state.gs_113201;
        // N s_147_1: branch s_147_0 b189 b148
        if s_147_0 {
            return block_189(state, tracer, fn_state);
        } else {
            return block_148(state, tracer, fn_state);
        };
    }
    fn block_148<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_148_0: const #0u : u8
        let s_148_0: bool = false;
        // D s_148_1: write-var gs#113202 <= s_148_0
        fn_state.gs_113202 = s_148_0;
        // N s_148_2: jump b149
        return block_149(state, tracer, fn_state);
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_149_0: read-var gs#113202:u8
        let s_149_0: bool = fn_state.gs_113202;
        // N s_149_1: branch s_149_0 b188 b150
        if s_149_0 {
            return block_188(state, tracer, fn_state);
        } else {
            return block_150(state, tracer, fn_state);
        };
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_150_0: const #() : ()
        let s_150_0: () = ();
        // S s_150_1: call EL2Enabled(s_150_0)
        let s_150_1: bool = EL2Enabled(state, tracer, s_150_0);
        // N s_150_2: branch s_150_1 b187 b151
        if s_150_1 {
            return block_187(state, tracer, fn_state);
        } else {
            return block_151(state, tracer, fn_state);
        };
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_151_0: const #0u : u8
        let s_151_0: bool = false;
        // D s_151_1: write-var gs#113203 <= s_151_0
        fn_state.gs_113203 = s_151_0;
        // N s_151_2: jump b152
        return block_152(state, tracer, fn_state);
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_152_0: read-var gs#113203:u8
        let s_152_0: bool = fn_state.gs_113203;
        // N s_152_1: branch s_152_0 b186 b153
        if s_152_0 {
            return block_186(state, tracer, fn_state);
        } else {
            return block_153(state, tracer, fn_state);
        };
    }
    fn block_153<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_153_0: const #0u : u8
        let s_153_0: bool = false;
        // D s_153_1: write-var gs#113204 <= s_153_0
        fn_state.gs_113204 = s_153_0;
        // N s_153_2: jump b154
        return block_154(state, tracer, fn_state);
    }
    fn block_154<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_154_0: read-var gs#113204:u8
        let s_154_0: bool = fn_state.gs_113204;
        // N s_154_1: branch s_154_0 b185 b155
        if s_154_0 {
            return block_185(state, tracer, fn_state);
        } else {
            return block_155(state, tracer, fn_state);
        };
    }
    fn block_155<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_155_0: const #() : ()
        let s_155_0: () = ();
        // S s_155_1: call EL2Enabled(s_155_0)
        let s_155_1: bool = EL2Enabled(state, tracer, s_155_0);
        // N s_155_2: branch s_155_1 b184 b156
        if s_155_1 {
            return block_184(state, tracer, fn_state);
        } else {
            return block_156(state, tracer, fn_state);
        };
    }
    fn block_156<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_156_0: const #0u : u8
        let s_156_0: bool = false;
        // D s_156_1: write-var gs#113205 <= s_156_0
        fn_state.gs_113205 = s_156_0;
        // N s_156_2: jump b157
        return block_157(state, tracer, fn_state);
    }
    fn block_157<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_157_0: read-var gs#113205:u8
        let s_157_0: bool = fn_state.gs_113205;
        // N s_157_1: branch s_157_0 b183 b158
        if s_157_0 {
            return block_183(state, tracer, fn_state);
        } else {
            return block_158(state, tracer, fn_state);
        };
    }
    fn block_158<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_158_0: const #0u : u8
        let s_158_0: bool = false;
        // D s_158_1: write-var gs#113206 <= s_158_0
        fn_state.gs_113206 = s_158_0;
        // N s_158_2: jump b159
        return block_159(state, tracer, fn_state);
    }
    fn block_159<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_159_0: read-var gs#113206:u8
        let s_159_0: bool = fn_state.gs_113206;
        // N s_159_1: branch s_159_0 b182 b160
        if s_159_0 {
            return block_182(state, tracer, fn_state);
        } else {
            return block_160(state, tracer, fn_state);
        };
    }
    fn block_160<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_160_0: const #() : ()
        let s_160_0: () = ();
        // S s_160_1: call EL2Enabled(s_160_0)
        let s_160_1: bool = EL2Enabled(state, tracer, s_160_0);
        // N s_160_2: branch s_160_1 b181 b161
        if s_160_1 {
            return block_181(state, tracer, fn_state);
        } else {
            return block_161(state, tracer, fn_state);
        };
    }
    fn block_161<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_161_0: const #0u : u8
        let s_161_0: bool = false;
        // D s_161_1: write-var gs#113207 <= s_161_0
        fn_state.gs_113207 = s_161_0;
        // N s_161_2: jump b162
        return block_162(state, tracer, fn_state);
    }
    fn block_162<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_162_0: read-var gs#113207:u8
        let s_162_0: bool = fn_state.gs_113207;
        // N s_162_1: branch s_162_0 b180 b163
        if s_162_0 {
            return block_180(state, tracer, fn_state);
        } else {
            return block_163(state, tracer, fn_state);
        };
    }
    fn block_163<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_163_0: const #0u : u8
        let s_163_0: bool = false;
        // D s_163_1: write-var gs#113208 <= s_163_0
        fn_state.gs_113208 = s_163_0;
        // N s_163_2: jump b164
        return block_164(state, tracer, fn_state);
    }
    fn block_164<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_164_0: read-var gs#113208:u8
        let s_164_0: bool = fn_state.gs_113208;
        // N s_164_1: branch s_164_0 b179 b165
        if s_164_0 {
            return block_179(state, tracer, fn_state);
        } else {
            return block_165(state, tracer, fn_state);
        };
    }
    fn block_165<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_165_0: const #424u : u32
        let s_165_0: u32 = 424;
        // D s_165_1: read-reg s_165_0:u8
        let s_165_1: u8 = {
            let value = state.read_register::<u8>(s_165_0 as isize);
            tracer.read_register(s_165_0 as isize, value);
            value
        };
        // C s_165_2: const #2u : u8
        let s_165_2: u8 = 2;
        // D s_165_3: cmp-lt s_165_1 s_165_2
        let s_165_3: bool = ((s_165_1) < (s_165_2));
        // N s_165_4: branch s_165_3 b178 b166
        if s_165_3 {
            return block_178(state, tracer, fn_state);
        } else {
            return block_166(state, tracer, fn_state);
        };
    }
    fn block_166<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_166_0: const #0u : u8
        let s_166_0: bool = false;
        // D s_166_1: write-var gs#113209 <= s_166_0
        fn_state.gs_113209 = s_166_0;
        // N s_166_2: jump b167
        return block_167(state, tracer, fn_state);
    }
    fn block_167<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_167_0: read-var gs#113209:u8
        let s_167_0: bool = fn_state.gs_113209;
        // N s_167_1: branch s_167_0 b177 b168
        if s_167_0 {
            return block_177(state, tracer, fn_state);
        } else {
            return block_168(state, tracer, fn_state);
        };
    }
    fn block_168<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_168_0: const #0u : u8
        let s_168_0: bool = false;
        // D s_168_1: write-var gs#113210 <= s_168_0
        fn_state.gs_113210 = s_168_0;
        // N s_168_2: jump b169
        return block_169(state, tracer, fn_state);
    }
    fn block_169<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_169_0: read-var gs#113210:u8
        let s_169_0: bool = fn_state.gs_113210;
        // N s_169_1: branch s_169_0 b171 b170
        if s_169_0 {
            return block_171(state, tracer, fn_state);
        } else {
            return block_170(state, tracer, fn_state);
        };
    }
    fn block_170<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_170_0: const #() : ()
        let s_170_0: () = ();
        // S s_170_1: call PMCR_read(s_170_0)
        let s_170_1: ProductType700c18a878c5601b = PMCR_read(state, tracer, s_170_0);
        // S s_170_2: call __get_PMCR(s_170_1)
        let s_170_2: ProductType700c18a878c5601b = u__get_PMCR(state, tracer, s_170_1);
        // D s_170_3: write-var ga#185752 <= s_170_2
        fn_state.ga_185752 = s_170_2;
        // D s_170_4: read-var ga#185752.0:struct
        let s_170_4: u32 = fn_state.ga_185752._0;
        // D s_170_5: read-var t:i
        let s_170_5: i128 = fn_state.t;
        // D s_170_6: call R_set(s_170_5, s_170_4)
        let s_170_6: () = R_set(state, tracer, s_170_5, s_170_4);
        // N s_170_7: return
        return;
    }
    fn block_171<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_171_0: const #() : ()
        let s_171_0: () = ();
        // S s_171_1: call Halted(s_171_0)
        let s_171_1: bool = Halted(state, tracer, s_171_0);
        // N s_171_2: branch s_171_1 b176 b172
        if s_171_1 {
            return block_176(state, tracer, fn_state);
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
        // D s_172_1: write-var gs#113211 <= s_172_0
        fn_state.gs_113211 = s_172_0;
        // N s_172_2: jump b173
        return block_173(state, tracer, fn_state);
    }
    fn block_173<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_173_0: read-var gs#113211:u8
        let s_173_0: bool = fn_state.gs_113211;
        // N s_173_1: branch s_173_0 b175 b174
        if s_173_0 {
            return block_175(state, tracer, fn_state);
        } else {
            return block_174(state, tracer, fn_state);
        };
    }
    fn block_174<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_174_0: const #3u : u8
        let s_174_0: u8 = 3;
        // C s_174_1: cast zx s_174_0 -> bv
        let s_174_1: Bits = Bits::new(s_174_0 as u128, 8u16);
        // C s_174_2: cast zx s_174_1 -> i
        let s_174_2: i128 = (s_174_1.value() as i128);
        // C s_174_3: cast reint s_174_2 -> i64
        let s_174_3: i64 = (s_174_2 as i64);
        // C s_174_4: cast zx s_174_3 -> i
        let s_174_4: i128 = (i128::try_from(s_174_3).unwrap());
        // C s_174_5: const #424u : u32
        let s_174_5: u32 = 424;
        // D s_174_6: read-reg s_174_5:u8
        let s_174_6: u8 = {
            let value = state.read_register::<u8>(s_174_5 as isize);
            tracer.read_register(s_174_5 as isize, value);
            value
        };
        // D s_174_7: call AArch64_AArch32SystemAccessTrap(s_174_6, s_174_4)
        let s_174_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_174_6,
            s_174_4,
        );
        // N s_174_8: return
        return;
    }
    fn block_175<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_175_0: panic
        panic!("{:?}", ());
        // N s_175_1: return
        return;
    }
    fn block_176<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_176_0: const #() : ()
        let s_176_0: () = ();
        // S s_176_1: call EDSCR_read(s_176_0)
        let s_176_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_176_0);
        // S s_176_2: call _get_EDSCR_Type_SDD(s_176_1)
        let s_176_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_176_1);
        // S s_176_3: cast zx s_176_2 -> bv
        let s_176_3: Bits = Bits::new(s_176_2 as u128, 1u16);
        // C s_176_4: const #1u : u8
        let s_176_4: bool = true;
        // C s_176_5: cast zx s_176_4 -> bv
        let s_176_5: Bits = Bits::new(s_176_4 as u128, 1u16);
        // S s_176_6: cmp-eq s_176_3 s_176_5
        let s_176_6: bool = ((s_176_3) == (s_176_5));
        // D s_176_7: write-var gs#113211 <= s_176_6
        fn_state.gs_113211 = s_176_6;
        // N s_176_8: jump b173
        return block_173(state, tracer, fn_state);
    }
    fn block_177<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_177_0: read-var __MDCR_EL3_TPM:u8
        let s_177_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_177_1: cast zx s_177_0 -> bv
        let s_177_1: Bits = Bits::new(s_177_0 as u128, 1u16);
        // C s_177_2: const #1u : u8
        let s_177_2: bool = true;
        // C s_177_3: cast zx s_177_2 -> bv
        let s_177_3: Bits = Bits::new(s_177_2 as u128, 1u16);
        // D s_177_4: cmp-eq s_177_1 s_177_3
        let s_177_4: bool = ((s_177_1) == (s_177_3));
        // D s_177_5: write-var gs#113210 <= s_177_4
        fn_state.gs_113210 = s_177_4;
        // N s_177_6: jump b169
        return block_169(state, tracer, fn_state);
    }
    fn block_178<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_178_0: const #424u : u32
        let s_178_0: u32 = 424;
        // D s_178_1: read-reg s_178_0:u8
        let s_178_1: u8 = {
            let value = state.read_register::<u8>(s_178_0 as isize);
            tracer.read_register(s_178_0 as isize, value);
            value
        };
        // D s_178_2: call ELUsingAArch32(s_178_1)
        let s_178_2: bool = ELUsingAArch32(state, tracer, s_178_1);
        // D s_178_3: not s_178_2
        let s_178_3: bool = !s_178_2;
        // D s_178_4: write-var gs#113209 <= s_178_3
        fn_state.gs_113209 = s_178_3;
        // N s_178_5: jump b167
        return block_167(state, tracer, fn_state);
    }
    fn block_179<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_179_0: const #3u : u8
        let s_179_0: u8 = 3;
        // C s_179_1: cast zx s_179_0 -> bv
        let s_179_1: Bits = Bits::new(s_179_0 as u128, 8u16);
        // C s_179_2: cast zx s_179_1 -> i
        let s_179_2: i128 = (s_179_1.value() as i128);
        // C s_179_3: cast reint s_179_2 -> i64
        let s_179_3: i64 = (s_179_2 as i64);
        // C s_179_4: cast zx s_179_3 -> i
        let s_179_4: i128 = (i128::try_from(s_179_3).unwrap());
        // S s_179_5: call AArch32_TakeHypTrapException(s_179_4)
        let s_179_5: () = AArch32_TakeHypTrapException(state, tracer, s_179_4);
        // N s_179_6: return
        return;
    }
    fn block_180<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_180_0: read-var __HDCR_TPMCR:u8
        let s_180_0: bool = fn_state.u__HDCR_TPMCR;
        // D s_180_1: cast zx s_180_0 -> bv
        let s_180_1: Bits = Bits::new(s_180_0 as u128, 1u16);
        // C s_180_2: const #1u : u8
        let s_180_2: bool = true;
        // C s_180_3: cast zx s_180_2 -> bv
        let s_180_3: Bits = Bits::new(s_180_2 as u128, 1u16);
        // D s_180_4: cmp-eq s_180_1 s_180_3
        let s_180_4: bool = ((s_180_1) == (s_180_3));
        // D s_180_5: write-var gs#113208 <= s_180_4
        fn_state.gs_113208 = s_180_4;
        // N s_180_6: jump b164
        return block_164(state, tracer, fn_state);
    }
    fn block_181<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_181_0: const #432u : u32
        let s_181_0: u32 = 432;
        // D s_181_1: read-reg s_181_0:u8
        let s_181_1: u8 = {
            let value = state.read_register::<u8>(s_181_0 as isize);
            tracer.read_register(s_181_0 as isize, value);
            value
        };
        // D s_181_2: call ELUsingAArch32(s_181_1)
        let s_181_2: bool = ELUsingAArch32(state, tracer, s_181_1);
        // D s_181_3: write-var gs#113207 <= s_181_2
        fn_state.gs_113207 = s_181_2;
        // N s_181_4: jump b162
        return block_162(state, tracer, fn_state);
    }
    fn block_182<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_182_0: const #3u : u8
        let s_182_0: u8 = 3;
        // C s_182_1: cast zx s_182_0 -> bv
        let s_182_1: Bits = Bits::new(s_182_0 as u128, 8u16);
        // C s_182_2: cast zx s_182_1 -> i
        let s_182_2: i128 = (s_182_1.value() as i128);
        // C s_182_3: cast reint s_182_2 -> i64
        let s_182_3: i64 = (s_182_2 as i64);
        // C s_182_4: cast zx s_182_3 -> i
        let s_182_4: i128 = (i128::try_from(s_182_3).unwrap());
        // S s_182_5: call AArch32_TakeHypTrapException(s_182_4)
        let s_182_5: () = AArch32_TakeHypTrapException(state, tracer, s_182_4);
        // N s_182_6: return
        return;
    }
    fn block_183<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_183_0: read-var __HDCR_TPM:u8
        let s_183_0: bool = fn_state.u__HDCR_TPM;
        // D s_183_1: cast zx s_183_0 -> bv
        let s_183_1: Bits = Bits::new(s_183_0 as u128, 1u16);
        // C s_183_2: const #1u : u8
        let s_183_2: bool = true;
        // C s_183_3: cast zx s_183_2 -> bv
        let s_183_3: Bits = Bits::new(s_183_2 as u128, 1u16);
        // D s_183_4: cmp-eq s_183_1 s_183_3
        let s_183_4: bool = ((s_183_1) == (s_183_3));
        // D s_183_5: write-var gs#113206 <= s_183_4
        fn_state.gs_113206 = s_183_4;
        // N s_183_6: jump b159
        return block_159(state, tracer, fn_state);
    }
    fn block_184<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_184_0: const #432u : u32
        let s_184_0: u32 = 432;
        // D s_184_1: read-reg s_184_0:u8
        let s_184_1: u8 = {
            let value = state.read_register::<u8>(s_184_0 as isize);
            tracer.read_register(s_184_0 as isize, value);
            value
        };
        // D s_184_2: call ELUsingAArch32(s_184_1)
        let s_184_2: bool = ELUsingAArch32(state, tracer, s_184_1);
        // D s_184_3: write-var gs#113205 <= s_184_2
        fn_state.gs_113205 = s_184_2;
        // N s_184_4: jump b157
        return block_157(state, tracer, fn_state);
    }
    fn block_185<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_185_0: const #3u : u8
        let s_185_0: u8 = 3;
        // C s_185_1: cast zx s_185_0 -> bv
        let s_185_1: Bits = Bits::new(s_185_0 as u128, 8u16);
        // C s_185_2: cast zx s_185_1 -> i
        let s_185_2: i128 = (s_185_1.value() as i128);
        // C s_185_3: cast reint s_185_2 -> i64
        let s_185_3: i64 = (s_185_2 as i64);
        // C s_185_4: cast zx s_185_3 -> i
        let s_185_4: i128 = (i128::try_from(s_185_3).unwrap());
        // C s_185_5: const #432u : u32
        let s_185_5: u32 = 432;
        // D s_185_6: read-reg s_185_5:u8
        let s_185_6: u8 = {
            let value = state.read_register::<u8>(s_185_5 as isize);
            tracer.read_register(s_185_5 as isize, value);
            value
        };
        // D s_185_7: call AArch64_AArch32SystemAccessTrap(s_185_6, s_185_4)
        let s_185_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_185_6,
            s_185_4,
        );
        // N s_185_8: return
        return;
    }
    fn block_186<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_186_0: read-var __MDCR_EL2_TPMCR:u8
        let s_186_0: bool = fn_state.u__MDCR_EL2_TPMCR;
        // D s_186_1: cast zx s_186_0 -> bv
        let s_186_1: Bits = Bits::new(s_186_0 as u128, 1u16);
        // C s_186_2: const #1u : u8
        let s_186_2: bool = true;
        // C s_186_3: cast zx s_186_2 -> bv
        let s_186_3: Bits = Bits::new(s_186_2 as u128, 1u16);
        // D s_186_4: cmp-eq s_186_1 s_186_3
        let s_186_4: bool = ((s_186_1) == (s_186_3));
        // D s_186_5: write-var gs#113204 <= s_186_4
        fn_state.gs_113204 = s_186_4;
        // N s_186_6: jump b154
        return block_154(state, tracer, fn_state);
    }
    fn block_187<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_187_0: const #432u : u32
        let s_187_0: u32 = 432;
        // D s_187_1: read-reg s_187_0:u8
        let s_187_1: u8 = {
            let value = state.read_register::<u8>(s_187_0 as isize);
            tracer.read_register(s_187_0 as isize, value);
            value
        };
        // D s_187_2: call ELUsingAArch32(s_187_1)
        let s_187_2: bool = ELUsingAArch32(state, tracer, s_187_1);
        // D s_187_3: not s_187_2
        let s_187_3: bool = !s_187_2;
        // D s_187_4: write-var gs#113203 <= s_187_3
        fn_state.gs_113203 = s_187_3;
        // N s_187_5: jump b152
        return block_152(state, tracer, fn_state);
    }
    fn block_188<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_188_0: const #3u : u8
        let s_188_0: u8 = 3;
        // C s_188_1: cast zx s_188_0 -> bv
        let s_188_1: Bits = Bits::new(s_188_0 as u128, 8u16);
        // C s_188_2: cast zx s_188_1 -> i
        let s_188_2: i128 = (s_188_1.value() as i128);
        // C s_188_3: cast reint s_188_2 -> i64
        let s_188_3: i64 = (s_188_2 as i64);
        // C s_188_4: cast zx s_188_3 -> i
        let s_188_4: i128 = (i128::try_from(s_188_3).unwrap());
        // C s_188_5: const #432u : u32
        let s_188_5: u32 = 432;
        // D s_188_6: read-reg s_188_5:u8
        let s_188_6: u8 = {
            let value = state.read_register::<u8>(s_188_5 as isize);
            tracer.read_register(s_188_5 as isize, value);
            value
        };
        // D s_188_7: call AArch64_AArch32SystemAccessTrap(s_188_6, s_188_4)
        let s_188_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_188_6,
            s_188_4,
        );
        // N s_188_8: return
        return;
    }
    fn block_189<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_189_0: read-var __MDCR_EL2_TPM:u8
        let s_189_0: bool = fn_state.u__MDCR_EL2_TPM;
        // D s_189_1: cast zx s_189_0 -> bv
        let s_189_1: Bits = Bits::new(s_189_0 as u128, 1u16);
        // C s_189_2: const #1u : u8
        let s_189_2: bool = true;
        // C s_189_3: cast zx s_189_2 -> bv
        let s_189_3: Bits = Bits::new(s_189_2 as u128, 1u16);
        // D s_189_4: cmp-eq s_189_1 s_189_3
        let s_189_4: bool = ((s_189_1) == (s_189_3));
        // D s_189_5: write-var gs#113202 <= s_189_4
        fn_state.gs_113202 = s_189_4;
        // N s_189_6: jump b149
        return block_149(state, tracer, fn_state);
    }
    fn block_190<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_190_0: const #432u : u32
        let s_190_0: u32 = 432;
        // D s_190_1: read-reg s_190_0:u8
        let s_190_1: u8 = {
            let value = state.read_register::<u8>(s_190_0 as isize);
            tracer.read_register(s_190_0 as isize, value);
            value
        };
        // D s_190_2: call ELUsingAArch32(s_190_1)
        let s_190_2: bool = ELUsingAArch32(state, tracer, s_190_1);
        // D s_190_3: not s_190_2
        let s_190_3: bool = !s_190_2;
        // D s_190_4: write-var gs#113201 <= s_190_3
        fn_state.gs_113201 = s_190_3;
        // N s_190_5: jump b147
        return block_147(state, tracer, fn_state);
    }
    fn block_191<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_191_0: const #3u : u8
        let s_191_0: u8 = 3;
        // C s_191_1: cast zx s_191_0 -> bv
        let s_191_1: Bits = Bits::new(s_191_0 as u128, 8u16);
        // C s_191_2: cast zx s_191_1 -> i
        let s_191_2: i128 = (s_191_1.value() as i128);
        // C s_191_3: cast reint s_191_2 -> i64
        let s_191_3: i64 = (s_191_2 as i64);
        // C s_191_4: cast zx s_191_3 -> i
        let s_191_4: i128 = (i128::try_from(s_191_3).unwrap());
        // S s_191_5: call AArch32_TakeHypTrapException(s_191_4)
        let s_191_5: () = AArch32_TakeHypTrapException(state, tracer, s_191_4);
        // N s_191_6: return
        return;
    }
    fn block_192<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_192_0: read-var __HSTR_T9:u8
        let s_192_0: bool = fn_state.u__HSTR_T9;
        // D s_192_1: cast zx s_192_0 -> bv
        let s_192_1: Bits = Bits::new(s_192_0 as u128, 1u16);
        // C s_192_2: const #1u : u8
        let s_192_2: bool = true;
        // C s_192_3: cast zx s_192_2 -> bv
        let s_192_3: Bits = Bits::new(s_192_2 as u128, 1u16);
        // D s_192_4: cmp-eq s_192_1 s_192_3
        let s_192_4: bool = ((s_192_1) == (s_192_3));
        // D s_192_5: write-var gs#113200 <= s_192_4
        fn_state.gs_113200 = s_192_4;
        // N s_192_6: jump b144
        return block_144(state, tracer, fn_state);
    }
    fn block_193<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_193_0: const #432u : u32
        let s_193_0: u32 = 432;
        // D s_193_1: read-reg s_193_0:u8
        let s_193_1: u8 = {
            let value = state.read_register::<u8>(s_193_0 as isize);
            tracer.read_register(s_193_0 as isize, value);
            value
        };
        // D s_193_2: call ELUsingAArch32(s_193_1)
        let s_193_2: bool = ELUsingAArch32(state, tracer, s_193_1);
        // D s_193_3: write-var gs#113199 <= s_193_2
        fn_state.gs_113199 = s_193_2;
        // N s_193_4: jump b142
        return block_142(state, tracer, fn_state);
    }
    fn block_194<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_194_0: const #3u : u8
        let s_194_0: u8 = 3;
        // C s_194_1: cast zx s_194_0 -> bv
        let s_194_1: Bits = Bits::new(s_194_0 as u128, 8u16);
        // C s_194_2: cast zx s_194_1 -> i
        let s_194_2: i128 = (s_194_1.value() as i128);
        // C s_194_3: cast reint s_194_2 -> i64
        let s_194_3: i64 = (s_194_2 as i64);
        // C s_194_4: cast zx s_194_3 -> i
        let s_194_4: i128 = (i128::try_from(s_194_3).unwrap());
        // C s_194_5: const #432u : u32
        let s_194_5: u32 = 432;
        // D s_194_6: read-reg s_194_5:u8
        let s_194_6: u8 = {
            let value = state.read_register::<u8>(s_194_5 as isize);
            tracer.read_register(s_194_5 as isize, value);
            value
        };
        // D s_194_7: call AArch64_AArch32SystemAccessTrap(s_194_6, s_194_4)
        let s_194_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_194_6,
            s_194_4,
        );
        // N s_194_8: return
        return;
    }
    fn block_195<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_195_0: read-var __HSTR_EL2_T9:u8
        let s_195_0: bool = fn_state.u__HSTR_EL2_T9;
        // D s_195_1: cast zx s_195_0 -> bv
        let s_195_1: Bits = Bits::new(s_195_0 as u128, 1u16);
        // C s_195_2: const #1u : u8
        let s_195_2: bool = true;
        // C s_195_3: cast zx s_195_2 -> bv
        let s_195_3: Bits = Bits::new(s_195_2 as u128, 1u16);
        // D s_195_4: cmp-eq s_195_1 s_195_3
        let s_195_4: bool = ((s_195_1) == (s_195_3));
        // D s_195_5: write-var gs#113198 <= s_195_4
        fn_state.gs_113198 = s_195_4;
        // N s_195_6: jump b139
        return block_139(state, tracer, fn_state);
    }
    fn block_196<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_196_0: const #102552u : u32
        let s_196_0: u32 = 102552;
        // D s_196_1: read-reg s_196_0:struct
        let s_196_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_196_0 as isize);
            tracer.read_register(s_196_0 as isize, value);
            value
        };
        // D s_196_2: call _get_HCR_EL2_Type_E2H(s_196_1)
        let s_196_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_196_1);
        // C s_196_3: const #102552u : u32
        let s_196_3: u32 = 102552;
        // D s_196_4: read-reg s_196_3:struct
        let s_196_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_196_3 as isize);
            tracer.read_register(s_196_3 as isize, value);
            value
        };
        // D s_196_5: call _get_HCR_EL2_Type_TGE(s_196_4)
        let s_196_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_196_4);
        // D s_196_6: cast zx s_196_2 -> bv
        let s_196_6: Bits = Bits::new(s_196_2 as u128, 1u16);
        // D s_196_7: cast zx s_196_5 -> bv
        let s_196_7: Bits = Bits::new(s_196_5 as u128, 1u16);
        // D s_196_8: cast reint s_196_6 -> u128
        let s_196_8: u128 = (s_196_6.value() as u128);
        // D s_196_9: size-of s_196_6
        let s_196_9: u16 = s_196_6.length();
        // D s_196_10: cast reint s_196_7 -> u128
        let s_196_10: u128 = (s_196_7.value() as u128);
        // D s_196_11: size-of s_196_7
        let s_196_11: u16 = s_196_7.length();
        // D s_196_12: lsl s_196_8 s_196_11
        let s_196_12: u128 = s_196_8 << s_196_11;
        // D s_196_13: or s_196_12 s_196_10
        let s_196_13: u128 = ((s_196_12) | (s_196_10));
        // D s_196_14: add s_196_9 s_196_11
        let s_196_14: u16 = (s_196_9 + s_196_11);
        // D s_196_15: create-bits s_196_13 s_196_14
        let s_196_15: Bits = Bits::new(s_196_13, s_196_14);
        // D s_196_16: cast reint s_196_15 -> u8
        let s_196_16: u8 = (s_196_15.value() as u8);
        // D s_196_17: cast zx s_196_16 -> bv
        let s_196_17: Bits = Bits::new(s_196_16 as u128, 2u16);
        // C s_196_18: const #3u : u8
        let s_196_18: u8 = 3;
        // C s_196_19: cast zx s_196_18 -> bv
        let s_196_19: Bits = Bits::new(s_196_18 as u128, 2u16);
        // D s_196_20: cmp-ne s_196_17 s_196_19
        let s_196_20: bool = ((s_196_17) != (s_196_19));
        // D s_196_21: write-var gs#113197 <= s_196_20
        fn_state.gs_113197 = s_196_20;
        // N s_196_22: jump b137
        return block_137(state, tracer, fn_state);
    }
    fn block_197<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_197_0: const #432u : u32
        let s_197_0: u32 = 432;
        // D s_197_1: read-reg s_197_0:u8
        let s_197_1: u8 = {
            let value = state.read_register::<u8>(s_197_0 as isize);
            tracer.read_register(s_197_0 as isize, value);
            value
        };
        // D s_197_2: call ELUsingAArch32(s_197_1)
        let s_197_2: bool = ELUsingAArch32(state, tracer, s_197_1);
        // D s_197_3: not s_197_2
        let s_197_3: bool = !s_197_2;
        // D s_197_4: write-var gs#113196 <= s_197_3
        fn_state.gs_113196 = s_197_3;
        // N s_197_5: jump b135
        return block_135(state, tracer, fn_state);
    }
    fn block_198<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_198_0: const #() : ()
        let s_198_0: () = ();
        // S s_198_1: call EL2Enabled(s_198_0)
        let s_198_1: bool = EL2Enabled(state, tracer, s_198_0);
        // N s_198_2: branch s_198_1 b214 b199
        if s_198_1 {
            return block_214(state, tracer, fn_state);
        } else {
            return block_199(state, tracer, fn_state);
        };
    }
    fn block_199<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_199_0: const #0u : u8
        let s_199_0: bool = false;
        // D s_199_1: write-var gs#113212 <= s_199_0
        fn_state.gs_113212 = s_199_0;
        // N s_199_2: jump b200
        return block_200(state, tracer, fn_state);
    }
    fn block_200<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_200_0: read-var gs#113212:u8
        let s_200_0: bool = fn_state.gs_113212;
        // N s_200_1: branch s_200_0 b213 b201
        if s_200_0 {
            return block_213(state, tracer, fn_state);
        } else {
            return block_201(state, tracer, fn_state);
        };
    }
    fn block_201<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_201_0: const #0u : u8
        let s_201_0: bool = false;
        // D s_201_1: write-var gs#113213 <= s_201_0
        fn_state.gs_113213 = s_201_0;
        // N s_201_2: jump b202
        return block_202(state, tracer, fn_state);
    }
    fn block_202<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_202_0: read-var gs#113213:u8
        let s_202_0: bool = fn_state.gs_113213;
        // N s_202_1: branch s_202_0 b212 b203
        if s_202_0 {
            return block_212(state, tracer, fn_state);
        } else {
            return block_203(state, tracer, fn_state);
        };
    }
    fn block_203<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_203_0: const #() : ()
        let s_203_0: () = ();
        // S s_203_1: call EL2Enabled(s_203_0)
        let s_203_1: bool = EL2Enabled(state, tracer, s_203_0);
        // N s_203_2: branch s_203_1 b211 b204
        if s_203_1 {
            return block_211(state, tracer, fn_state);
        } else {
            return block_204(state, tracer, fn_state);
        };
    }
    fn block_204<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_204_0: const #0u : u8
        let s_204_0: bool = false;
        // D s_204_1: write-var gs#113214 <= s_204_0
        fn_state.gs_113214 = s_204_0;
        // N s_204_2: jump b205
        return block_205(state, tracer, fn_state);
    }
    fn block_205<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_205_0: read-var gs#113214:u8
        let s_205_0: bool = fn_state.gs_113214;
        // N s_205_1: branch s_205_0 b210 b206
        if s_205_0 {
            return block_210(state, tracer, fn_state);
        } else {
            return block_206(state, tracer, fn_state);
        };
    }
    fn block_206<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_206_0: const #0u : u8
        let s_206_0: bool = false;
        // D s_206_1: write-var gs#113215 <= s_206_0
        fn_state.gs_113215 = s_206_0;
        // N s_206_2: jump b207
        return block_207(state, tracer, fn_state);
    }
    fn block_207<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_207_0: read-var gs#113215:u8
        let s_207_0: bool = fn_state.gs_113215;
        // N s_207_1: branch s_207_0 b209 b208
        if s_207_0 {
            return block_209(state, tracer, fn_state);
        } else {
            return block_208(state, tracer, fn_state);
        };
    }
    fn block_208<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_208_0: panic
        panic!("{:?}", ());
        // N s_208_1: return
        return;
    }
    fn block_209<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_209_0: const #0u : u8
        let s_209_0: u8 = 0;
        // C s_209_1: cast zx s_209_0 -> bv
        let s_209_1: Bits = Bits::new(s_209_0 as u128, 8u16);
        // C s_209_2: cast zx s_209_1 -> i
        let s_209_2: i128 = (s_209_1.value() as i128);
        // C s_209_3: cast reint s_209_2 -> i64
        let s_209_3: i64 = (s_209_2 as i64);
        // C s_209_4: cast zx s_209_3 -> i
        let s_209_4: i128 = (i128::try_from(s_209_3).unwrap());
        // S s_209_5: call AArch32_TakeHypTrapException(s_209_4)
        let s_209_5: () = AArch32_TakeHypTrapException(state, tracer, s_209_4);
        // N s_209_6: return
        return;
    }
    fn block_210<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_210_0: read-var __HCR_TGE:u8
        let s_210_0: bool = fn_state.u__HCR_TGE;
        // D s_210_1: cast zx s_210_0 -> bv
        let s_210_1: Bits = Bits::new(s_210_0 as u128, 1u16);
        // C s_210_2: const #1u : u8
        let s_210_2: bool = true;
        // C s_210_3: cast zx s_210_2 -> bv
        let s_210_3: Bits = Bits::new(s_210_2 as u128, 1u16);
        // D s_210_4: cmp-eq s_210_1 s_210_3
        let s_210_4: bool = ((s_210_1) == (s_210_3));
        // D s_210_5: write-var gs#113215 <= s_210_4
        fn_state.gs_113215 = s_210_4;
        // N s_210_6: jump b207
        return block_207(state, tracer, fn_state);
    }
    fn block_211<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_211_0: const #432u : u32
        let s_211_0: u32 = 432;
        // D s_211_1: read-reg s_211_0:u8
        let s_211_1: u8 = {
            let value = state.read_register::<u8>(s_211_0 as isize);
            tracer.read_register(s_211_0 as isize, value);
            value
        };
        // D s_211_2: call ELUsingAArch32(s_211_1)
        let s_211_2: bool = ELUsingAArch32(state, tracer, s_211_1);
        // D s_211_3: write-var gs#113214 <= s_211_2
        fn_state.gs_113214 = s_211_2;
        // N s_211_4: jump b205
        return block_205(state, tracer, fn_state);
    }
    fn block_212<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_212_0: const #3u : u8
        let s_212_0: u8 = 3;
        // C s_212_1: cast zx s_212_0 -> bv
        let s_212_1: Bits = Bits::new(s_212_0 as u128, 8u16);
        // C s_212_2: cast zx s_212_1 -> i
        let s_212_2: i128 = (s_212_1.value() as i128);
        // C s_212_3: cast reint s_212_2 -> i64
        let s_212_3: i64 = (s_212_2 as i64);
        // C s_212_4: cast zx s_212_3 -> i
        let s_212_4: i128 = (i128::try_from(s_212_3).unwrap());
        // C s_212_5: const #432u : u32
        let s_212_5: u32 = 432;
        // D s_212_6: read-reg s_212_5:u8
        let s_212_6: u8 = {
            let value = state.read_register::<u8>(s_212_5 as isize);
            tracer.read_register(s_212_5 as isize, value);
            value
        };
        // D s_212_7: call AArch64_AArch32SystemAccessTrap(s_212_6, s_212_4)
        let s_212_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_212_6,
            s_212_4,
        );
        // N s_212_8: return
        return;
    }
    fn block_213<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_213_0: read-var __HCR_EL2_TGE:u8
        let s_213_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_213_1: cast zx s_213_0 -> bv
        let s_213_1: Bits = Bits::new(s_213_0 as u128, 1u16);
        // C s_213_2: const #1u : u8
        let s_213_2: bool = true;
        // C s_213_3: cast zx s_213_2 -> bv
        let s_213_3: Bits = Bits::new(s_213_2 as u128, 1u16);
        // D s_213_4: cmp-eq s_213_1 s_213_3
        let s_213_4: bool = ((s_213_1) == (s_213_3));
        // D s_213_5: write-var gs#113213 <= s_213_4
        fn_state.gs_113213 = s_213_4;
        // N s_213_6: jump b202
        return block_202(state, tracer, fn_state);
    }
    fn block_214<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_214_0: const #432u : u32
        let s_214_0: u32 = 432;
        // D s_214_1: read-reg s_214_0:u8
        let s_214_1: u8 = {
            let value = state.read_register::<u8>(s_214_0 as isize);
            tracer.read_register(s_214_0 as isize, value);
            value
        };
        // D s_214_2: call ELUsingAArch32(s_214_1)
        let s_214_2: bool = ELUsingAArch32(state, tracer, s_214_1);
        // D s_214_3: not s_214_2
        let s_214_3: bool = !s_214_2;
        // D s_214_4: write-var gs#113212 <= s_214_3
        fn_state.gs_113212 = s_214_3;
        // N s_214_5: jump b200
        return block_200(state, tracer, fn_state);
    }
    fn block_215<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_215_0: read-var __PMUSERENR_EN:u8
        let s_215_0: bool = fn_state.u__PMUSERENR_EN;
        // D s_215_1: cast zx s_215_0 -> bv
        let s_215_1: Bits = Bits::new(s_215_0 as u128, 1u16);
        // C s_215_2: const #0u : u8
        let s_215_2: bool = false;
        // C s_215_3: cast zx s_215_2 -> bv
        let s_215_3: Bits = Bits::new(s_215_2 as u128, 1u16);
        // D s_215_4: cmp-eq s_215_1 s_215_3
        let s_215_4: bool = ((s_215_1) == (s_215_3));
        // D s_215_5: write-var gs#113195 <= s_215_4
        fn_state.gs_113195 = s_215_4;
        // N s_215_6: jump b132
        return block_132(state, tracer, fn_state);
    }
    fn block_216<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_216_0: const #() : ()
        let s_216_0: () = ();
        // S s_216_1: call EL2Enabled(s_216_0)
        let s_216_1: bool = EL2Enabled(state, tracer, s_216_0);
        // N s_216_2: branch s_216_1 b224 b217
        if s_216_1 {
            return block_224(state, tracer, fn_state);
        } else {
            return block_217(state, tracer, fn_state);
        };
    }
    fn block_217<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_217_0: const #0u : u8
        let s_217_0: bool = false;
        // D s_217_1: write-var gs#113216 <= s_217_0
        fn_state.gs_113216 = s_217_0;
        // N s_217_2: jump b218
        return block_218(state, tracer, fn_state);
    }
    fn block_218<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_218_0: read-var gs#113216:u8
        let s_218_0: bool = fn_state.gs_113216;
        // N s_218_1: branch s_218_0 b223 b219
        if s_218_0 {
            return block_223(state, tracer, fn_state);
        } else {
            return block_219(state, tracer, fn_state);
        };
    }
    fn block_219<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_219_0: const #0u : u8
        let s_219_0: bool = false;
        // D s_219_1: write-var gs#113217 <= s_219_0
        fn_state.gs_113217 = s_219_0;
        // N s_219_2: jump b220
        return block_220(state, tracer, fn_state);
    }
    fn block_220<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_220_0: read-var gs#113217:u8
        let s_220_0: bool = fn_state.gs_113217;
        // N s_220_1: branch s_220_0 b222 b221
        if s_220_0 {
            return block_222(state, tracer, fn_state);
        } else {
            return block_221(state, tracer, fn_state);
        };
    }
    fn block_221<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_221_0: const #3u : u8
        let s_221_0: u8 = 3;
        // C s_221_1: cast zx s_221_0 -> bv
        let s_221_1: Bits = Bits::new(s_221_0 as u128, 8u16);
        // C s_221_2: cast zx s_221_1 -> i
        let s_221_2: i128 = (s_221_1.value() as i128);
        // C s_221_3: cast reint s_221_2 -> i64
        let s_221_3: i64 = (s_221_2 as i64);
        // C s_221_4: cast zx s_221_3 -> i
        let s_221_4: i128 = (i128::try_from(s_221_3).unwrap());
        // C s_221_5: const #440u : u32
        let s_221_5: u32 = 440;
        // D s_221_6: read-reg s_221_5:u8
        let s_221_6: u8 = {
            let value = state.read_register::<u8>(s_221_5 as isize);
            tracer.read_register(s_221_5 as isize, value);
            value
        };
        // D s_221_7: call AArch64_AArch32SystemAccessTrap(s_221_6, s_221_4)
        let s_221_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_221_6,
            s_221_4,
        );
        // N s_221_8: return
        return;
    }
    fn block_222<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_222_0: const #3u : u8
        let s_222_0: u8 = 3;
        // C s_222_1: cast zx s_222_0 -> bv
        let s_222_1: Bits = Bits::new(s_222_0 as u128, 8u16);
        // C s_222_2: cast zx s_222_1 -> i
        let s_222_2: i128 = (s_222_1.value() as i128);
        // C s_222_3: cast reint s_222_2 -> i64
        let s_222_3: i64 = (s_222_2 as i64);
        // C s_222_4: cast zx s_222_3 -> i
        let s_222_4: i128 = (i128::try_from(s_222_3).unwrap());
        // C s_222_5: const #432u : u32
        let s_222_5: u32 = 432;
        // D s_222_6: read-reg s_222_5:u8
        let s_222_6: u8 = {
            let value = state.read_register::<u8>(s_222_5 as isize);
            tracer.read_register(s_222_5 as isize, value);
            value
        };
        // D s_222_7: call AArch64_AArch32SystemAccessTrap(s_222_6, s_222_4)
        let s_222_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_222_6,
            s_222_4,
        );
        // N s_222_8: return
        return;
    }
    fn block_223<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_223_0: read-var __HCR_EL2_TGE:u8
        let s_223_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_223_1: cast zx s_223_0 -> bv
        let s_223_1: Bits = Bits::new(s_223_0 as u128, 1u16);
        // C s_223_2: const #1u : u8
        let s_223_2: bool = true;
        // C s_223_3: cast zx s_223_2 -> bv
        let s_223_3: Bits = Bits::new(s_223_2 as u128, 1u16);
        // D s_223_4: cmp-eq s_223_1 s_223_3
        let s_223_4: bool = ((s_223_1) == (s_223_3));
        // D s_223_5: write-var gs#113217 <= s_223_4
        fn_state.gs_113217 = s_223_4;
        // N s_223_6: jump b220
        return block_220(state, tracer, fn_state);
    }
    fn block_224<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_224_0: const #432u : u32
        let s_224_0: u32 = 432;
        // D s_224_1: read-reg s_224_0:u8
        let s_224_1: u8 = {
            let value = state.read_register::<u8>(s_224_0 as isize);
            tracer.read_register(s_224_0 as isize, value);
            value
        };
        // D s_224_2: call ELUsingAArch32(s_224_1)
        let s_224_2: bool = ELUsingAArch32(state, tracer, s_224_1);
        // D s_224_3: not s_224_2
        let s_224_3: bool = !s_224_2;
        // D s_224_4: write-var gs#113216 <= s_224_3
        fn_state.gs_113216 = s_224_3;
        // N s_224_5: jump b218
        return block_218(state, tracer, fn_state);
    }
    fn block_225<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_225_0: const #204u : u32
        let s_225_0: u32 = 204;
        // S s_225_1: call IsFeatureImplemented(s_225_0)
        let s_225_1: bool = IsFeatureImplemented(state, tracer, s_225_0);
        // N s_225_2: branch s_225_1 b234 b226
        if s_225_1 {
            return block_234(state, tracer, fn_state);
        } else {
            return block_226(state, tracer, fn_state);
        };
    }
    fn block_226<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_226_0: const #0u : u8
        let s_226_0: bool = false;
        // D s_226_1: write-var gs#113191 <= s_226_0
        fn_state.gs_113191 = s_226_0;
        // N s_226_2: jump b227
        return block_227(state, tracer, fn_state);
    }
    fn block_227<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_227_0: read-var gs#113191:u8
        let s_227_0: bool = fn_state.gs_113191;
        // N s_227_1: branch s_227_0 b233 b228
        if s_227_0 {
            return block_233(state, tracer, fn_state);
        } else {
            return block_228(state, tracer, fn_state);
        };
    }
    fn block_228<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_228_0: const #204u : u32
        let s_228_0: u32 = 204;
        // S s_228_1: call IsFeatureImplemented(s_228_0)
        let s_228_1: bool = IsFeatureImplemented(state, tracer, s_228_0);
        // S s_228_2: not s_228_1
        let s_228_2: bool = !s_228_1;
        // N s_228_3: branch s_228_2 b232 b229
        if s_228_2 {
            return block_232(state, tracer, fn_state);
        } else {
            return block_229(state, tracer, fn_state);
        };
    }
    fn block_229<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_229_0: const #0u : u8
        let s_229_0: bool = false;
        // D s_229_1: write-var gs#113192 <= s_229_0
        fn_state.gs_113192 = s_229_0;
        // N s_229_2: jump b230
        return block_230(state, tracer, fn_state);
    }
    fn block_230<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_230_0: read-var gs#113192:u8
        let s_230_0: bool = fn_state.gs_113192;
        // D s_230_1: write-var gs#113193 <= s_230_0
        fn_state.gs_113193 = s_230_0;
        // N s_230_2: jump b231
        return block_231(state, tracer, fn_state);
    }
    fn block_231<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_231_0: read-var gs#113193:u8
        let s_231_0: bool = fn_state.gs_113193;
        // D s_231_1: write-var gs#113194 <= s_231_0
        fn_state.gs_113194 = s_231_0;
        // N s_231_2: jump b129
        return block_129(state, tracer, fn_state);
    }
    fn block_232<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_232_0: read-var __PMUSERENR_EL0_EN:u8
        let s_232_0: bool = fn_state.u__PMUSERENR_EL0_EN;
        // D s_232_1: cast zx s_232_0 -> bv
        let s_232_1: Bits = Bits::new(s_232_0 as u128, 1u16);
        // C s_232_2: const #0u : u8
        let s_232_2: bool = false;
        // C s_232_3: cast zx s_232_2 -> bv
        let s_232_3: Bits = Bits::new(s_232_2 as u128, 1u16);
        // D s_232_4: cmp-eq s_232_1 s_232_3
        let s_232_4: bool = ((s_232_1) == (s_232_3));
        // D s_232_5: write-var gs#113192 <= s_232_4
        fn_state.gs_113192 = s_232_4;
        // N s_232_6: jump b230
        return block_230(state, tracer, fn_state);
    }
    fn block_233<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_233_0: const #1u : u8
        let s_233_0: bool = true;
        // D s_233_1: write-var gs#113193 <= s_233_0
        fn_state.gs_113193 = s_233_0;
        // N s_233_2: jump b231
        return block_231(state, tracer, fn_state);
    }
    fn block_234<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_234_0: const #102624u : u32
        let s_234_0: u32 = 102624;
        // D s_234_1: read-reg s_234_0:struct
        let s_234_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_234_0 as isize);
            tracer.read_register(s_234_0 as isize, value);
            value
        };
        // D s_234_2: call _get_PMUSERENR_EL0_Type_UEN(s_234_1)
        let s_234_2: bool = u_get_PMUSERENR_EL0_Type_UEN(state, tracer, s_234_1);
        // C s_234_3: const #102624u : u32
        let s_234_3: u32 = 102624;
        // D s_234_4: read-reg s_234_3:struct
        let s_234_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_234_3 as isize);
            tracer.read_register(s_234_3 as isize, value);
            value
        };
        // D s_234_5: call _get_PMUSERENR_EL0_Type_EN(s_234_4)
        let s_234_5: bool = u_get_PMUSERENR_EL0_Type_EN(state, tracer, s_234_4);
        // D s_234_6: cast zx s_234_2 -> bv
        let s_234_6: Bits = Bits::new(s_234_2 as u128, 1u16);
        // D s_234_7: cast zx s_234_5 -> bv
        let s_234_7: Bits = Bits::new(s_234_5 as u128, 1u16);
        // D s_234_8: cast reint s_234_6 -> u128
        let s_234_8: u128 = (s_234_6.value() as u128);
        // D s_234_9: size-of s_234_6
        let s_234_9: u16 = s_234_6.length();
        // D s_234_10: cast reint s_234_7 -> u128
        let s_234_10: u128 = (s_234_7.value() as u128);
        // D s_234_11: size-of s_234_7
        let s_234_11: u16 = s_234_7.length();
        // D s_234_12: lsl s_234_8 s_234_11
        let s_234_12: u128 = s_234_8 << s_234_11;
        // D s_234_13: or s_234_12 s_234_10
        let s_234_13: u128 = ((s_234_12) | (s_234_10));
        // D s_234_14: add s_234_9 s_234_11
        let s_234_14: u16 = (s_234_9 + s_234_11);
        // D s_234_15: create-bits s_234_13 s_234_14
        let s_234_15: Bits = Bits::new(s_234_13, s_234_14);
        // D s_234_16: cast reint s_234_15 -> u8
        let s_234_16: u8 = (s_234_15.value() as u8);
        // D s_234_17: cast zx s_234_16 -> bv
        let s_234_17: Bits = Bits::new(s_234_16 as u128, 2u16);
        // C s_234_18: const #1u : u8
        let s_234_18: u8 = 1;
        // C s_234_19: cast zx s_234_18 -> bv
        let s_234_19: Bits = Bits::new(s_234_18 as u128, 2u16);
        // D s_234_20: cmp-ne s_234_17 s_234_19
        let s_234_20: bool = ((s_234_17) != (s_234_19));
        // D s_234_21: write-var gs#113191 <= s_234_20
        fn_state.gs_113191 = s_234_20;
        // N s_234_22: jump b227
        return block_227(state, tracer, fn_state);
    }
    fn block_235<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_235_0: panic
        panic!("{:?}", ());
        // N s_235_1: return
        return;
    }
    fn block_236<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_236_0: read-var __MDCR_EL3_TPM:u8
        let s_236_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_236_1: cast zx s_236_0 -> bv
        let s_236_1: Bits = Bits::new(s_236_0 as u128, 1u16);
        // C s_236_2: const #1u : u8
        let s_236_2: bool = true;
        // C s_236_3: cast zx s_236_2 -> bv
        let s_236_3: Bits = Bits::new(s_236_2 as u128, 1u16);
        // D s_236_4: cmp-eq s_236_1 s_236_3
        let s_236_4: bool = ((s_236_1) == (s_236_3));
        // D s_236_5: write-var gs#113190 <= s_236_4
        fn_state.gs_113190 = s_236_4;
        // N s_236_6: jump b126
        return block_126(state, tracer, fn_state);
    }
    fn block_237<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_237_0: const #424u : u32
        let s_237_0: u32 = 424;
        // D s_237_1: read-reg s_237_0:u8
        let s_237_1: u8 = {
            let value = state.read_register::<u8>(s_237_0 as isize);
            tracer.read_register(s_237_0 as isize, value);
            value
        };
        // D s_237_2: call ELUsingAArch32(s_237_1)
        let s_237_2: bool = ELUsingAArch32(state, tracer, s_237_1);
        // D s_237_3: not s_237_2
        let s_237_3: bool = !s_237_2;
        // D s_237_4: write-var gs#113189 <= s_237_3
        fn_state.gs_113189 = s_237_3;
        // N s_237_5: jump b124
        return block_124(state, tracer, fn_state);
    }
    fn block_238<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_238_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_238_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_238_1: call __IMPDEF_boolean(s_238_0)
        let s_238_1: bool = u__IMPDEF_boolean(state, tracer, s_238_0);
        // D s_238_2: write-var gs#113188 <= s_238_1
        fn_state.gs_113188 = s_238_1;
        // N s_238_3: jump b122
        return block_122(state, tracer, fn_state);
    }
    fn block_239<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_239_0: const #() : ()
        let s_239_0: () = ();
        // S s_239_1: call EDSCR_read(s_239_0)
        let s_239_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_239_0);
        // S s_239_2: call _get_EDSCR_Type_SDD(s_239_1)
        let s_239_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_239_1);
        // S s_239_3: cast zx s_239_2 -> bv
        let s_239_3: Bits = Bits::new(s_239_2 as u128, 1u16);
        // C s_239_4: const #1u : u8
        let s_239_4: bool = true;
        // C s_239_5: cast zx s_239_4 -> bv
        let s_239_5: Bits = Bits::new(s_239_4 as u128, 1u16);
        // S s_239_6: cmp-eq s_239_3 s_239_5
        let s_239_6: bool = ((s_239_3) == (s_239_5));
        // D s_239_7: write-var gs#113187 <= s_239_6
        fn_state.gs_113187 = s_239_6;
        // N s_239_8: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_240<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_240_0: const #424u : u32
        let s_240_0: u32 = 424;
        // D s_240_1: read-reg s_240_0:u8
        let s_240_1: u8 = {
            let value = state.read_register::<u8>(s_240_0 as isize);
            tracer.read_register(s_240_0 as isize, value);
            value
        };
        // C s_240_2: const #2u : u8
        let s_240_2: u8 = 2;
        // D s_240_3: cmp-lt s_240_1 s_240_2
        let s_240_3: bool = ((s_240_1) < (s_240_2));
        // D s_240_4: write-var gs#113186 <= s_240_3
        fn_state.gs_113186 = s_240_3;
        // N s_240_5: jump b118
        return block_118(state, tracer, fn_state);
    }
}
