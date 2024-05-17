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
use u_get_MDCR_EL2_Type_TPM::*;
use u_get_PMUSERENR_EL0_Type_UEN::*;
use u_get_SCR_EL3_Type_FGTEn::*;
use u_get_HCR_EL2_Type_E2H::*;
use Halted::*;
use IsFeatureImplemented::*;
use AArch64_SystemAccessTrap::*;
use u__IMPDEF_boolean::*;
use ConstrainUnpredictableProcedure::*;
use u_get_PMUSERENR_EL0_Type_ER::*;
use u_get_MDCR_EL3_Type_TPM::*;
use u_get_PMSELR_EL0_Type_SEL::*;
use X_set::*;
use AArch64_GetNumEventCountersAccessible::*;
use u_get_PMUSERENR_EL0_Type_EN::*;
use u_get_EDSCR_Type_SDD::*;
use u_get_HCR_EL2_Type_TGE::*;
use EL2Enabled::*;
use EDSCR_read::*;
use u_get_HDFGRTR_EL2_Type_PMEVCNTRn_EL0::*;
use common::*;
pub fn PMXEVCNTR_EL0_SysRegRead_52bf82e65ceb6b97<T: Tracer>(
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
        gs_68829: bool,
        gs_68807: bool,
        gs_68799: bool,
        gs_68818: bool,
        gs_68826: bool,
        gs_68808: bool,
        gs_68833: bool,
        gs_68821: bool,
        u__MDCR_EL3_TPM: bool,
        gs_68824: bool,
        gs_68797: bool,
        gs_68803: bool,
        gs_68798: bool,
        gs_68819: bool,
        gs_68822: bool,
        gs_68827: bool,
        gs_68806: bool,
        gs_68816: bool,
        u__PSTATE_EL: u8,
        gs_68801: bool,
        gs_68800: bool,
        u__MDCR_EL2_TPM: bool,
        gs_68828: bool,
        gs_68809: bool,
        u__HCR_EL2_TGE: bool,
        u__EDSCR_SDD: bool,
        gs_68820: bool,
        gs_68825: bool,
        u__SCR_EL3_FGTEn: bool,
        gs_68805: bool,
        gs_68834: bool,
        gs_68810: bool,
        u__HDFGRTR_EL2_PMEVCNTRn_EL0: bool,
        gs_68804: bool,
        gs_68830: bool,
        gs_68814: bool,
        gs_68813: bool,
        gs_68811: bool,
        gs_68823: bool,
        gs_68817: bool,
        gs_68831: bool,
        gs_68812: bool,
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
        // C s_0_7: const #22712u : u32
        let s_0_7: u32 = 22712;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_MDCR_EL3_Type_TPM(s_0_8)
        let s_0_9: bool = u_get_MDCR_EL3_Type_TPM(state, tracer, s_0_8);
        // D s_0_10: write-var __MDCR_EL3_TPM <= s_0_9
        fn_state.u__MDCR_EL3_TPM = s_0_9;
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
        // C s_0_27: const #19136u : u32
        let s_0_27: u32 = 19136;
        // D s_0_28: read-reg s_0_27:struct
        let s_0_28: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_27 as isize);
            tracer.read_register(s_0_27 as isize, value);
            value
        };
        // D s_0_29: call _get_PMSELR_EL0_Type_SEL(s_0_28)
        let s_0_29: u8 = u_get_PMSELR_EL0_Type_SEL(state, tracer, s_0_28);
        // D s_0_30: cast zx s_0_29 -> bv
        let s_0_30: Bits = Bits::new(s_0_29 as u128, 5u16);
        // D s_0_31: cast zx s_0_30 -> i
        let s_0_31: i128 = (s_0_30.value() as i128);
        // D s_0_32: cast reint s_0_31 -> i64
        let s_0_32: i64 = (s_0_31 as i64);
        // C s_0_33: const #31s : i
        let s_0_33: i128 = 31;
        // D s_0_34: cast zx s_0_32 -> i
        let s_0_34: i128 = (i128::try_from(s_0_32).unwrap());
        // D s_0_35: cmp-ge s_0_34 s_0_33
        let s_0_35: bool = ((s_0_34) >= (s_0_33));
        // N s_0_36: branch s_0_35 b153 b1
        if s_0_35 {
            return block_153(state, tracer, fn_state);
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
        // C s_1_2: const #448u : u32
        let s_1_2: u32 = 448;
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
        // N s_1_6: branch s_1_5 b83 b2
        if s_1_5 {
            return block_83(state, tracer, fn_state);
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
        // C s_2_2: const #440u : u32
        let s_2_2: u32 = 440;
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
        // N s_2_6: branch s_2_5 b32 b3
        if s_2_5 {
            return block_32(state, tracer, fn_state);
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
        // C s_3_2: const #432u : u32
        let s_3_2: u32 = 432;
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
        // N s_3_6: branch s_3_5 b7 b4
        if s_3_5 {
            return block_7(state, tracer, fn_state);
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
        // C s_4_2: const #424u : u32
        let s_4_2: u32 = 424;
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
        // N s_4_6: branch s_4_5 b6 b5
        if s_4_5 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_5_0: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #64s : i64
        let s_6_0: i64 = 64;
        // C s_6_1: const #19136u : u32
        let s_6_1: u32 = 19136;
        // D s_6_2: read-reg s_6_1:struct
        let s_6_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_6_1 as isize);
            tracer.read_register(s_6_1 as isize, value);
            value
        };
        // D s_6_3: call _get_PMSELR_EL0_Type_SEL(s_6_2)
        let s_6_3: u8 = u_get_PMSELR_EL0_Type_SEL(state, tracer, s_6_2);
        // D s_6_4: cast zx s_6_3 -> bv
        let s_6_4: Bits = Bits::new(s_6_3 as u128, 5u16);
        // D s_6_5: cast zx s_6_4 -> i
        let s_6_5: i128 = (s_6_4.value() as i128);
        // D s_6_6: cast reint s_6_5 -> i64
        let s_6_6: i64 = (s_6_5 as i64);
        // C s_6_7: const #10744u : u32
        let s_6_7: u32 = 10744;
        // D s_6_8: read-reg s_6_7:[u64; 32]
        let s_6_8: [u64; 32usize] = {
            let value = state.read_register::<[u64; 32usize]>(s_6_7 as isize);
            tracer.read_register(s_6_7 as isize, value);
            value
        };
        // D s_6_9: cast zx s_6_6 -> i
        let s_6_9: i128 = (i128::try_from(s_6_6).unwrap());
        // D s_6_10: read-element s_6_8[s_6_9]
        let s_6_10: u64 = s_6_8[(s_6_9) as usize];
        // D s_6_11: cast zx s_6_10 -> bv
        let s_6_11: Bits = Bits::new(s_6_10 as u128, 64u16);
        // D s_6_12: read-var t:i
        let s_6_12: i128 = fn_state.t;
        // D s_6_13: call X_set(s_6_12, s_6_0, s_6_11)
        let s_6_13: () = X_set(state, tracer, s_6_12, s_6_0, s_6_11);
        // N s_6_14: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #() : ()
        let s_7_0: () = ();
        // S s_7_1: call Halted(s_7_0)
        let s_7_1: bool = Halted(state, tracer, s_7_0);
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
        // D s_8_1: write-var gs#68797 <= s_8_0
        fn_state.gs_68797 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#68797:u8
        let s_9_0: bool = fn_state.gs_68797;
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
        // D s_10_1: write-var gs#68798 <= s_10_0
        fn_state.gs_68798 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#68798:u8
        let s_11_0: bool = fn_state.gs_68798;
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
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var gs#68799 <= s_12_0
        fn_state.gs_68799 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#68799:u8
        let s_13_0: bool = fn_state.gs_68799;
        // N s_13_1: branch s_13_0 b28 b14
        if s_13_0 {
            return block_28(state, tracer, fn_state);
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
        // D s_14_1: write-var gs#68800 <= s_14_0
        fn_state.gs_68800 = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var gs#68800:u8
        let s_15_0: bool = fn_state.gs_68800;
        // N s_15_1: branch s_15_0 b27 b16
        if s_15_0 {
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
        // C s_16_0: const #424u : u32
        let s_16_0: u32 = 424;
        // D s_16_1: read-reg s_16_0:u8
        let s_16_1: u8 = {
            let value = state.read_register::<u8>(s_16_0 as isize);
            tracer.read_register(s_16_0 as isize, value);
            value
        };
        // C s_16_2: const #2u : u8
        let s_16_2: u8 = 2;
        // D s_16_3: cmp-lt s_16_1 s_16_2
        let s_16_3: bool = ((s_16_1) < (s_16_2));
        // N s_16_4: branch s_16_3 b26 b17
        if s_16_3 {
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
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var gs#68801 <= s_17_0
        fn_state.gs_68801 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#68801:u8
        let s_18_0: bool = fn_state.gs_68801;
        // N s_18_1: branch s_18_0 b20 b19
        if s_18_0 {
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
        // C s_19_1: const #19136u : u32
        let s_19_1: u32 = 19136;
        // D s_19_2: read-reg s_19_1:struct
        let s_19_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_19_1 as isize);
            tracer.read_register(s_19_1 as isize, value);
            value
        };
        // D s_19_3: call _get_PMSELR_EL0_Type_SEL(s_19_2)
        let s_19_3: u8 = u_get_PMSELR_EL0_Type_SEL(state, tracer, s_19_2);
        // D s_19_4: cast zx s_19_3 -> bv
        let s_19_4: Bits = Bits::new(s_19_3 as u128, 5u16);
        // D s_19_5: cast zx s_19_4 -> i
        let s_19_5: i128 = (s_19_4.value() as i128);
        // D s_19_6: cast reint s_19_5 -> i64
        let s_19_6: i64 = (s_19_5 as i64);
        // C s_19_7: const #10744u : u32
        let s_19_7: u32 = 10744;
        // D s_19_8: read-reg s_19_7:[u64; 32]
        let s_19_8: [u64; 32usize] = {
            let value = state.read_register::<[u64; 32usize]>(s_19_7 as isize);
            tracer.read_register(s_19_7 as isize, value);
            value
        };
        // D s_19_9: cast zx s_19_6 -> i
        let s_19_9: i128 = (i128::try_from(s_19_6).unwrap());
        // D s_19_10: read-element s_19_8[s_19_9]
        let s_19_10: u64 = s_19_8[(s_19_9) as usize];
        // D s_19_11: cast zx s_19_10 -> bv
        let s_19_11: Bits = Bits::new(s_19_10 as u128, 64u16);
        // D s_19_12: read-var t:i
        let s_19_12: i128 = fn_state.t;
        // D s_19_13: call X_set(s_19_12, s_19_0, s_19_11)
        let s_19_13: () = X_set(state, tracer, s_19_12, s_19_0, s_19_11);
        // N s_19_14: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #() : ()
        let s_20_0: () = ();
        // S s_20_1: call Halted(s_20_0)
        let s_20_1: bool = Halted(state, tracer, s_20_0);
        // N s_20_2: branch s_20_1 b25 b21
        if s_20_1 {
            return block_25(state, tracer, fn_state);
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
        // D s_21_1: write-var gs#68803 <= s_21_0
        fn_state.gs_68803 = s_21_0;
        // N s_21_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#68803:u8
        let s_22_0: bool = fn_state.gs_68803;
        // N s_22_1: branch s_22_0 b24 b23
        if s_22_0 {
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
        // C s_23_0: const #24u : u8
        let s_23_0: u8 = 24;
        // C s_23_1: cast zx s_23_0 -> bv
        let s_23_1: Bits = Bits::new(s_23_0 as u128, 8u16);
        // C s_23_2: cast zx s_23_1 -> i
        let s_23_2: i128 = (s_23_1.value() as i128);
        // C s_23_3: cast reint s_23_2 -> i64
        let s_23_3: i64 = (s_23_2 as i64);
        // C s_23_4: cast zx s_23_3 -> i
        let s_23_4: i128 = (i128::try_from(s_23_3).unwrap());
        // C s_23_5: const #424u : u32
        let s_23_5: u32 = 424;
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
        // N s_24_0: panic
        panic!("{:?}", ());
        // N s_24_1: return
        return;
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var __EDSCR_SDD:u8
        let s_25_0: bool = fn_state.u__EDSCR_SDD;
        // D s_25_1: cast zx s_25_0 -> bv
        let s_25_1: Bits = Bits::new(s_25_0 as u128, 1u16);
        // C s_25_2: const #1u : u8
        let s_25_2: bool = true;
        // C s_25_3: cast zx s_25_2 -> bv
        let s_25_3: Bits = Bits::new(s_25_2 as u128, 1u16);
        // D s_25_4: cmp-eq s_25_1 s_25_3
        let s_25_4: bool = ((s_25_1) == (s_25_3));
        // D s_25_5: write-var gs#68803 <= s_25_4
        fn_state.gs_68803 = s_25_4;
        // N s_25_6: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var __MDCR_EL3_TPM:u8
        let s_26_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_26_1: cast zx s_26_0 -> bv
        let s_26_1: Bits = Bits::new(s_26_0 as u128, 1u16);
        // C s_26_2: const #1u : u8
        let s_26_2: bool = true;
        // C s_26_3: cast zx s_26_2 -> bv
        let s_26_3: Bits = Bits::new(s_26_2 as u128, 1u16);
        // D s_26_4: cmp-eq s_26_1 s_26_3
        let s_26_4: bool = ((s_26_1) == (s_26_3));
        // D s_26_5: write-var gs#68801 <= s_26_4
        fn_state.gs_68801 = s_26_4;
        // N s_26_6: jump b18
        return block_18(state, tracer, fn_state);
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
        // D s_28_0: read-var __MDCR_EL3_TPM:u8
        let s_28_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_28_1: cast zx s_28_0 -> bv
        let s_28_1: Bits = Bits::new(s_28_0 as u128, 1u16);
        // C s_28_2: const #1u : u8
        let s_28_2: bool = true;
        // C s_28_3: cast zx s_28_2 -> bv
        let s_28_3: Bits = Bits::new(s_28_2 as u128, 1u16);
        // D s_28_4: cmp-eq s_28_1 s_28_3
        let s_28_4: bool = ((s_28_1) == (s_28_3));
        // D s_28_5: write-var gs#68800 <= s_28_4
        fn_state.gs_68800 = s_28_4;
        // N s_28_6: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_29_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_29_1: call __IMPDEF_boolean(s_29_0)
        let s_29_1: bool = u__IMPDEF_boolean(state, tracer, s_29_0);
        // D s_29_2: write-var gs#68799 <= s_29_1
        fn_state.gs_68799 = s_29_1;
        // N s_29_3: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var __EDSCR_SDD:u8
        let s_30_0: bool = fn_state.u__EDSCR_SDD;
        // D s_30_1: cast zx s_30_0 -> bv
        let s_30_1: Bits = Bits::new(s_30_0 as u128, 1u16);
        // C s_30_2: const #1u : u8
        let s_30_2: bool = true;
        // C s_30_3: cast zx s_30_2 -> bv
        let s_30_3: Bits = Bits::new(s_30_2 as u128, 1u16);
        // D s_30_4: cmp-eq s_30_1 s_30_3
        let s_30_4: bool = ((s_30_1) == (s_30_3));
        // D s_30_5: write-var gs#68798 <= s_30_4
        fn_state.gs_68798 = s_30_4;
        // N s_30_6: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #424u : u32
        let s_31_0: u32 = 424;
        // D s_31_1: read-reg s_31_0:u8
        let s_31_1: u8 = {
            let value = state.read_register::<u8>(s_31_0 as isize);
            tracer.read_register(s_31_0 as isize, value);
            value
        };
        // C s_31_2: const #2u : u8
        let s_31_2: u8 = 2;
        // D s_31_3: cmp-lt s_31_1 s_31_2
        let s_31_3: bool = ((s_31_1) < (s_31_2));
        // D s_31_4: write-var gs#68797 <= s_31_3
        fn_state.gs_68797 = s_31_3;
        // N s_31_5: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #() : ()
        let s_32_0: () = ();
        // S s_32_1: call Halted(s_32_0)
        let s_32_1: bool = Halted(state, tracer, s_32_0);
        // N s_32_2: branch s_32_1 b82 b33
        if s_32_1 {
            return block_82(state, tracer, fn_state);
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
        // D s_33_1: write-var gs#68804 <= s_33_0
        fn_state.gs_68804 = s_33_0;
        // N s_33_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var gs#68804:u8
        let s_34_0: bool = fn_state.gs_68804;
        // N s_34_1: branch s_34_0 b81 b35
        if s_34_0 {
            return block_81(state, tracer, fn_state);
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
        // D s_35_1: write-var gs#68805 <= s_35_0
        fn_state.gs_68805 = s_35_0;
        // N s_35_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var gs#68805:u8
        let s_36_0: bool = fn_state.gs_68805;
        // N s_36_1: branch s_36_0 b80 b37
        if s_36_0 {
            return block_80(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #0u : u8
        let s_37_0: bool = false;
        // D s_37_1: write-var gs#68806 <= s_37_0
        fn_state.gs_68806 = s_37_0;
        // N s_37_2: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_38_0: read-var gs#68806:u8
        let s_38_0: bool = fn_state.gs_68806;
        // N s_38_1: branch s_38_0 b79 b39
        if s_38_0 {
            return block_79(state, tracer, fn_state);
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
        // D s_39_1: write-var gs#68807 <= s_39_0
        fn_state.gs_68807 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#68807:u8
        let s_40_0: bool = fn_state.gs_68807;
        // N s_40_1: branch s_40_0 b78 b41
        if s_40_0 {
            return block_78(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
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
        // N s_41_2: branch s_41_1 b77 b42
        if s_41_1 {
            return block_77(state, tracer, fn_state);
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
        // D s_42_1: write-var gs#68808 <= s_42_0
        fn_state.gs_68808 = s_42_0;
        // N s_42_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var gs#68808:u8
        let s_43_0: bool = fn_state.gs_68808;
        // N s_43_1: branch s_43_0 b73 b44
        if s_43_0 {
            return block_73(state, tracer, fn_state);
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
        // D s_44_1: write-var gs#68810 <= s_44_0
        fn_state.gs_68810 = s_44_0;
        // N s_44_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_45_0: read-var gs#68810:u8
        let s_45_0: bool = fn_state.gs_68810;
        // N s_45_1: branch s_45_0 b72 b46
        if s_45_0 {
            return block_72(state, tracer, fn_state);
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
        // D s_46_1: write-var gs#68811 <= s_46_0
        fn_state.gs_68811 = s_46_0;
        // N s_46_2: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var gs#68811:u8
        let s_47_0: bool = fn_state.gs_68811;
        // N s_47_1: branch s_47_0 b71 b48
        if s_47_0 {
            return block_71(state, tracer, fn_state);
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
        // N s_48_2: branch s_48_1 b70 b49
        if s_48_1 {
            return block_70(state, tracer, fn_state);
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
        // D s_49_1: write-var gs#68812 <= s_49_0
        fn_state.gs_68812 = s_49_0;
        // N s_49_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var gs#68812:u8
        let s_50_0: bool = fn_state.gs_68812;
        // N s_50_1: branch s_50_0 b69 b51
        if s_50_0 {
            return block_69(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #() : ()
        let s_51_0: () = ();
        // S s_51_1: call EL2Enabled(s_51_0)
        let s_51_1: bool = EL2Enabled(state, tracer, s_51_0);
        // N s_51_2: branch s_51_1 b68 b52
        if s_51_1 {
            return block_68(state, tracer, fn_state);
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
        // D s_52_1: write-var gs#68813 <= s_52_0
        fn_state.gs_68813 = s_52_0;
        // N s_52_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var gs#68813:u8
        let s_53_0: bool = fn_state.gs_68813;
        // N s_53_1: branch s_53_0 b65 b54
        if s_53_0 {
            return block_65(state, tracer, fn_state);
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
        // N s_54_4: branch s_54_3 b64 b55
        if s_54_3 {
            return block_64(state, tracer, fn_state);
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
        // D s_55_1: write-var gs#68814 <= s_55_0
        fn_state.gs_68814 = s_55_0;
        // N s_55_2: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var gs#68814:u8
        let s_56_0: bool = fn_state.gs_68814;
        // N s_56_1: branch s_56_0 b58 b57
        if s_56_0 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #64s : i64
        let s_57_0: i64 = 64;
        // C s_57_1: const #19136u : u32
        let s_57_1: u32 = 19136;
        // D s_57_2: read-reg s_57_1:struct
        let s_57_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_57_1 as isize);
            tracer.read_register(s_57_1 as isize, value);
            value
        };
        // D s_57_3: call _get_PMSELR_EL0_Type_SEL(s_57_2)
        let s_57_3: u8 = u_get_PMSELR_EL0_Type_SEL(state, tracer, s_57_2);
        // D s_57_4: cast zx s_57_3 -> bv
        let s_57_4: Bits = Bits::new(s_57_3 as u128, 5u16);
        // D s_57_5: cast zx s_57_4 -> i
        let s_57_5: i128 = (s_57_4.value() as i128);
        // D s_57_6: cast reint s_57_5 -> i64
        let s_57_6: i64 = (s_57_5 as i64);
        // C s_57_7: const #10744u : u32
        let s_57_7: u32 = 10744;
        // D s_57_8: read-reg s_57_7:[u64; 32]
        let s_57_8: [u64; 32usize] = {
            let value = state.read_register::<[u64; 32usize]>(s_57_7 as isize);
            tracer.read_register(s_57_7 as isize, value);
            value
        };
        // D s_57_9: cast zx s_57_6 -> i
        let s_57_9: i128 = (i128::try_from(s_57_6).unwrap());
        // D s_57_10: read-element s_57_8[s_57_9]
        let s_57_10: u64 = s_57_8[(s_57_9) as usize];
        // D s_57_11: cast zx s_57_10 -> bv
        let s_57_11: Bits = Bits::new(s_57_10 as u128, 64u16);
        // D s_57_12: read-var t:i
        let s_57_12: i128 = fn_state.t;
        // D s_57_13: call X_set(s_57_12, s_57_0, s_57_11)
        let s_57_13: () = X_set(state, tracer, s_57_12, s_57_0, s_57_11);
        // N s_57_14: return
        return;
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #() : ()
        let s_58_0: () = ();
        // S s_58_1: call Halted(s_58_0)
        let s_58_1: bool = Halted(state, tracer, s_58_0);
        // N s_58_2: branch s_58_1 b63 b59
        if s_58_1 {
            return block_63(state, tracer, fn_state);
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
        // D s_59_1: write-var gs#68816 <= s_59_0
        fn_state.gs_68816 = s_59_0;
        // N s_59_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var gs#68816:u8
        let s_60_0: bool = fn_state.gs_68816;
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
        // C s_61_0: const #24u : u8
        let s_61_0: u8 = 24;
        // C s_61_1: cast zx s_61_0 -> bv
        let s_61_1: Bits = Bits::new(s_61_0 as u128, 8u16);
        // C s_61_2: cast zx s_61_1 -> i
        let s_61_2: i128 = (s_61_1.value() as i128);
        // C s_61_3: cast reint s_61_2 -> i64
        let s_61_3: i64 = (s_61_2 as i64);
        // C s_61_4: cast zx s_61_3 -> i
        let s_61_4: i128 = (i128::try_from(s_61_3).unwrap());
        // C s_61_5: const #424u : u32
        let s_61_5: u32 = 424;
        // D s_61_6: read-reg s_61_5:u8
        let s_61_6: u8 = {
            let value = state.read_register::<u8>(s_61_5 as isize);
            tracer.read_register(s_61_5 as isize, value);
            value
        };
        // D s_61_7: call AArch64_SystemAccessTrap(s_61_6, s_61_4)
        let s_61_7: () = AArch64_SystemAccessTrap(state, tracer, s_61_6, s_61_4);
        // N s_61_8: return
        return;
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_62_0: panic
        panic!("{:?}", ());
        // N s_62_1: return
        return;
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_63_0: read-var __EDSCR_SDD:u8
        let s_63_0: bool = fn_state.u__EDSCR_SDD;
        // D s_63_1: cast zx s_63_0 -> bv
        let s_63_1: Bits = Bits::new(s_63_0 as u128, 1u16);
        // C s_63_2: const #1u : u8
        let s_63_2: bool = true;
        // C s_63_3: cast zx s_63_2 -> bv
        let s_63_3: Bits = Bits::new(s_63_2 as u128, 1u16);
        // D s_63_4: cmp-eq s_63_1 s_63_3
        let s_63_4: bool = ((s_63_1) == (s_63_3));
        // D s_63_5: write-var gs#68816 <= s_63_4
        fn_state.gs_68816 = s_63_4;
        // N s_63_6: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var __MDCR_EL3_TPM:u8
        let s_64_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_64_1: cast zx s_64_0 -> bv
        let s_64_1: Bits = Bits::new(s_64_0 as u128, 1u16);
        // C s_64_2: const #1u : u8
        let s_64_2: bool = true;
        // C s_64_3: cast zx s_64_2 -> bv
        let s_64_3: Bits = Bits::new(s_64_2 as u128, 1u16);
        // D s_64_4: cmp-eq s_64_1 s_64_3
        let s_64_4: bool = ((s_64_1) == (s_64_3));
        // D s_64_5: write-var gs#68814 <= s_64_4
        fn_state.gs_68814 = s_64_4;
        // N s_64_6: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #146u : u32
        let s_65_0: u32 = 146;
        // S s_65_1: call IsFeatureImplemented(s_65_0)
        let s_65_1: bool = IsFeatureImplemented(state, tracer, s_65_0);
        // S s_65_2: not s_65_1
        let s_65_2: bool = !s_65_1;
        // N s_65_3: branch s_65_2 b67 b66
        if s_65_2 {
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
        // C s_66_0: const #24u : u8
        let s_66_0: u8 = 24;
        // C s_66_1: cast zx s_66_0 -> bv
        let s_66_1: Bits = Bits::new(s_66_0 as u128, 8u16);
        // C s_66_2: cast zx s_66_1 -> i
        let s_66_2: i128 = (s_66_1.value() as i128);
        // C s_66_3: cast reint s_66_2 -> i64
        let s_66_3: i64 = (s_66_2 as i64);
        // C s_66_4: cast zx s_66_3 -> i
        let s_66_4: i128 = (i128::try_from(s_66_3).unwrap());
        // C s_66_5: const #432u : u32
        let s_66_5: u32 = 432;
        // D s_66_6: read-reg s_66_5:u8
        let s_66_6: u8 = {
            let value = state.read_register::<u8>(s_66_5 as isize);
            tracer.read_register(s_66_5 as isize, value);
            value
        };
        // D s_66_7: call AArch64_SystemAccessTrap(s_66_6, s_66_4)
        let s_66_7: () = AArch64_SystemAccessTrap(state, tracer, s_66_6, s_66_4);
        // N s_66_8: return
        return;
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #72u : u32
        let s_67_0: u32 = 72;
        // S s_67_1: call ConstrainUnpredictableProcedure(s_67_0)
        let s_67_1: () = ConstrainUnpredictableProcedure(state, tracer, s_67_0);
        // N s_67_2: return
        return;
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #19136u : u32
        let s_68_0: u32 = 19136;
        // D s_68_1: read-reg s_68_0:struct
        let s_68_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_68_0 as isize);
            tracer.read_register(s_68_0 as isize, value);
            value
        };
        // D s_68_2: call _get_PMSELR_EL0_Type_SEL(s_68_1)
        let s_68_2: u8 = u_get_PMSELR_EL0_Type_SEL(state, tracer, s_68_1);
        // D s_68_3: cast zx s_68_2 -> bv
        let s_68_3: Bits = Bits::new(s_68_2 as u128, 5u16);
        // D s_68_4: cast zx s_68_3 -> i
        let s_68_4: i128 = (s_68_3.value() as i128);
        // D s_68_5: cast reint s_68_4 -> i64
        let s_68_5: i64 = (s_68_4 as i64);
        // C s_68_6: const #() : ()
        let s_68_6: () = ();
        // S s_68_7: call AArch64_GetNumEventCountersAccessible(s_68_6)
        let s_68_7: i128 = AArch64_GetNumEventCountersAccessible(state, tracer, s_68_6);
        // D s_68_8: cast zx s_68_5 -> i
        let s_68_8: i128 = (i128::try_from(s_68_5).unwrap());
        // D s_68_9: cmp-ge s_68_8 s_68_7
        let s_68_9: bool = ((s_68_8) >= (s_68_7));
        // D s_68_10: write-var gs#68813 <= s_68_9
        fn_state.gs_68813 = s_68_9;
        // N s_68_11: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #24u : u8
        let s_69_0: u8 = 24;
        // C s_69_1: cast zx s_69_0 -> bv
        let s_69_1: Bits = Bits::new(s_69_0 as u128, 8u16);
        // C s_69_2: cast zx s_69_1 -> i
        let s_69_2: i128 = (s_69_1.value() as i128);
        // C s_69_3: cast reint s_69_2 -> i64
        let s_69_3: i64 = (s_69_2 as i64);
        // C s_69_4: cast zx s_69_3 -> i
        let s_69_4: i128 = (i128::try_from(s_69_3).unwrap());
        // C s_69_5: const #432u : u32
        let s_69_5: u32 = 432;
        // D s_69_6: read-reg s_69_5:u8
        let s_69_6: u8 = {
            let value = state.read_register::<u8>(s_69_5 as isize);
            tracer.read_register(s_69_5 as isize, value);
            value
        };
        // D s_69_7: call AArch64_SystemAccessTrap(s_69_6, s_69_4)
        let s_69_7: () = AArch64_SystemAccessTrap(state, tracer, s_69_6, s_69_4);
        // N s_69_8: return
        return;
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_70_0: read-var __MDCR_EL2_TPM:u8
        let s_70_0: bool = fn_state.u__MDCR_EL2_TPM;
        // D s_70_1: cast zx s_70_0 -> bv
        let s_70_1: Bits = Bits::new(s_70_0 as u128, 1u16);
        // C s_70_2: const #1u : u8
        let s_70_2: bool = true;
        // C s_70_3: cast zx s_70_2 -> bv
        let s_70_3: Bits = Bits::new(s_70_2 as u128, 1u16);
        // D s_70_4: cmp-eq s_70_1 s_70_3
        let s_70_4: bool = ((s_70_1) == (s_70_3));
        // D s_70_5: write-var gs#68812 <= s_70_4
        fn_state.gs_68812 = s_70_4;
        // N s_70_6: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #24u : u8
        let s_71_0: u8 = 24;
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
        // D s_71_7: call AArch64_SystemAccessTrap(s_71_6, s_71_4)
        let s_71_7: () = AArch64_SystemAccessTrap(state, tracer, s_71_6, s_71_4);
        // N s_71_8: return
        return;
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_72_0: read-var __HDFGRTR_EL2_PMEVCNTRn_EL0:u8
        let s_72_0: bool = fn_state.u__HDFGRTR_EL2_PMEVCNTRn_EL0;
        // D s_72_1: cast zx s_72_0 -> bv
        let s_72_1: Bits = Bits::new(s_72_0 as u128, 1u16);
        // C s_72_2: const #1u : u8
        let s_72_2: bool = true;
        // C s_72_3: cast zx s_72_2 -> bv
        let s_72_3: Bits = Bits::new(s_72_2 as u128, 1u16);
        // D s_72_4: cmp-eq s_72_1 s_72_3
        let s_72_4: bool = ((s_72_1) == (s_72_3));
        // D s_72_5: write-var gs#68811 <= s_72_4
        fn_state.gs_68811 = s_72_4;
        // N s_72_6: jump b47
        return block_47(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #424u : u32
        let s_73_0: u32 = 424;
        // D s_73_1: read-reg s_73_0:u8
        let s_73_1: u8 = {
            let value = state.read_register::<u8>(s_73_0 as isize);
            tracer.read_register(s_73_0 as isize, value);
            value
        };
        // C s_73_2: const #2u : u8
        let s_73_2: u8 = 2;
        // D s_73_3: cmp-lt s_73_1 s_73_2
        let s_73_3: bool = ((s_73_1) < (s_73_2));
        // D s_73_4: not s_73_3
        let s_73_4: bool = !s_73_3;
        // N s_73_5: branch s_73_4 b76 b74
        if s_73_4 {
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
        // D s_74_0: read-var __SCR_EL3_FGTEn:u8
        let s_74_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_74_1: cast zx s_74_0 -> bv
        let s_74_1: Bits = Bits::new(s_74_0 as u128, 1u16);
        // C s_74_2: const #1u : u8
        let s_74_2: bool = true;
        // C s_74_3: cast zx s_74_2 -> bv
        let s_74_3: Bits = Bits::new(s_74_2 as u128, 1u16);
        // D s_74_4: cmp-eq s_74_1 s_74_3
        let s_74_4: bool = ((s_74_1) == (s_74_3));
        // D s_74_5: write-var gs#68809 <= s_74_4
        fn_state.gs_68809 = s_74_4;
        // N s_74_6: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_75_0: read-var gs#68809:u8
        let s_75_0: bool = fn_state.gs_68809;
        // D s_75_1: write-var gs#68810 <= s_75_0
        fn_state.gs_68810 = s_75_0;
        // N s_75_2: jump b45
        return block_45(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_76_0: const #1u : u8
        let s_76_0: bool = true;
        // D s_76_1: write-var gs#68809 <= s_76_0
        fn_state.gs_68809 = s_76_0;
        // N s_76_2: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #146u : u32
        let s_77_0: u32 = 146;
        // S s_77_1: call IsFeatureImplemented(s_77_0)
        let s_77_1: bool = IsFeatureImplemented(state, tracer, s_77_0);
        // D s_77_2: write-var gs#68808 <= s_77_1
        fn_state.gs_68808 = s_77_1;
        // N s_77_3: jump b43
        return block_43(state, tracer, fn_state);
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
        // D s_79_0: read-var __MDCR_EL3_TPM:u8
        let s_79_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_79_1: cast zx s_79_0 -> bv
        let s_79_1: Bits = Bits::new(s_79_0 as u128, 1u16);
        // C s_79_2: const #1u : u8
        let s_79_2: bool = true;
        // C s_79_3: cast zx s_79_2 -> bv
        let s_79_3: Bits = Bits::new(s_79_2 as u128, 1u16);
        // D s_79_4: cmp-eq s_79_1 s_79_3
        let s_79_4: bool = ((s_79_1) == (s_79_3));
        // D s_79_5: write-var gs#68807 <= s_79_4
        fn_state.gs_68807 = s_79_4;
        // N s_79_6: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_80_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_80_1: call __IMPDEF_boolean(s_80_0)
        let s_80_1: bool = u__IMPDEF_boolean(state, tracer, s_80_0);
        // D s_80_2: write-var gs#68806 <= s_80_1
        fn_state.gs_68806 = s_80_1;
        // N s_80_3: jump b38
        return block_38(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_81_0: read-var __EDSCR_SDD:u8
        let s_81_0: bool = fn_state.u__EDSCR_SDD;
        // D s_81_1: cast zx s_81_0 -> bv
        let s_81_1: Bits = Bits::new(s_81_0 as u128, 1u16);
        // C s_81_2: const #1u : u8
        let s_81_2: bool = true;
        // C s_81_3: cast zx s_81_2 -> bv
        let s_81_3: Bits = Bits::new(s_81_2 as u128, 1u16);
        // D s_81_4: cmp-eq s_81_1 s_81_3
        let s_81_4: bool = ((s_81_1) == (s_81_3));
        // D s_81_5: write-var gs#68805 <= s_81_4
        fn_state.gs_68805 = s_81_4;
        // N s_81_6: jump b36
        return block_36(state, tracer, fn_state);
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
        // C s_82_2: const #2u : u8
        let s_82_2: u8 = 2;
        // D s_82_3: cmp-lt s_82_1 s_82_2
        let s_82_3: bool = ((s_82_1) < (s_82_2));
        // D s_82_4: write-var gs#68804 <= s_82_3
        fn_state.gs_68804 = s_82_3;
        // N s_82_5: jump b34
        return block_34(state, tracer, fn_state);
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
        // N s_83_2: branch s_83_1 b152 b84
        if s_83_1 {
            return block_152(state, tracer, fn_state);
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
        // D s_84_1: write-var gs#68817 <= s_84_0
        fn_state.gs_68817 = s_84_0;
        // N s_84_2: jump b85
        return block_85(state, tracer, fn_state);
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_85_0: read-var gs#68817:u8
        let s_85_0: bool = fn_state.gs_68817;
        // N s_85_1: branch s_85_0 b151 b86
        if s_85_0 {
            return block_151(state, tracer, fn_state);
        } else {
            return block_86(state, tracer, fn_state);
        };
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_86_0: const #0u : u8
        let s_86_0: bool = false;
        // D s_86_1: write-var gs#68818 <= s_86_0
        fn_state.gs_68818 = s_86_0;
        // N s_86_2: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_87_0: read-var gs#68818:u8
        let s_87_0: bool = fn_state.gs_68818;
        // N s_87_1: branch s_87_0 b150 b88
        if s_87_0 {
            return block_150(state, tracer, fn_state);
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
        // D s_88_1: write-var gs#68819 <= s_88_0
        fn_state.gs_68819 = s_88_0;
        // N s_88_2: jump b89
        return block_89(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_89_0: read-var gs#68819:u8
        let s_89_0: bool = fn_state.gs_68819;
        // N s_89_1: branch s_89_0 b149 b90
        if s_89_0 {
            return block_149(state, tracer, fn_state);
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
        // D s_90_1: write-var gs#68820 <= s_90_0
        fn_state.gs_68820 = s_90_0;
        // N s_90_2: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_91_0: read-var gs#68820:u8
        let s_91_0: bool = fn_state.gs_68820;
        // N s_91_1: branch s_91_0 b148 b92
        if s_91_0 {
            return block_148(state, tracer, fn_state);
        } else {
            return block_92(state, tracer, fn_state);
        };
    }
    fn block_92<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_92_0: const #204u : u32
        let s_92_0: u32 = 204;
        // S s_92_1: call IsFeatureImplemented(s_92_0)
        let s_92_1: bool = IsFeatureImplemented(state, tracer, s_92_0);
        // N s_92_2: branch s_92_1 b147 b93
        if s_92_1 {
            return block_147(state, tracer, fn_state);
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
        // D s_93_1: write-var gs#68821 <= s_93_0
        fn_state.gs_68821 = s_93_0;
        // N s_93_2: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_94_0: read-var gs#68821:u8
        let s_94_0: bool = fn_state.gs_68821;
        // N s_94_1: branch s_94_0 b146 b95
        if s_94_0 {
            return block_146(state, tracer, fn_state);
        } else {
            return block_95(state, tracer, fn_state);
        };
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_95_0: const #204u : u32
        let s_95_0: u32 = 204;
        // S s_95_1: call IsFeatureImplemented(s_95_0)
        let s_95_1: bool = IsFeatureImplemented(state, tracer, s_95_0);
        // S s_95_2: not s_95_1
        let s_95_2: bool = !s_95_1;
        // N s_95_3: branch s_95_2 b145 b96
        if s_95_2 {
            return block_145(state, tracer, fn_state);
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
        // D s_96_1: write-var gs#68822 <= s_96_0
        fn_state.gs_68822 = s_96_0;
        // N s_96_2: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_97_0: read-var gs#68822:u8
        let s_97_0: bool = fn_state.gs_68822;
        // D s_97_1: write-var gs#68823 <= s_97_0
        fn_state.gs_68823 = s_97_0;
        // N s_97_2: jump b98
        return block_98(state, tracer, fn_state);
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_98_0: read-var gs#68823:u8
        let s_98_0: bool = fn_state.gs_68823;
        // N s_98_1: branch s_98_0 b139 b99
        if s_98_0 {
            return block_139(state, tracer, fn_state);
        } else {
            return block_99(state, tracer, fn_state);
        };
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_99_0: const #() : ()
        let s_99_0: () = ();
        // S s_99_1: call EL2Enabled(s_99_0)
        let s_99_1: bool = EL2Enabled(state, tracer, s_99_0);
        // N s_99_2: branch s_99_1 b138 b100
        if s_99_1 {
            return block_138(state, tracer, fn_state);
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
        // D s_100_1: write-var gs#68824 <= s_100_0
        fn_state.gs_68824 = s_100_0;
        // N s_100_2: jump b101
        return block_101(state, tracer, fn_state);
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_101_0: read-var gs#68824:u8
        let s_101_0: bool = fn_state.gs_68824;
        // N s_101_1: branch s_101_0 b137 b102
        if s_101_0 {
            return block_137(state, tracer, fn_state);
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
        // D s_102_1: write-var gs#68825 <= s_102_0
        fn_state.gs_68825 = s_102_0;
        // N s_102_2: jump b103
        return block_103(state, tracer, fn_state);
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_103_0: read-var gs#68825:u8
        let s_103_0: bool = fn_state.gs_68825;
        // N s_103_1: branch s_103_0 b133 b104
        if s_103_0 {
            return block_133(state, tracer, fn_state);
        } else {
            return block_104(state, tracer, fn_state);
        };
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_104_0: const #0u : u8
        let s_104_0: bool = false;
        // D s_104_1: write-var gs#68827 <= s_104_0
        fn_state.gs_68827 = s_104_0;
        // N s_104_2: jump b105
        return block_105(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_105_0: read-var gs#68827:u8
        let s_105_0: bool = fn_state.gs_68827;
        // N s_105_1: branch s_105_0 b132 b106
        if s_105_0 {
            return block_132(state, tracer, fn_state);
        } else {
            return block_106(state, tracer, fn_state);
        };
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_106_0: const #0u : u8
        let s_106_0: bool = false;
        // D s_106_1: write-var gs#68828 <= s_106_0
        fn_state.gs_68828 = s_106_0;
        // N s_106_2: jump b107
        return block_107(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_107_0: read-var gs#68828:u8
        let s_107_0: bool = fn_state.gs_68828;
        // N s_107_1: branch s_107_0 b131 b108
        if s_107_0 {
            return block_131(state, tracer, fn_state);
        } else {
            return block_108(state, tracer, fn_state);
        };
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_108_0: const #() : ()
        let s_108_0: () = ();
        // S s_108_1: call EL2Enabled(s_108_0)
        let s_108_1: bool = EL2Enabled(state, tracer, s_108_0);
        // N s_108_2: branch s_108_1 b130 b109
        if s_108_1 {
            return block_130(state, tracer, fn_state);
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
        // D s_109_1: write-var gs#68829 <= s_109_0
        fn_state.gs_68829 = s_109_0;
        // N s_109_2: jump b110
        return block_110(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_110_0: read-var gs#68829:u8
        let s_110_0: bool = fn_state.gs_68829;
        // N s_110_1: branch s_110_0 b129 b111
        if s_110_0 {
            return block_129(state, tracer, fn_state);
        } else {
            return block_111(state, tracer, fn_state);
        };
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
        // N s_111_2: branch s_111_1 b128 b112
        if s_111_1 {
            return block_128(state, tracer, fn_state);
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
        // D s_112_1: write-var gs#68830 <= s_112_0
        fn_state.gs_68830 = s_112_0;
        // N s_112_2: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_113_0: read-var gs#68830:u8
        let s_113_0: bool = fn_state.gs_68830;
        // N s_113_1: branch s_113_0 b125 b114
        if s_113_0 {
            return block_125(state, tracer, fn_state);
        } else {
            return block_114(state, tracer, fn_state);
        };
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_114_0: const #424u : u32
        let s_114_0: u32 = 424;
        // D s_114_1: read-reg s_114_0:u8
        let s_114_1: u8 = {
            let value = state.read_register::<u8>(s_114_0 as isize);
            tracer.read_register(s_114_0 as isize, value);
            value
        };
        // C s_114_2: const #2u : u8
        let s_114_2: u8 = 2;
        // D s_114_3: cmp-lt s_114_1 s_114_2
        let s_114_3: bool = ((s_114_1) < (s_114_2));
        // N s_114_4: branch s_114_3 b124 b115
        if s_114_3 {
            return block_124(state, tracer, fn_state);
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
        // D s_115_1: write-var gs#68831 <= s_115_0
        fn_state.gs_68831 = s_115_0;
        // N s_115_2: jump b116
        return block_116(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_116_0: read-var gs#68831:u8
        let s_116_0: bool = fn_state.gs_68831;
        // N s_116_1: branch s_116_0 b118 b117
        if s_116_0 {
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
        // C s_117_1: const #19136u : u32
        let s_117_1: u32 = 19136;
        // D s_117_2: read-reg s_117_1:struct
        let s_117_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_117_1 as isize);
            tracer.read_register(s_117_1 as isize, value);
            value
        };
        // D s_117_3: call _get_PMSELR_EL0_Type_SEL(s_117_2)
        let s_117_3: u8 = u_get_PMSELR_EL0_Type_SEL(state, tracer, s_117_2);
        // D s_117_4: cast zx s_117_3 -> bv
        let s_117_4: Bits = Bits::new(s_117_3 as u128, 5u16);
        // D s_117_5: cast zx s_117_4 -> i
        let s_117_5: i128 = (s_117_4.value() as i128);
        // D s_117_6: cast reint s_117_5 -> i64
        let s_117_6: i64 = (s_117_5 as i64);
        // C s_117_7: const #10744u : u32
        let s_117_7: u32 = 10744;
        // D s_117_8: read-reg s_117_7:[u64; 32]
        let s_117_8: [u64; 32usize] = {
            let value = state.read_register::<[u64; 32usize]>(s_117_7 as isize);
            tracer.read_register(s_117_7 as isize, value);
            value
        };
        // D s_117_9: cast zx s_117_6 -> i
        let s_117_9: i128 = (i128::try_from(s_117_6).unwrap());
        // D s_117_10: read-element s_117_8[s_117_9]
        let s_117_10: u64 = s_117_8[(s_117_9) as usize];
        // D s_117_11: cast zx s_117_10 -> bv
        let s_117_11: Bits = Bits::new(s_117_10 as u128, 64u16);
        // D s_117_12: read-var t:i
        let s_117_12: i128 = fn_state.t;
        // D s_117_13: call X_set(s_117_12, s_117_0, s_117_11)
        let s_117_13: () = X_set(state, tracer, s_117_12, s_117_0, s_117_11);
        // N s_117_14: return
        return;
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_118_0: const #() : ()
        let s_118_0: () = ();
        // S s_118_1: call Halted(s_118_0)
        let s_118_1: bool = Halted(state, tracer, s_118_0);
        // N s_118_2: branch s_118_1 b123 b119
        if s_118_1 {
            return block_123(state, tracer, fn_state);
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
        // D s_119_1: write-var gs#68833 <= s_119_0
        fn_state.gs_68833 = s_119_0;
        // N s_119_2: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_120_0: read-var gs#68833:u8
        let s_120_0: bool = fn_state.gs_68833;
        // N s_120_1: branch s_120_0 b122 b121
        if s_120_0 {
            return block_122(state, tracer, fn_state);
        } else {
            return block_121(state, tracer, fn_state);
        };
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_121_0: const #24u : u8
        let s_121_0: u8 = 24;
        // C s_121_1: cast zx s_121_0 -> bv
        let s_121_1: Bits = Bits::new(s_121_0 as u128, 8u16);
        // C s_121_2: cast zx s_121_1 -> i
        let s_121_2: i128 = (s_121_1.value() as i128);
        // C s_121_3: cast reint s_121_2 -> i64
        let s_121_3: i64 = (s_121_2 as i64);
        // C s_121_4: cast zx s_121_3 -> i
        let s_121_4: i128 = (i128::try_from(s_121_3).unwrap());
        // C s_121_5: const #424u : u32
        let s_121_5: u32 = 424;
        // D s_121_6: read-reg s_121_5:u8
        let s_121_6: u8 = {
            let value = state.read_register::<u8>(s_121_5 as isize);
            tracer.read_register(s_121_5 as isize, value);
            value
        };
        // D s_121_7: call AArch64_SystemAccessTrap(s_121_6, s_121_4)
        let s_121_7: () = AArch64_SystemAccessTrap(state, tracer, s_121_6, s_121_4);
        // N s_121_8: return
        return;
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_122_0: panic
        panic!("{:?}", ());
        // N s_122_1: return
        return;
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_123_0: read-var __EDSCR_SDD:u8
        let s_123_0: bool = fn_state.u__EDSCR_SDD;
        // D s_123_1: cast zx s_123_0 -> bv
        let s_123_1: Bits = Bits::new(s_123_0 as u128, 1u16);
        // C s_123_2: const #1u : u8
        let s_123_2: bool = true;
        // C s_123_3: cast zx s_123_2 -> bv
        let s_123_3: Bits = Bits::new(s_123_2 as u128, 1u16);
        // D s_123_4: cmp-eq s_123_1 s_123_3
        let s_123_4: bool = ((s_123_1) == (s_123_3));
        // D s_123_5: write-var gs#68833 <= s_123_4
        fn_state.gs_68833 = s_123_4;
        // N s_123_6: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_124_0: read-var __MDCR_EL3_TPM:u8
        let s_124_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_124_1: cast zx s_124_0 -> bv
        let s_124_1: Bits = Bits::new(s_124_0 as u128, 1u16);
        // C s_124_2: const #1u : u8
        let s_124_2: bool = true;
        // C s_124_3: cast zx s_124_2 -> bv
        let s_124_3: Bits = Bits::new(s_124_2 as u128, 1u16);
        // D s_124_4: cmp-eq s_124_1 s_124_3
        let s_124_4: bool = ((s_124_1) == (s_124_3));
        // D s_124_5: write-var gs#68831 <= s_124_4
        fn_state.gs_68831 = s_124_4;
        // N s_124_6: jump b116
        return block_116(state, tracer, fn_state);
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_125_0: const #146u : u32
        let s_125_0: u32 = 146;
        // S s_125_1: call IsFeatureImplemented(s_125_0)
        let s_125_1: bool = IsFeatureImplemented(state, tracer, s_125_0);
        // S s_125_2: not s_125_1
        let s_125_2: bool = !s_125_1;
        // N s_125_3: branch s_125_2 b127 b126
        if s_125_2 {
            return block_127(state, tracer, fn_state);
        } else {
            return block_126(state, tracer, fn_state);
        };
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_126_0: const #24u : u8
        let s_126_0: u8 = 24;
        // C s_126_1: cast zx s_126_0 -> bv
        let s_126_1: Bits = Bits::new(s_126_0 as u128, 8u16);
        // C s_126_2: cast zx s_126_1 -> i
        let s_126_2: i128 = (s_126_1.value() as i128);
        // C s_126_3: cast reint s_126_2 -> i64
        let s_126_3: i64 = (s_126_2 as i64);
        // C s_126_4: cast zx s_126_3 -> i
        let s_126_4: i128 = (i128::try_from(s_126_3).unwrap());
        // C s_126_5: const #432u : u32
        let s_126_5: u32 = 432;
        // D s_126_6: read-reg s_126_5:u8
        let s_126_6: u8 = {
            let value = state.read_register::<u8>(s_126_5 as isize);
            tracer.read_register(s_126_5 as isize, value);
            value
        };
        // D s_126_7: call AArch64_SystemAccessTrap(s_126_6, s_126_4)
        let s_126_7: () = AArch64_SystemAccessTrap(state, tracer, s_126_6, s_126_4);
        // N s_126_8: return
        return;
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_127_0: const #72u : u32
        let s_127_0: u32 = 72;
        // S s_127_1: call ConstrainUnpredictableProcedure(s_127_0)
        let s_127_1: () = ConstrainUnpredictableProcedure(state, tracer, s_127_0);
        // N s_127_2: return
        return;
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_128_0: const #19136u : u32
        let s_128_0: u32 = 19136;
        // D s_128_1: read-reg s_128_0:struct
        let s_128_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_128_0 as isize);
            tracer.read_register(s_128_0 as isize, value);
            value
        };
        // D s_128_2: call _get_PMSELR_EL0_Type_SEL(s_128_1)
        let s_128_2: u8 = u_get_PMSELR_EL0_Type_SEL(state, tracer, s_128_1);
        // D s_128_3: cast zx s_128_2 -> bv
        let s_128_3: Bits = Bits::new(s_128_2 as u128, 5u16);
        // D s_128_4: cast zx s_128_3 -> i
        let s_128_4: i128 = (s_128_3.value() as i128);
        // D s_128_5: cast reint s_128_4 -> i64
        let s_128_5: i64 = (s_128_4 as i64);
        // C s_128_6: const #() : ()
        let s_128_6: () = ();
        // S s_128_7: call AArch64_GetNumEventCountersAccessible(s_128_6)
        let s_128_7: i128 = AArch64_GetNumEventCountersAccessible(
            state,
            tracer,
            s_128_6,
        );
        // D s_128_8: cast zx s_128_5 -> i
        let s_128_8: i128 = (i128::try_from(s_128_5).unwrap());
        // D s_128_9: cmp-ge s_128_8 s_128_7
        let s_128_9: bool = ((s_128_8) >= (s_128_7));
        // D s_128_10: write-var gs#68830 <= s_128_9
        fn_state.gs_68830 = s_128_9;
        // N s_128_11: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_129_0: const #24u : u8
        let s_129_0: u8 = 24;
        // C s_129_1: cast zx s_129_0 -> bv
        let s_129_1: Bits = Bits::new(s_129_0 as u128, 8u16);
        // C s_129_2: cast zx s_129_1 -> i
        let s_129_2: i128 = (s_129_1.value() as i128);
        // C s_129_3: cast reint s_129_2 -> i64
        let s_129_3: i64 = (s_129_2 as i64);
        // C s_129_4: cast zx s_129_3 -> i
        let s_129_4: i128 = (i128::try_from(s_129_3).unwrap());
        // C s_129_5: const #432u : u32
        let s_129_5: u32 = 432;
        // D s_129_6: read-reg s_129_5:u8
        let s_129_6: u8 = {
            let value = state.read_register::<u8>(s_129_5 as isize);
            tracer.read_register(s_129_5 as isize, value);
            value
        };
        // D s_129_7: call AArch64_SystemAccessTrap(s_129_6, s_129_4)
        let s_129_7: () = AArch64_SystemAccessTrap(state, tracer, s_129_6, s_129_4);
        // N s_129_8: return
        return;
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_130_0: read-var __MDCR_EL2_TPM:u8
        let s_130_0: bool = fn_state.u__MDCR_EL2_TPM;
        // D s_130_1: cast zx s_130_0 -> bv
        let s_130_1: Bits = Bits::new(s_130_0 as u128, 1u16);
        // C s_130_2: const #1u : u8
        let s_130_2: bool = true;
        // C s_130_3: cast zx s_130_2 -> bv
        let s_130_3: Bits = Bits::new(s_130_2 as u128, 1u16);
        // D s_130_4: cmp-eq s_130_1 s_130_3
        let s_130_4: bool = ((s_130_1) == (s_130_3));
        // D s_130_5: write-var gs#68829 <= s_130_4
        fn_state.gs_68829 = s_130_4;
        // N s_130_6: jump b110
        return block_110(state, tracer, fn_state);
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_131_0: const #24u : u8
        let s_131_0: u8 = 24;
        // C s_131_1: cast zx s_131_0 -> bv
        let s_131_1: Bits = Bits::new(s_131_0 as u128, 8u16);
        // C s_131_2: cast zx s_131_1 -> i
        let s_131_2: i128 = (s_131_1.value() as i128);
        // C s_131_3: cast reint s_131_2 -> i64
        let s_131_3: i64 = (s_131_2 as i64);
        // C s_131_4: cast zx s_131_3 -> i
        let s_131_4: i128 = (i128::try_from(s_131_3).unwrap());
        // C s_131_5: const #432u : u32
        let s_131_5: u32 = 432;
        // D s_131_6: read-reg s_131_5:u8
        let s_131_6: u8 = {
            let value = state.read_register::<u8>(s_131_5 as isize);
            tracer.read_register(s_131_5 as isize, value);
            value
        };
        // D s_131_7: call AArch64_SystemAccessTrap(s_131_6, s_131_4)
        let s_131_7: () = AArch64_SystemAccessTrap(state, tracer, s_131_6, s_131_4);
        // N s_131_8: return
        return;
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_132_0: read-var __HDFGRTR_EL2_PMEVCNTRn_EL0:u8
        let s_132_0: bool = fn_state.u__HDFGRTR_EL2_PMEVCNTRn_EL0;
        // D s_132_1: cast zx s_132_0 -> bv
        let s_132_1: Bits = Bits::new(s_132_0 as u128, 1u16);
        // C s_132_2: const #1u : u8
        let s_132_2: bool = true;
        // C s_132_3: cast zx s_132_2 -> bv
        let s_132_3: Bits = Bits::new(s_132_2 as u128, 1u16);
        // D s_132_4: cmp-eq s_132_1 s_132_3
        let s_132_4: bool = ((s_132_1) == (s_132_3));
        // D s_132_5: write-var gs#68828 <= s_132_4
        fn_state.gs_68828 = s_132_4;
        // N s_132_6: jump b107
        return block_107(state, tracer, fn_state);
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_133_0: const #424u : u32
        let s_133_0: u32 = 424;
        // D s_133_1: read-reg s_133_0:u8
        let s_133_1: u8 = {
            let value = state.read_register::<u8>(s_133_0 as isize);
            tracer.read_register(s_133_0 as isize, value);
            value
        };
        // C s_133_2: const #2u : u8
        let s_133_2: u8 = 2;
        // D s_133_3: cmp-lt s_133_1 s_133_2
        let s_133_3: bool = ((s_133_1) < (s_133_2));
        // D s_133_4: not s_133_3
        let s_133_4: bool = !s_133_3;
        // N s_133_5: branch s_133_4 b136 b134
        if s_133_4 {
            return block_136(state, tracer, fn_state);
        } else {
            return block_134(state, tracer, fn_state);
        };
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_134_0: read-var __SCR_EL3_FGTEn:u8
        let s_134_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_134_1: cast zx s_134_0 -> bv
        let s_134_1: Bits = Bits::new(s_134_0 as u128, 1u16);
        // C s_134_2: const #1u : u8
        let s_134_2: bool = true;
        // C s_134_3: cast zx s_134_2 -> bv
        let s_134_3: Bits = Bits::new(s_134_2 as u128, 1u16);
        // D s_134_4: cmp-eq s_134_1 s_134_3
        let s_134_4: bool = ((s_134_1) == (s_134_3));
        // D s_134_5: write-var gs#68826 <= s_134_4
        fn_state.gs_68826 = s_134_4;
        // N s_134_6: jump b135
        return block_135(state, tracer, fn_state);
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_135_0: read-var gs#68826:u8
        let s_135_0: bool = fn_state.gs_68826;
        // D s_135_1: write-var gs#68827 <= s_135_0
        fn_state.gs_68827 = s_135_0;
        // N s_135_2: jump b105
        return block_105(state, tracer, fn_state);
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_136_0: const #1u : u8
        let s_136_0: bool = true;
        // D s_136_1: write-var gs#68826 <= s_136_0
        fn_state.gs_68826 = s_136_0;
        // N s_136_2: jump b135
        return block_135(state, tracer, fn_state);
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_137_0: const #146u : u32
        let s_137_0: u32 = 146;
        // S s_137_1: call IsFeatureImplemented(s_137_0)
        let s_137_1: bool = IsFeatureImplemented(state, tracer, s_137_0);
        // D s_137_2: write-var gs#68825 <= s_137_1
        fn_state.gs_68825 = s_137_1;
        // N s_137_3: jump b103
        return block_103(state, tracer, fn_state);
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
        // D s_138_20: cmp-ne s_138_17 s_138_19
        let s_138_20: bool = ((s_138_17) != (s_138_19));
        // D s_138_21: write-var gs#68824 <= s_138_20
        fn_state.gs_68824 = s_138_20;
        // N s_138_22: jump b101
        return block_101(state, tracer, fn_state);
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_139_0: const #() : ()
        let s_139_0: () = ();
        // S s_139_1: call EL2Enabled(s_139_0)
        let s_139_1: bool = EL2Enabled(state, tracer, s_139_0);
        // N s_139_2: branch s_139_1 b144 b140
        if s_139_1 {
            return block_144(state, tracer, fn_state);
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
        // D s_140_1: write-var gs#68834 <= s_140_0
        fn_state.gs_68834 = s_140_0;
        // N s_140_2: jump b141
        return block_141(state, tracer, fn_state);
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_141_0: read-var gs#68834:u8
        let s_141_0: bool = fn_state.gs_68834;
        // N s_141_1: branch s_141_0 b143 b142
        if s_141_0 {
            return block_143(state, tracer, fn_state);
        } else {
            return block_142(state, tracer, fn_state);
        };
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_142_0: const #24u : u8
        let s_142_0: u8 = 24;
        // C s_142_1: cast zx s_142_0 -> bv
        let s_142_1: Bits = Bits::new(s_142_0 as u128, 8u16);
        // C s_142_2: cast zx s_142_1 -> i
        let s_142_2: i128 = (s_142_1.value() as i128);
        // C s_142_3: cast reint s_142_2 -> i64
        let s_142_3: i64 = (s_142_2 as i64);
        // C s_142_4: cast zx s_142_3 -> i
        let s_142_4: i128 = (i128::try_from(s_142_3).unwrap());
        // C s_142_5: const #440u : u32
        let s_142_5: u32 = 440;
        // D s_142_6: read-reg s_142_5:u8
        let s_142_6: u8 = {
            let value = state.read_register::<u8>(s_142_5 as isize);
            tracer.read_register(s_142_5 as isize, value);
            value
        };
        // D s_142_7: call AArch64_SystemAccessTrap(s_142_6, s_142_4)
        let s_142_7: () = AArch64_SystemAccessTrap(state, tracer, s_142_6, s_142_4);
        // N s_142_8: return
        return;
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_143_0: const #24u : u8
        let s_143_0: u8 = 24;
        // C s_143_1: cast zx s_143_0 -> bv
        let s_143_1: Bits = Bits::new(s_143_0 as u128, 8u16);
        // C s_143_2: cast zx s_143_1 -> i
        let s_143_2: i128 = (s_143_1.value() as i128);
        // C s_143_3: cast reint s_143_2 -> i64
        let s_143_3: i64 = (s_143_2 as i64);
        // C s_143_4: cast zx s_143_3 -> i
        let s_143_4: i128 = (i128::try_from(s_143_3).unwrap());
        // C s_143_5: const #432u : u32
        let s_143_5: u32 = 432;
        // D s_143_6: read-reg s_143_5:u8
        let s_143_6: u8 = {
            let value = state.read_register::<u8>(s_143_5 as isize);
            tracer.read_register(s_143_5 as isize, value);
            value
        };
        // D s_143_7: call AArch64_SystemAccessTrap(s_143_6, s_143_4)
        let s_143_7: () = AArch64_SystemAccessTrap(state, tracer, s_143_6, s_143_4);
        // N s_143_8: return
        return;
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_144_0: read-var __HCR_EL2_TGE:u8
        let s_144_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_144_1: cast zx s_144_0 -> bv
        let s_144_1: Bits = Bits::new(s_144_0 as u128, 1u16);
        // C s_144_2: const #1u : u8
        let s_144_2: bool = true;
        // C s_144_3: cast zx s_144_2 -> bv
        let s_144_3: Bits = Bits::new(s_144_2 as u128, 1u16);
        // D s_144_4: cmp-eq s_144_1 s_144_3
        let s_144_4: bool = ((s_144_1) == (s_144_3));
        // D s_144_5: write-var gs#68834 <= s_144_4
        fn_state.gs_68834 = s_144_4;
        // N s_144_6: jump b141
        return block_141(state, tracer, fn_state);
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_145_0: const #102624u : u32
        let s_145_0: u32 = 102624;
        // D s_145_1: read-reg s_145_0:struct
        let s_145_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_145_0 as isize);
            tracer.read_register(s_145_0 as isize, value);
            value
        };
        // D s_145_2: call _get_PMUSERENR_EL0_Type_ER(s_145_1)
        let s_145_2: bool = u_get_PMUSERENR_EL0_Type_ER(state, tracer, s_145_1);
        // C s_145_3: const #102624u : u32
        let s_145_3: u32 = 102624;
        // D s_145_4: read-reg s_145_3:struct
        let s_145_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_145_3 as isize);
            tracer.read_register(s_145_3 as isize, value);
            value
        };
        // D s_145_5: call _get_PMUSERENR_EL0_Type_EN(s_145_4)
        let s_145_5: bool = u_get_PMUSERENR_EL0_Type_EN(state, tracer, s_145_4);
        // D s_145_6: cast zx s_145_2 -> bv
        let s_145_6: Bits = Bits::new(s_145_2 as u128, 1u16);
        // D s_145_7: cast zx s_145_5 -> bv
        let s_145_7: Bits = Bits::new(s_145_5 as u128, 1u16);
        // D s_145_8: cast reint s_145_6 -> u128
        let s_145_8: u128 = (s_145_6.value() as u128);
        // D s_145_9: size-of s_145_6
        let s_145_9: u16 = s_145_6.length();
        // D s_145_10: cast reint s_145_7 -> u128
        let s_145_10: u128 = (s_145_7.value() as u128);
        // D s_145_11: size-of s_145_7
        let s_145_11: u16 = s_145_7.length();
        // D s_145_12: lsl s_145_8 s_145_11
        let s_145_12: u128 = s_145_8 << s_145_11;
        // D s_145_13: or s_145_12 s_145_10
        let s_145_13: u128 = ((s_145_12) | (s_145_10));
        // D s_145_14: add s_145_9 s_145_11
        let s_145_14: u16 = (s_145_9 + s_145_11);
        // D s_145_15: create-bits s_145_13 s_145_14
        let s_145_15: Bits = Bits::new(s_145_13, s_145_14);
        // D s_145_16: cast reint s_145_15 -> u8
        let s_145_16: u8 = (s_145_15.value() as u8);
        // D s_145_17: cast zx s_145_16 -> bv
        let s_145_17: Bits = Bits::new(s_145_16 as u128, 2u16);
        // C s_145_18: const #0u : u8
        let s_145_18: u8 = 0;
        // C s_145_19: cast zx s_145_18 -> bv
        let s_145_19: Bits = Bits::new(s_145_18 as u128, 2u16);
        // D s_145_20: cmp-eq s_145_17 s_145_19
        let s_145_20: bool = ((s_145_17) == (s_145_19));
        // D s_145_21: write-var gs#68822 <= s_145_20
        fn_state.gs_68822 = s_145_20;
        // N s_145_22: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_146_0: const #1u : u8
        let s_146_0: bool = true;
        // D s_146_1: write-var gs#68823 <= s_146_0
        fn_state.gs_68823 = s_146_0;
        // N s_146_2: jump b98
        return block_98(state, tracer, fn_state);
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_147_0: const #102624u : u32
        let s_147_0: u32 = 102624;
        // D s_147_1: read-reg s_147_0:struct
        let s_147_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_147_0 as isize);
            tracer.read_register(s_147_0 as isize, value);
            value
        };
        // D s_147_2: call _get_PMUSERENR_EL0_Type_UEN(s_147_1)
        let s_147_2: bool = u_get_PMUSERENR_EL0_Type_UEN(state, tracer, s_147_1);
        // C s_147_3: const #102624u : u32
        let s_147_3: u32 = 102624;
        // D s_147_4: read-reg s_147_3:struct
        let s_147_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_147_3 as isize);
            tracer.read_register(s_147_3 as isize, value);
            value
        };
        // D s_147_5: call _get_PMUSERENR_EL0_Type_ER(s_147_4)
        let s_147_5: bool = u_get_PMUSERENR_EL0_Type_ER(state, tracer, s_147_4);
        // C s_147_6: const #102624u : u32
        let s_147_6: u32 = 102624;
        // D s_147_7: read-reg s_147_6:struct
        let s_147_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_147_6 as isize);
            tracer.read_register(s_147_6 as isize, value);
            value
        };
        // D s_147_8: call _get_PMUSERENR_EL0_Type_EN(s_147_7)
        let s_147_8: bool = u_get_PMUSERENR_EL0_Type_EN(state, tracer, s_147_7);
        // D s_147_9: cast zx s_147_5 -> bv
        let s_147_9: Bits = Bits::new(s_147_5 as u128, 1u16);
        // D s_147_10: cast zx s_147_8 -> bv
        let s_147_10: Bits = Bits::new(s_147_8 as u128, 1u16);
        // D s_147_11: cast reint s_147_9 -> u128
        let s_147_11: u128 = (s_147_9.value() as u128);
        // D s_147_12: size-of s_147_9
        let s_147_12: u16 = s_147_9.length();
        // D s_147_13: cast reint s_147_10 -> u128
        let s_147_13: u128 = (s_147_10.value() as u128);
        // D s_147_14: size-of s_147_10
        let s_147_14: u16 = s_147_10.length();
        // D s_147_15: lsl s_147_11 s_147_14
        let s_147_15: u128 = s_147_11 << s_147_14;
        // D s_147_16: or s_147_15 s_147_13
        let s_147_16: u128 = ((s_147_15) | (s_147_13));
        // D s_147_17: add s_147_12 s_147_14
        let s_147_17: u16 = (s_147_12 + s_147_14);
        // D s_147_18: create-bits s_147_16 s_147_17
        let s_147_18: Bits = Bits::new(s_147_16, s_147_17);
        // D s_147_19: cast reint s_147_18 -> u8
        let s_147_19: u8 = (s_147_18.value() as u8);
        // D s_147_20: cast zx s_147_2 -> bv
        let s_147_20: Bits = Bits::new(s_147_2 as u128, 1u16);
        // D s_147_21: cast zx s_147_19 -> bv
        let s_147_21: Bits = Bits::new(s_147_19 as u128, 2u16);
        // D s_147_22: cast reint s_147_20 -> u128
        let s_147_22: u128 = (s_147_20.value() as u128);
        // D s_147_23: size-of s_147_20
        let s_147_23: u16 = s_147_20.length();
        // D s_147_24: cast reint s_147_21 -> u128
        let s_147_24: u128 = (s_147_21.value() as u128);
        // D s_147_25: size-of s_147_21
        let s_147_25: u16 = s_147_21.length();
        // D s_147_26: lsl s_147_22 s_147_25
        let s_147_26: u128 = s_147_22 << s_147_25;
        // D s_147_27: or s_147_26 s_147_24
        let s_147_27: u128 = ((s_147_26) | (s_147_24));
        // D s_147_28: add s_147_23 s_147_25
        let s_147_28: u16 = (s_147_23 + s_147_25);
        // D s_147_29: create-bits s_147_27 s_147_28
        let s_147_29: Bits = Bits::new(s_147_27, s_147_28);
        // D s_147_30: cast reint s_147_29 -> u8
        let s_147_30: u8 = (s_147_29.value() as u8);
        // D s_147_31: cast zx s_147_30 -> bv
        let s_147_31: Bits = Bits::new(s_147_30 as u128, 3u16);
        // C s_147_32: const #0u : u8
        let s_147_32: u8 = 0;
        // C s_147_33: cast zx s_147_32 -> bv
        let s_147_33: Bits = Bits::new(s_147_32 as u128, 3u16);
        // D s_147_34: cmp-eq s_147_31 s_147_33
        let s_147_34: bool = ((s_147_31) == (s_147_33));
        // D s_147_35: write-var gs#68821 <= s_147_34
        fn_state.gs_68821 = s_147_34;
        // N s_147_36: jump b94
        return block_94(state, tracer, fn_state);
    }
    fn block_148<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_148_0: panic
        panic!("{:?}", ());
        // N s_148_1: return
        return;
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_149_0: read-var __MDCR_EL3_TPM:u8
        let s_149_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_149_1: cast zx s_149_0 -> bv
        let s_149_1: Bits = Bits::new(s_149_0 as u128, 1u16);
        // C s_149_2: const #1u : u8
        let s_149_2: bool = true;
        // C s_149_3: cast zx s_149_2 -> bv
        let s_149_3: Bits = Bits::new(s_149_2 as u128, 1u16);
        // D s_149_4: cmp-eq s_149_1 s_149_3
        let s_149_4: bool = ((s_149_1) == (s_149_3));
        // D s_149_5: write-var gs#68820 <= s_149_4
        fn_state.gs_68820 = s_149_4;
        // N s_149_6: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_150_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_150_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_150_1: call __IMPDEF_boolean(s_150_0)
        let s_150_1: bool = u__IMPDEF_boolean(state, tracer, s_150_0);
        // D s_150_2: write-var gs#68819 <= s_150_1
        fn_state.gs_68819 = s_150_1;
        // N s_150_3: jump b89
        return block_89(state, tracer, fn_state);
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_151_0: read-var __EDSCR_SDD:u8
        let s_151_0: bool = fn_state.u__EDSCR_SDD;
        // D s_151_1: cast zx s_151_0 -> bv
        let s_151_1: Bits = Bits::new(s_151_0 as u128, 1u16);
        // C s_151_2: const #1u : u8
        let s_151_2: bool = true;
        // C s_151_3: cast zx s_151_2 -> bv
        let s_151_3: Bits = Bits::new(s_151_2 as u128, 1u16);
        // D s_151_4: cmp-eq s_151_1 s_151_3
        let s_151_4: bool = ((s_151_1) == (s_151_3));
        // D s_151_5: write-var gs#68818 <= s_151_4
        fn_state.gs_68818 = s_151_4;
        // N s_151_6: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_152_0: const #424u : u32
        let s_152_0: u32 = 424;
        // D s_152_1: read-reg s_152_0:u8
        let s_152_1: u8 = {
            let value = state.read_register::<u8>(s_152_0 as isize);
            tracer.read_register(s_152_0 as isize, value);
            value
        };
        // C s_152_2: const #2u : u8
        let s_152_2: u8 = 2;
        // D s_152_3: cmp-lt s_152_1 s_152_2
        let s_152_3: bool = ((s_152_1) < (s_152_2));
        // D s_152_4: write-var gs#68817 <= s_152_3
        fn_state.gs_68817 = s_152_3;
        // N s_152_5: jump b85
        return block_85(state, tracer, fn_state);
    }
    fn block_153<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_153_0: const #146u : u32
        let s_153_0: u32 = 146;
        // S s_153_1: call IsFeatureImplemented(s_153_0)
        let s_153_1: bool = IsFeatureImplemented(state, tracer, s_153_0);
        // N s_153_2: branch s_153_1 b155 b154
        if s_153_1 {
            return block_155(state, tracer, fn_state);
        } else {
            return block_154(state, tracer, fn_state);
        };
    }
    fn block_154<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_154_0: const #72u : u32
        let s_154_0: u32 = 72;
        // S s_154_1: call ConstrainUnpredictableProcedure(s_154_0)
        let s_154_1: () = ConstrainUnpredictableProcedure(state, tracer, s_154_0);
        // N s_154_2: return
        return;
    }
    fn block_155<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_155_0: panic
        panic!("{:?}", ());
        // N s_155_1: return
        return;
    }
}
