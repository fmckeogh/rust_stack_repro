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
use u_get_HDFGWTR_EL2_Type_PMSWINC_EL0::*;
use IsFeatureImplemented::*;
use u__IMPDEF_boolean::*;
use X_read::*;
use Mk_PMSWINC_EL0_Type::*;
use AArch64_SystemAccessTrap::*;
use u_get_MDCR_EL3_Type_TPM::*;
use u_get_PMUSERENR_EL0_Type_SW::*;
use u_get_PMUSERENR_EL0_Type_EN::*;
use u_get_EDSCR_Type_SDD::*;
use EL2Enabled::*;
use u_get_HCR_EL2_Type_TGE::*;
use EDSCR_read::*;
use common::*;
pub fn PMSWINC_EL0_SysRegWrite_47242e8bf7a2ab9a<T: Tracer>(
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
        gs_91328: bool,
        gs_91320: bool,
        gs_91331: bool,
        gs_91340: bool,
        gs_91313: bool,
        gs_91344: bool,
        gs_91323: bool,
        gs_91336: bool,
        gs_91319: bool,
        u__MDCR_EL3_TPM: bool,
        gs_91318: bool,
        gs_91339: bool,
        gs_91326: bool,
        gs_91329: bool,
        gs_91321: bool,
        gs_91316: bool,
        gs_91341: bool,
        u__PSTATE_EL: u8,
        gs_91314: bool,
        u__MDCR_EL2_TPM: bool,
        gs_91332: bool,
        gs_91310: bool,
        gs_91334: bool,
        u__HCR_EL2_TGE: bool,
        u__EDSCR_SDD: bool,
        gs_91325: bool,
        u__SCR_EL3_FGTEn: bool,
        gs_91311: bool,
        gs_91333: bool,
        gs_91345: bool,
        gs_91322: bool,
        gs_91317: bool,
        u__HDFGWTR_EL2_PMSWINC_EL0: bool,
        gs_91337: bool,
        gs_91312: bool,
        gs_91342: bool,
        gs_91330: bool,
        gs_91335: bool,
        gs_91338: bool,
        gs_91324: bool,
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
        // C s_0_19: const #17360u : u32
        let s_0_19: u32 = 17360;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_HDFGWTR_EL2_Type_PMSWINC_EL0(s_0_20)
        let s_0_21: bool = u_get_HDFGWTR_EL2_Type_PMSWINC_EL0(state, tracer, s_0_20);
        // D s_0_22: write-var __HDFGWTR_EL2_PMSWINC_EL0 <= s_0_21
        fn_state.u__HDFGWTR_EL2_PMSWINC_EL0 = s_0_21;
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
        // N s_0_33: branch s_0_32 b75 b1
        if s_0_32 {
            return block_75(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b31 b2
        if s_1_5 {
            return block_31(state, tracer, fn_state);
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
        // D s_5_4: call Mk_PMSWINC_EL0_Type(s_5_3)
        let s_5_4: ProductType5c790c8ef59cc8b2 = Mk_PMSWINC_EL0_Type(
            state,
            tracer,
            s_5_3,
        );
        // C s_5_5: const #103944u : u32
        let s_5_5: u32 = 103944;
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
        // N s_6_2: branch s_6_1 b30 b7
        if s_6_1 {
            return block_30(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#91310 <= s_7_0
        fn_state.gs_91310 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#91310:u8
        let s_8_0: bool = fn_state.gs_91310;
        // N s_8_1: branch s_8_0 b29 b9
        if s_8_0 {
            return block_29(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#91311 <= s_9_0
        fn_state.gs_91311 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#91311:u8
        let s_10_0: bool = fn_state.gs_91311;
        // N s_10_1: branch s_10_0 b28 b11
        if s_10_0 {
            return block_28(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#91312 <= s_11_0
        fn_state.gs_91312 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#91312:u8
        let s_12_0: bool = fn_state.gs_91312;
        // N s_12_1: branch s_12_0 b27 b13
        if s_12_0 {
            return block_27(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#91313 <= s_13_0
        fn_state.gs_91313 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#91313:u8
        let s_14_0: bool = fn_state.gs_91313;
        // N s_14_1: branch s_14_0 b26 b15
        if s_14_0 {
            return block_26(state, tracer, fn_state);
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
        // N s_15_4: branch s_15_3 b25 b16
        if s_15_3 {
            return block_25(state, tracer, fn_state);
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
        // D s_16_1: write-var gs#91314 <= s_16_0
        fn_state.gs_91314 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#91314:u8
        let s_17_0: bool = fn_state.gs_91314;
        // N s_17_1: branch s_17_0 b19 b18
        if s_17_0 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #64s : i64
        let s_18_0: i64 = 64;
        // D s_18_1: read-var t:i
        let s_18_1: i128 = fn_state.t;
        // D s_18_2: call X_read(s_18_1, s_18_0)
        let s_18_2: Bits = X_read(state, tracer, s_18_1, s_18_0);
        // D s_18_3: cast reint s_18_2 -> u64
        let s_18_3: u64 = (s_18_2.value() as u64);
        // D s_18_4: call Mk_PMSWINC_EL0_Type(s_18_3)
        let s_18_4: ProductType5c790c8ef59cc8b2 = Mk_PMSWINC_EL0_Type(
            state,
            tracer,
            s_18_3,
        );
        // C s_18_5: const #103944u : u32
        let s_18_5: u32 = 103944;
        // N s_18_6: write-reg s_18_5 <= s_18_4
        let s_18_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_18_5 as isize, s_18_4);
            tracer.write_register(s_18_5 as isize, s_18_4);
        };
        // N s_18_7: return
        return;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #() : ()
        let s_19_0: () = ();
        // S s_19_1: call Halted(s_19_0)
        let s_19_1: bool = Halted(state, tracer, s_19_0);
        // N s_19_2: branch s_19_1 b24 b20
        if s_19_1 {
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
        // D s_20_1: write-var gs#91316 <= s_20_0
        fn_state.gs_91316 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#91316:u8
        let s_21_0: bool = fn_state.gs_91316;
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
        // C s_22_0: const #24u : u8
        let s_22_0: u8 = 24;
        // C s_22_1: cast zx s_22_0 -> bv
        let s_22_1: Bits = Bits::new(s_22_0 as u128, 8u16);
        // C s_22_2: cast zx s_22_1 -> i
        let s_22_2: i128 = (s_22_1.value() as i128);
        // C s_22_3: cast reint s_22_2 -> i64
        let s_22_3: i64 = (s_22_2 as i64);
        // C s_22_4: cast zx s_22_3 -> i
        let s_22_4: i128 = (i128::try_from(s_22_3).unwrap());
        // C s_22_5: const #424u : u32
        let s_22_5: u32 = 424;
        // D s_22_6: read-reg s_22_5:u8
        let s_22_6: u8 = {
            let value = state.read_register::<u8>(s_22_5 as isize);
            tracer.read_register(s_22_5 as isize, value);
            value
        };
        // D s_22_7: call AArch64_SystemAccessTrap(s_22_6, s_22_4)
        let s_22_7: () = AArch64_SystemAccessTrap(state, tracer, s_22_6, s_22_4);
        // N s_22_8: return
        return;
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_23_0: panic
        panic!("{:?}", ());
        // N s_23_1: return
        return;
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var __EDSCR_SDD:u8
        let s_24_0: bool = fn_state.u__EDSCR_SDD;
        // D s_24_1: cast zx s_24_0 -> bv
        let s_24_1: Bits = Bits::new(s_24_0 as u128, 1u16);
        // C s_24_2: const #1u : u8
        let s_24_2: bool = true;
        // C s_24_3: cast zx s_24_2 -> bv
        let s_24_3: Bits = Bits::new(s_24_2 as u128, 1u16);
        // D s_24_4: cmp-eq s_24_1 s_24_3
        let s_24_4: bool = ((s_24_1) == (s_24_3));
        // D s_24_5: write-var gs#91316 <= s_24_4
        fn_state.gs_91316 = s_24_4;
        // N s_24_6: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var __MDCR_EL3_TPM:u8
        let s_25_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_25_1: cast zx s_25_0 -> bv
        let s_25_1: Bits = Bits::new(s_25_0 as u128, 1u16);
        // C s_25_2: const #1u : u8
        let s_25_2: bool = true;
        // C s_25_3: cast zx s_25_2 -> bv
        let s_25_3: Bits = Bits::new(s_25_2 as u128, 1u16);
        // D s_25_4: cmp-eq s_25_1 s_25_3
        let s_25_4: bool = ((s_25_1) == (s_25_3));
        // D s_25_5: write-var gs#91314 <= s_25_4
        fn_state.gs_91314 = s_25_4;
        // N s_25_6: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_26_0: panic
        panic!("{:?}", ());
        // N s_26_1: return
        return;
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var __MDCR_EL3_TPM:u8
        let s_27_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_27_1: cast zx s_27_0 -> bv
        let s_27_1: Bits = Bits::new(s_27_0 as u128, 1u16);
        // C s_27_2: const #1u : u8
        let s_27_2: bool = true;
        // C s_27_3: cast zx s_27_2 -> bv
        let s_27_3: Bits = Bits::new(s_27_2 as u128, 1u16);
        // D s_27_4: cmp-eq s_27_1 s_27_3
        let s_27_4: bool = ((s_27_1) == (s_27_3));
        // D s_27_5: write-var gs#91313 <= s_27_4
        fn_state.gs_91313 = s_27_4;
        // N s_27_6: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_28_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_28_1: call __IMPDEF_boolean(s_28_0)
        let s_28_1: bool = u__IMPDEF_boolean(state, tracer, s_28_0);
        // D s_28_2: write-var gs#91312 <= s_28_1
        fn_state.gs_91312 = s_28_1;
        // N s_28_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var __EDSCR_SDD:u8
        let s_29_0: bool = fn_state.u__EDSCR_SDD;
        // D s_29_1: cast zx s_29_0 -> bv
        let s_29_1: Bits = Bits::new(s_29_0 as u128, 1u16);
        // C s_29_2: const #1u : u8
        let s_29_2: bool = true;
        // C s_29_3: cast zx s_29_2 -> bv
        let s_29_3: Bits = Bits::new(s_29_2 as u128, 1u16);
        // D s_29_4: cmp-eq s_29_1 s_29_3
        let s_29_4: bool = ((s_29_1) == (s_29_3));
        // D s_29_5: write-var gs#91311 <= s_29_4
        fn_state.gs_91311 = s_29_4;
        // N s_29_6: jump b10
        return block_10(state, tracer, fn_state);
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
        // C s_30_2: const #2u : u8
        let s_30_2: u8 = 2;
        // D s_30_3: cmp-lt s_30_1 s_30_2
        let s_30_3: bool = ((s_30_1) < (s_30_2));
        // D s_30_4: write-var gs#91310 <= s_30_3
        fn_state.gs_91310 = s_30_3;
        // N s_30_5: jump b8
        return block_8(state, tracer, fn_state);
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
        // N s_31_2: branch s_31_1 b74 b32
        if s_31_1 {
            return block_74(state, tracer, fn_state);
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
        // D s_32_1: write-var gs#91317 <= s_32_0
        fn_state.gs_91317 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#91317:u8
        let s_33_0: bool = fn_state.gs_91317;
        // N s_33_1: branch s_33_0 b73 b34
        if s_33_0 {
            return block_73(state, tracer, fn_state);
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
        // D s_34_1: write-var gs#91318 <= s_34_0
        fn_state.gs_91318 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#91318:u8
        let s_35_0: bool = fn_state.gs_91318;
        // N s_35_1: branch s_35_0 b72 b36
        if s_35_0 {
            return block_72(state, tracer, fn_state);
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
        // D s_36_1: write-var gs#91319 <= s_36_0
        fn_state.gs_91319 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#91319:u8
        let s_37_0: bool = fn_state.gs_91319;
        // N s_37_1: branch s_37_0 b71 b38
        if s_37_0 {
            return block_71(state, tracer, fn_state);
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
        // D s_38_1: write-var gs#91320 <= s_38_0
        fn_state.gs_91320 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#91320:u8
        let s_39_0: bool = fn_state.gs_91320;
        // N s_39_1: branch s_39_0 b70 b40
        if s_39_0 {
            return block_70(state, tracer, fn_state);
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
        // N s_40_2: branch s_40_1 b69 b41
        if s_40_1 {
            return block_69(state, tracer, fn_state);
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
        // D s_41_1: write-var gs#91321 <= s_41_0
        fn_state.gs_91321 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#91321:u8
        let s_42_0: bool = fn_state.gs_91321;
        // N s_42_1: branch s_42_0 b65 b43
        if s_42_0 {
            return block_65(state, tracer, fn_state);
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
        // D s_43_1: write-var gs#91323 <= s_43_0
        fn_state.gs_91323 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#91323:u8
        let s_44_0: bool = fn_state.gs_91323;
        // N s_44_1: branch s_44_0 b64 b45
        if s_44_0 {
            return block_64(state, tracer, fn_state);
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
        // D s_45_1: write-var gs#91324 <= s_45_0
        fn_state.gs_91324 = s_45_0;
        // N s_45_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var gs#91324:u8
        let s_46_0: bool = fn_state.gs_91324;
        // N s_46_1: branch s_46_0 b63 b47
        if s_46_0 {
            return block_63(state, tracer, fn_state);
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
        // N s_47_2: branch s_47_1 b62 b48
        if s_47_1 {
            return block_62(state, tracer, fn_state);
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
        // D s_48_1: write-var gs#91325 <= s_48_0
        fn_state.gs_91325 = s_48_0;
        // N s_48_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var gs#91325:u8
        let s_49_0: bool = fn_state.gs_91325;
        // N s_49_1: branch s_49_0 b61 b50
        if s_49_0 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #424u : u32
        let s_50_0: u32 = 424;
        // D s_50_1: read-reg s_50_0:u8
        let s_50_1: u8 = {
            let value = state.read_register::<u8>(s_50_0 as isize);
            tracer.read_register(s_50_0 as isize, value);
            value
        };
        // C s_50_2: const #2u : u8
        let s_50_2: u8 = 2;
        // D s_50_3: cmp-lt s_50_1 s_50_2
        let s_50_3: bool = ((s_50_1) < (s_50_2));
        // N s_50_4: branch s_50_3 b60 b51
        if s_50_3 {
            return block_60(state, tracer, fn_state);
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
        // D s_51_1: write-var gs#91326 <= s_51_0
        fn_state.gs_91326 = s_51_0;
        // N s_51_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var gs#91326:u8
        let s_52_0: bool = fn_state.gs_91326;
        // N s_52_1: branch s_52_0 b54 b53
        if s_52_0 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_53(state, tracer, fn_state);
        };
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #64s : i64
        let s_53_0: i64 = 64;
        // D s_53_1: read-var t:i
        let s_53_1: i128 = fn_state.t;
        // D s_53_2: call X_read(s_53_1, s_53_0)
        let s_53_2: Bits = X_read(state, tracer, s_53_1, s_53_0);
        // D s_53_3: cast reint s_53_2 -> u64
        let s_53_3: u64 = (s_53_2.value() as u64);
        // D s_53_4: call Mk_PMSWINC_EL0_Type(s_53_3)
        let s_53_4: ProductType5c790c8ef59cc8b2 = Mk_PMSWINC_EL0_Type(
            state,
            tracer,
            s_53_3,
        );
        // C s_53_5: const #103944u : u32
        let s_53_5: u32 = 103944;
        // N s_53_6: write-reg s_53_5 <= s_53_4
        let s_53_6: () = {
            state.write_register::<ProductType5c790c8ef59cc8b2>(s_53_5 as isize, s_53_4);
            tracer.write_register(s_53_5 as isize, s_53_4);
        };
        // N s_53_7: return
        return;
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #() : ()
        let s_54_0: () = ();
        // S s_54_1: call Halted(s_54_0)
        let s_54_1: bool = Halted(state, tracer, s_54_0);
        // N s_54_2: branch s_54_1 b59 b55
        if s_54_1 {
            return block_59(state, tracer, fn_state);
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
        // D s_55_1: write-var gs#91328 <= s_55_0
        fn_state.gs_91328 = s_55_0;
        // N s_55_2: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var gs#91328:u8
        let s_56_0: bool = fn_state.gs_91328;
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
        // C s_57_5: const #424u : u32
        let s_57_5: u32 = 424;
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
        // N s_58_0: panic
        panic!("{:?}", ());
        // N s_58_1: return
        return;
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_59_0: read-var __EDSCR_SDD:u8
        let s_59_0: bool = fn_state.u__EDSCR_SDD;
        // D s_59_1: cast zx s_59_0 -> bv
        let s_59_1: Bits = Bits::new(s_59_0 as u128, 1u16);
        // C s_59_2: const #1u : u8
        let s_59_2: bool = true;
        // C s_59_3: cast zx s_59_2 -> bv
        let s_59_3: Bits = Bits::new(s_59_2 as u128, 1u16);
        // D s_59_4: cmp-eq s_59_1 s_59_3
        let s_59_4: bool = ((s_59_1) == (s_59_3));
        // D s_59_5: write-var gs#91328 <= s_59_4
        fn_state.gs_91328 = s_59_4;
        // N s_59_6: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var __MDCR_EL3_TPM:u8
        let s_60_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_60_1: cast zx s_60_0 -> bv
        let s_60_1: Bits = Bits::new(s_60_0 as u128, 1u16);
        // C s_60_2: const #1u : u8
        let s_60_2: bool = true;
        // C s_60_3: cast zx s_60_2 -> bv
        let s_60_3: Bits = Bits::new(s_60_2 as u128, 1u16);
        // D s_60_4: cmp-eq s_60_1 s_60_3
        let s_60_4: bool = ((s_60_1) == (s_60_3));
        // D s_60_5: write-var gs#91326 <= s_60_4
        fn_state.gs_91326 = s_60_4;
        // N s_60_6: jump b52
        return block_52(state, tracer, fn_state);
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
        // C s_61_5: const #432u : u32
        let s_61_5: u32 = 432;
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
        // D s_62_0: read-var __MDCR_EL2_TPM:u8
        let s_62_0: bool = fn_state.u__MDCR_EL2_TPM;
        // D s_62_1: cast zx s_62_0 -> bv
        let s_62_1: Bits = Bits::new(s_62_0 as u128, 1u16);
        // C s_62_2: const #1u : u8
        let s_62_2: bool = true;
        // C s_62_3: cast zx s_62_2 -> bv
        let s_62_3: Bits = Bits::new(s_62_2 as u128, 1u16);
        // D s_62_4: cmp-eq s_62_1 s_62_3
        let s_62_4: bool = ((s_62_1) == (s_62_3));
        // D s_62_5: write-var gs#91325 <= s_62_4
        fn_state.gs_91325 = s_62_4;
        // N s_62_6: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #24u : u8
        let s_63_0: u8 = 24;
        // C s_63_1: cast zx s_63_0 -> bv
        let s_63_1: Bits = Bits::new(s_63_0 as u128, 8u16);
        // C s_63_2: cast zx s_63_1 -> i
        let s_63_2: i128 = (s_63_1.value() as i128);
        // C s_63_3: cast reint s_63_2 -> i64
        let s_63_3: i64 = (s_63_2 as i64);
        // C s_63_4: cast zx s_63_3 -> i
        let s_63_4: i128 = (i128::try_from(s_63_3).unwrap());
        // C s_63_5: const #432u : u32
        let s_63_5: u32 = 432;
        // D s_63_6: read-reg s_63_5:u8
        let s_63_6: u8 = {
            let value = state.read_register::<u8>(s_63_5 as isize);
            tracer.read_register(s_63_5 as isize, value);
            value
        };
        // D s_63_7: call AArch64_SystemAccessTrap(s_63_6, s_63_4)
        let s_63_7: () = AArch64_SystemAccessTrap(state, tracer, s_63_6, s_63_4);
        // N s_63_8: return
        return;
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var __HDFGWTR_EL2_PMSWINC_EL0:u8
        let s_64_0: bool = fn_state.u__HDFGWTR_EL2_PMSWINC_EL0;
        // D s_64_1: cast zx s_64_0 -> bv
        let s_64_1: Bits = Bits::new(s_64_0 as u128, 1u16);
        // C s_64_2: const #1u : u8
        let s_64_2: bool = true;
        // C s_64_3: cast zx s_64_2 -> bv
        let s_64_3: Bits = Bits::new(s_64_2 as u128, 1u16);
        // D s_64_4: cmp-eq s_64_1 s_64_3
        let s_64_4: bool = ((s_64_1) == (s_64_3));
        // D s_64_5: write-var gs#91324 <= s_64_4
        fn_state.gs_91324 = s_64_4;
        // N s_64_6: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #424u : u32
        let s_65_0: u32 = 424;
        // D s_65_1: read-reg s_65_0:u8
        let s_65_1: u8 = {
            let value = state.read_register::<u8>(s_65_0 as isize);
            tracer.read_register(s_65_0 as isize, value);
            value
        };
        // C s_65_2: const #2u : u8
        let s_65_2: u8 = 2;
        // D s_65_3: cmp-lt s_65_1 s_65_2
        let s_65_3: bool = ((s_65_1) < (s_65_2));
        // D s_65_4: not s_65_3
        let s_65_4: bool = !s_65_3;
        // N s_65_5: branch s_65_4 b68 b66
        if s_65_4 {
            return block_68(state, tracer, fn_state);
        } else {
            return block_66(state, tracer, fn_state);
        };
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_66_0: read-var __SCR_EL3_FGTEn:u8
        let s_66_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_66_1: cast zx s_66_0 -> bv
        let s_66_1: Bits = Bits::new(s_66_0 as u128, 1u16);
        // C s_66_2: const #1u : u8
        let s_66_2: bool = true;
        // C s_66_3: cast zx s_66_2 -> bv
        let s_66_3: Bits = Bits::new(s_66_2 as u128, 1u16);
        // D s_66_4: cmp-eq s_66_1 s_66_3
        let s_66_4: bool = ((s_66_1) == (s_66_3));
        // D s_66_5: write-var gs#91322 <= s_66_4
        fn_state.gs_91322 = s_66_4;
        // N s_66_6: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_67_0: read-var gs#91322:u8
        let s_67_0: bool = fn_state.gs_91322;
        // D s_67_1: write-var gs#91323 <= s_67_0
        fn_state.gs_91323 = s_67_0;
        // N s_67_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_68_0: const #1u : u8
        let s_68_0: bool = true;
        // D s_68_1: write-var gs#91322 <= s_68_0
        fn_state.gs_91322 = s_68_0;
        // N s_68_2: jump b67
        return block_67(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #146u : u32
        let s_69_0: u32 = 146;
        // S s_69_1: call IsFeatureImplemented(s_69_0)
        let s_69_1: bool = IsFeatureImplemented(state, tracer, s_69_0);
        // D s_69_2: write-var gs#91321 <= s_69_1
        fn_state.gs_91321 = s_69_1;
        // N s_69_3: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_70_0: panic
        panic!("{:?}", ());
        // N s_70_1: return
        return;
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_71_0: read-var __MDCR_EL3_TPM:u8
        let s_71_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_71_1: cast zx s_71_0 -> bv
        let s_71_1: Bits = Bits::new(s_71_0 as u128, 1u16);
        // C s_71_2: const #1u : u8
        let s_71_2: bool = true;
        // C s_71_3: cast zx s_71_2 -> bv
        let s_71_3: Bits = Bits::new(s_71_2 as u128, 1u16);
        // D s_71_4: cmp-eq s_71_1 s_71_3
        let s_71_4: bool = ((s_71_1) == (s_71_3));
        // D s_71_5: write-var gs#91320 <= s_71_4
        fn_state.gs_91320 = s_71_4;
        // N s_71_6: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_72_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_72_1: call __IMPDEF_boolean(s_72_0)
        let s_72_1: bool = u__IMPDEF_boolean(state, tracer, s_72_0);
        // D s_72_2: write-var gs#91319 <= s_72_1
        fn_state.gs_91319 = s_72_1;
        // N s_72_3: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_73_0: read-var __EDSCR_SDD:u8
        let s_73_0: bool = fn_state.u__EDSCR_SDD;
        // D s_73_1: cast zx s_73_0 -> bv
        let s_73_1: Bits = Bits::new(s_73_0 as u128, 1u16);
        // C s_73_2: const #1u : u8
        let s_73_2: bool = true;
        // C s_73_3: cast zx s_73_2 -> bv
        let s_73_3: Bits = Bits::new(s_73_2 as u128, 1u16);
        // D s_73_4: cmp-eq s_73_1 s_73_3
        let s_73_4: bool = ((s_73_1) == (s_73_3));
        // D s_73_5: write-var gs#91318 <= s_73_4
        fn_state.gs_91318 = s_73_4;
        // N s_73_6: jump b35
        return block_35(state, tracer, fn_state);
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
        // C s_74_2: const #2u : u8
        let s_74_2: u8 = 2;
        // D s_74_3: cmp-lt s_74_1 s_74_2
        let s_74_3: bool = ((s_74_1) < (s_74_2));
        // D s_74_4: write-var gs#91317 <= s_74_3
        fn_state.gs_91317 = s_74_3;
        // N s_74_5: jump b33
        return block_33(state, tracer, fn_state);
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
        // N s_75_2: branch s_75_1 b137 b76
        if s_75_1 {
            return block_137(state, tracer, fn_state);
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
        // D s_76_1: write-var gs#91329 <= s_76_0
        fn_state.gs_91329 = s_76_0;
        // N s_76_2: jump b77
        return block_77(state, tracer, fn_state);
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_77_0: read-var gs#91329:u8
        let s_77_0: bool = fn_state.gs_91329;
        // N s_77_1: branch s_77_0 b136 b78
        if s_77_0 {
            return block_136(state, tracer, fn_state);
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
        // D s_78_1: write-var gs#91330 <= s_78_0
        fn_state.gs_91330 = s_78_0;
        // N s_78_2: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_79_0: read-var gs#91330:u8
        let s_79_0: bool = fn_state.gs_91330;
        // N s_79_1: branch s_79_0 b135 b80
        if s_79_0 {
            return block_135(state, tracer, fn_state);
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
        // D s_80_1: write-var gs#91331 <= s_80_0
        fn_state.gs_91331 = s_80_0;
        // N s_80_2: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_81_0: read-var gs#91331:u8
        let s_81_0: bool = fn_state.gs_91331;
        // N s_81_1: branch s_81_0 b134 b82
        if s_81_0 {
            return block_134(state, tracer, fn_state);
        } else {
            return block_82(state, tracer, fn_state);
        };
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_82_0: const #0u : u8
        let s_82_0: bool = false;
        // D s_82_1: write-var gs#91332 <= s_82_0
        fn_state.gs_91332 = s_82_0;
        // N s_82_2: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_83_0: read-var gs#91332:u8
        let s_83_0: bool = fn_state.gs_91332;
        // N s_83_1: branch s_83_0 b133 b84
        if s_83_0 {
            return block_133(state, tracer, fn_state);
        } else {
            return block_84(state, tracer, fn_state);
        };
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_84_0: const #204u : u32
        let s_84_0: u32 = 204;
        // S s_84_1: call IsFeatureImplemented(s_84_0)
        let s_84_1: bool = IsFeatureImplemented(state, tracer, s_84_0);
        // N s_84_2: branch s_84_1 b132 b85
        if s_84_1 {
            return block_132(state, tracer, fn_state);
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
        // D s_85_1: write-var gs#91333 <= s_85_0
        fn_state.gs_91333 = s_85_0;
        // N s_85_2: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_86_0: read-var gs#91333:u8
        let s_86_0: bool = fn_state.gs_91333;
        // N s_86_1: branch s_86_0 b131 b87
        if s_86_0 {
            return block_131(state, tracer, fn_state);
        } else {
            return block_87(state, tracer, fn_state);
        };
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_87_0: const #204u : u32
        let s_87_0: u32 = 204;
        // S s_87_1: call IsFeatureImplemented(s_87_0)
        let s_87_1: bool = IsFeatureImplemented(state, tracer, s_87_0);
        // S s_87_2: not s_87_1
        let s_87_2: bool = !s_87_1;
        // N s_87_3: branch s_87_2 b130 b88
        if s_87_2 {
            return block_130(state, tracer, fn_state);
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
        // D s_88_1: write-var gs#91334 <= s_88_0
        fn_state.gs_91334 = s_88_0;
        // N s_88_2: jump b89
        return block_89(state, tracer, fn_state);
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_89_0: read-var gs#91334:u8
        let s_89_0: bool = fn_state.gs_91334;
        // D s_89_1: write-var gs#91335 <= s_89_0
        fn_state.gs_91335 = s_89_0;
        // N s_89_2: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_90_0: read-var gs#91335:u8
        let s_90_0: bool = fn_state.gs_91335;
        // N s_90_1: branch s_90_0 b124 b91
        if s_90_0 {
            return block_124(state, tracer, fn_state);
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
        // N s_91_2: branch s_91_1 b123 b92
        if s_91_1 {
            return block_123(state, tracer, fn_state);
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
        // D s_92_1: write-var gs#91336 <= s_92_0
        fn_state.gs_91336 = s_92_0;
        // N s_92_2: jump b93
        return block_93(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_93_0: read-var gs#91336:u8
        let s_93_0: bool = fn_state.gs_91336;
        // N s_93_1: branch s_93_0 b122 b94
        if s_93_0 {
            return block_122(state, tracer, fn_state);
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
        // D s_94_1: write-var gs#91337 <= s_94_0
        fn_state.gs_91337 = s_94_0;
        // N s_94_2: jump b95
        return block_95(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_95_0: read-var gs#91337:u8
        let s_95_0: bool = fn_state.gs_91337;
        // N s_95_1: branch s_95_0 b118 b96
        if s_95_0 {
            return block_118(state, tracer, fn_state);
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
        // D s_96_1: write-var gs#91339 <= s_96_0
        fn_state.gs_91339 = s_96_0;
        // N s_96_2: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_97_0: read-var gs#91339:u8
        let s_97_0: bool = fn_state.gs_91339;
        // N s_97_1: branch s_97_0 b117 b98
        if s_97_0 {
            return block_117(state, tracer, fn_state);
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
        // D s_98_1: write-var gs#91340 <= s_98_0
        fn_state.gs_91340 = s_98_0;
        // N s_98_2: jump b99
        return block_99(state, tracer, fn_state);
    }
    fn block_99<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_99_0: read-var gs#91340:u8
        let s_99_0: bool = fn_state.gs_91340;
        // N s_99_1: branch s_99_0 b116 b100
        if s_99_0 {
            return block_116(state, tracer, fn_state);
        } else {
            return block_100(state, tracer, fn_state);
        };
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_100_0: const #() : ()
        let s_100_0: () = ();
        // S s_100_1: call EL2Enabled(s_100_0)
        let s_100_1: bool = EL2Enabled(state, tracer, s_100_0);
        // N s_100_2: branch s_100_1 b115 b101
        if s_100_1 {
            return block_115(state, tracer, fn_state);
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
        // D s_101_1: write-var gs#91341 <= s_101_0
        fn_state.gs_91341 = s_101_0;
        // N s_101_2: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_102<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_102_0: read-var gs#91341:u8
        let s_102_0: bool = fn_state.gs_91341;
        // N s_102_1: branch s_102_0 b114 b103
        if s_102_0 {
            return block_114(state, tracer, fn_state);
        } else {
            return block_103(state, tracer, fn_state);
        };
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_103_0: const #424u : u32
        let s_103_0: u32 = 424;
        // D s_103_1: read-reg s_103_0:u8
        let s_103_1: u8 = {
            let value = state.read_register::<u8>(s_103_0 as isize);
            tracer.read_register(s_103_0 as isize, value);
            value
        };
        // C s_103_2: const #2u : u8
        let s_103_2: u8 = 2;
        // D s_103_3: cmp-lt s_103_1 s_103_2
        let s_103_3: bool = ((s_103_1) < (s_103_2));
        // N s_103_4: branch s_103_3 b113 b104
        if s_103_3 {
            return block_113(state, tracer, fn_state);
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
        // D s_104_1: write-var gs#91342 <= s_104_0
        fn_state.gs_91342 = s_104_0;
        // N s_104_2: jump b105
        return block_105(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_105_0: read-var gs#91342:u8
        let s_105_0: bool = fn_state.gs_91342;
        // N s_105_1: branch s_105_0 b107 b106
        if s_105_0 {
            return block_107(state, tracer, fn_state);
        } else {
            return block_106(state, tracer, fn_state);
        };
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_106_0: const #64s : i64
        let s_106_0: i64 = 64;
        // D s_106_1: read-var t:i
        let s_106_1: i128 = fn_state.t;
        // D s_106_2: call X_read(s_106_1, s_106_0)
        let s_106_2: Bits = X_read(state, tracer, s_106_1, s_106_0);
        // D s_106_3: cast reint s_106_2 -> u64
        let s_106_3: u64 = (s_106_2.value() as u64);
        // D s_106_4: call Mk_PMSWINC_EL0_Type(s_106_3)
        let s_106_4: ProductType5c790c8ef59cc8b2 = Mk_PMSWINC_EL0_Type(
            state,
            tracer,
            s_106_3,
        );
        // C s_106_5: const #103944u : u32
        let s_106_5: u32 = 103944;
        // N s_106_6: write-reg s_106_5 <= s_106_4
        let s_106_6: () = {
            state
                .write_register::<
                    ProductType5c790c8ef59cc8b2,
                >(s_106_5 as isize, s_106_4);
            tracer.write_register(s_106_5 as isize, s_106_4);
        };
        // N s_106_7: return
        return;
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_107_0: const #() : ()
        let s_107_0: () = ();
        // S s_107_1: call Halted(s_107_0)
        let s_107_1: bool = Halted(state, tracer, s_107_0);
        // N s_107_2: branch s_107_1 b112 b108
        if s_107_1 {
            return block_112(state, tracer, fn_state);
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
        // D s_108_1: write-var gs#91344 <= s_108_0
        fn_state.gs_91344 = s_108_0;
        // N s_108_2: jump b109
        return block_109(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_109_0: read-var gs#91344:u8
        let s_109_0: bool = fn_state.gs_91344;
        // N s_109_1: branch s_109_0 b111 b110
        if s_109_0 {
            return block_111(state, tracer, fn_state);
        } else {
            return block_110(state, tracer, fn_state);
        };
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_110_0: const #24u : u8
        let s_110_0: u8 = 24;
        // C s_110_1: cast zx s_110_0 -> bv
        let s_110_1: Bits = Bits::new(s_110_0 as u128, 8u16);
        // C s_110_2: cast zx s_110_1 -> i
        let s_110_2: i128 = (s_110_1.value() as i128);
        // C s_110_3: cast reint s_110_2 -> i64
        let s_110_3: i64 = (s_110_2 as i64);
        // C s_110_4: cast zx s_110_3 -> i
        let s_110_4: i128 = (i128::try_from(s_110_3).unwrap());
        // C s_110_5: const #424u : u32
        let s_110_5: u32 = 424;
        // D s_110_6: read-reg s_110_5:u8
        let s_110_6: u8 = {
            let value = state.read_register::<u8>(s_110_5 as isize);
            tracer.read_register(s_110_5 as isize, value);
            value
        };
        // D s_110_7: call AArch64_SystemAccessTrap(s_110_6, s_110_4)
        let s_110_7: () = AArch64_SystemAccessTrap(state, tracer, s_110_6, s_110_4);
        // N s_110_8: return
        return;
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_111_0: panic
        panic!("{:?}", ());
        // N s_111_1: return
        return;
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_112_0: read-var __EDSCR_SDD:u8
        let s_112_0: bool = fn_state.u__EDSCR_SDD;
        // D s_112_1: cast zx s_112_0 -> bv
        let s_112_1: Bits = Bits::new(s_112_0 as u128, 1u16);
        // C s_112_2: const #1u : u8
        let s_112_2: bool = true;
        // C s_112_3: cast zx s_112_2 -> bv
        let s_112_3: Bits = Bits::new(s_112_2 as u128, 1u16);
        // D s_112_4: cmp-eq s_112_1 s_112_3
        let s_112_4: bool = ((s_112_1) == (s_112_3));
        // D s_112_5: write-var gs#91344 <= s_112_4
        fn_state.gs_91344 = s_112_4;
        // N s_112_6: jump b109
        return block_109(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_113_0: read-var __MDCR_EL3_TPM:u8
        let s_113_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_113_1: cast zx s_113_0 -> bv
        let s_113_1: Bits = Bits::new(s_113_0 as u128, 1u16);
        // C s_113_2: const #1u : u8
        let s_113_2: bool = true;
        // C s_113_3: cast zx s_113_2 -> bv
        let s_113_3: Bits = Bits::new(s_113_2 as u128, 1u16);
        // D s_113_4: cmp-eq s_113_1 s_113_3
        let s_113_4: bool = ((s_113_1) == (s_113_3));
        // D s_113_5: write-var gs#91342 <= s_113_4
        fn_state.gs_91342 = s_113_4;
        // N s_113_6: jump b105
        return block_105(state, tracer, fn_state);
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
        // D s_115_0: read-var __MDCR_EL2_TPM:u8
        let s_115_0: bool = fn_state.u__MDCR_EL2_TPM;
        // D s_115_1: cast zx s_115_0 -> bv
        let s_115_1: Bits = Bits::new(s_115_0 as u128, 1u16);
        // C s_115_2: const #1u : u8
        let s_115_2: bool = true;
        // C s_115_3: cast zx s_115_2 -> bv
        let s_115_3: Bits = Bits::new(s_115_2 as u128, 1u16);
        // D s_115_4: cmp-eq s_115_1 s_115_3
        let s_115_4: bool = ((s_115_1) == (s_115_3));
        // D s_115_5: write-var gs#91341 <= s_115_4
        fn_state.gs_91341 = s_115_4;
        // N s_115_6: jump b102
        return block_102(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_116_0: const #24u : u8
        let s_116_0: u8 = 24;
        // C s_116_1: cast zx s_116_0 -> bv
        let s_116_1: Bits = Bits::new(s_116_0 as u128, 8u16);
        // C s_116_2: cast zx s_116_1 -> i
        let s_116_2: i128 = (s_116_1.value() as i128);
        // C s_116_3: cast reint s_116_2 -> i64
        let s_116_3: i64 = (s_116_2 as i64);
        // C s_116_4: cast zx s_116_3 -> i
        let s_116_4: i128 = (i128::try_from(s_116_3).unwrap());
        // C s_116_5: const #432u : u32
        let s_116_5: u32 = 432;
        // D s_116_6: read-reg s_116_5:u8
        let s_116_6: u8 = {
            let value = state.read_register::<u8>(s_116_5 as isize);
            tracer.read_register(s_116_5 as isize, value);
            value
        };
        // D s_116_7: call AArch64_SystemAccessTrap(s_116_6, s_116_4)
        let s_116_7: () = AArch64_SystemAccessTrap(state, tracer, s_116_6, s_116_4);
        // N s_116_8: return
        return;
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_117_0: read-var __HDFGWTR_EL2_PMSWINC_EL0:u8
        let s_117_0: bool = fn_state.u__HDFGWTR_EL2_PMSWINC_EL0;
        // D s_117_1: cast zx s_117_0 -> bv
        let s_117_1: Bits = Bits::new(s_117_0 as u128, 1u16);
        // C s_117_2: const #1u : u8
        let s_117_2: bool = true;
        // C s_117_3: cast zx s_117_2 -> bv
        let s_117_3: Bits = Bits::new(s_117_2 as u128, 1u16);
        // D s_117_4: cmp-eq s_117_1 s_117_3
        let s_117_4: bool = ((s_117_1) == (s_117_3));
        // D s_117_5: write-var gs#91340 <= s_117_4
        fn_state.gs_91340 = s_117_4;
        // N s_117_6: jump b99
        return block_99(state, tracer, fn_state);
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_118_0: const #424u : u32
        let s_118_0: u32 = 424;
        // D s_118_1: read-reg s_118_0:u8
        let s_118_1: u8 = {
            let value = state.read_register::<u8>(s_118_0 as isize);
            tracer.read_register(s_118_0 as isize, value);
            value
        };
        // C s_118_2: const #2u : u8
        let s_118_2: u8 = 2;
        // D s_118_3: cmp-lt s_118_1 s_118_2
        let s_118_3: bool = ((s_118_1) < (s_118_2));
        // D s_118_4: not s_118_3
        let s_118_4: bool = !s_118_3;
        // N s_118_5: branch s_118_4 b121 b119
        if s_118_4 {
            return block_121(state, tracer, fn_state);
        } else {
            return block_119(state, tracer, fn_state);
        };
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_119_0: read-var __SCR_EL3_FGTEn:u8
        let s_119_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_119_1: cast zx s_119_0 -> bv
        let s_119_1: Bits = Bits::new(s_119_0 as u128, 1u16);
        // C s_119_2: const #1u : u8
        let s_119_2: bool = true;
        // C s_119_3: cast zx s_119_2 -> bv
        let s_119_3: Bits = Bits::new(s_119_2 as u128, 1u16);
        // D s_119_4: cmp-eq s_119_1 s_119_3
        let s_119_4: bool = ((s_119_1) == (s_119_3));
        // D s_119_5: write-var gs#91338 <= s_119_4
        fn_state.gs_91338 = s_119_4;
        // N s_119_6: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_120_0: read-var gs#91338:u8
        let s_120_0: bool = fn_state.gs_91338;
        // D s_120_1: write-var gs#91339 <= s_120_0
        fn_state.gs_91339 = s_120_0;
        // N s_120_2: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_121_0: const #1u : u8
        let s_121_0: bool = true;
        // D s_121_1: write-var gs#91338 <= s_121_0
        fn_state.gs_91338 = s_121_0;
        // N s_121_2: jump b120
        return block_120(state, tracer, fn_state);
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_122_0: const #146u : u32
        let s_122_0: u32 = 146;
        // S s_122_1: call IsFeatureImplemented(s_122_0)
        let s_122_1: bool = IsFeatureImplemented(state, tracer, s_122_0);
        // D s_122_2: write-var gs#91337 <= s_122_1
        fn_state.gs_91337 = s_122_1;
        // N s_122_3: jump b95
        return block_95(state, tracer, fn_state);
    }
    fn block_123<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_123_0: const #102552u : u32
        let s_123_0: u32 = 102552;
        // D s_123_1: read-reg s_123_0:struct
        let s_123_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_123_0 as isize);
            tracer.read_register(s_123_0 as isize, value);
            value
        };
        // D s_123_2: call _get_HCR_EL2_Type_E2H(s_123_1)
        let s_123_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_123_1);
        // C s_123_3: const #102552u : u32
        let s_123_3: u32 = 102552;
        // D s_123_4: read-reg s_123_3:struct
        let s_123_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_123_3 as isize);
            tracer.read_register(s_123_3 as isize, value);
            value
        };
        // D s_123_5: call _get_HCR_EL2_Type_TGE(s_123_4)
        let s_123_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_123_4);
        // D s_123_6: cast zx s_123_2 -> bv
        let s_123_6: Bits = Bits::new(s_123_2 as u128, 1u16);
        // D s_123_7: cast zx s_123_5 -> bv
        let s_123_7: Bits = Bits::new(s_123_5 as u128, 1u16);
        // D s_123_8: cast reint s_123_6 -> u128
        let s_123_8: u128 = (s_123_6.value() as u128);
        // D s_123_9: size-of s_123_6
        let s_123_9: u16 = s_123_6.length();
        // D s_123_10: cast reint s_123_7 -> u128
        let s_123_10: u128 = (s_123_7.value() as u128);
        // D s_123_11: size-of s_123_7
        let s_123_11: u16 = s_123_7.length();
        // D s_123_12: lsl s_123_8 s_123_11
        let s_123_12: u128 = s_123_8 << s_123_11;
        // D s_123_13: or s_123_12 s_123_10
        let s_123_13: u128 = ((s_123_12) | (s_123_10));
        // D s_123_14: add s_123_9 s_123_11
        let s_123_14: u16 = (s_123_9 + s_123_11);
        // D s_123_15: create-bits s_123_13 s_123_14
        let s_123_15: Bits = Bits::new(s_123_13, s_123_14);
        // D s_123_16: cast reint s_123_15 -> u8
        let s_123_16: u8 = (s_123_15.value() as u8);
        // D s_123_17: cast zx s_123_16 -> bv
        let s_123_17: Bits = Bits::new(s_123_16 as u128, 2u16);
        // C s_123_18: const #3u : u8
        let s_123_18: u8 = 3;
        // C s_123_19: cast zx s_123_18 -> bv
        let s_123_19: Bits = Bits::new(s_123_18 as u128, 2u16);
        // D s_123_20: cmp-ne s_123_17 s_123_19
        let s_123_20: bool = ((s_123_17) != (s_123_19));
        // D s_123_21: write-var gs#91336 <= s_123_20
        fn_state.gs_91336 = s_123_20;
        // N s_123_22: jump b93
        return block_93(state, tracer, fn_state);
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
        // N s_124_2: branch s_124_1 b129 b125
        if s_124_1 {
            return block_129(state, tracer, fn_state);
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
        // D s_125_1: write-var gs#91345 <= s_125_0
        fn_state.gs_91345 = s_125_0;
        // N s_125_2: jump b126
        return block_126(state, tracer, fn_state);
    }
    fn block_126<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_126_0: read-var gs#91345:u8
        let s_126_0: bool = fn_state.gs_91345;
        // N s_126_1: branch s_126_0 b128 b127
        if s_126_0 {
            return block_128(state, tracer, fn_state);
        } else {
            return block_127(state, tracer, fn_state);
        };
    }
    fn block_127<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_127_0: const #24u : u8
        let s_127_0: u8 = 24;
        // C s_127_1: cast zx s_127_0 -> bv
        let s_127_1: Bits = Bits::new(s_127_0 as u128, 8u16);
        // C s_127_2: cast zx s_127_1 -> i
        let s_127_2: i128 = (s_127_1.value() as i128);
        // C s_127_3: cast reint s_127_2 -> i64
        let s_127_3: i64 = (s_127_2 as i64);
        // C s_127_4: cast zx s_127_3 -> i
        let s_127_4: i128 = (i128::try_from(s_127_3).unwrap());
        // C s_127_5: const #440u : u32
        let s_127_5: u32 = 440;
        // D s_127_6: read-reg s_127_5:u8
        let s_127_6: u8 = {
            let value = state.read_register::<u8>(s_127_5 as isize);
            tracer.read_register(s_127_5 as isize, value);
            value
        };
        // D s_127_7: call AArch64_SystemAccessTrap(s_127_6, s_127_4)
        let s_127_7: () = AArch64_SystemAccessTrap(state, tracer, s_127_6, s_127_4);
        // N s_127_8: return
        return;
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
        // D s_129_0: read-var __HCR_EL2_TGE:u8
        let s_129_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_129_1: cast zx s_129_0 -> bv
        let s_129_1: Bits = Bits::new(s_129_0 as u128, 1u16);
        // C s_129_2: const #1u : u8
        let s_129_2: bool = true;
        // C s_129_3: cast zx s_129_2 -> bv
        let s_129_3: Bits = Bits::new(s_129_2 as u128, 1u16);
        // D s_129_4: cmp-eq s_129_1 s_129_3
        let s_129_4: bool = ((s_129_1) == (s_129_3));
        // D s_129_5: write-var gs#91345 <= s_129_4
        fn_state.gs_91345 = s_129_4;
        // N s_129_6: jump b126
        return block_126(state, tracer, fn_state);
    }
    fn block_130<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_130_0: const #102624u : u32
        let s_130_0: u32 = 102624;
        // D s_130_1: read-reg s_130_0:struct
        let s_130_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_130_0 as isize);
            tracer.read_register(s_130_0 as isize, value);
            value
        };
        // D s_130_2: call _get_PMUSERENR_EL0_Type_SW(s_130_1)
        let s_130_2: bool = u_get_PMUSERENR_EL0_Type_SW(state, tracer, s_130_1);
        // C s_130_3: const #102624u : u32
        let s_130_3: u32 = 102624;
        // D s_130_4: read-reg s_130_3:struct
        let s_130_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_130_3 as isize);
            tracer.read_register(s_130_3 as isize, value);
            value
        };
        // D s_130_5: call _get_PMUSERENR_EL0_Type_EN(s_130_4)
        let s_130_5: bool = u_get_PMUSERENR_EL0_Type_EN(state, tracer, s_130_4);
        // D s_130_6: cast zx s_130_2 -> bv
        let s_130_6: Bits = Bits::new(s_130_2 as u128, 1u16);
        // D s_130_7: cast zx s_130_5 -> bv
        let s_130_7: Bits = Bits::new(s_130_5 as u128, 1u16);
        // D s_130_8: cast reint s_130_6 -> u128
        let s_130_8: u128 = (s_130_6.value() as u128);
        // D s_130_9: size-of s_130_6
        let s_130_9: u16 = s_130_6.length();
        // D s_130_10: cast reint s_130_7 -> u128
        let s_130_10: u128 = (s_130_7.value() as u128);
        // D s_130_11: size-of s_130_7
        let s_130_11: u16 = s_130_7.length();
        // D s_130_12: lsl s_130_8 s_130_11
        let s_130_12: u128 = s_130_8 << s_130_11;
        // D s_130_13: or s_130_12 s_130_10
        let s_130_13: u128 = ((s_130_12) | (s_130_10));
        // D s_130_14: add s_130_9 s_130_11
        let s_130_14: u16 = (s_130_9 + s_130_11);
        // D s_130_15: create-bits s_130_13 s_130_14
        let s_130_15: Bits = Bits::new(s_130_13, s_130_14);
        // D s_130_16: cast reint s_130_15 -> u8
        let s_130_16: u8 = (s_130_15.value() as u8);
        // D s_130_17: cast zx s_130_16 -> bv
        let s_130_17: Bits = Bits::new(s_130_16 as u128, 2u16);
        // C s_130_18: const #0u : u8
        let s_130_18: u8 = 0;
        // C s_130_19: cast zx s_130_18 -> bv
        let s_130_19: Bits = Bits::new(s_130_18 as u128, 2u16);
        // D s_130_20: cmp-eq s_130_17 s_130_19
        let s_130_20: bool = ((s_130_17) == (s_130_19));
        // D s_130_21: write-var gs#91334 <= s_130_20
        fn_state.gs_91334 = s_130_20;
        // N s_130_22: jump b89
        return block_89(state, tracer, fn_state);
    }
    fn block_131<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_131_0: const #1u : u8
        let s_131_0: bool = true;
        // D s_131_1: write-var gs#91335 <= s_131_0
        fn_state.gs_91335 = s_131_0;
        // N s_131_2: jump b90
        return block_90(state, tracer, fn_state);
    }
    fn block_132<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_132_0: const #102624u : u32
        let s_132_0: u32 = 102624;
        // D s_132_1: read-reg s_132_0:struct
        let s_132_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_132_0 as isize);
            tracer.read_register(s_132_0 as isize, value);
            value
        };
        // D s_132_2: call _get_PMUSERENR_EL0_Type_UEN(s_132_1)
        let s_132_2: bool = u_get_PMUSERENR_EL0_Type_UEN(state, tracer, s_132_1);
        // C s_132_3: const #102624u : u32
        let s_132_3: u32 = 102624;
        // D s_132_4: read-reg s_132_3:struct
        let s_132_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_132_3 as isize);
            tracer.read_register(s_132_3 as isize, value);
            value
        };
        // D s_132_5: call _get_PMUSERENR_EL0_Type_SW(s_132_4)
        let s_132_5: bool = u_get_PMUSERENR_EL0_Type_SW(state, tracer, s_132_4);
        // C s_132_6: const #102624u : u32
        let s_132_6: u32 = 102624;
        // D s_132_7: read-reg s_132_6:struct
        let s_132_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_132_6 as isize);
            tracer.read_register(s_132_6 as isize, value);
            value
        };
        // D s_132_8: call _get_PMUSERENR_EL0_Type_EN(s_132_7)
        let s_132_8: bool = u_get_PMUSERENR_EL0_Type_EN(state, tracer, s_132_7);
        // D s_132_9: cast zx s_132_5 -> bv
        let s_132_9: Bits = Bits::new(s_132_5 as u128, 1u16);
        // D s_132_10: cast zx s_132_8 -> bv
        let s_132_10: Bits = Bits::new(s_132_8 as u128, 1u16);
        // D s_132_11: cast reint s_132_9 -> u128
        let s_132_11: u128 = (s_132_9.value() as u128);
        // D s_132_12: size-of s_132_9
        let s_132_12: u16 = s_132_9.length();
        // D s_132_13: cast reint s_132_10 -> u128
        let s_132_13: u128 = (s_132_10.value() as u128);
        // D s_132_14: size-of s_132_10
        let s_132_14: u16 = s_132_10.length();
        // D s_132_15: lsl s_132_11 s_132_14
        let s_132_15: u128 = s_132_11 << s_132_14;
        // D s_132_16: or s_132_15 s_132_13
        let s_132_16: u128 = ((s_132_15) | (s_132_13));
        // D s_132_17: add s_132_12 s_132_14
        let s_132_17: u16 = (s_132_12 + s_132_14);
        // D s_132_18: create-bits s_132_16 s_132_17
        let s_132_18: Bits = Bits::new(s_132_16, s_132_17);
        // D s_132_19: cast reint s_132_18 -> u8
        let s_132_19: u8 = (s_132_18.value() as u8);
        // D s_132_20: cast zx s_132_2 -> bv
        let s_132_20: Bits = Bits::new(s_132_2 as u128, 1u16);
        // D s_132_21: cast zx s_132_19 -> bv
        let s_132_21: Bits = Bits::new(s_132_19 as u128, 2u16);
        // D s_132_22: cast reint s_132_20 -> u128
        let s_132_22: u128 = (s_132_20.value() as u128);
        // D s_132_23: size-of s_132_20
        let s_132_23: u16 = s_132_20.length();
        // D s_132_24: cast reint s_132_21 -> u128
        let s_132_24: u128 = (s_132_21.value() as u128);
        // D s_132_25: size-of s_132_21
        let s_132_25: u16 = s_132_21.length();
        // D s_132_26: lsl s_132_22 s_132_25
        let s_132_26: u128 = s_132_22 << s_132_25;
        // D s_132_27: or s_132_26 s_132_24
        let s_132_27: u128 = ((s_132_26) | (s_132_24));
        // D s_132_28: add s_132_23 s_132_25
        let s_132_28: u16 = (s_132_23 + s_132_25);
        // D s_132_29: create-bits s_132_27 s_132_28
        let s_132_29: Bits = Bits::new(s_132_27, s_132_28);
        // D s_132_30: cast reint s_132_29 -> u8
        let s_132_30: u8 = (s_132_29.value() as u8);
        // D s_132_31: cast zx s_132_30 -> bv
        let s_132_31: Bits = Bits::new(s_132_30 as u128, 3u16);
        // C s_132_32: const #0u : u8
        let s_132_32: u8 = 0;
        // C s_132_33: cast zx s_132_32 -> bv
        let s_132_33: Bits = Bits::new(s_132_32 as u128, 3u16);
        // D s_132_34: cmp-eq s_132_31 s_132_33
        let s_132_34: bool = ((s_132_31) == (s_132_33));
        // D s_132_35: write-var gs#91333 <= s_132_34
        fn_state.gs_91333 = s_132_34;
        // N s_132_36: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_133<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_133_0: panic
        panic!("{:?}", ());
        // N s_133_1: return
        return;
    }
    fn block_134<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_134_0: read-var __MDCR_EL3_TPM:u8
        let s_134_0: bool = fn_state.u__MDCR_EL3_TPM;
        // D s_134_1: cast zx s_134_0 -> bv
        let s_134_1: Bits = Bits::new(s_134_0 as u128, 1u16);
        // C s_134_2: const #1u : u8
        let s_134_2: bool = true;
        // C s_134_3: cast zx s_134_2 -> bv
        let s_134_3: Bits = Bits::new(s_134_2 as u128, 1u16);
        // D s_134_4: cmp-eq s_134_1 s_134_3
        let s_134_4: bool = ((s_134_1) == (s_134_3));
        // D s_134_5: write-var gs#91332 <= s_134_4
        fn_state.gs_91332 = s_134_4;
        // N s_134_6: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_135<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_135_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_135_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_135_1: call __IMPDEF_boolean(s_135_0)
        let s_135_1: bool = u__IMPDEF_boolean(state, tracer, s_135_0);
        // D s_135_2: write-var gs#91331 <= s_135_1
        fn_state.gs_91331 = s_135_1;
        // N s_135_3: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_136<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_136_0: read-var __EDSCR_SDD:u8
        let s_136_0: bool = fn_state.u__EDSCR_SDD;
        // D s_136_1: cast zx s_136_0 -> bv
        let s_136_1: Bits = Bits::new(s_136_0 as u128, 1u16);
        // C s_136_2: const #1u : u8
        let s_136_2: bool = true;
        // C s_136_3: cast zx s_136_2 -> bv
        let s_136_3: Bits = Bits::new(s_136_2 as u128, 1u16);
        // D s_136_4: cmp-eq s_136_1 s_136_3
        let s_136_4: bool = ((s_136_1) == (s_136_3));
        // D s_136_5: write-var gs#91330 <= s_136_4
        fn_state.gs_91330 = s_136_4;
        // N s_136_6: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_137<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_137_0: const #424u : u32
        let s_137_0: u32 = 424;
        // D s_137_1: read-reg s_137_0:u8
        let s_137_1: u8 = {
            let value = state.read_register::<u8>(s_137_0 as isize);
            tracer.read_register(s_137_0 as isize, value);
            value
        };
        // C s_137_2: const #2u : u8
        let s_137_2: u8 = 2;
        // D s_137_3: cmp-lt s_137_1 s_137_2
        let s_137_3: bool = ((s_137_1) < (s_137_2));
        // D s_137_4: write-var gs#91329 <= s_137_3
        fn_state.gs_91329 = s_137_3;
        // N s_137_5: jump b77
        return block_77(state, tracer, fn_state);
    }
}
