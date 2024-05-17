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
use AArch32_TakeMonitorTrapException::*;
use EDSCR_read::*;
use DBGDSCRext_read::*;
use Halted::*;
use u_get_HDCR_Type_TDE::*;
use u_get_MDCR_EL2_Type_TDCC::*;
use u__get_DBGDSCRint::*;
use u_get_DBGDSCRext_Type_UDCCdis::*;
use u_get_SDCR_Type_TDCC::*;
use u_get_MDSCR_EL1_Type_TDCC::*;
use u_get_EDSCR_Type_SDD::*;
use EL2Enabled::*;
use u_get_HCR_EL2_Type_TGE::*;
use u_get_HDCR_Type_TDCC::*;
use HCR_read::*;
use HDCR_read::*;
use ConstrainUnpredictableBool::*;
use u_get_MDCR_EL3_Type_TDCC::*;
use u__IMPDEF_boolean::*;
use AArch64_AArch32SystemAccessTrap::*;
use u_get_HDCR_Type_TDA::*;
use u_get_MDCR_EL2_Type_TDE::*;
use u_get_HCR_Type_TGE::*;
use u_get_MDCR_EL3_Type_TDA::*;
use R_set::*;
use ELUsingAArch32::*;
use DBGDSCRint_read::*;
use u_get_MDCR_EL2_Type_TDA::*;
use AArch32_TakeHypTrapException::*;
use common::*;
pub fn DBGDSCRint_SysRegRead32_4d6240b644e5e8d8<T: Tracer>(
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
        gs_109077: bool,
        gs_109107: bool,
        gs_109038: bool,
        gs_109061: bool,
        gs_109128: bool,
        gs_109160: bool,
        gs_109081: bool,
        gs_109133: bool,
        gs_109122: bool,
        u__SDCR_TDCC: bool,
        gs_109062: bool,
        gs_109042: bool,
        gs_109067: bool,
        gs_109060: bool,
        gs_109129: bool,
        gs_109024: bool,
        gs_109115: bool,
        ga_172579: ProductType700c18a878c5601b,
        gs_109158: bool,
        gs_109074: bool,
        gs_109022: bool,
        gs_109112: bool,
        gs_109111: bool,
        gs_109135: bool,
        gs_109029: bool,
        gs_109070: bool,
        gs_109126: bool,
        gs_109157: bool,
        gs_109069: bool,
        ga_172696: ProductType700c18a878c5601b,
        ga_172853: ProductType700c18a878c5601b,
        gs_109136: bool,
        gs_109072: bool,
        ga_172859: ProductType700c18a878c5601b,
        gs_109156: bool,
        gs_109084: bool,
        gs_109120: bool,
        gs_109130: bool,
        gs_109138: bool,
        gs_109006: bool,
        gs_109023: bool,
        gs_109105: bool,
        ga_172850: ProductType700c18a878c5601b,
        gs_109121: bool,
        gs_109085: bool,
        gs_109131: bool,
        gs_109066: bool,
        gs_109073: bool,
        gs_109059: bool,
        ga_172862: ProductType700c18a878c5601b,
        gs_109065: bool,
        gs_109039: bool,
        u__PSTATE_M: u8,
        gs_109108: bool,
        gs_109031: bool,
        u__MDCR_EL2_TDCC: bool,
        gs_109057: bool,
        ga_172699: ProductType700c18a878c5601b,
        gs_109106: bool,
        u__DBGDSCRext_UDCCdis: bool,
        gs_109134: bool,
        gs_109063: bool,
        ga_172786: ProductType700c18a878c5601b,
        gs_109033: bool,
        ga_172789: ProductType700c18a878c5601b,
        gs_109037: bool,
        gs_109041: bool,
        gs_109161: bool,
        gs_109040: bool,
        gs_109078: bool,
        gs_109082: bool,
        gs_109025: bool,
        gs_109080: bool,
        gs_109114: bool,
        gs_109103: bool,
        gs_109028: bool,
        gs_109155: bool,
        gs_109154: bool,
        gs_109035: bool,
        u__PSTATE_EL: u8,
        gs_109159: bool,
        gs_109058: bool,
        gs_109127: bool,
        gs_109034: bool,
        gs_109076: bool,
        gs_109064: bool,
        gs_109125: bool,
        gs_109027: bool,
        gs_109123: bool,
        gs_109110: bool,
        gs_109036: bool,
        gs_109030: bool,
        gs_109109: bool,
        gs_109124: bool,
        gs_109132: bool,
        gs_109079: bool,
        gs_109007: bool,
        u__MDCR_EL3_TDA: bool,
        gs_109087: bool,
        gs_109068: bool,
        ga_172576: ProductType700c18a878c5601b,
        gs_109032: bool,
        gs_109088: bool,
        gs_109104: bool,
        gs_109026: bool,
        gs_109118: bool,
        u__MDCR_EL3_TDCC: bool,
        gs_109113: bool,
        gs_109153: bool,
        gs_109083: bool,
        u__MDSCR_EL1_TDCC: bool,
        gs_109071: bool,
        gs_109075: bool,
        gs_109086: bool,
        gs_109117: bool,
        gs_109119: bool,
        u__HDCR_TDCC: bool,
        gs_109137: bool,
        gs_109116: bool,
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
        // D s_0_5: call _get_MDCR_EL3_Type_TDCC(s_0_4)
        let s_0_5: bool = u_get_MDCR_EL3_Type_TDCC(state, tracer, s_0_4);
        // D s_0_6: write-var __MDCR_EL3_TDCC <= s_0_5
        fn_state.u__MDCR_EL3_TDCC = s_0_5;
        // C s_0_7: const #15048u : u32
        let s_0_7: u32 = 15048;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_SDCR_Type_TDCC(s_0_8)
        let s_0_9: bool = u_get_SDCR_Type_TDCC(state, tracer, s_0_8);
        // D s_0_10: write-var __SDCR_TDCC <= s_0_9
        fn_state.u__SDCR_TDCC = s_0_9;
        // C s_0_11: const #22712u : u32
        let s_0_11: u32 = 22712;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_MDCR_EL3_Type_TDA(s_0_12)
        let s_0_13: bool = u_get_MDCR_EL3_Type_TDA(state, tracer, s_0_12);
        // D s_0_14: write-var __MDCR_EL3_TDA <= s_0_13
        fn_state.u__MDCR_EL3_TDA = s_0_13;
        // C s_0_15: const #104648u : u32
        let s_0_15: u32 = 104648;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_MDSCR_EL1_Type_TDCC(s_0_16)
        let s_0_17: bool = u_get_MDSCR_EL1_Type_TDCC(state, tracer, s_0_16);
        // D s_0_18: write-var __MDSCR_EL1_TDCC <= s_0_17
        fn_state.u__MDSCR_EL1_TDCC = s_0_17;
        // C s_0_19: const #() : ()
        let s_0_19: () = ();
        // S s_0_20: call DBGDSCRext_read(s_0_19)
        let s_0_20: ProductType700c18a878c5601b = DBGDSCRext_read(state, tracer, s_0_19);
        // S s_0_21: call _get_DBGDSCRext_Type_UDCCdis(s_0_20)
        let s_0_21: bool = u_get_DBGDSCRext_Type_UDCCdis(state, tracer, s_0_20);
        // D s_0_22: write-var __DBGDSCRext_UDCCdis <= s_0_21
        fn_state.u__DBGDSCRext_UDCCdis = s_0_21;
        // C s_0_23: const #104880u : u32
        let s_0_23: u32 = 104880;
        // D s_0_24: read-reg s_0_23:struct
        let s_0_24: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_23 as isize);
            tracer.read_register(s_0_23 as isize, value);
            value
        };
        // D s_0_25: call _get_MDCR_EL2_Type_TDCC(s_0_24)
        let s_0_25: bool = u_get_MDCR_EL2_Type_TDCC(state, tracer, s_0_24);
        // D s_0_26: write-var __MDCR_EL2_TDCC <= s_0_25
        fn_state.u__MDCR_EL2_TDCC = s_0_25;
        // C s_0_27: const #() : ()
        let s_0_27: () = ();
        // S s_0_28: call HDCR_read(s_0_27)
        let s_0_28: ProductType700c18a878c5601b = HDCR_read(state, tracer, s_0_27);
        // S s_0_29: call _get_HDCR_Type_TDCC(s_0_28)
        let s_0_29: bool = u_get_HDCR_Type_TDCC(state, tracer, s_0_28);
        // D s_0_30: write-var __HDCR_TDCC <= s_0_29
        fn_state.u__HDCR_TDCC = s_0_29;
        // C s_0_31: const #16983u : u32
        let s_0_31: u32 = 16983;
        // D s_0_32: read-reg s_0_31:u8
        let s_0_32: u8 = {
            let value = state.read_register::<u8>(s_0_31 as isize);
            tracer.read_register(s_0_31 as isize, value);
            value
        };
        // D s_0_33: write-var __PSTATE_M <= s_0_32
        fn_state.u__PSTATE_M = s_0_32;
        // C s_0_34: const #() : ()
        let s_0_34: () = ();
        // S s_0_35: call Halted(s_0_34)
        let s_0_35: bool = Halted(state, tracer, s_0_34);
        // N s_0_36: branch s_0_35 b402 b1
        if s_0_35 {
            return block_402(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#109006 <= s_1_0
        fn_state.gs_109006 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#109006:u8
        let s_2_0: bool = fn_state.gs_109006;
        // N s_2_1: branch s_2_0 b399 b3
        if s_2_0 {
            return block_399(state, tracer, fn_state);
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
        // C s_3_2: const #448u : u32
        let s_3_2: u32 = 448;
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
        // N s_3_6: branch s_3_5 b234 b4
        if s_3_5 {
            return block_234(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var __PSTATE_EL:u8
        let s_4_0: u8 = fn_state.u__PSTATE_EL;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 2u16);
        // C s_4_2: const #440u : u32
        let s_4_2: u32 = 440;
        // D s_4_3: read-reg s_4_2:u8
        let s_4_3: u8 = {
            let value = state.read_register::<u8>(s_4_2 as isize);
            tracer.read_register(s_4_2 as isize, value);
            value
        };
        // D s_4_4: cast zx s_4_3 -> bv
        let s_4_4: Bits = Bits::new(s_4_3 as u128, 2u16);
        // D s_4_5: cmp-eq s_4_1 s_4_4
        let s_4_5: bool = ((s_4_1) == (s_4_4));
        // N s_4_6: branch s_4_5 b109 b5
        if s_4_5 {
            return block_109(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var __PSTATE_EL:u8
        let s_5_0: u8 = fn_state.u__PSTATE_EL;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 2u16);
        // C s_5_2: const #432u : u32
        let s_5_2: u32 = 432;
        // D s_5_3: read-reg s_5_2:u8
        let s_5_3: u8 = {
            let value = state.read_register::<u8>(s_5_2 as isize);
            tracer.read_register(s_5_2 as isize, value);
            value
        };
        // D s_5_4: cast zx s_5_3 -> bv
        let s_5_4: Bits = Bits::new(s_5_3 as u128, 2u16);
        // D s_5_5: cmp-eq s_5_1 s_5_4
        let s_5_5: bool = ((s_5_1) == (s_5_4));
        // N s_5_6: branch s_5_5 b16 b6
        if s_5_5 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var __PSTATE_EL:u8
        let s_6_0: u8 = fn_state.u__PSTATE_EL;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 2u16);
        // C s_6_2: const #424u : u32
        let s_6_2: u32 = 424;
        // D s_6_3: read-reg s_6_2:u8
        let s_6_3: u8 = {
            let value = state.read_register::<u8>(s_6_2 as isize);
            tracer.read_register(s_6_2 as isize, value);
            value
        };
        // D s_6_4: cast zx s_6_3 -> bv
        let s_6_4: Bits = Bits::new(s_6_3 as u128, 2u16);
        // D s_6_5: cmp-eq s_6_1 s_6_4
        let s_6_5: bool = ((s_6_1) == (s_6_4));
        // N s_6_6: branch s_6_5 b8 b7
        if s_6_5 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var __PSTATE_M:u8
        let s_8_0: u8 = fn_state.u__PSTATE_M;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 5u16);
        // C s_8_2: const #384u : u32
        let s_8_2: u32 = 384;
        // D s_8_3: read-reg s_8_2:u8
        let s_8_3: u8 = {
            let value = state.read_register::<u8>(s_8_2 as isize);
            tracer.read_register(s_8_2 as isize, value);
            value
        };
        // D s_8_4: cast zx s_8_3 -> bv
        let s_8_4: Bits = Bits::new(s_8_3 as u128, 5u16);
        // D s_8_5: cmp-ne s_8_1 s_8_4
        let s_8_5: bool = ((s_8_1) != (s_8_4));
        // N s_8_6: branch s_8_5 b15 b9
        if s_8_5 {
            return block_15(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#109007 <= s_9_0
        fn_state.gs_109007 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#109007:u8
        let s_10_0: bool = fn_state.gs_109007;
        // N s_10_1: branch s_10_0 b14 b11
        if s_10_0 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #15s : i
        let s_11_0: i128 = 15;
        // D s_11_1: read-var t:i
        let s_11_1: i128 = fn_state.t;
        // D s_11_2: cmp-eq s_11_1 s_11_0
        let s_11_2: bool = ((s_11_1) == (s_11_0));
        // N s_11_3: branch s_11_2 b13 b12
        if s_11_2 {
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
        // C s_12_0: const #() : ()
        let s_12_0: () = ();
        // S s_12_1: call DBGDSCRint_read(s_12_0)
        let s_12_1: ProductType700c18a878c5601b = DBGDSCRint_read(state, tracer, s_12_0);
        // S s_12_2: call __get_DBGDSCRint(s_12_1)
        let s_12_2: ProductType700c18a878c5601b = u__get_DBGDSCRint(
            state,
            tracer,
            s_12_1,
        );
        // D s_12_3: write-var ga#172862 <= s_12_2
        fn_state.ga_172862 = s_12_2;
        // D s_12_4: read-var ga#172862.0:struct
        let s_12_4: u32 = fn_state.ga_172862._0;
        // D s_12_5: read-var t:i
        let s_12_5: i128 = fn_state.t;
        // D s_12_6: call R_set(s_12_5, s_12_4)
        let s_12_6: () = R_set(state, tracer, s_12_5, s_12_4);
        // N s_12_7: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #() : ()
        let s_13_0: () = ();
        // S s_13_1: call DBGDSCRint_read(s_13_0)
        let s_13_1: ProductType700c18a878c5601b = DBGDSCRint_read(state, tracer, s_13_0);
        // D s_13_2: write-var ga#172859 <= s_13_1
        fn_state.ga_172859 = s_13_1;
        // D s_13_3: read-var ga#172859.0:struct
        let s_13_3: u32 = fn_state.ga_172859._0;
        // C s_13_4: const #28s : i
        let s_13_4: i128 = 28;
        // D s_13_5: cast zx s_13_3 -> bv
        let s_13_5: Bits = Bits::new(s_13_3 as u128, 32u16);
        // C s_13_6: const #1s : i64
        let s_13_6: i64 = 1;
        // C s_13_7: cast zx s_13_6 -> i
        let s_13_7: i128 = (i128::try_from(s_13_6).unwrap());
        // C s_13_8: const #3s : i
        let s_13_8: i128 = 3;
        // C s_13_9: add s_13_8 s_13_7
        let s_13_9: i128 = (s_13_8 + s_13_7);
        // D s_13_10: bit-extract s_13_5 s_13_4 s_13_9
        let s_13_10: Bits = (Bits::new(
            ((s_13_5) >> (s_13_4)).value(),
            u16::try_from(s_13_9).unwrap(),
        ));
        // D s_13_11: cast reint s_13_10 -> u8
        let s_13_11: u8 = (s_13_10.value() as u8);
        // C s_13_12: const #3s : i
        let s_13_12: i128 = 3;
        // D s_13_13: cast zx s_13_11 -> bv
        let s_13_13: Bits = Bits::new(s_13_11 as u128, 4u16);
        // C s_13_14: const #1s : i64
        let s_13_14: i64 = 1;
        // C s_13_15: cast zx s_13_14 -> i
        let s_13_15: i128 = (i128::try_from(s_13_14).unwrap());
        // C s_13_16: const #0s : i
        let s_13_16: i128 = 0;
        // C s_13_17: add s_13_16 s_13_15
        let s_13_17: i128 = (s_13_16 + s_13_15);
        // D s_13_18: bit-extract s_13_13 s_13_12 s_13_17
        let s_13_18: Bits = (Bits::new(
            ((s_13_13) >> (s_13_12)).value(),
            u16::try_from(s_13_17).unwrap(),
        ));
        // D s_13_19: cast reint s_13_18 -> u8
        let s_13_19: bool = ((s_13_18.value()) != 0);
        // C s_13_20: const #16984u : u32
        let s_13_20: u32 = 16984;
        // N s_13_21: write-reg s_13_20 <= s_13_19
        let s_13_21: () = {
            state.write_register::<bool>(s_13_20 as isize, s_13_19);
            tracer.write_register(s_13_20 as isize, s_13_19);
        };
        // C s_13_22: const #2s : i
        let s_13_22: i128 = 2;
        // D s_13_23: cast zx s_13_11 -> bv
        let s_13_23: Bits = Bits::new(s_13_11 as u128, 4u16);
        // C s_13_24: const #1s : i64
        let s_13_24: i64 = 1;
        // C s_13_25: cast zx s_13_24 -> i
        let s_13_25: i128 = (i128::try_from(s_13_24).unwrap());
        // C s_13_26: const #0s : i
        let s_13_26: i128 = 0;
        // C s_13_27: add s_13_26 s_13_25
        let s_13_27: i128 = (s_13_26 + s_13_25);
        // D s_13_28: bit-extract s_13_23 s_13_22 s_13_27
        let s_13_28: Bits = (Bits::new(
            ((s_13_23) >> (s_13_22)).value(),
            u16::try_from(s_13_27).unwrap(),
        ));
        // D s_13_29: cast reint s_13_28 -> u8
        let s_13_29: bool = ((s_13_28.value()) != 0);
        // C s_13_30: const #16997u : u32
        let s_13_30: u32 = 16997;
        // N s_13_31: write-reg s_13_30 <= s_13_29
        let s_13_31: () = {
            state.write_register::<bool>(s_13_30 as isize, s_13_29);
            tracer.write_register(s_13_30 as isize, s_13_29);
        };
        // C s_13_32: const #1s : i
        let s_13_32: i128 = 1;
        // D s_13_33: cast zx s_13_11 -> bv
        let s_13_33: Bits = Bits::new(s_13_11 as u128, 4u16);
        // C s_13_34: const #1s : i64
        let s_13_34: i64 = 1;
        // C s_13_35: cast zx s_13_34 -> i
        let s_13_35: i128 = (i128::try_from(s_13_34).unwrap());
        // C s_13_36: const #0s : i
        let s_13_36: i128 = 0;
        // C s_13_37: add s_13_36 s_13_35
        let s_13_37: i128 = (s_13_36 + s_13_35);
        // D s_13_38: bit-extract s_13_33 s_13_32 s_13_37
        let s_13_38: Bits = (Bits::new(
            ((s_13_33) >> (s_13_32)).value(),
            u16::try_from(s_13_37).unwrap(),
        ));
        // D s_13_39: cast reint s_13_38 -> u8
        let s_13_39: bool = ((s_13_38.value()) != 0);
        // C s_13_40: const #16971u : u32
        let s_13_40: u32 = 16971;
        // N s_13_41: write-reg s_13_40 <= s_13_39
        let s_13_41: () = {
            state.write_register::<bool>(s_13_40 as isize, s_13_39);
            tracer.write_register(s_13_40 as isize, s_13_39);
        };
        // C s_13_42: const #0s : i
        let s_13_42: i128 = 0;
        // D s_13_43: cast zx s_13_11 -> bv
        let s_13_43: Bits = Bits::new(s_13_11 as u128, 4u16);
        // C s_13_44: const #1s : i64
        let s_13_44: i64 = 1;
        // C s_13_45: cast zx s_13_44 -> i
        let s_13_45: i128 = (i128::try_from(s_13_44).unwrap());
        // C s_13_46: const #0s : i
        let s_13_46: i128 = 0;
        // C s_13_47: add s_13_46 s_13_45
        let s_13_47: i128 = (s_13_46 + s_13_45);
        // D s_13_48: bit-extract s_13_43 s_13_42 s_13_47
        let s_13_48: Bits = (Bits::new(
            ((s_13_43) >> (s_13_42)).value(),
            u16::try_from(s_13_47).unwrap(),
        ));
        // D s_13_49: cast reint s_13_48 -> u8
        let s_13_49: bool = ((s_13_48.value()) != 0);
        // C s_13_50: const #16996u : u32
        let s_13_50: u32 = 16996;
        // N s_13_51: write-reg s_13_50 <= s_13_49
        let s_13_51: () = {
            state.write_register::<bool>(s_13_50 as isize, s_13_49);
            tracer.write_register(s_13_50 as isize, s_13_49);
        };
        // N s_13_52: return
        return;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call AArch32_TakeMonitorTrapException(s_14_0)
        let s_14_1: () = AArch32_TakeMonitorTrapException(state, tracer, s_14_0);
        // N s_14_2: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var __SDCR_TDCC:u8
        let s_15_0: bool = fn_state.u__SDCR_TDCC;
        // D s_15_1: cast zx s_15_0 -> bv
        let s_15_1: Bits = Bits::new(s_15_0 as u128, 1u16);
        // C s_15_2: const #1u : u8
        let s_15_2: bool = true;
        // C s_15_3: cast zx s_15_2 -> bv
        let s_15_3: Bits = Bits::new(s_15_2 as u128, 1u16);
        // D s_15_4: cmp-eq s_15_1 s_15_3
        let s_15_4: bool = ((s_15_1) == (s_15_3));
        // D s_15_5: write-var gs#109007 <= s_15_4
        fn_state.gs_109007 = s_15_4;
        // N s_15_6: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #() : ()
        let s_16_0: () = ();
        // S s_16_1: call Halted(s_16_0)
        let s_16_1: bool = Halted(state, tracer, s_16_0);
        // N s_16_2: branch s_16_1 b108 b17
        if s_16_1 {
            return block_108(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var gs#109022 <= s_17_0
        fn_state.gs_109022 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#109022:u8
        let s_18_0: bool = fn_state.gs_109022;
        // N s_18_1: branch s_18_0 b107 b19
        if s_18_0 {
            return block_107(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #0u : u8
        let s_19_0: bool = false;
        // D s_19_1: write-var gs#109023 <= s_19_0
        fn_state.gs_109023 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#109023:u8
        let s_20_0: bool = fn_state.gs_109023;
        // N s_20_1: branch s_20_0 b106 b21
        if s_20_0 {
            return block_106(state, tracer, fn_state);
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
        // D s_21_1: write-var gs#109024 <= s_21_0
        fn_state.gs_109024 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#109024:u8
        let s_22_0: bool = fn_state.gs_109024;
        // N s_22_1: branch s_22_0 b105 b23
        if s_22_0 {
            return block_105(state, tracer, fn_state);
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
        // D s_23_1: write-var gs#109025 <= s_23_0
        fn_state.gs_109025 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#109025:u8
        let s_24_0: bool = fn_state.gs_109025;
        // N s_24_1: branch s_24_0 b104 b25
        if s_24_0 {
            return block_104(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #0u : u8
        let s_25_0: bool = false;
        // D s_25_1: write-var gs#109026 <= s_25_0
        fn_state.gs_109026 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#109026:u8
        let s_26_0: bool = fn_state.gs_109026;
        // N s_26_1: branch s_26_0 b103 b27
        if s_26_0 {
            return block_103(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #() : ()
        let s_27_0: () = ();
        // S s_27_1: call Halted(s_27_0)
        let s_27_1: bool = Halted(state, tracer, s_27_0);
        // N s_27_2: branch s_27_1 b102 b28
        if s_27_1 {
            return block_102(state, tracer, fn_state);
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
        // D s_28_1: write-var gs#109027 <= s_28_0
        fn_state.gs_109027 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#109027:u8
        let s_29_0: bool = fn_state.gs_109027;
        // N s_29_1: branch s_29_0 b101 b30
        if s_29_0 {
            return block_101(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #0u : u8
        let s_30_0: bool = false;
        // D s_30_1: write-var gs#109028 <= s_30_0
        fn_state.gs_109028 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var gs#109028:u8
        let s_31_0: bool = fn_state.gs_109028;
        // N s_31_1: branch s_31_0 b100 b32
        if s_31_0 {
            return block_100(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #0u : u8
        let s_32_0: bool = false;
        // D s_32_1: write-var gs#109029 <= s_32_0
        fn_state.gs_109029 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#109029:u8
        let s_33_0: bool = fn_state.gs_109029;
        // N s_33_1: branch s_33_0 b99 b34
        if s_33_0 {
            return block_99(state, tracer, fn_state);
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
        // D s_34_1: write-var gs#109030 <= s_34_0
        fn_state.gs_109030 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#109030:u8
        let s_35_0: bool = fn_state.gs_109030;
        // N s_35_1: branch s_35_0 b98 b36
        if s_35_0 {
            return block_98(state, tracer, fn_state);
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
        // D s_36_1: write-var gs#109031 <= s_36_0
        fn_state.gs_109031 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#109031:u8
        let s_37_0: bool = fn_state.gs_109031;
        // N s_37_1: branch s_37_0 b97 b38
        if s_37_0 {
            return block_97(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #() : ()
        let s_38_0: () = ();
        // S s_38_1: call Halted(s_38_0)
        let s_38_1: bool = Halted(state, tracer, s_38_0);
        // N s_38_2: branch s_38_1 b96 b39
        if s_38_1 {
            return block_96(state, tracer, fn_state);
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
        // D s_39_1: write-var gs#109032 <= s_39_0
        fn_state.gs_109032 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#109032:u8
        let s_40_0: bool = fn_state.gs_109032;
        // N s_40_1: branch s_40_0 b95 b41
        if s_40_0 {
            return block_95(state, tracer, fn_state);
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
        // D s_41_1: write-var gs#109033 <= s_41_0
        fn_state.gs_109033 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#109033:u8
        let s_42_0: bool = fn_state.gs_109033;
        // N s_42_1: branch s_42_0 b94 b43
        if s_42_0 {
            return block_94(state, tracer, fn_state);
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
        // D s_43_1: write-var gs#109034 <= s_43_0
        fn_state.gs_109034 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#109034:u8
        let s_44_0: bool = fn_state.gs_109034;
        // N s_44_1: branch s_44_0 b93 b45
        if s_44_0 {
            return block_93(state, tracer, fn_state);
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
        // D s_45_1: write-var gs#109035 <= s_45_0
        fn_state.gs_109035 = s_45_0;
        // N s_45_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var gs#109035:u8
        let s_46_0: bool = fn_state.gs_109035;
        // N s_46_1: branch s_46_0 b92 b47
        if s_46_0 {
            return block_92(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #0u : u8
        let s_47_0: bool = false;
        // D s_47_1: write-var gs#109036 <= s_47_0
        fn_state.gs_109036 = s_47_0;
        // N s_47_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var gs#109036:u8
        let s_48_0: bool = fn_state.gs_109036;
        // N s_48_1: branch s_48_0 b91 b49
        if s_48_0 {
            return block_91(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #424u : u32
        let s_49_0: u32 = 424;
        // D s_49_1: read-reg s_49_0:u8
        let s_49_1: u8 = {
            let value = state.read_register::<u8>(s_49_0 as isize);
            tracer.read_register(s_49_0 as isize, value);
            value
        };
        // C s_49_2: const #2u : u8
        let s_49_2: u8 = 2;
        // D s_49_3: cmp-lt s_49_1 s_49_2
        let s_49_3: bool = ((s_49_1) < (s_49_2));
        // N s_49_4: branch s_49_3 b90 b50
        if s_49_3 {
            return block_90(state, tracer, fn_state);
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
        // D s_50_1: write-var gs#109037 <= s_50_0
        fn_state.gs_109037 = s_50_0;
        // N s_50_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var gs#109037:u8
        let s_51_0: bool = fn_state.gs_109037;
        // N s_51_1: branch s_51_0 b89 b52
        if s_51_0 {
            return block_89(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #0u : u8
        let s_52_0: bool = false;
        // D s_52_1: write-var gs#109038 <= s_52_0
        fn_state.gs_109038 = s_52_0;
        // N s_52_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var gs#109038:u8
        let s_53_0: bool = fn_state.gs_109038;
        // N s_53_1: branch s_53_0 b83 b54
        if s_53_0 {
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
        // C s_54_0: const #424u : u32
        let s_54_0: u32 = 424;
        // D s_54_1: read-reg s_54_0:u8
        let s_54_1: u8 = {
            let value = state.read_register::<u8>(s_54_0 as isize);
            tracer.read_register(s_54_0 as isize, value);
            value
        };
        // C s_54_2: const #2u : u8
        let s_54_2: u8 = 2;
        // D s_54_3: cmp-lt s_54_1 s_54_2
        let s_54_3: bool = ((s_54_1) < (s_54_2));
        // N s_54_4: branch s_54_3 b82 b55
        if s_54_3 {
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
        // D s_55_1: write-var gs#109039 <= s_55_0
        fn_state.gs_109039 = s_55_0;
        // N s_55_2: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var gs#109039:u8
        let s_56_0: bool = fn_state.gs_109039;
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
        // D s_57_1: write-var gs#109040 <= s_57_0
        fn_state.gs_109040 = s_57_0;
        // N s_57_2: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_58_0: read-var gs#109040:u8
        let s_58_0: bool = fn_state.gs_109040;
        // N s_58_1: branch s_58_0 b75 b59
        if s_58_0 {
            return block_75(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #424u : u32
        let s_59_0: u32 = 424;
        // D s_59_1: read-reg s_59_0:u8
        let s_59_1: u8 = {
            let value = state.read_register::<u8>(s_59_0 as isize);
            tracer.read_register(s_59_0 as isize, value);
            value
        };
        // C s_59_2: const #2u : u8
        let s_59_2: u8 = 2;
        // D s_59_3: cmp-lt s_59_1 s_59_2
        let s_59_3: bool = ((s_59_1) < (s_59_2));
        // N s_59_4: branch s_59_3 b74 b60
        if s_59_3 {
            return block_74(state, tracer, fn_state);
        } else {
            return block_60(state, tracer, fn_state);
        };
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #0u : u8
        let s_60_0: bool = false;
        // D s_60_1: write-var gs#109041 <= s_60_0
        fn_state.gs_109041 = s_60_0;
        // N s_60_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_61_0: read-var gs#109041:u8
        let s_61_0: bool = fn_state.gs_109041;
        // N s_61_1: branch s_61_0 b73 b62
        if s_61_0 {
            return block_73(state, tracer, fn_state);
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
        // D s_62_1: write-var gs#109042 <= s_62_0
        fn_state.gs_109042 = s_62_0;
        // N s_62_2: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_63_0: read-var gs#109042:u8
        let s_63_0: bool = fn_state.gs_109042;
        // N s_63_1: branch s_63_0 b67 b64
        if s_63_0 {
            return block_67(state, tracer, fn_state);
        } else {
            return block_64(state, tracer, fn_state);
        };
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #15s : i
        let s_64_0: i128 = 15;
        // D s_64_1: read-var t:i
        let s_64_1: i128 = fn_state.t;
        // D s_64_2: cmp-eq s_64_1 s_64_0
        let s_64_2: bool = ((s_64_1) == (s_64_0));
        // N s_64_3: branch s_64_2 b66 b65
        if s_64_2 {
            return block_66(state, tracer, fn_state);
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
        // S s_65_1: call DBGDSCRint_read(s_65_0)
        let s_65_1: ProductType700c18a878c5601b = DBGDSCRint_read(state, tracer, s_65_0);
        // S s_65_2: call __get_DBGDSCRint(s_65_1)
        let s_65_2: ProductType700c18a878c5601b = u__get_DBGDSCRint(
            state,
            tracer,
            s_65_1,
        );
        // D s_65_3: write-var ga#172853 <= s_65_2
        fn_state.ga_172853 = s_65_2;
        // D s_65_4: read-var ga#172853.0:struct
        let s_65_4: u32 = fn_state.ga_172853._0;
        // D s_65_5: read-var t:i
        let s_65_5: i128 = fn_state.t;
        // D s_65_6: call R_set(s_65_5, s_65_4)
        let s_65_6: () = R_set(state, tracer, s_65_5, s_65_4);
        // N s_65_7: return
        return;
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #() : ()
        let s_66_0: () = ();
        // S s_66_1: call DBGDSCRint_read(s_66_0)
        let s_66_1: ProductType700c18a878c5601b = DBGDSCRint_read(state, tracer, s_66_0);
        // D s_66_2: write-var ga#172850 <= s_66_1
        fn_state.ga_172850 = s_66_1;
        // D s_66_3: read-var ga#172850.0:struct
        let s_66_3: u32 = fn_state.ga_172850._0;
        // C s_66_4: const #28s : i
        let s_66_4: i128 = 28;
        // D s_66_5: cast zx s_66_3 -> bv
        let s_66_5: Bits = Bits::new(s_66_3 as u128, 32u16);
        // C s_66_6: const #1s : i64
        let s_66_6: i64 = 1;
        // C s_66_7: cast zx s_66_6 -> i
        let s_66_7: i128 = (i128::try_from(s_66_6).unwrap());
        // C s_66_8: const #3s : i
        let s_66_8: i128 = 3;
        // C s_66_9: add s_66_8 s_66_7
        let s_66_9: i128 = (s_66_8 + s_66_7);
        // D s_66_10: bit-extract s_66_5 s_66_4 s_66_9
        let s_66_10: Bits = (Bits::new(
            ((s_66_5) >> (s_66_4)).value(),
            u16::try_from(s_66_9).unwrap(),
        ));
        // D s_66_11: cast reint s_66_10 -> u8
        let s_66_11: u8 = (s_66_10.value() as u8);
        // C s_66_12: const #3s : i
        let s_66_12: i128 = 3;
        // D s_66_13: cast zx s_66_11 -> bv
        let s_66_13: Bits = Bits::new(s_66_11 as u128, 4u16);
        // C s_66_14: const #1s : i64
        let s_66_14: i64 = 1;
        // C s_66_15: cast zx s_66_14 -> i
        let s_66_15: i128 = (i128::try_from(s_66_14).unwrap());
        // C s_66_16: const #0s : i
        let s_66_16: i128 = 0;
        // C s_66_17: add s_66_16 s_66_15
        let s_66_17: i128 = (s_66_16 + s_66_15);
        // D s_66_18: bit-extract s_66_13 s_66_12 s_66_17
        let s_66_18: Bits = (Bits::new(
            ((s_66_13) >> (s_66_12)).value(),
            u16::try_from(s_66_17).unwrap(),
        ));
        // D s_66_19: cast reint s_66_18 -> u8
        let s_66_19: bool = ((s_66_18.value()) != 0);
        // C s_66_20: const #16984u : u32
        let s_66_20: u32 = 16984;
        // N s_66_21: write-reg s_66_20 <= s_66_19
        let s_66_21: () = {
            state.write_register::<bool>(s_66_20 as isize, s_66_19);
            tracer.write_register(s_66_20 as isize, s_66_19);
        };
        // C s_66_22: const #2s : i
        let s_66_22: i128 = 2;
        // D s_66_23: cast zx s_66_11 -> bv
        let s_66_23: Bits = Bits::new(s_66_11 as u128, 4u16);
        // C s_66_24: const #1s : i64
        let s_66_24: i64 = 1;
        // C s_66_25: cast zx s_66_24 -> i
        let s_66_25: i128 = (i128::try_from(s_66_24).unwrap());
        // C s_66_26: const #0s : i
        let s_66_26: i128 = 0;
        // C s_66_27: add s_66_26 s_66_25
        let s_66_27: i128 = (s_66_26 + s_66_25);
        // D s_66_28: bit-extract s_66_23 s_66_22 s_66_27
        let s_66_28: Bits = (Bits::new(
            ((s_66_23) >> (s_66_22)).value(),
            u16::try_from(s_66_27).unwrap(),
        ));
        // D s_66_29: cast reint s_66_28 -> u8
        let s_66_29: bool = ((s_66_28.value()) != 0);
        // C s_66_30: const #16997u : u32
        let s_66_30: u32 = 16997;
        // N s_66_31: write-reg s_66_30 <= s_66_29
        let s_66_31: () = {
            state.write_register::<bool>(s_66_30 as isize, s_66_29);
            tracer.write_register(s_66_30 as isize, s_66_29);
        };
        // C s_66_32: const #1s : i
        let s_66_32: i128 = 1;
        // D s_66_33: cast zx s_66_11 -> bv
        let s_66_33: Bits = Bits::new(s_66_11 as u128, 4u16);
        // C s_66_34: const #1s : i64
        let s_66_34: i64 = 1;
        // C s_66_35: cast zx s_66_34 -> i
        let s_66_35: i128 = (i128::try_from(s_66_34).unwrap());
        // C s_66_36: const #0s : i
        let s_66_36: i128 = 0;
        // C s_66_37: add s_66_36 s_66_35
        let s_66_37: i128 = (s_66_36 + s_66_35);
        // D s_66_38: bit-extract s_66_33 s_66_32 s_66_37
        let s_66_38: Bits = (Bits::new(
            ((s_66_33) >> (s_66_32)).value(),
            u16::try_from(s_66_37).unwrap(),
        ));
        // D s_66_39: cast reint s_66_38 -> u8
        let s_66_39: bool = ((s_66_38.value()) != 0);
        // C s_66_40: const #16971u : u32
        let s_66_40: u32 = 16971;
        // N s_66_41: write-reg s_66_40 <= s_66_39
        let s_66_41: () = {
            state.write_register::<bool>(s_66_40 as isize, s_66_39);
            tracer.write_register(s_66_40 as isize, s_66_39);
        };
        // C s_66_42: const #0s : i
        let s_66_42: i128 = 0;
        // D s_66_43: cast zx s_66_11 -> bv
        let s_66_43: Bits = Bits::new(s_66_11 as u128, 4u16);
        // C s_66_44: const #1s : i64
        let s_66_44: i64 = 1;
        // C s_66_45: cast zx s_66_44 -> i
        let s_66_45: i128 = (i128::try_from(s_66_44).unwrap());
        // C s_66_46: const #0s : i
        let s_66_46: i128 = 0;
        // C s_66_47: add s_66_46 s_66_45
        let s_66_47: i128 = (s_66_46 + s_66_45);
        // D s_66_48: bit-extract s_66_43 s_66_42 s_66_47
        let s_66_48: Bits = (Bits::new(
            ((s_66_43) >> (s_66_42)).value(),
            u16::try_from(s_66_47).unwrap(),
        ));
        // D s_66_49: cast reint s_66_48 -> u8
        let s_66_49: bool = ((s_66_48.value()) != 0);
        // C s_66_50: const #16996u : u32
        let s_66_50: u32 = 16996;
        // N s_66_51: write-reg s_66_50 <= s_66_49
        let s_66_51: () = {
            state.write_register::<bool>(s_66_50 as isize, s_66_49);
            tracer.write_register(s_66_50 as isize, s_66_49);
        };
        // N s_66_52: return
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
        // D s_68_1: write-var gs#109057 <= s_68_0
        fn_state.gs_109057 = s_68_0;
        // N s_68_2: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_69_0: read-var gs#109057:u8
        let s_69_0: bool = fn_state.gs_109057;
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
        // C s_70_0: const #5u : u8
        let s_70_0: u8 = 5;
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
        // D s_72_7: write-var gs#109057 <= s_72_6
        fn_state.gs_109057 = s_72_6;
        // N s_72_8: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_73_0: read-var __MDCR_EL3_TDA:u8
        let s_73_0: bool = fn_state.u__MDCR_EL3_TDA;
        // D s_73_1: cast zx s_73_0 -> bv
        let s_73_1: Bits = Bits::new(s_73_0 as u128, 1u16);
        // C s_73_2: const #1u : u8
        let s_73_2: bool = true;
        // C s_73_3: cast zx s_73_2 -> bv
        let s_73_3: Bits = Bits::new(s_73_2 as u128, 1u16);
        // D s_73_4: cmp-eq s_73_1 s_73_3
        let s_73_4: bool = ((s_73_1) == (s_73_3));
        // D s_73_5: write-var gs#109042 <= s_73_4
        fn_state.gs_109042 = s_73_4;
        // N s_73_6: jump b63
        return block_63(state, tracer, fn_state);
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
        // D s_74_4: write-var gs#109041 <= s_74_3
        fn_state.gs_109041 = s_74_3;
        // N s_74_5: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #() : ()
        let s_75_0: () = ();
        // S s_75_1: call Halted(s_75_0)
        let s_75_1: bool = Halted(state, tracer, s_75_0);
        // N s_75_2: branch s_75_1 b80 b76
        if s_75_1 {
            return block_80(state, tracer, fn_state);
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
        // D s_76_1: write-var gs#109058 <= s_76_0
        fn_state.gs_109058 = s_76_0;
        // N s_76_2: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_77_0: read-var gs#109058:u8
        let s_77_0: bool = fn_state.gs_109058;
        // N s_77_1: branch s_77_0 b79 b78
        if s_77_0 {
            return block_79(state, tracer, fn_state);
        } else {
            return block_78(state, tracer, fn_state);
        };
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_78_0: const #() : ()
        let s_78_0: () = ();
        // S s_78_1: call AArch32_TakeMonitorTrapException(s_78_0)
        let s_78_1: () = AArch32_TakeMonitorTrapException(state, tracer, s_78_0);
        // N s_78_2: return
        return;
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_79_0: panic
        panic!("{:?}", ());
        // N s_79_1: return
        return;
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #() : ()
        let s_80_0: () = ();
        // S s_80_1: call EDSCR_read(s_80_0)
        let s_80_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_80_0);
        // S s_80_2: call _get_EDSCR_Type_SDD(s_80_1)
        let s_80_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_80_1);
        // S s_80_3: cast zx s_80_2 -> bv
        let s_80_3: Bits = Bits::new(s_80_2 as u128, 1u16);
        // C s_80_4: const #1u : u8
        let s_80_4: bool = true;
        // C s_80_5: cast zx s_80_4 -> bv
        let s_80_5: Bits = Bits::new(s_80_4 as u128, 1u16);
        // S s_80_6: cmp-eq s_80_3 s_80_5
        let s_80_6: bool = ((s_80_3) == (s_80_5));
        // D s_80_7: write-var gs#109058 <= s_80_6
        fn_state.gs_109058 = s_80_6;
        // N s_80_8: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_81_0: read-var __SDCR_TDCC:u8
        let s_81_0: bool = fn_state.u__SDCR_TDCC;
        // D s_81_1: cast zx s_81_0 -> bv
        let s_81_1: Bits = Bits::new(s_81_0 as u128, 1u16);
        // C s_81_2: const #1u : u8
        let s_81_2: bool = true;
        // C s_81_3: cast zx s_81_2 -> bv
        let s_81_3: Bits = Bits::new(s_81_2 as u128, 1u16);
        // D s_81_4: cmp-eq s_81_1 s_81_3
        let s_81_4: bool = ((s_81_1) == (s_81_3));
        // D s_81_5: write-var gs#109040 <= s_81_4
        fn_state.gs_109040 = s_81_4;
        // N s_81_6: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #424u : u32
        let s_82_0: u32 = 424;
        // D s_82_1: read-reg s_82_0:u8
        let s_82_1: u8 = {
            let value = state.read_register::<u8>(s_82_0 as isize);
            tracer.read_register(s_82_0 as isize, value);
            value
        };
        // D s_82_2: call ELUsingAArch32(s_82_1)
        let s_82_2: bool = ELUsingAArch32(state, tracer, s_82_1);
        // D s_82_3: write-var gs#109039 <= s_82_2
        fn_state.gs_109039 = s_82_2;
        // N s_82_4: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_83_0: const #() : ()
        let s_83_0: () = ();
        // S s_83_1: call Halted(s_83_0)
        let s_83_1: bool = Halted(state, tracer, s_83_0);
        // N s_83_2: branch s_83_1 b88 b84
        if s_83_1 {
            return block_88(state, tracer, fn_state);
        } else {
            return block_84(state, tracer, fn_state);
        };
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_84_0: const #0u : u8
        let s_84_0: bool = false;
        // D s_84_1: write-var gs#109059 <= s_84_0
        fn_state.gs_109059 = s_84_0;
        // N s_84_2: jump b85
        return block_85(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_85_0: read-var gs#109059:u8
        let s_85_0: bool = fn_state.gs_109059;
        // N s_85_1: branch s_85_0 b87 b86
        if s_85_0 {
            return block_87(state, tracer, fn_state);
        } else {
            return block_86(state, tracer, fn_state);
        };
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_86_0: const #5u : u8
        let s_86_0: u8 = 5;
        // C s_86_1: cast zx s_86_0 -> bv
        let s_86_1: Bits = Bits::new(s_86_0 as u128, 8u16);
        // C s_86_2: cast zx s_86_1 -> i
        let s_86_2: i128 = (s_86_1.value() as i128);
        // C s_86_3: cast reint s_86_2 -> i64
        let s_86_3: i64 = (s_86_2 as i64);
        // C s_86_4: cast zx s_86_3 -> i
        let s_86_4: i128 = (i128::try_from(s_86_3).unwrap());
        // C s_86_5: const #424u : u32
        let s_86_5: u32 = 424;
        // D s_86_6: read-reg s_86_5:u8
        let s_86_6: u8 = {
            let value = state.read_register::<u8>(s_86_5 as isize);
            tracer.read_register(s_86_5 as isize, value);
            value
        };
        // D s_86_7: call AArch64_AArch32SystemAccessTrap(s_86_6, s_86_4)
        let s_86_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_86_6, s_86_4);
        // N s_86_8: return
        return;
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
        // C s_88_0: const #() : ()
        let s_88_0: () = ();
        // S s_88_1: call EDSCR_read(s_88_0)
        let s_88_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_88_0);
        // S s_88_2: call _get_EDSCR_Type_SDD(s_88_1)
        let s_88_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_88_1);
        // S s_88_3: cast zx s_88_2 -> bv
        let s_88_3: Bits = Bits::new(s_88_2 as u128, 1u16);
        // C s_88_4: const #1u : u8
        let s_88_4: bool = true;
        // C s_88_5: cast zx s_88_4 -> bv
        let s_88_5: Bits = Bits::new(s_88_4 as u128, 1u16);
        // S s_88_6: cmp-eq s_88_3 s_88_5
        let s_88_6: bool = ((s_88_3) == (s_88_5));
        // D s_88_7: write-var gs#109059 <= s_88_6
        fn_state.gs_109059 = s_88_6;
        // N s_88_8: jump b85
        return block_85(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_89_0: read-var __MDCR_EL3_TDCC:u8
        let s_89_0: bool = fn_state.u__MDCR_EL3_TDCC;
        // D s_89_1: cast zx s_89_0 -> bv
        let s_89_1: Bits = Bits::new(s_89_0 as u128, 1u16);
        // C s_89_2: const #1u : u8
        let s_89_2: bool = true;
        // C s_89_3: cast zx s_89_2 -> bv
        let s_89_3: Bits = Bits::new(s_89_2 as u128, 1u16);
        // D s_89_4: cmp-eq s_89_1 s_89_3
        let s_89_4: bool = ((s_89_1) == (s_89_3));
        // D s_89_5: write-var gs#109038 <= s_89_4
        fn_state.gs_109038 = s_89_4;
        // N s_89_6: jump b53
        return block_53(state, tracer, fn_state);
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
        // D s_90_2: call ELUsingAArch32(s_90_1)
        let s_90_2: bool = ELUsingAArch32(state, tracer, s_90_1);
        // D s_90_3: not s_90_2
        let s_90_3: bool = !s_90_2;
        // D s_90_4: write-var gs#109037 <= s_90_3
        fn_state.gs_109037 = s_90_3;
        // N s_90_5: jump b51
        return block_51(state, tracer, fn_state);
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
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_92_0: read-var __MDCR_EL3_TDA:u8
        let s_92_0: bool = fn_state.u__MDCR_EL3_TDA;
        // D s_92_1: cast zx s_92_0 -> bv
        let s_92_1: Bits = Bits::new(s_92_0 as u128, 1u16);
        // C s_92_2: const #1u : u8
        let s_92_2: bool = true;
        // C s_92_3: cast zx s_92_2 -> bv
        let s_92_3: Bits = Bits::new(s_92_2 as u128, 1u16);
        // D s_92_4: cmp-eq s_92_1 s_92_3
        let s_92_4: bool = ((s_92_1) == (s_92_3));
        // D s_92_5: write-var gs#109036 <= s_92_4
        fn_state.gs_109036 = s_92_4;
        // N s_92_6: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_93_0: const #424u : u32
        let s_93_0: u32 = 424;
        // D s_93_1: read-reg s_93_0:u8
        let s_93_1: u8 = {
            let value = state.read_register::<u8>(s_93_0 as isize);
            tracer.read_register(s_93_0 as isize, value);
            value
        };
        // D s_93_2: call ELUsingAArch32(s_93_1)
        let s_93_2: bool = ELUsingAArch32(state, tracer, s_93_1);
        // D s_93_3: not s_93_2
        let s_93_3: bool = !s_93_2;
        // D s_93_4: write-var gs#109035 <= s_93_3
        fn_state.gs_109035 = s_93_3;
        // N s_93_5: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_94_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_94_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_94_1: call __IMPDEF_boolean(s_94_0)
        let s_94_1: bool = u__IMPDEF_boolean(state, tracer, s_94_0);
        // D s_94_2: write-var gs#109034 <= s_94_1
        fn_state.gs_109034 = s_94_1;
        // N s_94_3: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_95_0: const #() : ()
        let s_95_0: () = ();
        // S s_95_1: call EDSCR_read(s_95_0)
        let s_95_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_95_0);
        // S s_95_2: call _get_EDSCR_Type_SDD(s_95_1)
        let s_95_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_95_1);
        // S s_95_3: cast zx s_95_2 -> bv
        let s_95_3: Bits = Bits::new(s_95_2 as u128, 1u16);
        // C s_95_4: const #1u : u8
        let s_95_4: bool = true;
        // C s_95_5: cast zx s_95_4 -> bv
        let s_95_5: Bits = Bits::new(s_95_4 as u128, 1u16);
        // S s_95_6: cmp-eq s_95_3 s_95_5
        let s_95_6: bool = ((s_95_3) == (s_95_5));
        // D s_95_7: write-var gs#109033 <= s_95_6
        fn_state.gs_109033 = s_95_6;
        // N s_95_8: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_96_0: const #424u : u32
        let s_96_0: u32 = 424;
        // D s_96_1: read-reg s_96_0:u8
        let s_96_1: u8 = {
            let value = state.read_register::<u8>(s_96_0 as isize);
            tracer.read_register(s_96_0 as isize, value);
            value
        };
        // C s_96_2: const #2u : u8
        let s_96_2: u8 = 2;
        // D s_96_3: cmp-lt s_96_1 s_96_2
        let s_96_3: bool = ((s_96_1) < (s_96_2));
        // D s_96_4: write-var gs#109032 <= s_96_3
        fn_state.gs_109032 = s_96_3;
        // N s_96_5: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_97_0: panic
        panic!("{:?}", ());
        // N s_97_1: return
        return;
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_98_0: read-var __SDCR_TDCC:u8
        let s_98_0: bool = fn_state.u__SDCR_TDCC;
        // D s_98_1: cast zx s_98_0 -> bv
        let s_98_1: Bits = Bits::new(s_98_0 as u128, 1u16);
        // C s_98_2: const #1u : u8
        let s_98_2: bool = true;
        // C s_98_3: cast zx s_98_2 -> bv
        let s_98_3: Bits = Bits::new(s_98_2 as u128, 1u16);
        // D s_98_4: cmp-eq s_98_1 s_98_3
        let s_98_4: bool = ((s_98_1) == (s_98_3));
        // D s_98_5: write-var gs#109031 <= s_98_4
        fn_state.gs_109031 = s_98_4;
        // N s_98_6: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_99_0: const #424u : u32
        let s_99_0: u32 = 424;
        // D s_99_1: read-reg s_99_0:u8
        let s_99_1: u8 = {
            let value = state.read_register::<u8>(s_99_0 as isize);
            tracer.read_register(s_99_0 as isize, value);
            value
        };
        // D s_99_2: call ELUsingAArch32(s_99_1)
        let s_99_2: bool = ELUsingAArch32(state, tracer, s_99_1);
        // D s_99_3: write-var gs#109030 <= s_99_2
        fn_state.gs_109030 = s_99_2;
        // N s_99_4: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_100_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_100_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_100_1: call __IMPDEF_boolean(s_100_0)
        let s_100_1: bool = u__IMPDEF_boolean(state, tracer, s_100_0);
        // D s_100_2: write-var gs#109029 <= s_100_1
        fn_state.gs_109029 = s_100_1;
        // N s_100_3: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_101_0: const #() : ()
        let s_101_0: () = ();
        // S s_101_1: call EDSCR_read(s_101_0)
        let s_101_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_101_0);
        // S s_101_2: call _get_EDSCR_Type_SDD(s_101_1)
        let s_101_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_101_1);
        // S s_101_3: cast zx s_101_2 -> bv
        let s_101_3: Bits = Bits::new(s_101_2 as u128, 1u16);
        // C s_101_4: const #1u : u8
        let s_101_4: bool = true;
        // C s_101_5: cast zx s_101_4 -> bv
        let s_101_5: Bits = Bits::new(s_101_4 as u128, 1u16);
        // S s_101_6: cmp-eq s_101_3 s_101_5
        let s_101_6: bool = ((s_101_3) == (s_101_5));
        // D s_101_7: write-var gs#109028 <= s_101_6
        fn_state.gs_109028 = s_101_6;
        // N s_101_8: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_102_0: const #424u : u32
        let s_102_0: u32 = 424;
        // D s_102_1: read-reg s_102_0:u8
        let s_102_1: u8 = {
            let value = state.read_register::<u8>(s_102_0 as isize);
            tracer.read_register(s_102_0 as isize, value);
            value
        };
        // C s_102_2: const #2u : u8
        let s_102_2: u8 = 2;
        // D s_102_3: cmp-lt s_102_1 s_102_2
        let s_102_3: bool = ((s_102_1) < (s_102_2));
        // D s_102_4: write-var gs#109027 <= s_102_3
        fn_state.gs_109027 = s_102_3;
        // N s_102_5: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_103_0: panic
        panic!("{:?}", ());
        // N s_103_1: return
        return;
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_104_0: read-var __MDCR_EL3_TDCC:u8
        let s_104_0: bool = fn_state.u__MDCR_EL3_TDCC;
        // D s_104_1: cast zx s_104_0 -> bv
        let s_104_1: Bits = Bits::new(s_104_0 as u128, 1u16);
        // C s_104_2: const #1u : u8
        let s_104_2: bool = true;
        // C s_104_3: cast zx s_104_2 -> bv
        let s_104_3: Bits = Bits::new(s_104_2 as u128, 1u16);
        // D s_104_4: cmp-eq s_104_1 s_104_3
        let s_104_4: bool = ((s_104_1) == (s_104_3));
        // D s_104_5: write-var gs#109026 <= s_104_4
        fn_state.gs_109026 = s_104_4;
        // N s_104_6: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_105_0: const #424u : u32
        let s_105_0: u32 = 424;
        // D s_105_1: read-reg s_105_0:u8
        let s_105_1: u8 = {
            let value = state.read_register::<u8>(s_105_0 as isize);
            tracer.read_register(s_105_0 as isize, value);
            value
        };
        // D s_105_2: call ELUsingAArch32(s_105_1)
        let s_105_2: bool = ELUsingAArch32(state, tracer, s_105_1);
        // D s_105_3: not s_105_2
        let s_105_3: bool = !s_105_2;
        // D s_105_4: write-var gs#109025 <= s_105_3
        fn_state.gs_109025 = s_105_3;
        // N s_105_5: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_106_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_106_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_106_1: call __IMPDEF_boolean(s_106_0)
        let s_106_1: bool = u__IMPDEF_boolean(state, tracer, s_106_0);
        // D s_106_2: write-var gs#109024 <= s_106_1
        fn_state.gs_109024 = s_106_1;
        // N s_106_3: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_107_0: const #() : ()
        let s_107_0: () = ();
        // S s_107_1: call EDSCR_read(s_107_0)
        let s_107_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_107_0);
        // S s_107_2: call _get_EDSCR_Type_SDD(s_107_1)
        let s_107_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_107_1);
        // S s_107_3: cast zx s_107_2 -> bv
        let s_107_3: Bits = Bits::new(s_107_2 as u128, 1u16);
        // C s_107_4: const #1u : u8
        let s_107_4: bool = true;
        // C s_107_5: cast zx s_107_4 -> bv
        let s_107_5: Bits = Bits::new(s_107_4 as u128, 1u16);
        // S s_107_6: cmp-eq s_107_3 s_107_5
        let s_107_6: bool = ((s_107_3) == (s_107_5));
        // D s_107_7: write-var gs#109023 <= s_107_6
        fn_state.gs_109023 = s_107_6;
        // N s_107_8: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_108_0: const #424u : u32
        let s_108_0: u32 = 424;
        // D s_108_1: read-reg s_108_0:u8
        let s_108_1: u8 = {
            let value = state.read_register::<u8>(s_108_0 as isize);
            tracer.read_register(s_108_0 as isize, value);
            value
        };
        // C s_108_2: const #2u : u8
        let s_108_2: u8 = 2;
        // D s_108_3: cmp-lt s_108_1 s_108_2
        let s_108_3: bool = ((s_108_1) < (s_108_2));
        // D s_108_4: write-var gs#109022 <= s_108_3
        fn_state.gs_109022 = s_108_3;
        // N s_108_5: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_109_0: const #() : ()
        let s_109_0: () = ();
        // S s_109_1: call Halted(s_109_0)
        let s_109_1: bool = Halted(state, tracer, s_109_0);
        // N s_109_2: branch s_109_1 b233 b110
        if s_109_1 {
            return block_233(state, tracer, fn_state);
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
        // D s_110_1: write-var gs#109060 <= s_110_0
        fn_state.gs_109060 = s_110_0;
        // N s_110_2: jump b111
        return block_111(state, tracer, fn_state);
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_111_0: read-var gs#109060:u8
        let s_111_0: bool = fn_state.gs_109060;
        // N s_111_1: branch s_111_0 b232 b112
        if s_111_0 {
            return block_232(state, tracer, fn_state);
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
        // D s_112_1: write-var gs#109061 <= s_112_0
        fn_state.gs_109061 = s_112_0;
        // N s_112_2: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_113_0: read-var gs#109061:u8
        let s_113_0: bool = fn_state.gs_109061;
        // N s_113_1: branch s_113_0 b231 b114
        if s_113_0 {
            return block_231(state, tracer, fn_state);
        } else {
            return block_114(state, tracer, fn_state);
        };
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_114_0: const #0u : u8
        let s_114_0: bool = false;
        // D s_114_1: write-var gs#109062 <= s_114_0
        fn_state.gs_109062 = s_114_0;
        // N s_114_2: jump b115
        return block_115(state, tracer, fn_state);
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_115_0: read-var gs#109062:u8
        let s_115_0: bool = fn_state.gs_109062;
        // N s_115_1: branch s_115_0 b230 b116
        if s_115_0 {
            return block_230(state, tracer, fn_state);
        } else {
            return block_116(state, tracer, fn_state);
        };
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_116_0: const #0u : u8
        let s_116_0: bool = false;
        // D s_116_1: write-var gs#109063 <= s_116_0
        fn_state.gs_109063 = s_116_0;
        // N s_116_2: jump b117
        return block_117(state, tracer, fn_state);
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_117_0: read-var gs#109063:u8
        let s_117_0: bool = fn_state.gs_109063;
        // N s_117_1: branch s_117_0 b229 b118
        if s_117_0 {
            return block_229(state, tracer, fn_state);
        } else {
            return block_118(state, tracer, fn_state);
        };
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_118_0: const #0u : u8
        let s_118_0: bool = false;
        // D s_118_1: write-var gs#109064 <= s_118_0
        fn_state.gs_109064 = s_118_0;
        // N s_118_2: jump b119
        return block_119(state, tracer, fn_state);
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_119_0: read-var gs#109064:u8
        let s_119_0: bool = fn_state.gs_109064;
        // N s_119_1: branch s_119_0 b228 b120
        if s_119_0 {
            return block_228(state, tracer, fn_state);
        } else {
            return block_120(state, tracer, fn_state);
        };
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_120_0: const #() : ()
        let s_120_0: () = ();
        // S s_120_1: call Halted(s_120_0)
        let s_120_1: bool = Halted(state, tracer, s_120_0);
        // N s_120_2: branch s_120_1 b227 b121
        if s_120_1 {
            return block_227(state, tracer, fn_state);
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
        // D s_121_1: write-var gs#109065 <= s_121_0
        fn_state.gs_109065 = s_121_0;
        // N s_121_2: jump b122
        return block_122(state, tracer, fn_state);
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_122_0: read-var gs#109065:u8
        let s_122_0: bool = fn_state.gs_109065;
        // N s_122_1: branch s_122_0 b226 b123
        if s_122_0 {
            return block_226(state, tracer, fn_state);
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
        // D s_123_1: write-var gs#109066 <= s_123_0
        fn_state.gs_109066 = s_123_0;
        // N s_123_2: jump b124
        return block_124(state, tracer, fn_state);
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_124_0: read-var gs#109066:u8
        let s_124_0: bool = fn_state.gs_109066;
        // N s_124_1: branch s_124_0 b225 b125
        if s_124_0 {
            return block_225(state, tracer, fn_state);
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
        // D s_125_1: write-var gs#109067 <= s_125_0
        fn_state.gs_109067 = s_125_0;
        // N s_125_2: jump b126
        return block_126(state, tracer, fn_state);
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_126_0: read-var gs#109067:u8
        let s_126_0: bool = fn_state.gs_109067;
        // N s_126_1: branch s_126_0 b224 b127
        if s_126_0 {
            return block_224(state, tracer, fn_state);
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
        // D s_127_1: write-var gs#109068 <= s_127_0
        fn_state.gs_109068 = s_127_0;
        // N s_127_2: jump b128
        return block_128(state, tracer, fn_state);
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_128_0: read-var gs#109068:u8
        let s_128_0: bool = fn_state.gs_109068;
        // N s_128_1: branch s_128_0 b223 b129
        if s_128_0 {
            return block_223(state, tracer, fn_state);
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
        // D s_129_1: write-var gs#109069 <= s_129_0
        fn_state.gs_109069 = s_129_0;
        // N s_129_2: jump b130
        return block_130(state, tracer, fn_state);
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_130_0: read-var gs#109069:u8
        let s_130_0: bool = fn_state.gs_109069;
        // N s_130_1: branch s_130_0 b222 b131
        if s_130_0 {
            return block_222(state, tracer, fn_state);
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
        // S s_131_1: call Halted(s_131_0)
        let s_131_1: bool = Halted(state, tracer, s_131_0);
        // N s_131_2: branch s_131_1 b221 b132
        if s_131_1 {
            return block_221(state, tracer, fn_state);
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
        // D s_132_1: write-var gs#109070 <= s_132_0
        fn_state.gs_109070 = s_132_0;
        // N s_132_2: jump b133
        return block_133(state, tracer, fn_state);
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_133_0: read-var gs#109070:u8
        let s_133_0: bool = fn_state.gs_109070;
        // N s_133_1: branch s_133_0 b220 b134
        if s_133_0 {
            return block_220(state, tracer, fn_state);
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
        // D s_134_1: write-var gs#109071 <= s_134_0
        fn_state.gs_109071 = s_134_0;
        // N s_134_2: jump b135
        return block_135(state, tracer, fn_state);
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_135_0: read-var gs#109071:u8
        let s_135_0: bool = fn_state.gs_109071;
        // N s_135_1: branch s_135_0 b219 b136
        if s_135_0 {
            return block_219(state, tracer, fn_state);
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
        // D s_136_1: write-var gs#109072 <= s_136_0
        fn_state.gs_109072 = s_136_0;
        // N s_136_2: jump b137
        return block_137(state, tracer, fn_state);
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_137_0: read-var gs#109072:u8
        let s_137_0: bool = fn_state.gs_109072;
        // N s_137_1: branch s_137_0 b218 b138
        if s_137_0 {
            return block_218(state, tracer, fn_state);
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
        // D s_138_1: write-var gs#109073 <= s_138_0
        fn_state.gs_109073 = s_138_0;
        // N s_138_2: jump b139
        return block_139(state, tracer, fn_state);
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_139_0: read-var gs#109073:u8
        let s_139_0: bool = fn_state.gs_109073;
        // N s_139_1: branch s_139_0 b217 b140
        if s_139_0 {
            return block_217(state, tracer, fn_state);
        } else {
            return block_140(state, tracer, fn_state);
        };
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_140_0: const #0u : u8
        let s_140_0: bool = false;
        // D s_140_1: write-var gs#109074 <= s_140_0
        fn_state.gs_109074 = s_140_0;
        // N s_140_2: jump b141
        return block_141(state, tracer, fn_state);
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_141_0: read-var gs#109074:u8
        let s_141_0: bool = fn_state.gs_109074;
        // N s_141_1: branch s_141_0 b216 b142
        if s_141_0 {
            return block_216(state, tracer, fn_state);
        } else {
            return block_142(state, tracer, fn_state);
        };
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_142_0: const #() : ()
        let s_142_0: () = ();
        // S s_142_1: call EL2Enabled(s_142_0)
        let s_142_1: bool = EL2Enabled(state, tracer, s_142_0);
        // N s_142_2: branch s_142_1 b215 b143
        if s_142_1 {
            return block_215(state, tracer, fn_state);
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
        // D s_143_1: write-var gs#109075 <= s_143_0
        fn_state.gs_109075 = s_143_0;
        // N s_143_2: jump b144
        return block_144(state, tracer, fn_state);
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_144_0: read-var gs#109075:u8
        let s_144_0: bool = fn_state.gs_109075;
        // N s_144_1: branch s_144_0 b214 b145
        if s_144_0 {
            return block_214(state, tracer, fn_state);
        } else {
            return block_145(state, tracer, fn_state);
        };
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_145_0: const #0u : u8
        let s_145_0: bool = false;
        // D s_145_1: write-var gs#109076 <= s_145_0
        fn_state.gs_109076 = s_145_0;
        // N s_145_2: jump b146
        return block_146(state, tracer, fn_state);
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_146_0: read-var gs#109076:u8
        let s_146_0: bool = fn_state.gs_109076;
        // N s_146_1: branch s_146_0 b213 b147
        if s_146_0 {
            return block_213(state, tracer, fn_state);
        } else {
            return block_147(state, tracer, fn_state);
        };
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_147_0: const #() : ()
        let s_147_0: () = ();
        // S s_147_1: call EL2Enabled(s_147_0)
        let s_147_1: bool = EL2Enabled(state, tracer, s_147_0);
        // N s_147_2: branch s_147_1 b212 b148
        if s_147_1 {
            return block_212(state, tracer, fn_state);
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
        // D s_148_1: write-var gs#109077 <= s_148_0
        fn_state.gs_109077 = s_148_0;
        // N s_148_2: jump b149
        return block_149(state, tracer, fn_state);
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_149_0: read-var gs#109077:u8
        let s_149_0: bool = fn_state.gs_109077;
        // N s_149_1: branch s_149_0 b211 b150
        if s_149_0 {
            return block_211(state, tracer, fn_state);
        } else {
            return block_150(state, tracer, fn_state);
        };
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_150_0: const #0u : u8
        let s_150_0: bool = false;
        // D s_150_1: write-var gs#109078 <= s_150_0
        fn_state.gs_109078 = s_150_0;
        // N s_150_2: jump b151
        return block_151(state, tracer, fn_state);
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_151_0: read-var gs#109078:u8
        let s_151_0: bool = fn_state.gs_109078;
        // N s_151_1: branch s_151_0 b210 b152
        if s_151_0 {
            return block_210(state, tracer, fn_state);
        } else {
            return block_152(state, tracer, fn_state);
        };
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_152_0: const #() : ()
        let s_152_0: () = ();
        // S s_152_1: call EL2Enabled(s_152_0)
        let s_152_1: bool = EL2Enabled(state, tracer, s_152_0);
        // N s_152_2: branch s_152_1 b209 b153
        if s_152_1 {
            return block_209(state, tracer, fn_state);
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
        // D s_153_1: write-var gs#109079 <= s_153_0
        fn_state.gs_109079 = s_153_0;
        // N s_153_2: jump b154
        return block_154(state, tracer, fn_state);
    }
    fn block_154<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_154_0: read-var gs#109079:u8
        let s_154_0: bool = fn_state.gs_109079;
        // N s_154_1: branch s_154_0 b208 b155
        if s_154_0 {
            return block_208(state, tracer, fn_state);
        } else {
            return block_155(state, tracer, fn_state);
        };
    }
    fn block_155<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_155_0: const #0u : u8
        let s_155_0: bool = false;
        // D s_155_1: write-var gs#109080 <= s_155_0
        fn_state.gs_109080 = s_155_0;
        // N s_155_2: jump b156
        return block_156(state, tracer, fn_state);
    }
    fn block_156<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_156_0: read-var gs#109080:u8
        let s_156_0: bool = fn_state.gs_109080;
        // N s_156_1: branch s_156_0 b207 b157
        if s_156_0 {
            return block_207(state, tracer, fn_state);
        } else {
            return block_157(state, tracer, fn_state);
        };
    }
    fn block_157<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_157_0: const #() : ()
        let s_157_0: () = ();
        // S s_157_1: call EL2Enabled(s_157_0)
        let s_157_1: bool = EL2Enabled(state, tracer, s_157_0);
        // N s_157_2: branch s_157_1 b206 b158
        if s_157_1 {
            return block_206(state, tracer, fn_state);
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
        // D s_158_1: write-var gs#109081 <= s_158_0
        fn_state.gs_109081 = s_158_0;
        // N s_158_2: jump b159
        return block_159(state, tracer, fn_state);
    }
    fn block_159<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_159_0: read-var gs#109081:u8
        let s_159_0: bool = fn_state.gs_109081;
        // N s_159_1: branch s_159_0 b205 b160
        if s_159_0 {
            return block_205(state, tracer, fn_state);
        } else {
            return block_160(state, tracer, fn_state);
        };
    }
    fn block_160<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_160_0: const #0u : u8
        let s_160_0: bool = false;
        // D s_160_1: write-var gs#109082 <= s_160_0
        fn_state.gs_109082 = s_160_0;
        // N s_160_2: jump b161
        return block_161(state, tracer, fn_state);
    }
    fn block_161<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_161_0: read-var gs#109082:u8
        let s_161_0: bool = fn_state.gs_109082;
        // N s_161_1: branch s_161_0 b204 b162
        if s_161_0 {
            return block_204(state, tracer, fn_state);
        } else {
            return block_162(state, tracer, fn_state);
        };
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
        // N s_162_4: branch s_162_3 b203 b163
        if s_162_3 {
            return block_203(state, tracer, fn_state);
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
        // D s_163_1: write-var gs#109083 <= s_163_0
        fn_state.gs_109083 = s_163_0;
        // N s_163_2: jump b164
        return block_164(state, tracer, fn_state);
    }
    fn block_164<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_164_0: read-var gs#109083:u8
        let s_164_0: bool = fn_state.gs_109083;
        // N s_164_1: branch s_164_0 b202 b165
        if s_164_0 {
            return block_202(state, tracer, fn_state);
        } else {
            return block_165(state, tracer, fn_state);
        };
    }
    fn block_165<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_165_0: const #0u : u8
        let s_165_0: bool = false;
        // D s_165_1: write-var gs#109084 <= s_165_0
        fn_state.gs_109084 = s_165_0;
        // N s_165_2: jump b166
        return block_166(state, tracer, fn_state);
    }
    fn block_166<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_166_0: read-var gs#109084:u8
        let s_166_0: bool = fn_state.gs_109084;
        // N s_166_1: branch s_166_0 b196 b167
        if s_166_0 {
            return block_196(state, tracer, fn_state);
        } else {
            return block_167(state, tracer, fn_state);
        };
    }
    fn block_167<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_167_0: const #424u : u32
        let s_167_0: u32 = 424;
        // D s_167_1: read-reg s_167_0:u8
        let s_167_1: u8 = {
            let value = state.read_register::<u8>(s_167_0 as isize);
            tracer.read_register(s_167_0 as isize, value);
            value
        };
        // C s_167_2: const #2u : u8
        let s_167_2: u8 = 2;
        // D s_167_3: cmp-lt s_167_1 s_167_2
        let s_167_3: bool = ((s_167_1) < (s_167_2));
        // N s_167_4: branch s_167_3 b195 b168
        if s_167_3 {
            return block_195(state, tracer, fn_state);
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
        // D s_168_1: write-var gs#109085 <= s_168_0
        fn_state.gs_109085 = s_168_0;
        // N s_168_2: jump b169
        return block_169(state, tracer, fn_state);
    }
    fn block_169<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_169_0: read-var gs#109085:u8
        let s_169_0: bool = fn_state.gs_109085;
        // N s_169_1: branch s_169_0 b194 b170
        if s_169_0 {
            return block_194(state, tracer, fn_state);
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
        // D s_170_1: write-var gs#109086 <= s_170_0
        fn_state.gs_109086 = s_170_0;
        // N s_170_2: jump b171
        return block_171(state, tracer, fn_state);
    }
    fn block_171<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_171_0: read-var gs#109086:u8
        let s_171_0: bool = fn_state.gs_109086;
        // N s_171_1: branch s_171_0 b188 b172
        if s_171_0 {
            return block_188(state, tracer, fn_state);
        } else {
            return block_172(state, tracer, fn_state);
        };
    }
    fn block_172<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_172_0: const #424u : u32
        let s_172_0: u32 = 424;
        // D s_172_1: read-reg s_172_0:u8
        let s_172_1: u8 = {
            let value = state.read_register::<u8>(s_172_0 as isize);
            tracer.read_register(s_172_0 as isize, value);
            value
        };
        // C s_172_2: const #2u : u8
        let s_172_2: u8 = 2;
        // D s_172_3: cmp-lt s_172_1 s_172_2
        let s_172_3: bool = ((s_172_1) < (s_172_2));
        // N s_172_4: branch s_172_3 b187 b173
        if s_172_3 {
            return block_187(state, tracer, fn_state);
        } else {
            return block_173(state, tracer, fn_state);
        };
    }
    fn block_173<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_173_0: const #0u : u8
        let s_173_0: bool = false;
        // D s_173_1: write-var gs#109087 <= s_173_0
        fn_state.gs_109087 = s_173_0;
        // N s_173_2: jump b174
        return block_174(state, tracer, fn_state);
    }
    fn block_174<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_174_0: read-var gs#109087:u8
        let s_174_0: bool = fn_state.gs_109087;
        // N s_174_1: branch s_174_0 b186 b175
        if s_174_0 {
            return block_186(state, tracer, fn_state);
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
        // D s_175_1: write-var gs#109088 <= s_175_0
        fn_state.gs_109088 = s_175_0;
        // N s_175_2: jump b176
        return block_176(state, tracer, fn_state);
    }
    fn block_176<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_176_0: read-var gs#109088:u8
        let s_176_0: bool = fn_state.gs_109088;
        // N s_176_1: branch s_176_0 b180 b177
        if s_176_0 {
            return block_180(state, tracer, fn_state);
        } else {
            return block_177(state, tracer, fn_state);
        };
    }
    fn block_177<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_177_0: const #15s : i
        let s_177_0: i128 = 15;
        // D s_177_1: read-var t:i
        let s_177_1: i128 = fn_state.t;
        // D s_177_2: cmp-eq s_177_1 s_177_0
        let s_177_2: bool = ((s_177_1) == (s_177_0));
        // N s_177_3: branch s_177_2 b179 b178
        if s_177_2 {
            return block_179(state, tracer, fn_state);
        } else {
            return block_178(state, tracer, fn_state);
        };
    }
    fn block_178<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_178_0: const #() : ()
        let s_178_0: () = ();
        // S s_178_1: call DBGDSCRint_read(s_178_0)
        let s_178_1: ProductType700c18a878c5601b = DBGDSCRint_read(
            state,
            tracer,
            s_178_0,
        );
        // S s_178_2: call __get_DBGDSCRint(s_178_1)
        let s_178_2: ProductType700c18a878c5601b = u__get_DBGDSCRint(
            state,
            tracer,
            s_178_1,
        );
        // D s_178_3: write-var ga#172789 <= s_178_2
        fn_state.ga_172789 = s_178_2;
        // D s_178_4: read-var ga#172789.0:struct
        let s_178_4: u32 = fn_state.ga_172789._0;
        // D s_178_5: read-var t:i
        let s_178_5: i128 = fn_state.t;
        // D s_178_6: call R_set(s_178_5, s_178_4)
        let s_178_6: () = R_set(state, tracer, s_178_5, s_178_4);
        // N s_178_7: return
        return;
    }
    fn block_179<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_179_0: const #() : ()
        let s_179_0: () = ();
        // S s_179_1: call DBGDSCRint_read(s_179_0)
        let s_179_1: ProductType700c18a878c5601b = DBGDSCRint_read(
            state,
            tracer,
            s_179_0,
        );
        // D s_179_2: write-var ga#172786 <= s_179_1
        fn_state.ga_172786 = s_179_1;
        // D s_179_3: read-var ga#172786.0:struct
        let s_179_3: u32 = fn_state.ga_172786._0;
        // C s_179_4: const #28s : i
        let s_179_4: i128 = 28;
        // D s_179_5: cast zx s_179_3 -> bv
        let s_179_5: Bits = Bits::new(s_179_3 as u128, 32u16);
        // C s_179_6: const #1s : i64
        let s_179_6: i64 = 1;
        // C s_179_7: cast zx s_179_6 -> i
        let s_179_7: i128 = (i128::try_from(s_179_6).unwrap());
        // C s_179_8: const #3s : i
        let s_179_8: i128 = 3;
        // C s_179_9: add s_179_8 s_179_7
        let s_179_9: i128 = (s_179_8 + s_179_7);
        // D s_179_10: bit-extract s_179_5 s_179_4 s_179_9
        let s_179_10: Bits = (Bits::new(
            ((s_179_5) >> (s_179_4)).value(),
            u16::try_from(s_179_9).unwrap(),
        ));
        // D s_179_11: cast reint s_179_10 -> u8
        let s_179_11: u8 = (s_179_10.value() as u8);
        // C s_179_12: const #3s : i
        let s_179_12: i128 = 3;
        // D s_179_13: cast zx s_179_11 -> bv
        let s_179_13: Bits = Bits::new(s_179_11 as u128, 4u16);
        // C s_179_14: const #1s : i64
        let s_179_14: i64 = 1;
        // C s_179_15: cast zx s_179_14 -> i
        let s_179_15: i128 = (i128::try_from(s_179_14).unwrap());
        // C s_179_16: const #0s : i
        let s_179_16: i128 = 0;
        // C s_179_17: add s_179_16 s_179_15
        let s_179_17: i128 = (s_179_16 + s_179_15);
        // D s_179_18: bit-extract s_179_13 s_179_12 s_179_17
        let s_179_18: Bits = (Bits::new(
            ((s_179_13) >> (s_179_12)).value(),
            u16::try_from(s_179_17).unwrap(),
        ));
        // D s_179_19: cast reint s_179_18 -> u8
        let s_179_19: bool = ((s_179_18.value()) != 0);
        // C s_179_20: const #16984u : u32
        let s_179_20: u32 = 16984;
        // N s_179_21: write-reg s_179_20 <= s_179_19
        let s_179_21: () = {
            state.write_register::<bool>(s_179_20 as isize, s_179_19);
            tracer.write_register(s_179_20 as isize, s_179_19);
        };
        // C s_179_22: const #2s : i
        let s_179_22: i128 = 2;
        // D s_179_23: cast zx s_179_11 -> bv
        let s_179_23: Bits = Bits::new(s_179_11 as u128, 4u16);
        // C s_179_24: const #1s : i64
        let s_179_24: i64 = 1;
        // C s_179_25: cast zx s_179_24 -> i
        let s_179_25: i128 = (i128::try_from(s_179_24).unwrap());
        // C s_179_26: const #0s : i
        let s_179_26: i128 = 0;
        // C s_179_27: add s_179_26 s_179_25
        let s_179_27: i128 = (s_179_26 + s_179_25);
        // D s_179_28: bit-extract s_179_23 s_179_22 s_179_27
        let s_179_28: Bits = (Bits::new(
            ((s_179_23) >> (s_179_22)).value(),
            u16::try_from(s_179_27).unwrap(),
        ));
        // D s_179_29: cast reint s_179_28 -> u8
        let s_179_29: bool = ((s_179_28.value()) != 0);
        // C s_179_30: const #16997u : u32
        let s_179_30: u32 = 16997;
        // N s_179_31: write-reg s_179_30 <= s_179_29
        let s_179_31: () = {
            state.write_register::<bool>(s_179_30 as isize, s_179_29);
            tracer.write_register(s_179_30 as isize, s_179_29);
        };
        // C s_179_32: const #1s : i
        let s_179_32: i128 = 1;
        // D s_179_33: cast zx s_179_11 -> bv
        let s_179_33: Bits = Bits::new(s_179_11 as u128, 4u16);
        // C s_179_34: const #1s : i64
        let s_179_34: i64 = 1;
        // C s_179_35: cast zx s_179_34 -> i
        let s_179_35: i128 = (i128::try_from(s_179_34).unwrap());
        // C s_179_36: const #0s : i
        let s_179_36: i128 = 0;
        // C s_179_37: add s_179_36 s_179_35
        let s_179_37: i128 = (s_179_36 + s_179_35);
        // D s_179_38: bit-extract s_179_33 s_179_32 s_179_37
        let s_179_38: Bits = (Bits::new(
            ((s_179_33) >> (s_179_32)).value(),
            u16::try_from(s_179_37).unwrap(),
        ));
        // D s_179_39: cast reint s_179_38 -> u8
        let s_179_39: bool = ((s_179_38.value()) != 0);
        // C s_179_40: const #16971u : u32
        let s_179_40: u32 = 16971;
        // N s_179_41: write-reg s_179_40 <= s_179_39
        let s_179_41: () = {
            state.write_register::<bool>(s_179_40 as isize, s_179_39);
            tracer.write_register(s_179_40 as isize, s_179_39);
        };
        // C s_179_42: const #0s : i
        let s_179_42: i128 = 0;
        // D s_179_43: cast zx s_179_11 -> bv
        let s_179_43: Bits = Bits::new(s_179_11 as u128, 4u16);
        // C s_179_44: const #1s : i64
        let s_179_44: i64 = 1;
        // C s_179_45: cast zx s_179_44 -> i
        let s_179_45: i128 = (i128::try_from(s_179_44).unwrap());
        // C s_179_46: const #0s : i
        let s_179_46: i128 = 0;
        // C s_179_47: add s_179_46 s_179_45
        let s_179_47: i128 = (s_179_46 + s_179_45);
        // D s_179_48: bit-extract s_179_43 s_179_42 s_179_47
        let s_179_48: Bits = (Bits::new(
            ((s_179_43) >> (s_179_42)).value(),
            u16::try_from(s_179_47).unwrap(),
        ));
        // D s_179_49: cast reint s_179_48 -> u8
        let s_179_49: bool = ((s_179_48.value()) != 0);
        // C s_179_50: const #16996u : u32
        let s_179_50: u32 = 16996;
        // N s_179_51: write-reg s_179_50 <= s_179_49
        let s_179_51: () = {
            state.write_register::<bool>(s_179_50 as isize, s_179_49);
            tracer.write_register(s_179_50 as isize, s_179_49);
        };
        // N s_179_52: return
        return;
    }
    fn block_180<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_180_0: const #() : ()
        let s_180_0: () = ();
        // S s_180_1: call Halted(s_180_0)
        let s_180_1: bool = Halted(state, tracer, s_180_0);
        // N s_180_2: branch s_180_1 b185 b181
        if s_180_1 {
            return block_185(state, tracer, fn_state);
        } else {
            return block_181(state, tracer, fn_state);
        };
    }
    fn block_181<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_181_0: const #0u : u8
        let s_181_0: bool = false;
        // D s_181_1: write-var gs#109103 <= s_181_0
        fn_state.gs_109103 = s_181_0;
        // N s_181_2: jump b182
        return block_182(state, tracer, fn_state);
    }
    fn block_182<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_182_0: read-var gs#109103:u8
        let s_182_0: bool = fn_state.gs_109103;
        // N s_182_1: branch s_182_0 b184 b183
        if s_182_0 {
            return block_184(state, tracer, fn_state);
        } else {
            return block_183(state, tracer, fn_state);
        };
    }
    fn block_183<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_183_0: const #5u : u8
        let s_183_0: u8 = 5;
        // C s_183_1: cast zx s_183_0 -> bv
        let s_183_1: Bits = Bits::new(s_183_0 as u128, 8u16);
        // C s_183_2: cast zx s_183_1 -> i
        let s_183_2: i128 = (s_183_1.value() as i128);
        // C s_183_3: cast reint s_183_2 -> i64
        let s_183_3: i64 = (s_183_2 as i64);
        // C s_183_4: cast zx s_183_3 -> i
        let s_183_4: i128 = (i128::try_from(s_183_3).unwrap());
        // C s_183_5: const #424u : u32
        let s_183_5: u32 = 424;
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
        // N s_184_0: panic
        panic!("{:?}", ());
        // N s_184_1: return
        return;
    }
    fn block_185<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_185_0: const #() : ()
        let s_185_0: () = ();
        // S s_185_1: call EDSCR_read(s_185_0)
        let s_185_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_185_0);
        // S s_185_2: call _get_EDSCR_Type_SDD(s_185_1)
        let s_185_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_185_1);
        // S s_185_3: cast zx s_185_2 -> bv
        let s_185_3: Bits = Bits::new(s_185_2 as u128, 1u16);
        // C s_185_4: const #1u : u8
        let s_185_4: bool = true;
        // C s_185_5: cast zx s_185_4 -> bv
        let s_185_5: Bits = Bits::new(s_185_4 as u128, 1u16);
        // S s_185_6: cmp-eq s_185_3 s_185_5
        let s_185_6: bool = ((s_185_3) == (s_185_5));
        // D s_185_7: write-var gs#109103 <= s_185_6
        fn_state.gs_109103 = s_185_6;
        // N s_185_8: jump b182
        return block_182(state, tracer, fn_state);
    }
    fn block_186<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_186_0: read-var __MDCR_EL3_TDA:u8
        let s_186_0: bool = fn_state.u__MDCR_EL3_TDA;
        // D s_186_1: cast zx s_186_0 -> bv
        let s_186_1: Bits = Bits::new(s_186_0 as u128, 1u16);
        // C s_186_2: const #1u : u8
        let s_186_2: bool = true;
        // C s_186_3: cast zx s_186_2 -> bv
        let s_186_3: Bits = Bits::new(s_186_2 as u128, 1u16);
        // D s_186_4: cmp-eq s_186_1 s_186_3
        let s_186_4: bool = ((s_186_1) == (s_186_3));
        // D s_186_5: write-var gs#109088 <= s_186_4
        fn_state.gs_109088 = s_186_4;
        // N s_186_6: jump b176
        return block_176(state, tracer, fn_state);
    }
    fn block_187<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_187_0: const #424u : u32
        let s_187_0: u32 = 424;
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
        // D s_187_4: write-var gs#109087 <= s_187_3
        fn_state.gs_109087 = s_187_3;
        // N s_187_5: jump b174
        return block_174(state, tracer, fn_state);
    }
    fn block_188<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_188_0: const #() : ()
        let s_188_0: () = ();
        // S s_188_1: call Halted(s_188_0)
        let s_188_1: bool = Halted(state, tracer, s_188_0);
        // N s_188_2: branch s_188_1 b193 b189
        if s_188_1 {
            return block_193(state, tracer, fn_state);
        } else {
            return block_189(state, tracer, fn_state);
        };
    }
    fn block_189<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_189_0: const #0u : u8
        let s_189_0: bool = false;
        // D s_189_1: write-var gs#109104 <= s_189_0
        fn_state.gs_109104 = s_189_0;
        // N s_189_2: jump b190
        return block_190(state, tracer, fn_state);
    }
    fn block_190<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_190_0: read-var gs#109104:u8
        let s_190_0: bool = fn_state.gs_109104;
        // N s_190_1: branch s_190_0 b192 b191
        if s_190_0 {
            return block_192(state, tracer, fn_state);
        } else {
            return block_191(state, tracer, fn_state);
        };
    }
    fn block_191<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_191_0: const #() : ()
        let s_191_0: () = ();
        // S s_191_1: call AArch32_TakeMonitorTrapException(s_191_0)
        let s_191_1: () = AArch32_TakeMonitorTrapException(state, tracer, s_191_0);
        // N s_191_2: return
        return;
    }
    fn block_192<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_192_0: panic
        panic!("{:?}", ());
        // N s_192_1: return
        return;
    }
    fn block_193<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_193_0: const #() : ()
        let s_193_0: () = ();
        // S s_193_1: call EDSCR_read(s_193_0)
        let s_193_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_193_0);
        // S s_193_2: call _get_EDSCR_Type_SDD(s_193_1)
        let s_193_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_193_1);
        // S s_193_3: cast zx s_193_2 -> bv
        let s_193_3: Bits = Bits::new(s_193_2 as u128, 1u16);
        // C s_193_4: const #1u : u8
        let s_193_4: bool = true;
        // C s_193_5: cast zx s_193_4 -> bv
        let s_193_5: Bits = Bits::new(s_193_4 as u128, 1u16);
        // S s_193_6: cmp-eq s_193_3 s_193_5
        let s_193_6: bool = ((s_193_3) == (s_193_5));
        // D s_193_7: write-var gs#109104 <= s_193_6
        fn_state.gs_109104 = s_193_6;
        // N s_193_8: jump b190
        return block_190(state, tracer, fn_state);
    }
    fn block_194<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_194_0: read-var __SDCR_TDCC:u8
        let s_194_0: bool = fn_state.u__SDCR_TDCC;
        // D s_194_1: cast zx s_194_0 -> bv
        let s_194_1: Bits = Bits::new(s_194_0 as u128, 1u16);
        // C s_194_2: const #1u : u8
        let s_194_2: bool = true;
        // C s_194_3: cast zx s_194_2 -> bv
        let s_194_3: Bits = Bits::new(s_194_2 as u128, 1u16);
        // D s_194_4: cmp-eq s_194_1 s_194_3
        let s_194_4: bool = ((s_194_1) == (s_194_3));
        // D s_194_5: write-var gs#109086 <= s_194_4
        fn_state.gs_109086 = s_194_4;
        // N s_194_6: jump b171
        return block_171(state, tracer, fn_state);
    }
    fn block_195<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_195_0: const #424u : u32
        let s_195_0: u32 = 424;
        // D s_195_1: read-reg s_195_0:u8
        let s_195_1: u8 = {
            let value = state.read_register::<u8>(s_195_0 as isize);
            tracer.read_register(s_195_0 as isize, value);
            value
        };
        // D s_195_2: call ELUsingAArch32(s_195_1)
        let s_195_2: bool = ELUsingAArch32(state, tracer, s_195_1);
        // D s_195_3: write-var gs#109085 <= s_195_2
        fn_state.gs_109085 = s_195_2;
        // N s_195_4: jump b169
        return block_169(state, tracer, fn_state);
    }
    fn block_196<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_196_0: const #() : ()
        let s_196_0: () = ();
        // S s_196_1: call Halted(s_196_0)
        let s_196_1: bool = Halted(state, tracer, s_196_0);
        // N s_196_2: branch s_196_1 b201 b197
        if s_196_1 {
            return block_201(state, tracer, fn_state);
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
        // D s_197_1: write-var gs#109105 <= s_197_0
        fn_state.gs_109105 = s_197_0;
        // N s_197_2: jump b198
        return block_198(state, tracer, fn_state);
    }
    fn block_198<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_198_0: read-var gs#109105:u8
        let s_198_0: bool = fn_state.gs_109105;
        // N s_198_1: branch s_198_0 b200 b199
        if s_198_0 {
            return block_200(state, tracer, fn_state);
        } else {
            return block_199(state, tracer, fn_state);
        };
    }
    fn block_199<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_199_0: const #5u : u8
        let s_199_0: u8 = 5;
        // C s_199_1: cast zx s_199_0 -> bv
        let s_199_1: Bits = Bits::new(s_199_0 as u128, 8u16);
        // C s_199_2: cast zx s_199_1 -> i
        let s_199_2: i128 = (s_199_1.value() as i128);
        // C s_199_3: cast reint s_199_2 -> i64
        let s_199_3: i64 = (s_199_2 as i64);
        // C s_199_4: cast zx s_199_3 -> i
        let s_199_4: i128 = (i128::try_from(s_199_3).unwrap());
        // C s_199_5: const #424u : u32
        let s_199_5: u32 = 424;
        // D s_199_6: read-reg s_199_5:u8
        let s_199_6: u8 = {
            let value = state.read_register::<u8>(s_199_5 as isize);
            tracer.read_register(s_199_5 as isize, value);
            value
        };
        // D s_199_7: call AArch64_AArch32SystemAccessTrap(s_199_6, s_199_4)
        let s_199_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_199_6,
            s_199_4,
        );
        // N s_199_8: return
        return;
    }
    fn block_200<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_200_0: panic
        panic!("{:?}", ());
        // N s_200_1: return
        return;
    }
    fn block_201<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_201_0: const #() : ()
        let s_201_0: () = ();
        // S s_201_1: call EDSCR_read(s_201_0)
        let s_201_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_201_0);
        // S s_201_2: call _get_EDSCR_Type_SDD(s_201_1)
        let s_201_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_201_1);
        // S s_201_3: cast zx s_201_2 -> bv
        let s_201_3: Bits = Bits::new(s_201_2 as u128, 1u16);
        // C s_201_4: const #1u : u8
        let s_201_4: bool = true;
        // C s_201_5: cast zx s_201_4 -> bv
        let s_201_5: Bits = Bits::new(s_201_4 as u128, 1u16);
        // S s_201_6: cmp-eq s_201_3 s_201_5
        let s_201_6: bool = ((s_201_3) == (s_201_5));
        // D s_201_7: write-var gs#109105 <= s_201_6
        fn_state.gs_109105 = s_201_6;
        // N s_201_8: jump b198
        return block_198(state, tracer, fn_state);
    }
    fn block_202<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_202_0: read-var __MDCR_EL3_TDCC:u8
        let s_202_0: bool = fn_state.u__MDCR_EL3_TDCC;
        // D s_202_1: cast zx s_202_0 -> bv
        let s_202_1: Bits = Bits::new(s_202_0 as u128, 1u16);
        // C s_202_2: const #1u : u8
        let s_202_2: bool = true;
        // C s_202_3: cast zx s_202_2 -> bv
        let s_202_3: Bits = Bits::new(s_202_2 as u128, 1u16);
        // D s_202_4: cmp-eq s_202_1 s_202_3
        let s_202_4: bool = ((s_202_1) == (s_202_3));
        // D s_202_5: write-var gs#109084 <= s_202_4
        fn_state.gs_109084 = s_202_4;
        // N s_202_6: jump b166
        return block_166(state, tracer, fn_state);
    }
    fn block_203<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_203_0: const #424u : u32
        let s_203_0: u32 = 424;
        // D s_203_1: read-reg s_203_0:u8
        let s_203_1: u8 = {
            let value = state.read_register::<u8>(s_203_0 as isize);
            tracer.read_register(s_203_0 as isize, value);
            value
        };
        // D s_203_2: call ELUsingAArch32(s_203_1)
        let s_203_2: bool = ELUsingAArch32(state, tracer, s_203_1);
        // D s_203_3: not s_203_2
        let s_203_3: bool = !s_203_2;
        // D s_203_4: write-var gs#109083 <= s_203_3
        fn_state.gs_109083 = s_203_3;
        // N s_203_5: jump b164
        return block_164(state, tracer, fn_state);
    }
    fn block_204<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_204_0: const #5u : u8
        let s_204_0: u8 = 5;
        // C s_204_1: cast zx s_204_0 -> bv
        let s_204_1: Bits = Bits::new(s_204_0 as u128, 8u16);
        // C s_204_2: cast zx s_204_1 -> i
        let s_204_2: i128 = (s_204_1.value() as i128);
        // C s_204_3: cast reint s_204_2 -> i64
        let s_204_3: i64 = (s_204_2 as i64);
        // C s_204_4: cast zx s_204_3 -> i
        let s_204_4: i128 = (i128::try_from(s_204_3).unwrap());
        // S s_204_5: call AArch32_TakeHypTrapException(s_204_4)
        let s_204_5: () = AArch32_TakeHypTrapException(state, tracer, s_204_4);
        // N s_204_6: return
        return;
    }
    fn block_205<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_205_0: const #() : ()
        let s_205_0: () = ();
        // S s_205_1: call HDCR_read(s_205_0)
        let s_205_1: ProductType700c18a878c5601b = HDCR_read(state, tracer, s_205_0);
        // S s_205_2: call _get_HDCR_Type_TDE(s_205_1)
        let s_205_2: bool = u_get_HDCR_Type_TDE(state, tracer, s_205_1);
        // C s_205_3: const #() : ()
        let s_205_3: () = ();
        // S s_205_4: call HDCR_read(s_205_3)
        let s_205_4: ProductType700c18a878c5601b = HDCR_read(state, tracer, s_205_3);
        // S s_205_5: call _get_HDCR_Type_TDA(s_205_4)
        let s_205_5: bool = u_get_HDCR_Type_TDA(state, tracer, s_205_4);
        // S s_205_6: cast zx s_205_2 -> bv
        let s_205_6: Bits = Bits::new(s_205_2 as u128, 1u16);
        // S s_205_7: cast zx s_205_5 -> bv
        let s_205_7: Bits = Bits::new(s_205_5 as u128, 1u16);
        // S s_205_8: cast reint s_205_6 -> u128
        let s_205_8: u128 = (s_205_6.value() as u128);
        // D s_205_9: size-of s_205_6
        let s_205_9: u16 = s_205_6.length();
        // S s_205_10: cast reint s_205_7 -> u128
        let s_205_10: u128 = (s_205_7.value() as u128);
        // D s_205_11: size-of s_205_7
        let s_205_11: u16 = s_205_7.length();
        // D s_205_12: lsl s_205_8 s_205_11
        let s_205_12: u128 = s_205_8 << s_205_11;
        // D s_205_13: or s_205_12 s_205_10
        let s_205_13: u128 = ((s_205_12) | (s_205_10));
        // D s_205_14: add s_205_9 s_205_11
        let s_205_14: u16 = (s_205_9 + s_205_11);
        // D s_205_15: create-bits s_205_13 s_205_14
        let s_205_15: Bits = Bits::new(s_205_13, s_205_14);
        // D s_205_16: cast reint s_205_15 -> u8
        let s_205_16: u8 = (s_205_15.value() as u8);
        // D s_205_17: cast zx s_205_16 -> bv
        let s_205_17: Bits = Bits::new(s_205_16 as u128, 2u16);
        // C s_205_18: const #0u : u8
        let s_205_18: u8 = 0;
        // C s_205_19: cast zx s_205_18 -> bv
        let s_205_19: Bits = Bits::new(s_205_18 as u128, 2u16);
        // D s_205_20: cmp-ne s_205_17 s_205_19
        let s_205_20: bool = ((s_205_17) != (s_205_19));
        // D s_205_21: write-var gs#109082 <= s_205_20
        fn_state.gs_109082 = s_205_20;
        // N s_205_22: jump b161
        return block_161(state, tracer, fn_state);
    }
    fn block_206<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_206_0: const #432u : u32
        let s_206_0: u32 = 432;
        // D s_206_1: read-reg s_206_0:u8
        let s_206_1: u8 = {
            let value = state.read_register::<u8>(s_206_0 as isize);
            tracer.read_register(s_206_0 as isize, value);
            value
        };
        // D s_206_2: call ELUsingAArch32(s_206_1)
        let s_206_2: bool = ELUsingAArch32(state, tracer, s_206_1);
        // D s_206_3: write-var gs#109081 <= s_206_2
        fn_state.gs_109081 = s_206_2;
        // N s_206_4: jump b159
        return block_159(state, tracer, fn_state);
    }
    fn block_207<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_207_0: const #5u : u8
        let s_207_0: u8 = 5;
        // C s_207_1: cast zx s_207_0 -> bv
        let s_207_1: Bits = Bits::new(s_207_0 as u128, 8u16);
        // C s_207_2: cast zx s_207_1 -> i
        let s_207_2: i128 = (s_207_1.value() as i128);
        // C s_207_3: cast reint s_207_2 -> i64
        let s_207_3: i64 = (s_207_2 as i64);
        // C s_207_4: cast zx s_207_3 -> i
        let s_207_4: i128 = (i128::try_from(s_207_3).unwrap());
        // C s_207_5: const #432u : u32
        let s_207_5: u32 = 432;
        // D s_207_6: read-reg s_207_5:u8
        let s_207_6: u8 = {
            let value = state.read_register::<u8>(s_207_5 as isize);
            tracer.read_register(s_207_5 as isize, value);
            value
        };
        // D s_207_7: call AArch64_AArch32SystemAccessTrap(s_207_6, s_207_4)
        let s_207_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_207_6,
            s_207_4,
        );
        // N s_207_8: return
        return;
    }
    fn block_208<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_208_0: const #104880u : u32
        let s_208_0: u32 = 104880;
        // D s_208_1: read-reg s_208_0:struct
        let s_208_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_208_0 as isize);
            tracer.read_register(s_208_0 as isize, value);
            value
        };
        // D s_208_2: call _get_MDCR_EL2_Type_TDE(s_208_1)
        let s_208_2: bool = u_get_MDCR_EL2_Type_TDE(state, tracer, s_208_1);
        // C s_208_3: const #104880u : u32
        let s_208_3: u32 = 104880;
        // D s_208_4: read-reg s_208_3:struct
        let s_208_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_208_3 as isize);
            tracer.read_register(s_208_3 as isize, value);
            value
        };
        // D s_208_5: call _get_MDCR_EL2_Type_TDA(s_208_4)
        let s_208_5: bool = u_get_MDCR_EL2_Type_TDA(state, tracer, s_208_4);
        // D s_208_6: cast zx s_208_2 -> bv
        let s_208_6: Bits = Bits::new(s_208_2 as u128, 1u16);
        // D s_208_7: cast zx s_208_5 -> bv
        let s_208_7: Bits = Bits::new(s_208_5 as u128, 1u16);
        // D s_208_8: cast reint s_208_6 -> u128
        let s_208_8: u128 = (s_208_6.value() as u128);
        // D s_208_9: size-of s_208_6
        let s_208_9: u16 = s_208_6.length();
        // D s_208_10: cast reint s_208_7 -> u128
        let s_208_10: u128 = (s_208_7.value() as u128);
        // D s_208_11: size-of s_208_7
        let s_208_11: u16 = s_208_7.length();
        // D s_208_12: lsl s_208_8 s_208_11
        let s_208_12: u128 = s_208_8 << s_208_11;
        // D s_208_13: or s_208_12 s_208_10
        let s_208_13: u128 = ((s_208_12) | (s_208_10));
        // D s_208_14: add s_208_9 s_208_11
        let s_208_14: u16 = (s_208_9 + s_208_11);
        // D s_208_15: create-bits s_208_13 s_208_14
        let s_208_15: Bits = Bits::new(s_208_13, s_208_14);
        // D s_208_16: cast reint s_208_15 -> u8
        let s_208_16: u8 = (s_208_15.value() as u8);
        // D s_208_17: cast zx s_208_16 -> bv
        let s_208_17: Bits = Bits::new(s_208_16 as u128, 2u16);
        // C s_208_18: const #0u : u8
        let s_208_18: u8 = 0;
        // C s_208_19: cast zx s_208_18 -> bv
        let s_208_19: Bits = Bits::new(s_208_18 as u128, 2u16);
        // D s_208_20: cmp-ne s_208_17 s_208_19
        let s_208_20: bool = ((s_208_17) != (s_208_19));
        // D s_208_21: write-var gs#109080 <= s_208_20
        fn_state.gs_109080 = s_208_20;
        // N s_208_22: jump b156
        return block_156(state, tracer, fn_state);
    }
    fn block_209<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_209_0: const #432u : u32
        let s_209_0: u32 = 432;
        // D s_209_1: read-reg s_209_0:u8
        let s_209_1: u8 = {
            let value = state.read_register::<u8>(s_209_0 as isize);
            tracer.read_register(s_209_0 as isize, value);
            value
        };
        // D s_209_2: call ELUsingAArch32(s_209_1)
        let s_209_2: bool = ELUsingAArch32(state, tracer, s_209_1);
        // D s_209_3: not s_209_2
        let s_209_3: bool = !s_209_2;
        // D s_209_4: write-var gs#109079 <= s_209_3
        fn_state.gs_109079 = s_209_3;
        // N s_209_5: jump b154
        return block_154(state, tracer, fn_state);
    }
    fn block_210<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_210_0: const #5u : u8
        let s_210_0: u8 = 5;
        // C s_210_1: cast zx s_210_0 -> bv
        let s_210_1: Bits = Bits::new(s_210_0 as u128, 8u16);
        // C s_210_2: cast zx s_210_1 -> i
        let s_210_2: i128 = (s_210_1.value() as i128);
        // C s_210_3: cast reint s_210_2 -> i64
        let s_210_3: i64 = (s_210_2 as i64);
        // C s_210_4: cast zx s_210_3 -> i
        let s_210_4: i128 = (i128::try_from(s_210_3).unwrap());
        // S s_210_5: call AArch32_TakeHypTrapException(s_210_4)
        let s_210_5: () = AArch32_TakeHypTrapException(state, tracer, s_210_4);
        // N s_210_6: return
        return;
    }
    fn block_211<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_211_0: read-var __HDCR_TDCC:u8
        let s_211_0: bool = fn_state.u__HDCR_TDCC;
        // D s_211_1: cast zx s_211_0 -> bv
        let s_211_1: Bits = Bits::new(s_211_0 as u128, 1u16);
        // C s_211_2: const #1u : u8
        let s_211_2: bool = true;
        // C s_211_3: cast zx s_211_2 -> bv
        let s_211_3: Bits = Bits::new(s_211_2 as u128, 1u16);
        // D s_211_4: cmp-eq s_211_1 s_211_3
        let s_211_4: bool = ((s_211_1) == (s_211_3));
        // D s_211_5: write-var gs#109078 <= s_211_4
        fn_state.gs_109078 = s_211_4;
        // N s_211_6: jump b151
        return block_151(state, tracer, fn_state);
    }
    fn block_212<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_212_0: const #432u : u32
        let s_212_0: u32 = 432;
        // D s_212_1: read-reg s_212_0:u8
        let s_212_1: u8 = {
            let value = state.read_register::<u8>(s_212_0 as isize);
            tracer.read_register(s_212_0 as isize, value);
            value
        };
        // D s_212_2: call ELUsingAArch32(s_212_1)
        let s_212_2: bool = ELUsingAArch32(state, tracer, s_212_1);
        // D s_212_3: write-var gs#109077 <= s_212_2
        fn_state.gs_109077 = s_212_2;
        // N s_212_4: jump b149
        return block_149(state, tracer, fn_state);
    }
    fn block_213<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_213_0: const #5u : u8
        let s_213_0: u8 = 5;
        // C s_213_1: cast zx s_213_0 -> bv
        let s_213_1: Bits = Bits::new(s_213_0 as u128, 8u16);
        // C s_213_2: cast zx s_213_1 -> i
        let s_213_2: i128 = (s_213_1.value() as i128);
        // C s_213_3: cast reint s_213_2 -> i64
        let s_213_3: i64 = (s_213_2 as i64);
        // C s_213_4: cast zx s_213_3 -> i
        let s_213_4: i128 = (i128::try_from(s_213_3).unwrap());
        // C s_213_5: const #432u : u32
        let s_213_5: u32 = 432;
        // D s_213_6: read-reg s_213_5:u8
        let s_213_6: u8 = {
            let value = state.read_register::<u8>(s_213_5 as isize);
            tracer.read_register(s_213_5 as isize, value);
            value
        };
        // D s_213_7: call AArch64_AArch32SystemAccessTrap(s_213_6, s_213_4)
        let s_213_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_213_6,
            s_213_4,
        );
        // N s_213_8: return
        return;
    }
    fn block_214<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_214_0: read-var __MDCR_EL2_TDCC:u8
        let s_214_0: bool = fn_state.u__MDCR_EL2_TDCC;
        // D s_214_1: cast zx s_214_0 -> bv
        let s_214_1: Bits = Bits::new(s_214_0 as u128, 1u16);
        // C s_214_2: const #1u : u8
        let s_214_2: bool = true;
        // C s_214_3: cast zx s_214_2 -> bv
        let s_214_3: Bits = Bits::new(s_214_2 as u128, 1u16);
        // D s_214_4: cmp-eq s_214_1 s_214_3
        let s_214_4: bool = ((s_214_1) == (s_214_3));
        // D s_214_5: write-var gs#109076 <= s_214_4
        fn_state.gs_109076 = s_214_4;
        // N s_214_6: jump b146
        return block_146(state, tracer, fn_state);
    }
    fn block_215<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_215_0: const #432u : u32
        let s_215_0: u32 = 432;
        // D s_215_1: read-reg s_215_0:u8
        let s_215_1: u8 = {
            let value = state.read_register::<u8>(s_215_0 as isize);
            tracer.read_register(s_215_0 as isize, value);
            value
        };
        // D s_215_2: call ELUsingAArch32(s_215_1)
        let s_215_2: bool = ELUsingAArch32(state, tracer, s_215_1);
        // D s_215_3: not s_215_2
        let s_215_3: bool = !s_215_2;
        // D s_215_4: write-var gs#109075 <= s_215_3
        fn_state.gs_109075 = s_215_3;
        // N s_215_5: jump b144
        return block_144(state, tracer, fn_state);
    }
    fn block_216<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_216_0: panic
        panic!("{:?}", ());
        // N s_216_1: return
        return;
    }
    fn block_217<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_217_0: read-var __MDCR_EL3_TDA:u8
        let s_217_0: bool = fn_state.u__MDCR_EL3_TDA;
        // D s_217_1: cast zx s_217_0 -> bv
        let s_217_1: Bits = Bits::new(s_217_0 as u128, 1u16);
        // C s_217_2: const #1u : u8
        let s_217_2: bool = true;
        // C s_217_3: cast zx s_217_2 -> bv
        let s_217_3: Bits = Bits::new(s_217_2 as u128, 1u16);
        // D s_217_4: cmp-eq s_217_1 s_217_3
        let s_217_4: bool = ((s_217_1) == (s_217_3));
        // D s_217_5: write-var gs#109074 <= s_217_4
        fn_state.gs_109074 = s_217_4;
        // N s_217_6: jump b141
        return block_141(state, tracer, fn_state);
    }
    fn block_218<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_218_0: const #424u : u32
        let s_218_0: u32 = 424;
        // D s_218_1: read-reg s_218_0:u8
        let s_218_1: u8 = {
            let value = state.read_register::<u8>(s_218_0 as isize);
            tracer.read_register(s_218_0 as isize, value);
            value
        };
        // D s_218_2: call ELUsingAArch32(s_218_1)
        let s_218_2: bool = ELUsingAArch32(state, tracer, s_218_1);
        // D s_218_3: not s_218_2
        let s_218_3: bool = !s_218_2;
        // D s_218_4: write-var gs#109073 <= s_218_3
        fn_state.gs_109073 = s_218_3;
        // N s_218_5: jump b139
        return block_139(state, tracer, fn_state);
    }
    fn block_219<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_219_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_219_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_219_1: call __IMPDEF_boolean(s_219_0)
        let s_219_1: bool = u__IMPDEF_boolean(state, tracer, s_219_0);
        // D s_219_2: write-var gs#109072 <= s_219_1
        fn_state.gs_109072 = s_219_1;
        // N s_219_3: jump b137
        return block_137(state, tracer, fn_state);
    }
    fn block_220<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_220_0: const #() : ()
        let s_220_0: () = ();
        // S s_220_1: call EDSCR_read(s_220_0)
        let s_220_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_220_0);
        // S s_220_2: call _get_EDSCR_Type_SDD(s_220_1)
        let s_220_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_220_1);
        // S s_220_3: cast zx s_220_2 -> bv
        let s_220_3: Bits = Bits::new(s_220_2 as u128, 1u16);
        // C s_220_4: const #1u : u8
        let s_220_4: bool = true;
        // C s_220_5: cast zx s_220_4 -> bv
        let s_220_5: Bits = Bits::new(s_220_4 as u128, 1u16);
        // S s_220_6: cmp-eq s_220_3 s_220_5
        let s_220_6: bool = ((s_220_3) == (s_220_5));
        // D s_220_7: write-var gs#109071 <= s_220_6
        fn_state.gs_109071 = s_220_6;
        // N s_220_8: jump b135
        return block_135(state, tracer, fn_state);
    }
    fn block_221<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_221_0: const #424u : u32
        let s_221_0: u32 = 424;
        // D s_221_1: read-reg s_221_0:u8
        let s_221_1: u8 = {
            let value = state.read_register::<u8>(s_221_0 as isize);
            tracer.read_register(s_221_0 as isize, value);
            value
        };
        // C s_221_2: const #2u : u8
        let s_221_2: u8 = 2;
        // D s_221_3: cmp-lt s_221_1 s_221_2
        let s_221_3: bool = ((s_221_1) < (s_221_2));
        // D s_221_4: write-var gs#109070 <= s_221_3
        fn_state.gs_109070 = s_221_3;
        // N s_221_5: jump b133
        return block_133(state, tracer, fn_state);
    }
    fn block_222<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_222_0: panic
        panic!("{:?}", ());
        // N s_222_1: return
        return;
    }
    fn block_223<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_223_0: read-var __SDCR_TDCC:u8
        let s_223_0: bool = fn_state.u__SDCR_TDCC;
        // D s_223_1: cast zx s_223_0 -> bv
        let s_223_1: Bits = Bits::new(s_223_0 as u128, 1u16);
        // C s_223_2: const #1u : u8
        let s_223_2: bool = true;
        // C s_223_3: cast zx s_223_2 -> bv
        let s_223_3: Bits = Bits::new(s_223_2 as u128, 1u16);
        // D s_223_4: cmp-eq s_223_1 s_223_3
        let s_223_4: bool = ((s_223_1) == (s_223_3));
        // D s_223_5: write-var gs#109069 <= s_223_4
        fn_state.gs_109069 = s_223_4;
        // N s_223_6: jump b130
        return block_130(state, tracer, fn_state);
    }
    fn block_224<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_224_0: const #424u : u32
        let s_224_0: u32 = 424;
        // D s_224_1: read-reg s_224_0:u8
        let s_224_1: u8 = {
            let value = state.read_register::<u8>(s_224_0 as isize);
            tracer.read_register(s_224_0 as isize, value);
            value
        };
        // D s_224_2: call ELUsingAArch32(s_224_1)
        let s_224_2: bool = ELUsingAArch32(state, tracer, s_224_1);
        // D s_224_3: write-var gs#109068 <= s_224_2
        fn_state.gs_109068 = s_224_2;
        // N s_224_4: jump b128
        return block_128(state, tracer, fn_state);
    }
    fn block_225<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_225_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_225_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_225_1: call __IMPDEF_boolean(s_225_0)
        let s_225_1: bool = u__IMPDEF_boolean(state, tracer, s_225_0);
        // D s_225_2: write-var gs#109067 <= s_225_1
        fn_state.gs_109067 = s_225_1;
        // N s_225_3: jump b126
        return block_126(state, tracer, fn_state);
    }
    fn block_226<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_226_0: const #() : ()
        let s_226_0: () = ();
        // S s_226_1: call EDSCR_read(s_226_0)
        let s_226_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_226_0);
        // S s_226_2: call _get_EDSCR_Type_SDD(s_226_1)
        let s_226_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_226_1);
        // S s_226_3: cast zx s_226_2 -> bv
        let s_226_3: Bits = Bits::new(s_226_2 as u128, 1u16);
        // C s_226_4: const #1u : u8
        let s_226_4: bool = true;
        // C s_226_5: cast zx s_226_4 -> bv
        let s_226_5: Bits = Bits::new(s_226_4 as u128, 1u16);
        // S s_226_6: cmp-eq s_226_3 s_226_5
        let s_226_6: bool = ((s_226_3) == (s_226_5));
        // D s_226_7: write-var gs#109066 <= s_226_6
        fn_state.gs_109066 = s_226_6;
        // N s_226_8: jump b124
        return block_124(state, tracer, fn_state);
    }
    fn block_227<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_227_0: const #424u : u32
        let s_227_0: u32 = 424;
        // D s_227_1: read-reg s_227_0:u8
        let s_227_1: u8 = {
            let value = state.read_register::<u8>(s_227_0 as isize);
            tracer.read_register(s_227_0 as isize, value);
            value
        };
        // C s_227_2: const #2u : u8
        let s_227_2: u8 = 2;
        // D s_227_3: cmp-lt s_227_1 s_227_2
        let s_227_3: bool = ((s_227_1) < (s_227_2));
        // D s_227_4: write-var gs#109065 <= s_227_3
        fn_state.gs_109065 = s_227_3;
        // N s_227_5: jump b122
        return block_122(state, tracer, fn_state);
    }
    fn block_228<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_228_0: panic
        panic!("{:?}", ());
        // N s_228_1: return
        return;
    }
    fn block_229<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_229_0: read-var __MDCR_EL3_TDCC:u8
        let s_229_0: bool = fn_state.u__MDCR_EL3_TDCC;
        // D s_229_1: cast zx s_229_0 -> bv
        let s_229_1: Bits = Bits::new(s_229_0 as u128, 1u16);
        // C s_229_2: const #1u : u8
        let s_229_2: bool = true;
        // C s_229_3: cast zx s_229_2 -> bv
        let s_229_3: Bits = Bits::new(s_229_2 as u128, 1u16);
        // D s_229_4: cmp-eq s_229_1 s_229_3
        let s_229_4: bool = ((s_229_1) == (s_229_3));
        // D s_229_5: write-var gs#109064 <= s_229_4
        fn_state.gs_109064 = s_229_4;
        // N s_229_6: jump b119
        return block_119(state, tracer, fn_state);
    }
    fn block_230<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_230_0: const #424u : u32
        let s_230_0: u32 = 424;
        // D s_230_1: read-reg s_230_0:u8
        let s_230_1: u8 = {
            let value = state.read_register::<u8>(s_230_0 as isize);
            tracer.read_register(s_230_0 as isize, value);
            value
        };
        // D s_230_2: call ELUsingAArch32(s_230_1)
        let s_230_2: bool = ELUsingAArch32(state, tracer, s_230_1);
        // D s_230_3: not s_230_2
        let s_230_3: bool = !s_230_2;
        // D s_230_4: write-var gs#109063 <= s_230_3
        fn_state.gs_109063 = s_230_3;
        // N s_230_5: jump b117
        return block_117(state, tracer, fn_state);
    }
    fn block_231<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_231_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_231_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_231_1: call __IMPDEF_boolean(s_231_0)
        let s_231_1: bool = u__IMPDEF_boolean(state, tracer, s_231_0);
        // D s_231_2: write-var gs#109062 <= s_231_1
        fn_state.gs_109062 = s_231_1;
        // N s_231_3: jump b115
        return block_115(state, tracer, fn_state);
    }
    fn block_232<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_232_0: const #() : ()
        let s_232_0: () = ();
        // S s_232_1: call EDSCR_read(s_232_0)
        let s_232_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_232_0);
        // S s_232_2: call _get_EDSCR_Type_SDD(s_232_1)
        let s_232_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_232_1);
        // S s_232_3: cast zx s_232_2 -> bv
        let s_232_3: Bits = Bits::new(s_232_2 as u128, 1u16);
        // C s_232_4: const #1u : u8
        let s_232_4: bool = true;
        // C s_232_5: cast zx s_232_4 -> bv
        let s_232_5: Bits = Bits::new(s_232_4 as u128, 1u16);
        // S s_232_6: cmp-eq s_232_3 s_232_5
        let s_232_6: bool = ((s_232_3) == (s_232_5));
        // D s_232_7: write-var gs#109061 <= s_232_6
        fn_state.gs_109061 = s_232_6;
        // N s_232_8: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_233<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_233_0: const #424u : u32
        let s_233_0: u32 = 424;
        // D s_233_1: read-reg s_233_0:u8
        let s_233_1: u8 = {
            let value = state.read_register::<u8>(s_233_0 as isize);
            tracer.read_register(s_233_0 as isize, value);
            value
        };
        // C s_233_2: const #2u : u8
        let s_233_2: u8 = 2;
        // D s_233_3: cmp-lt s_233_1 s_233_2
        let s_233_3: bool = ((s_233_1) < (s_233_2));
        // D s_233_4: write-var gs#109060 <= s_233_3
        fn_state.gs_109060 = s_233_3;
        // N s_233_5: jump b111
        return block_111(state, tracer, fn_state);
    }
    fn block_234<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_234_0: const #() : ()
        let s_234_0: () = ();
        // S s_234_1: call Halted(s_234_0)
        let s_234_1: bool = Halted(state, tracer, s_234_0);
        // N s_234_2: branch s_234_1 b398 b235
        if s_234_1 {
            return block_398(state, tracer, fn_state);
        } else {
            return block_235(state, tracer, fn_state);
        };
    }
    fn block_235<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_235_0: const #0u : u8
        let s_235_0: bool = false;
        // D s_235_1: write-var gs#109106 <= s_235_0
        fn_state.gs_109106 = s_235_0;
        // N s_235_2: jump b236
        return block_236(state, tracer, fn_state);
    }
    fn block_236<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_236_0: read-var gs#109106:u8
        let s_236_0: bool = fn_state.gs_109106;
        // N s_236_1: branch s_236_0 b397 b237
        if s_236_0 {
            return block_397(state, tracer, fn_state);
        } else {
            return block_237(state, tracer, fn_state);
        };
    }
    fn block_237<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_237_0: const #0u : u8
        let s_237_0: bool = false;
        // D s_237_1: write-var gs#109107 <= s_237_0
        fn_state.gs_109107 = s_237_0;
        // N s_237_2: jump b238
        return block_238(state, tracer, fn_state);
    }
    fn block_238<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_238_0: read-var gs#109107:u8
        let s_238_0: bool = fn_state.gs_109107;
        // N s_238_1: branch s_238_0 b396 b239
        if s_238_0 {
            return block_396(state, tracer, fn_state);
        } else {
            return block_239(state, tracer, fn_state);
        };
    }
    fn block_239<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_239_0: const #0u : u8
        let s_239_0: bool = false;
        // D s_239_1: write-var gs#109108 <= s_239_0
        fn_state.gs_109108 = s_239_0;
        // N s_239_2: jump b240
        return block_240(state, tracer, fn_state);
    }
    fn block_240<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_240_0: read-var gs#109108:u8
        let s_240_0: bool = fn_state.gs_109108;
        // N s_240_1: branch s_240_0 b395 b241
        if s_240_0 {
            return block_395(state, tracer, fn_state);
        } else {
            return block_241(state, tracer, fn_state);
        };
    }
    fn block_241<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_241_0: const #0u : u8
        let s_241_0: bool = false;
        // D s_241_1: write-var gs#109109 <= s_241_0
        fn_state.gs_109109 = s_241_0;
        // N s_241_2: jump b242
        return block_242(state, tracer, fn_state);
    }
    fn block_242<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_242_0: read-var gs#109109:u8
        let s_242_0: bool = fn_state.gs_109109;
        // N s_242_1: branch s_242_0 b394 b243
        if s_242_0 {
            return block_394(state, tracer, fn_state);
        } else {
            return block_243(state, tracer, fn_state);
        };
    }
    fn block_243<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_243_0: const #0u : u8
        let s_243_0: bool = false;
        // D s_243_1: write-var gs#109110 <= s_243_0
        fn_state.gs_109110 = s_243_0;
        // N s_243_2: jump b244
        return block_244(state, tracer, fn_state);
    }
    fn block_244<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_244_0: read-var gs#109110:u8
        let s_244_0: bool = fn_state.gs_109110;
        // N s_244_1: branch s_244_0 b393 b245
        if s_244_0 {
            return block_393(state, tracer, fn_state);
        } else {
            return block_245(state, tracer, fn_state);
        };
    }
    fn block_245<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_245_0: const #() : ()
        let s_245_0: () = ();
        // S s_245_1: call Halted(s_245_0)
        let s_245_1: bool = Halted(state, tracer, s_245_0);
        // N s_245_2: branch s_245_1 b392 b246
        if s_245_1 {
            return block_392(state, tracer, fn_state);
        } else {
            return block_246(state, tracer, fn_state);
        };
    }
    fn block_246<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_246_0: const #0u : u8
        let s_246_0: bool = false;
        // D s_246_1: write-var gs#109111 <= s_246_0
        fn_state.gs_109111 = s_246_0;
        // N s_246_2: jump b247
        return block_247(state, tracer, fn_state);
    }
    fn block_247<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_247_0: read-var gs#109111:u8
        let s_247_0: bool = fn_state.gs_109111;
        // N s_247_1: branch s_247_0 b391 b248
        if s_247_0 {
            return block_391(state, tracer, fn_state);
        } else {
            return block_248(state, tracer, fn_state);
        };
    }
    fn block_248<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_248_0: const #0u : u8
        let s_248_0: bool = false;
        // D s_248_1: write-var gs#109112 <= s_248_0
        fn_state.gs_109112 = s_248_0;
        // N s_248_2: jump b249
        return block_249(state, tracer, fn_state);
    }
    fn block_249<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_249_0: read-var gs#109112:u8
        let s_249_0: bool = fn_state.gs_109112;
        // N s_249_1: branch s_249_0 b390 b250
        if s_249_0 {
            return block_390(state, tracer, fn_state);
        } else {
            return block_250(state, tracer, fn_state);
        };
    }
    fn block_250<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_250_0: const #0u : u8
        let s_250_0: bool = false;
        // D s_250_1: write-var gs#109113 <= s_250_0
        fn_state.gs_109113 = s_250_0;
        // N s_250_2: jump b251
        return block_251(state, tracer, fn_state);
    }
    fn block_251<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_251_0: read-var gs#109113:u8
        let s_251_0: bool = fn_state.gs_109113;
        // N s_251_1: branch s_251_0 b389 b252
        if s_251_0 {
            return block_389(state, tracer, fn_state);
        } else {
            return block_252(state, tracer, fn_state);
        };
    }
    fn block_252<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_252_0: const #0u : u8
        let s_252_0: bool = false;
        // D s_252_1: write-var gs#109114 <= s_252_0
        fn_state.gs_109114 = s_252_0;
        // N s_252_2: jump b253
        return block_253(state, tracer, fn_state);
    }
    fn block_253<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_253_0: read-var gs#109114:u8
        let s_253_0: bool = fn_state.gs_109114;
        // N s_253_1: branch s_253_0 b388 b254
        if s_253_0 {
            return block_388(state, tracer, fn_state);
        } else {
            return block_254(state, tracer, fn_state);
        };
    }
    fn block_254<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_254_0: const #0u : u8
        let s_254_0: bool = false;
        // D s_254_1: write-var gs#109115 <= s_254_0
        fn_state.gs_109115 = s_254_0;
        // N s_254_2: jump b255
        return block_255(state, tracer, fn_state);
    }
    fn block_255<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_255_0: read-var gs#109115:u8
        let s_255_0: bool = fn_state.gs_109115;
        // N s_255_1: branch s_255_0 b387 b256
        if s_255_0 {
            return block_387(state, tracer, fn_state);
        } else {
            return block_256(state, tracer, fn_state);
        };
    }
    fn block_256<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_256_0: const #() : ()
        let s_256_0: () = ();
        // S s_256_1: call Halted(s_256_0)
        let s_256_1: bool = Halted(state, tracer, s_256_0);
        // N s_256_2: branch s_256_1 b386 b257
        if s_256_1 {
            return block_386(state, tracer, fn_state);
        } else {
            return block_257(state, tracer, fn_state);
        };
    }
    fn block_257<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_257_0: const #0u : u8
        let s_257_0: bool = false;
        // D s_257_1: write-var gs#109116 <= s_257_0
        fn_state.gs_109116 = s_257_0;
        // N s_257_2: jump b258
        return block_258(state, tracer, fn_state);
    }
    fn block_258<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_258_0: read-var gs#109116:u8
        let s_258_0: bool = fn_state.gs_109116;
        // N s_258_1: branch s_258_0 b385 b259
        if s_258_0 {
            return block_385(state, tracer, fn_state);
        } else {
            return block_259(state, tracer, fn_state);
        };
    }
    fn block_259<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_259_0: const #0u : u8
        let s_259_0: bool = false;
        // D s_259_1: write-var gs#109117 <= s_259_0
        fn_state.gs_109117 = s_259_0;
        // N s_259_2: jump b260
        return block_260(state, tracer, fn_state);
    }
    fn block_260<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_260_0: read-var gs#109117:u8
        let s_260_0: bool = fn_state.gs_109117;
        // N s_260_1: branch s_260_0 b384 b261
        if s_260_0 {
            return block_384(state, tracer, fn_state);
        } else {
            return block_261(state, tracer, fn_state);
        };
    }
    fn block_261<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_261_0: const #0u : u8
        let s_261_0: bool = false;
        // D s_261_1: write-var gs#109118 <= s_261_0
        fn_state.gs_109118 = s_261_0;
        // N s_261_2: jump b262
        return block_262(state, tracer, fn_state);
    }
    fn block_262<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_262_0: read-var gs#109118:u8
        let s_262_0: bool = fn_state.gs_109118;
        // N s_262_1: branch s_262_0 b383 b263
        if s_262_0 {
            return block_383(state, tracer, fn_state);
        } else {
            return block_263(state, tracer, fn_state);
        };
    }
    fn block_263<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_263_0: const #0u : u8
        let s_263_0: bool = false;
        // D s_263_1: write-var gs#109119 <= s_263_0
        fn_state.gs_109119 = s_263_0;
        // N s_263_2: jump b264
        return block_264(state, tracer, fn_state);
    }
    fn block_264<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_264_0: read-var gs#109119:u8
        let s_264_0: bool = fn_state.gs_109119;
        // N s_264_1: branch s_264_0 b382 b265
        if s_264_0 {
            return block_382(state, tracer, fn_state);
        } else {
            return block_265(state, tracer, fn_state);
        };
    }
    fn block_265<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_265_0: const #0u : u8
        let s_265_0: bool = false;
        // D s_265_1: write-var gs#109120 <= s_265_0
        fn_state.gs_109120 = s_265_0;
        // N s_265_2: jump b266
        return block_266(state, tracer, fn_state);
    }
    fn block_266<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_266_0: read-var gs#109120:u8
        let s_266_0: bool = fn_state.gs_109120;
        // N s_266_1: branch s_266_0 b381 b267
        if s_266_0 {
            return block_381(state, tracer, fn_state);
        } else {
            return block_267(state, tracer, fn_state);
        };
    }
    fn block_267<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_267_0: const #440u : u32
        let s_267_0: u32 = 440;
        // D s_267_1: read-reg s_267_0:u8
        let s_267_1: u8 = {
            let value = state.read_register::<u8>(s_267_0 as isize);
            tracer.read_register(s_267_0 as isize, value);
            value
        };
        // D s_267_2: call ELUsingAArch32(s_267_1)
        let s_267_2: bool = ELUsingAArch32(state, tracer, s_267_1);
        // D s_267_3: not s_267_2
        let s_267_3: bool = !s_267_2;
        // N s_267_4: branch s_267_3 b380 b268
        if s_267_3 {
            return block_380(state, tracer, fn_state);
        } else {
            return block_268(state, tracer, fn_state);
        };
    }
    fn block_268<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_268_0: const #0u : u8
        let s_268_0: bool = false;
        // D s_268_1: write-var gs#109121 <= s_268_0
        fn_state.gs_109121 = s_268_0;
        // N s_268_2: jump b269
        return block_269(state, tracer, fn_state);
    }
    fn block_269<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_269_0: read-var gs#109121:u8
        let s_269_0: bool = fn_state.gs_109121;
        // N s_269_1: branch s_269_0 b371 b270
        if s_269_0 {
            return block_371(state, tracer, fn_state);
        } else {
            return block_270(state, tracer, fn_state);
        };
    }
    fn block_270<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_270_0: const #440u : u32
        let s_270_0: u32 = 440;
        // D s_270_1: read-reg s_270_0:u8
        let s_270_1: u8 = {
            let value = state.read_register::<u8>(s_270_0 as isize);
            tracer.read_register(s_270_0 as isize, value);
            value
        };
        // D s_270_2: call ELUsingAArch32(s_270_1)
        let s_270_2: bool = ELUsingAArch32(state, tracer, s_270_1);
        // N s_270_3: branch s_270_2 b370 b271
        if s_270_2 {
            return block_370(state, tracer, fn_state);
        } else {
            return block_271(state, tracer, fn_state);
        };
    }
    fn block_271<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_271_0: const #0u : u8
        let s_271_0: bool = false;
        // D s_271_1: write-var gs#109122 <= s_271_0
        fn_state.gs_109122 = s_271_0;
        // N s_271_2: jump b272
        return block_272(state, tracer, fn_state);
    }
    fn block_272<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_272_0: read-var gs#109122:u8
        let s_272_0: bool = fn_state.gs_109122;
        // N s_272_1: branch s_272_0 b353 b273
        if s_272_0 {
            return block_353(state, tracer, fn_state);
        } else {
            return block_273(state, tracer, fn_state);
        };
    }
    fn block_273<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_273_0: const #() : ()
        let s_273_0: () = ();
        // S s_273_1: call EL2Enabled(s_273_0)
        let s_273_1: bool = EL2Enabled(state, tracer, s_273_0);
        // N s_273_2: branch s_273_1 b352 b274
        if s_273_1 {
            return block_352(state, tracer, fn_state);
        } else {
            return block_274(state, tracer, fn_state);
        };
    }
    fn block_274<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_274_0: const #0u : u8
        let s_274_0: bool = false;
        // D s_274_1: write-var gs#109123 <= s_274_0
        fn_state.gs_109123 = s_274_0;
        // N s_274_2: jump b275
        return block_275(state, tracer, fn_state);
    }
    fn block_275<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_275_0: read-var gs#109123:u8
        let s_275_0: bool = fn_state.gs_109123;
        // N s_275_1: branch s_275_0 b351 b276
        if s_275_0 {
            return block_351(state, tracer, fn_state);
        } else {
            return block_276(state, tracer, fn_state);
        };
    }
    fn block_276<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_276_0: const #0u : u8
        let s_276_0: bool = false;
        // D s_276_1: write-var gs#109124 <= s_276_0
        fn_state.gs_109124 = s_276_0;
        // N s_276_2: jump b277
        return block_277(state, tracer, fn_state);
    }
    fn block_277<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_277_0: read-var gs#109124:u8
        let s_277_0: bool = fn_state.gs_109124;
        // N s_277_1: branch s_277_0 b350 b278
        if s_277_0 {
            return block_350(state, tracer, fn_state);
        } else {
            return block_278(state, tracer, fn_state);
        };
    }
    fn block_278<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_278_0: const #() : ()
        let s_278_0: () = ();
        // S s_278_1: call EL2Enabled(s_278_0)
        let s_278_1: bool = EL2Enabled(state, tracer, s_278_0);
        // N s_278_2: branch s_278_1 b349 b279
        if s_278_1 {
            return block_349(state, tracer, fn_state);
        } else {
            return block_279(state, tracer, fn_state);
        };
    }
    fn block_279<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_279_0: const #0u : u8
        let s_279_0: bool = false;
        // D s_279_1: write-var gs#109125 <= s_279_0
        fn_state.gs_109125 = s_279_0;
        // N s_279_2: jump b280
        return block_280(state, tracer, fn_state);
    }
    fn block_280<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_280_0: read-var gs#109125:u8
        let s_280_0: bool = fn_state.gs_109125;
        // N s_280_1: branch s_280_0 b348 b281
        if s_280_0 {
            return block_348(state, tracer, fn_state);
        } else {
            return block_281(state, tracer, fn_state);
        };
    }
    fn block_281<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_281_0: const #0u : u8
        let s_281_0: bool = false;
        // D s_281_1: write-var gs#109126 <= s_281_0
        fn_state.gs_109126 = s_281_0;
        // N s_281_2: jump b282
        return block_282(state, tracer, fn_state);
    }
    fn block_282<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_282_0: read-var gs#109126:u8
        let s_282_0: bool = fn_state.gs_109126;
        // N s_282_1: branch s_282_0 b347 b283
        if s_282_0 {
            return block_347(state, tracer, fn_state);
        } else {
            return block_283(state, tracer, fn_state);
        };
    }
    fn block_283<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_283_0: const #() : ()
        let s_283_0: () = ();
        // S s_283_1: call EL2Enabled(s_283_0)
        let s_283_1: bool = EL2Enabled(state, tracer, s_283_0);
        // N s_283_2: branch s_283_1 b346 b284
        if s_283_1 {
            return block_346(state, tracer, fn_state);
        } else {
            return block_284(state, tracer, fn_state);
        };
    }
    fn block_284<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_284_0: const #0u : u8
        let s_284_0: bool = false;
        // D s_284_1: write-var gs#109127 <= s_284_0
        fn_state.gs_109127 = s_284_0;
        // N s_284_2: jump b285
        return block_285(state, tracer, fn_state);
    }
    fn block_285<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_285_0: read-var gs#109127:u8
        let s_285_0: bool = fn_state.gs_109127;
        // N s_285_1: branch s_285_0 b342 b286
        if s_285_0 {
            return block_342(state, tracer, fn_state);
        } else {
            return block_286(state, tracer, fn_state);
        };
    }
    fn block_286<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_286_0: const #0u : u8
        let s_286_0: bool = false;
        // D s_286_1: write-var gs#109129 <= s_286_0
        fn_state.gs_109129 = s_286_0;
        // N s_286_2: jump b287
        return block_287(state, tracer, fn_state);
    }
    fn block_287<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_287_0: read-var gs#109129:u8
        let s_287_0: bool = fn_state.gs_109129;
        // N s_287_1: branch s_287_0 b341 b288
        if s_287_0 {
            return block_341(state, tracer, fn_state);
        } else {
            return block_288(state, tracer, fn_state);
        };
    }
    fn block_288<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_288_0: const #() : ()
        let s_288_0: () = ();
        // S s_288_1: call EL2Enabled(s_288_0)
        let s_288_1: bool = EL2Enabled(state, tracer, s_288_0);
        // N s_288_2: branch s_288_1 b340 b289
        if s_288_1 {
            return block_340(state, tracer, fn_state);
        } else {
            return block_289(state, tracer, fn_state);
        };
    }
    fn block_289<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_289_0: const #0u : u8
        let s_289_0: bool = false;
        // D s_289_1: write-var gs#109130 <= s_289_0
        fn_state.gs_109130 = s_289_0;
        // N s_289_2: jump b290
        return block_290(state, tracer, fn_state);
    }
    fn block_290<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_290_0: read-var gs#109130:u8
        let s_290_0: bool = fn_state.gs_109130;
        // N s_290_1: branch s_290_0 b336 b291
        if s_290_0 {
            return block_336(state, tracer, fn_state);
        } else {
            return block_291(state, tracer, fn_state);
        };
    }
    fn block_291<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_291_0: const #0u : u8
        let s_291_0: bool = false;
        // D s_291_1: write-var gs#109132 <= s_291_0
        fn_state.gs_109132 = s_291_0;
        // N s_291_2: jump b292
        return block_292(state, tracer, fn_state);
    }
    fn block_292<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_292_0: read-var gs#109132:u8
        let s_292_0: bool = fn_state.gs_109132;
        // N s_292_1: branch s_292_0 b335 b293
        if s_292_0 {
            return block_335(state, tracer, fn_state);
        } else {
            return block_293(state, tracer, fn_state);
        };
    }
    fn block_293<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_293_0: const #424u : u32
        let s_293_0: u32 = 424;
        // D s_293_1: read-reg s_293_0:u8
        let s_293_1: u8 = {
            let value = state.read_register::<u8>(s_293_0 as isize);
            tracer.read_register(s_293_0 as isize, value);
            value
        };
        // C s_293_2: const #2u : u8
        let s_293_2: u8 = 2;
        // D s_293_3: cmp-lt s_293_1 s_293_2
        let s_293_3: bool = ((s_293_1) < (s_293_2));
        // N s_293_4: branch s_293_3 b334 b294
        if s_293_3 {
            return block_334(state, tracer, fn_state);
        } else {
            return block_294(state, tracer, fn_state);
        };
    }
    fn block_294<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_294_0: const #0u : u8
        let s_294_0: bool = false;
        // D s_294_1: write-var gs#109133 <= s_294_0
        fn_state.gs_109133 = s_294_0;
        // N s_294_2: jump b295
        return block_295(state, tracer, fn_state);
    }
    fn block_295<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_295_0: read-var gs#109133:u8
        let s_295_0: bool = fn_state.gs_109133;
        // N s_295_1: branch s_295_0 b333 b296
        if s_295_0 {
            return block_333(state, tracer, fn_state);
        } else {
            return block_296(state, tracer, fn_state);
        };
    }
    fn block_296<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_296_0: const #0u : u8
        let s_296_0: bool = false;
        // D s_296_1: write-var gs#109134 <= s_296_0
        fn_state.gs_109134 = s_296_0;
        // N s_296_2: jump b297
        return block_297(state, tracer, fn_state);
    }
    fn block_297<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_297_0: read-var gs#109134:u8
        let s_297_0: bool = fn_state.gs_109134;
        // N s_297_1: branch s_297_0 b327 b298
        if s_297_0 {
            return block_327(state, tracer, fn_state);
        } else {
            return block_298(state, tracer, fn_state);
        };
    }
    fn block_298<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_298_0: const #424u : u32
        let s_298_0: u32 = 424;
        // D s_298_1: read-reg s_298_0:u8
        let s_298_1: u8 = {
            let value = state.read_register::<u8>(s_298_0 as isize);
            tracer.read_register(s_298_0 as isize, value);
            value
        };
        // C s_298_2: const #2u : u8
        let s_298_2: u8 = 2;
        // D s_298_3: cmp-lt s_298_1 s_298_2
        let s_298_3: bool = ((s_298_1) < (s_298_2));
        // N s_298_4: branch s_298_3 b326 b299
        if s_298_3 {
            return block_326(state, tracer, fn_state);
        } else {
            return block_299(state, tracer, fn_state);
        };
    }
    fn block_299<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_299_0: const #0u : u8
        let s_299_0: bool = false;
        // D s_299_1: write-var gs#109135 <= s_299_0
        fn_state.gs_109135 = s_299_0;
        // N s_299_2: jump b300
        return block_300(state, tracer, fn_state);
    }
    fn block_300<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_300_0: read-var gs#109135:u8
        let s_300_0: bool = fn_state.gs_109135;
        // N s_300_1: branch s_300_0 b325 b301
        if s_300_0 {
            return block_325(state, tracer, fn_state);
        } else {
            return block_301(state, tracer, fn_state);
        };
    }
    fn block_301<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_301_0: const #0u : u8
        let s_301_0: bool = false;
        // D s_301_1: write-var gs#109136 <= s_301_0
        fn_state.gs_109136 = s_301_0;
        // N s_301_2: jump b302
        return block_302(state, tracer, fn_state);
    }
    fn block_302<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_302_0: read-var gs#109136:u8
        let s_302_0: bool = fn_state.gs_109136;
        // N s_302_1: branch s_302_0 b319 b303
        if s_302_0 {
            return block_319(state, tracer, fn_state);
        } else {
            return block_303(state, tracer, fn_state);
        };
    }
    fn block_303<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_303_0: const #424u : u32
        let s_303_0: u32 = 424;
        // D s_303_1: read-reg s_303_0:u8
        let s_303_1: u8 = {
            let value = state.read_register::<u8>(s_303_0 as isize);
            tracer.read_register(s_303_0 as isize, value);
            value
        };
        // C s_303_2: const #2u : u8
        let s_303_2: u8 = 2;
        // D s_303_3: cmp-lt s_303_1 s_303_2
        let s_303_3: bool = ((s_303_1) < (s_303_2));
        // N s_303_4: branch s_303_3 b318 b304
        if s_303_3 {
            return block_318(state, tracer, fn_state);
        } else {
            return block_304(state, tracer, fn_state);
        };
    }
    fn block_304<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_304_0: const #0u : u8
        let s_304_0: bool = false;
        // D s_304_1: write-var gs#109137 <= s_304_0
        fn_state.gs_109137 = s_304_0;
        // N s_304_2: jump b305
        return block_305(state, tracer, fn_state);
    }
    fn block_305<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_305_0: read-var gs#109137:u8
        let s_305_0: bool = fn_state.gs_109137;
        // N s_305_1: branch s_305_0 b317 b306
        if s_305_0 {
            return block_317(state, tracer, fn_state);
        } else {
            return block_306(state, tracer, fn_state);
        };
    }
    fn block_306<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_306_0: const #0u : u8
        let s_306_0: bool = false;
        // D s_306_1: write-var gs#109138 <= s_306_0
        fn_state.gs_109138 = s_306_0;
        // N s_306_2: jump b307
        return block_307(state, tracer, fn_state);
    }
    fn block_307<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_307_0: read-var gs#109138:u8
        let s_307_0: bool = fn_state.gs_109138;
        // N s_307_1: branch s_307_0 b311 b308
        if s_307_0 {
            return block_311(state, tracer, fn_state);
        } else {
            return block_308(state, tracer, fn_state);
        };
    }
    fn block_308<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_308_0: const #15s : i
        let s_308_0: i128 = 15;
        // D s_308_1: read-var t:i
        let s_308_1: i128 = fn_state.t;
        // D s_308_2: cmp-eq s_308_1 s_308_0
        let s_308_2: bool = ((s_308_1) == (s_308_0));
        // N s_308_3: branch s_308_2 b310 b309
        if s_308_2 {
            return block_310(state, tracer, fn_state);
        } else {
            return block_309(state, tracer, fn_state);
        };
    }
    fn block_309<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_309_0: const #() : ()
        let s_309_0: () = ();
        // S s_309_1: call DBGDSCRint_read(s_309_0)
        let s_309_1: ProductType700c18a878c5601b = DBGDSCRint_read(
            state,
            tracer,
            s_309_0,
        );
        // S s_309_2: call __get_DBGDSCRint(s_309_1)
        let s_309_2: ProductType700c18a878c5601b = u__get_DBGDSCRint(
            state,
            tracer,
            s_309_1,
        );
        // D s_309_3: write-var ga#172699 <= s_309_2
        fn_state.ga_172699 = s_309_2;
        // D s_309_4: read-var ga#172699.0:struct
        let s_309_4: u32 = fn_state.ga_172699._0;
        // D s_309_5: read-var t:i
        let s_309_5: i128 = fn_state.t;
        // D s_309_6: call R_set(s_309_5, s_309_4)
        let s_309_6: () = R_set(state, tracer, s_309_5, s_309_4);
        // N s_309_7: return
        return;
    }
    fn block_310<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_310_0: const #() : ()
        let s_310_0: () = ();
        // S s_310_1: call DBGDSCRint_read(s_310_0)
        let s_310_1: ProductType700c18a878c5601b = DBGDSCRint_read(
            state,
            tracer,
            s_310_0,
        );
        // D s_310_2: write-var ga#172696 <= s_310_1
        fn_state.ga_172696 = s_310_1;
        // D s_310_3: read-var ga#172696.0:struct
        let s_310_3: u32 = fn_state.ga_172696._0;
        // C s_310_4: const #28s : i
        let s_310_4: i128 = 28;
        // D s_310_5: cast zx s_310_3 -> bv
        let s_310_5: Bits = Bits::new(s_310_3 as u128, 32u16);
        // C s_310_6: const #1s : i64
        let s_310_6: i64 = 1;
        // C s_310_7: cast zx s_310_6 -> i
        let s_310_7: i128 = (i128::try_from(s_310_6).unwrap());
        // C s_310_8: const #3s : i
        let s_310_8: i128 = 3;
        // C s_310_9: add s_310_8 s_310_7
        let s_310_9: i128 = (s_310_8 + s_310_7);
        // D s_310_10: bit-extract s_310_5 s_310_4 s_310_9
        let s_310_10: Bits = (Bits::new(
            ((s_310_5) >> (s_310_4)).value(),
            u16::try_from(s_310_9).unwrap(),
        ));
        // D s_310_11: cast reint s_310_10 -> u8
        let s_310_11: u8 = (s_310_10.value() as u8);
        // C s_310_12: const #3s : i
        let s_310_12: i128 = 3;
        // D s_310_13: cast zx s_310_11 -> bv
        let s_310_13: Bits = Bits::new(s_310_11 as u128, 4u16);
        // C s_310_14: const #1s : i64
        let s_310_14: i64 = 1;
        // C s_310_15: cast zx s_310_14 -> i
        let s_310_15: i128 = (i128::try_from(s_310_14).unwrap());
        // C s_310_16: const #0s : i
        let s_310_16: i128 = 0;
        // C s_310_17: add s_310_16 s_310_15
        let s_310_17: i128 = (s_310_16 + s_310_15);
        // D s_310_18: bit-extract s_310_13 s_310_12 s_310_17
        let s_310_18: Bits = (Bits::new(
            ((s_310_13) >> (s_310_12)).value(),
            u16::try_from(s_310_17).unwrap(),
        ));
        // D s_310_19: cast reint s_310_18 -> u8
        let s_310_19: bool = ((s_310_18.value()) != 0);
        // C s_310_20: const #16984u : u32
        let s_310_20: u32 = 16984;
        // N s_310_21: write-reg s_310_20 <= s_310_19
        let s_310_21: () = {
            state.write_register::<bool>(s_310_20 as isize, s_310_19);
            tracer.write_register(s_310_20 as isize, s_310_19);
        };
        // C s_310_22: const #2s : i
        let s_310_22: i128 = 2;
        // D s_310_23: cast zx s_310_11 -> bv
        let s_310_23: Bits = Bits::new(s_310_11 as u128, 4u16);
        // C s_310_24: const #1s : i64
        let s_310_24: i64 = 1;
        // C s_310_25: cast zx s_310_24 -> i
        let s_310_25: i128 = (i128::try_from(s_310_24).unwrap());
        // C s_310_26: const #0s : i
        let s_310_26: i128 = 0;
        // C s_310_27: add s_310_26 s_310_25
        let s_310_27: i128 = (s_310_26 + s_310_25);
        // D s_310_28: bit-extract s_310_23 s_310_22 s_310_27
        let s_310_28: Bits = (Bits::new(
            ((s_310_23) >> (s_310_22)).value(),
            u16::try_from(s_310_27).unwrap(),
        ));
        // D s_310_29: cast reint s_310_28 -> u8
        let s_310_29: bool = ((s_310_28.value()) != 0);
        // C s_310_30: const #16997u : u32
        let s_310_30: u32 = 16997;
        // N s_310_31: write-reg s_310_30 <= s_310_29
        let s_310_31: () = {
            state.write_register::<bool>(s_310_30 as isize, s_310_29);
            tracer.write_register(s_310_30 as isize, s_310_29);
        };
        // C s_310_32: const #1s : i
        let s_310_32: i128 = 1;
        // D s_310_33: cast zx s_310_11 -> bv
        let s_310_33: Bits = Bits::new(s_310_11 as u128, 4u16);
        // C s_310_34: const #1s : i64
        let s_310_34: i64 = 1;
        // C s_310_35: cast zx s_310_34 -> i
        let s_310_35: i128 = (i128::try_from(s_310_34).unwrap());
        // C s_310_36: const #0s : i
        let s_310_36: i128 = 0;
        // C s_310_37: add s_310_36 s_310_35
        let s_310_37: i128 = (s_310_36 + s_310_35);
        // D s_310_38: bit-extract s_310_33 s_310_32 s_310_37
        let s_310_38: Bits = (Bits::new(
            ((s_310_33) >> (s_310_32)).value(),
            u16::try_from(s_310_37).unwrap(),
        ));
        // D s_310_39: cast reint s_310_38 -> u8
        let s_310_39: bool = ((s_310_38.value()) != 0);
        // C s_310_40: const #16971u : u32
        let s_310_40: u32 = 16971;
        // N s_310_41: write-reg s_310_40 <= s_310_39
        let s_310_41: () = {
            state.write_register::<bool>(s_310_40 as isize, s_310_39);
            tracer.write_register(s_310_40 as isize, s_310_39);
        };
        // C s_310_42: const #0s : i
        let s_310_42: i128 = 0;
        // D s_310_43: cast zx s_310_11 -> bv
        let s_310_43: Bits = Bits::new(s_310_11 as u128, 4u16);
        // C s_310_44: const #1s : i64
        let s_310_44: i64 = 1;
        // C s_310_45: cast zx s_310_44 -> i
        let s_310_45: i128 = (i128::try_from(s_310_44).unwrap());
        // C s_310_46: const #0s : i
        let s_310_46: i128 = 0;
        // C s_310_47: add s_310_46 s_310_45
        let s_310_47: i128 = (s_310_46 + s_310_45);
        // D s_310_48: bit-extract s_310_43 s_310_42 s_310_47
        let s_310_48: Bits = (Bits::new(
            ((s_310_43) >> (s_310_42)).value(),
            u16::try_from(s_310_47).unwrap(),
        ));
        // D s_310_49: cast reint s_310_48 -> u8
        let s_310_49: bool = ((s_310_48.value()) != 0);
        // C s_310_50: const #16996u : u32
        let s_310_50: u32 = 16996;
        // N s_310_51: write-reg s_310_50 <= s_310_49
        let s_310_51: () = {
            state.write_register::<bool>(s_310_50 as isize, s_310_49);
            tracer.write_register(s_310_50 as isize, s_310_49);
        };
        // N s_310_52: return
        return;
    }
    fn block_311<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_311_0: const #() : ()
        let s_311_0: () = ();
        // S s_311_1: call Halted(s_311_0)
        let s_311_1: bool = Halted(state, tracer, s_311_0);
        // N s_311_2: branch s_311_1 b316 b312
        if s_311_1 {
            return block_316(state, tracer, fn_state);
        } else {
            return block_312(state, tracer, fn_state);
        };
    }
    fn block_312<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_312_0: const #0u : u8
        let s_312_0: bool = false;
        // D s_312_1: write-var gs#109153 <= s_312_0
        fn_state.gs_109153 = s_312_0;
        // N s_312_2: jump b313
        return block_313(state, tracer, fn_state);
    }
    fn block_313<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_313_0: read-var gs#109153:u8
        let s_313_0: bool = fn_state.gs_109153;
        // N s_313_1: branch s_313_0 b315 b314
        if s_313_0 {
            return block_315(state, tracer, fn_state);
        } else {
            return block_314(state, tracer, fn_state);
        };
    }
    fn block_314<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_314_0: const #5u : u8
        let s_314_0: u8 = 5;
        // C s_314_1: cast zx s_314_0 -> bv
        let s_314_1: Bits = Bits::new(s_314_0 as u128, 8u16);
        // C s_314_2: cast zx s_314_1 -> i
        let s_314_2: i128 = (s_314_1.value() as i128);
        // C s_314_3: cast reint s_314_2 -> i64
        let s_314_3: i64 = (s_314_2 as i64);
        // C s_314_4: cast zx s_314_3 -> i
        let s_314_4: i128 = (i128::try_from(s_314_3).unwrap());
        // C s_314_5: const #424u : u32
        let s_314_5: u32 = 424;
        // D s_314_6: read-reg s_314_5:u8
        let s_314_6: u8 = {
            let value = state.read_register::<u8>(s_314_5 as isize);
            tracer.read_register(s_314_5 as isize, value);
            value
        };
        // D s_314_7: call AArch64_AArch32SystemAccessTrap(s_314_6, s_314_4)
        let s_314_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_314_6,
            s_314_4,
        );
        // N s_314_8: return
        return;
    }
    fn block_315<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_315_0: panic
        panic!("{:?}", ());
        // N s_315_1: return
        return;
    }
    fn block_316<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_316_0: const #() : ()
        let s_316_0: () = ();
        // S s_316_1: call EDSCR_read(s_316_0)
        let s_316_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_316_0);
        // S s_316_2: call _get_EDSCR_Type_SDD(s_316_1)
        let s_316_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_316_1);
        // S s_316_3: cast zx s_316_2 -> bv
        let s_316_3: Bits = Bits::new(s_316_2 as u128, 1u16);
        // C s_316_4: const #1u : u8
        let s_316_4: bool = true;
        // C s_316_5: cast zx s_316_4 -> bv
        let s_316_5: Bits = Bits::new(s_316_4 as u128, 1u16);
        // S s_316_6: cmp-eq s_316_3 s_316_5
        let s_316_6: bool = ((s_316_3) == (s_316_5));
        // D s_316_7: write-var gs#109153 <= s_316_6
        fn_state.gs_109153 = s_316_6;
        // N s_316_8: jump b313
        return block_313(state, tracer, fn_state);
    }
    fn block_317<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_317_0: read-var __MDCR_EL3_TDA:u8
        let s_317_0: bool = fn_state.u__MDCR_EL3_TDA;
        // D s_317_1: cast zx s_317_0 -> bv
        let s_317_1: Bits = Bits::new(s_317_0 as u128, 1u16);
        // C s_317_2: const #1u : u8
        let s_317_2: bool = true;
        // C s_317_3: cast zx s_317_2 -> bv
        let s_317_3: Bits = Bits::new(s_317_2 as u128, 1u16);
        // D s_317_4: cmp-eq s_317_1 s_317_3
        let s_317_4: bool = ((s_317_1) == (s_317_3));
        // D s_317_5: write-var gs#109138 <= s_317_4
        fn_state.gs_109138 = s_317_4;
        // N s_317_6: jump b307
        return block_307(state, tracer, fn_state);
    }
    fn block_318<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_318_0: const #424u : u32
        let s_318_0: u32 = 424;
        // D s_318_1: read-reg s_318_0:u8
        let s_318_1: u8 = {
            let value = state.read_register::<u8>(s_318_0 as isize);
            tracer.read_register(s_318_0 as isize, value);
            value
        };
        // D s_318_2: call ELUsingAArch32(s_318_1)
        let s_318_2: bool = ELUsingAArch32(state, tracer, s_318_1);
        // D s_318_3: not s_318_2
        let s_318_3: bool = !s_318_2;
        // D s_318_4: write-var gs#109137 <= s_318_3
        fn_state.gs_109137 = s_318_3;
        // N s_318_5: jump b305
        return block_305(state, tracer, fn_state);
    }
    fn block_319<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_319_0: const #() : ()
        let s_319_0: () = ();
        // S s_319_1: call Halted(s_319_0)
        let s_319_1: bool = Halted(state, tracer, s_319_0);
        // N s_319_2: branch s_319_1 b324 b320
        if s_319_1 {
            return block_324(state, tracer, fn_state);
        } else {
            return block_320(state, tracer, fn_state);
        };
    }
    fn block_320<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_320_0: const #0u : u8
        let s_320_0: bool = false;
        // D s_320_1: write-var gs#109154 <= s_320_0
        fn_state.gs_109154 = s_320_0;
        // N s_320_2: jump b321
        return block_321(state, tracer, fn_state);
    }
    fn block_321<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_321_0: read-var gs#109154:u8
        let s_321_0: bool = fn_state.gs_109154;
        // N s_321_1: branch s_321_0 b323 b322
        if s_321_0 {
            return block_323(state, tracer, fn_state);
        } else {
            return block_322(state, tracer, fn_state);
        };
    }
    fn block_322<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_322_0: const #() : ()
        let s_322_0: () = ();
        // S s_322_1: call AArch32_TakeMonitorTrapException(s_322_0)
        let s_322_1: () = AArch32_TakeMonitorTrapException(state, tracer, s_322_0);
        // N s_322_2: return
        return;
    }
    fn block_323<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_323_0: panic
        panic!("{:?}", ());
        // N s_323_1: return
        return;
    }
    fn block_324<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_324_0: const #() : ()
        let s_324_0: () = ();
        // S s_324_1: call EDSCR_read(s_324_0)
        let s_324_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_324_0);
        // S s_324_2: call _get_EDSCR_Type_SDD(s_324_1)
        let s_324_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_324_1);
        // S s_324_3: cast zx s_324_2 -> bv
        let s_324_3: Bits = Bits::new(s_324_2 as u128, 1u16);
        // C s_324_4: const #1u : u8
        let s_324_4: bool = true;
        // C s_324_5: cast zx s_324_4 -> bv
        let s_324_5: Bits = Bits::new(s_324_4 as u128, 1u16);
        // S s_324_6: cmp-eq s_324_3 s_324_5
        let s_324_6: bool = ((s_324_3) == (s_324_5));
        // D s_324_7: write-var gs#109154 <= s_324_6
        fn_state.gs_109154 = s_324_6;
        // N s_324_8: jump b321
        return block_321(state, tracer, fn_state);
    }
    fn block_325<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_325_0: read-var __SDCR_TDCC:u8
        let s_325_0: bool = fn_state.u__SDCR_TDCC;
        // D s_325_1: cast zx s_325_0 -> bv
        let s_325_1: Bits = Bits::new(s_325_0 as u128, 1u16);
        // C s_325_2: const #1u : u8
        let s_325_2: bool = true;
        // C s_325_3: cast zx s_325_2 -> bv
        let s_325_3: Bits = Bits::new(s_325_2 as u128, 1u16);
        // D s_325_4: cmp-eq s_325_1 s_325_3
        let s_325_4: bool = ((s_325_1) == (s_325_3));
        // D s_325_5: write-var gs#109136 <= s_325_4
        fn_state.gs_109136 = s_325_4;
        // N s_325_6: jump b302
        return block_302(state, tracer, fn_state);
    }
    fn block_326<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_326_0: const #424u : u32
        let s_326_0: u32 = 424;
        // D s_326_1: read-reg s_326_0:u8
        let s_326_1: u8 = {
            let value = state.read_register::<u8>(s_326_0 as isize);
            tracer.read_register(s_326_0 as isize, value);
            value
        };
        // D s_326_2: call ELUsingAArch32(s_326_1)
        let s_326_2: bool = ELUsingAArch32(state, tracer, s_326_1);
        // D s_326_3: write-var gs#109135 <= s_326_2
        fn_state.gs_109135 = s_326_2;
        // N s_326_4: jump b300
        return block_300(state, tracer, fn_state);
    }
    fn block_327<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_327_0: const #() : ()
        let s_327_0: () = ();
        // S s_327_1: call Halted(s_327_0)
        let s_327_1: bool = Halted(state, tracer, s_327_0);
        // N s_327_2: branch s_327_1 b332 b328
        if s_327_1 {
            return block_332(state, tracer, fn_state);
        } else {
            return block_328(state, tracer, fn_state);
        };
    }
    fn block_328<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_328_0: const #0u : u8
        let s_328_0: bool = false;
        // D s_328_1: write-var gs#109155 <= s_328_0
        fn_state.gs_109155 = s_328_0;
        // N s_328_2: jump b329
        return block_329(state, tracer, fn_state);
    }
    fn block_329<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_329_0: read-var gs#109155:u8
        let s_329_0: bool = fn_state.gs_109155;
        // N s_329_1: branch s_329_0 b331 b330
        if s_329_0 {
            return block_331(state, tracer, fn_state);
        } else {
            return block_330(state, tracer, fn_state);
        };
    }
    fn block_330<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_330_0: const #5u : u8
        let s_330_0: u8 = 5;
        // C s_330_1: cast zx s_330_0 -> bv
        let s_330_1: Bits = Bits::new(s_330_0 as u128, 8u16);
        // C s_330_2: cast zx s_330_1 -> i
        let s_330_2: i128 = (s_330_1.value() as i128);
        // C s_330_3: cast reint s_330_2 -> i64
        let s_330_3: i64 = (s_330_2 as i64);
        // C s_330_4: cast zx s_330_3 -> i
        let s_330_4: i128 = (i128::try_from(s_330_3).unwrap());
        // C s_330_5: const #424u : u32
        let s_330_5: u32 = 424;
        // D s_330_6: read-reg s_330_5:u8
        let s_330_6: u8 = {
            let value = state.read_register::<u8>(s_330_5 as isize);
            tracer.read_register(s_330_5 as isize, value);
            value
        };
        // D s_330_7: call AArch64_AArch32SystemAccessTrap(s_330_6, s_330_4)
        let s_330_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_330_6,
            s_330_4,
        );
        // N s_330_8: return
        return;
    }
    fn block_331<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_331_0: panic
        panic!("{:?}", ());
        // N s_331_1: return
        return;
    }
    fn block_332<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_332_0: const #() : ()
        let s_332_0: () = ();
        // S s_332_1: call EDSCR_read(s_332_0)
        let s_332_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_332_0);
        // S s_332_2: call _get_EDSCR_Type_SDD(s_332_1)
        let s_332_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_332_1);
        // S s_332_3: cast zx s_332_2 -> bv
        let s_332_3: Bits = Bits::new(s_332_2 as u128, 1u16);
        // C s_332_4: const #1u : u8
        let s_332_4: bool = true;
        // C s_332_5: cast zx s_332_4 -> bv
        let s_332_5: Bits = Bits::new(s_332_4 as u128, 1u16);
        // S s_332_6: cmp-eq s_332_3 s_332_5
        let s_332_6: bool = ((s_332_3) == (s_332_5));
        // D s_332_7: write-var gs#109155 <= s_332_6
        fn_state.gs_109155 = s_332_6;
        // N s_332_8: jump b329
        return block_329(state, tracer, fn_state);
    }
    fn block_333<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_333_0: read-var __MDCR_EL3_TDCC:u8
        let s_333_0: bool = fn_state.u__MDCR_EL3_TDCC;
        // D s_333_1: cast zx s_333_0 -> bv
        let s_333_1: Bits = Bits::new(s_333_0 as u128, 1u16);
        // C s_333_2: const #1u : u8
        let s_333_2: bool = true;
        // C s_333_3: cast zx s_333_2 -> bv
        let s_333_3: Bits = Bits::new(s_333_2 as u128, 1u16);
        // D s_333_4: cmp-eq s_333_1 s_333_3
        let s_333_4: bool = ((s_333_1) == (s_333_3));
        // D s_333_5: write-var gs#109134 <= s_333_4
        fn_state.gs_109134 = s_333_4;
        // N s_333_6: jump b297
        return block_297(state, tracer, fn_state);
    }
    fn block_334<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_334_0: const #424u : u32
        let s_334_0: u32 = 424;
        // D s_334_1: read-reg s_334_0:u8
        let s_334_1: u8 = {
            let value = state.read_register::<u8>(s_334_0 as isize);
            tracer.read_register(s_334_0 as isize, value);
            value
        };
        // D s_334_2: call ELUsingAArch32(s_334_1)
        let s_334_2: bool = ELUsingAArch32(state, tracer, s_334_1);
        // D s_334_3: not s_334_2
        let s_334_3: bool = !s_334_2;
        // D s_334_4: write-var gs#109133 <= s_334_3
        fn_state.gs_109133 = s_334_3;
        // N s_334_5: jump b295
        return block_295(state, tracer, fn_state);
    }
    fn block_335<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_335_0: const #5u : u8
        let s_335_0: u8 = 5;
        // C s_335_1: cast zx s_335_0 -> bv
        let s_335_1: Bits = Bits::new(s_335_0 as u128, 8u16);
        // C s_335_2: cast zx s_335_1 -> i
        let s_335_2: i128 = (s_335_1.value() as i128);
        // C s_335_3: cast reint s_335_2 -> i64
        let s_335_3: i64 = (s_335_2 as i64);
        // C s_335_4: cast zx s_335_3 -> i
        let s_335_4: i128 = (i128::try_from(s_335_3).unwrap());
        // S s_335_5: call AArch32_TakeHypTrapException(s_335_4)
        let s_335_5: () = AArch32_TakeHypTrapException(state, tracer, s_335_4);
        // N s_335_6: return
        return;
    }
    fn block_336<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_336_0: const #() : ()
        let s_336_0: () = ();
        // S s_336_1: call HCR_read(s_336_0)
        let s_336_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_336_0);
        // S s_336_2: call _get_HCR_Type_TGE(s_336_1)
        let s_336_2: bool = u_get_HCR_Type_TGE(state, tracer, s_336_1);
        // S s_336_3: cast zx s_336_2 -> bv
        let s_336_3: Bits = Bits::new(s_336_2 as u128, 1u16);
        // C s_336_4: const #1u : u8
        let s_336_4: bool = true;
        // C s_336_5: cast zx s_336_4 -> bv
        let s_336_5: Bits = Bits::new(s_336_4 as u128, 1u16);
        // S s_336_6: cmp-eq s_336_3 s_336_5
        let s_336_6: bool = ((s_336_3) == (s_336_5));
        // N s_336_7: branch s_336_6 b339 b337
        if s_336_6 {
            return block_339(state, tracer, fn_state);
        } else {
            return block_337(state, tracer, fn_state);
        };
    }
    fn block_337<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_337_0: const #() : ()
        let s_337_0: () = ();
        // S s_337_1: call HDCR_read(s_337_0)
        let s_337_1: ProductType700c18a878c5601b = HDCR_read(state, tracer, s_337_0);
        // S s_337_2: call _get_HDCR_Type_TDE(s_337_1)
        let s_337_2: bool = u_get_HDCR_Type_TDE(state, tracer, s_337_1);
        // C s_337_3: const #() : ()
        let s_337_3: () = ();
        // S s_337_4: call HDCR_read(s_337_3)
        let s_337_4: ProductType700c18a878c5601b = HDCR_read(state, tracer, s_337_3);
        // S s_337_5: call _get_HDCR_Type_TDA(s_337_4)
        let s_337_5: bool = u_get_HDCR_Type_TDA(state, tracer, s_337_4);
        // S s_337_6: cast zx s_337_2 -> bv
        let s_337_6: Bits = Bits::new(s_337_2 as u128, 1u16);
        // S s_337_7: cast zx s_337_5 -> bv
        let s_337_7: Bits = Bits::new(s_337_5 as u128, 1u16);
        // S s_337_8: cast reint s_337_6 -> u128
        let s_337_8: u128 = (s_337_6.value() as u128);
        // D s_337_9: size-of s_337_6
        let s_337_9: u16 = s_337_6.length();
        // S s_337_10: cast reint s_337_7 -> u128
        let s_337_10: u128 = (s_337_7.value() as u128);
        // D s_337_11: size-of s_337_7
        let s_337_11: u16 = s_337_7.length();
        // D s_337_12: lsl s_337_8 s_337_11
        let s_337_12: u128 = s_337_8 << s_337_11;
        // D s_337_13: or s_337_12 s_337_10
        let s_337_13: u128 = ((s_337_12) | (s_337_10));
        // D s_337_14: add s_337_9 s_337_11
        let s_337_14: u16 = (s_337_9 + s_337_11);
        // D s_337_15: create-bits s_337_13 s_337_14
        let s_337_15: Bits = Bits::new(s_337_13, s_337_14);
        // D s_337_16: cast reint s_337_15 -> u8
        let s_337_16: u8 = (s_337_15.value() as u8);
        // D s_337_17: cast zx s_337_16 -> bv
        let s_337_17: Bits = Bits::new(s_337_16 as u128, 2u16);
        // C s_337_18: const #0u : u8
        let s_337_18: u8 = 0;
        // C s_337_19: cast zx s_337_18 -> bv
        let s_337_19: Bits = Bits::new(s_337_18 as u128, 2u16);
        // D s_337_20: cmp-ne s_337_17 s_337_19
        let s_337_20: bool = ((s_337_17) != (s_337_19));
        // D s_337_21: write-var gs#109131 <= s_337_20
        fn_state.gs_109131 = s_337_20;
        // N s_337_22: jump b338
        return block_338(state, tracer, fn_state);
    }
    fn block_338<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_338_0: read-var gs#109131:u8
        let s_338_0: bool = fn_state.gs_109131;
        // D s_338_1: write-var gs#109132 <= s_338_0
        fn_state.gs_109132 = s_338_0;
        // N s_338_2: jump b292
        return block_292(state, tracer, fn_state);
    }
    fn block_339<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_339_0: const #1u : u8
        let s_339_0: bool = true;
        // D s_339_1: write-var gs#109131 <= s_339_0
        fn_state.gs_109131 = s_339_0;
        // N s_339_2: jump b338
        return block_338(state, tracer, fn_state);
    }
    fn block_340<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_340_0: const #432u : u32
        let s_340_0: u32 = 432;
        // D s_340_1: read-reg s_340_0:u8
        let s_340_1: u8 = {
            let value = state.read_register::<u8>(s_340_0 as isize);
            tracer.read_register(s_340_0 as isize, value);
            value
        };
        // D s_340_2: call ELUsingAArch32(s_340_1)
        let s_340_2: bool = ELUsingAArch32(state, tracer, s_340_1);
        // D s_340_3: write-var gs#109130 <= s_340_2
        fn_state.gs_109130 = s_340_2;
        // N s_340_4: jump b290
        return block_290(state, tracer, fn_state);
    }
    fn block_341<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_341_0: const #5u : u8
        let s_341_0: u8 = 5;
        // C s_341_1: cast zx s_341_0 -> bv
        let s_341_1: Bits = Bits::new(s_341_0 as u128, 8u16);
        // C s_341_2: cast zx s_341_1 -> i
        let s_341_2: i128 = (s_341_1.value() as i128);
        // C s_341_3: cast reint s_341_2 -> i64
        let s_341_3: i64 = (s_341_2 as i64);
        // C s_341_4: cast zx s_341_3 -> i
        let s_341_4: i128 = (i128::try_from(s_341_3).unwrap());
        // C s_341_5: const #432u : u32
        let s_341_5: u32 = 432;
        // D s_341_6: read-reg s_341_5:u8
        let s_341_6: u8 = {
            let value = state.read_register::<u8>(s_341_5 as isize);
            tracer.read_register(s_341_5 as isize, value);
            value
        };
        // D s_341_7: call AArch64_AArch32SystemAccessTrap(s_341_6, s_341_4)
        let s_341_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_341_6,
            s_341_4,
        );
        // N s_341_8: return
        return;
    }
    fn block_342<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_342_0: const #102552u : u32
        let s_342_0: u32 = 102552;
        // D s_342_1: read-reg s_342_0:struct
        let s_342_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_342_0 as isize);
            tracer.read_register(s_342_0 as isize, value);
            value
        };
        // D s_342_2: call _get_HCR_EL2_Type_TGE(s_342_1)
        let s_342_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_342_1);
        // D s_342_3: cast zx s_342_2 -> bv
        let s_342_3: Bits = Bits::new(s_342_2 as u128, 1u16);
        // C s_342_4: const #1u : u8
        let s_342_4: bool = true;
        // C s_342_5: cast zx s_342_4 -> bv
        let s_342_5: Bits = Bits::new(s_342_4 as u128, 1u16);
        // D s_342_6: cmp-eq s_342_3 s_342_5
        let s_342_6: bool = ((s_342_3) == (s_342_5));
        // N s_342_7: branch s_342_6 b345 b343
        if s_342_6 {
            return block_345(state, tracer, fn_state);
        } else {
            return block_343(state, tracer, fn_state);
        };
    }
    fn block_343<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_343_0: const #104880u : u32
        let s_343_0: u32 = 104880;
        // D s_343_1: read-reg s_343_0:struct
        let s_343_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_343_0 as isize);
            tracer.read_register(s_343_0 as isize, value);
            value
        };
        // D s_343_2: call _get_MDCR_EL2_Type_TDE(s_343_1)
        let s_343_2: bool = u_get_MDCR_EL2_Type_TDE(state, tracer, s_343_1);
        // C s_343_3: const #104880u : u32
        let s_343_3: u32 = 104880;
        // D s_343_4: read-reg s_343_3:struct
        let s_343_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_343_3 as isize);
            tracer.read_register(s_343_3 as isize, value);
            value
        };
        // D s_343_5: call _get_MDCR_EL2_Type_TDA(s_343_4)
        let s_343_5: bool = u_get_MDCR_EL2_Type_TDA(state, tracer, s_343_4);
        // D s_343_6: cast zx s_343_2 -> bv
        let s_343_6: Bits = Bits::new(s_343_2 as u128, 1u16);
        // D s_343_7: cast zx s_343_5 -> bv
        let s_343_7: Bits = Bits::new(s_343_5 as u128, 1u16);
        // D s_343_8: cast reint s_343_6 -> u128
        let s_343_8: u128 = (s_343_6.value() as u128);
        // D s_343_9: size-of s_343_6
        let s_343_9: u16 = s_343_6.length();
        // D s_343_10: cast reint s_343_7 -> u128
        let s_343_10: u128 = (s_343_7.value() as u128);
        // D s_343_11: size-of s_343_7
        let s_343_11: u16 = s_343_7.length();
        // D s_343_12: lsl s_343_8 s_343_11
        let s_343_12: u128 = s_343_8 << s_343_11;
        // D s_343_13: or s_343_12 s_343_10
        let s_343_13: u128 = ((s_343_12) | (s_343_10));
        // D s_343_14: add s_343_9 s_343_11
        let s_343_14: u16 = (s_343_9 + s_343_11);
        // D s_343_15: create-bits s_343_13 s_343_14
        let s_343_15: Bits = Bits::new(s_343_13, s_343_14);
        // D s_343_16: cast reint s_343_15 -> u8
        let s_343_16: u8 = (s_343_15.value() as u8);
        // D s_343_17: cast zx s_343_16 -> bv
        let s_343_17: Bits = Bits::new(s_343_16 as u128, 2u16);
        // C s_343_18: const #0u : u8
        let s_343_18: u8 = 0;
        // C s_343_19: cast zx s_343_18 -> bv
        let s_343_19: Bits = Bits::new(s_343_18 as u128, 2u16);
        // D s_343_20: cmp-ne s_343_17 s_343_19
        let s_343_20: bool = ((s_343_17) != (s_343_19));
        // D s_343_21: write-var gs#109128 <= s_343_20
        fn_state.gs_109128 = s_343_20;
        // N s_343_22: jump b344
        return block_344(state, tracer, fn_state);
    }
    fn block_344<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_344_0: read-var gs#109128:u8
        let s_344_0: bool = fn_state.gs_109128;
        // D s_344_1: write-var gs#109129 <= s_344_0
        fn_state.gs_109129 = s_344_0;
        // N s_344_2: jump b287
        return block_287(state, tracer, fn_state);
    }
    fn block_345<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_345_0: const #1u : u8
        let s_345_0: bool = true;
        // D s_345_1: write-var gs#109128 <= s_345_0
        fn_state.gs_109128 = s_345_0;
        // N s_345_2: jump b344
        return block_344(state, tracer, fn_state);
    }
    fn block_346<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_346_0: const #432u : u32
        let s_346_0: u32 = 432;
        // D s_346_1: read-reg s_346_0:u8
        let s_346_1: u8 = {
            let value = state.read_register::<u8>(s_346_0 as isize);
            tracer.read_register(s_346_0 as isize, value);
            value
        };
        // D s_346_2: call ELUsingAArch32(s_346_1)
        let s_346_2: bool = ELUsingAArch32(state, tracer, s_346_1);
        // D s_346_3: not s_346_2
        let s_346_3: bool = !s_346_2;
        // D s_346_4: write-var gs#109127 <= s_346_3
        fn_state.gs_109127 = s_346_3;
        // N s_346_5: jump b285
        return block_285(state, tracer, fn_state);
    }
    fn block_347<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_347_0: const #5u : u8
        let s_347_0: u8 = 5;
        // C s_347_1: cast zx s_347_0 -> bv
        let s_347_1: Bits = Bits::new(s_347_0 as u128, 8u16);
        // C s_347_2: cast zx s_347_1 -> i
        let s_347_2: i128 = (s_347_1.value() as i128);
        // C s_347_3: cast reint s_347_2 -> i64
        let s_347_3: i64 = (s_347_2 as i64);
        // C s_347_4: cast zx s_347_3 -> i
        let s_347_4: i128 = (i128::try_from(s_347_3).unwrap());
        // S s_347_5: call AArch32_TakeHypTrapException(s_347_4)
        let s_347_5: () = AArch32_TakeHypTrapException(state, tracer, s_347_4);
        // N s_347_6: return
        return;
    }
    fn block_348<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_348_0: read-var __HDCR_TDCC:u8
        let s_348_0: bool = fn_state.u__HDCR_TDCC;
        // D s_348_1: cast zx s_348_0 -> bv
        let s_348_1: Bits = Bits::new(s_348_0 as u128, 1u16);
        // C s_348_2: const #1u : u8
        let s_348_2: bool = true;
        // C s_348_3: cast zx s_348_2 -> bv
        let s_348_3: Bits = Bits::new(s_348_2 as u128, 1u16);
        // D s_348_4: cmp-eq s_348_1 s_348_3
        let s_348_4: bool = ((s_348_1) == (s_348_3));
        // D s_348_5: write-var gs#109126 <= s_348_4
        fn_state.gs_109126 = s_348_4;
        // N s_348_6: jump b282
        return block_282(state, tracer, fn_state);
    }
    fn block_349<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_349_0: const #432u : u32
        let s_349_0: u32 = 432;
        // D s_349_1: read-reg s_349_0:u8
        let s_349_1: u8 = {
            let value = state.read_register::<u8>(s_349_0 as isize);
            tracer.read_register(s_349_0 as isize, value);
            value
        };
        // D s_349_2: call ELUsingAArch32(s_349_1)
        let s_349_2: bool = ELUsingAArch32(state, tracer, s_349_1);
        // D s_349_3: write-var gs#109125 <= s_349_2
        fn_state.gs_109125 = s_349_2;
        // N s_349_4: jump b280
        return block_280(state, tracer, fn_state);
    }
    fn block_350<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_350_0: const #5u : u8
        let s_350_0: u8 = 5;
        // C s_350_1: cast zx s_350_0 -> bv
        let s_350_1: Bits = Bits::new(s_350_0 as u128, 8u16);
        // C s_350_2: cast zx s_350_1 -> i
        let s_350_2: i128 = (s_350_1.value() as i128);
        // C s_350_3: cast reint s_350_2 -> i64
        let s_350_3: i64 = (s_350_2 as i64);
        // C s_350_4: cast zx s_350_3 -> i
        let s_350_4: i128 = (i128::try_from(s_350_3).unwrap());
        // C s_350_5: const #432u : u32
        let s_350_5: u32 = 432;
        // D s_350_6: read-reg s_350_5:u8
        let s_350_6: u8 = {
            let value = state.read_register::<u8>(s_350_5 as isize);
            tracer.read_register(s_350_5 as isize, value);
            value
        };
        // D s_350_7: call AArch64_AArch32SystemAccessTrap(s_350_6, s_350_4)
        let s_350_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_350_6,
            s_350_4,
        );
        // N s_350_8: return
        return;
    }
    fn block_351<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_351_0: read-var __MDCR_EL2_TDCC:u8
        let s_351_0: bool = fn_state.u__MDCR_EL2_TDCC;
        // D s_351_1: cast zx s_351_0 -> bv
        let s_351_1: Bits = Bits::new(s_351_0 as u128, 1u16);
        // C s_351_2: const #1u : u8
        let s_351_2: bool = true;
        // C s_351_3: cast zx s_351_2 -> bv
        let s_351_3: Bits = Bits::new(s_351_2 as u128, 1u16);
        // D s_351_4: cmp-eq s_351_1 s_351_3
        let s_351_4: bool = ((s_351_1) == (s_351_3));
        // D s_351_5: write-var gs#109124 <= s_351_4
        fn_state.gs_109124 = s_351_4;
        // N s_351_6: jump b277
        return block_277(state, tracer, fn_state);
    }
    fn block_352<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_352_0: const #432u : u32
        let s_352_0: u32 = 432;
        // D s_352_1: read-reg s_352_0:u8
        let s_352_1: u8 = {
            let value = state.read_register::<u8>(s_352_0 as isize);
            tracer.read_register(s_352_0 as isize, value);
            value
        };
        // D s_352_2: call ELUsingAArch32(s_352_1)
        let s_352_2: bool = ELUsingAArch32(state, tracer, s_352_1);
        // D s_352_3: not s_352_2
        let s_352_3: bool = !s_352_2;
        // D s_352_4: write-var gs#109123 <= s_352_3
        fn_state.gs_109123 = s_352_3;
        // N s_352_5: jump b275
        return block_275(state, tracer, fn_state);
    }
    fn block_353<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_353_0: const #() : ()
        let s_353_0: () = ();
        // S s_353_1: call EL2Enabled(s_353_0)
        let s_353_1: bool = EL2Enabled(state, tracer, s_353_0);
        // N s_353_2: branch s_353_1 b369 b354
        if s_353_1 {
            return block_369(state, tracer, fn_state);
        } else {
            return block_354(state, tracer, fn_state);
        };
    }
    fn block_354<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_354_0: const #0u : u8
        let s_354_0: bool = false;
        // D s_354_1: write-var gs#109156 <= s_354_0
        fn_state.gs_109156 = s_354_0;
        // N s_354_2: jump b355
        return block_355(state, tracer, fn_state);
    }
    fn block_355<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_355_0: read-var gs#109156:u8
        let s_355_0: bool = fn_state.gs_109156;
        // N s_355_1: branch s_355_0 b368 b356
        if s_355_0 {
            return block_368(state, tracer, fn_state);
        } else {
            return block_356(state, tracer, fn_state);
        };
    }
    fn block_356<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_356_0: const #0u : u8
        let s_356_0: bool = false;
        // D s_356_1: write-var gs#109157 <= s_356_0
        fn_state.gs_109157 = s_356_0;
        // N s_356_2: jump b357
        return block_357(state, tracer, fn_state);
    }
    fn block_357<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_357_0: read-var gs#109157:u8
        let s_357_0: bool = fn_state.gs_109157;
        // N s_357_1: branch s_357_0 b367 b358
        if s_357_0 {
            return block_367(state, tracer, fn_state);
        } else {
            return block_358(state, tracer, fn_state);
        };
    }
    fn block_358<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_358_0: const #() : ()
        let s_358_0: () = ();
        // S s_358_1: call EL2Enabled(s_358_0)
        let s_358_1: bool = EL2Enabled(state, tracer, s_358_0);
        // N s_358_2: branch s_358_1 b366 b359
        if s_358_1 {
            return block_366(state, tracer, fn_state);
        } else {
            return block_359(state, tracer, fn_state);
        };
    }
    fn block_359<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_359_0: const #0u : u8
        let s_359_0: bool = false;
        // D s_359_1: write-var gs#109158 <= s_359_0
        fn_state.gs_109158 = s_359_0;
        // N s_359_2: jump b360
        return block_360(state, tracer, fn_state);
    }
    fn block_360<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_360_0: read-var gs#109158:u8
        let s_360_0: bool = fn_state.gs_109158;
        // N s_360_1: branch s_360_0 b365 b361
        if s_360_0 {
            return block_365(state, tracer, fn_state);
        } else {
            return block_361(state, tracer, fn_state);
        };
    }
    fn block_361<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_361_0: const #0u : u8
        let s_361_0: bool = false;
        // D s_361_1: write-var gs#109159 <= s_361_0
        fn_state.gs_109159 = s_361_0;
        // N s_361_2: jump b362
        return block_362(state, tracer, fn_state);
    }
    fn block_362<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_362_0: read-var gs#109159:u8
        let s_362_0: bool = fn_state.gs_109159;
        // N s_362_1: branch s_362_0 b364 b363
        if s_362_0 {
            return block_364(state, tracer, fn_state);
        } else {
            return block_363(state, tracer, fn_state);
        };
    }
    fn block_363<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_363_0: panic
        panic!("{:?}", ());
        // N s_363_1: return
        return;
    }
    fn block_364<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_364_0: const #0u : u8
        let s_364_0: u8 = 0;
        // C s_364_1: cast zx s_364_0 -> bv
        let s_364_1: Bits = Bits::new(s_364_0 as u128, 8u16);
        // C s_364_2: cast zx s_364_1 -> i
        let s_364_2: i128 = (s_364_1.value() as i128);
        // C s_364_3: cast reint s_364_2 -> i64
        let s_364_3: i64 = (s_364_2 as i64);
        // C s_364_4: cast zx s_364_3 -> i
        let s_364_4: i128 = (i128::try_from(s_364_3).unwrap());
        // S s_364_5: call AArch32_TakeHypTrapException(s_364_4)
        let s_364_5: () = AArch32_TakeHypTrapException(state, tracer, s_364_4);
        // N s_364_6: return
        return;
    }
    fn block_365<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_365_0: const #() : ()
        let s_365_0: () = ();
        // S s_365_1: call HCR_read(s_365_0)
        let s_365_1: ProductType700c18a878c5601b = HCR_read(state, tracer, s_365_0);
        // S s_365_2: call _get_HCR_Type_TGE(s_365_1)
        let s_365_2: bool = u_get_HCR_Type_TGE(state, tracer, s_365_1);
        // S s_365_3: cast zx s_365_2 -> bv
        let s_365_3: Bits = Bits::new(s_365_2 as u128, 1u16);
        // C s_365_4: const #1u : u8
        let s_365_4: bool = true;
        // C s_365_5: cast zx s_365_4 -> bv
        let s_365_5: Bits = Bits::new(s_365_4 as u128, 1u16);
        // S s_365_6: cmp-eq s_365_3 s_365_5
        let s_365_6: bool = ((s_365_3) == (s_365_5));
        // D s_365_7: write-var gs#109159 <= s_365_6
        fn_state.gs_109159 = s_365_6;
        // N s_365_8: jump b362
        return block_362(state, tracer, fn_state);
    }
    fn block_366<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_366_0: const #432u : u32
        let s_366_0: u32 = 432;
        // D s_366_1: read-reg s_366_0:u8
        let s_366_1: u8 = {
            let value = state.read_register::<u8>(s_366_0 as isize);
            tracer.read_register(s_366_0 as isize, value);
            value
        };
        // D s_366_2: call ELUsingAArch32(s_366_1)
        let s_366_2: bool = ELUsingAArch32(state, tracer, s_366_1);
        // D s_366_3: write-var gs#109158 <= s_366_2
        fn_state.gs_109158 = s_366_2;
        // N s_366_4: jump b360
        return block_360(state, tracer, fn_state);
    }
    fn block_367<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_367_0: const #5u : u8
        let s_367_0: u8 = 5;
        // C s_367_1: cast zx s_367_0 -> bv
        let s_367_1: Bits = Bits::new(s_367_0 as u128, 8u16);
        // C s_367_2: cast zx s_367_1 -> i
        let s_367_2: i128 = (s_367_1.value() as i128);
        // C s_367_3: cast reint s_367_2 -> i64
        let s_367_3: i64 = (s_367_2 as i64);
        // C s_367_4: cast zx s_367_3 -> i
        let s_367_4: i128 = (i128::try_from(s_367_3).unwrap());
        // C s_367_5: const #432u : u32
        let s_367_5: u32 = 432;
        // D s_367_6: read-reg s_367_5:u8
        let s_367_6: u8 = {
            let value = state.read_register::<u8>(s_367_5 as isize);
            tracer.read_register(s_367_5 as isize, value);
            value
        };
        // D s_367_7: call AArch64_AArch32SystemAccessTrap(s_367_6, s_367_4)
        let s_367_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_367_6,
            s_367_4,
        );
        // N s_367_8: return
        return;
    }
    fn block_368<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_368_0: const #102552u : u32
        let s_368_0: u32 = 102552;
        // D s_368_1: read-reg s_368_0:struct
        let s_368_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_368_0 as isize);
            tracer.read_register(s_368_0 as isize, value);
            value
        };
        // D s_368_2: call _get_HCR_EL2_Type_TGE(s_368_1)
        let s_368_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_368_1);
        // D s_368_3: cast zx s_368_2 -> bv
        let s_368_3: Bits = Bits::new(s_368_2 as u128, 1u16);
        // C s_368_4: const #1u : u8
        let s_368_4: bool = true;
        // C s_368_5: cast zx s_368_4 -> bv
        let s_368_5: Bits = Bits::new(s_368_4 as u128, 1u16);
        // D s_368_6: cmp-eq s_368_3 s_368_5
        let s_368_6: bool = ((s_368_3) == (s_368_5));
        // D s_368_7: write-var gs#109157 <= s_368_6
        fn_state.gs_109157 = s_368_6;
        // N s_368_8: jump b357
        return block_357(state, tracer, fn_state);
    }
    fn block_369<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_369_0: const #432u : u32
        let s_369_0: u32 = 432;
        // D s_369_1: read-reg s_369_0:u8
        let s_369_1: u8 = {
            let value = state.read_register::<u8>(s_369_0 as isize);
            tracer.read_register(s_369_0 as isize, value);
            value
        };
        // D s_369_2: call ELUsingAArch32(s_369_1)
        let s_369_2: bool = ELUsingAArch32(state, tracer, s_369_1);
        // D s_369_3: not s_369_2
        let s_369_3: bool = !s_369_2;
        // D s_369_4: write-var gs#109156 <= s_369_3
        fn_state.gs_109156 = s_369_3;
        // N s_369_5: jump b355
        return block_355(state, tracer, fn_state);
    }
    fn block_370<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_370_0: read-var __DBGDSCRext_UDCCdis:u8
        let s_370_0: bool = fn_state.u__DBGDSCRext_UDCCdis;
        // D s_370_1: cast zx s_370_0 -> bv
        let s_370_1: Bits = Bits::new(s_370_0 as u128, 1u16);
        // C s_370_2: const #1u : u8
        let s_370_2: bool = true;
        // C s_370_3: cast zx s_370_2 -> bv
        let s_370_3: Bits = Bits::new(s_370_2 as u128, 1u16);
        // D s_370_4: cmp-eq s_370_1 s_370_3
        let s_370_4: bool = ((s_370_1) == (s_370_3));
        // D s_370_5: write-var gs#109122 <= s_370_4
        fn_state.gs_109122 = s_370_4;
        // N s_370_6: jump b272
        return block_272(state, tracer, fn_state);
    }
    fn block_371<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_371_0: const #() : ()
        let s_371_0: () = ();
        // S s_371_1: call EL2Enabled(s_371_0)
        let s_371_1: bool = EL2Enabled(state, tracer, s_371_0);
        // N s_371_2: branch s_371_1 b379 b372
        if s_371_1 {
            return block_379(state, tracer, fn_state);
        } else {
            return block_372(state, tracer, fn_state);
        };
    }
    fn block_372<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_372_0: const #0u : u8
        let s_372_0: bool = false;
        // D s_372_1: write-var gs#109160 <= s_372_0
        fn_state.gs_109160 = s_372_0;
        // N s_372_2: jump b373
        return block_373(state, tracer, fn_state);
    }
    fn block_373<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_373_0: read-var gs#109160:u8
        let s_373_0: bool = fn_state.gs_109160;
        // N s_373_1: branch s_373_0 b378 b374
        if s_373_0 {
            return block_378(state, tracer, fn_state);
        } else {
            return block_374(state, tracer, fn_state);
        };
    }
    fn block_374<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_374_0: const #0u : u8
        let s_374_0: bool = false;
        // D s_374_1: write-var gs#109161 <= s_374_0
        fn_state.gs_109161 = s_374_0;
        // N s_374_2: jump b375
        return block_375(state, tracer, fn_state);
    }
    fn block_375<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_375_0: read-var gs#109161:u8
        let s_375_0: bool = fn_state.gs_109161;
        // N s_375_1: branch s_375_0 b377 b376
        if s_375_0 {
            return block_377(state, tracer, fn_state);
        } else {
            return block_376(state, tracer, fn_state);
        };
    }
    fn block_376<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_376_0: const #5u : u8
        let s_376_0: u8 = 5;
        // C s_376_1: cast zx s_376_0 -> bv
        let s_376_1: Bits = Bits::new(s_376_0 as u128, 8u16);
        // C s_376_2: cast zx s_376_1 -> i
        let s_376_2: i128 = (s_376_1.value() as i128);
        // C s_376_3: cast reint s_376_2 -> i64
        let s_376_3: i64 = (s_376_2 as i64);
        // C s_376_4: cast zx s_376_3 -> i
        let s_376_4: i128 = (i128::try_from(s_376_3).unwrap());
        // C s_376_5: const #440u : u32
        let s_376_5: u32 = 440;
        // D s_376_6: read-reg s_376_5:u8
        let s_376_6: u8 = {
            let value = state.read_register::<u8>(s_376_5 as isize);
            tracer.read_register(s_376_5 as isize, value);
            value
        };
        // D s_376_7: call AArch64_AArch32SystemAccessTrap(s_376_6, s_376_4)
        let s_376_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_376_6,
            s_376_4,
        );
        // N s_376_8: return
        return;
    }
    fn block_377<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_377_0: const #5u : u8
        let s_377_0: u8 = 5;
        // C s_377_1: cast zx s_377_0 -> bv
        let s_377_1: Bits = Bits::new(s_377_0 as u128, 8u16);
        // C s_377_2: cast zx s_377_1 -> i
        let s_377_2: i128 = (s_377_1.value() as i128);
        // C s_377_3: cast reint s_377_2 -> i64
        let s_377_3: i64 = (s_377_2 as i64);
        // C s_377_4: cast zx s_377_3 -> i
        let s_377_4: i128 = (i128::try_from(s_377_3).unwrap());
        // C s_377_5: const #432u : u32
        let s_377_5: u32 = 432;
        // D s_377_6: read-reg s_377_5:u8
        let s_377_6: u8 = {
            let value = state.read_register::<u8>(s_377_5 as isize);
            tracer.read_register(s_377_5 as isize, value);
            value
        };
        // D s_377_7: call AArch64_AArch32SystemAccessTrap(s_377_6, s_377_4)
        let s_377_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_377_6,
            s_377_4,
        );
        // N s_377_8: return
        return;
    }
    fn block_378<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_378_0: const #102552u : u32
        let s_378_0: u32 = 102552;
        // D s_378_1: read-reg s_378_0:struct
        let s_378_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_378_0 as isize);
            tracer.read_register(s_378_0 as isize, value);
            value
        };
        // D s_378_2: call _get_HCR_EL2_Type_TGE(s_378_1)
        let s_378_2: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_378_1);
        // D s_378_3: cast zx s_378_2 -> bv
        let s_378_3: Bits = Bits::new(s_378_2 as u128, 1u16);
        // C s_378_4: const #1u : u8
        let s_378_4: bool = true;
        // C s_378_5: cast zx s_378_4 -> bv
        let s_378_5: Bits = Bits::new(s_378_4 as u128, 1u16);
        // D s_378_6: cmp-eq s_378_3 s_378_5
        let s_378_6: bool = ((s_378_3) == (s_378_5));
        // D s_378_7: write-var gs#109161 <= s_378_6
        fn_state.gs_109161 = s_378_6;
        // N s_378_8: jump b375
        return block_375(state, tracer, fn_state);
    }
    fn block_379<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_379_0: const #432u : u32
        let s_379_0: u32 = 432;
        // D s_379_1: read-reg s_379_0:u8
        let s_379_1: u8 = {
            let value = state.read_register::<u8>(s_379_0 as isize);
            tracer.read_register(s_379_0 as isize, value);
            value
        };
        // D s_379_2: call ELUsingAArch32(s_379_1)
        let s_379_2: bool = ELUsingAArch32(state, tracer, s_379_1);
        // D s_379_3: not s_379_2
        let s_379_3: bool = !s_379_2;
        // D s_379_4: write-var gs#109160 <= s_379_3
        fn_state.gs_109160 = s_379_3;
        // N s_379_5: jump b373
        return block_373(state, tracer, fn_state);
    }
    fn block_380<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_380_0: read-var __MDSCR_EL1_TDCC:u8
        let s_380_0: bool = fn_state.u__MDSCR_EL1_TDCC;
        // D s_380_1: cast zx s_380_0 -> bv
        let s_380_1: Bits = Bits::new(s_380_0 as u128, 1u16);
        // C s_380_2: const #1u : u8
        let s_380_2: bool = true;
        // C s_380_3: cast zx s_380_2 -> bv
        let s_380_3: Bits = Bits::new(s_380_2 as u128, 1u16);
        // D s_380_4: cmp-eq s_380_1 s_380_3
        let s_380_4: bool = ((s_380_1) == (s_380_3));
        // D s_380_5: write-var gs#109121 <= s_380_4
        fn_state.gs_109121 = s_380_4;
        // N s_380_6: jump b269
        return block_269(state, tracer, fn_state);
    }
    fn block_381<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_381_0: panic
        panic!("{:?}", ());
        // N s_381_1: return
        return;
    }
    fn block_382<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_382_0: read-var __MDCR_EL3_TDA:u8
        let s_382_0: bool = fn_state.u__MDCR_EL3_TDA;
        // D s_382_1: cast zx s_382_0 -> bv
        let s_382_1: Bits = Bits::new(s_382_0 as u128, 1u16);
        // C s_382_2: const #1u : u8
        let s_382_2: bool = true;
        // C s_382_3: cast zx s_382_2 -> bv
        let s_382_3: Bits = Bits::new(s_382_2 as u128, 1u16);
        // D s_382_4: cmp-eq s_382_1 s_382_3
        let s_382_4: bool = ((s_382_1) == (s_382_3));
        // D s_382_5: write-var gs#109120 <= s_382_4
        fn_state.gs_109120 = s_382_4;
        // N s_382_6: jump b266
        return block_266(state, tracer, fn_state);
    }
    fn block_383<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_383_0: const #424u : u32
        let s_383_0: u32 = 424;
        // D s_383_1: read-reg s_383_0:u8
        let s_383_1: u8 = {
            let value = state.read_register::<u8>(s_383_0 as isize);
            tracer.read_register(s_383_0 as isize, value);
            value
        };
        // D s_383_2: call ELUsingAArch32(s_383_1)
        let s_383_2: bool = ELUsingAArch32(state, tracer, s_383_1);
        // D s_383_3: not s_383_2
        let s_383_3: bool = !s_383_2;
        // D s_383_4: write-var gs#109119 <= s_383_3
        fn_state.gs_109119 = s_383_3;
        // N s_383_5: jump b264
        return block_264(state, tracer, fn_state);
    }
    fn block_384<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_384_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_384_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_384_1: call __IMPDEF_boolean(s_384_0)
        let s_384_1: bool = u__IMPDEF_boolean(state, tracer, s_384_0);
        // D s_384_2: write-var gs#109118 <= s_384_1
        fn_state.gs_109118 = s_384_1;
        // N s_384_3: jump b262
        return block_262(state, tracer, fn_state);
    }
    fn block_385<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_385_0: const #() : ()
        let s_385_0: () = ();
        // S s_385_1: call EDSCR_read(s_385_0)
        let s_385_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_385_0);
        // S s_385_2: call _get_EDSCR_Type_SDD(s_385_1)
        let s_385_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_385_1);
        // S s_385_3: cast zx s_385_2 -> bv
        let s_385_3: Bits = Bits::new(s_385_2 as u128, 1u16);
        // C s_385_4: const #1u : u8
        let s_385_4: bool = true;
        // C s_385_5: cast zx s_385_4 -> bv
        let s_385_5: Bits = Bits::new(s_385_4 as u128, 1u16);
        // S s_385_6: cmp-eq s_385_3 s_385_5
        let s_385_6: bool = ((s_385_3) == (s_385_5));
        // D s_385_7: write-var gs#109117 <= s_385_6
        fn_state.gs_109117 = s_385_6;
        // N s_385_8: jump b260
        return block_260(state, tracer, fn_state);
    }
    fn block_386<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_386_0: const #424u : u32
        let s_386_0: u32 = 424;
        // D s_386_1: read-reg s_386_0:u8
        let s_386_1: u8 = {
            let value = state.read_register::<u8>(s_386_0 as isize);
            tracer.read_register(s_386_0 as isize, value);
            value
        };
        // C s_386_2: const #2u : u8
        let s_386_2: u8 = 2;
        // D s_386_3: cmp-lt s_386_1 s_386_2
        let s_386_3: bool = ((s_386_1) < (s_386_2));
        // D s_386_4: write-var gs#109116 <= s_386_3
        fn_state.gs_109116 = s_386_3;
        // N s_386_5: jump b258
        return block_258(state, tracer, fn_state);
    }
    fn block_387<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_387_0: panic
        panic!("{:?}", ());
        // N s_387_1: return
        return;
    }
    fn block_388<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_388_0: read-var __SDCR_TDCC:u8
        let s_388_0: bool = fn_state.u__SDCR_TDCC;
        // D s_388_1: cast zx s_388_0 -> bv
        let s_388_1: Bits = Bits::new(s_388_0 as u128, 1u16);
        // C s_388_2: const #1u : u8
        let s_388_2: bool = true;
        // C s_388_3: cast zx s_388_2 -> bv
        let s_388_3: Bits = Bits::new(s_388_2 as u128, 1u16);
        // D s_388_4: cmp-eq s_388_1 s_388_3
        let s_388_4: bool = ((s_388_1) == (s_388_3));
        // D s_388_5: write-var gs#109115 <= s_388_4
        fn_state.gs_109115 = s_388_4;
        // N s_388_6: jump b255
        return block_255(state, tracer, fn_state);
    }
    fn block_389<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_389_0: const #424u : u32
        let s_389_0: u32 = 424;
        // D s_389_1: read-reg s_389_0:u8
        let s_389_1: u8 = {
            let value = state.read_register::<u8>(s_389_0 as isize);
            tracer.read_register(s_389_0 as isize, value);
            value
        };
        // D s_389_2: call ELUsingAArch32(s_389_1)
        let s_389_2: bool = ELUsingAArch32(state, tracer, s_389_1);
        // D s_389_3: write-var gs#109114 <= s_389_2
        fn_state.gs_109114 = s_389_2;
        // N s_389_4: jump b253
        return block_253(state, tracer, fn_state);
    }
    fn block_390<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_390_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_390_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_390_1: call __IMPDEF_boolean(s_390_0)
        let s_390_1: bool = u__IMPDEF_boolean(state, tracer, s_390_0);
        // D s_390_2: write-var gs#109113 <= s_390_1
        fn_state.gs_109113 = s_390_1;
        // N s_390_3: jump b251
        return block_251(state, tracer, fn_state);
    }
    fn block_391<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_391_0: const #() : ()
        let s_391_0: () = ();
        // S s_391_1: call EDSCR_read(s_391_0)
        let s_391_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_391_0);
        // S s_391_2: call _get_EDSCR_Type_SDD(s_391_1)
        let s_391_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_391_1);
        // S s_391_3: cast zx s_391_2 -> bv
        let s_391_3: Bits = Bits::new(s_391_2 as u128, 1u16);
        // C s_391_4: const #1u : u8
        let s_391_4: bool = true;
        // C s_391_5: cast zx s_391_4 -> bv
        let s_391_5: Bits = Bits::new(s_391_4 as u128, 1u16);
        // S s_391_6: cmp-eq s_391_3 s_391_5
        let s_391_6: bool = ((s_391_3) == (s_391_5));
        // D s_391_7: write-var gs#109112 <= s_391_6
        fn_state.gs_109112 = s_391_6;
        // N s_391_8: jump b249
        return block_249(state, tracer, fn_state);
    }
    fn block_392<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_392_0: const #424u : u32
        let s_392_0: u32 = 424;
        // D s_392_1: read-reg s_392_0:u8
        let s_392_1: u8 = {
            let value = state.read_register::<u8>(s_392_0 as isize);
            tracer.read_register(s_392_0 as isize, value);
            value
        };
        // C s_392_2: const #2u : u8
        let s_392_2: u8 = 2;
        // D s_392_3: cmp-lt s_392_1 s_392_2
        let s_392_3: bool = ((s_392_1) < (s_392_2));
        // D s_392_4: write-var gs#109111 <= s_392_3
        fn_state.gs_109111 = s_392_3;
        // N s_392_5: jump b247
        return block_247(state, tracer, fn_state);
    }
    fn block_393<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_393_0: panic
        panic!("{:?}", ());
        // N s_393_1: return
        return;
    }
    fn block_394<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_394_0: read-var __MDCR_EL3_TDCC:u8
        let s_394_0: bool = fn_state.u__MDCR_EL3_TDCC;
        // D s_394_1: cast zx s_394_0 -> bv
        let s_394_1: Bits = Bits::new(s_394_0 as u128, 1u16);
        // C s_394_2: const #1u : u8
        let s_394_2: bool = true;
        // C s_394_3: cast zx s_394_2 -> bv
        let s_394_3: Bits = Bits::new(s_394_2 as u128, 1u16);
        // D s_394_4: cmp-eq s_394_1 s_394_3
        let s_394_4: bool = ((s_394_1) == (s_394_3));
        // D s_394_5: write-var gs#109110 <= s_394_4
        fn_state.gs_109110 = s_394_4;
        // N s_394_6: jump b244
        return block_244(state, tracer, fn_state);
    }
    fn block_395<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_395_0: const #424u : u32
        let s_395_0: u32 = 424;
        // D s_395_1: read-reg s_395_0:u8
        let s_395_1: u8 = {
            let value = state.read_register::<u8>(s_395_0 as isize);
            tracer.read_register(s_395_0 as isize, value);
            value
        };
        // D s_395_2: call ELUsingAArch32(s_395_1)
        let s_395_2: bool = ELUsingAArch32(state, tracer, s_395_1);
        // D s_395_3: not s_395_2
        let s_395_3: bool = !s_395_2;
        // D s_395_4: write-var gs#109109 <= s_395_3
        fn_state.gs_109109 = s_395_3;
        // N s_395_5: jump b242
        return block_242(state, tracer, fn_state);
    }
    fn block_396<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_396_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_396_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_396_1: call __IMPDEF_boolean(s_396_0)
        let s_396_1: bool = u__IMPDEF_boolean(state, tracer, s_396_0);
        // D s_396_2: write-var gs#109108 <= s_396_1
        fn_state.gs_109108 = s_396_1;
        // N s_396_3: jump b240
        return block_240(state, tracer, fn_state);
    }
    fn block_397<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_397_0: const #() : ()
        let s_397_0: () = ();
        // S s_397_1: call EDSCR_read(s_397_0)
        let s_397_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_397_0);
        // S s_397_2: call _get_EDSCR_Type_SDD(s_397_1)
        let s_397_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_397_1);
        // S s_397_3: cast zx s_397_2 -> bv
        let s_397_3: Bits = Bits::new(s_397_2 as u128, 1u16);
        // C s_397_4: const #1u : u8
        let s_397_4: bool = true;
        // C s_397_5: cast zx s_397_4 -> bv
        let s_397_5: Bits = Bits::new(s_397_4 as u128, 1u16);
        // S s_397_6: cmp-eq s_397_3 s_397_5
        let s_397_6: bool = ((s_397_3) == (s_397_5));
        // D s_397_7: write-var gs#109107 <= s_397_6
        fn_state.gs_109107 = s_397_6;
        // N s_397_8: jump b238
        return block_238(state, tracer, fn_state);
    }
    fn block_398<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_398_0: const #424u : u32
        let s_398_0: u32 = 424;
        // D s_398_1: read-reg s_398_0:u8
        let s_398_1: u8 = {
            let value = state.read_register::<u8>(s_398_0 as isize);
            tracer.read_register(s_398_0 as isize, value);
            value
        };
        // C s_398_2: const #2u : u8
        let s_398_2: u8 = 2;
        // D s_398_3: cmp-lt s_398_1 s_398_2
        let s_398_3: bool = ((s_398_1) < (s_398_2));
        // D s_398_4: write-var gs#109106 <= s_398_3
        fn_state.gs_109106 = s_398_3;
        // N s_398_5: jump b236
        return block_236(state, tracer, fn_state);
    }
    fn block_399<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_399_0: const #15s : i
        let s_399_0: i128 = 15;
        // D s_399_1: read-var t:i
        let s_399_1: i128 = fn_state.t;
        // D s_399_2: cmp-eq s_399_1 s_399_0
        let s_399_2: bool = ((s_399_1) == (s_399_0));
        // N s_399_3: branch s_399_2 b401 b400
        if s_399_2 {
            return block_401(state, tracer, fn_state);
        } else {
            return block_400(state, tracer, fn_state);
        };
    }
    fn block_400<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_400_0: const #() : ()
        let s_400_0: () = ();
        // S s_400_1: call DBGDSCRint_read(s_400_0)
        let s_400_1: ProductType700c18a878c5601b = DBGDSCRint_read(
            state,
            tracer,
            s_400_0,
        );
        // S s_400_2: call __get_DBGDSCRint(s_400_1)
        let s_400_2: ProductType700c18a878c5601b = u__get_DBGDSCRint(
            state,
            tracer,
            s_400_1,
        );
        // D s_400_3: write-var ga#172579 <= s_400_2
        fn_state.ga_172579 = s_400_2;
        // D s_400_4: read-var ga#172579.0:struct
        let s_400_4: u32 = fn_state.ga_172579._0;
        // D s_400_5: read-var t:i
        let s_400_5: i128 = fn_state.t;
        // D s_400_6: call R_set(s_400_5, s_400_4)
        let s_400_6: () = R_set(state, tracer, s_400_5, s_400_4);
        // N s_400_7: return
        return;
    }
    fn block_401<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_401_0: const #() : ()
        let s_401_0: () = ();
        // S s_401_1: call DBGDSCRint_read(s_401_0)
        let s_401_1: ProductType700c18a878c5601b = DBGDSCRint_read(
            state,
            tracer,
            s_401_0,
        );
        // D s_401_2: write-var ga#172576 <= s_401_1
        fn_state.ga_172576 = s_401_1;
        // D s_401_3: read-var ga#172576.0:struct
        let s_401_3: u32 = fn_state.ga_172576._0;
        // C s_401_4: const #28s : i
        let s_401_4: i128 = 28;
        // D s_401_5: cast zx s_401_3 -> bv
        let s_401_5: Bits = Bits::new(s_401_3 as u128, 32u16);
        // C s_401_6: const #1s : i64
        let s_401_6: i64 = 1;
        // C s_401_7: cast zx s_401_6 -> i
        let s_401_7: i128 = (i128::try_from(s_401_6).unwrap());
        // C s_401_8: const #3s : i
        let s_401_8: i128 = 3;
        // C s_401_9: add s_401_8 s_401_7
        let s_401_9: i128 = (s_401_8 + s_401_7);
        // D s_401_10: bit-extract s_401_5 s_401_4 s_401_9
        let s_401_10: Bits = (Bits::new(
            ((s_401_5) >> (s_401_4)).value(),
            u16::try_from(s_401_9).unwrap(),
        ));
        // D s_401_11: cast reint s_401_10 -> u8
        let s_401_11: u8 = (s_401_10.value() as u8);
        // C s_401_12: const #3s : i
        let s_401_12: i128 = 3;
        // D s_401_13: cast zx s_401_11 -> bv
        let s_401_13: Bits = Bits::new(s_401_11 as u128, 4u16);
        // C s_401_14: const #1s : i64
        let s_401_14: i64 = 1;
        // C s_401_15: cast zx s_401_14 -> i
        let s_401_15: i128 = (i128::try_from(s_401_14).unwrap());
        // C s_401_16: const #0s : i
        let s_401_16: i128 = 0;
        // C s_401_17: add s_401_16 s_401_15
        let s_401_17: i128 = (s_401_16 + s_401_15);
        // D s_401_18: bit-extract s_401_13 s_401_12 s_401_17
        let s_401_18: Bits = (Bits::new(
            ((s_401_13) >> (s_401_12)).value(),
            u16::try_from(s_401_17).unwrap(),
        ));
        // D s_401_19: cast reint s_401_18 -> u8
        let s_401_19: bool = ((s_401_18.value()) != 0);
        // C s_401_20: const #16984u : u32
        let s_401_20: u32 = 16984;
        // N s_401_21: write-reg s_401_20 <= s_401_19
        let s_401_21: () = {
            state.write_register::<bool>(s_401_20 as isize, s_401_19);
            tracer.write_register(s_401_20 as isize, s_401_19);
        };
        // C s_401_22: const #2s : i
        let s_401_22: i128 = 2;
        // D s_401_23: cast zx s_401_11 -> bv
        let s_401_23: Bits = Bits::new(s_401_11 as u128, 4u16);
        // C s_401_24: const #1s : i64
        let s_401_24: i64 = 1;
        // C s_401_25: cast zx s_401_24 -> i
        let s_401_25: i128 = (i128::try_from(s_401_24).unwrap());
        // C s_401_26: const #0s : i
        let s_401_26: i128 = 0;
        // C s_401_27: add s_401_26 s_401_25
        let s_401_27: i128 = (s_401_26 + s_401_25);
        // D s_401_28: bit-extract s_401_23 s_401_22 s_401_27
        let s_401_28: Bits = (Bits::new(
            ((s_401_23) >> (s_401_22)).value(),
            u16::try_from(s_401_27).unwrap(),
        ));
        // D s_401_29: cast reint s_401_28 -> u8
        let s_401_29: bool = ((s_401_28.value()) != 0);
        // C s_401_30: const #16997u : u32
        let s_401_30: u32 = 16997;
        // N s_401_31: write-reg s_401_30 <= s_401_29
        let s_401_31: () = {
            state.write_register::<bool>(s_401_30 as isize, s_401_29);
            tracer.write_register(s_401_30 as isize, s_401_29);
        };
        // C s_401_32: const #1s : i
        let s_401_32: i128 = 1;
        // D s_401_33: cast zx s_401_11 -> bv
        let s_401_33: Bits = Bits::new(s_401_11 as u128, 4u16);
        // C s_401_34: const #1s : i64
        let s_401_34: i64 = 1;
        // C s_401_35: cast zx s_401_34 -> i
        let s_401_35: i128 = (i128::try_from(s_401_34).unwrap());
        // C s_401_36: const #0s : i
        let s_401_36: i128 = 0;
        // C s_401_37: add s_401_36 s_401_35
        let s_401_37: i128 = (s_401_36 + s_401_35);
        // D s_401_38: bit-extract s_401_33 s_401_32 s_401_37
        let s_401_38: Bits = (Bits::new(
            ((s_401_33) >> (s_401_32)).value(),
            u16::try_from(s_401_37).unwrap(),
        ));
        // D s_401_39: cast reint s_401_38 -> u8
        let s_401_39: bool = ((s_401_38.value()) != 0);
        // C s_401_40: const #16971u : u32
        let s_401_40: u32 = 16971;
        // N s_401_41: write-reg s_401_40 <= s_401_39
        let s_401_41: () = {
            state.write_register::<bool>(s_401_40 as isize, s_401_39);
            tracer.write_register(s_401_40 as isize, s_401_39);
        };
        // C s_401_42: const #0s : i
        let s_401_42: i128 = 0;
        // D s_401_43: cast zx s_401_11 -> bv
        let s_401_43: Bits = Bits::new(s_401_11 as u128, 4u16);
        // C s_401_44: const #1s : i64
        let s_401_44: i64 = 1;
        // C s_401_45: cast zx s_401_44 -> i
        let s_401_45: i128 = (i128::try_from(s_401_44).unwrap());
        // C s_401_46: const #0s : i
        let s_401_46: i128 = 0;
        // C s_401_47: add s_401_46 s_401_45
        let s_401_47: i128 = (s_401_46 + s_401_45);
        // D s_401_48: bit-extract s_401_43 s_401_42 s_401_47
        let s_401_48: Bits = (Bits::new(
            ((s_401_43) >> (s_401_42)).value(),
            u16::try_from(s_401_47).unwrap(),
        ));
        // D s_401_49: cast reint s_401_48 -> u8
        let s_401_49: bool = ((s_401_48.value()) != 0);
        // C s_401_50: const #16996u : u32
        let s_401_50: u32 = 16996;
        // N s_401_51: write-reg s_401_50 <= s_401_49
        let s_401_51: () = {
            state.write_register::<bool>(s_401_50 as isize, s_401_49);
            tracer.write_register(s_401_50 as isize, s_401_49);
        };
        // N s_401_52: return
        return;
    }
    fn block_402<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_402_0: const #70u : u32
        let s_402_0: u32 = 70;
        // S s_402_1: call ConstrainUnpredictableBool(s_402_0)
        let s_402_1: bool = ConstrainUnpredictableBool(state, tracer, s_402_0);
        // D s_402_2: write-var gs#109006 <= s_402_1
        fn_state.gs_109006 = s_402_1;
        // N s_402_3: jump b2
        return block_2(state, tracer, fn_state);
    }
}
