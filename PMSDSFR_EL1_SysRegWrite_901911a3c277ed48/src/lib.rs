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
use Mk_PMSDSFR_EL1_Type::*;
use Halted::*;
use u_get_HDFGWTR2_EL2_Type_nPMSDSFR_EL1::*;
use u_get_MDCR_EL3_Type_NSPB::*;
use X_read::*;
use AArch64_SystemAccessTrap::*;
use IsFeatureImplemented::*;
use u_get_SCR_EL3_Type_NS::*;
use u__IMPDEF_boolean::*;
use NVMem_set::*;
use u_get_MDCR_EL3_Type_EnPMS3::*;
use u_get_HCR_EL2_Type_NV::*;
use u_get_SCR_EL3_Type_NSE::*;
use u_get_MDCR_EL3_Type_NSPBE::*;
use u_get_SCR_EL3_Type_FGTEn2::*;
use u_get_MDCR_EL2_Type_TPMS::*;
use u_get_EDSCR_Type_SDD::*;
use EL2Enabled::*;
use EDSCR_read::*;
use u_get_HCR_EL2_Type_NV2::*;
use common::*;
pub fn PMSDSFR_EL1_SysRegWrite_901911a3c277ed48<T: Tracer>(
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
        gs_90919: bool,
        gs_90906: bool,
        u__MDCR_EL3_EnPMS3: bool,
        gs_90927: bool,
        gs_90924: bool,
        gs_90918: bool,
        gs_90922: bool,
        gs_90921: bool,
        gs_90933: bool,
        gs_90907: bool,
        gs_90928: bool,
        u__MDCR_EL3_NSPBE: bool,
        u__HDFGWTR2_EL2_nPMSDSFR_EL1: bool,
        gs_90891: bool,
        u__SCR_EL3_FGTEn2: bool,
        gs_90911: bool,
        gs_90892: bool,
        gs_90890: bool,
        gs_90902: bool,
        gs_90893: bool,
        gs_90903: bool,
        gs_90905: bool,
        gs_90923: bool,
        gs_90881: bool,
        gs_90913: bool,
        u__PSTATE_EL: u8,
        gs_90940: bool,
        gs_90934: bool,
        gs_90900: bool,
        gs_90920: bool,
        gs_90887: bool,
        gs_90880: bool,
        gs_90909: bool,
        gs_90926: bool,
        gs_90925: bool,
        gs_90901: bool,
        gs_90935: bool,
        u__MDCR_EL2_TPMS: bool,
        gs_90910: bool,
        gs_90912: bool,
        gs_90937: bool,
        gs_90889: bool,
        gs_90882: bool,
        gs_90936: bool,
        gs_90894: bool,
        u__SCR_EL3_NSE: bool,
        gs_90908: bool,
        gs_90899: bool,
        gs_90941: bool,
        gs_90888: bool,
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
        // C s_0_3: const #22712u : u32
        let s_0_3: u32 = 22712;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_MDCR_EL3_Type_EnPMS3(s_0_4)
        let s_0_5: bool = u_get_MDCR_EL3_Type_EnPMS3(state, tracer, s_0_4);
        // D s_0_6: write-var __MDCR_EL3_EnPMS3 <= s_0_5
        fn_state.u__MDCR_EL3_EnPMS3 = s_0_5;
        // C s_0_7: const #22712u : u32
        let s_0_7: u32 = 22712;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_MDCR_EL3_Type_NSPBE(s_0_8)
        let s_0_9: bool = u_get_MDCR_EL3_Type_NSPBE(state, tracer, s_0_8);
        // D s_0_10: write-var __MDCR_EL3_NSPBE <= s_0_9
        fn_state.u__MDCR_EL3_NSPBE = s_0_9;
        // C s_0_11: const #90704u : u32
        let s_0_11: u32 = 90704;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_SCR_EL3_Type_NSE(s_0_12)
        let s_0_13: bool = u_get_SCR_EL3_Type_NSE(state, tracer, s_0_12);
        // D s_0_14: write-var __SCR_EL3_NSE <= s_0_13
        fn_state.u__SCR_EL3_NSE = s_0_13;
        // C s_0_15: const #90704u : u32
        let s_0_15: u32 = 90704;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_SCR_EL3_Type_FGTEn2(s_0_16)
        let s_0_17: bool = u_get_SCR_EL3_Type_FGTEn2(state, tracer, s_0_16);
        // D s_0_18: write-var __SCR_EL3_FGTEn2 <= s_0_17
        fn_state.u__SCR_EL3_FGTEn2 = s_0_17;
        // C s_0_19: const #17664u : u32
        let s_0_19: u32 = 17664;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_HDFGWTR2_EL2_Type_nPMSDSFR_EL1(s_0_20)
        let s_0_21: bool = u_get_HDFGWTR2_EL2_Type_nPMSDSFR_EL1(state, tracer, s_0_20);
        // D s_0_22: write-var __HDFGWTR2_EL2_nPMSDSFR_EL1 <= s_0_21
        fn_state.u__HDFGWTR2_EL2_nPMSDSFR_EL1 = s_0_21;
        // C s_0_23: const #104880u : u32
        let s_0_23: u32 = 104880;
        // D s_0_24: read-reg s_0_23:struct
        let s_0_24: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_23 as isize);
            tracer.read_register(s_0_23 as isize, value);
            value
        };
        // D s_0_25: call _get_MDCR_EL2_Type_TPMS(s_0_24)
        let s_0_25: bool = u_get_MDCR_EL2_Type_TPMS(state, tracer, s_0_24);
        // D s_0_26: write-var __MDCR_EL2_TPMS <= s_0_25
        fn_state.u__MDCR_EL2_TPMS = s_0_25;
        // D s_0_27: read-var __PSTATE_EL:u8
        let s_0_27: u8 = fn_state.u__PSTATE_EL;
        // D s_0_28: cast zx s_0_27 -> bv
        let s_0_28: Bits = Bits::new(s_0_27 as u128, 2u16);
        // C s_0_29: const #448u : u32
        let s_0_29: u32 = 448;
        // D s_0_30: read-reg s_0_29:u8
        let s_0_30: u8 = {
            let value = state.read_register::<u8>(s_0_29 as isize);
            tracer.read_register(s_0_29 as isize, value);
            value
        };
        // D s_0_31: cast zx s_0_30 -> bv
        let s_0_31: Bits = Bits::new(s_0_30 as u128, 2u16);
        // D s_0_32: cmp-eq s_0_28 s_0_31
        let s_0_32: bool = ((s_0_28) == (s_0_31));
        // N s_0_33: branch s_0_32 b169 b1
        if s_0_32 {
            return block_169(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b73 b2
        if s_1_5 {
            return block_73(state, tracer, fn_state);
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
        // D s_5_4: call Mk_PMSDSFR_EL1_Type(s_5_3)
        let s_5_4: ProductType5c790c8ef59cc8b2 = Mk_PMSDSFR_EL1_Type(
            state,
            tracer,
            s_5_3,
        );
        // C s_5_5: const #1520u : u32
        let s_5_5: u32 = 1520;
        // N s_5_6: write-reg s_5_5 <= s_5_4
        let s_5_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_5_5 as isize, s_5_4);
            tracer.write_register(s_5_5 as isize, s_5_4);
        };
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
        // N s_6_2: branch s_6_1 b72 b7
        if s_6_1 {
            return block_72(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#90880 <= s_7_0
        fn_state.gs_90880 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#90880:u8
        let s_8_0: bool = fn_state.gs_90880;
        // N s_8_1: branch s_8_0 b71 b9
        if s_8_0 {
            return block_71(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#90881 <= s_9_0
        fn_state.gs_90881 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#90881:u8
        let s_10_0: bool = fn_state.gs_90881;
        // N s_10_1: branch s_10_0 b70 b11
        if s_10_0 {
            return block_70(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#90882 <= s_11_0
        fn_state.gs_90882 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#90882:u8
        let s_12_0: bool = fn_state.gs_90882;
        // N s_12_1: branch s_12_0 b60 b13
        if s_12_0 {
            return block_60(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#90890 <= s_13_0
        fn_state.gs_90890 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#90890:u8
        let s_14_0: bool = fn_state.gs_90890;
        // N s_14_1: branch s_14_0 b59 b15
        if s_14_0 {
            return block_59(state, tracer, fn_state);
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
        // N s_15_2: branch s_15_1 b58 b16
        if s_15_1 {
            return block_58(state, tracer, fn_state);
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
        // D s_16_1: write-var gs#90891 <= s_16_0
        fn_state.gs_90891 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#90891:u8
        let s_17_0: bool = fn_state.gs_90891;
        // N s_17_1: branch s_17_0 b57 b18
        if s_17_0 {
            return block_57(state, tracer, fn_state);
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
        // D s_18_1: write-var gs#90892 <= s_18_0
        fn_state.gs_90892 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#90892:u8
        let s_19_0: bool = fn_state.gs_90892;
        // N s_19_1: branch s_19_0 b56 b20
        if s_19_0 {
            return block_56(state, tracer, fn_state);
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
        // D s_20_1: write-var gs#90893 <= s_20_0
        fn_state.gs_90893 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#90893:u8
        let s_21_0: bool = fn_state.gs_90893;
        // N s_21_1: branch s_21_0 b55 b22
        if s_21_0 {
            return block_55(state, tracer, fn_state);
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
        // D s_22_1: write-var gs#90894 <= s_22_0
        fn_state.gs_90894 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#90894:u8
        let s_23_0: bool = fn_state.gs_90894;
        // N s_23_1: branch s_23_0 b54 b24
        if s_23_0 {
            return block_54(state, tracer, fn_state);
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
        // D s_25_1: write-var gs#90902 <= s_25_0
        fn_state.gs_90902 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#90902:u8
        let s_26_0: bool = fn_state.gs_90902;
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
        // D s_28_1: write-var gs#90903 <= s_28_0
        fn_state.gs_90903 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#90903:u8
        let s_29_0: bool = fn_state.gs_90903;
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
        // D s_30_1: read-var t:i
        let s_30_1: i128 = fn_state.t;
        // D s_30_2: call X_read(s_30_1, s_30_0)
        let s_30_2: Bits = X_read(state, tracer, s_30_1, s_30_0);
        // D s_30_3: cast reint s_30_2 -> u64
        let s_30_3: u64 = (s_30_2.value() as u64);
        // D s_30_4: call Mk_PMSDSFR_EL1_Type(s_30_3)
        let s_30_4: ProductType5c790c8ef59cc8b2 = Mk_PMSDSFR_EL1_Type(
            state,
            tracer,
            s_30_3,
        );
        // C s_30_5: const #1520u : u32
        let s_30_5: u32 = 1520;
        // N s_30_6: write-reg s_30_5 <= s_30_4
        let s_30_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_30_5 as isize, s_30_4);
            tracer.write_register(s_30_5 as isize, s_30_4);
        };
        // N s_30_7: return
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
        // D s_32_1: write-var gs#90905 <= s_32_0
        fn_state.gs_90905 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#90905:u8
        let s_33_0: bool = fn_state.gs_90905;
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
        // C s_36_0: const #() : ()
        let s_36_0: () = ();
        // S s_36_1: call EDSCR_read(s_36_0)
        let s_36_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_36_0);
        // S s_36_2: call _get_EDSCR_Type_SDD(s_36_1)
        let s_36_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_36_1);
        // S s_36_3: cast zx s_36_2 -> bv
        let s_36_3: Bits = Bits::new(s_36_2 as u128, 1u16);
        // C s_36_4: const #1u : u8
        let s_36_4: bool = true;
        // C s_36_5: cast zx s_36_4 -> bv
        let s_36_5: Bits = Bits::new(s_36_4 as u128, 1u16);
        // S s_36_6: cmp-eq s_36_3 s_36_5
        let s_36_6: bool = ((s_36_3) == (s_36_5));
        // D s_36_7: write-var gs#90905 <= s_36_6
        fn_state.gs_90905 = s_36_6;
        // N s_36_8: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var __MDCR_EL3_EnPMS3:u8
        let s_37_0: bool = fn_state.u__MDCR_EL3_EnPMS3;
        // D s_37_1: cast zx s_37_0 -> bv
        let s_37_1: Bits = Bits::new(s_37_0 as u128, 1u16);
        // C s_37_2: const #0u : u8
        let s_37_2: bool = false;
        // C s_37_3: cast zx s_37_2 -> bv
        let s_37_3: Bits = Bits::new(s_37_2 as u128, 1u16);
        // D s_37_4: cmp-eq s_37_1 s_37_3
        let s_37_4: bool = ((s_37_1) == (s_37_3));
        // D s_37_5: write-var gs#90903 <= s_37_4
        fn_state.gs_90903 = s_37_4;
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
        // D s_39_1: write-var gs#90906 <= s_39_0
        fn_state.gs_90906 = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_40_0: read-var gs#90906:u8
        let s_40_0: bool = fn_state.gs_90906;
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
        // C s_43_0: const #() : ()
        let s_43_0: () = ();
        // S s_43_1: call EDSCR_read(s_43_0)
        let s_43_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_43_0);
        // S s_43_2: call _get_EDSCR_Type_SDD(s_43_1)
        let s_43_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_43_1);
        // S s_43_3: cast zx s_43_2 -> bv
        let s_43_3: Bits = Bits::new(s_43_2 as u128, 1u16);
        // C s_43_4: const #1u : u8
        let s_43_4: bool = true;
        // C s_43_5: cast zx s_43_4 -> bv
        let s_43_5: Bits = Bits::new(s_43_4 as u128, 1u16);
        // S s_43_6: cmp-eq s_43_3 s_43_5
        let s_43_6: bool = ((s_43_3) == (s_43_5));
        // D s_43_7: write-var gs#90906 <= s_43_6
        fn_state.gs_90906 = s_43_6;
        // N s_43_8: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_44_0: const #22712u : u32
        let s_44_0: u32 = 22712;
        // D s_44_1: read-reg s_44_0:struct
        let s_44_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_44_0 as isize);
            tracer.read_register(s_44_0 as isize, value);
            value
        };
        // D s_44_2: call _get_MDCR_EL3_Type_NSPB(s_44_1)
        let s_44_2: u8 = u_get_MDCR_EL3_Type_NSPB(state, tracer, s_44_1);
        // C s_44_3: const #0s : i
        let s_44_3: i128 = 0;
        // D s_44_4: cast zx s_44_2 -> bv
        let s_44_4: Bits = Bits::new(s_44_2 as u128, 2u16);
        // C s_44_5: const #1u : u64
        let s_44_5: u64 = 1;
        // D s_44_6: bit-extract s_44_4 s_44_3 s_44_5
        let s_44_6: Bits = (Bits::new(
            ((s_44_4) >> (s_44_3)).value(),
            u16::try_from(s_44_5).unwrap(),
        ));
        // D s_44_7: cast reint s_44_6 -> u8
        let s_44_7: bool = ((s_44_6.value()) != 0);
        // C s_44_8: const #0s : i
        let s_44_8: i128 = 0;
        // C s_44_9: const #0u : u64
        let s_44_9: u64 = 0;
        // D s_44_10: cast zx s_44_7 -> u64
        let s_44_10: u64 = (s_44_7 as u64);
        // C s_44_11: const #1u : u64
        let s_44_11: u64 = 1;
        // D s_44_12: and s_44_10 s_44_11
        let s_44_12: u64 = ((s_44_10) & (s_44_11));
        // D s_44_13: cmp-eq s_44_12 s_44_11
        let s_44_13: bool = ((s_44_12) == (s_44_11));
        // D s_44_14: lsl s_44_10 s_44_8
        let s_44_14: u64 = s_44_10 << s_44_8;
        // D s_44_15: or s_44_9 s_44_14
        let s_44_15: u64 = ((s_44_9) | (s_44_14));
        // D s_44_16: cmpl s_44_14
        let s_44_16: u64 = !s_44_14;
        // D s_44_17: and s_44_9 s_44_16
        let s_44_17: u64 = ((s_44_9) & (s_44_16));
        // D s_44_18: select s_44_13 s_44_15 s_44_17
        let s_44_18: u64 = if s_44_13 { s_44_15 } else { s_44_17 };
        // D s_44_19: cast trunc s_44_18 -> u8
        let s_44_19: bool = ((s_44_18) != 0);
        // D s_44_20: cast zx s_44_19 -> bv
        let s_44_20: Bits = Bits::new(s_44_19 as u128, 1u16);
        // C s_44_21: const #0u : u8
        let s_44_21: bool = false;
        // C s_44_22: cast zx s_44_21 -> bv
        let s_44_22: Bits = Bits::new(s_44_21 as u128, 1u16);
        // D s_44_23: cmp-eq s_44_20 s_44_22
        let s_44_23: bool = ((s_44_20) == (s_44_22));
        // N s_44_24: branch s_44_23 b53 b45
        if s_44_23 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #22712u : u32
        let s_45_0: u32 = 22712;
        // D s_45_1: read-reg s_45_0:struct
        let s_45_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_45_0 as isize);
            tracer.read_register(s_45_0 as isize, value);
            value
        };
        // D s_45_2: call _get_MDCR_EL3_Type_NSPB(s_45_1)
        let s_45_2: u8 = u_get_MDCR_EL3_Type_NSPB(state, tracer, s_45_1);
        // C s_45_3: const #1s : i
        let s_45_3: i128 = 1;
        // D s_45_4: cast zx s_45_2 -> bv
        let s_45_4: Bits = Bits::new(s_45_2 as u128, 2u16);
        // C s_45_5: const #1u : u64
        let s_45_5: u64 = 1;
        // D s_45_6: bit-extract s_45_4 s_45_3 s_45_5
        let s_45_6: Bits = (Bits::new(
            ((s_45_4) >> (s_45_3)).value(),
            u16::try_from(s_45_5).unwrap(),
        ));
        // D s_45_7: cast reint s_45_6 -> u8
        let s_45_7: bool = ((s_45_6.value()) != 0);
        // C s_45_8: const #0s : i
        let s_45_8: i128 = 0;
        // C s_45_9: const #0u : u64
        let s_45_9: u64 = 0;
        // D s_45_10: cast zx s_45_7 -> u64
        let s_45_10: u64 = (s_45_7 as u64);
        // C s_45_11: const #1u : u64
        let s_45_11: u64 = 1;
        // D s_45_12: and s_45_10 s_45_11
        let s_45_12: u64 = ((s_45_10) & (s_45_11));
        // D s_45_13: cmp-eq s_45_12 s_45_11
        let s_45_13: bool = ((s_45_12) == (s_45_11));
        // D s_45_14: lsl s_45_10 s_45_8
        let s_45_14: u64 = s_45_10 << s_45_8;
        // D s_45_15: or s_45_9 s_45_14
        let s_45_15: u64 = ((s_45_9) | (s_45_14));
        // D s_45_16: cmpl s_45_14
        let s_45_16: u64 = !s_45_14;
        // D s_45_17: and s_45_9 s_45_16
        let s_45_17: u64 = ((s_45_9) & (s_45_16));
        // D s_45_18: select s_45_13 s_45_15 s_45_17
        let s_45_18: u64 = if s_45_13 { s_45_15 } else { s_45_17 };
        // D s_45_19: cast trunc s_45_18 -> u8
        let s_45_19: bool = ((s_45_18) != 0);
        // C s_45_20: const #90704u : u32
        let s_45_20: u32 = 90704;
        // D s_45_21: read-reg s_45_20:struct
        let s_45_21: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_45_20 as isize);
            tracer.read_register(s_45_20 as isize, value);
            value
        };
        // D s_45_22: call _get_SCR_EL3_Type_NS(s_45_21)
        let s_45_22: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_45_21);
        // D s_45_23: cast zx s_45_19 -> bv
        let s_45_23: Bits = Bits::new(s_45_19 as u128, 1u16);
        // D s_45_24: cast zx s_45_22 -> bv
        let s_45_24: Bits = Bits::new(s_45_22 as u128, 1u16);
        // D s_45_25: cmp-ne s_45_23 s_45_24
        let s_45_25: bool = ((s_45_23) != (s_45_24));
        // D s_45_26: write-var gs#90899 <= s_45_25
        fn_state.gs_90899 = s_45_25;
        // N s_45_27: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var gs#90899:u8
        let s_46_0: bool = fn_state.gs_90899;
        // N s_46_1: branch s_46_0 b52 b47
        if s_46_0 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #232u : u32
        let s_47_0: u32 = 232;
        // S s_47_1: call IsFeatureImplemented(s_47_0)
        let s_47_1: bool = IsFeatureImplemented(state, tracer, s_47_0);
        // N s_47_2: branch s_47_1 b51 b48
        if s_47_1 {
            return block_51(state, tracer, fn_state);
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
        // D s_48_1: write-var gs#90900 <= s_48_0
        fn_state.gs_90900 = s_48_0;
        // N s_48_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var gs#90900:u8
        let s_49_0: bool = fn_state.gs_90900;
        // D s_49_1: write-var gs#90901 <= s_49_0
        fn_state.gs_90901 = s_49_0;
        // N s_49_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var gs#90901:u8
        let s_50_0: bool = fn_state.gs_90901;
        // D s_50_1: write-var gs#90902 <= s_50_0
        fn_state.gs_90902 = s_50_0;
        // N s_50_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var __MDCR_EL3_NSPBE:u8
        let s_51_0: bool = fn_state.u__MDCR_EL3_NSPBE;
        // D s_51_1: cast zx s_51_0 -> bv
        let s_51_1: Bits = Bits::new(s_51_0 as u128, 1u16);
        // D s_51_2: read-var __SCR_EL3_NSE:u8
        let s_51_2: bool = fn_state.u__SCR_EL3_NSE;
        // D s_51_3: cast zx s_51_2 -> bv
        let s_51_3: Bits = Bits::new(s_51_2 as u128, 1u16);
        // D s_51_4: cmp-ne s_51_1 s_51_3
        let s_51_4: bool = ((s_51_1) != (s_51_3));
        // D s_51_5: write-var gs#90900 <= s_51_4
        fn_state.gs_90900 = s_51_4;
        // N s_51_6: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #1u : u8
        let s_52_0: bool = true;
        // D s_52_1: write-var gs#90901 <= s_52_0
        fn_state.gs_90901 = s_52_0;
        // N s_52_2: jump b50
        return block_50(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #1u : u8
        let s_53_0: bool = true;
        // D s_53_1: write-var gs#90899 <= s_53_0
        fn_state.gs_90899 = s_53_0;
        // N s_53_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_54_0: panic
        panic!("{:?}", ());
        // N s_54_1: return
        return;
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_55_0: read-var __MDCR_EL3_EnPMS3:u8
        let s_55_0: bool = fn_state.u__MDCR_EL3_EnPMS3;
        // D s_55_1: cast zx s_55_0 -> bv
        let s_55_1: Bits = Bits::new(s_55_0 as u128, 1u16);
        // C s_55_2: const #0u : u8
        let s_55_2: bool = false;
        // C s_55_3: cast zx s_55_2 -> bv
        let s_55_3: Bits = Bits::new(s_55_2 as u128, 1u16);
        // D s_55_4: cmp-eq s_55_1 s_55_3
        let s_55_4: bool = ((s_55_1) == (s_55_3));
        // D s_55_5: write-var gs#90894 <= s_55_4
        fn_state.gs_90894 = s_55_4;
        // N s_55_6: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_56_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_56_1: call __IMPDEF_boolean(s_56_0)
        let s_56_1: bool = u__IMPDEF_boolean(state, tracer, s_56_0);
        // D s_56_2: write-var gs#90893 <= s_56_1
        fn_state.gs_90893 = s_56_1;
        // N s_56_3: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #() : ()
        let s_57_0: () = ();
        // S s_57_1: call EDSCR_read(s_57_0)
        let s_57_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_57_0);
        // S s_57_2: call _get_EDSCR_Type_SDD(s_57_1)
        let s_57_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_57_1);
        // S s_57_3: cast zx s_57_2 -> bv
        let s_57_3: Bits = Bits::new(s_57_2 as u128, 1u16);
        // C s_57_4: const #1u : u8
        let s_57_4: bool = true;
        // C s_57_5: cast zx s_57_4 -> bv
        let s_57_5: Bits = Bits::new(s_57_4 as u128, 1u16);
        // S s_57_6: cmp-eq s_57_3 s_57_5
        let s_57_6: bool = ((s_57_3) == (s_57_5));
        // D s_57_7: write-var gs#90892 <= s_57_6
        fn_state.gs_90892 = s_57_6;
        // N s_57_8: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #424u : u32
        let s_58_0: u32 = 424;
        // D s_58_1: read-reg s_58_0:u8
        let s_58_1: u8 = {
            let value = state.read_register::<u8>(s_58_0 as isize);
            tracer.read_register(s_58_0 as isize, value);
            value
        };
        // C s_58_2: const #2u : u8
        let s_58_2: u8 = 2;
        // D s_58_3: cmp-lt s_58_1 s_58_2
        let s_58_3: bool = ((s_58_1) < (s_58_2));
        // D s_58_4: write-var gs#90891 <= s_58_3
        fn_state.gs_90891 = s_58_3;
        // N s_58_5: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_59_0: panic
        panic!("{:?}", ());
        // N s_59_1: return
        return;
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #22712u : u32
        let s_60_0: u32 = 22712;
        // D s_60_1: read-reg s_60_0:struct
        let s_60_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_60_0 as isize);
            tracer.read_register(s_60_0 as isize, value);
            value
        };
        // D s_60_2: call _get_MDCR_EL3_Type_NSPB(s_60_1)
        let s_60_2: u8 = u_get_MDCR_EL3_Type_NSPB(state, tracer, s_60_1);
        // C s_60_3: const #0s : i
        let s_60_3: i128 = 0;
        // D s_60_4: cast zx s_60_2 -> bv
        let s_60_4: Bits = Bits::new(s_60_2 as u128, 2u16);
        // C s_60_5: const #1u : u64
        let s_60_5: u64 = 1;
        // D s_60_6: bit-extract s_60_4 s_60_3 s_60_5
        let s_60_6: Bits = (Bits::new(
            ((s_60_4) >> (s_60_3)).value(),
            u16::try_from(s_60_5).unwrap(),
        ));
        // D s_60_7: cast reint s_60_6 -> u8
        let s_60_7: bool = ((s_60_6.value()) != 0);
        // C s_60_8: const #0s : i
        let s_60_8: i128 = 0;
        // C s_60_9: const #0u : u64
        let s_60_9: u64 = 0;
        // D s_60_10: cast zx s_60_7 -> u64
        let s_60_10: u64 = (s_60_7 as u64);
        // C s_60_11: const #1u : u64
        let s_60_11: u64 = 1;
        // D s_60_12: and s_60_10 s_60_11
        let s_60_12: u64 = ((s_60_10) & (s_60_11));
        // D s_60_13: cmp-eq s_60_12 s_60_11
        let s_60_13: bool = ((s_60_12) == (s_60_11));
        // D s_60_14: lsl s_60_10 s_60_8
        let s_60_14: u64 = s_60_10 << s_60_8;
        // D s_60_15: or s_60_9 s_60_14
        let s_60_15: u64 = ((s_60_9) | (s_60_14));
        // D s_60_16: cmpl s_60_14
        let s_60_16: u64 = !s_60_14;
        // D s_60_17: and s_60_9 s_60_16
        let s_60_17: u64 = ((s_60_9) & (s_60_16));
        // D s_60_18: select s_60_13 s_60_15 s_60_17
        let s_60_18: u64 = if s_60_13 { s_60_15 } else { s_60_17 };
        // D s_60_19: cast trunc s_60_18 -> u8
        let s_60_19: bool = ((s_60_18) != 0);
        // D s_60_20: cast zx s_60_19 -> bv
        let s_60_20: Bits = Bits::new(s_60_19 as u128, 1u16);
        // C s_60_21: const #0u : u8
        let s_60_21: bool = false;
        // C s_60_22: cast zx s_60_21 -> bv
        let s_60_22: Bits = Bits::new(s_60_21 as u128, 1u16);
        // D s_60_23: cmp-eq s_60_20 s_60_22
        let s_60_23: bool = ((s_60_20) == (s_60_22));
        // N s_60_24: branch s_60_23 b69 b61
        if s_60_23 {
            return block_69(state, tracer, fn_state);
        } else {
            return block_61(state, tracer, fn_state);
        };
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #22712u : u32
        let s_61_0: u32 = 22712;
        // D s_61_1: read-reg s_61_0:struct
        let s_61_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_61_0 as isize);
            tracer.read_register(s_61_0 as isize, value);
            value
        };
        // D s_61_2: call _get_MDCR_EL3_Type_NSPB(s_61_1)
        let s_61_2: u8 = u_get_MDCR_EL3_Type_NSPB(state, tracer, s_61_1);
        // C s_61_3: const #1s : i
        let s_61_3: i128 = 1;
        // D s_61_4: cast zx s_61_2 -> bv
        let s_61_4: Bits = Bits::new(s_61_2 as u128, 2u16);
        // C s_61_5: const #1u : u64
        let s_61_5: u64 = 1;
        // D s_61_6: bit-extract s_61_4 s_61_3 s_61_5
        let s_61_6: Bits = (Bits::new(
            ((s_61_4) >> (s_61_3)).value(),
            u16::try_from(s_61_5).unwrap(),
        ));
        // D s_61_7: cast reint s_61_6 -> u8
        let s_61_7: bool = ((s_61_6.value()) != 0);
        // C s_61_8: const #0s : i
        let s_61_8: i128 = 0;
        // C s_61_9: const #0u : u64
        let s_61_9: u64 = 0;
        // D s_61_10: cast zx s_61_7 -> u64
        let s_61_10: u64 = (s_61_7 as u64);
        // C s_61_11: const #1u : u64
        let s_61_11: u64 = 1;
        // D s_61_12: and s_61_10 s_61_11
        let s_61_12: u64 = ((s_61_10) & (s_61_11));
        // D s_61_13: cmp-eq s_61_12 s_61_11
        let s_61_13: bool = ((s_61_12) == (s_61_11));
        // D s_61_14: lsl s_61_10 s_61_8
        let s_61_14: u64 = s_61_10 << s_61_8;
        // D s_61_15: or s_61_9 s_61_14
        let s_61_15: u64 = ((s_61_9) | (s_61_14));
        // D s_61_16: cmpl s_61_14
        let s_61_16: u64 = !s_61_14;
        // D s_61_17: and s_61_9 s_61_16
        let s_61_17: u64 = ((s_61_9) & (s_61_16));
        // D s_61_18: select s_61_13 s_61_15 s_61_17
        let s_61_18: u64 = if s_61_13 { s_61_15 } else { s_61_17 };
        // D s_61_19: cast trunc s_61_18 -> u8
        let s_61_19: bool = ((s_61_18) != 0);
        // C s_61_20: const #90704u : u32
        let s_61_20: u32 = 90704;
        // D s_61_21: read-reg s_61_20:struct
        let s_61_21: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_61_20 as isize);
            tracer.read_register(s_61_20 as isize, value);
            value
        };
        // D s_61_22: call _get_SCR_EL3_Type_NS(s_61_21)
        let s_61_22: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_61_21);
        // D s_61_23: cast zx s_61_19 -> bv
        let s_61_23: Bits = Bits::new(s_61_19 as u128, 1u16);
        // D s_61_24: cast zx s_61_22 -> bv
        let s_61_24: Bits = Bits::new(s_61_22 as u128, 1u16);
        // D s_61_25: cmp-ne s_61_23 s_61_24
        let s_61_25: bool = ((s_61_23) != (s_61_24));
        // D s_61_26: write-var gs#90887 <= s_61_25
        fn_state.gs_90887 = s_61_25;
        // N s_61_27: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var gs#90887:u8
        let s_62_0: bool = fn_state.gs_90887;
        // N s_62_1: branch s_62_0 b68 b63
        if s_62_0 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_63(state, tracer, fn_state);
        };
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #232u : u32
        let s_63_0: u32 = 232;
        // S s_63_1: call IsFeatureImplemented(s_63_0)
        let s_63_1: bool = IsFeatureImplemented(state, tracer, s_63_0);
        // N s_63_2: branch s_63_1 b67 b64
        if s_63_1 {
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
        // C s_64_0: const #0u : u8
        let s_64_0: bool = false;
        // D s_64_1: write-var gs#90888 <= s_64_0
        fn_state.gs_90888 = s_64_0;
        // N s_64_2: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_65_0: read-var gs#90888:u8
        let s_65_0: bool = fn_state.gs_90888;
        // D s_65_1: write-var gs#90889 <= s_65_0
        fn_state.gs_90889 = s_65_0;
        // N s_65_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_66_0: read-var gs#90889:u8
        let s_66_0: bool = fn_state.gs_90889;
        // D s_66_1: write-var gs#90890 <= s_66_0
        fn_state.gs_90890 = s_66_0;
        // N s_66_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var __MDCR_EL3_NSPBE:u8
        let s_67_0: bool = fn_state.u__MDCR_EL3_NSPBE;
        // D s_67_1: cast zx s_67_0 -> bv
        let s_67_1: Bits = Bits::new(s_67_0 as u128, 1u16);
        // D s_67_2: read-var __SCR_EL3_NSE:u8
        let s_67_2: bool = fn_state.u__SCR_EL3_NSE;
        // D s_67_3: cast zx s_67_2 -> bv
        let s_67_3: Bits = Bits::new(s_67_2 as u128, 1u16);
        // D s_67_4: cmp-ne s_67_1 s_67_3
        let s_67_4: bool = ((s_67_1) != (s_67_3));
        // D s_67_5: write-var gs#90888 <= s_67_4
        fn_state.gs_90888 = s_67_4;
        // N s_67_6: jump b65
        return block_65(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #1u : u8
        let s_68_0: bool = true;
        // D s_68_1: write-var gs#90889 <= s_68_0
        fn_state.gs_90889 = s_68_0;
        // N s_68_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #1u : u8
        let s_69_0: bool = true;
        // D s_69_1: write-var gs#90887 <= s_69_0
        fn_state.gs_90887 = s_69_0;
        // N s_69_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_70_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_70_1: call __IMPDEF_boolean(s_70_0)
        let s_70_1: bool = u__IMPDEF_boolean(state, tracer, s_70_0);
        // D s_70_2: write-var gs#90882 <= s_70_1
        fn_state.gs_90882 = s_70_1;
        // N s_70_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #() : ()
        let s_71_0: () = ();
        // S s_71_1: call EDSCR_read(s_71_0)
        let s_71_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_71_0);
        // S s_71_2: call _get_EDSCR_Type_SDD(s_71_1)
        let s_71_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_71_1);
        // S s_71_3: cast zx s_71_2 -> bv
        let s_71_3: Bits = Bits::new(s_71_2 as u128, 1u16);
        // C s_71_4: const #1u : u8
        let s_71_4: bool = true;
        // C s_71_5: cast zx s_71_4 -> bv
        let s_71_5: Bits = Bits::new(s_71_4 as u128, 1u16);
        // S s_71_6: cmp-eq s_71_3 s_71_5
        let s_71_6: bool = ((s_71_3) == (s_71_5));
        // D s_71_7: write-var gs#90881 <= s_71_6
        fn_state.gs_90881 = s_71_6;
        // N s_71_8: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #424u : u32
        let s_72_0: u32 = 424;
        // D s_72_1: read-reg s_72_0:u8
        let s_72_1: u8 = {
            let value = state.read_register::<u8>(s_72_0 as isize);
            tracer.read_register(s_72_0 as isize, value);
            value
        };
        // C s_72_2: const #2u : u8
        let s_72_2: u8 = 2;
        // D s_72_3: cmp-lt s_72_1 s_72_2
        let s_72_3: bool = ((s_72_1) < (s_72_2));
        // D s_72_4: write-var gs#90880 <= s_72_3
        fn_state.gs_90880 = s_72_3;
        // N s_72_5: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #() : ()
        let s_73_0: () = ();
        // S s_73_1: call Halted(s_73_0)
        let s_73_1: bool = Halted(state, tracer, s_73_0);
        // N s_73_2: branch s_73_1 b168 b74
        if s_73_1 {
            return block_168(state, tracer, fn_state);
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
        // D s_74_1: write-var gs#90907 <= s_74_0
        fn_state.gs_90907 = s_74_0;
        // N s_74_2: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_75_0: read-var gs#90907:u8
        let s_75_0: bool = fn_state.gs_90907;
        // N s_75_1: branch s_75_0 b167 b76
        if s_75_0 {
            return block_167(state, tracer, fn_state);
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
        // D s_76_1: write-var gs#90908 <= s_76_0
        fn_state.gs_90908 = s_76_0;
        // N s_76_2: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_77_0: read-var gs#90908:u8
        let s_77_0: bool = fn_state.gs_90908;
        // N s_77_1: branch s_77_0 b166 b78
        if s_77_0 {
            return block_166(state, tracer, fn_state);
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
        // D s_78_1: write-var gs#90909 <= s_78_0
        fn_state.gs_90909 = s_78_0;
        // N s_78_2: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_79_0: read-var gs#90909:u8
        let s_79_0: bool = fn_state.gs_90909;
        // N s_79_1: branch s_79_0 b165 b80
        if s_79_0 {
            return block_165(state, tracer, fn_state);
        } else {
            return block_80(state, tracer, fn_state);
        };
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #0u : u8
        let s_80_0: bool = false;
        // D s_80_1: write-var gs#90910 <= s_80_0
        fn_state.gs_90910 = s_80_0;
        // N s_80_2: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_81_0: read-var gs#90910:u8
        let s_81_0: bool = fn_state.gs_90910;
        // N s_81_1: branch s_81_0 b164 b82
        if s_81_0 {
            return block_164(state, tracer, fn_state);
        } else {
            return block_82(state, tracer, fn_state);
        };
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #() : ()
        let s_82_0: () = ();
        // S s_82_1: call Halted(s_82_0)
        let s_82_1: bool = Halted(state, tracer, s_82_0);
        // N s_82_2: branch s_82_1 b163 b83
        if s_82_1 {
            return block_163(state, tracer, fn_state);
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
        // D s_83_1: write-var gs#90911 <= s_83_0
        fn_state.gs_90911 = s_83_0;
        // N s_83_2: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_84_0: read-var gs#90911:u8
        let s_84_0: bool = fn_state.gs_90911;
        // N s_84_1: branch s_84_0 b162 b85
        if s_84_0 {
            return block_162(state, tracer, fn_state);
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
        // D s_85_1: write-var gs#90912 <= s_85_0
        fn_state.gs_90912 = s_85_0;
        // N s_85_2: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_86_0: read-var gs#90912:u8
        let s_86_0: bool = fn_state.gs_90912;
        // N s_86_1: branch s_86_0 b161 b87
        if s_86_0 {
            return block_161(state, tracer, fn_state);
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
        // D s_87_1: write-var gs#90913 <= s_87_0
        fn_state.gs_90913 = s_87_0;
        // N s_87_2: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_88_0: read-var gs#90913:u8
        let s_88_0: bool = fn_state.gs_90913;
        // N s_88_1: branch s_88_0 b151 b89
        if s_88_0 {
            return block_151(state, tracer, fn_state);
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
        // D s_89_1: write-var gs#90921 <= s_89_0
        fn_state.gs_90921 = s_89_0;
        // N s_89_2: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_90_0: read-var gs#90921:u8
        let s_90_0: bool = fn_state.gs_90921;
        // N s_90_1: branch s_90_0 b150 b91
        if s_90_0 {
            return block_150(state, tracer, fn_state);
        } else {
            return block_91(state, tracer, fn_state);
        };
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
        // N s_91_2: branch s_91_1 b149 b92
        if s_91_1 {
            return block_149(state, tracer, fn_state);
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
        // D s_92_1: write-var gs#90922 <= s_92_0
        fn_state.gs_90922 = s_92_0;
        // N s_92_2: jump b93
        return block_93(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_93_0: read-var gs#90922:u8
        let s_93_0: bool = fn_state.gs_90922;
        // N s_93_1: branch s_93_0 b148 b94
        if s_93_0 {
            return block_148(state, tracer, fn_state);
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
        // D s_94_1: write-var gs#90923 <= s_94_0
        fn_state.gs_90923 = s_94_0;
        // N s_94_2: jump b95
        return block_95(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_95_0: read-var gs#90923:u8
        let s_95_0: bool = fn_state.gs_90923;
        // N s_95_1: branch s_95_0 b147 b96
        if s_95_0 {
            return block_147(state, tracer, fn_state);
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
        // D s_96_1: write-var gs#90924 <= s_96_0
        fn_state.gs_90924 = s_96_0;
        // N s_96_2: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_97_0: read-var gs#90924:u8
        let s_97_0: bool = fn_state.gs_90924;
        // N s_97_1: branch s_97_0 b146 b98
        if s_97_0 {
            return block_146(state, tracer, fn_state);
        } else {
            return block_98(state, tracer, fn_state);
        };
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_98_0: const #() : ()
        let s_98_0: () = ();
        // S s_98_1: call EL2Enabled(s_98_0)
        let s_98_1: bool = EL2Enabled(state, tracer, s_98_0);
        // N s_98_2: branch s_98_1 b145 b99
        if s_98_1 {
            return block_145(state, tracer, fn_state);
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
        // D s_99_1: write-var gs#90925 <= s_99_0
        fn_state.gs_90925 = s_99_0;
        // N s_99_2: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_100_0: read-var gs#90925:u8
        let s_100_0: bool = fn_state.gs_90925;
        // N s_100_1: branch s_100_0 b144 b101
        if s_100_0 {
            return block_144(state, tracer, fn_state);
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
        // D s_101_1: write-var gs#90926 <= s_101_0
        fn_state.gs_90926 = s_101_0;
        // N s_101_2: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_102_0: read-var gs#90926:u8
        let s_102_0: bool = fn_state.gs_90926;
        // N s_102_1: branch s_102_0 b143 b103
        if s_102_0 {
            return block_143(state, tracer, fn_state);
        } else {
            return block_103(state, tracer, fn_state);
        };
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_103_0: const #() : ()
        let s_103_0: () = ();
        // S s_103_1: call EL2Enabled(s_103_0)
        let s_103_1: bool = EL2Enabled(state, tracer, s_103_0);
        // N s_103_2: branch s_103_1 b142 b104
        if s_103_1 {
            return block_142(state, tracer, fn_state);
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
        // D s_104_1: write-var gs#90927 <= s_104_0
        fn_state.gs_90927 = s_104_0;
        // N s_104_2: jump b105
        return block_105(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_105_0: read-var gs#90927:u8
        let s_105_0: bool = fn_state.gs_90927;
        // N s_105_1: branch s_105_0 b141 b106
        if s_105_0 {
            return block_141(state, tracer, fn_state);
        } else {
            return block_106(state, tracer, fn_state);
        };
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_106_0: const #424u : u32
        let s_106_0: u32 = 424;
        // D s_106_1: read-reg s_106_0:u8
        let s_106_1: u8 = {
            let value = state.read_register::<u8>(s_106_0 as isize);
            tracer.read_register(s_106_0 as isize, value);
            value
        };
        // C s_106_2: const #2u : u8
        let s_106_2: u8 = 2;
        // D s_106_3: cmp-lt s_106_1 s_106_2
        let s_106_3: bool = ((s_106_1) < (s_106_2));
        // N s_106_4: branch s_106_3 b140 b107
        if s_106_3 {
            return block_140(state, tracer, fn_state);
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
        // D s_107_1: write-var gs#90928 <= s_107_0
        fn_state.gs_90928 = s_107_0;
        // N s_107_2: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_108_0: read-var gs#90928:u8
        let s_108_0: bool = fn_state.gs_90928;
        // N s_108_1: branch s_108_0 b134 b109
        if s_108_0 {
            return block_134(state, tracer, fn_state);
        } else {
            return block_109(state, tracer, fn_state);
        };
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_109_0: const #424u : u32
        let s_109_0: u32 = 424;
        // D s_109_1: read-reg s_109_0:u8
        let s_109_1: u8 = {
            let value = state.read_register::<u8>(s_109_0 as isize);
            tracer.read_register(s_109_0 as isize, value);
            value
        };
        // C s_109_2: const #2u : u8
        let s_109_2: u8 = 2;
        // D s_109_3: cmp-lt s_109_1 s_109_2
        let s_109_3: bool = ((s_109_1) < (s_109_2));
        // N s_109_4: branch s_109_3 b124 b110
        if s_109_3 {
            return block_124(state, tracer, fn_state);
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
        // D s_110_1: write-var gs#90936 <= s_110_0
        fn_state.gs_90936 = s_110_0;
        // N s_110_2: jump b111
        return block_111(state, tracer, fn_state);
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_111_0: read-var gs#90936:u8
        let s_111_0: bool = fn_state.gs_90936;
        // N s_111_1: branch s_111_0 b118 b112
        if s_111_0 {
            return block_118(state, tracer, fn_state);
        } else {
            return block_112(state, tracer, fn_state);
        };
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_112_0: const #() : ()
        let s_112_0: () = ();
        // S s_112_1: call EL2Enabled(s_112_0)
        let s_112_1: bool = EL2Enabled(state, tracer, s_112_0);
        // N s_112_2: branch s_112_1 b117 b113
        if s_112_1 {
            return block_117(state, tracer, fn_state);
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
        // D s_113_1: write-var gs#90937 <= s_113_0
        fn_state.gs_90937 = s_113_0;
        // N s_113_2: jump b114
        return block_114(state, tracer, fn_state);
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_114_0: read-var gs#90937:u8
        let s_114_0: bool = fn_state.gs_90937;
        // N s_114_1: branch s_114_0 b116 b115
        if s_114_0 {
            return block_116(state, tracer, fn_state);
        } else {
            return block_115(state, tracer, fn_state);
        };
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_115_0: const #64s : i64
        let s_115_0: i64 = 64;
        // D s_115_1: read-var t:i
        let s_115_1: i128 = fn_state.t;
        // D s_115_2: call X_read(s_115_1, s_115_0)
        let s_115_2: Bits = X_read(state, tracer, s_115_1, s_115_0);
        // D s_115_3: cast reint s_115_2 -> u64
        let s_115_3: u64 = (s_115_2.value() as u64);
        // D s_115_4: call Mk_PMSDSFR_EL1_Type(s_115_3)
        let s_115_4: ProductType5c790c8ef59cc8b2 = Mk_PMSDSFR_EL1_Type(
            state,
            tracer,
            s_115_3,
        );
        // C s_115_5: const #1520u : u32
        let s_115_5: u32 = 1520;
        // N s_115_6: write-reg s_115_5 <= s_115_4
        let s_115_6: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_115_5 as isize, s_115_4);
            tracer.write_register(s_115_5 as isize, s_115_4);
        };
        // N s_115_7: return
        return;
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_116_0: const #2136u : u12
        let s_116_0: u16 = 2136;
        // C s_116_1: cast zx s_116_0 -> bv
        let s_116_1: Bits = Bits::new(s_116_0 as u128, 12u16);
        // C s_116_2: cast zx s_116_1 -> i
        let s_116_2: i128 = (s_116_1.value() as i128);
        // C s_116_3: cast reint s_116_2 -> i64
        let s_116_3: i64 = (s_116_2 as i64);
        // C s_116_4: const #64s : i64
        let s_116_4: i64 = 64;
        // D s_116_5: read-var t:i
        let s_116_5: i128 = fn_state.t;
        // D s_116_6: call X_read(s_116_5, s_116_4)
        let s_116_6: Bits = X_read(state, tracer, s_116_5, s_116_4);
        // D s_116_7: cast reint s_116_6 -> u64
        let s_116_7: u64 = (s_116_6.value() as u64);
        // C s_116_8: cast zx s_116_3 -> i
        let s_116_8: i128 = (i128::try_from(s_116_3).unwrap());
        // D s_116_9: call NVMem_set(s_116_8, s_116_7)
        let s_116_9: () = NVMem_set(state, tracer, s_116_8, s_116_7);
        // N s_116_10: return
        return;
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_117_0: const #102552u : u32
        let s_117_0: u32 = 102552;
        // D s_117_1: read-reg s_117_0:struct
        let s_117_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_117_0 as isize);
            tracer.read_register(s_117_0 as isize, value);
            value
        };
        // D s_117_2: call _get_HCR_EL2_Type_NV2(s_117_1)
        let s_117_2: bool = u_get_HCR_EL2_Type_NV2(state, tracer, s_117_1);
        // C s_117_3: const #102552u : u32
        let s_117_3: u32 = 102552;
        // D s_117_4: read-reg s_117_3:struct
        let s_117_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_117_3 as isize);
            tracer.read_register(s_117_3 as isize, value);
            value
        };
        // D s_117_5: call _get_HCR_EL2_Type_NV(s_117_4)
        let s_117_5: bool = u_get_HCR_EL2_Type_NV(state, tracer, s_117_4);
        // D s_117_6: cast zx s_117_2 -> bv
        let s_117_6: Bits = Bits::new(s_117_2 as u128, 1u16);
        // D s_117_7: cast zx s_117_5 -> bv
        let s_117_7: Bits = Bits::new(s_117_5 as u128, 1u16);
        // D s_117_8: cast reint s_117_6 -> u128
        let s_117_8: u128 = (s_117_6.value() as u128);
        // D s_117_9: size-of s_117_6
        let s_117_9: u16 = s_117_6.length();
        // D s_117_10: cast reint s_117_7 -> u128
        let s_117_10: u128 = (s_117_7.value() as u128);
        // D s_117_11: size-of s_117_7
        let s_117_11: u16 = s_117_7.length();
        // D s_117_12: lsl s_117_8 s_117_11
        let s_117_12: u128 = s_117_8 << s_117_11;
        // D s_117_13: or s_117_12 s_117_10
        let s_117_13: u128 = ((s_117_12) | (s_117_10));
        // D s_117_14: add s_117_9 s_117_11
        let s_117_14: u16 = (s_117_9 + s_117_11);
        // D s_117_15: create-bits s_117_13 s_117_14
        let s_117_15: Bits = Bits::new(s_117_13, s_117_14);
        // D s_117_16: cast reint s_117_15 -> u8
        let s_117_16: u8 = (s_117_15.value() as u8);
        // D s_117_17: cast zx s_117_16 -> bv
        let s_117_17: Bits = Bits::new(s_117_16 as u128, 2u16);
        // C s_117_18: const #3u : u8
        let s_117_18: u8 = 3;
        // C s_117_19: cast zx s_117_18 -> bv
        let s_117_19: Bits = Bits::new(s_117_18 as u128, 2u16);
        // D s_117_20: cmp-eq s_117_17 s_117_19
        let s_117_20: bool = ((s_117_17) == (s_117_19));
        // D s_117_21: write-var gs#90937 <= s_117_20
        fn_state.gs_90937 = s_117_20;
        // N s_117_22: jump b114
        return block_114(state, tracer, fn_state);
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
        // D s_119_1: write-var gs#90940 <= s_119_0
        fn_state.gs_90940 = s_119_0;
        // N s_119_2: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_120_0: read-var gs#90940:u8
        let s_120_0: bool = fn_state.gs_90940;
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
        // C s_123_0: const #() : ()
        let s_123_0: () = ();
        // S s_123_1: call EDSCR_read(s_123_0)
        let s_123_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_123_0);
        // S s_123_2: call _get_EDSCR_Type_SDD(s_123_1)
        let s_123_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_123_1);
        // S s_123_3: cast zx s_123_2 -> bv
        let s_123_3: Bits = Bits::new(s_123_2 as u128, 1u16);
        // C s_123_4: const #1u : u8
        let s_123_4: bool = true;
        // C s_123_5: cast zx s_123_4 -> bv
        let s_123_5: Bits = Bits::new(s_123_4 as u128, 1u16);
        // S s_123_6: cmp-eq s_123_3 s_123_5
        let s_123_6: bool = ((s_123_3) == (s_123_5));
        // D s_123_7: write-var gs#90940 <= s_123_6
        fn_state.gs_90940 = s_123_6;
        // N s_123_8: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_124<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_124_0: const #22712u : u32
        let s_124_0: u32 = 22712;
        // D s_124_1: read-reg s_124_0:struct
        let s_124_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_124_0 as isize);
            tracer.read_register(s_124_0 as isize, value);
            value
        };
        // D s_124_2: call _get_MDCR_EL3_Type_NSPB(s_124_1)
        let s_124_2: u8 = u_get_MDCR_EL3_Type_NSPB(state, tracer, s_124_1);
        // C s_124_3: const #0s : i
        let s_124_3: i128 = 0;
        // D s_124_4: cast zx s_124_2 -> bv
        let s_124_4: Bits = Bits::new(s_124_2 as u128, 2u16);
        // C s_124_5: const #1u : u64
        let s_124_5: u64 = 1;
        // D s_124_6: bit-extract s_124_4 s_124_3 s_124_5
        let s_124_6: Bits = (Bits::new(
            ((s_124_4) >> (s_124_3)).value(),
            u16::try_from(s_124_5).unwrap(),
        ));
        // D s_124_7: cast reint s_124_6 -> u8
        let s_124_7: bool = ((s_124_6.value()) != 0);
        // C s_124_8: const #0s : i
        let s_124_8: i128 = 0;
        // C s_124_9: const #0u : u64
        let s_124_9: u64 = 0;
        // D s_124_10: cast zx s_124_7 -> u64
        let s_124_10: u64 = (s_124_7 as u64);
        // C s_124_11: const #1u : u64
        let s_124_11: u64 = 1;
        // D s_124_12: and s_124_10 s_124_11
        let s_124_12: u64 = ((s_124_10) & (s_124_11));
        // D s_124_13: cmp-eq s_124_12 s_124_11
        let s_124_13: bool = ((s_124_12) == (s_124_11));
        // D s_124_14: lsl s_124_10 s_124_8
        let s_124_14: u64 = s_124_10 << s_124_8;
        // D s_124_15: or s_124_9 s_124_14
        let s_124_15: u64 = ((s_124_9) | (s_124_14));
        // D s_124_16: cmpl s_124_14
        let s_124_16: u64 = !s_124_14;
        // D s_124_17: and s_124_9 s_124_16
        let s_124_17: u64 = ((s_124_9) & (s_124_16));
        // D s_124_18: select s_124_13 s_124_15 s_124_17
        let s_124_18: u64 = if s_124_13 { s_124_15 } else { s_124_17 };
        // D s_124_19: cast trunc s_124_18 -> u8
        let s_124_19: bool = ((s_124_18) != 0);
        // D s_124_20: cast zx s_124_19 -> bv
        let s_124_20: Bits = Bits::new(s_124_19 as u128, 1u16);
        // C s_124_21: const #0u : u8
        let s_124_21: bool = false;
        // C s_124_22: cast zx s_124_21 -> bv
        let s_124_22: Bits = Bits::new(s_124_21 as u128, 1u16);
        // D s_124_23: cmp-eq s_124_20 s_124_22
        let s_124_23: bool = ((s_124_20) == (s_124_22));
        // N s_124_24: branch s_124_23 b133 b125
        if s_124_23 {
            return block_133(state, tracer, fn_state);
        } else {
            return block_125(state, tracer, fn_state);
        };
    }
    fn block_125<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_125_0: const #22712u : u32
        let s_125_0: u32 = 22712;
        // D s_125_1: read-reg s_125_0:struct
        let s_125_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_125_0 as isize);
            tracer.read_register(s_125_0 as isize, value);
            value
        };
        // D s_125_2: call _get_MDCR_EL3_Type_NSPB(s_125_1)
        let s_125_2: u8 = u_get_MDCR_EL3_Type_NSPB(state, tracer, s_125_1);
        // C s_125_3: const #1s : i
        let s_125_3: i128 = 1;
        // D s_125_4: cast zx s_125_2 -> bv
        let s_125_4: Bits = Bits::new(s_125_2 as u128, 2u16);
        // C s_125_5: const #1u : u64
        let s_125_5: u64 = 1;
        // D s_125_6: bit-extract s_125_4 s_125_3 s_125_5
        let s_125_6: Bits = (Bits::new(
            ((s_125_4) >> (s_125_3)).value(),
            u16::try_from(s_125_5).unwrap(),
        ));
        // D s_125_7: cast reint s_125_6 -> u8
        let s_125_7: bool = ((s_125_6.value()) != 0);
        // C s_125_8: const #0s : i
        let s_125_8: i128 = 0;
        // C s_125_9: const #0u : u64
        let s_125_9: u64 = 0;
        // D s_125_10: cast zx s_125_7 -> u64
        let s_125_10: u64 = (s_125_7 as u64);
        // C s_125_11: const #1u : u64
        let s_125_11: u64 = 1;
        // D s_125_12: and s_125_10 s_125_11
        let s_125_12: u64 = ((s_125_10) & (s_125_11));
        // D s_125_13: cmp-eq s_125_12 s_125_11
        let s_125_13: bool = ((s_125_12) == (s_125_11));
        // D s_125_14: lsl s_125_10 s_125_8
        let s_125_14: u64 = s_125_10 << s_125_8;
        // D s_125_15: or s_125_9 s_125_14
        let s_125_15: u64 = ((s_125_9) | (s_125_14));
        // D s_125_16: cmpl s_125_14
        let s_125_16: u64 = !s_125_14;
        // D s_125_17: and s_125_9 s_125_16
        let s_125_17: u64 = ((s_125_9) & (s_125_16));
        // D s_125_18: select s_125_13 s_125_15 s_125_17
        let s_125_18: u64 = if s_125_13 { s_125_15 } else { s_125_17 };
        // D s_125_19: cast trunc s_125_18 -> u8
        let s_125_19: bool = ((s_125_18) != 0);
        // C s_125_20: const #90704u : u32
        let s_125_20: u32 = 90704;
        // D s_125_21: read-reg s_125_20:struct
        let s_125_21: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_125_20 as isize);
            tracer.read_register(s_125_20 as isize, value);
            value
        };
        // D s_125_22: call _get_SCR_EL3_Type_NS(s_125_21)
        let s_125_22: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_125_21);
        // D s_125_23: cast zx s_125_19 -> bv
        let s_125_23: Bits = Bits::new(s_125_19 as u128, 1u16);
        // D s_125_24: cast zx s_125_22 -> bv
        let s_125_24: Bits = Bits::new(s_125_22 as u128, 1u16);
        // D s_125_25: cmp-ne s_125_23 s_125_24
        let s_125_25: bool = ((s_125_23) != (s_125_24));
        // D s_125_26: write-var gs#90933 <= s_125_25
        fn_state.gs_90933 = s_125_25;
        // N s_125_27: jump b126
        return block_126(state, tracer, fn_state);
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_126_0: read-var gs#90933:u8
        let s_126_0: bool = fn_state.gs_90933;
        // N s_126_1: branch s_126_0 b132 b127
        if s_126_0 {
            return block_132(state, tracer, fn_state);
        } else {
            return block_127(state, tracer, fn_state);
        };
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_127_0: const #232u : u32
        let s_127_0: u32 = 232;
        // S s_127_1: call IsFeatureImplemented(s_127_0)
        let s_127_1: bool = IsFeatureImplemented(state, tracer, s_127_0);
        // N s_127_2: branch s_127_1 b131 b128
        if s_127_1 {
            return block_131(state, tracer, fn_state);
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
        // D s_128_1: write-var gs#90934 <= s_128_0
        fn_state.gs_90934 = s_128_0;
        // N s_128_2: jump b129
        return block_129(state, tracer, fn_state);
    }
    fn block_129<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_129_0: read-var gs#90934:u8
        let s_129_0: bool = fn_state.gs_90934;
        // D s_129_1: write-var gs#90935 <= s_129_0
        fn_state.gs_90935 = s_129_0;
        // N s_129_2: jump b130
        return block_130(state, tracer, fn_state);
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_130_0: read-var gs#90935:u8
        let s_130_0: bool = fn_state.gs_90935;
        // D s_130_1: write-var gs#90936 <= s_130_0
        fn_state.gs_90936 = s_130_0;
        // N s_130_2: jump b111
        return block_111(state, tracer, fn_state);
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_131_0: read-var __MDCR_EL3_NSPBE:u8
        let s_131_0: bool = fn_state.u__MDCR_EL3_NSPBE;
        // D s_131_1: cast zx s_131_0 -> bv
        let s_131_1: Bits = Bits::new(s_131_0 as u128, 1u16);
        // D s_131_2: read-var __SCR_EL3_NSE:u8
        let s_131_2: bool = fn_state.u__SCR_EL3_NSE;
        // D s_131_3: cast zx s_131_2 -> bv
        let s_131_3: Bits = Bits::new(s_131_2 as u128, 1u16);
        // D s_131_4: cmp-ne s_131_1 s_131_3
        let s_131_4: bool = ((s_131_1) != (s_131_3));
        // D s_131_5: write-var gs#90934 <= s_131_4
        fn_state.gs_90934 = s_131_4;
        // N s_131_6: jump b129
        return block_129(state, tracer, fn_state);
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_132_0: const #1u : u8
        let s_132_0: bool = true;
        // D s_132_1: write-var gs#90935 <= s_132_0
        fn_state.gs_90935 = s_132_0;
        // N s_132_2: jump b130
        return block_130(state, tracer, fn_state);
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_133_0: const #1u : u8
        let s_133_0: bool = true;
        // D s_133_1: write-var gs#90933 <= s_133_0
        fn_state.gs_90933 = s_133_0;
        // N s_133_2: jump b126
        return block_126(state, tracer, fn_state);
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_134_0: const #() : ()
        let s_134_0: () = ();
        // S s_134_1: call Halted(s_134_0)
        let s_134_1: bool = Halted(state, tracer, s_134_0);
        // N s_134_2: branch s_134_1 b139 b135
        if s_134_1 {
            return block_139(state, tracer, fn_state);
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
        // D s_135_1: write-var gs#90941 <= s_135_0
        fn_state.gs_90941 = s_135_0;
        // N s_135_2: jump b136
        return block_136(state, tracer, fn_state);
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_136_0: read-var gs#90941:u8
        let s_136_0: bool = fn_state.gs_90941;
        // N s_136_1: branch s_136_0 b138 b137
        if s_136_0 {
            return block_138(state, tracer, fn_state);
        } else {
            return block_137(state, tracer, fn_state);
        };
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_137_0: const #24u : u8
        let s_137_0: u8 = 24;
        // C s_137_1: cast zx s_137_0 -> bv
        let s_137_1: Bits = Bits::new(s_137_0 as u128, 8u16);
        // C s_137_2: cast zx s_137_1 -> i
        let s_137_2: i128 = (s_137_1.value() as i128);
        // C s_137_3: cast reint s_137_2 -> i64
        let s_137_3: i64 = (s_137_2 as i64);
        // C s_137_4: cast zx s_137_3 -> i
        let s_137_4: i128 = (i128::try_from(s_137_3).unwrap());
        // C s_137_5: const #424u : u32
        let s_137_5: u32 = 424;
        // D s_137_6: read-reg s_137_5:u8
        let s_137_6: u8 = {
            let value = state.read_register::<u8>(s_137_5 as isize);
            tracer.read_register(s_137_5 as isize, value);
            value
        };
        // D s_137_7: call AArch64_SystemAccessTrap(s_137_6, s_137_4)
        let s_137_7: () = AArch64_SystemAccessTrap(state, tracer, s_137_6, s_137_4);
        // N s_137_8: return
        return;
    }
    fn block_138<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_138_0: panic
        panic!("{:?}", ());
        // N s_138_1: return
        return;
    }
    fn block_139<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_139_0: const #() : ()
        let s_139_0: () = ();
        // S s_139_1: call EDSCR_read(s_139_0)
        let s_139_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_139_0);
        // S s_139_2: call _get_EDSCR_Type_SDD(s_139_1)
        let s_139_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_139_1);
        // S s_139_3: cast zx s_139_2 -> bv
        let s_139_3: Bits = Bits::new(s_139_2 as u128, 1u16);
        // C s_139_4: const #1u : u8
        let s_139_4: bool = true;
        // C s_139_5: cast zx s_139_4 -> bv
        let s_139_5: Bits = Bits::new(s_139_4 as u128, 1u16);
        // S s_139_6: cmp-eq s_139_3 s_139_5
        let s_139_6: bool = ((s_139_3) == (s_139_5));
        // D s_139_7: write-var gs#90941 <= s_139_6
        fn_state.gs_90941 = s_139_6;
        // N s_139_8: jump b136
        return block_136(state, tracer, fn_state);
    }
    fn block_140<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_140_0: read-var __MDCR_EL3_EnPMS3:u8
        let s_140_0: bool = fn_state.u__MDCR_EL3_EnPMS3;
        // D s_140_1: cast zx s_140_0 -> bv
        let s_140_1: Bits = Bits::new(s_140_0 as u128, 1u16);
        // C s_140_2: const #0u : u8
        let s_140_2: bool = false;
        // C s_140_3: cast zx s_140_2 -> bv
        let s_140_3: Bits = Bits::new(s_140_2 as u128, 1u16);
        // D s_140_4: cmp-eq s_140_1 s_140_3
        let s_140_4: bool = ((s_140_1) == (s_140_3));
        // D s_140_5: write-var gs#90928 <= s_140_4
        fn_state.gs_90928 = s_140_4;
        // N s_140_6: jump b108
        return block_108(state, tracer, fn_state);
    }
    fn block_141<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_141_0: const #24u : u8
        let s_141_0: u8 = 24;
        // C s_141_1: cast zx s_141_0 -> bv
        let s_141_1: Bits = Bits::new(s_141_0 as u128, 8u16);
        // C s_141_2: cast zx s_141_1 -> i
        let s_141_2: i128 = (s_141_1.value() as i128);
        // C s_141_3: cast reint s_141_2 -> i64
        let s_141_3: i64 = (s_141_2 as i64);
        // C s_141_4: cast zx s_141_3 -> i
        let s_141_4: i128 = (i128::try_from(s_141_3).unwrap());
        // C s_141_5: const #432u : u32
        let s_141_5: u32 = 432;
        // D s_141_6: read-reg s_141_5:u8
        let s_141_6: u8 = {
            let value = state.read_register::<u8>(s_141_5 as isize);
            tracer.read_register(s_141_5 as isize, value);
            value
        };
        // D s_141_7: call AArch64_SystemAccessTrap(s_141_6, s_141_4)
        let s_141_7: () = AArch64_SystemAccessTrap(state, tracer, s_141_6, s_141_4);
        // N s_141_8: return
        return;
    }
    fn block_142<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_142_0: read-var __MDCR_EL2_TPMS:u8
        let s_142_0: bool = fn_state.u__MDCR_EL2_TPMS;
        // D s_142_1: cast zx s_142_0 -> bv
        let s_142_1: Bits = Bits::new(s_142_0 as u128, 1u16);
        // C s_142_2: const #1u : u8
        let s_142_2: bool = true;
        // C s_142_3: cast zx s_142_2 -> bv
        let s_142_3: Bits = Bits::new(s_142_2 as u128, 1u16);
        // D s_142_4: cmp-eq s_142_1 s_142_3
        let s_142_4: bool = ((s_142_1) == (s_142_3));
        // D s_142_5: write-var gs#90927 <= s_142_4
        fn_state.gs_90927 = s_142_4;
        // N s_142_6: jump b105
        return block_105(state, tracer, fn_state);
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
        // D s_144_0: read-var __HDFGWTR2_EL2_nPMSDSFR_EL1:u8
        let s_144_0: bool = fn_state.u__HDFGWTR2_EL2_nPMSDSFR_EL1;
        // D s_144_1: cast zx s_144_0 -> bv
        let s_144_1: Bits = Bits::new(s_144_0 as u128, 1u16);
        // C s_144_2: const #0u : u8
        let s_144_2: bool = false;
        // C s_144_3: cast zx s_144_2 -> bv
        let s_144_3: Bits = Bits::new(s_144_2 as u128, 1u16);
        // D s_144_4: cmp-eq s_144_1 s_144_3
        let s_144_4: bool = ((s_144_1) == (s_144_3));
        // D s_144_5: write-var gs#90926 <= s_144_4
        fn_state.gs_90926 = s_144_4;
        // N s_144_6: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_145<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_145_0: const #188u : u32
        let s_145_0: u32 = 188;
        // S s_145_1: call IsFeatureImplemented(s_145_0)
        let s_145_1: bool = IsFeatureImplemented(state, tracer, s_145_0);
        // D s_145_2: write-var gs#90925 <= s_145_1
        fn_state.gs_90925 = s_145_1;
        // N s_145_3: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_146<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_146_0: const #24u : u8
        let s_146_0: u8 = 24;
        // C s_146_1: cast zx s_146_0 -> bv
        let s_146_1: Bits = Bits::new(s_146_0 as u128, 8u16);
        // C s_146_2: cast zx s_146_1 -> i
        let s_146_2: i128 = (s_146_1.value() as i128);
        // C s_146_3: cast reint s_146_2 -> i64
        let s_146_3: i64 = (s_146_2 as i64);
        // C s_146_4: cast zx s_146_3 -> i
        let s_146_4: i128 = (i128::try_from(s_146_3).unwrap());
        // C s_146_5: const #432u : u32
        let s_146_5: u32 = 432;
        // D s_146_6: read-reg s_146_5:u8
        let s_146_6: u8 = {
            let value = state.read_register::<u8>(s_146_5 as isize);
            tracer.read_register(s_146_5 as isize, value);
            value
        };
        // D s_146_7: call AArch64_SystemAccessTrap(s_146_6, s_146_4)
        let s_146_7: () = AArch64_SystemAccessTrap(state, tracer, s_146_6, s_146_4);
        // N s_146_8: return
        return;
    }
    fn block_147<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_147_0: read-var __SCR_EL3_FGTEn2:u8
        let s_147_0: bool = fn_state.u__SCR_EL3_FGTEn2;
        // D s_147_1: cast zx s_147_0 -> bv
        let s_147_1: Bits = Bits::new(s_147_0 as u128, 1u16);
        // C s_147_2: const #0u : u8
        let s_147_2: bool = false;
        // C s_147_3: cast zx s_147_2 -> bv
        let s_147_3: Bits = Bits::new(s_147_2 as u128, 1u16);
        // D s_147_4: cmp-eq s_147_1 s_147_3
        let s_147_4: bool = ((s_147_1) == (s_147_3));
        // D s_147_5: write-var gs#90924 <= s_147_4
        fn_state.gs_90924 = s_147_4;
        // N s_147_6: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_148<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_148_0: const #424u : u32
        let s_148_0: u32 = 424;
        // D s_148_1: read-reg s_148_0:u8
        let s_148_1: u8 = {
            let value = state.read_register::<u8>(s_148_0 as isize);
            tracer.read_register(s_148_0 as isize, value);
            value
        };
        // C s_148_2: const #2u : u8
        let s_148_2: u8 = 2;
        // D s_148_3: cmp-lt s_148_1 s_148_2
        let s_148_3: bool = ((s_148_1) < (s_148_2));
        // D s_148_4: write-var gs#90923 <= s_148_3
        fn_state.gs_90923 = s_148_3;
        // N s_148_5: jump b95
        return block_95(state, tracer, fn_state);
    }
    fn block_149<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_149_0: const #188u : u32
        let s_149_0: u32 = 188;
        // S s_149_1: call IsFeatureImplemented(s_149_0)
        let s_149_1: bool = IsFeatureImplemented(state, tracer, s_149_0);
        // D s_149_2: write-var gs#90922 <= s_149_1
        fn_state.gs_90922 = s_149_1;
        // N s_149_3: jump b93
        return block_93(state, tracer, fn_state);
    }
    fn block_150<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_150_0: panic
        panic!("{:?}", ());
        // N s_150_1: return
        return;
    }
    fn block_151<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_151_0: const #22712u : u32
        let s_151_0: u32 = 22712;
        // D s_151_1: read-reg s_151_0:struct
        let s_151_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_151_0 as isize);
            tracer.read_register(s_151_0 as isize, value);
            value
        };
        // D s_151_2: call _get_MDCR_EL3_Type_NSPB(s_151_1)
        let s_151_2: u8 = u_get_MDCR_EL3_Type_NSPB(state, tracer, s_151_1);
        // C s_151_3: const #0s : i
        let s_151_3: i128 = 0;
        // D s_151_4: cast zx s_151_2 -> bv
        let s_151_4: Bits = Bits::new(s_151_2 as u128, 2u16);
        // C s_151_5: const #1u : u64
        let s_151_5: u64 = 1;
        // D s_151_6: bit-extract s_151_4 s_151_3 s_151_5
        let s_151_6: Bits = (Bits::new(
            ((s_151_4) >> (s_151_3)).value(),
            u16::try_from(s_151_5).unwrap(),
        ));
        // D s_151_7: cast reint s_151_6 -> u8
        let s_151_7: bool = ((s_151_6.value()) != 0);
        // C s_151_8: const #0s : i
        let s_151_8: i128 = 0;
        // C s_151_9: const #0u : u64
        let s_151_9: u64 = 0;
        // D s_151_10: cast zx s_151_7 -> u64
        let s_151_10: u64 = (s_151_7 as u64);
        // C s_151_11: const #1u : u64
        let s_151_11: u64 = 1;
        // D s_151_12: and s_151_10 s_151_11
        let s_151_12: u64 = ((s_151_10) & (s_151_11));
        // D s_151_13: cmp-eq s_151_12 s_151_11
        let s_151_13: bool = ((s_151_12) == (s_151_11));
        // D s_151_14: lsl s_151_10 s_151_8
        let s_151_14: u64 = s_151_10 << s_151_8;
        // D s_151_15: or s_151_9 s_151_14
        let s_151_15: u64 = ((s_151_9) | (s_151_14));
        // D s_151_16: cmpl s_151_14
        let s_151_16: u64 = !s_151_14;
        // D s_151_17: and s_151_9 s_151_16
        let s_151_17: u64 = ((s_151_9) & (s_151_16));
        // D s_151_18: select s_151_13 s_151_15 s_151_17
        let s_151_18: u64 = if s_151_13 { s_151_15 } else { s_151_17 };
        // D s_151_19: cast trunc s_151_18 -> u8
        let s_151_19: bool = ((s_151_18) != 0);
        // D s_151_20: cast zx s_151_19 -> bv
        let s_151_20: Bits = Bits::new(s_151_19 as u128, 1u16);
        // C s_151_21: const #0u : u8
        let s_151_21: bool = false;
        // C s_151_22: cast zx s_151_21 -> bv
        let s_151_22: Bits = Bits::new(s_151_21 as u128, 1u16);
        // D s_151_23: cmp-eq s_151_20 s_151_22
        let s_151_23: bool = ((s_151_20) == (s_151_22));
        // N s_151_24: branch s_151_23 b160 b152
        if s_151_23 {
            return block_160(state, tracer, fn_state);
        } else {
            return block_152(state, tracer, fn_state);
        };
    }
    fn block_152<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_152_0: const #22712u : u32
        let s_152_0: u32 = 22712;
        // D s_152_1: read-reg s_152_0:struct
        let s_152_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_152_0 as isize);
            tracer.read_register(s_152_0 as isize, value);
            value
        };
        // D s_152_2: call _get_MDCR_EL3_Type_NSPB(s_152_1)
        let s_152_2: u8 = u_get_MDCR_EL3_Type_NSPB(state, tracer, s_152_1);
        // C s_152_3: const #1s : i
        let s_152_3: i128 = 1;
        // D s_152_4: cast zx s_152_2 -> bv
        let s_152_4: Bits = Bits::new(s_152_2 as u128, 2u16);
        // C s_152_5: const #1u : u64
        let s_152_5: u64 = 1;
        // D s_152_6: bit-extract s_152_4 s_152_3 s_152_5
        let s_152_6: Bits = (Bits::new(
            ((s_152_4) >> (s_152_3)).value(),
            u16::try_from(s_152_5).unwrap(),
        ));
        // D s_152_7: cast reint s_152_6 -> u8
        let s_152_7: bool = ((s_152_6.value()) != 0);
        // C s_152_8: const #0s : i
        let s_152_8: i128 = 0;
        // C s_152_9: const #0u : u64
        let s_152_9: u64 = 0;
        // D s_152_10: cast zx s_152_7 -> u64
        let s_152_10: u64 = (s_152_7 as u64);
        // C s_152_11: const #1u : u64
        let s_152_11: u64 = 1;
        // D s_152_12: and s_152_10 s_152_11
        let s_152_12: u64 = ((s_152_10) & (s_152_11));
        // D s_152_13: cmp-eq s_152_12 s_152_11
        let s_152_13: bool = ((s_152_12) == (s_152_11));
        // D s_152_14: lsl s_152_10 s_152_8
        let s_152_14: u64 = s_152_10 << s_152_8;
        // D s_152_15: or s_152_9 s_152_14
        let s_152_15: u64 = ((s_152_9) | (s_152_14));
        // D s_152_16: cmpl s_152_14
        let s_152_16: u64 = !s_152_14;
        // D s_152_17: and s_152_9 s_152_16
        let s_152_17: u64 = ((s_152_9) & (s_152_16));
        // D s_152_18: select s_152_13 s_152_15 s_152_17
        let s_152_18: u64 = if s_152_13 { s_152_15 } else { s_152_17 };
        // D s_152_19: cast trunc s_152_18 -> u8
        let s_152_19: bool = ((s_152_18) != 0);
        // C s_152_20: const #90704u : u32
        let s_152_20: u32 = 90704;
        // D s_152_21: read-reg s_152_20:struct
        let s_152_21: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_152_20 as isize);
            tracer.read_register(s_152_20 as isize, value);
            value
        };
        // D s_152_22: call _get_SCR_EL3_Type_NS(s_152_21)
        let s_152_22: bool = u_get_SCR_EL3_Type_NS(state, tracer, s_152_21);
        // D s_152_23: cast zx s_152_19 -> bv
        let s_152_23: Bits = Bits::new(s_152_19 as u128, 1u16);
        // D s_152_24: cast zx s_152_22 -> bv
        let s_152_24: Bits = Bits::new(s_152_22 as u128, 1u16);
        // D s_152_25: cmp-ne s_152_23 s_152_24
        let s_152_25: bool = ((s_152_23) != (s_152_24));
        // D s_152_26: write-var gs#90918 <= s_152_25
        fn_state.gs_90918 = s_152_25;
        // N s_152_27: jump b153
        return block_153(state, tracer, fn_state);
    }
    fn block_153<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_153_0: read-var gs#90918:u8
        let s_153_0: bool = fn_state.gs_90918;
        // N s_153_1: branch s_153_0 b159 b154
        if s_153_0 {
            return block_159(state, tracer, fn_state);
        } else {
            return block_154(state, tracer, fn_state);
        };
    }
    fn block_154<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_154_0: const #232u : u32
        let s_154_0: u32 = 232;
        // S s_154_1: call IsFeatureImplemented(s_154_0)
        let s_154_1: bool = IsFeatureImplemented(state, tracer, s_154_0);
        // N s_154_2: branch s_154_1 b158 b155
        if s_154_1 {
            return block_158(state, tracer, fn_state);
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
        // D s_155_1: write-var gs#90919 <= s_155_0
        fn_state.gs_90919 = s_155_0;
        // N s_155_2: jump b156
        return block_156(state, tracer, fn_state);
    }
    fn block_156<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_156_0: read-var gs#90919:u8
        let s_156_0: bool = fn_state.gs_90919;
        // D s_156_1: write-var gs#90920 <= s_156_0
        fn_state.gs_90920 = s_156_0;
        // N s_156_2: jump b157
        return block_157(state, tracer, fn_state);
    }
    fn block_157<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_157_0: read-var gs#90920:u8
        let s_157_0: bool = fn_state.gs_90920;
        // D s_157_1: write-var gs#90921 <= s_157_0
        fn_state.gs_90921 = s_157_0;
        // N s_157_2: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_158<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_158_0: read-var __MDCR_EL3_NSPBE:u8
        let s_158_0: bool = fn_state.u__MDCR_EL3_NSPBE;
        // D s_158_1: cast zx s_158_0 -> bv
        let s_158_1: Bits = Bits::new(s_158_0 as u128, 1u16);
        // D s_158_2: read-var __SCR_EL3_NSE:u8
        let s_158_2: bool = fn_state.u__SCR_EL3_NSE;
        // D s_158_3: cast zx s_158_2 -> bv
        let s_158_3: Bits = Bits::new(s_158_2 as u128, 1u16);
        // D s_158_4: cmp-ne s_158_1 s_158_3
        let s_158_4: bool = ((s_158_1) != (s_158_3));
        // D s_158_5: write-var gs#90919 <= s_158_4
        fn_state.gs_90919 = s_158_4;
        // N s_158_6: jump b156
        return block_156(state, tracer, fn_state);
    }
    fn block_159<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_159_0: const #1u : u8
        let s_159_0: bool = true;
        // D s_159_1: write-var gs#90920 <= s_159_0
        fn_state.gs_90920 = s_159_0;
        // N s_159_2: jump b157
        return block_157(state, tracer, fn_state);
    }
    fn block_160<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_160_0: const #1u : u8
        let s_160_0: bool = true;
        // D s_160_1: write-var gs#90918 <= s_160_0
        fn_state.gs_90918 = s_160_0;
        // N s_160_2: jump b153
        return block_153(state, tracer, fn_state);
    }
    fn block_161<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_161_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_161_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_161_1: call __IMPDEF_boolean(s_161_0)
        let s_161_1: bool = u__IMPDEF_boolean(state, tracer, s_161_0);
        // D s_161_2: write-var gs#90913 <= s_161_1
        fn_state.gs_90913 = s_161_1;
        // N s_161_3: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_162<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_162_0: const #() : ()
        let s_162_0: () = ();
        // S s_162_1: call EDSCR_read(s_162_0)
        let s_162_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_162_0);
        // S s_162_2: call _get_EDSCR_Type_SDD(s_162_1)
        let s_162_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_162_1);
        // S s_162_3: cast zx s_162_2 -> bv
        let s_162_3: Bits = Bits::new(s_162_2 as u128, 1u16);
        // C s_162_4: const #1u : u8
        let s_162_4: bool = true;
        // C s_162_5: cast zx s_162_4 -> bv
        let s_162_5: Bits = Bits::new(s_162_4 as u128, 1u16);
        // S s_162_6: cmp-eq s_162_3 s_162_5
        let s_162_6: bool = ((s_162_3) == (s_162_5));
        // D s_162_7: write-var gs#90912 <= s_162_6
        fn_state.gs_90912 = s_162_6;
        // N s_162_8: jump b86
        return block_86(state, tracer, fn_state);
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
        // C s_163_2: const #2u : u8
        let s_163_2: u8 = 2;
        // D s_163_3: cmp-lt s_163_1 s_163_2
        let s_163_3: bool = ((s_163_1) < (s_163_2));
        // D s_163_4: write-var gs#90911 <= s_163_3
        fn_state.gs_90911 = s_163_3;
        // N s_163_5: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_164<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_164_0: panic
        panic!("{:?}", ());
        // N s_164_1: return
        return;
    }
    fn block_165<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_165_0: read-var __MDCR_EL3_EnPMS3:u8
        let s_165_0: bool = fn_state.u__MDCR_EL3_EnPMS3;
        // D s_165_1: cast zx s_165_0 -> bv
        let s_165_1: Bits = Bits::new(s_165_0 as u128, 1u16);
        // C s_165_2: const #0u : u8
        let s_165_2: bool = false;
        // C s_165_3: cast zx s_165_2 -> bv
        let s_165_3: Bits = Bits::new(s_165_2 as u128, 1u16);
        // D s_165_4: cmp-eq s_165_1 s_165_3
        let s_165_4: bool = ((s_165_1) == (s_165_3));
        // D s_165_5: write-var gs#90910 <= s_165_4
        fn_state.gs_90910 = s_165_4;
        // N s_165_6: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_166<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_166_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_166_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_166_1: call __IMPDEF_boolean(s_166_0)
        let s_166_1: bool = u__IMPDEF_boolean(state, tracer, s_166_0);
        // D s_166_2: write-var gs#90909 <= s_166_1
        fn_state.gs_90909 = s_166_1;
        // N s_166_3: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_167<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_167_0: const #() : ()
        let s_167_0: () = ();
        // S s_167_1: call EDSCR_read(s_167_0)
        let s_167_1: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_167_0);
        // S s_167_2: call _get_EDSCR_Type_SDD(s_167_1)
        let s_167_2: bool = u_get_EDSCR_Type_SDD(state, tracer, s_167_1);
        // S s_167_3: cast zx s_167_2 -> bv
        let s_167_3: Bits = Bits::new(s_167_2 as u128, 1u16);
        // C s_167_4: const #1u : u8
        let s_167_4: bool = true;
        // C s_167_5: cast zx s_167_4 -> bv
        let s_167_5: Bits = Bits::new(s_167_4 as u128, 1u16);
        // S s_167_6: cmp-eq s_167_3 s_167_5
        let s_167_6: bool = ((s_167_3) == (s_167_5));
        // D s_167_7: write-var gs#90908 <= s_167_6
        fn_state.gs_90908 = s_167_6;
        // N s_167_8: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_168<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_168_0: const #424u : u32
        let s_168_0: u32 = 424;
        // D s_168_1: read-reg s_168_0:u8
        let s_168_1: u8 = {
            let value = state.read_register::<u8>(s_168_0 as isize);
            tracer.read_register(s_168_0 as isize, value);
            value
        };
        // C s_168_2: const #2u : u8
        let s_168_2: u8 = 2;
        // D s_168_3: cmp-lt s_168_1 s_168_2
        let s_168_3: bool = ((s_168_1) < (s_168_2));
        // D s_168_4: write-var gs#90907 <= s_168_3
        fn_state.gs_90907 = s_168_3;
        // N s_168_5: jump b75
        return block_75(state, tracer, fn_state);
    }
    fn block_169<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_169_0: panic
        panic!("{:?}", ());
        // N s_169_1: return
        return;
    }
}
