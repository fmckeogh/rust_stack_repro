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
use u_get_PMUSERENR_EL0_Type_UEN::*;
use u_get_MDCR_EL2_Type_TPM::*;
use EDSCR_read::*;
use u_get_HCR_EL2_Type_E2H::*;
use Halted::*;
use IsFeatureImplemented::*;
use AArch64_SystemAccessTrap::*;
use u__IMPDEF_boolean::*;
use u_get_MDCR_EL3_Type_TPM::*;
use X_set::*;
use u_get_SCR_EL3_Type_FGTEn2::*;
use u_get_EDSCR_Type_SDD::*;
use EL2Enabled::*;
use u_get_HDFGRTR2_EL2_Type_nPMICNTR_EL0::*;
use u_get_HCR_EL2_Type_TGE::*;
use u_get_MDCR_EL3_Type_EnPM2::*;
use common::*;
pub fn PMICNTR_EL0_SysRegRead_468ebcab1dcb0580<T: Tracer>(
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
        gs_67988: bool,
        gs_67949: bool,
        gs_67977: bool,
        gs_67943: bool,
        gs_67946: bool,
        u__MDCR_EL3_EnPM2: bool,
        gs_67979: bool,
        gs_67978: bool,
        u__PMUSERENR_EL0_UEN: bool,
        gs_67966: bool,
        gs_67956: bool,
        gs_67991: bool,
        u__MDCR_EL3_TPM: bool,
        gs_67993: bool,
        gs_67942: bool,
        gs_67960: bool,
        gs_67984: bool,
        u__SCR_EL3_FGTEn2: bool,
        gs_67981: bool,
        gs_67982: bool,
        gs_67980: bool,
        gs_67970: bool,
        gs_67950: bool,
        gs_67994: bool,
        gs_67972: bool,
        u__PSTATE_EL: u8,
        gs_67983: bool,
        gs_67962: bool,
        gs_67965: bool,
        u__HDFGRTR2_EL2_nPMICNTR_EL0: bool,
        u__MDCR_EL2_TPM: bool,
        gs_67947: bool,
        gs_67945: bool,
        u__HCR_EL2_TGE: bool,
        u__EDSCR_SDD: bool,
        gs_67995: bool,
        gs_67963: bool,
        gs_67968: bool,
        gs_67974: bool,
        gs_67969: bool,
        gs_67959: bool,
        gs_67958: bool,
        gs_67987: bool,
        gs_67967: bool,
        gs_67986: bool,
        gs_67954: bool,
        gs_67973: bool,
        gs_67985: bool,
        gs_67990: bool,
        gs_67989: bool,
        gs_67948: bool,
        gs_67951: bool,
        gs_67953: bool,
        gs_67955: bool,
        gs_67957: bool,
        gs_67961: bool,
        gs_67964: bool,
        gs_67944: bool,
        gs_67976: bool,
        gs_67975: bool,
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
        // D s_0_9: call _get_MDCR_EL3_Type_EnPM2(s_0_8)
        let s_0_9: bool = u_get_MDCR_EL3_Type_EnPM2(state, tracer, s_0_8);
        // D s_0_10: write-var __MDCR_EL3_EnPM2 <= s_0_9
        fn_state.u__MDCR_EL3_EnPM2 = s_0_9;
        // C s_0_11: const #22712u : u32
        let s_0_11: u32 = 22712;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_MDCR_EL3_Type_TPM(s_0_12)
        let s_0_13: bool = u_get_MDCR_EL3_Type_TPM(state, tracer, s_0_12);
        // D s_0_14: write-var __MDCR_EL3_TPM <= s_0_13
        fn_state.u__MDCR_EL3_TPM = s_0_13;
        // C s_0_15: const #102624u : u32
        let s_0_15: u32 = 102624;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_PMUSERENR_EL0_Type_UEN(s_0_16)
        let s_0_17: bool = u_get_PMUSERENR_EL0_Type_UEN(state, tracer, s_0_16);
        // D s_0_18: write-var __PMUSERENR_EL0_UEN <= s_0_17
        fn_state.u__PMUSERENR_EL0_UEN = s_0_17;
        // C s_0_19: const #102552u : u32
        let s_0_19: u32 = 102552;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_HCR_EL2_Type_TGE(s_0_20)
        let s_0_21: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_0_20);
        // D s_0_22: write-var __HCR_EL2_TGE <= s_0_21
        fn_state.u__HCR_EL2_TGE = s_0_21;
        // C s_0_23: const #90704u : u32
        let s_0_23: u32 = 90704;
        // D s_0_24: read-reg s_0_23:struct
        let s_0_24: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_23 as isize);
            tracer.read_register(s_0_23 as isize, value);
            value
        };
        // D s_0_25: call _get_SCR_EL3_Type_FGTEn2(s_0_24)
        let s_0_25: bool = u_get_SCR_EL3_Type_FGTEn2(state, tracer, s_0_24);
        // D s_0_26: write-var __SCR_EL3_FGTEn2 <= s_0_25
        fn_state.u__SCR_EL3_FGTEn2 = s_0_25;
        // C s_0_27: const #101224u : u32
        let s_0_27: u32 = 101224;
        // D s_0_28: read-reg s_0_27:struct
        let s_0_28: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_27 as isize);
            tracer.read_register(s_0_27 as isize, value);
            value
        };
        // D s_0_29: call _get_HDFGRTR2_EL2_Type_nPMICNTR_EL0(s_0_28)
        let s_0_29: bool = u_get_HDFGRTR2_EL2_Type_nPMICNTR_EL0(state, tracer, s_0_28);
        // D s_0_30: write-var __HDFGRTR2_EL2_nPMICNTR_EL0 <= s_0_29
        fn_state.u__HDFGRTR2_EL2_nPMICNTR_EL0 = s_0_29;
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
        // D s_0_35: read-var __PSTATE_EL:u8
        let s_0_35: u8 = fn_state.u__PSTATE_EL;
        // D s_0_36: cast zx s_0_35 -> bv
        let s_0_36: Bits = Bits::new(s_0_35 as u128, 2u16);
        // C s_0_37: const #448u : u32
        let s_0_37: u32 = 448;
        // D s_0_38: read-reg s_0_37:u8
        let s_0_38: u8 = {
            let value = state.read_register::<u8>(s_0_37 as isize);
            tracer.read_register(s_0_37 as isize, value);
            value
        };
        // D s_0_39: cast zx s_0_38 -> bv
        let s_0_39: Bits = Bits::new(s_0_38 as u128, 2u16);
        // D s_0_40: cmp-eq s_0_36 s_0_39
        let s_0_40: bool = ((s_0_36) == (s_0_39));
        // N s_0_41: branch s_0_40 b128 b1
        if s_0_40 {
            return block_128(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b55 b2
        if s_1_5 {
            return block_55(state, tracer, fn_state);
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
        // C s_5_1: const #12944u : u32
        let s_5_1: u32 = 12944;
        // D s_5_2: read-reg s_5_1:u64
        let s_5_2: u64 = {
            let value = state.read_register::<u64>(s_5_1 as isize);
            tracer.read_register(s_5_1 as isize, value);
            value
        };
        // D s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 64u16);
        // D s_5_4: read-var t:i
        let s_5_4: i128 = fn_state.t;
        // D s_5_5: call X_set(s_5_4, s_5_0, s_5_3)
        let s_5_5: () = X_set(state, tracer, s_5_4, s_5_0, s_5_3);
        // N s_5_6: return
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
        // N s_6_2: branch s_6_1 b54 b7
        if s_6_1 {
            return block_54(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#67942 <= s_7_0
        fn_state.gs_67942 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#67942:u8
        let s_8_0: bool = fn_state.gs_67942;
        // N s_8_1: branch s_8_0 b53 b9
        if s_8_0 {
            return block_53(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#67943 <= s_9_0
        fn_state.gs_67943 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#67943:u8
        let s_10_0: bool = fn_state.gs_67943;
        // N s_10_1: branch s_10_0 b52 b11
        if s_10_0 {
            return block_52(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#67944 <= s_11_0
        fn_state.gs_67944 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#67944:u8
        let s_12_0: bool = fn_state.gs_67944;
        // N s_12_1: branch s_12_0 b51 b13
        if s_12_0 {
            return block_51(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#67945 <= s_13_0
        fn_state.gs_67945 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#67945:u8
        let s_14_0: bool = fn_state.gs_67945;
        // N s_14_1: branch s_14_0 b50 b15
        if s_14_0 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #() : ()
        let s_15_0: () = ();
        // S s_15_1: call Halted(s_15_0)
        let s_15_1: bool = Halted(state, tracer, s_15_0);
        // N s_15_2: branch s_15_1 b49 b16
        if s_15_1 {
            return block_49(state, tracer, fn_state);
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
        // D s_16_1: write-var gs#67946 <= s_16_0
        fn_state.gs_67946 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#67946:u8
        let s_17_0: bool = fn_state.gs_67946;
        // N s_17_1: branch s_17_0 b48 b18
        if s_17_0 {
            return block_48(state, tracer, fn_state);
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
        // D s_18_1: write-var gs#67947 <= s_18_0
        fn_state.gs_67947 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#67947:u8
        let s_19_0: bool = fn_state.gs_67947;
        // N s_19_1: branch s_19_0 b47 b20
        if s_19_0 {
            return block_47(state, tracer, fn_state);
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
        // D s_20_1: write-var gs#67948 <= s_20_0
        fn_state.gs_67948 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#67948:u8
        let s_21_0: bool = fn_state.gs_67948;
        // N s_21_1: branch s_21_0 b46 b22
        if s_21_0 {
            return block_46(state, tracer, fn_state);
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
        // D s_22_1: write-var gs#67949 <= s_22_0
        fn_state.gs_67949 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#67949:u8
        let s_23_0: bool = fn_state.gs_67949;
        // N s_23_1: branch s_23_0 b45 b24
        if s_23_0 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #424u : u32
        let s_24_0: u32 = 424;
        // D s_24_1: read-reg s_24_0:u8
        let s_24_1: u8 = {
            let value = state.read_register::<u8>(s_24_0 as isize);
            tracer.read_register(s_24_0 as isize, value);
            value
        };
        // C s_24_2: const #2u : u8
        let s_24_2: u8 = 2;
        // D s_24_3: cmp-lt s_24_1 s_24_2
        let s_24_3: bool = ((s_24_1) < (s_24_2));
        // N s_24_4: branch s_24_3 b44 b25
        if s_24_3 {
            return block_44(state, tracer, fn_state);
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
        // D s_25_1: write-var gs#67950 <= s_25_0
        fn_state.gs_67950 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#67950:u8
        let s_26_0: bool = fn_state.gs_67950;
        // N s_26_1: branch s_26_0 b38 b27
        if s_26_0 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #424u : u32
        let s_27_0: u32 = 424;
        // D s_27_1: read-reg s_27_0:u8
        let s_27_1: u8 = {
            let value = state.read_register::<u8>(s_27_0 as isize);
            tracer.read_register(s_27_0 as isize, value);
            value
        };
        // C s_27_2: const #2u : u8
        let s_27_2: u8 = 2;
        // D s_27_3: cmp-lt s_27_1 s_27_2
        let s_27_3: bool = ((s_27_1) < (s_27_2));
        // N s_27_4: branch s_27_3 b37 b28
        if s_27_3 {
            return block_37(state, tracer, fn_state);
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
        // D s_28_1: write-var gs#67951 <= s_28_0
        fn_state.gs_67951 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#67951:u8
        let s_29_0: bool = fn_state.gs_67951;
        // N s_29_1: branch s_29_0 b31 b30
        if s_29_0 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #64s : i64
        let s_30_0: i64 = 64;
        // C s_30_1: const #12944u : u32
        let s_30_1: u32 = 12944;
        // D s_30_2: read-reg s_30_1:u64
        let s_30_2: u64 = {
            let value = state.read_register::<u64>(s_30_1 as isize);
            tracer.read_register(s_30_1 as isize, value);
            value
        };
        // D s_30_3: cast zx s_30_2 -> bv
        let s_30_3: Bits = Bits::new(s_30_2 as u128, 64u16);
        // D s_30_4: read-var t:i
        let s_30_4: i128 = fn_state.t;
        // D s_30_5: call X_set(s_30_4, s_30_0, s_30_3)
        let s_30_5: () = X_set(state, tracer, s_30_4, s_30_0, s_30_3);
        // N s_30_6: return
        return;
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #() : ()
        let s_31_0: () = ();
        // S s_31_1: call Halted(s_31_0)
        let s_31_1: bool = Halted(state, tracer, s_31_0);
        // N s_31_2: branch s_31_1 b36 b32
        if s_31_1 {
            return block_36(state, tracer, fn_state);
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
        // D s_32_1: write-var gs#67953 <= s_32_0
        fn_state.gs_67953 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#67953:u8
        let s_33_0: bool = fn_state.gs_67953;
        // N s_33_1: branch s_33_0 b35 b34
        if s_33_0 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #24u : u8
        let s_34_0: u8 = 24;
        // C s_34_1: cast zx s_34_0 -> bv
        let s_34_1: Bits = Bits::new(s_34_0 as u128, 8u16);
        // C s_34_2: cast zx s_34_1 -> i
        let s_34_2: i128 = (s_34_1.value() as i128);
        // C s_34_3: cast reint s_34_2 -> i64
        let s_34_3: i64 = (s_34_2 as i64);
        // C s_34_4: cast zx s_34_3 -> i
        let s_34_4: i128 = (i128::try_from(s_34_3).unwrap());
        // C s_34_5: const #424u : u32
        let s_34_5: u32 = 424;
        // D s_34_6: read-reg s_34_5:u8
        let s_34_6: u8 = {
            let value = state.read_register::<u8>(s_34_5 as isize);
            tracer.read_register(s_34_5 as isize, value);
            value
        };
        // D s_34_7: call AArch64_SystemAccessTrap(s_34_6, s_34_4)
        let s_34_7: () = AArch64_SystemAccessTrap(state, tracer, s_34_6, s_34_4);
        // N s_34_8: return
        return;
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_35_0: panic
        panic!("{:?}", ());
        // N s_35_1: return
        return;
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var __EDSCR_SDD:u8
        let s_36_0: bool = fn_state.u__EDSCR_SDD;
        // D s_36_1: cast zx s_36_0 -> bv
        let s_36_1: Bits = Bits::new(s_36_0 as u128, 1u16);
        // C s_36_2: const #1u : u8
        let s_36_2: bool = true;
        // C s_36_3: cast zx s_36_2 -> bv
        let s_36_3: Bits = Bits::new(s_36_2 as u128, 1u16);
        // D s_36_4: cmp-eq s_36_1 s_36_3
        let s_36_4: bool = ((s_36_1) == (s_36_3));
        // D s_36_5: write-var gs#67953 <= s_36_4
        fn_state.gs_67953 = s_36_4;
        // N s_36_6: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var __MDCR_EL3_TPM:u8
        let s_37_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_37_1: cast zx s_37_0 -> bv
        let s_37_1: Bits = Bits::new(s_37_0 as u128, 1u16);
        // C s_37_2: const #1u : u8
        let s_37_2: bool = true;
        // C s_37_3: cast zx s_37_2 -> bv
        let s_37_3: Bits = Bits::new(s_37_2 as u128, 1u16);
        // D s_37_4: cmp-eq s_37_1 s_37_3
        let s_37_4: bool = ((s_37_1) == (s_37_3));
        // D s_37_5: write-var gs#67951 <= s_37_4
        fn_state.gs_67951 = s_37_4;
        // N s_37_6: jump b29
        return block_29(state, tracer, fn_state);
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
        // N s_38_2: branch s_38_1 b43 b39
        if s_38_1 {
            return block_43(state, tracer, fn_state);
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
        // D s_39_1: write-var gs#67954 <= s_39_0
        fn_state.gs_67954 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#67954:u8
        let s_40_0: bool = fn_state.gs_67954;
        // N s_40_1: branch s_40_0 b42 b41
        if s_40_0 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_41_0: const #24u : u8
        let s_41_0: u8 = 24;
        // C s_41_1: cast zx s_41_0 -> bv
        let s_41_1: Bits = Bits::new(s_41_0 as u128, 8u16);
        // C s_41_2: cast zx s_41_1 -> i
        let s_41_2: i128 = (s_41_1.value() as i128);
        // C s_41_3: cast reint s_41_2 -> i64
        let s_41_3: i64 = (s_41_2 as i64);
        // C s_41_4: cast zx s_41_3 -> i
        let s_41_4: i128 = (i128::try_from(s_41_3).unwrap());
        // C s_41_5: const #424u : u32
        let s_41_5: u32 = 424;
        // D s_41_6: read-reg s_41_5:u8
        let s_41_6: u8 = {
            let value = state.read_register::<u8>(s_41_5 as isize);
            tracer.read_register(s_41_5 as isize, value);
            value
        };
        // D s_41_7: call AArch64_SystemAccessTrap(s_41_6, s_41_4)
        let s_41_7: () = AArch64_SystemAccessTrap(state, tracer, s_41_6, s_41_4);
        // N s_41_8: return
        return;
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_42_0: panic
        panic!("{:?}", ());
        // N s_42_1: return
        return;
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_43_0: read-var __EDSCR_SDD:u8
        let s_43_0: bool = fn_state.u__EDSCR_SDD;
        // D s_43_1: cast zx s_43_0 -> bv
        let s_43_1: Bits = Bits::new(s_43_0 as u128, 1u16);
        // C s_43_2: const #1u : u8
        let s_43_2: bool = true;
        // C s_43_3: cast zx s_43_2 -> bv
        let s_43_3: Bits = Bits::new(s_43_2 as u128, 1u16);
        // D s_43_4: cmp-eq s_43_1 s_43_3
        let s_43_4: bool = ((s_43_1) == (s_43_3));
        // D s_43_5: write-var gs#67954 <= s_43_4
        fn_state.gs_67954 = s_43_4;
        // N s_43_6: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var __MDCR_EL3_EnPM2:u8
        let s_44_0: bool = fn_state.u__MDCR_EL3_EnPM2;
        // D s_44_1: cast zx s_44_0 -> bv
        let s_44_1: Bits = Bits::new(s_44_0 as u128, 1u16);
        // C s_44_2: const #0u : u8
        let s_44_2: bool = false;
        // C s_44_3: cast zx s_44_2 -> bv
        let s_44_3: Bits = Bits::new(s_44_2 as u128, 1u16);
        // D s_44_4: cmp-eq s_44_1 s_44_3
        let s_44_4: bool = ((s_44_1) == (s_44_3));
        // D s_44_5: write-var gs#67950 <= s_44_4
        fn_state.gs_67950 = s_44_4;
        // N s_44_6: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_45_0: panic
        panic!("{:?}", ());
        // N s_45_1: return
        return;
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var __MDCR_EL3_TPM:u8
        let s_46_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_46_1: cast zx s_46_0 -> bv
        let s_46_1: Bits = Bits::new(s_46_0 as u128, 1u16);
        // C s_46_2: const #1u : u8
        let s_46_2: bool = true;
        // C s_46_3: cast zx s_46_2 -> bv
        let s_46_3: Bits = Bits::new(s_46_2 as u128, 1u16);
        // D s_46_4: cmp-eq s_46_1 s_46_3
        let s_46_4: bool = ((s_46_1) == (s_46_3));
        // D s_46_5: write-var gs#67949 <= s_46_4
        fn_state.gs_67949 = s_46_4;
        // N s_46_6: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_47_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_47_1: call __IMPDEF_boolean(s_47_0)
        let s_47_1: bool = u__IMPDEF_boolean(state, tracer, s_47_0);
        // D s_47_2: write-var gs#67948 <= s_47_1
        fn_state.gs_67948 = s_47_1;
        // N s_47_3: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var __EDSCR_SDD:u8
        let s_48_0: bool = fn_state.u__EDSCR_SDD;
        // D s_48_1: cast zx s_48_0 -> bv
        let s_48_1: Bits = Bits::new(s_48_0 as u128, 1u16);
        // C s_48_2: const #1u : u8
        let s_48_2: bool = true;
        // C s_48_3: cast zx s_48_2 -> bv
        let s_48_3: Bits = Bits::new(s_48_2 as u128, 1u16);
        // D s_48_4: cmp-eq s_48_1 s_48_3
        let s_48_4: bool = ((s_48_1) == (s_48_3));
        // D s_48_5: write-var gs#67947 <= s_48_4
        fn_state.gs_67947 = s_48_4;
        // N s_48_6: jump b19
        return block_19(state, tracer, fn_state);
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
        // D s_49_4: write-var gs#67946 <= s_49_3
        fn_state.gs_67946 = s_49_3;
        // N s_49_5: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_50_0: panic
        panic!("{:?}", ());
        // N s_50_1: return
        return;
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var __MDCR_EL3_EnPM2:u8
        let s_51_0: bool = fn_state.u__MDCR_EL3_EnPM2;
        // D s_51_1: cast zx s_51_0 -> bv
        let s_51_1: Bits = Bits::new(s_51_0 as u128, 1u16);
        // C s_51_2: const #0u : u8
        let s_51_2: bool = false;
        // C s_51_3: cast zx s_51_2 -> bv
        let s_51_3: Bits = Bits::new(s_51_2 as u128, 1u16);
        // D s_51_4: cmp-eq s_51_1 s_51_3
        let s_51_4: bool = ((s_51_1) == (s_51_3));
        // D s_51_5: write-var gs#67945 <= s_51_4
        fn_state.gs_67945 = s_51_4;
        // N s_51_6: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_52_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_52_1: call __IMPDEF_boolean(s_52_0)
        let s_52_1: bool = u__IMPDEF_boolean(state, tracer, s_52_0);
        // D s_52_2: write-var gs#67944 <= s_52_1
        fn_state.gs_67944 = s_52_1;
        // N s_52_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var __EDSCR_SDD:u8
        let s_53_0: bool = fn_state.u__EDSCR_SDD;
        // D s_53_1: cast zx s_53_0 -> bv
        let s_53_1: Bits = Bits::new(s_53_0 as u128, 1u16);
        // C s_53_2: const #1u : u8
        let s_53_2: bool = true;
        // C s_53_3: cast zx s_53_2 -> bv
        let s_53_3: Bits = Bits::new(s_53_2 as u128, 1u16);
        // D s_53_4: cmp-eq s_53_1 s_53_3
        let s_53_4: bool = ((s_53_1) == (s_53_3));
        // D s_53_5: write-var gs#67943 <= s_53_4
        fn_state.gs_67943 = s_53_4;
        // N s_53_6: jump b10
        return block_10(state, tracer, fn_state);
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
        // D s_54_4: write-var gs#67942 <= s_54_3
        fn_state.gs_67942 = s_54_3;
        // N s_54_5: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #() : ()
        let s_55_0: () = ();
        // S s_55_1: call Halted(s_55_0)
        let s_55_1: bool = Halted(state, tracer, s_55_0);
        // N s_55_2: branch s_55_1 b127 b56
        if s_55_1 {
            return block_127(state, tracer, fn_state);
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
        // D s_56_1: write-var gs#67955 <= s_56_0
        fn_state.gs_67955 = s_56_0;
        // N s_56_2: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var gs#67955:u8
        let s_57_0: bool = fn_state.gs_67955;
        // N s_57_1: branch s_57_0 b126 b58
        if s_57_0 {
            return block_126(state, tracer, fn_state);
        } else {
            return block_58(state, tracer, fn_state);
        };
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #0u : u8
        let s_58_0: bool = false;
        // D s_58_1: write-var gs#67956 <= s_58_0
        fn_state.gs_67956 = s_58_0;
        // N s_58_2: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_59_0: read-var gs#67956:u8
        let s_59_0: bool = fn_state.gs_67956;
        // N s_59_1: branch s_59_0 b125 b60
        if s_59_0 {
            return block_125(state, tracer, fn_state);
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
        // D s_60_1: write-var gs#67957 <= s_60_0
        fn_state.gs_67957 = s_60_0;
        // N s_60_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_61_0: read-var gs#67957:u8
        let s_61_0: bool = fn_state.gs_67957;
        // N s_61_1: branch s_61_0 b124 b62
        if s_61_0 {
            return block_124(state, tracer, fn_state);
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
        // D s_62_1: write-var gs#67958 <= s_62_0
        fn_state.gs_67958 = s_62_0;
        // N s_62_2: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_63_0: read-var gs#67958:u8
        let s_63_0: bool = fn_state.gs_67958;
        // N s_63_1: branch s_63_0 b123 b64
        if s_63_0 {
            return block_123(state, tracer, fn_state);
        } else {
            return block_64(state, tracer, fn_state);
        };
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #() : ()
        let s_64_0: () = ();
        // S s_64_1: call Halted(s_64_0)
        let s_64_1: bool = Halted(state, tracer, s_64_0);
        // N s_64_2: branch s_64_1 b122 b65
        if s_64_1 {
            return block_122(state, tracer, fn_state);
        } else {
            return block_65(state, tracer, fn_state);
        };
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #0u : u8
        let s_65_0: bool = false;
        // D s_65_1: write-var gs#67959 <= s_65_0
        fn_state.gs_67959 = s_65_0;
        // N s_65_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_66_0: read-var gs#67959:u8
        let s_66_0: bool = fn_state.gs_67959;
        // N s_66_1: branch s_66_0 b121 b67
        if s_66_0 {
            return block_121(state, tracer, fn_state);
        } else {
            return block_67(state, tracer, fn_state);
        };
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #0u : u8
        let s_67_0: bool = false;
        // D s_67_1: write-var gs#67960 <= s_67_0
        fn_state.gs_67960 = s_67_0;
        // N s_67_2: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_68_0: read-var gs#67960:u8
        let s_68_0: bool = fn_state.gs_67960;
        // N s_68_1: branch s_68_0 b120 b69
        if s_68_0 {
            return block_120(state, tracer, fn_state);
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
        // D s_69_1: write-var gs#67961 <= s_69_0
        fn_state.gs_67961 = s_69_0;
        // N s_69_2: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_70_0: read-var gs#67961:u8
        let s_70_0: bool = fn_state.gs_67961;
        // N s_70_1: branch s_70_0 b119 b71
        if s_70_0 {
            return block_119(state, tracer, fn_state);
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
        // D s_71_1: write-var gs#67962 <= s_71_0
        fn_state.gs_67962 = s_71_0;
        // N s_71_2: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_72_0: read-var gs#67962:u8
        let s_72_0: bool = fn_state.gs_67962;
        // N s_72_1: branch s_72_0 b118 b73
        if s_72_0 {
            return block_118(state, tracer, fn_state);
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
        // N s_73_2: branch s_73_1 b117 b74
        if s_73_1 {
            return block_117(state, tracer, fn_state);
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
        // D s_74_1: write-var gs#67963 <= s_74_0
        fn_state.gs_67963 = s_74_0;
        // N s_74_2: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_75_0: read-var gs#67963:u8
        let s_75_0: bool = fn_state.gs_67963;
        // N s_75_1: branch s_75_0 b116 b76
        if s_75_0 {
            return block_116(state, tracer, fn_state);
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
        // D s_76_1: write-var gs#67964 <= s_76_0
        fn_state.gs_67964 = s_76_0;
        // N s_76_2: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_77_0: read-var gs#67964:u8
        let s_77_0: bool = fn_state.gs_67964;
        // N s_77_1: branch s_77_0 b115 b78
        if s_77_0 {
            return block_115(state, tracer, fn_state);
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
        // D s_78_1: write-var gs#67965 <= s_78_0
        fn_state.gs_67965 = s_78_0;
        // N s_78_2: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_79_0: read-var gs#67965:u8
        let s_79_0: bool = fn_state.gs_67965;
        // N s_79_1: branch s_79_0 b114 b80
        if s_79_0 {
            return block_114(state, tracer, fn_state);
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
        // N s_80_2: branch s_80_1 b113 b81
        if s_80_1 {
            return block_113(state, tracer, fn_state);
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
        // D s_81_1: write-var gs#67966 <= s_81_0
        fn_state.gs_67966 = s_81_0;
        // N s_81_2: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_82_0: read-var gs#67966:u8
        let s_82_0: bool = fn_state.gs_67966;
        // N s_82_1: branch s_82_0 b112 b83
        if s_82_0 {
            return block_112(state, tracer, fn_state);
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
        // D s_83_1: write-var gs#67967 <= s_83_0
        fn_state.gs_67967 = s_83_0;
        // N s_83_2: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_84_0: read-var gs#67967:u8
        let s_84_0: bool = fn_state.gs_67967;
        // N s_84_1: branch s_84_0 b111 b85
        if s_84_0 {
            return block_111(state, tracer, fn_state);
        } else {
            return block_85(state, tracer, fn_state);
        };
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_85_0: const #() : ()
        let s_85_0: () = ();
        // S s_85_1: call EL2Enabled(s_85_0)
        let s_85_1: bool = EL2Enabled(state, tracer, s_85_0);
        // N s_85_2: branch s_85_1 b110 b86
        if s_85_1 {
            return block_110(state, tracer, fn_state);
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
        // D s_86_1: write-var gs#67968 <= s_86_0
        fn_state.gs_67968 = s_86_0;
        // N s_86_2: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_87_0: read-var gs#67968:u8
        let s_87_0: bool = fn_state.gs_67968;
        // N s_87_1: branch s_87_0 b109 b88
        if s_87_0 {
            return block_109(state, tracer, fn_state);
        } else {
            return block_88(state, tracer, fn_state);
        };
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_88_0: const #424u : u32
        let s_88_0: u32 = 424;
        // D s_88_1: read-reg s_88_0:u8
        let s_88_1: u8 = {
            let value = state.read_register::<u8>(s_88_0 as isize);
            tracer.read_register(s_88_0 as isize, value);
            value
        };
        // C s_88_2: const #2u : u8
        let s_88_2: u8 = 2;
        // D s_88_3: cmp-lt s_88_1 s_88_2
        let s_88_3: bool = ((s_88_1) < (s_88_2));
        // N s_88_4: branch s_88_3 b108 b89
        if s_88_3 {
            return block_108(state, tracer, fn_state);
        } else {
            return block_89(state, tracer, fn_state);
        };
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_89_0: const #0u : u8
        let s_89_0: bool = false;
        // D s_89_1: write-var gs#67969 <= s_89_0
        fn_state.gs_67969 = s_89_0;
        // N s_89_2: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_90_0: read-var gs#67969:u8
        let s_90_0: bool = fn_state.gs_67969;
        // N s_90_1: branch s_90_0 b102 b91
        if s_90_0 {
            return block_102(state, tracer, fn_state);
        } else {
            return block_91(state, tracer, fn_state);
        };
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
        // C s_91_2: const #2u : u8
        let s_91_2: u8 = 2;
        // D s_91_3: cmp-lt s_91_1 s_91_2
        let s_91_3: bool = ((s_91_1) < (s_91_2));
        // N s_91_4: branch s_91_3 b101 b92
        if s_91_3 {
            return block_101(state, tracer, fn_state);
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
        // D s_92_1: write-var gs#67970 <= s_92_0
        fn_state.gs_67970 = s_92_0;
        // N s_92_2: jump b93
        return block_93(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_93_0: read-var gs#67970:u8
        let s_93_0: bool = fn_state.gs_67970;
        // N s_93_1: branch s_93_0 b95 b94
        if s_93_0 {
            return block_95(state, tracer, fn_state);
        } else {
            return block_94(state, tracer, fn_state);
        };
    }
    fn block_94<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_94_0: const #64s : i64
        let s_94_0: i64 = 64;
        // C s_94_1: const #12944u : u32
        let s_94_1: u32 = 12944;
        // D s_94_2: read-reg s_94_1:u64
        let s_94_2: u64 = {
            let value = state.read_register::<u64>(s_94_1 as isize);
            tracer.read_register(s_94_1 as isize, value);
            value
        };
        // D s_94_3: cast zx s_94_2 -> bv
        let s_94_3: Bits = Bits::new(s_94_2 as u128, 64u16);
        // D s_94_4: read-var t:i
        let s_94_4: i128 = fn_state.t;
        // D s_94_5: call X_set(s_94_4, s_94_0, s_94_3)
        let s_94_5: () = X_set(state, tracer, s_94_4, s_94_0, s_94_3);
        // N s_94_6: return
        return;
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_95_0: const #() : ()
        let s_95_0: () = ();
        // S s_95_1: call Halted(s_95_0)
        let s_95_1: bool = Halted(state, tracer, s_95_0);
        // N s_95_2: branch s_95_1 b100 b96
        if s_95_1 {
            return block_100(state, tracer, fn_state);
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
        // D s_96_1: write-var gs#67972 <= s_96_0
        fn_state.gs_67972 = s_96_0;
        // N s_96_2: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_97_0: read-var gs#67972:u8
        let s_97_0: bool = fn_state.gs_67972;
        // N s_97_1: branch s_97_0 b99 b98
        if s_97_0 {
            return block_99(state, tracer, fn_state);
        } else {
            return block_98(state, tracer, fn_state);
        };
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_98_0: const #24u : u8
        let s_98_0: u8 = 24;
        // C s_98_1: cast zx s_98_0 -> bv
        let s_98_1: Bits = Bits::new(s_98_0 as u128, 8u16);
        // C s_98_2: cast zx s_98_1 -> i
        let s_98_2: i128 = (s_98_1.value() as i128);
        // C s_98_3: cast reint s_98_2 -> i64
        let s_98_3: i64 = (s_98_2 as i64);
        // C s_98_4: cast zx s_98_3 -> i
        let s_98_4: i128 = (i128::try_from(s_98_3).unwrap());
        // C s_98_5: const #424u : u32
        let s_98_5: u32 = 424;
        // D s_98_6: read-reg s_98_5:u8
        let s_98_6: u8 = {
            let value = state.read_register::<u8>(s_98_5 as isize);
            tracer.read_register(s_98_5 as isize, value);
            value
        };
        // D s_98_7: call AArch64_SystemAccessTrap(s_98_6, s_98_4)
        let s_98_7: () = AArch64_SystemAccessTrap(state, tracer, s_98_6, s_98_4);
        // N s_98_8: return
        return;
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_99_0: panic
        panic!("{:?}", ());
        // N s_99_1: return
        return;
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_100_0: read-var __EDSCR_SDD:u8
        let s_100_0: bool = fn_state.u__EDSCR_SDD;
        // D s_100_1: cast zx s_100_0 -> bv
        let s_100_1: Bits = Bits::new(s_100_0 as u128, 1u16);
        // C s_100_2: const #1u : u8
        let s_100_2: bool = true;
        // C s_100_3: cast zx s_100_2 -> bv
        let s_100_3: Bits = Bits::new(s_100_2 as u128, 1u16);
        // D s_100_4: cmp-eq s_100_1 s_100_3
        let s_100_4: bool = ((s_100_1) == (s_100_3));
        // D s_100_5: write-var gs#67972 <= s_100_4
        fn_state.gs_67972 = s_100_4;
        // N s_100_6: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_101_0: read-var __MDCR_EL3_TPM:u8
        let s_101_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_101_1: cast zx s_101_0 -> bv
        let s_101_1: Bits = Bits::new(s_101_0 as u128, 1u16);
        // C s_101_2: const #1u : u8
        let s_101_2: bool = true;
        // C s_101_3: cast zx s_101_2 -> bv
        let s_101_3: Bits = Bits::new(s_101_2 as u128, 1u16);
        // D s_101_4: cmp-eq s_101_1 s_101_3
        let s_101_4: bool = ((s_101_1) == (s_101_3));
        // D s_101_5: write-var gs#67970 <= s_101_4
        fn_state.gs_67970 = s_101_4;
        // N s_101_6: jump b93
        return block_93(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_102_0: const #() : ()
        let s_102_0: () = ();
        // S s_102_1: call Halted(s_102_0)
        let s_102_1: bool = Halted(state, tracer, s_102_0);
        // N s_102_2: branch s_102_1 b107 b103
        if s_102_1 {
            return block_107(state, tracer, fn_state);
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
        // D s_103_1: write-var gs#67973 <= s_103_0
        fn_state.gs_67973 = s_103_0;
        // N s_103_2: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_104_0: read-var gs#67973:u8
        let s_104_0: bool = fn_state.gs_67973;
        // N s_104_1: branch s_104_0 b106 b105
        if s_104_0 {
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
        // C s_105_5: const #424u : u32
        let s_105_5: u32 = 424;
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
        // N s_106_0: panic
        panic!("{:?}", ());
        // N s_106_1: return
        return;
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_107_0: read-var __EDSCR_SDD:u8
        let s_107_0: bool = fn_state.u__EDSCR_SDD;
        // D s_107_1: cast zx s_107_0 -> bv
        let s_107_1: Bits = Bits::new(s_107_0 as u128, 1u16);
        // C s_107_2: const #1u : u8
        let s_107_2: bool = true;
        // C s_107_3: cast zx s_107_2 -> bv
        let s_107_3: Bits = Bits::new(s_107_2 as u128, 1u16);
        // D s_107_4: cmp-eq s_107_1 s_107_3
        let s_107_4: bool = ((s_107_1) == (s_107_3));
        // D s_107_5: write-var gs#67973 <= s_107_4
        fn_state.gs_67973 = s_107_4;
        // N s_107_6: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_108_0: read-var __MDCR_EL3_EnPM2:u8
        let s_108_0: bool = fn_state.u__MDCR_EL3_EnPM2;
        // D s_108_1: cast zx s_108_0 -> bv
        let s_108_1: Bits = Bits::new(s_108_0 as u128, 1u16);
        // C s_108_2: const #0u : u8
        let s_108_2: bool = false;
        // C s_108_3: cast zx s_108_2 -> bv
        let s_108_3: Bits = Bits::new(s_108_2 as u128, 1u16);
        // D s_108_4: cmp-eq s_108_1 s_108_3
        let s_108_4: bool = ((s_108_1) == (s_108_3));
        // D s_108_5: write-var gs#67969 <= s_108_4
        fn_state.gs_67969 = s_108_4;
        // N s_108_6: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_109_0: const #24u : u8
        let s_109_0: u8 = 24;
        // C s_109_1: cast zx s_109_0 -> bv
        let s_109_1: Bits = Bits::new(s_109_0 as u128, 8u16);
        // C s_109_2: cast zx s_109_1 -> i
        let s_109_2: i128 = (s_109_1.value() as i128);
        // C s_109_3: cast reint s_109_2 -> i64
        let s_109_3: i64 = (s_109_2 as i64);
        // C s_109_4: cast zx s_109_3 -> i
        let s_109_4: i128 = (i128::try_from(s_109_3).unwrap());
        // C s_109_5: const #432u : u32
        let s_109_5: u32 = 432;
        // D s_109_6: read-reg s_109_5:u8
        let s_109_6: u8 = {
            let value = state.read_register::<u8>(s_109_5 as isize);
            tracer.read_register(s_109_5 as isize, value);
            value
        };
        // D s_109_7: call AArch64_SystemAccessTrap(s_109_6, s_109_4)
        let s_109_7: () = AArch64_SystemAccessTrap(state, tracer, s_109_6, s_109_4);
        // N s_109_8: return
        return;
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_110_0: read-var __MDCR_EL2_TPM:u8
        let s_110_0: bool = fn_state.u__MDCR_EL2_TPM;
        // D s_110_1: cast zx s_110_0 -> bv
        let s_110_1: Bits = Bits::new(s_110_0 as u128, 1u16);
        // C s_110_2: const #1u : u8
        let s_110_2: bool = true;
        // C s_110_3: cast zx s_110_2 -> bv
        let s_110_3: Bits = Bits::new(s_110_2 as u128, 1u16);
        // D s_110_4: cmp-eq s_110_1 s_110_3
        let s_110_4: bool = ((s_110_1) == (s_110_3));
        // D s_110_5: write-var gs#67968 <= s_110_4
        fn_state.gs_67968 = s_110_4;
        // N s_110_6: jump b87
        return block_87(state, tracer, fn_state);
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_111_0: const #24u : u8
        let s_111_0: u8 = 24;
        // C s_111_1: cast zx s_111_0 -> bv
        let s_111_1: Bits = Bits::new(s_111_0 as u128, 8u16);
        // C s_111_2: cast zx s_111_1 -> i
        let s_111_2: i128 = (s_111_1.value() as i128);
        // C s_111_3: cast reint s_111_2 -> i64
        let s_111_3: i64 = (s_111_2 as i64);
        // C s_111_4: cast zx s_111_3 -> i
        let s_111_4: i128 = (i128::try_from(s_111_3).unwrap());
        // C s_111_5: const #432u : u32
        let s_111_5: u32 = 432;
        // D s_111_6: read-reg s_111_5:u8
        let s_111_6: u8 = {
            let value = state.read_register::<u8>(s_111_5 as isize);
            tracer.read_register(s_111_5 as isize, value);
            value
        };
        // D s_111_7: call AArch64_SystemAccessTrap(s_111_6, s_111_4)
        let s_111_7: () = AArch64_SystemAccessTrap(state, tracer, s_111_6, s_111_4);
        // N s_111_8: return
        return;
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_112_0: read-var __HDFGRTR2_EL2_nPMICNTR_EL0:u8
        let s_112_0: bool = fn_state.u__HDFGRTR2_EL2_nPMICNTR_EL0;
        // D s_112_1: cast zx s_112_0 -> bv
        let s_112_1: Bits = Bits::new(s_112_0 as u128, 1u16);
        // C s_112_2: const #0u : u8
        let s_112_2: bool = false;
        // C s_112_3: cast zx s_112_2 -> bv
        let s_112_3: Bits = Bits::new(s_112_2 as u128, 1u16);
        // D s_112_4: cmp-eq s_112_1 s_112_3
        let s_112_4: bool = ((s_112_1) == (s_112_3));
        // D s_112_5: write-var gs#67967 <= s_112_4
        fn_state.gs_67967 = s_112_4;
        // N s_112_6: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_113_0: const #188u : u32
        let s_113_0: u32 = 188;
        // S s_113_1: call IsFeatureImplemented(s_113_0)
        let s_113_1: bool = IsFeatureImplemented(state, tracer, s_113_0);
        // D s_113_2: write-var gs#67966 <= s_113_1
        fn_state.gs_67966 = s_113_1;
        // N s_113_3: jump b82
        return block_82(state, tracer, fn_state);
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
        // C s_114_5: const #432u : u32
        let s_114_5: u32 = 432;
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
        // D s_115_0: read-var __SCR_EL3_FGTEn2:u8
        let s_115_0: bool = fn_state.u__SCR_EL3_FGTEn2;
        // D s_115_1: cast zx s_115_0 -> bv
        let s_115_1: Bits = Bits::new(s_115_0 as u128, 1u16);
        // C s_115_2: const #0u : u8
        let s_115_2: bool = false;
        // C s_115_3: cast zx s_115_2 -> bv
        let s_115_3: Bits = Bits::new(s_115_2 as u128, 1u16);
        // D s_115_4: cmp-eq s_115_1 s_115_3
        let s_115_4: bool = ((s_115_1) == (s_115_3));
        // D s_115_5: write-var gs#67965 <= s_115_4
        fn_state.gs_67965 = s_115_4;
        // N s_115_6: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_116_0: const #424u : u32
        let s_116_0: u32 = 424;
        // D s_116_1: read-reg s_116_0:u8
        let s_116_1: u8 = {
            let value = state.read_register::<u8>(s_116_0 as isize);
            tracer.read_register(s_116_0 as isize, value);
            value
        };
        // C s_116_2: const #2u : u8
        let s_116_2: u8 = 2;
        // D s_116_3: cmp-lt s_116_1 s_116_2
        let s_116_3: bool = ((s_116_1) < (s_116_2));
        // D s_116_4: write-var gs#67964 <= s_116_3
        fn_state.gs_67964 = s_116_3;
        // N s_116_5: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_117_0: const #188u : u32
        let s_117_0: u32 = 188;
        // S s_117_1: call IsFeatureImplemented(s_117_0)
        let s_117_1: bool = IsFeatureImplemented(state, tracer, s_117_0);
        // D s_117_2: write-var gs#67963 <= s_117_1
        fn_state.gs_67963 = s_117_1;
        // N s_117_3: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_118_0: panic
        panic!("{:?}", ());
        // N s_118_1: return
        return;
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_119_0: read-var __MDCR_EL3_TPM:u8
        let s_119_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_119_1: cast zx s_119_0 -> bv
        let s_119_1: Bits = Bits::new(s_119_0 as u128, 1u16);
        // C s_119_2: const #1u : u8
        let s_119_2: bool = true;
        // C s_119_3: cast zx s_119_2 -> bv
        let s_119_3: Bits = Bits::new(s_119_2 as u128, 1u16);
        // D s_119_4: cmp-eq s_119_1 s_119_3
        let s_119_4: bool = ((s_119_1) == (s_119_3));
        // D s_119_5: write-var gs#67962 <= s_119_4
        fn_state.gs_67962 = s_119_4;
        // N s_119_6: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_120_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_120_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_120_1: call __IMPDEF_boolean(s_120_0)
        let s_120_1: bool = u__IMPDEF_boolean(state, tracer, s_120_0);
        // D s_120_2: write-var gs#67961 <= s_120_1
        fn_state.gs_67961 = s_120_1;
        // N s_120_3: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_121_0: read-var __EDSCR_SDD:u8
        let s_121_0: bool = fn_state.u__EDSCR_SDD;
        // D s_121_1: cast zx s_121_0 -> bv
        let s_121_1: Bits = Bits::new(s_121_0 as u128, 1u16);
        // C s_121_2: const #1u : u8
        let s_121_2: bool = true;
        // C s_121_3: cast zx s_121_2 -> bv
        let s_121_3: Bits = Bits::new(s_121_2 as u128, 1u16);
        // D s_121_4: cmp-eq s_121_1 s_121_3
        let s_121_4: bool = ((s_121_1) == (s_121_3));
        // D s_121_5: write-var gs#67960 <= s_121_4
        fn_state.gs_67960 = s_121_4;
        // N s_121_6: jump b68
        return block_68(state, tracer, fn_state);
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_122_0: const #424u : u32
        let s_122_0: u32 = 424;
        // D s_122_1: read-reg s_122_0:u8
        let s_122_1: u8 = {
            let value = state.read_register::<u8>(s_122_0 as isize);
            tracer.read_register(s_122_0 as isize, value);
            value
        };
        // C s_122_2: const #2u : u8
        let s_122_2: u8 = 2;
        // D s_122_3: cmp-lt s_122_1 s_122_2
        let s_122_3: bool = ((s_122_1) < (s_122_2));
        // D s_122_4: write-var gs#67959 <= s_122_3
        fn_state.gs_67959 = s_122_3;
        // N s_122_5: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_123_0: panic
        panic!("{:?}", ());
        // N s_123_1: return
        return;
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_124_0: read-var __MDCR_EL3_EnPM2:u8
        let s_124_0: bool = fn_state.u__MDCR_EL3_EnPM2;
        // D s_124_1: cast zx s_124_0 -> bv
        let s_124_1: Bits = Bits::new(s_124_0 as u128, 1u16);
        // C s_124_2: const #0u : u8
        let s_124_2: bool = false;
        // C s_124_3: cast zx s_124_2 -> bv
        let s_124_3: Bits = Bits::new(s_124_2 as u128, 1u16);
        // D s_124_4: cmp-eq s_124_1 s_124_3
        let s_124_4: bool = ((s_124_1) == (s_124_3));
        // D s_124_5: write-var gs#67958 <= s_124_4
        fn_state.gs_67958 = s_124_4;
        // N s_124_6: jump b63
        return block_63(state, tracer, fn_state);
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_125_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_125_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_125_1: call __IMPDEF_boolean(s_125_0)
        let s_125_1: bool = u__IMPDEF_boolean(state, tracer, s_125_0);
        // D s_125_2: write-var gs#67957 <= s_125_1
        fn_state.gs_67957 = s_125_1;
        // N s_125_3: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_126_0: read-var __EDSCR_SDD:u8
        let s_126_0: bool = fn_state.u__EDSCR_SDD;
        // D s_126_1: cast zx s_126_0 -> bv
        let s_126_1: Bits = Bits::new(s_126_0 as u128, 1u16);
        // C s_126_2: const #1u : u8
        let s_126_2: bool = true;
        // C s_126_3: cast zx s_126_2 -> bv
        let s_126_3: Bits = Bits::new(s_126_2 as u128, 1u16);
        // D s_126_4: cmp-eq s_126_1 s_126_3
        let s_126_4: bool = ((s_126_1) == (s_126_3));
        // D s_126_5: write-var gs#67956 <= s_126_4
        fn_state.gs_67956 = s_126_4;
        // N s_126_6: jump b59
        return block_59(state, tracer, fn_state);
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_127_0: const #424u : u32
        let s_127_0: u32 = 424;
        // D s_127_1: read-reg s_127_0:u8
        let s_127_1: u8 = {
            let value = state.read_register::<u8>(s_127_0 as isize);
            tracer.read_register(s_127_0 as isize, value);
            value
        };
        // C s_127_2: const #2u : u8
        let s_127_2: u8 = 2;
        // D s_127_3: cmp-lt s_127_1 s_127_2
        let s_127_3: bool = ((s_127_1) < (s_127_2));
        // D s_127_4: write-var gs#67955 <= s_127_3
        fn_state.gs_67955 = s_127_3;
        // N s_127_5: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_128<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_128_0: const #() : ()
        let s_128_0: () = ();
        // S s_128_1: call Halted(s_128_0)
        let s_128_1: bool = Halted(state, tracer, s_128_0);
        // N s_128_2: branch s_128_1 b213 b129
        if s_128_1 {
            return block_213(state, tracer, fn_state);
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
        // D s_129_1: write-var gs#67974 <= s_129_0
        fn_state.gs_67974 = s_129_0;
        // N s_129_2: jump b130
        return block_130(state, tracer, fn_state);
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_130_0: read-var gs#67974:u8
        let s_130_0: bool = fn_state.gs_67974;
        // N s_130_1: branch s_130_0 b212 b131
        if s_130_0 {
            return block_212(state, tracer, fn_state);
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
        // D s_131_1: write-var gs#67975 <= s_131_0
        fn_state.gs_67975 = s_131_0;
        // N s_131_2: jump b132
        return block_132(state, tracer, fn_state);
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_132_0: read-var gs#67975:u8
        let s_132_0: bool = fn_state.gs_67975;
        // N s_132_1: branch s_132_0 b211 b133
        if s_132_0 {
            return block_211(state, tracer, fn_state);
        } else {
            return block_133(state, tracer, fn_state);
        };
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_133_0: const #0u : u8
        let s_133_0: bool = false;
        // D s_133_1: write-var gs#67976 <= s_133_0
        fn_state.gs_67976 = s_133_0;
        // N s_133_2: jump b134
        return block_134(state, tracer, fn_state);
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_134_0: read-var gs#67976:u8
        let s_134_0: bool = fn_state.gs_67976;
        // N s_134_1: branch s_134_0 b210 b135
        if s_134_0 {
            return block_210(state, tracer, fn_state);
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
        // D s_135_1: write-var gs#67977 <= s_135_0
        fn_state.gs_67977 = s_135_0;
        // N s_135_2: jump b136
        return block_136(state, tracer, fn_state);
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_136_0: read-var gs#67977:u8
        let s_136_0: bool = fn_state.gs_67977;
        // N s_136_1: branch s_136_0 b209 b137
        if s_136_0 {
            return block_209(state, tracer, fn_state);
        } else {
            return block_137(state, tracer, fn_state);
        };
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_137_0: const #() : ()
        let s_137_0: () = ();
        // S s_137_1: call Halted(s_137_0)
        let s_137_1: bool = Halted(state, tracer, s_137_0);
        // N s_137_2: branch s_137_1 b208 b138
        if s_137_1 {
            return block_208(state, tracer, fn_state);
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
        // D s_138_1: write-var gs#67978 <= s_138_0
        fn_state.gs_67978 = s_138_0;
        // N s_138_2: jump b139
        return block_139(state, tracer, fn_state);
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_139_0: read-var gs#67978:u8
        let s_139_0: bool = fn_state.gs_67978;
        // N s_139_1: branch s_139_0 b207 b140
        if s_139_0 {
            return block_207(state, tracer, fn_state);
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
        // D s_140_1: write-var gs#67979 <= s_140_0
        fn_state.gs_67979 = s_140_0;
        // N s_140_2: jump b141
        return block_141(state, tracer, fn_state);
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_141_0: read-var gs#67979:u8
        let s_141_0: bool = fn_state.gs_67979;
        // N s_141_1: branch s_141_0 b206 b142
        if s_141_0 {
            return block_206(state, tracer, fn_state);
        } else {
            return block_142(state, tracer, fn_state);
        };
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_142_0: const #0u : u8
        let s_142_0: bool = false;
        // D s_142_1: write-var gs#67980 <= s_142_0
        fn_state.gs_67980 = s_142_0;
        // N s_142_2: jump b143
        return block_143(state, tracer, fn_state);
    }
    fn block_143<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_143_0: read-var gs#67980:u8
        let s_143_0: bool = fn_state.gs_67980;
        // N s_143_1: branch s_143_0 b205 b144
        if s_143_0 {
            return block_205(state, tracer, fn_state);
        } else {
            return block_144(state, tracer, fn_state);
        };
    }
    fn block_144<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_144_0: const #0u : u8
        let s_144_0: bool = false;
        // D s_144_1: write-var gs#67981 <= s_144_0
        fn_state.gs_67981 = s_144_0;
        // N s_144_2: jump b145
        return block_145(state, tracer, fn_state);
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_145_0: read-var gs#67981:u8
        let s_145_0: bool = fn_state.gs_67981;
        // N s_145_1: branch s_145_0 b204 b146
        if s_145_0 {
            return block_204(state, tracer, fn_state);
        } else {
            return block_146(state, tracer, fn_state);
        };
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_146_0: read-var __PMUSERENR_EL0_UEN:u8
        let s_146_0: bool = fn_state.u__PMUSERENR_EL0_UEN;
        // D s_146_1: cast zx s_146_0 -> bv
        let s_146_1: Bits = Bits::new(s_146_0 as u128, 1u16);
        // C s_146_2: const #0u : u8
        let s_146_2: bool = false;
        // C s_146_3: cast zx s_146_2 -> bv
        let s_146_3: Bits = Bits::new(s_146_2 as u128, 1u16);
        // D s_146_4: cmp-eq s_146_1 s_146_3
        let s_146_4: bool = ((s_146_1) == (s_146_3));
        // N s_146_5: branch s_146_4 b198 b147
        if s_146_4 {
            return block_198(state, tracer, fn_state);
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
        // N s_147_2: branch s_147_1 b197 b148
        if s_147_1 {
            return block_197(state, tracer, fn_state);
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
        // D s_148_1: write-var gs#67982 <= s_148_0
        fn_state.gs_67982 = s_148_0;
        // N s_148_2: jump b149
        return block_149(state, tracer, fn_state);
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_149_0: read-var gs#67982:u8
        let s_149_0: bool = fn_state.gs_67982;
        // N s_149_1: branch s_149_0 b196 b150
        if s_149_0 {
            return block_196(state, tracer, fn_state);
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
        // D s_150_1: write-var gs#67983 <= s_150_0
        fn_state.gs_67983 = s_150_0;
        // N s_150_2: jump b151
        return block_151(state, tracer, fn_state);
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_151_0: read-var gs#67983:u8
        let s_151_0: bool = fn_state.gs_67983;
        // N s_151_1: branch s_151_0 b195 b152
        if s_151_0 {
            return block_195(state, tracer, fn_state);
        } else {
            return block_152(state, tracer, fn_state);
        };
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_152_0: const #0u : u8
        let s_152_0: bool = false;
        // D s_152_1: write-var gs#67984 <= s_152_0
        fn_state.gs_67984 = s_152_0;
        // N s_152_2: jump b153
        return block_153(state, tracer, fn_state);
    }
    fn block_153<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_153_0: read-var gs#67984:u8
        let s_153_0: bool = fn_state.gs_67984;
        // N s_153_1: branch s_153_0 b194 b154
        if s_153_0 {
            return block_194(state, tracer, fn_state);
        } else {
            return block_154(state, tracer, fn_state);
        };
    }
    fn block_154<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_154_0: const #0u : u8
        let s_154_0: bool = false;
        // D s_154_1: write-var gs#67985 <= s_154_0
        fn_state.gs_67985 = s_154_0;
        // N s_154_2: jump b155
        return block_155(state, tracer, fn_state);
    }
    fn block_155<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_155_0: read-var gs#67985:u8
        let s_155_0: bool = fn_state.gs_67985;
        // N s_155_1: branch s_155_0 b193 b156
        if s_155_0 {
            return block_193(state, tracer, fn_state);
        } else {
            return block_156(state, tracer, fn_state);
        };
    }
    fn block_156<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_156_0: const #() : ()
        let s_156_0: () = ();
        // S s_156_1: call EL2Enabled(s_156_0)
        let s_156_1: bool = EL2Enabled(state, tracer, s_156_0);
        // N s_156_2: branch s_156_1 b192 b157
        if s_156_1 {
            return block_192(state, tracer, fn_state);
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
        // D s_157_1: write-var gs#67986 <= s_157_0
        fn_state.gs_67986 = s_157_0;
        // N s_157_2: jump b158
        return block_158(state, tracer, fn_state);
    }
    fn block_158<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_158_0: read-var gs#67986:u8
        let s_158_0: bool = fn_state.gs_67986;
        // N s_158_1: branch s_158_0 b191 b159
        if s_158_0 {
            return block_191(state, tracer, fn_state);
        } else {
            return block_159(state, tracer, fn_state);
        };
    }
    fn block_159<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_159_0: const #0u : u8
        let s_159_0: bool = false;
        // D s_159_1: write-var gs#67987 <= s_159_0
        fn_state.gs_67987 = s_159_0;
        // N s_159_2: jump b160
        return block_160(state, tracer, fn_state);
    }
    fn block_160<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_160_0: read-var gs#67987:u8
        let s_160_0: bool = fn_state.gs_67987;
        // N s_160_1: branch s_160_0 b190 b161
        if s_160_0 {
            return block_190(state, tracer, fn_state);
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
        // D s_161_1: write-var gs#67988 <= s_161_0
        fn_state.gs_67988 = s_161_0;
        // N s_161_2: jump b162
        return block_162(state, tracer, fn_state);
    }
    fn block_162<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_162_0: read-var gs#67988:u8
        let s_162_0: bool = fn_state.gs_67988;
        // N s_162_1: branch s_162_0 b189 b163
        if s_162_0 {
            return block_189(state, tracer, fn_state);
        } else {
            return block_163(state, tracer, fn_state);
        };
    }
    fn block_163<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_163_0: const #() : ()
        let s_163_0: () = ();
        // S s_163_1: call EL2Enabled(s_163_0)
        let s_163_1: bool = EL2Enabled(state, tracer, s_163_0);
        // N s_163_2: branch s_163_1 b188 b164
        if s_163_1 {
            return block_188(state, tracer, fn_state);
        } else {
            return block_164(state, tracer, fn_state);
        };
    }
    fn block_164<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_164_0: const #0u : u8
        let s_164_0: bool = false;
        // D s_164_1: write-var gs#67989 <= s_164_0
        fn_state.gs_67989 = s_164_0;
        // N s_164_2: jump b165
        return block_165(state, tracer, fn_state);
    }
    fn block_165<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_165_0: read-var gs#67989:u8
        let s_165_0: bool = fn_state.gs_67989;
        // N s_165_1: branch s_165_0 b187 b166
        if s_165_0 {
            return block_187(state, tracer, fn_state);
        } else {
            return block_166(state, tracer, fn_state);
        };
    }
    fn block_166<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_166_0: const #424u : u32
        let s_166_0: u32 = 424;
        // D s_166_1: read-reg s_166_0:u8
        let s_166_1: u8 = {
            let value = state.read_register::<u8>(s_166_0 as isize);
            tracer.read_register(s_166_0 as isize, value);
            value
        };
        // C s_166_2: const #2u : u8
        let s_166_2: u8 = 2;
        // D s_166_3: cmp-lt s_166_1 s_166_2
        let s_166_3: bool = ((s_166_1) < (s_166_2));
        // N s_166_4: branch s_166_3 b186 b167
        if s_166_3 {
            return block_186(state, tracer, fn_state);
        } else {
            return block_167(state, tracer, fn_state);
        };
    }
    fn block_167<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_167_0: const #0u : u8
        let s_167_0: bool = false;
        // D s_167_1: write-var gs#67990 <= s_167_0
        fn_state.gs_67990 = s_167_0;
        // N s_167_2: jump b168
        return block_168(state, tracer, fn_state);
    }
    fn block_168<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_168_0: read-var gs#67990:u8
        let s_168_0: bool = fn_state.gs_67990;
        // N s_168_1: branch s_168_0 b180 b169
        if s_168_0 {
            return block_180(state, tracer, fn_state);
        } else {
            return block_169(state, tracer, fn_state);
        };
    }
    fn block_169<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_169_0: const #424u : u32
        let s_169_0: u32 = 424;
        // D s_169_1: read-reg s_169_0:u8
        let s_169_1: u8 = {
            let value = state.read_register::<u8>(s_169_0 as isize);
            tracer.read_register(s_169_0 as isize, value);
            value
        };
        // C s_169_2: const #2u : u8
        let s_169_2: u8 = 2;
        // D s_169_3: cmp-lt s_169_1 s_169_2
        let s_169_3: bool = ((s_169_1) < (s_169_2));
        // N s_169_4: branch s_169_3 b179 b170
        if s_169_3 {
            return block_179(state, tracer, fn_state);
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
        // D s_170_1: write-var gs#67991 <= s_170_0
        fn_state.gs_67991 = s_170_0;
        // N s_170_2: jump b171
        return block_171(state, tracer, fn_state);
    }
    fn block_171<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_171_0: read-var gs#67991:u8
        let s_171_0: bool = fn_state.gs_67991;
        // N s_171_1: branch s_171_0 b173 b172
        if s_171_0 {
            return block_173(state, tracer, fn_state);
        } else {
            return block_172(state, tracer, fn_state);
        };
    }
    fn block_172<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_172_0: const #64s : i64
        let s_172_0: i64 = 64;
        // C s_172_1: const #12944u : u32
        let s_172_1: u32 = 12944;
        // D s_172_2: read-reg s_172_1:u64
        let s_172_2: u64 = {
            let value = state.read_register::<u64>(s_172_1 as isize);
            tracer.read_register(s_172_1 as isize, value);
            value
        };
        // D s_172_3: cast zx s_172_2 -> bv
        let s_172_3: Bits = Bits::new(s_172_2 as u128, 64u16);
        // D s_172_4: read-var t:i
        let s_172_4: i128 = fn_state.t;
        // D s_172_5: call X_set(s_172_4, s_172_0, s_172_3)
        let s_172_5: () = X_set(state, tracer, s_172_4, s_172_0, s_172_3);
        // N s_172_6: return
        return;
    }
    fn block_173<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_173_0: const #() : ()
        let s_173_0: () = ();
        // S s_173_1: call Halted(s_173_0)
        let s_173_1: bool = Halted(state, tracer, s_173_0);
        // N s_173_2: branch s_173_1 b178 b174
        if s_173_1 {
            return block_178(state, tracer, fn_state);
        } else {
            return block_174(state, tracer, fn_state);
        };
    }
    fn block_174<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_174_0: const #0u : u8
        let s_174_0: bool = false;
        // D s_174_1: write-var gs#67993 <= s_174_0
        fn_state.gs_67993 = s_174_0;
        // N s_174_2: jump b175
        return block_175(state, tracer, fn_state);
    }
    fn block_175<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_175_0: read-var gs#67993:u8
        let s_175_0: bool = fn_state.gs_67993;
        // N s_175_1: branch s_175_0 b177 b176
        if s_175_0 {
            return block_177(state, tracer, fn_state);
        } else {
            return block_176(state, tracer, fn_state);
        };
    }
    fn block_176<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_176_0: const #24u : u8
        let s_176_0: u8 = 24;
        // C s_176_1: cast zx s_176_0 -> bv
        let s_176_1: Bits = Bits::new(s_176_0 as u128, 8u16);
        // C s_176_2: cast zx s_176_1 -> i
        let s_176_2: i128 = (s_176_1.value() as i128);
        // C s_176_3: cast reint s_176_2 -> i64
        let s_176_3: i64 = (s_176_2 as i64);
        // C s_176_4: cast zx s_176_3 -> i
        let s_176_4: i128 = (i128::try_from(s_176_3).unwrap());
        // C s_176_5: const #424u : u32
        let s_176_5: u32 = 424;
        // D s_176_6: read-reg s_176_5:u8
        let s_176_6: u8 = {
            let value = state.read_register::<u8>(s_176_5 as isize);
            tracer.read_register(s_176_5 as isize, value);
            value
        };
        // D s_176_7: call AArch64_SystemAccessTrap(s_176_6, s_176_4)
        let s_176_7: () = AArch64_SystemAccessTrap(state, tracer, s_176_6, s_176_4);
        // N s_176_8: return
        return;
    }
    fn block_177<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_177_0: panic
        panic!("{:?}", ());
        // N s_177_1: return
        return;
    }
    fn block_178<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_178_0: read-var __EDSCR_SDD:u8
        let s_178_0: bool = fn_state.u__EDSCR_SDD;
        // D s_178_1: cast zx s_178_0 -> bv
        let s_178_1: Bits = Bits::new(s_178_0 as u128, 1u16);
        // C s_178_2: const #1u : u8
        let s_178_2: bool = true;
        // C s_178_3: cast zx s_178_2 -> bv
        let s_178_3: Bits = Bits::new(s_178_2 as u128, 1u16);
        // D s_178_4: cmp-eq s_178_1 s_178_3
        let s_178_4: bool = ((s_178_1) == (s_178_3));
        // D s_178_5: write-var gs#67993 <= s_178_4
        fn_state.gs_67993 = s_178_4;
        // N s_178_6: jump b175
        return block_175(state, tracer, fn_state);
    }
    fn block_179<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_179_0: read-var __MDCR_EL3_TPM:u8
        let s_179_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_179_1: cast zx s_179_0 -> bv
        let s_179_1: Bits = Bits::new(s_179_0 as u128, 1u16);
        // C s_179_2: const #1u : u8
        let s_179_2: bool = true;
        // C s_179_3: cast zx s_179_2 -> bv
        let s_179_3: Bits = Bits::new(s_179_2 as u128, 1u16);
        // D s_179_4: cmp-eq s_179_1 s_179_3
        let s_179_4: bool = ((s_179_1) == (s_179_3));
        // D s_179_5: write-var gs#67991 <= s_179_4
        fn_state.gs_67991 = s_179_4;
        // N s_179_6: jump b171
        return block_171(state, tracer, fn_state);
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
        // D s_181_1: write-var gs#67994 <= s_181_0
        fn_state.gs_67994 = s_181_0;
        // N s_181_2: jump b182
        return block_182(state, tracer, fn_state);
    }
    fn block_182<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_182_0: read-var gs#67994:u8
        let s_182_0: bool = fn_state.gs_67994;
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
        // C s_183_0: const #24u : u8
        let s_183_0: u8 = 24;
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
        // D s_183_7: call AArch64_SystemAccessTrap(s_183_6, s_183_4)
        let s_183_7: () = AArch64_SystemAccessTrap(state, tracer, s_183_6, s_183_4);
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
        // D s_185_0: read-var __EDSCR_SDD:u8
        let s_185_0: bool = fn_state.u__EDSCR_SDD;
        // D s_185_1: cast zx s_185_0 -> bv
        let s_185_1: Bits = Bits::new(s_185_0 as u128, 1u16);
        // C s_185_2: const #1u : u8
        let s_185_2: bool = true;
        // C s_185_3: cast zx s_185_2 -> bv
        let s_185_3: Bits = Bits::new(s_185_2 as u128, 1u16);
        // D s_185_4: cmp-eq s_185_1 s_185_3
        let s_185_4: bool = ((s_185_1) == (s_185_3));
        // D s_185_5: write-var gs#67994 <= s_185_4
        fn_state.gs_67994 = s_185_4;
        // N s_185_6: jump b182
        return block_182(state, tracer, fn_state);
    }
    fn block_186<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_186_0: read-var __MDCR_EL3_EnPM2:u8
        let s_186_0: bool = fn_state.u__MDCR_EL3_EnPM2;
        // D s_186_1: cast zx s_186_0 -> bv
        let s_186_1: Bits = Bits::new(s_186_0 as u128, 1u16);
        // C s_186_2: const #0u : u8
        let s_186_2: bool = false;
        // C s_186_3: cast zx s_186_2 -> bv
        let s_186_3: Bits = Bits::new(s_186_2 as u128, 1u16);
        // D s_186_4: cmp-eq s_186_1 s_186_3
        let s_186_4: bool = ((s_186_1) == (s_186_3));
        // D s_186_5: write-var gs#67990 <= s_186_4
        fn_state.gs_67990 = s_186_4;
        // N s_186_6: jump b168
        return block_168(state, tracer, fn_state);
    }
    fn block_187<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_187_0: const #24u : u8
        let s_187_0: u8 = 24;
        // C s_187_1: cast zx s_187_0 -> bv
        let s_187_1: Bits = Bits::new(s_187_0 as u128, 8u16);
        // C s_187_2: cast zx s_187_1 -> i
        let s_187_2: i128 = (s_187_1.value() as i128);
        // C s_187_3: cast reint s_187_2 -> i64
        let s_187_3: i64 = (s_187_2 as i64);
        // C s_187_4: cast zx s_187_3 -> i
        let s_187_4: i128 = (i128::try_from(s_187_3).unwrap());
        // C s_187_5: const #432u : u32
        let s_187_5: u32 = 432;
        // D s_187_6: read-reg s_187_5:u8
        let s_187_6: u8 = {
            let value = state.read_register::<u8>(s_187_5 as isize);
            tracer.read_register(s_187_5 as isize, value);
            value
        };
        // D s_187_7: call AArch64_SystemAccessTrap(s_187_6, s_187_4)
        let s_187_7: () = AArch64_SystemAccessTrap(state, tracer, s_187_6, s_187_4);
        // N s_187_8: return
        return;
    }
    fn block_188<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_188_0: read-var __MDCR_EL2_TPM:u8
        let s_188_0: bool = fn_state.u__MDCR_EL2_TPM;
        // D s_188_1: cast zx s_188_0 -> bv
        let s_188_1: Bits = Bits::new(s_188_0 as u128, 1u16);
        // C s_188_2: const #1u : u8
        let s_188_2: bool = true;
        // C s_188_3: cast zx s_188_2 -> bv
        let s_188_3: Bits = Bits::new(s_188_2 as u128, 1u16);
        // D s_188_4: cmp-eq s_188_1 s_188_3
        let s_188_4: bool = ((s_188_1) == (s_188_3));
        // D s_188_5: write-var gs#67989 <= s_188_4
        fn_state.gs_67989 = s_188_4;
        // N s_188_6: jump b165
        return block_165(state, tracer, fn_state);
    }
    fn block_189<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_189_0: const #24u : u8
        let s_189_0: u8 = 24;
        // C s_189_1: cast zx s_189_0 -> bv
        let s_189_1: Bits = Bits::new(s_189_0 as u128, 8u16);
        // C s_189_2: cast zx s_189_1 -> i
        let s_189_2: i128 = (s_189_1.value() as i128);
        // C s_189_3: cast reint s_189_2 -> i64
        let s_189_3: i64 = (s_189_2 as i64);
        // C s_189_4: cast zx s_189_3 -> i
        let s_189_4: i128 = (i128::try_from(s_189_3).unwrap());
        // C s_189_5: const #432u : u32
        let s_189_5: u32 = 432;
        // D s_189_6: read-reg s_189_5:u8
        let s_189_6: u8 = {
            let value = state.read_register::<u8>(s_189_5 as isize);
            tracer.read_register(s_189_5 as isize, value);
            value
        };
        // D s_189_7: call AArch64_SystemAccessTrap(s_189_6, s_189_4)
        let s_189_7: () = AArch64_SystemAccessTrap(state, tracer, s_189_6, s_189_4);
        // N s_189_8: return
        return;
    }
    fn block_190<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_190_0: read-var __HDFGRTR2_EL2_nPMICNTR_EL0:u8
        let s_190_0: bool = fn_state.u__HDFGRTR2_EL2_nPMICNTR_EL0;
        // D s_190_1: cast zx s_190_0 -> bv
        let s_190_1: Bits = Bits::new(s_190_0 as u128, 1u16);
        // C s_190_2: const #0u : u8
        let s_190_2: bool = false;
        // C s_190_3: cast zx s_190_2 -> bv
        let s_190_3: Bits = Bits::new(s_190_2 as u128, 1u16);
        // D s_190_4: cmp-eq s_190_1 s_190_3
        let s_190_4: bool = ((s_190_1) == (s_190_3));
        // D s_190_5: write-var gs#67988 <= s_190_4
        fn_state.gs_67988 = s_190_4;
        // N s_190_6: jump b162
        return block_162(state, tracer, fn_state);
    }
    fn block_191<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_191_0: const #188u : u32
        let s_191_0: u32 = 188;
        // S s_191_1: call IsFeatureImplemented(s_191_0)
        let s_191_1: bool = IsFeatureImplemented(state, tracer, s_191_0);
        // D s_191_2: write-var gs#67987 <= s_191_1
        fn_state.gs_67987 = s_191_1;
        // N s_191_3: jump b160
        return block_160(state, tracer, fn_state);
    }
    fn block_192<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_192_0: const #102552u : u32
        let s_192_0: u32 = 102552;
        // D s_192_1: read-reg s_192_0:struct
        let s_192_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_192_0 as isize);
            tracer.read_register(s_192_0 as isize, value);
            value
        };
        // D s_192_2: call _get_HCR_EL2_Type_E2H(s_192_1)
        let s_192_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_192_1);
        // C s_192_3: const #102552u : u32
        let s_192_3: u32 = 102552;
        // D s_192_4: read-reg s_192_3:struct
        let s_192_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_192_3 as isize);
            tracer.read_register(s_192_3 as isize, value);
            value
        };
        // D s_192_5: call _get_HCR_EL2_Type_TGE(s_192_4)
        let s_192_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_192_4);
        // D s_192_6: cast zx s_192_2 -> bv
        let s_192_6: Bits = Bits::new(s_192_2 as u128, 1u16);
        // D s_192_7: cast zx s_192_5 -> bv
        let s_192_7: Bits = Bits::new(s_192_5 as u128, 1u16);
        // D s_192_8: cast reint s_192_6 -> u128
        let s_192_8: u128 = (s_192_6.value() as u128);
        // D s_192_9: size-of s_192_6
        let s_192_9: u16 = s_192_6.length();
        // D s_192_10: cast reint s_192_7 -> u128
        let s_192_10: u128 = (s_192_7.value() as u128);
        // D s_192_11: size-of s_192_7
        let s_192_11: u16 = s_192_7.length();
        // D s_192_12: lsl s_192_8 s_192_11
        let s_192_12: u128 = s_192_8 << s_192_11;
        // D s_192_13: or s_192_12 s_192_10
        let s_192_13: u128 = ((s_192_12) | (s_192_10));
        // D s_192_14: add s_192_9 s_192_11
        let s_192_14: u16 = (s_192_9 + s_192_11);
        // D s_192_15: create-bits s_192_13 s_192_14
        let s_192_15: Bits = Bits::new(s_192_13, s_192_14);
        // D s_192_16: cast reint s_192_15 -> u8
        let s_192_16: u8 = (s_192_15.value() as u8);
        // D s_192_17: cast zx s_192_16 -> bv
        let s_192_17: Bits = Bits::new(s_192_16 as u128, 2u16);
        // C s_192_18: const #3u : u8
        let s_192_18: u8 = 3;
        // C s_192_19: cast zx s_192_18 -> bv
        let s_192_19: Bits = Bits::new(s_192_18 as u128, 2u16);
        // D s_192_20: cmp-ne s_192_17 s_192_19
        let s_192_20: bool = ((s_192_17) != (s_192_19));
        // D s_192_21: write-var gs#67986 <= s_192_20
        fn_state.gs_67986 = s_192_20;
        // N s_192_22: jump b158
        return block_158(state, tracer, fn_state);
    }
    fn block_193<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_193_0: const #24u : u8
        let s_193_0: u8 = 24;
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
        // D s_193_7: call AArch64_SystemAccessTrap(s_193_6, s_193_4)
        let s_193_7: () = AArch64_SystemAccessTrap(state, tracer, s_193_6, s_193_4);
        // N s_193_8: return
        return;
    }
    fn block_194<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_194_0: read-var __SCR_EL3_FGTEn2:u8
        let s_194_0: bool = fn_state.u__SCR_EL3_FGTEn2;
        // D s_194_1: cast zx s_194_0 -> bv
        let s_194_1: Bits = Bits::new(s_194_0 as u128, 1u16);
        // C s_194_2: const #0u : u8
        let s_194_2: bool = false;
        // C s_194_3: cast zx s_194_2 -> bv
        let s_194_3: Bits = Bits::new(s_194_2 as u128, 1u16);
        // D s_194_4: cmp-eq s_194_1 s_194_3
        let s_194_4: bool = ((s_194_1) == (s_194_3));
        // D s_194_5: write-var gs#67985 <= s_194_4
        fn_state.gs_67985 = s_194_4;
        // N s_194_6: jump b155
        return block_155(state, tracer, fn_state);
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
        // C s_195_2: const #2u : u8
        let s_195_2: u8 = 2;
        // D s_195_3: cmp-lt s_195_1 s_195_2
        let s_195_3: bool = ((s_195_1) < (s_195_2));
        // D s_195_4: write-var gs#67984 <= s_195_3
        fn_state.gs_67984 = s_195_3;
        // N s_195_5: jump b153
        return block_153(state, tracer, fn_state);
    }
    fn block_196<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_196_0: const #188u : u32
        let s_196_0: u32 = 188;
        // S s_196_1: call IsFeatureImplemented(s_196_0)
        let s_196_1: bool = IsFeatureImplemented(state, tracer, s_196_0);
        // D s_196_2: write-var gs#67983 <= s_196_1
        fn_state.gs_67983 = s_196_1;
        // N s_196_3: jump b151
        return block_151(state, tracer, fn_state);
    }
    fn block_197<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_197_0: const #102552u : u32
        let s_197_0: u32 = 102552;
        // D s_197_1: read-reg s_197_0:struct
        let s_197_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_197_0 as isize);
            tracer.read_register(s_197_0 as isize, value);
            value
        };
        // D s_197_2: call _get_HCR_EL2_Type_E2H(s_197_1)
        let s_197_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_197_1);
        // C s_197_3: const #102552u : u32
        let s_197_3: u32 = 102552;
        // D s_197_4: read-reg s_197_3:struct
        let s_197_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_197_3 as isize);
            tracer.read_register(s_197_3 as isize, value);
            value
        };
        // D s_197_5: call _get_HCR_EL2_Type_TGE(s_197_4)
        let s_197_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_197_4);
        // D s_197_6: cast zx s_197_2 -> bv
        let s_197_6: Bits = Bits::new(s_197_2 as u128, 1u16);
        // D s_197_7: cast zx s_197_5 -> bv
        let s_197_7: Bits = Bits::new(s_197_5 as u128, 1u16);
        // D s_197_8: cast reint s_197_6 -> u128
        let s_197_8: u128 = (s_197_6.value() as u128);
        // D s_197_9: size-of s_197_6
        let s_197_9: u16 = s_197_6.length();
        // D s_197_10: cast reint s_197_7 -> u128
        let s_197_10: u128 = (s_197_7.value() as u128);
        // D s_197_11: size-of s_197_7
        let s_197_11: u16 = s_197_7.length();
        // D s_197_12: lsl s_197_8 s_197_11
        let s_197_12: u128 = s_197_8 << s_197_11;
        // D s_197_13: or s_197_12 s_197_10
        let s_197_13: u128 = ((s_197_12) | (s_197_10));
        // D s_197_14: add s_197_9 s_197_11
        let s_197_14: u16 = (s_197_9 + s_197_11);
        // D s_197_15: create-bits s_197_13 s_197_14
        let s_197_15: Bits = Bits::new(s_197_13, s_197_14);
        // D s_197_16: cast reint s_197_15 -> u8
        let s_197_16: u8 = (s_197_15.value() as u8);
        // D s_197_17: cast zx s_197_16 -> bv
        let s_197_17: Bits = Bits::new(s_197_16 as u128, 2u16);
        // C s_197_18: const #3u : u8
        let s_197_18: u8 = 3;
        // C s_197_19: cast zx s_197_18 -> bv
        let s_197_19: Bits = Bits::new(s_197_18 as u128, 2u16);
        // D s_197_20: cmp-ne s_197_17 s_197_19
        let s_197_20: bool = ((s_197_17) != (s_197_19));
        // D s_197_21: write-var gs#67982 <= s_197_20
        fn_state.gs_67982 = s_197_20;
        // N s_197_22: jump b149
        return block_149(state, tracer, fn_state);
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
        // N s_198_2: branch s_198_1 b203 b199
        if s_198_1 {
            return block_203(state, tracer, fn_state);
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
        // D s_199_1: write-var gs#67995 <= s_199_0
        fn_state.gs_67995 = s_199_0;
        // N s_199_2: jump b200
        return block_200(state, tracer, fn_state);
    }
    fn block_200<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_200_0: read-var gs#67995:u8
        let s_200_0: bool = fn_state.gs_67995;
        // N s_200_1: branch s_200_0 b202 b201
        if s_200_0 {
            return block_202(state, tracer, fn_state);
        } else {
            return block_201(state, tracer, fn_state);
        };
    }
    fn block_201<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_201_0: const #24u : u8
        let s_201_0: u8 = 24;
        // C s_201_1: cast zx s_201_0 -> bv
        let s_201_1: Bits = Bits::new(s_201_0 as u128, 8u16);
        // C s_201_2: cast zx s_201_1 -> i
        let s_201_2: i128 = (s_201_1.value() as i128);
        // C s_201_3: cast reint s_201_2 -> i64
        let s_201_3: i64 = (s_201_2 as i64);
        // C s_201_4: cast zx s_201_3 -> i
        let s_201_4: i128 = (i128::try_from(s_201_3).unwrap());
        // C s_201_5: const #440u : u32
        let s_201_5: u32 = 440;
        // D s_201_6: read-reg s_201_5:u8
        let s_201_6: u8 = {
            let value = state.read_register::<u8>(s_201_5 as isize);
            tracer.read_register(s_201_5 as isize, value);
            value
        };
        // D s_201_7: call AArch64_SystemAccessTrap(s_201_6, s_201_4)
        let s_201_7: () = AArch64_SystemAccessTrap(state, tracer, s_201_6, s_201_4);
        // N s_201_8: return
        return;
    }
    fn block_202<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_202_0: const #24u : u8
        let s_202_0: u8 = 24;
        // C s_202_1: cast zx s_202_0 -> bv
        let s_202_1: Bits = Bits::new(s_202_0 as u128, 8u16);
        // C s_202_2: cast zx s_202_1 -> i
        let s_202_2: i128 = (s_202_1.value() as i128);
        // C s_202_3: cast reint s_202_2 -> i64
        let s_202_3: i64 = (s_202_2 as i64);
        // C s_202_4: cast zx s_202_3 -> i
        let s_202_4: i128 = (i128::try_from(s_202_3).unwrap());
        // C s_202_5: const #432u : u32
        let s_202_5: u32 = 432;
        // D s_202_6: read-reg s_202_5:u8
        let s_202_6: u8 = {
            let value = state.read_register::<u8>(s_202_5 as isize);
            tracer.read_register(s_202_5 as isize, value);
            value
        };
        // D s_202_7: call AArch64_SystemAccessTrap(s_202_6, s_202_4)
        let s_202_7: () = AArch64_SystemAccessTrap(state, tracer, s_202_6, s_202_4);
        // N s_202_8: return
        return;
    }
    fn block_203<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_203_0: read-var __HCR_EL2_TGE:u8
        let s_203_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_203_1: cast zx s_203_0 -> bv
        let s_203_1: Bits = Bits::new(s_203_0 as u128, 1u16);
        // C s_203_2: const #1u : u8
        let s_203_2: bool = true;
        // C s_203_3: cast zx s_203_2 -> bv
        let s_203_3: Bits = Bits::new(s_203_2 as u128, 1u16);
        // D s_203_4: cmp-eq s_203_1 s_203_3
        let s_203_4: bool = ((s_203_1) == (s_203_3));
        // D s_203_5: write-var gs#67995 <= s_203_4
        fn_state.gs_67995 = s_203_4;
        // N s_203_6: jump b200
        return block_200(state, tracer, fn_state);
    }
    fn block_204<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_204_0: panic
        panic!("{:?}", ());
        // N s_204_1: return
        return;
    }
    fn block_205<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_205_0: read-var __MDCR_EL3_TPM:u8
        let s_205_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_205_1: cast zx s_205_0 -> bv
        let s_205_1: Bits = Bits::new(s_205_0 as u128, 1u16);
        // C s_205_2: const #1u : u8
        let s_205_2: bool = true;
        // C s_205_3: cast zx s_205_2 -> bv
        let s_205_3: Bits = Bits::new(s_205_2 as u128, 1u16);
        // D s_205_4: cmp-eq s_205_1 s_205_3
        let s_205_4: bool = ((s_205_1) == (s_205_3));
        // D s_205_5: write-var gs#67981 <= s_205_4
        fn_state.gs_67981 = s_205_4;
        // N s_205_6: jump b145
        return block_145(state, tracer, fn_state);
    }
    fn block_206<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_206_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_206_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_206_1: call __IMPDEF_boolean(s_206_0)
        let s_206_1: bool = u__IMPDEF_boolean(state, tracer, s_206_0);
        // D s_206_2: write-var gs#67980 <= s_206_1
        fn_state.gs_67980 = s_206_1;
        // N s_206_3: jump b143
        return block_143(state, tracer, fn_state);
    }
    fn block_207<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_207_0: read-var __EDSCR_SDD:u8
        let s_207_0: bool = fn_state.u__EDSCR_SDD;
        // D s_207_1: cast zx s_207_0 -> bv
        let s_207_1: Bits = Bits::new(s_207_0 as u128, 1u16);
        // C s_207_2: const #1u : u8
        let s_207_2: bool = true;
        // C s_207_3: cast zx s_207_2 -> bv
        let s_207_3: Bits = Bits::new(s_207_2 as u128, 1u16);
        // D s_207_4: cmp-eq s_207_1 s_207_3
        let s_207_4: bool = ((s_207_1) == (s_207_3));
        // D s_207_5: write-var gs#67979 <= s_207_4
        fn_state.gs_67979 = s_207_4;
        // N s_207_6: jump b141
        return block_141(state, tracer, fn_state);
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
        // C s_208_2: const #2u : u8
        let s_208_2: u8 = 2;
        // D s_208_3: cmp-lt s_208_1 s_208_2
        let s_208_3: bool = ((s_208_1) < (s_208_2));
        // D s_208_4: write-var gs#67978 <= s_208_3
        fn_state.gs_67978 = s_208_3;
        // N s_208_5: jump b139
        return block_139(state, tracer, fn_state);
    }
    fn block_209<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_209_0: panic
        panic!("{:?}", ());
        // N s_209_1: return
        return;
    }
    fn block_210<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_210_0: read-var __MDCR_EL3_EnPM2:u8
        let s_210_0: bool = fn_state.u__MDCR_EL3_EnPM2;
        // D s_210_1: cast zx s_210_0 -> bv
        let s_210_1: Bits = Bits::new(s_210_0 as u128, 1u16);
        // C s_210_2: const #0u : u8
        let s_210_2: bool = false;
        // C s_210_3: cast zx s_210_2 -> bv
        let s_210_3: Bits = Bits::new(s_210_2 as u128, 1u16);
        // D s_210_4: cmp-eq s_210_1 s_210_3
        let s_210_4: bool = ((s_210_1) == (s_210_3));
        // D s_210_5: write-var gs#67977 <= s_210_4
        fn_state.gs_67977 = s_210_4;
        // N s_210_6: jump b136
        return block_136(state, tracer, fn_state);
    }
    fn block_211<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_211_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_211_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_211_1: call __IMPDEF_boolean(s_211_0)
        let s_211_1: bool = u__IMPDEF_boolean(state, tracer, s_211_0);
        // D s_211_2: write-var gs#67976 <= s_211_1
        fn_state.gs_67976 = s_211_1;
        // N s_211_3: jump b134
        return block_134(state, tracer, fn_state);
    }
    fn block_212<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_212_0: read-var __EDSCR_SDD:u8
        let s_212_0: bool = fn_state.u__EDSCR_SDD;
        // D s_212_1: cast zx s_212_0 -> bv
        let s_212_1: Bits = Bits::new(s_212_0 as u128, 1u16);
        // C s_212_2: const #1u : u8
        let s_212_2: bool = true;
        // C s_212_3: cast zx s_212_2 -> bv
        let s_212_3: Bits = Bits::new(s_212_2 as u128, 1u16);
        // D s_212_4: cmp-eq s_212_1 s_212_3
        let s_212_4: bool = ((s_212_1) == (s_212_3));
        // D s_212_5: write-var gs#67975 <= s_212_4
        fn_state.gs_67975 = s_212_4;
        // N s_212_6: jump b132
        return block_132(state, tracer, fn_state);
    }
    fn block_213<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_213_0: const #424u : u32
        let s_213_0: u32 = 424;
        // D s_213_1: read-reg s_213_0:u8
        let s_213_1: u8 = {
            let value = state.read_register::<u8>(s_213_0 as isize);
            tracer.read_register(s_213_0 as isize, value);
            value
        };
        // C s_213_2: const #2u : u8
        let s_213_2: u8 = 2;
        // D s_213_3: cmp-lt s_213_1 s_213_2
        let s_213_3: bool = ((s_213_1) < (s_213_2));
        // D s_213_4: write-var gs#67974 <= s_213_3
        fn_state.gs_67974 = s_213_3;
        // N s_213_5: jump b130
        return block_130(state, tracer, fn_state);
    }
}
