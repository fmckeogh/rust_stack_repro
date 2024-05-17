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
use u_get_HSTR_Type_T9::*;
use u_get_MDCR_EL3_Type_TPM::*;
use u_get_PMUSERENR_Type_SW::*;
use R_read::*;
use u_get_PMUSERENR_Type_EN::*;
use u_get_PMUSERENR_EL0_Type_SW::*;
use u_get_EDSCR_Type_SDD::*;
use u_get_HSTR_EL2_Type_T9::*;
use u_get_HCR_EL2_Type_TGE::*;
use EL2Enabled::*;
use u_get_MDCR_EL2_Type_TPM::*;
use u_get_PMUSERENR_EL0_Type_UEN::*;
use u_get_HCR_EL2_Type_E2H::*;
use HCR_read::*;
use u_get_HDFGWTR_EL2_Type_PMSWINC_EL0::*;
use HDCR_read::*;
use PMUSERENR_read::*;
use IsFeatureImplemented::*;
use u__IMPDEF_boolean::*;
use AArch64_AArch32SystemAccessTrap::*;
use u_get_HDCR_Type_TPM::*;
use HSTR_read::*;
use u_get_HCR_Type_TGE::*;
use ELUsingAArch32::*;
use u_get_PMUSERENR_EL0_Type_EN::*;
use Mk_PMSWINC_Type::*;
use PMSWINC_write::*;
use EDSCR_read::*;
use AArch32_TakeHypTrapException::*;
use common::*;
pub fn PMSWINC_SysRegWrite32_9b4c9b84c8e7597b<T: Tracer>(
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
        gs_133821: bool,
        gs_133798: bool,
        gs_133822: bool,
        gs_133843: bool,
        gs_133791: bool,
        gs_133795: bool,
        u__MDCR_EL3_TPM: bool,
        gs_133831: bool,
        gs_133816: bool,
        gs_133817: bool,
        gs_133837: bool,
        gs_133841: bool,
        u__HCR_TGE: bool,
        gs_133834: bool,
        gs_133815: bool,
        gs_133824: bool,
        gs_133840: bool,
        gs_133813: bool,
        gs_133833: bool,
        gs_133809: bool,
        gs_133803: bool,
        gs_133811: bool,
        gs_133835: bool,
        gs_133825: bool,
        u__PSTATE_EL: u8,
        gs_133818: bool,
        gs_133820: bool,
        gs_133838: bool,
        gs_133810: bool,
        u__MDCR_EL2_TPM: bool,
        gs_133842: bool,
        u__HCR_EL2_TGE: bool,
        gs_133786: bool,
        gs_133828: bool,
        u__SCR_EL3_FGTEn: bool,
        gs_133814: bool,
        gs_133827: bool,
        gs_133787: bool,
        u__HSTR_T9: bool,
        gs_133790: bool,
        gs_133808: bool,
        u__HDCR_TPM: bool,
        gs_133793: bool,
        gs_133807: bool,
        gs_133832: bool,
        gs_133812: bool,
        gs_133801: bool,
        gs_133830: bool,
        gs_133802: bool,
        u__HDFGWTR_EL2_PMSWINC_EL0: bool,
        gs_133804: bool,
        gs_133800: bool,
        gs_133789: bool,
        gs_133836: bool,
        gs_133826: bool,
        gs_133805: bool,
        gs_133792: bool,
        gs_133794: bool,
        gs_133799: bool,
        gs_133823: bool,
        gs_133796: bool,
        gs_133797: bool,
        gs_133839: bool,
        gs_133819: bool,
        gs_133788: bool,
        gs_133806: bool,
        gs_133829: bool,
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
        // C s_0_15: const #104936u : u32
        let s_0_15: u32 = 104936;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_HSTR_EL2_Type_T9(s_0_16)
        let s_0_17: bool = u_get_HSTR_EL2_Type_T9(state, tracer, s_0_16);
        // D s_0_18: write-var __HSTR_EL2_T9 <= s_0_17
        fn_state.u__HSTR_EL2_T9 = s_0_17;
        // C s_0_19: const #() : ()
        let s_0_19: () = ();
        // S s_0_20: call HSTR_read(s_0_19)
        let s_0_20: ProductType700c18a878c5601b = HSTR_read(state, tracer, s_0_19);
        // S s_0_21: call _get_HSTR_Type_T9(s_0_20)
        let s_0_21: bool = u_get_HSTR_Type_T9(state, tracer, s_0_20);
        // D s_0_22: write-var __HSTR_T9 <= s_0_21
        fn_state.u__HSTR_T9 = s_0_21;
        // C s_0_23: const #90704u : u32
        let s_0_23: u32 = 90704;
        // D s_0_24: read-reg s_0_23:struct
        let s_0_24: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_23 as isize);
            tracer.read_register(s_0_23 as isize, value);
            value
        };
        // D s_0_25: call _get_SCR_EL3_Type_FGTEn(s_0_24)
        let s_0_25: bool = u_get_SCR_EL3_Type_FGTEn(state, tracer, s_0_24);
        // D s_0_26: write-var __SCR_EL3_FGTEn <= s_0_25
        fn_state.u__SCR_EL3_FGTEn = s_0_25;
        // C s_0_27: const #17360u : u32
        let s_0_27: u32 = 17360;
        // D s_0_28: read-reg s_0_27:struct
        let s_0_28: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_27 as isize);
            tracer.read_register(s_0_27 as isize, value);
            value
        };
        // D s_0_29: call _get_HDFGWTR_EL2_Type_PMSWINC_EL0(s_0_28)
        let s_0_29: bool = u_get_HDFGWTR_EL2_Type_PMSWINC_EL0(state, tracer, s_0_28);
        // D s_0_30: write-var __HDFGWTR_EL2_PMSWINC_EL0 <= s_0_29
        fn_state.u__HDFGWTR_EL2_PMSWINC_EL0 = s_0_29;
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
        // C s_0_35: const #() : ()
        let s_0_35: () = ();
        // S s_0_36: call HDCR_read(s_0_35)
        let s_0_36: ProductType700c18a878c5601b = HDCR_read(state, tracer, s_0_35);
        // S s_0_37: call _get_HDCR_Type_TPM(s_0_36)
        let s_0_37: bool = u_get_HDCR_Type_TPM(state, tracer, s_0_36);
        // D s_0_38: write-var __HDCR_TPM <= s_0_37
        fn_state.u__HDCR_TPM = s_0_37;
        // D s_0_39: read-var __PSTATE_EL:u8
        let s_0_39: u8 = fn_state.u__PSTATE_EL;
        // D s_0_40: cast zx s_0_39 -> bv
        let s_0_40: Bits = Bits::new(s_0_39 as u128, 2u16);
        // C s_0_41: const #448u : u32
        let s_0_41: u32 = 448;
        // D s_0_42: read-reg s_0_41:u8
        let s_0_42: u8 = {
            let value = state.read_register::<u8>(s_0_41 as isize);
            tracer.read_register(s_0_41 as isize, value);
            value
        };
        // D s_0_43: cast zx s_0_42 -> bv
        let s_0_43: Bits = Bits::new(s_0_42 as u128, 2u16);
        // D s_0_44: cmp-eq s_0_40 s_0_43
        let s_0_44: bool = ((s_0_40) == (s_0_43));
        // N s_0_45: branch s_0_44 b100 b1
        if s_0_44 {
            return block_100(state, tracer, fn_state);
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
        // D s_5_0: read-var t:i
        let s_5_0: i128 = fn_state.t;
        // D s_5_1: call R_read(s_5_0)
        let s_5_1: u32 = R_read(state, tracer, s_5_0);
        // D s_5_2: call Mk_PMSWINC_Type(s_5_1)
        let s_5_2: ProductType700c18a878c5601b = Mk_PMSWINC_Type(state, tracer, s_5_1);
        // D s_5_3: call PMSWINC_write(s_5_2)
        let s_5_3: () = PMSWINC_write(state, tracer, s_5_2);
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
        // D s_7_1: write-var gs#133786 <= s_7_0
        fn_state.gs_133786 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#133786:u8
        let s_8_0: bool = fn_state.gs_133786;
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
        // D s_9_1: write-var gs#133787 <= s_9_0
        fn_state.gs_133787 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#133787:u8
        let s_10_0: bool = fn_state.gs_133787;
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
        // D s_11_1: write-var gs#133788 <= s_11_0
        fn_state.gs_133788 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#133788:u8
        let s_12_0: bool = fn_state.gs_133788;
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
        // D s_13_1: write-var gs#133789 <= s_13_0
        fn_state.gs_133789 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#133789:u8
        let s_14_0: bool = fn_state.gs_133789;
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
        // D s_15_1: write-var gs#133790 <= s_15_0
        fn_state.gs_133790 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#133790:u8
        let s_16_0: bool = fn_state.gs_133790;
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
        // D s_18_1: write-var gs#133791 <= s_18_0
        fn_state.gs_133791 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#133791:u8
        let s_19_0: bool = fn_state.gs_133791;
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
        // D s_20_1: write-var gs#133792 <= s_20_0
        fn_state.gs_133792 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#133792:u8
        let s_21_0: bool = fn_state.gs_133792;
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
        // D s_22_2: call Mk_PMSWINC_Type(s_22_1)
        let s_22_2: ProductType700c18a878c5601b = Mk_PMSWINC_Type(state, tracer, s_22_1);
        // D s_22_3: call PMSWINC_write(s_22_2)
        let s_22_3: () = PMSWINC_write(state, tracer, s_22_2);
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
        // D s_24_1: write-var gs#133793 <= s_24_0
        fn_state.gs_133793 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#133793:u8
        let s_25_0: bool = fn_state.gs_133793;
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
        // D s_28_7: write-var gs#133793 <= s_28_6
        fn_state.gs_133793 = s_28_6;
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
        // D s_29_5: write-var gs#133792 <= s_29_4
        fn_state.gs_133792 = s_29_4;
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
        // D s_30_4: write-var gs#133791 <= s_30_3
        fn_state.gs_133791 = s_30_3;
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
        // D s_32_5: write-var gs#133790 <= s_32_4
        fn_state.gs_133790 = s_32_4;
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
        // D s_33_4: write-var gs#133789 <= s_33_3
        fn_state.gs_133789 = s_33_3;
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
        // D s_34_2: write-var gs#133788 <= s_34_1
        fn_state.gs_133788 = s_34_1;
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
        // D s_35_7: write-var gs#133787 <= s_35_6
        fn_state.gs_133787 = s_35_6;
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
        // D s_36_4: write-var gs#133786 <= s_36_3
        fn_state.gs_133786 = s_36_3;
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
        // N s_37_2: branch s_37_1 b99 b38
        if s_37_1 {
            return block_99(state, tracer, fn_state);
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
        // D s_38_1: write-var gs#133794 <= s_38_0
        fn_state.gs_133794 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#133794:u8
        let s_39_0: bool = fn_state.gs_133794;
        // N s_39_1: branch s_39_0 b98 b40
        if s_39_0 {
            return block_98(state, tracer, fn_state);
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
        // D s_40_1: write-var gs#133795 <= s_40_0
        fn_state.gs_133795 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var gs#133795:u8
        let s_41_0: bool = fn_state.gs_133795;
        // N s_41_1: branch s_41_0 b97 b42
        if s_41_0 {
            return block_97(state, tracer, fn_state);
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
        // D s_42_1: write-var gs#133796 <= s_42_0
        fn_state.gs_133796 = s_42_0;
        // N s_42_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var gs#133796:u8
        let s_43_0: bool = fn_state.gs_133796;
        // N s_43_1: branch s_43_0 b96 b44
        if s_43_0 {
            return block_96(state, tracer, fn_state);
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
        // D s_44_1: write-var gs#133797 <= s_44_0
        fn_state.gs_133797 = s_44_0;
        // N s_44_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var gs#133797:u8
        let s_45_0: bool = fn_state.gs_133797;
        // N s_45_1: branch s_45_0 b95 b46
        if s_45_0 {
            return block_95(state, tracer, fn_state);
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
        // D s_46_1: write-var gs#133798 <= s_46_0
        fn_state.gs_133798 = s_46_0;
        // N s_46_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var gs#133798:u8
        let s_47_0: bool = fn_state.gs_133798;
        // N s_47_1: branch s_47_0 b94 b48
        if s_47_0 {
            return block_94(state, tracer, fn_state);
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
        // N s_48_2: branch s_48_1 b93 b49
        if s_48_1 {
            return block_93(state, tracer, fn_state);
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
        // D s_49_1: write-var gs#133799 <= s_49_0
        fn_state.gs_133799 = s_49_0;
        // N s_49_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var gs#133799:u8
        let s_50_0: bool = fn_state.gs_133799;
        // N s_50_1: branch s_50_0 b92 b51
        if s_50_0 {
            return block_92(state, tracer, fn_state);
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
        // D s_51_1: write-var gs#133800 <= s_51_0
        fn_state.gs_133800 = s_51_0;
        // N s_51_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var gs#133800:u8
        let s_52_0: bool = fn_state.gs_133800;
        // N s_52_1: branch s_52_0 b91 b53
        if s_52_0 {
            return block_91(state, tracer, fn_state);
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
        // N s_53_2: branch s_53_1 b90 b54
        if s_53_1 {
            return block_90(state, tracer, fn_state);
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
        // D s_54_1: write-var gs#133801 <= s_54_0
        fn_state.gs_133801 = s_54_0;
        // N s_54_2: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var gs#133801:u8
        let s_55_0: bool = fn_state.gs_133801;
        // N s_55_1: branch s_55_0 b89 b56
        if s_55_0 {
            return block_89(state, tracer, fn_state);
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
        // D s_56_1: write-var gs#133802 <= s_56_0
        fn_state.gs_133802 = s_56_0;
        // N s_56_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var gs#133802:u8
        let s_57_0: bool = fn_state.gs_133802;
        // N s_57_1: branch s_57_0 b88 b58
        if s_57_0 {
            return block_88(state, tracer, fn_state);
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
        // N s_58_2: branch s_58_1 b87 b59
        if s_58_1 {
            return block_87(state, tracer, fn_state);
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
        // D s_59_1: write-var gs#133803 <= s_59_0
        fn_state.gs_133803 = s_59_0;
        // N s_59_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var gs#133803:u8
        let s_60_0: bool = fn_state.gs_133803;
        // N s_60_1: branch s_60_0 b86 b61
        if s_60_0 {
            return block_86(state, tracer, fn_state);
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
        // D s_61_1: write-var gs#133804 <= s_61_0
        fn_state.gs_133804 = s_61_0;
        // N s_61_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var gs#133804:u8
        let s_62_0: bool = fn_state.gs_133804;
        // N s_62_1: branch s_62_0 b85 b63
        if s_62_0 {
            return block_85(state, tracer, fn_state);
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
        // N s_63_2: branch s_63_1 b84 b64
        if s_63_1 {
            return block_84(state, tracer, fn_state);
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
        // D s_64_1: write-var gs#133805 <= s_64_0
        fn_state.gs_133805 = s_64_0;
        // N s_64_2: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_65_0: read-var gs#133805:u8
        let s_65_0: bool = fn_state.gs_133805;
        // N s_65_1: branch s_65_0 b83 b66
        if s_65_0 {
            return block_83(state, tracer, fn_state);
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
        // D s_66_1: write-var gs#133806 <= s_66_0
        fn_state.gs_133806 = s_66_0;
        // N s_66_2: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var gs#133806:u8
        let s_67_0: bool = fn_state.gs_133806;
        // N s_67_1: branch s_67_0 b82 b68
        if s_67_0 {
            return block_82(state, tracer, fn_state);
        } else {
            return block_68(state, tracer, fn_state);
        };
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #424u : u32
        let s_68_0: u32 = 424;
        // D s_68_1: read-reg s_68_0:u8
        let s_68_1: u8 = {
            let value = state.read_register::<u8>(s_68_0 as isize);
            tracer.read_register(s_68_0 as isize, value);
            value
        };
        // C s_68_2: const #2u : u8
        let s_68_2: u8 = 2;
        // D s_68_3: cmp-lt s_68_1 s_68_2
        let s_68_3: bool = ((s_68_1) < (s_68_2));
        // N s_68_4: branch s_68_3 b81 b69
        if s_68_3 {
            return block_81(state, tracer, fn_state);
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
        // D s_69_1: write-var gs#133807 <= s_69_0
        fn_state.gs_133807 = s_69_0;
        // N s_69_2: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_70_0: read-var gs#133807:u8
        let s_70_0: bool = fn_state.gs_133807;
        // N s_70_1: branch s_70_0 b80 b71
        if s_70_0 {
            return block_80(state, tracer, fn_state);
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
        // D s_71_1: write-var gs#133808 <= s_71_0
        fn_state.gs_133808 = s_71_0;
        // N s_71_2: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_72_0: read-var gs#133808:u8
        let s_72_0: bool = fn_state.gs_133808;
        // N s_72_1: branch s_72_0 b74 b73
        if s_72_0 {
            return block_74(state, tracer, fn_state);
        } else {
            return block_73(state, tracer, fn_state);
        };
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_73_0: read-var t:i
        let s_73_0: i128 = fn_state.t;
        // D s_73_1: call R_read(s_73_0)
        let s_73_1: u32 = R_read(state, tracer, s_73_0);
        // D s_73_2: call Mk_PMSWINC_Type(s_73_1)
        let s_73_2: ProductType700c18a878c5601b = Mk_PMSWINC_Type(state, tracer, s_73_1);
        // D s_73_3: call PMSWINC_write(s_73_2)
        let s_73_3: () = PMSWINC_write(state, tracer, s_73_2);
        // N s_73_4: return
        return;
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #() : ()
        let s_74_0: () = ();
        // S s_74_1: call Halted(s_74_0)
        let s_74_1: bool = Halted(state, tracer, s_74_0);
        // N s_74_2: branch s_74_1 b79 b75
        if s_74_1 {
            return block_79(state, tracer, fn_state);
        } else {
            return block_75(state, tracer, fn_state);
        };
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #0u : u8
        let s_75_0: bool = false;
        // D s_75_1: write-var gs#133809 <= s_75_0
        fn_state.gs_133809 = s_75_0;
        // N s_75_2: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_76_0: read-var gs#133809:u8
        let s_76_0: bool = fn_state.gs_133809;
        // N s_76_1: branch s_76_0 b78 b77
        if s_76_0 {
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
        // C s_77_5: const #424u : u32
        let s_77_5: u32 = 424;
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
        // N s_78_0: panic
        panic!("{:?}", ());
        // N s_78_1: return
        return;
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #() : ()
        let s_79_0: () = ();
        // S s_79_1: call EDSCR_read(s_79_0)
        let s_79_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_79_0);
        // S s_79_2: call _get_EDSCR_Type_SDD(s_79_1)
        let s_79_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_79_1);
        // S s_79_3: cast zx s_79_2 -> bv
        let s_79_3: Bits = Bits::new(s_79_2 as u128, 1u16);
        // C s_79_4: const #1u : u8
        let s_79_4: bool = true;
        // C s_79_5: cast zx s_79_4 -> bv
        let s_79_5: Bits = Bits::new(s_79_4 as u128, 1u16);
        // S s_79_6: cmp-eq s_79_3 s_79_5
        let s_79_6: bool = ((s_79_3) == (s_79_5));
        // D s_79_7: write-var gs#133809 <= s_79_6
        fn_state.gs_133809 = s_79_6;
        // N s_79_8: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_80_0: read-var __MDCR_EL3_TPM:u8
        let s_80_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_80_1: cast zx s_80_0 -> bv
        let s_80_1: Bits = Bits::new(s_80_0 as u128, 1u16);
        // C s_80_2: const #1u : u8
        let s_80_2: bool = true;
        // C s_80_3: cast zx s_80_2 -> bv
        let s_80_3: Bits = Bits::new(s_80_2 as u128, 1u16);
        // D s_80_4: cmp-eq s_80_1 s_80_3
        let s_80_4: bool = ((s_80_1) == (s_80_3));
        // D s_80_5: write-var gs#133808 <= s_80_4
        fn_state.gs_133808 = s_80_4;
        // N s_80_6: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_81_0: const #424u : u32
        let s_81_0: u32 = 424;
        // D s_81_1: read-reg s_81_0:u8
        let s_81_1: u8 = {
            let value = state.read_register::<u8>(s_81_0 as isize);
            tracer.read_register(s_81_0 as isize, value);
            value
        };
        // D s_81_2: call ELUsingAArch32(s_81_1)
        let s_81_2: bool = ELUsingAArch32(state, tracer, s_81_1);
        // D s_81_3: not s_81_2
        let s_81_3: bool = !s_81_2;
        // D s_81_4: write-var gs#133807 <= s_81_3
        fn_state.gs_133807 = s_81_3;
        // N s_81_5: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #3u : u8
        let s_82_0: u8 = 3;
        // C s_82_1: cast zx s_82_0 -> bv
        let s_82_1: Bits = Bits::new(s_82_0 as u128, 8u16);
        // C s_82_2: cast zx s_82_1 -> i
        let s_82_2: i128 = (s_82_1.value() as i128);
        // C s_82_3: cast reint s_82_2 -> i64
        let s_82_3: i64 = (s_82_2 as i64);
        // C s_82_4: cast zx s_82_3 -> i
        let s_82_4: i128 = (i128::try_from(s_82_3).unwrap());
        // S s_82_5: call AArch32_TakeHypTrapException(s_82_4)
        let s_82_5: () = AArch32_TakeHypTrapException(state, tracer, s_82_4);
        // N s_82_6: return
        return;
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_83_0: read-var __HDCR_TPM:u8
        let s_83_0: bool = fn_state.u__HDCR_TPM;
        // D s_83_1: cast zx s_83_0 -> bv
        let s_83_1: Bits = Bits::new(s_83_0 as u128, 1u16);
        // C s_83_2: const #1u : u8
        let s_83_2: bool = true;
        // C s_83_3: cast zx s_83_2 -> bv
        let s_83_3: Bits = Bits::new(s_83_2 as u128, 1u16);
        // D s_83_4: cmp-eq s_83_1 s_83_3
        let s_83_4: bool = ((s_83_1) == (s_83_3));
        // D s_83_5: write-var gs#133806 <= s_83_4
        fn_state.gs_133806 = s_83_4;
        // N s_83_6: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_84_0: const #432u : u32
        let s_84_0: u32 = 432;
        // D s_84_1: read-reg s_84_0:u8
        let s_84_1: u8 = {
            let value = state.read_register::<u8>(s_84_0 as isize);
            tracer.read_register(s_84_0 as isize, value);
            value
        };
        // D s_84_2: call ELUsingAArch32(s_84_1)
        let s_84_2: bool = ELUsingAArch32(state, tracer, s_84_1);
        // D s_84_3: write-var gs#133805 <= s_84_2
        fn_state.gs_133805 = s_84_2;
        // N s_84_4: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_85_0: const #3u : u8
        let s_85_0: u8 = 3;
        // C s_85_1: cast zx s_85_0 -> bv
        let s_85_1: Bits = Bits::new(s_85_0 as u128, 8u16);
        // C s_85_2: cast zx s_85_1 -> i
        let s_85_2: i128 = (s_85_1.value() as i128);
        // C s_85_3: cast reint s_85_2 -> i64
        let s_85_3: i64 = (s_85_2 as i64);
        // C s_85_4: cast zx s_85_3 -> i
        let s_85_4: i128 = (i128::try_from(s_85_3).unwrap());
        // C s_85_5: const #432u : u32
        let s_85_5: u32 = 432;
        // D s_85_6: read-reg s_85_5:u8
        let s_85_6: u8 = {
            let value = state.read_register::<u8>(s_85_5 as isize);
            tracer.read_register(s_85_5 as isize, value);
            value
        };
        // D s_85_7: call AArch64_AArch32SystemAccessTrap(s_85_6, s_85_4)
        let s_85_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_85_6, s_85_4);
        // N s_85_8: return
        return;
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_86_0: read-var __MDCR_EL2_TPM:u8
        let s_86_0: bool = fn_state.u__MDCR_EL2_TPM;
        // D s_86_1: cast zx s_86_0 -> bv
        let s_86_1: Bits = Bits::new(s_86_0 as u128, 1u16);
        // C s_86_2: const #1u : u8
        let s_86_2: bool = true;
        // C s_86_3: cast zx s_86_2 -> bv
        let s_86_3: Bits = Bits::new(s_86_2 as u128, 1u16);
        // D s_86_4: cmp-eq s_86_1 s_86_3
        let s_86_4: bool = ((s_86_1) == (s_86_3));
        // D s_86_5: write-var gs#133804 <= s_86_4
        fn_state.gs_133804 = s_86_4;
        // N s_86_6: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_87_0: const #432u : u32
        let s_87_0: u32 = 432;
        // D s_87_1: read-reg s_87_0:u8
        let s_87_1: u8 = {
            let value = state.read_register::<u8>(s_87_0 as isize);
            tracer.read_register(s_87_0 as isize, value);
            value
        };
        // D s_87_2: call ELUsingAArch32(s_87_1)
        let s_87_2: bool = ELUsingAArch32(state, tracer, s_87_1);
        // D s_87_3: not s_87_2
        let s_87_3: bool = !s_87_2;
        // D s_87_4: write-var gs#133803 <= s_87_3
        fn_state.gs_133803 = s_87_3;
        // N s_87_5: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_88_0: const #3u : u8
        let s_88_0: u8 = 3;
        // C s_88_1: cast zx s_88_0 -> bv
        let s_88_1: Bits = Bits::new(s_88_0 as u128, 8u16);
        // C s_88_2: cast zx s_88_1 -> i
        let s_88_2: i128 = (s_88_1.value() as i128);
        // C s_88_3: cast reint s_88_2 -> i64
        let s_88_3: i64 = (s_88_2 as i64);
        // C s_88_4: cast zx s_88_3 -> i
        let s_88_4: i128 = (i128::try_from(s_88_3).unwrap());
        // S s_88_5: call AArch32_TakeHypTrapException(s_88_4)
        let s_88_5: () = AArch32_TakeHypTrapException(state, tracer, s_88_4);
        // N s_88_6: return
        return;
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_89_0: read-var __HSTR_T9:u8
        let s_89_0: bool = fn_state.u__HSTR_T9;
        // D s_89_1: cast zx s_89_0 -> bv
        let s_89_1: Bits = Bits::new(s_89_0 as u128, 1u16);
        // C s_89_2: const #1u : u8
        let s_89_2: bool = true;
        // C s_89_3: cast zx s_89_2 -> bv
        let s_89_3: Bits = Bits::new(s_89_2 as u128, 1u16);
        // D s_89_4: cmp-eq s_89_1 s_89_3
        let s_89_4: bool = ((s_89_1) == (s_89_3));
        // D s_89_5: write-var gs#133802 <= s_89_4
        fn_state.gs_133802 = s_89_4;
        // N s_89_6: jump b57
        return block_57(state, tracer, fn_state);
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
        // D s_90_3: write-var gs#133801 <= s_90_2
        fn_state.gs_133801 = s_90_2;
        // N s_90_4: jump b55
        return block_55(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_91_0: const #3u : u8
        let s_91_0: u8 = 3;
        // C s_91_1: cast zx s_91_0 -> bv
        let s_91_1: Bits = Bits::new(s_91_0 as u128, 8u16);
        // C s_91_2: cast zx s_91_1 -> i
        let s_91_2: i128 = (s_91_1.value() as i128);
        // C s_91_3: cast reint s_91_2 -> i64
        let s_91_3: i64 = (s_91_2 as i64);
        // C s_91_4: cast zx s_91_3 -> i
        let s_91_4: i128 = (i128::try_from(s_91_3).unwrap());
        // C s_91_5: const #432u : u32
        let s_91_5: u32 = 432;
        // D s_91_6: read-reg s_91_5:u8
        let s_91_6: u8 = {
            let value = state.read_register::<u8>(s_91_5 as isize);
            tracer.read_register(s_91_5 as isize, value);
            value
        };
        // D s_91_7: call AArch64_AArch32SystemAccessTrap(s_91_6, s_91_4)
        let s_91_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_91_6, s_91_4);
        // N s_91_8: return
        return;
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_92_0: read-var __HSTR_EL2_T9:u8
        let s_92_0: bool = fn_state.u__HSTR_EL2_T9;
        // D s_92_1: cast zx s_92_0 -> bv
        let s_92_1: Bits = Bits::new(s_92_0 as u128, 1u16);
        // C s_92_2: const #1u : u8
        let s_92_2: bool = true;
        // C s_92_3: cast zx s_92_2 -> bv
        let s_92_3: Bits = Bits::new(s_92_2 as u128, 1u16);
        // D s_92_4: cmp-eq s_92_1 s_92_3
        let s_92_4: bool = ((s_92_1) == (s_92_3));
        // D s_92_5: write-var gs#133800 <= s_92_4
        fn_state.gs_133800 = s_92_4;
        // N s_92_6: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_93_0: const #432u : u32
        let s_93_0: u32 = 432;
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
        // D s_93_4: write-var gs#133799 <= s_93_3
        fn_state.gs_133799 = s_93_3;
        // N s_93_5: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_94_0: panic
        panic!("{:?}", ());
        // N s_94_1: return
        return;
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_95_0: read-var __MDCR_EL3_TPM:u8
        let s_95_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_95_1: cast zx s_95_0 -> bv
        let s_95_1: Bits = Bits::new(s_95_0 as u128, 1u16);
        // C s_95_2: const #1u : u8
        let s_95_2: bool = true;
        // C s_95_3: cast zx s_95_2 -> bv
        let s_95_3: Bits = Bits::new(s_95_2 as u128, 1u16);
        // D s_95_4: cmp-eq s_95_1 s_95_3
        let s_95_4: bool = ((s_95_1) == (s_95_3));
        // D s_95_5: write-var gs#133798 <= s_95_4
        fn_state.gs_133798 = s_95_4;
        // N s_95_6: jump b47
        return block_47(state, tracer, fn_state);
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
        // D s_96_2: call ELUsingAArch32(s_96_1)
        let s_96_2: bool = ELUsingAArch32(state, tracer, s_96_1);
        // D s_96_3: not s_96_2
        let s_96_3: bool = !s_96_2;
        // D s_96_4: write-var gs#133797 <= s_96_3
        fn_state.gs_133797 = s_96_3;
        // N s_96_5: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_97_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_97_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_97_1: call __IMPDEF_boolean(s_97_0)
        let s_97_1: bool = u__IMPDEF_boolean(state, tracer, s_97_0);
        // D s_97_2: write-var gs#133796 <= s_97_1
        fn_state.gs_133796 = s_97_1;
        // N s_97_3: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_98_0: const #() : ()
        let s_98_0: () = ();
        // S s_98_1: call EDSCR_read(s_98_0)
        let s_98_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_98_0);
        // S s_98_2: call _get_EDSCR_Type_SDD(s_98_1)
        let s_98_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_98_1);
        // S s_98_3: cast zx s_98_2 -> bv
        let s_98_3: Bits = Bits::new(s_98_2 as u128, 1u16);
        // C s_98_4: const #1u : u8
        let s_98_4: bool = true;
        // C s_98_5: cast zx s_98_4 -> bv
        let s_98_5: Bits = Bits::new(s_98_4 as u128, 1u16);
        // S s_98_6: cmp-eq s_98_3 s_98_5
        let s_98_6: bool = ((s_98_3) == (s_98_5));
        // D s_98_7: write-var gs#133795 <= s_98_6
        fn_state.gs_133795 = s_98_6;
        // N s_98_8: jump b41
        return block_41(state, tracer, fn_state);
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
        // C s_99_2: const #2u : u8
        let s_99_2: u8 = 2;
        // D s_99_3: cmp-lt s_99_1 s_99_2
        let s_99_3: bool = ((s_99_1) < (s_99_2));
        // D s_99_4: write-var gs#133794 <= s_99_3
        fn_state.gs_133794 = s_99_3;
        // N s_99_5: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_100_0: const #() : ()
        let s_100_0: () = ();
        // S s_100_1: call Halted(s_100_0)
        let s_100_1: bool = Halted(state, tracer, s_100_0);
        // N s_100_2: branch s_100_1 b228 b101
        if s_100_1 {
            return block_228(state, tracer, fn_state);
        } else {
            return block_101(state, tracer, fn_state);
        };
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_101_0: const #0u : u8
        let s_101_0: bool = false;
        // D s_101_1: write-var gs#133810 <= s_101_0
        fn_state.gs_133810 = s_101_0;
        // N s_101_2: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_102_0: read-var gs#133810:u8
        let s_102_0: bool = fn_state.gs_133810;
        // N s_102_1: branch s_102_0 b227 b103
        if s_102_0 {
            return block_227(state, tracer, fn_state);
        } else {
            return block_103(state, tracer, fn_state);
        };
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_103_0: const #0u : u8
        let s_103_0: bool = false;
        // D s_103_1: write-var gs#133811 <= s_103_0
        fn_state.gs_133811 = s_103_0;
        // N s_103_2: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_104_0: read-var gs#133811:u8
        let s_104_0: bool = fn_state.gs_133811;
        // N s_104_1: branch s_104_0 b226 b105
        if s_104_0 {
            return block_226(state, tracer, fn_state);
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
        // D s_105_1: write-var gs#133812 <= s_105_0
        fn_state.gs_133812 = s_105_0;
        // N s_105_2: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_106_0: read-var gs#133812:u8
        let s_106_0: bool = fn_state.gs_133812;
        // N s_106_1: branch s_106_0 b225 b107
        if s_106_0 {
            return block_225(state, tracer, fn_state);
        } else {
            return block_107(state, tracer, fn_state);
        };
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_107_0: const #0u : u8
        let s_107_0: bool = false;
        // D s_107_1: write-var gs#133813 <= s_107_0
        fn_state.gs_133813 = s_107_0;
        // N s_107_2: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_108_0: read-var gs#133813:u8
        let s_108_0: bool = fn_state.gs_133813;
        // N s_108_1: branch s_108_0 b224 b109
        if s_108_0 {
            return block_224(state, tracer, fn_state);
        } else {
            return block_109(state, tracer, fn_state);
        };
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_109_0: const #0u : u8
        let s_109_0: bool = false;
        // D s_109_1: write-var gs#133814 <= s_109_0
        fn_state.gs_133814 = s_109_0;
        // N s_109_2: jump b110
        return block_110(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_110_0: read-var gs#133814:u8
        let s_110_0: bool = fn_state.gs_133814;
        // N s_110_1: branch s_110_0 b223 b111
        if s_110_0 {
            return block_223(state, tracer, fn_state);
        } else {
            return block_111(state, tracer, fn_state);
        };
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_111_0: const #440u : u32
        let s_111_0: u32 = 440;
        // D s_111_1: read-reg s_111_0:u8
        let s_111_1: u8 = {
            let value = state.read_register::<u8>(s_111_0 as isize);
            tracer.read_register(s_111_0 as isize, value);
            value
        };
        // D s_111_2: call ELUsingAArch32(s_111_1)
        let s_111_2: bool = ELUsingAArch32(state, tracer, s_111_1);
        // D s_111_3: not s_111_2
        let s_111_3: bool = !s_111_2;
        // N s_111_4: branch s_111_3 b213 b112
        if s_111_3 {
            return block_213(state, tracer, fn_state);
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
        // D s_112_1: write-var gs#133818 <= s_112_0
        fn_state.gs_133818 = s_112_0;
        // N s_112_2: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_113_0: read-var gs#133818:u8
        let s_113_0: bool = fn_state.gs_133818;
        // N s_113_1: branch s_113_0 b204 b114
        if s_113_0 {
            return block_204(state, tracer, fn_state);
        } else {
            return block_114(state, tracer, fn_state);
        };
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_114_0: const #440u : u32
        let s_114_0: u32 = 440;
        // D s_114_1: read-reg s_114_0:u8
        let s_114_1: u8 = {
            let value = state.read_register::<u8>(s_114_0 as isize);
            tracer.read_register(s_114_0 as isize, value);
            value
        };
        // D s_114_2: call ELUsingAArch32(s_114_1)
        let s_114_2: bool = ELUsingAArch32(state, tracer, s_114_1);
        // N s_114_3: branch s_114_2 b203 b115
        if s_114_2 {
            return block_203(state, tracer, fn_state);
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
        // D s_115_1: write-var gs#133819 <= s_115_0
        fn_state.gs_133819 = s_115_0;
        // N s_115_2: jump b116
        return block_116(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_116_0: read-var gs#133819:u8
        let s_116_0: bool = fn_state.gs_133819;
        // N s_116_1: branch s_116_0 b186 b117
        if s_116_0 {
            return block_186(state, tracer, fn_state);
        } else {
            return block_117(state, tracer, fn_state);
        };
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_117_0: const #() : ()
        let s_117_0: () = ();
        // S s_117_1: call EL2Enabled(s_117_0)
        let s_117_1: bool = EL2Enabled(state, tracer, s_117_0);
        // N s_117_2: branch s_117_1 b185 b118
        if s_117_1 {
            return block_185(state, tracer, fn_state);
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
        // D s_118_1: write-var gs#133820 <= s_118_0
        fn_state.gs_133820 = s_118_0;
        // N s_118_2: jump b119
        return block_119(state, tracer, fn_state);
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_119_0: read-var gs#133820:u8
        let s_119_0: bool = fn_state.gs_133820;
        // N s_119_1: branch s_119_0 b184 b120
        if s_119_0 {
            return block_184(state, tracer, fn_state);
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
        // D s_120_1: write-var gs#133821 <= s_120_0
        fn_state.gs_133821 = s_120_0;
        // N s_120_2: jump b121
        return block_121(state, tracer, fn_state);
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_121_0: read-var gs#133821:u8
        let s_121_0: bool = fn_state.gs_133821;
        // N s_121_1: branch s_121_0 b183 b122
        if s_121_0 {
            return block_183(state, tracer, fn_state);
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
        // D s_122_1: write-var gs#133822 <= s_122_0
        fn_state.gs_133822 = s_122_0;
        // N s_122_2: jump b123
        return block_123(state, tracer, fn_state);
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_123_0: read-var gs#133822:u8
        let s_123_0: bool = fn_state.gs_133822;
        // N s_123_1: branch s_123_0 b182 b124
        if s_123_0 {
            return block_182(state, tracer, fn_state);
        } else {
            return block_124(state, tracer, fn_state);
        };
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_124_0: const #() : ()
        let s_124_0: () = ();
        // S s_124_1: call EL2Enabled(s_124_0)
        let s_124_1: bool = EL2Enabled(state, tracer, s_124_0);
        // N s_124_2: branch s_124_1 b181 b125
        if s_124_1 {
            return block_181(state, tracer, fn_state);
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
        // D s_125_1: write-var gs#133823 <= s_125_0
        fn_state.gs_133823 = s_125_0;
        // N s_125_2: jump b126
        return block_126(state, tracer, fn_state);
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_126_0: read-var gs#133823:u8
        let s_126_0: bool = fn_state.gs_133823;
        // N s_126_1: branch s_126_0 b180 b127
        if s_126_0 {
            return block_180(state, tracer, fn_state);
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
        // D s_127_1: write-var gs#133824 <= s_127_0
        fn_state.gs_133824 = s_127_0;
        // N s_127_2: jump b128
        return block_128(state, tracer, fn_state);
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_128_0: read-var gs#133824:u8
        let s_128_0: bool = fn_state.gs_133824;
        // N s_128_1: branch s_128_0 b179 b129
        if s_128_0 {
            return block_179(state, tracer, fn_state);
        } else {
            return block_129(state, tracer, fn_state);
        };
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_129_0: const #() : ()
        let s_129_0: () = ();
        // S s_129_1: call EL2Enabled(s_129_0)
        let s_129_1: bool = EL2Enabled(state, tracer, s_129_0);
        // N s_129_2: branch s_129_1 b178 b130
        if s_129_1 {
            return block_178(state, tracer, fn_state);
        } else {
            return block_130(state, tracer, fn_state);
        };
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_130_0: const #0u : u8
        let s_130_0: bool = false;
        // D s_130_1: write-var gs#133825 <= s_130_0
        fn_state.gs_133825 = s_130_0;
        // N s_130_2: jump b131
        return block_131(state, tracer, fn_state);
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_131_0: read-var gs#133825:u8
        let s_131_0: bool = fn_state.gs_133825;
        // N s_131_1: branch s_131_0 b177 b132
        if s_131_0 {
            return block_177(state, tracer, fn_state);
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
        // D s_132_1: write-var gs#133826 <= s_132_0
        fn_state.gs_133826 = s_132_0;
        // N s_132_2: jump b133
        return block_133(state, tracer, fn_state);
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_133_0: read-var gs#133826:u8
        let s_133_0: bool = fn_state.gs_133826;
        // N s_133_1: branch s_133_0 b176 b134
        if s_133_0 {
            return block_176(state, tracer, fn_state);
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
        // D s_134_1: write-var gs#133827 <= s_134_0
        fn_state.gs_133827 = s_134_0;
        // N s_134_2: jump b135
        return block_135(state, tracer, fn_state);
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_135_0: read-var gs#133827:u8
        let s_135_0: bool = fn_state.gs_133827;
        // N s_135_1: branch s_135_0 b172 b136
        if s_135_0 {
            return block_172(state, tracer, fn_state);
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
        // D s_136_1: write-var gs#133829 <= s_136_0
        fn_state.gs_133829 = s_136_0;
        // N s_136_2: jump b137
        return block_137(state, tracer, fn_state);
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_137_0: read-var gs#133829:u8
        let s_137_0: bool = fn_state.gs_133829;
        // N s_137_1: branch s_137_0 b171 b138
        if s_137_0 {
            return block_171(state, tracer, fn_state);
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
        // D s_138_1: write-var gs#133830 <= s_138_0
        fn_state.gs_133830 = s_138_0;
        // N s_138_2: jump b139
        return block_139(state, tracer, fn_state);
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_139_0: read-var gs#133830:u8
        let s_139_0: bool = fn_state.gs_133830;
        // N s_139_1: branch s_139_0 b170 b140
        if s_139_0 {
            return block_170(state, tracer, fn_state);
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
        // N s_140_2: branch s_140_1 b169 b141
        if s_140_1 {
            return block_169(state, tracer, fn_state);
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
        // D s_141_1: write-var gs#133831 <= s_141_0
        fn_state.gs_133831 = s_141_0;
        // N s_141_2: jump b142
        return block_142(state, tracer, fn_state);
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_142_0: read-var gs#133831:u8
        let s_142_0: bool = fn_state.gs_133831;
        // N s_142_1: branch s_142_0 b168 b143
        if s_142_0 {
            return block_168(state, tracer, fn_state);
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
        // D s_143_1: write-var gs#133832 <= s_143_0
        fn_state.gs_133832 = s_143_0;
        // N s_143_2: jump b144
        return block_144(state, tracer, fn_state);
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_144_0: read-var gs#133832:u8
        let s_144_0: bool = fn_state.gs_133832;
        // N s_144_1: branch s_144_0 b167 b145
        if s_144_0 {
            return block_167(state, tracer, fn_state);
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
        // N s_145_2: branch s_145_1 b166 b146
        if s_145_1 {
            return block_166(state, tracer, fn_state);
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
        // D s_146_1: write-var gs#133833 <= s_146_0
        fn_state.gs_133833 = s_146_0;
        // N s_146_2: jump b147
        return block_147(state, tracer, fn_state);
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_147_0: read-var gs#133833:u8
        let s_147_0: bool = fn_state.gs_133833;
        // N s_147_1: branch s_147_0 b165 b148
        if s_147_0 {
            return block_165(state, tracer, fn_state);
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
        // D s_148_1: write-var gs#133834 <= s_148_0
        fn_state.gs_133834 = s_148_0;
        // N s_148_2: jump b149
        return block_149(state, tracer, fn_state);
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_149_0: read-var gs#133834:u8
        let s_149_0: bool = fn_state.gs_133834;
        // N s_149_1: branch s_149_0 b164 b150
        if s_149_0 {
            return block_164(state, tracer, fn_state);
        } else {
            return block_150(state, tracer, fn_state);
        };
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_150_0: const #424u : u32
        let s_150_0: u32 = 424;
        // D s_150_1: read-reg s_150_0:u8
        let s_150_1: u8 = {
            let value = state.read_register::<u8>(s_150_0 as isize);
            tracer.read_register(s_150_0 as isize, value);
            value
        };
        // C s_150_2: const #2u : u8
        let s_150_2: u8 = 2;
        // D s_150_3: cmp-lt s_150_1 s_150_2
        let s_150_3: bool = ((s_150_1) < (s_150_2));
        // N s_150_4: branch s_150_3 b163 b151
        if s_150_3 {
            return block_163(state, tracer, fn_state);
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
        // D s_151_1: write-var gs#133835 <= s_151_0
        fn_state.gs_133835 = s_151_0;
        // N s_151_2: jump b152
        return block_152(state, tracer, fn_state);
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_152_0: read-var gs#133835:u8
        let s_152_0: bool = fn_state.gs_133835;
        // N s_152_1: branch s_152_0 b162 b153
        if s_152_0 {
            return block_162(state, tracer, fn_state);
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
        // D s_153_1: write-var gs#133836 <= s_153_0
        fn_state.gs_133836 = s_153_0;
        // N s_153_2: jump b154
        return block_154(state, tracer, fn_state);
    }
    fn block_154<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_154_0: read-var gs#133836:u8
        let s_154_0: bool = fn_state.gs_133836;
        // N s_154_1: branch s_154_0 b156 b155
        if s_154_0 {
            return block_156(state, tracer, fn_state);
        } else {
            return block_155(state, tracer, fn_state);
        };
    }
    fn block_155<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_155_0: read-var t:i
        let s_155_0: i128 = fn_state.t;
        // D s_155_1: call R_read(s_155_0)
        let s_155_1: u32 = R_read(state, tracer, s_155_0);
        // D s_155_2: call Mk_PMSWINC_Type(s_155_1)
        let s_155_2: ProductType700c18a878c5601b = Mk_PMSWINC_Type(
            state,
            tracer,
            s_155_1,
        );
        // D s_155_3: call PMSWINC_write(s_155_2)
        let s_155_3: () = PMSWINC_write(state, tracer, s_155_2);
        // N s_155_4: return
        return;
    }
    fn block_156<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_156_0: const #() : ()
        let s_156_0: () = ();
        // S s_156_1: call Halted(s_156_0)
        let s_156_1: bool = Halted(state, tracer, s_156_0);
        // N s_156_2: branch s_156_1 b161 b157
        if s_156_1 {
            return block_161(state, tracer, fn_state);
        } else {
            return block_157(state, tracer, fn_state);
        };
    }
    fn block_157<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_157_0: const #0u : u8
        let s_157_0: bool = false;
        // D s_157_1: write-var gs#133837 <= s_157_0
        fn_state.gs_133837 = s_157_0;
        // N s_157_2: jump b158
        return block_158(state, tracer, fn_state);
    }
    fn block_158<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_158_0: read-var gs#133837:u8
        let s_158_0: bool = fn_state.gs_133837;
        // N s_158_1: branch s_158_0 b160 b159
        if s_158_0 {
            return block_160(state, tracer, fn_state);
        } else {
            return block_159(state, tracer, fn_state);
        };
    }
    fn block_159<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_159_0: const #3u : u8
        let s_159_0: u8 = 3;
        // C s_159_1: cast zx s_159_0 -> bv
        let s_159_1: Bits = Bits::new(s_159_0 as u128, 8u16);
        // C s_159_2: cast zx s_159_1 -> i
        let s_159_2: i128 = (s_159_1.value() as i128);
        // C s_159_3: cast reint s_159_2 -> i64
        let s_159_3: i64 = (s_159_2 as i64);
        // C s_159_4: cast zx s_159_3 -> i
        let s_159_4: i128 = (i128::try_from(s_159_3).unwrap());
        // C s_159_5: const #424u : u32
        let s_159_5: u32 = 424;
        // D s_159_6: read-reg s_159_5:u8
        let s_159_6: u8 = {
            let value = state.read_register::<u8>(s_159_5 as isize);
            tracer.read_register(s_159_5 as isize, value);
            value
        };
        // D s_159_7: call AArch64_AArch32SystemAccessTrap(s_159_6, s_159_4)
        let s_159_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_159_6,
            s_159_4,
        );
        // N s_159_8: return
        return;
    }
    fn block_160<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_160_0: panic
        panic!("{:?}", ());
        // N s_160_1: return
        return;
    }
    fn block_161<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_161_0: const #() : ()
        let s_161_0: () = ();
        // S s_161_1: call EDSCR_read(s_161_0)
        let s_161_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_161_0);
        // S s_161_2: call _get_EDSCR_Type_SDD(s_161_1)
        let s_161_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_161_1);
        // S s_161_3: cast zx s_161_2 -> bv
        let s_161_3: Bits = Bits::new(s_161_2 as u128, 1u16);
        // C s_161_4: const #1u : u8
        let s_161_4: bool = true;
        // C s_161_5: cast zx s_161_4 -> bv
        let s_161_5: Bits = Bits::new(s_161_4 as u128, 1u16);
        // S s_161_6: cmp-eq s_161_3 s_161_5
        let s_161_6: bool = ((s_161_3) == (s_161_5));
        // D s_161_7: write-var gs#133837 <= s_161_6
        fn_state.gs_133837 = s_161_6;
        // N s_161_8: jump b158
        return block_158(state, tracer, fn_state);
    }
    fn block_162<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_162_0: read-var __MDCR_EL3_TPM:u8
        let s_162_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_162_1: cast zx s_162_0 -> bv
        let s_162_1: Bits = Bits::new(s_162_0 as u128, 1u16);
        // C s_162_2: const #1u : u8
        let s_162_2: bool = true;
        // C s_162_3: cast zx s_162_2 -> bv
        let s_162_3: Bits = Bits::new(s_162_2 as u128, 1u16);
        // D s_162_4: cmp-eq s_162_1 s_162_3
        let s_162_4: bool = ((s_162_1) == (s_162_3));
        // D s_162_5: write-var gs#133836 <= s_162_4
        fn_state.gs_133836 = s_162_4;
        // N s_162_6: jump b154
        return block_154(state, tracer, fn_state);
    }
    fn block_163<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_163_0: const #424u : u32
        let s_163_0: u32 = 424;
        // D s_163_1: read-reg s_163_0:u8
        let s_163_1: u8 = {
            let value = state.read_register::<u8>(s_163_0 as isize);
            tracer.read_register(s_163_0 as isize, value);
            value
        };
        // D s_163_2: call ELUsingAArch32(s_163_1)
        let s_163_2: bool = ELUsingAArch32(state, tracer, s_163_1);
        // D s_163_3: not s_163_2
        let s_163_3: bool = !s_163_2;
        // D s_163_4: write-var gs#133835 <= s_163_3
        fn_state.gs_133835 = s_163_3;
        // N s_163_5: jump b152
        return block_152(state, tracer, fn_state);
    }
    fn block_164<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_164_0: const #3u : u8
        let s_164_0: u8 = 3;
        // C s_164_1: cast zx s_164_0 -> bv
        let s_164_1: Bits = Bits::new(s_164_0 as u128, 8u16);
        // C s_164_2: cast zx s_164_1 -> i
        let s_164_2: i128 = (s_164_1.value() as i128);
        // C s_164_3: cast reint s_164_2 -> i64
        let s_164_3: i64 = (s_164_2 as i64);
        // C s_164_4: cast zx s_164_3 -> i
        let s_164_4: i128 = (i128::try_from(s_164_3).unwrap());
        // S s_164_5: call AArch32_TakeHypTrapException(s_164_4)
        let s_164_5: () = AArch32_TakeHypTrapException(state, tracer, s_164_4);
        // N s_164_6: return
        return;
    }
    fn block_165<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_165_0: read-var __HDCR_TPM:u8
        let s_165_0: bool = fn_state.u__HDCR_TPM;
        // D s_165_1: cast zx s_165_0 -> bv
        let s_165_1: Bits = Bits::new(s_165_0 as u128, 1u16);
        // C s_165_2: const #1u : u8
        let s_165_2: bool = true;
        // C s_165_3: cast zx s_165_2 -> bv
        let s_165_3: Bits = Bits::new(s_165_2 as u128, 1u16);
        // D s_165_4: cmp-eq s_165_1 s_165_3
        let s_165_4: bool = ((s_165_1) == (s_165_3));
        // D s_165_5: write-var gs#133834 <= s_165_4
        fn_state.gs_133834 = s_165_4;
        // N s_165_6: jump b149
        return block_149(state, tracer, fn_state);
    }
    fn block_166<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_166_0: const #432u : u32
        let s_166_0: u32 = 432;
        // D s_166_1: read-reg s_166_0:u8
        let s_166_1: u8 = {
            let value = state.read_register::<u8>(s_166_0 as isize);
            tracer.read_register(s_166_0 as isize, value);
            value
        };
        // D s_166_2: call ELUsingAArch32(s_166_1)
        let s_166_2: bool = ELUsingAArch32(state, tracer, s_166_1);
        // D s_166_3: write-var gs#133833 <= s_166_2
        fn_state.gs_133833 = s_166_2;
        // N s_166_4: jump b147
        return block_147(state, tracer, fn_state);
    }
    fn block_167<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_167_0: const #3u : u8
        let s_167_0: u8 = 3;
        // C s_167_1: cast zx s_167_0 -> bv
        let s_167_1: Bits = Bits::new(s_167_0 as u128, 8u16);
        // C s_167_2: cast zx s_167_1 -> i
        let s_167_2: i128 = (s_167_1.value() as i128);
        // C s_167_3: cast reint s_167_2 -> i64
        let s_167_3: i64 = (s_167_2 as i64);
        // C s_167_4: cast zx s_167_3 -> i
        let s_167_4: i128 = (i128::try_from(s_167_3).unwrap());
        // C s_167_5: const #432u : u32
        let s_167_5: u32 = 432;
        // D s_167_6: read-reg s_167_5:u8
        let s_167_6: u8 = {
            let value = state.read_register::<u8>(s_167_5 as isize);
            tracer.read_register(s_167_5 as isize, value);
            value
        };
        // D s_167_7: call AArch64_AArch32SystemAccessTrap(s_167_6, s_167_4)
        let s_167_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_167_6,
            s_167_4,
        );
        // N s_167_8: return
        return;
    }
    fn block_168<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_168_0: read-var __MDCR_EL2_TPM:u8
        let s_168_0: bool = fn_state.u__MDCR_EL2_TPM;
        // D s_168_1: cast zx s_168_0 -> bv
        let s_168_1: Bits = Bits::new(s_168_0 as u128, 1u16);
        // C s_168_2: const #1u : u8
        let s_168_2: bool = true;
        // C s_168_3: cast zx s_168_2 -> bv
        let s_168_3: Bits = Bits::new(s_168_2 as u128, 1u16);
        // D s_168_4: cmp-eq s_168_1 s_168_3
        let s_168_4: bool = ((s_168_1) == (s_168_3));
        // D s_168_5: write-var gs#133832 <= s_168_4
        fn_state.gs_133832 = s_168_4;
        // N s_168_6: jump b144
        return block_144(state, tracer, fn_state);
    }
    fn block_169<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_169_0: const #432u : u32
        let s_169_0: u32 = 432;
        // D s_169_1: read-reg s_169_0:u8
        let s_169_1: u8 = {
            let value = state.read_register::<u8>(s_169_0 as isize);
            tracer.read_register(s_169_0 as isize, value);
            value
        };
        // D s_169_2: call ELUsingAArch32(s_169_1)
        let s_169_2: bool = ELUsingAArch32(state, tracer, s_169_1);
        // D s_169_3: not s_169_2
        let s_169_3: bool = !s_169_2;
        // D s_169_4: write-var gs#133831 <= s_169_3
        fn_state.gs_133831 = s_169_3;
        // N s_169_5: jump b142
        return block_142(state, tracer, fn_state);
    }
    fn block_170<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_170_0: const #3u : u8
        let s_170_0: u8 = 3;
        // C s_170_1: cast zx s_170_0 -> bv
        let s_170_1: Bits = Bits::new(s_170_0 as u128, 8u16);
        // C s_170_2: cast zx s_170_1 -> i
        let s_170_2: i128 = (s_170_1.value() as i128);
        // C s_170_3: cast reint s_170_2 -> i64
        let s_170_3: i64 = (s_170_2 as i64);
        // C s_170_4: cast zx s_170_3 -> i
        let s_170_4: i128 = (i128::try_from(s_170_3).unwrap());
        // C s_170_5: const #432u : u32
        let s_170_5: u32 = 432;
        // D s_170_6: read-reg s_170_5:u8
        let s_170_6: u8 = {
            let value = state.read_register::<u8>(s_170_5 as isize);
            tracer.read_register(s_170_5 as isize, value);
            value
        };
        // D s_170_7: call AArch64_AArch32SystemAccessTrap(s_170_6, s_170_4)
        let s_170_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_170_6,
            s_170_4,
        );
        // N s_170_8: return
        return;
    }
    fn block_171<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_171_0: read-var __HDFGWTR_EL2_PMSWINC_EL0:u8
        let s_171_0: bool = fn_state.u__HDFGWTR_EL2_PMSWINC_EL0;
        // D s_171_1: cast zx s_171_0 -> bv
        let s_171_1: Bits = Bits::new(s_171_0 as u128, 1u16);
        // C s_171_2: const #1u : u8
        let s_171_2: bool = true;
        // C s_171_3: cast zx s_171_2 -> bv
        let s_171_3: Bits = Bits::new(s_171_2 as u128, 1u16);
        // D s_171_4: cmp-eq s_171_1 s_171_3
        let s_171_4: bool = ((s_171_1) == (s_171_3));
        // D s_171_5: write-var gs#133830 <= s_171_4
        fn_state.gs_133830 = s_171_4;
        // N s_171_6: jump b139
        return block_139(state, tracer, fn_state);
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
        // D s_172_4: not s_172_3
        let s_172_4: bool = !s_172_3;
        // N s_172_5: branch s_172_4 b175 b173
        if s_172_4 {
            return block_175(state, tracer, fn_state);
        } else {
            return block_173(state, tracer, fn_state);
        };
    }
    fn block_173<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_173_0: read-var __SCR_EL3_FGTEn:u8
        let s_173_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_173_1: cast zx s_173_0 -> bv
        let s_173_1: Bits = Bits::new(s_173_0 as u128, 1u16);
        // C s_173_2: const #1u : u8
        let s_173_2: bool = true;
        // C s_173_3: cast zx s_173_2 -> bv
        let s_173_3: Bits = Bits::new(s_173_2 as u128, 1u16);
        // D s_173_4: cmp-eq s_173_1 s_173_3
        let s_173_4: bool = ((s_173_1) == (s_173_3));
        // D s_173_5: write-var gs#133828 <= s_173_4
        fn_state.gs_133828 = s_173_4;
        // N s_173_6: jump b174
        return block_174(state, tracer, fn_state);
    }
    fn block_174<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_174_0: read-var gs#133828:u8
        let s_174_0: bool = fn_state.gs_133828;
        // D s_174_1: write-var gs#133829 <= s_174_0
        fn_state.gs_133829 = s_174_0;
        // N s_174_2: jump b137
        return block_137(state, tracer, fn_state);
    }
    fn block_175<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_175_0: const #1u : u8
        let s_175_0: bool = true;
        // D s_175_1: write-var gs#133828 <= s_175_0
        fn_state.gs_133828 = s_175_0;
        // N s_175_2: jump b174
        return block_174(state, tracer, fn_state);
    }
    fn block_176<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_176_0: const #146u : u32
        let s_176_0: u32 = 146;
        // S s_176_1: call IsFeatureImplemented(s_176_0)
        let s_176_1: bool = IsFeatureImplemented(state, tracer, s_176_0);
        // D s_176_2: write-var gs#133827 <= s_176_1
        fn_state.gs_133827 = s_176_1;
        // N s_176_3: jump b135
        return block_135(state, tracer, fn_state);
    }
    fn block_177<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_177_0: const #102552u : u32
        let s_177_0: u32 = 102552;
        // D s_177_1: read-reg s_177_0:struct
        let s_177_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_177_0 as isize);
            tracer.read_register(s_177_0 as isize, value);
            value
        };
        // D s_177_2: call _get_HCR_EL2_Type_E2H(s_177_1)
        let s_177_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_177_1);
        // C s_177_3: const #102552u : u32
        let s_177_3: u32 = 102552;
        // D s_177_4: read-reg s_177_3:struct
        let s_177_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_177_3 as isize);
            tracer.read_register(s_177_3 as isize, value);
            value
        };
        // D s_177_5: call _get_HCR_EL2_Type_TGE(s_177_4)
        let s_177_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_177_4);
        // D s_177_6: cast zx s_177_2 -> bv
        let s_177_6: Bits = Bits::new(s_177_2 as u128, 1u16);
        // D s_177_7: cast zx s_177_5 -> bv
        let s_177_7: Bits = Bits::new(s_177_5 as u128, 1u16);
        // D s_177_8: cast reint s_177_6 -> u128
        let s_177_8: u128 = (s_177_6.value() as u128);
        // D s_177_9: size-of s_177_6
        let s_177_9: u16 = s_177_6.length();
        // D s_177_10: cast reint s_177_7 -> u128
        let s_177_10: u128 = (s_177_7.value() as u128);
        // D s_177_11: size-of s_177_7
        let s_177_11: u16 = s_177_7.length();
        // D s_177_12: lsl s_177_8 s_177_11
        let s_177_12: u128 = s_177_8 << s_177_11;
        // D s_177_13: or s_177_12 s_177_10
        let s_177_13: u128 = ((s_177_12) | (s_177_10));
        // D s_177_14: add s_177_9 s_177_11
        let s_177_14: u16 = (s_177_9 + s_177_11);
        // D s_177_15: create-bits s_177_13 s_177_14
        let s_177_15: Bits = Bits::new(s_177_13, s_177_14);
        // D s_177_16: cast reint s_177_15 -> u8
        let s_177_16: u8 = (s_177_15.value() as u8);
        // D s_177_17: cast zx s_177_16 -> bv
        let s_177_17: Bits = Bits::new(s_177_16 as u128, 2u16);
        // C s_177_18: const #3u : u8
        let s_177_18: u8 = 3;
        // C s_177_19: cast zx s_177_18 -> bv
        let s_177_19: Bits = Bits::new(s_177_18 as u128, 2u16);
        // D s_177_20: cmp-ne s_177_17 s_177_19
        let s_177_20: bool = ((s_177_17) != (s_177_19));
        // D s_177_21: write-var gs#133826 <= s_177_20
        fn_state.gs_133826 = s_177_20;
        // N s_177_22: jump b133
        return block_133(state, tracer, fn_state);
    }
    fn block_178<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_178_0: const #440u : u32
        let s_178_0: u32 = 440;
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
        // D s_178_4: write-var gs#133825 <= s_178_3
        fn_state.gs_133825 = s_178_3;
        // N s_178_5: jump b131
        return block_131(state, tracer, fn_state);
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
        // D s_180_0: read-var __HSTR_T9:u8
        let s_180_0: bool = fn_state.u__HSTR_T9;
        // D s_180_1: cast zx s_180_0 -> bv
        let s_180_1: Bits = Bits::new(s_180_0 as u128, 1u16);
        // C s_180_2: const #1u : u8
        let s_180_2: bool = true;
        // C s_180_3: cast zx s_180_2 -> bv
        let s_180_3: Bits = Bits::new(s_180_2 as u128, 1u16);
        // D s_180_4: cmp-eq s_180_1 s_180_3
        let s_180_4: bool = ((s_180_1) == (s_180_3));
        // D s_180_5: write-var gs#133824 <= s_180_4
        fn_state.gs_133824 = s_180_4;
        // N s_180_6: jump b128
        return block_128(state, tracer, fn_state);
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
        // D s_181_3: write-var gs#133823 <= s_181_2
        fn_state.gs_133823 = s_181_2;
        // N s_181_4: jump b126
        return block_126(state, tracer, fn_state);
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
        // C s_182_5: const #432u : u32
        let s_182_5: u32 = 432;
        // D s_182_6: read-reg s_182_5:u8
        let s_182_6: u8 = {
            let value = state.read_register::<u8>(s_182_5 as isize);
            tracer.read_register(s_182_5 as isize, value);
            value
        };
        // D s_182_7: call AArch64_AArch32SystemAccessTrap(s_182_6, s_182_4)
        let s_182_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_182_6,
            s_182_4,
        );
        // N s_182_8: return
        return;
    }
    fn block_183<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_183_0: read-var __HSTR_EL2_T9:u8
        let s_183_0: bool = fn_state.u__HSTR_EL2_T9;
        // D s_183_1: cast zx s_183_0 -> bv
        let s_183_1: Bits = Bits::new(s_183_0 as u128, 1u16);
        // C s_183_2: const #1u : u8
        let s_183_2: bool = true;
        // C s_183_3: cast zx s_183_2 -> bv
        let s_183_3: Bits = Bits::new(s_183_2 as u128, 1u16);
        // D s_183_4: cmp-eq s_183_1 s_183_3
        let s_183_4: bool = ((s_183_1) == (s_183_3));
        // D s_183_5: write-var gs#133822 <= s_183_4
        fn_state.gs_133822 = s_183_4;
        // N s_183_6: jump b123
        return block_123(state, tracer, fn_state);
    }
    fn block_184<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_184_0: const #102552u : u32
        let s_184_0: u32 = 102552;
        // D s_184_1: read-reg s_184_0:struct
        let s_184_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_184_0 as isize);
            tracer.read_register(s_184_0 as isize, value);
            value
        };
        // D s_184_2: call _get_HCR_EL2_Type_E2H(s_184_1)
        let s_184_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_184_1);
        // C s_184_3: const #102552u : u32
        let s_184_3: u32 = 102552;
        // D s_184_4: read-reg s_184_3:struct
        let s_184_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_184_3 as isize);
            tracer.read_register(s_184_3 as isize, value);
            value
        };
        // D s_184_5: call _get_HCR_EL2_Type_TGE(s_184_4)
        let s_184_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_184_4);
        // D s_184_6: cast zx s_184_2 -> bv
        let s_184_6: Bits = Bits::new(s_184_2 as u128, 1u16);
        // D s_184_7: cast zx s_184_5 -> bv
        let s_184_7: Bits = Bits::new(s_184_5 as u128, 1u16);
        // D s_184_8: cast reint s_184_6 -> u128
        let s_184_8: u128 = (s_184_6.value() as u128);
        // D s_184_9: size-of s_184_6
        let s_184_9: u16 = s_184_6.length();
        // D s_184_10: cast reint s_184_7 -> u128
        let s_184_10: u128 = (s_184_7.value() as u128);
        // D s_184_11: size-of s_184_7
        let s_184_11: u16 = s_184_7.length();
        // D s_184_12: lsl s_184_8 s_184_11
        let s_184_12: u128 = s_184_8 << s_184_11;
        // D s_184_13: or s_184_12 s_184_10
        let s_184_13: u128 = ((s_184_12) | (s_184_10));
        // D s_184_14: add s_184_9 s_184_11
        let s_184_14: u16 = (s_184_9 + s_184_11);
        // D s_184_15: create-bits s_184_13 s_184_14
        let s_184_15: Bits = Bits::new(s_184_13, s_184_14);
        // D s_184_16: cast reint s_184_15 -> u8
        let s_184_16: u8 = (s_184_15.value() as u8);
        // D s_184_17: cast zx s_184_16 -> bv
        let s_184_17: Bits = Bits::new(s_184_16 as u128, 2u16);
        // C s_184_18: const #3u : u8
        let s_184_18: u8 = 3;
        // C s_184_19: cast zx s_184_18 -> bv
        let s_184_19: Bits = Bits::new(s_184_18 as u128, 2u16);
        // D s_184_20: cmp-ne s_184_17 s_184_19
        let s_184_20: bool = ((s_184_17) != (s_184_19));
        // D s_184_21: write-var gs#133821 <= s_184_20
        fn_state.gs_133821 = s_184_20;
        // N s_184_22: jump b121
        return block_121(state, tracer, fn_state);
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
        // D s_185_4: write-var gs#133820 <= s_185_3
        fn_state.gs_133820 = s_185_3;
        // N s_185_5: jump b119
        return block_119(state, tracer, fn_state);
    }
    fn block_186<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_186_0: const #() : ()
        let s_186_0: () = ();
        // S s_186_1: call EL2Enabled(s_186_0)
        let s_186_1: bool = EL2Enabled(state, tracer, s_186_0);
        // N s_186_2: branch s_186_1 b202 b187
        if s_186_1 {
            return block_202(state, tracer, fn_state);
        } else {
            return block_187(state, tracer, fn_state);
        };
    }
    fn block_187<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_187_0: const #0u : u8
        let s_187_0: bool = false;
        // D s_187_1: write-var gs#133838 <= s_187_0
        fn_state.gs_133838 = s_187_0;
        // N s_187_2: jump b188
        return block_188(state, tracer, fn_state);
    }
    fn block_188<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_188_0: read-var gs#133838:u8
        let s_188_0: bool = fn_state.gs_133838;
        // N s_188_1: branch s_188_0 b201 b189
        if s_188_0 {
            return block_201(state, tracer, fn_state);
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
        // D s_189_1: write-var gs#133839 <= s_189_0
        fn_state.gs_133839 = s_189_0;
        // N s_189_2: jump b190
        return block_190(state, tracer, fn_state);
    }
    fn block_190<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_190_0: read-var gs#133839:u8
        let s_190_0: bool = fn_state.gs_133839;
        // N s_190_1: branch s_190_0 b200 b191
        if s_190_0 {
            return block_200(state, tracer, fn_state);
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
        // S s_191_1: call EL2Enabled(s_191_0)
        let s_191_1: bool = EL2Enabled(state, tracer, s_191_0);
        // N s_191_2: branch s_191_1 b199 b192
        if s_191_1 {
            return block_199(state, tracer, fn_state);
        } else {
            return block_192(state, tracer, fn_state);
        };
    }
    fn block_192<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_192_0: const #0u : u8
        let s_192_0: bool = false;
        // D s_192_1: write-var gs#133840 <= s_192_0
        fn_state.gs_133840 = s_192_0;
        // N s_192_2: jump b193
        return block_193(state, tracer, fn_state);
    }
    fn block_193<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_193_0: read-var gs#133840:u8
        let s_193_0: bool = fn_state.gs_133840;
        // N s_193_1: branch s_193_0 b198 b194
        if s_193_0 {
            return block_198(state, tracer, fn_state);
        } else {
            return block_194(state, tracer, fn_state);
        };
    }
    fn block_194<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_194_0: const #0u : u8
        let s_194_0: bool = false;
        // D s_194_1: write-var gs#133841 <= s_194_0
        fn_state.gs_133841 = s_194_0;
        // N s_194_2: jump b195
        return block_195(state, tracer, fn_state);
    }
    fn block_195<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_195_0: read-var gs#133841:u8
        let s_195_0: bool = fn_state.gs_133841;
        // N s_195_1: branch s_195_0 b197 b196
        if s_195_0 {
            return block_197(state, tracer, fn_state);
        } else {
            return block_196(state, tracer, fn_state);
        };
    }
    fn block_196<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_196_0: panic
        panic!("{:?}", ());
        // N s_196_1: return
        return;
    }
    fn block_197<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_197_0: const #0u : u8
        let s_197_0: u8 = 0;
        // C s_197_1: cast zx s_197_0 -> bv
        let s_197_1: Bits = Bits::new(s_197_0 as u128, 8u16);
        // C s_197_2: cast zx s_197_1 -> i
        let s_197_2: i128 = (s_197_1.value() as i128);
        // C s_197_3: cast reint s_197_2 -> i64
        let s_197_3: i64 = (s_197_2 as i64);
        // C s_197_4: cast zx s_197_3 -> i
        let s_197_4: i128 = (i128::try_from(s_197_3).unwrap());
        // S s_197_5: call AArch32_TakeHypTrapException(s_197_4)
        let s_197_5: () = AArch32_TakeHypTrapException(state, tracer, s_197_4);
        // N s_197_6: return
        return;
    }
    fn block_198<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_198_0: read-var __HCR_TGE:u8
        let s_198_0: bool = fn_state.u__HCR_TGE;
        // D s_198_1: cast zx s_198_0 -> bv
        let s_198_1: Bits = Bits::new(s_198_0 as u128, 1u16);
        // C s_198_2: const #1u : u8
        let s_198_2: bool = true;
        // C s_198_3: cast zx s_198_2 -> bv
        let s_198_3: Bits = Bits::new(s_198_2 as u128, 1u16);
        // D s_198_4: cmp-eq s_198_1 s_198_3
        let s_198_4: bool = ((s_198_1) == (s_198_3));
        // D s_198_5: write-var gs#133841 <= s_198_4
        fn_state.gs_133841 = s_198_4;
        // N s_198_6: jump b195
        return block_195(state, tracer, fn_state);
    }
    fn block_199<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_199_0: const #432u : u32
        let s_199_0: u32 = 432;
        // D s_199_1: read-reg s_199_0:u8
        let s_199_1: u8 = {
            let value = state.read_register::<u8>(s_199_0 as isize);
            tracer.read_register(s_199_0 as isize, value);
            value
        };
        // D s_199_2: call ELUsingAArch32(s_199_1)
        let s_199_2: bool = ELUsingAArch32(state, tracer, s_199_1);
        // D s_199_3: write-var gs#133840 <= s_199_2
        fn_state.gs_133840 = s_199_2;
        // N s_199_4: jump b193
        return block_193(state, tracer, fn_state);
    }
    fn block_200<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_200_0: const #3u : u8
        let s_200_0: u8 = 3;
        // C s_200_1: cast zx s_200_0 -> bv
        let s_200_1: Bits = Bits::new(s_200_0 as u128, 8u16);
        // C s_200_2: cast zx s_200_1 -> i
        let s_200_2: i128 = (s_200_1.value() as i128);
        // C s_200_3: cast reint s_200_2 -> i64
        let s_200_3: i64 = (s_200_2 as i64);
        // C s_200_4: cast zx s_200_3 -> i
        let s_200_4: i128 = (i128::try_from(s_200_3).unwrap());
        // C s_200_5: const #432u : u32
        let s_200_5: u32 = 432;
        // D s_200_6: read-reg s_200_5:u8
        let s_200_6: u8 = {
            let value = state.read_register::<u8>(s_200_5 as isize);
            tracer.read_register(s_200_5 as isize, value);
            value
        };
        // D s_200_7: call AArch64_AArch32SystemAccessTrap(s_200_6, s_200_4)
        let s_200_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_200_6,
            s_200_4,
        );
        // N s_200_8: return
        return;
    }
    fn block_201<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_201_0: read-var __HCR_EL2_TGE:u8
        let s_201_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_201_1: cast zx s_201_0 -> bv
        let s_201_1: Bits = Bits::new(s_201_0 as u128, 1u16);
        // C s_201_2: const #1u : u8
        let s_201_2: bool = true;
        // C s_201_3: cast zx s_201_2 -> bv
        let s_201_3: Bits = Bits::new(s_201_2 as u128, 1u16);
        // D s_201_4: cmp-eq s_201_1 s_201_3
        let s_201_4: bool = ((s_201_1) == (s_201_3));
        // D s_201_5: write-var gs#133839 <= s_201_4
        fn_state.gs_133839 = s_201_4;
        // N s_201_6: jump b190
        return block_190(state, tracer, fn_state);
    }
    fn block_202<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_202_0: const #432u : u32
        let s_202_0: u32 = 432;
        // D s_202_1: read-reg s_202_0:u8
        let s_202_1: u8 = {
            let value = state.read_register::<u8>(s_202_0 as isize);
            tracer.read_register(s_202_0 as isize, value);
            value
        };
        // D s_202_2: call ELUsingAArch32(s_202_1)
        let s_202_2: bool = ELUsingAArch32(state, tracer, s_202_1);
        // D s_202_3: not s_202_2
        let s_202_3: bool = !s_202_2;
        // D s_202_4: write-var gs#133838 <= s_202_3
        fn_state.gs_133838 = s_202_3;
        // N s_202_5: jump b188
        return block_188(state, tracer, fn_state);
    }
    fn block_203<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_203_0: const #() : ()
        let s_203_0: () = ();
        // S s_203_1: call PMUSERENR_read(s_203_0)
        let s_203_1: ProductType700c18a878c5601b = PMUSERENR_read(
            state,
            tracer,
            s_203_0,
        );
        // S s_203_2: call _get_PMUSERENR_Type_SW(s_203_1)
        let s_203_2: bool = u_get_PMUSERENR_Type_SW(state, tracer, s_203_1);
        // C s_203_3: const #() : ()
        let s_203_3: () = ();
        // S s_203_4: call PMUSERENR_read(s_203_3)
        let s_203_4: ProductType700c18a878c5601b = PMUSERENR_read(
            state,
            tracer,
            s_203_3,
        );
        // S s_203_5: call _get_PMUSERENR_Type_EN(s_203_4)
        let s_203_5: bool = u_get_PMUSERENR_Type_EN(state, tracer, s_203_4);
        // S s_203_6: cast zx s_203_2 -> bv
        let s_203_6: Bits = Bits::new(s_203_2 as u128, 1u16);
        // S s_203_7: cast zx s_203_5 -> bv
        let s_203_7: Bits = Bits::new(s_203_5 as u128, 1u16);
        // S s_203_8: cast reint s_203_6 -> u128
        let s_203_8: u128 = (s_203_6.value() as u128);
        // D s_203_9: size-of s_203_6
        let s_203_9: u16 = s_203_6.length();
        // S s_203_10: cast reint s_203_7 -> u128
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
        // D s_203_21: write-var gs#133819 <= s_203_20
        fn_state.gs_133819 = s_203_20;
        // N s_203_22: jump b116
        return block_116(state, tracer, fn_state);
    }
    fn block_204<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_204_0: const #() : ()
        let s_204_0: () = ();
        // S s_204_1: call EL2Enabled(s_204_0)
        let s_204_1: bool = EL2Enabled(state, tracer, s_204_0);
        // N s_204_2: branch s_204_1 b212 b205
        if s_204_1 {
            return block_212(state, tracer, fn_state);
        } else {
            return block_205(state, tracer, fn_state);
        };
    }
    fn block_205<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_205_0: const #0u : u8
        let s_205_0: bool = false;
        // D s_205_1: write-var gs#133842 <= s_205_0
        fn_state.gs_133842 = s_205_0;
        // N s_205_2: jump b206
        return block_206(state, tracer, fn_state);
    }
    fn block_206<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_206_0: read-var gs#133842:u8
        let s_206_0: bool = fn_state.gs_133842;
        // N s_206_1: branch s_206_0 b211 b207
        if s_206_0 {
            return block_211(state, tracer, fn_state);
        } else {
            return block_207(state, tracer, fn_state);
        };
    }
    fn block_207<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_207_0: const #0u : u8
        let s_207_0: bool = false;
        // D s_207_1: write-var gs#133843 <= s_207_0
        fn_state.gs_133843 = s_207_0;
        // N s_207_2: jump b208
        return block_208(state, tracer, fn_state);
    }
    fn block_208<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_208_0: read-var gs#133843:u8
        let s_208_0: bool = fn_state.gs_133843;
        // N s_208_1: branch s_208_0 b210 b209
        if s_208_0 {
            return block_210(state, tracer, fn_state);
        } else {
            return block_209(state, tracer, fn_state);
        };
    }
    fn block_209<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_209_0: const #3u : u8
        let s_209_0: u8 = 3;
        // C s_209_1: cast zx s_209_0 -> bv
        let s_209_1: Bits = Bits::new(s_209_0 as u128, 8u16);
        // C s_209_2: cast zx s_209_1 -> i
        let s_209_2: i128 = (s_209_1.value() as i128);
        // C s_209_3: cast reint s_209_2 -> i64
        let s_209_3: i64 = (s_209_2 as i64);
        // C s_209_4: cast zx s_209_3 -> i
        let s_209_4: i128 = (i128::try_from(s_209_3).unwrap());
        // C s_209_5: const #440u : u32
        let s_209_5: u32 = 440;
        // D s_209_6: read-reg s_209_5:u8
        let s_209_6: u8 = {
            let value = state.read_register::<u8>(s_209_5 as isize);
            tracer.read_register(s_209_5 as isize, value);
            value
        };
        // D s_209_7: call AArch64_AArch32SystemAccessTrap(s_209_6, s_209_4)
        let s_209_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_209_6,
            s_209_4,
        );
        // N s_209_8: return
        return;
    }
    fn block_210<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_210_0: const #3u : u8
        let s_210_0: u8 = 3;
        // C s_210_1: cast zx s_210_0 -> bv
        let s_210_1: Bits = Bits::new(s_210_0 as u128, 8u16);
        // C s_210_2: cast zx s_210_1 -> i
        let s_210_2: i128 = (s_210_1.value() as i128);
        // C s_210_3: cast reint s_210_2 -> i64
        let s_210_3: i64 = (s_210_2 as i64);
        // C s_210_4: cast zx s_210_3 -> i
        let s_210_4: i128 = (i128::try_from(s_210_3).unwrap());
        // C s_210_5: const #432u : u32
        let s_210_5: u32 = 432;
        // D s_210_6: read-reg s_210_5:u8
        let s_210_6: u8 = {
            let value = state.read_register::<u8>(s_210_5 as isize);
            tracer.read_register(s_210_5 as isize, value);
            value
        };
        // D s_210_7: call AArch64_AArch32SystemAccessTrap(s_210_6, s_210_4)
        let s_210_7: () = AArch64_AArch32SystemAccessTrap(
            state,
            tracer,
            s_210_6,
            s_210_4,
        );
        // N s_210_8: return
        return;
    }
    fn block_211<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_211_0: read-var __HCR_EL2_TGE:u8
        let s_211_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_211_1: cast zx s_211_0 -> bv
        let s_211_1: Bits = Bits::new(s_211_0 as u128, 1u16);
        // C s_211_2: const #1u : u8
        let s_211_2: bool = true;
        // C s_211_3: cast zx s_211_2 -> bv
        let s_211_3: Bits = Bits::new(s_211_2 as u128, 1u16);
        // D s_211_4: cmp-eq s_211_1 s_211_3
        let s_211_4: bool = ((s_211_1) == (s_211_3));
        // D s_211_5: write-var gs#133843 <= s_211_4
        fn_state.gs_133843 = s_211_4;
        // N s_211_6: jump b208
        return block_208(state, tracer, fn_state);
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
        // D s_212_3: not s_212_2
        let s_212_3: bool = !s_212_2;
        // D s_212_4: write-var gs#133842 <= s_212_3
        fn_state.gs_133842 = s_212_3;
        // N s_212_5: jump b206
        return block_206(state, tracer, fn_state);
    }
    fn block_213<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_213_0: const #204u : u32
        let s_213_0: u32 = 204;
        // S s_213_1: call IsFeatureImplemented(s_213_0)
        let s_213_1: bool = IsFeatureImplemented(state, tracer, s_213_0);
        // N s_213_2: branch s_213_1 b222 b214
        if s_213_1 {
            return block_222(state, tracer, fn_state);
        } else {
            return block_214(state, tracer, fn_state);
        };
    }
    fn block_214<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_214_0: const #0u : u8
        let s_214_0: bool = false;
        // D s_214_1: write-var gs#133815 <= s_214_0
        fn_state.gs_133815 = s_214_0;
        // N s_214_2: jump b215
        return block_215(state, tracer, fn_state);
    }
    fn block_215<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_215_0: read-var gs#133815:u8
        let s_215_0: bool = fn_state.gs_133815;
        // N s_215_1: branch s_215_0 b221 b216
        if s_215_0 {
            return block_221(state, tracer, fn_state);
        } else {
            return block_216(state, tracer, fn_state);
        };
    }
    fn block_216<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_216_0: const #204u : u32
        let s_216_0: u32 = 204;
        // S s_216_1: call IsFeatureImplemented(s_216_0)
        let s_216_1: bool = IsFeatureImplemented(state, tracer, s_216_0);
        // S s_216_2: not s_216_1
        let s_216_2: bool = !s_216_1;
        // N s_216_3: branch s_216_2 b220 b217
        if s_216_2 {
            return block_220(state, tracer, fn_state);
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
        // D s_217_1: write-var gs#133816 <= s_217_0
        fn_state.gs_133816 = s_217_0;
        // N s_217_2: jump b218
        return block_218(state, tracer, fn_state);
    }
    fn block_218<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_218_0: read-var gs#133816:u8
        let s_218_0: bool = fn_state.gs_133816;
        // D s_218_1: write-var gs#133817 <= s_218_0
        fn_state.gs_133817 = s_218_0;
        // N s_218_2: jump b219
        return block_219(state, tracer, fn_state);
    }
    fn block_219<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_219_0: read-var gs#133817:u8
        let s_219_0: bool = fn_state.gs_133817;
        // D s_219_1: write-var gs#133818 <= s_219_0
        fn_state.gs_133818 = s_219_0;
        // N s_219_2: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_220<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_220_0: const #102624u : u32
        let s_220_0: u32 = 102624;
        // D s_220_1: read-reg s_220_0:struct
        let s_220_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_220_0 as isize);
            tracer.read_register(s_220_0 as isize, value);
            value
        };
        // D s_220_2: call _get_PMUSERENR_EL0_Type_SW(s_220_1)
        let s_220_2: bool = u_get_PMUSERENR_EL0_Type_SW(state, tracer, s_220_1);
        // C s_220_3: const #102624u : u32
        let s_220_3: u32 = 102624;
        // D s_220_4: read-reg s_220_3:struct
        let s_220_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_220_3 as isize);
            tracer.read_register(s_220_3 as isize, value);
            value
        };
        // D s_220_5: call _get_PMUSERENR_EL0_Type_EN(s_220_4)
        let s_220_5: bool = u_get_PMUSERENR_EL0_Type_EN(state, tracer, s_220_4);
        // D s_220_6: cast zx s_220_2 -> bv
        let s_220_6: Bits = Bits::new(s_220_2 as u128, 1u16);
        // D s_220_7: cast zx s_220_5 -> bv
        let s_220_7: Bits = Bits::new(s_220_5 as u128, 1u16);
        // D s_220_8: cast reint s_220_6 -> u128
        let s_220_8: u128 = (s_220_6.value() as u128);
        // D s_220_9: size-of s_220_6
        let s_220_9: u16 = s_220_6.length();
        // D s_220_10: cast reint s_220_7 -> u128
        let s_220_10: u128 = (s_220_7.value() as u128);
        // D s_220_11: size-of s_220_7
        let s_220_11: u16 = s_220_7.length();
        // D s_220_12: lsl s_220_8 s_220_11
        let s_220_12: u128 = s_220_8 << s_220_11;
        // D s_220_13: or s_220_12 s_220_10
        let s_220_13: u128 = ((s_220_12) | (s_220_10));
        // D s_220_14: add s_220_9 s_220_11
        let s_220_14: u16 = (s_220_9 + s_220_11);
        // D s_220_15: create-bits s_220_13 s_220_14
        let s_220_15: Bits = Bits::new(s_220_13, s_220_14);
        // D s_220_16: cast reint s_220_15 -> u8
        let s_220_16: u8 = (s_220_15.value() as u8);
        // D s_220_17: cast zx s_220_16 -> bv
        let s_220_17: Bits = Bits::new(s_220_16 as u128, 2u16);
        // C s_220_18: const #0u : u8
        let s_220_18: u8 = 0;
        // C s_220_19: cast zx s_220_18 -> bv
        let s_220_19: Bits = Bits::new(s_220_18 as u128, 2u16);
        // D s_220_20: cmp-eq s_220_17 s_220_19
        let s_220_20: bool = ((s_220_17) == (s_220_19));
        // D s_220_21: write-var gs#133816 <= s_220_20
        fn_state.gs_133816 = s_220_20;
        // N s_220_22: jump b218
        return block_218(state, tracer, fn_state);
    }
    fn block_221<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_221_0: const #1u : u8
        let s_221_0: bool = true;
        // D s_221_1: write-var gs#133817 <= s_221_0
        fn_state.gs_133817 = s_221_0;
        // N s_221_2: jump b219
        return block_219(state, tracer, fn_state);
    }
    fn block_222<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_222_0: const #102624u : u32
        let s_222_0: u32 = 102624;
        // D s_222_1: read-reg s_222_0:struct
        let s_222_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_222_0 as isize);
            tracer.read_register(s_222_0 as isize, value);
            value
        };
        // D s_222_2: call _get_PMUSERENR_EL0_Type_UEN(s_222_1)
        let s_222_2: bool = u_get_PMUSERENR_EL0_Type_UEN(state, tracer, s_222_1);
        // C s_222_3: const #102624u : u32
        let s_222_3: u32 = 102624;
        // D s_222_4: read-reg s_222_3:struct
        let s_222_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_222_3 as isize);
            tracer.read_register(s_222_3 as isize, value);
            value
        };
        // D s_222_5: call _get_PMUSERENR_EL0_Type_SW(s_222_4)
        let s_222_5: bool = u_get_PMUSERENR_EL0_Type_SW(state, tracer, s_222_4);
        // C s_222_6: const #102624u : u32
        let s_222_6: u32 = 102624;
        // D s_222_7: read-reg s_222_6:struct
        let s_222_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_222_6 as isize);
            tracer.read_register(s_222_6 as isize, value);
            value
        };
        // D s_222_8: call _get_PMUSERENR_EL0_Type_EN(s_222_7)
        let s_222_8: bool = u_get_PMUSERENR_EL0_Type_EN(state, tracer, s_222_7);
        // D s_222_9: cast zx s_222_5 -> bv
        let s_222_9: Bits = Bits::new(s_222_5 as u128, 1u16);
        // D s_222_10: cast zx s_222_8 -> bv
        let s_222_10: Bits = Bits::new(s_222_8 as u128, 1u16);
        // D s_222_11: cast reint s_222_9 -> u128
        let s_222_11: u128 = (s_222_9.value() as u128);
        // D s_222_12: size-of s_222_9
        let s_222_12: u16 = s_222_9.length();
        // D s_222_13: cast reint s_222_10 -> u128
        let s_222_13: u128 = (s_222_10.value() as u128);
        // D s_222_14: size-of s_222_10
        let s_222_14: u16 = s_222_10.length();
        // D s_222_15: lsl s_222_11 s_222_14
        let s_222_15: u128 = s_222_11 << s_222_14;
        // D s_222_16: or s_222_15 s_222_13
        let s_222_16: u128 = ((s_222_15) | (s_222_13));
        // D s_222_17: add s_222_12 s_222_14
        let s_222_17: u16 = (s_222_12 + s_222_14);
        // D s_222_18: create-bits s_222_16 s_222_17
        let s_222_18: Bits = Bits::new(s_222_16, s_222_17);
        // D s_222_19: cast reint s_222_18 -> u8
        let s_222_19: u8 = (s_222_18.value() as u8);
        // D s_222_20: cast zx s_222_2 -> bv
        let s_222_20: Bits = Bits::new(s_222_2 as u128, 1u16);
        // D s_222_21: cast zx s_222_19 -> bv
        let s_222_21: Bits = Bits::new(s_222_19 as u128, 2u16);
        // D s_222_22: cast reint s_222_20 -> u128
        let s_222_22: u128 = (s_222_20.value() as u128);
        // D s_222_23: size-of s_222_20
        let s_222_23: u16 = s_222_20.length();
        // D s_222_24: cast reint s_222_21 -> u128
        let s_222_24: u128 = (s_222_21.value() as u128);
        // D s_222_25: size-of s_222_21
        let s_222_25: u16 = s_222_21.length();
        // D s_222_26: lsl s_222_22 s_222_25
        let s_222_26: u128 = s_222_22 << s_222_25;
        // D s_222_27: or s_222_26 s_222_24
        let s_222_27: u128 = ((s_222_26) | (s_222_24));
        // D s_222_28: add s_222_23 s_222_25
        let s_222_28: u16 = (s_222_23 + s_222_25);
        // D s_222_29: create-bits s_222_27 s_222_28
        let s_222_29: Bits = Bits::new(s_222_27, s_222_28);
        // D s_222_30: cast reint s_222_29 -> u8
        let s_222_30: u8 = (s_222_29.value() as u8);
        // D s_222_31: cast zx s_222_30 -> bv
        let s_222_31: Bits = Bits::new(s_222_30 as u128, 3u16);
        // C s_222_32: const #0u : u8
        let s_222_32: u8 = 0;
        // C s_222_33: cast zx s_222_32 -> bv
        let s_222_33: Bits = Bits::new(s_222_32 as u128, 3u16);
        // D s_222_34: cmp-eq s_222_31 s_222_33
        let s_222_34: bool = ((s_222_31) == (s_222_33));
        // D s_222_35: write-var gs#133815 <= s_222_34
        fn_state.gs_133815 = s_222_34;
        // N s_222_36: jump b215
        return block_215(state, tracer, fn_state);
    }
    fn block_223<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_223_0: panic
        panic!("{:?}", ());
        // N s_223_1: return
        return;
    }
    fn block_224<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_224_0: read-var __MDCR_EL3_TPM:u8
        let s_224_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_224_1: cast zx s_224_0 -> bv
        let s_224_1: Bits = Bits::new(s_224_0 as u128, 1u16);
        // C s_224_2: const #1u : u8
        let s_224_2: bool = true;
        // C s_224_3: cast zx s_224_2 -> bv
        let s_224_3: Bits = Bits::new(s_224_2 as u128, 1u16);
        // D s_224_4: cmp-eq s_224_1 s_224_3
        let s_224_4: bool = ((s_224_1) == (s_224_3));
        // D s_224_5: write-var gs#133814 <= s_224_4
        fn_state.gs_133814 = s_224_4;
        // N s_224_6: jump b110
        return block_110(state, tracer, fn_state);
    }
    fn block_225<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_225_0: const #424u : u32
        let s_225_0: u32 = 424;
        // D s_225_1: read-reg s_225_0:u8
        let s_225_1: u8 = {
            let value = state.read_register::<u8>(s_225_0 as isize);
            tracer.read_register(s_225_0 as isize, value);
            value
        };
        // D s_225_2: call ELUsingAArch32(s_225_1)
        let s_225_2: bool = ELUsingAArch32(state, tracer, s_225_1);
        // D s_225_3: not s_225_2
        let s_225_3: bool = !s_225_2;
        // D s_225_4: write-var gs#133813 <= s_225_3
        fn_state.gs_133813 = s_225_3;
        // N s_225_5: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_226<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_226_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_226_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_226_1: call __IMPDEF_boolean(s_226_0)
        let s_226_1: bool = u__IMPDEF_boolean(state, tracer, s_226_0);
        // D s_226_2: write-var gs#133812 <= s_226_1
        fn_state.gs_133812 = s_226_1;
        // N s_226_3: jump b106
        return block_106(state, tracer, fn_state);
    }
    fn block_227<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_227_0: const #() : ()
        let s_227_0: () = ();
        // S s_227_1: call EDSCR_read(s_227_0)
        let s_227_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_227_0);
        // S s_227_2: call _get_EDSCR_Type_SDD(s_227_1)
        let s_227_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_227_1);
        // S s_227_3: cast zx s_227_2 -> bv
        let s_227_3: Bits = Bits::new(s_227_2 as u128, 1u16);
        // C s_227_4: const #1u : u8
        let s_227_4: bool = true;
        // C s_227_5: cast zx s_227_4 -> bv
        let s_227_5: Bits = Bits::new(s_227_4 as u128, 1u16);
        // S s_227_6: cmp-eq s_227_3 s_227_5
        let s_227_6: bool = ((s_227_3) == (s_227_5));
        // D s_227_7: write-var gs#133811 <= s_227_6
        fn_state.gs_133811 = s_227_6;
        // N s_227_8: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_228<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_228_0: const #424u : u32
        let s_228_0: u32 = 424;
        // D s_228_1: read-reg s_228_0:u8
        let s_228_1: u8 = {
            let value = state.read_register::<u8>(s_228_0 as isize);
            tracer.read_register(s_228_0 as isize, value);
            value
        };
        // C s_228_2: const #2u : u8
        let s_228_2: u8 = 2;
        // D s_228_3: cmp-lt s_228_1 s_228_2
        let s_228_3: bool = ((s_228_1) < (s_228_2));
        // D s_228_4: write-var gs#133810 <= s_228_3
        fn_state.gs_133810 = s_228_3;
        // N s_228_5: jump b102
        return block_102(state, tracer, fn_state);
    }
}
