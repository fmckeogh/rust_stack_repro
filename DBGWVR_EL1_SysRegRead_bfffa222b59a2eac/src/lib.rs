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
use u_get_HDFGRTR_EL2_Type_DBGWVRn_EL1::*;
use u__get_DBGWVR_EL1::*;
use IsFeatureImplemented::*;
use AArch64_SystemAccessTrap::*;
use u__IMPDEF_boolean::*;
use u_get_MDCR_EL2_Type_TDE::*;
use u_get_MDSELR_EL1_Type_BANK::*;
use X_set::*;
use HaltingAllowed::*;
use u_get_MDCR_EL3_Type_TDA::*;
use u_get_OSLSR_EL1_Type_OSLK::*;
use u_get_EDSCR_Type_SDD::*;
use u_get_EDSCR_Type_TDA::*;
use Halt::*;
use EL2Enabled::*;
use EffectiveMDSELR_EL1_BANK::*;
use EDSCR_read::*;
use u_get_MDCR_EL2_Type_TDA::*;
use common::*;
pub fn DBGWVR_EL1_SysRegRead_bfffa222b59a2eac<T: Tracer>(
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
        ga_64141: ProductType5c790c8ef59cc8b2,
        gs_61076: bool,
        gs_61080: bool,
        gs_61104: bool,
        gs_61074: bool,
        ga_64137: ProductType5c790c8ef59cc8b2,
        gs_61097: bool,
        gs_61098: bool,
        ga_64159: ProductType5c790c8ef59cc8b2,
        gs_61094: bool,
        gs_61067: bool,
        gs_61078: bool,
        gs_61096: bool,
        gs_61087: bool,
        ga_64111: ProductType5c790c8ef59cc8b2,
        u__PSTATE_EL: u8,
        ga_64107: ProductType5c790c8ef59cc8b2,
        u__EDSCR_TDA: bool,
        gs_61093: bool,
        gs_61079: bool,
        gs_61091: bool,
        u__HDFGRTR_EL2_DBGWVRn_EL1: bool,
        u__EDSCR_SDD: bool,
        u__SCR_EL3_FGTEn: bool,
        gs_61095: bool,
        gs_61086: bool,
        u__MDCR_EL3_TDA: bool,
        gs_61077: bool,
        gs_61088: bool,
        gs_61092: bool,
        gs_61066: bool,
        gs_61090: bool,
        gs_61062: bool,
        gs_61075: bool,
        gs_61065: bool,
        gs_61068: bool,
        gs_61089: bool,
        ga_64155: ProductType5c790c8ef59cc8b2,
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
        // D s_0_9: call _get_MDCR_EL3_Type_TDA(s_0_8)
        let s_0_9: bool = u_get_MDCR_EL3_Type_TDA(state, tracer, s_0_8);
        // D s_0_10: write-var __MDCR_EL3_TDA <= s_0_9
        fn_state.u__MDCR_EL3_TDA = s_0_9;
        // C s_0_11: const #90704u : u32
        let s_0_11: u32 = 90704;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_SCR_EL3_Type_FGTEn(s_0_12)
        let s_0_13: bool = u_get_SCR_EL3_Type_FGTEn(state, tracer, s_0_12);
        // D s_0_14: write-var __SCR_EL3_FGTEn <= s_0_13
        fn_state.u__SCR_EL3_FGTEn = s_0_13;
        // C s_0_15: const #19144u : u32
        let s_0_15: u32 = 19144;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_HDFGRTR_EL2_Type_DBGWVRn_EL1(s_0_16)
        let s_0_17: bool = u_get_HDFGRTR_EL2_Type_DBGWVRn_EL1(state, tracer, s_0_16);
        // D s_0_18: write-var __HDFGRTR_EL2_DBGWVRn_EL1 <= s_0_17
        fn_state.u__HDFGRTR_EL2_DBGWVRn_EL1 = s_0_17;
        // C s_0_19: const #() : ()
        let s_0_19: () = ();
        // S s_0_20: call EDSCR_read(s_0_19)
        let s_0_20: ProductType700c18a878c5601b = EDSCR_read(state, tracer, s_0_19);
        // S s_0_21: call _get_EDSCR_Type_TDA(s_0_20)
        let s_0_21: bool = u_get_EDSCR_Type_TDA(state, tracer, s_0_20);
        // D s_0_22: write-var __EDSCR_TDA <= s_0_21
        fn_state.u__EDSCR_TDA = s_0_21;
        // C s_0_23: const #185u : u32
        let s_0_23: u32 = 185;
        // S s_0_24: call IsFeatureImplemented(s_0_23)
        let s_0_24: bool = IsFeatureImplemented(state, tracer, s_0_23);
        // S s_0_25: not s_0_24
        let s_0_25: bool = !s_0_24;
        // N s_0_26: branch s_0_25 b116 b1
        if s_0_25 {
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
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#61062 <= s_1_0
        fn_state.gs_61062 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#61062:u8
        let s_2_0: bool = fn_state.gs_61062;
        // N s_2_1: branch s_2_0 b115 b3
        if s_2_0 {
            return block_115(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #185u : u32
        let s_3_0: u32 = 185;
        // S s_3_1: call IsFeatureImplemented(s_3_0)
        let s_3_1: bool = IsFeatureImplemented(state, tracer, s_3_0);
        // N s_3_2: branch s_3_1 b114 b4
        if s_3_1 {
            return block_114(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var gs#61065 <= s_4_0
        fn_state.gs_61065 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#61065:u8
        let s_5_0: bool = fn_state.gs_61065;
        // D s_5_1: write-var gs#61066 <= s_5_0
        fn_state.gs_61066 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#61066:u8
        let s_6_0: bool = fn_state.gs_61066;
        // N s_6_1: branch s_6_0 b113 b7
        if s_6_0 {
            return block_113(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var __PSTATE_EL:u8
        let s_7_0: u8 = fn_state.u__PSTATE_EL;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 2u16);
        // C s_7_2: const #448u : u32
        let s_7_2: u32 = 448;
        // D s_7_3: read-reg s_7_2:u8
        let s_7_3: u8 = {
            let value = state.read_register::<u8>(s_7_2 as isize);
            tracer.read_register(s_7_2 as isize, value);
            value
        };
        // D s_7_4: cast zx s_7_3 -> bv
        let s_7_4: Bits = Bits::new(s_7_3 as u128, 2u16);
        // D s_7_5: cmp-eq s_7_1 s_7_4
        let s_7_5: bool = ((s_7_1) == (s_7_4));
        // N s_7_6: branch s_7_5 b112 b8
        if s_7_5 {
            return block_112(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var __PSTATE_EL:u8
        let s_8_0: u8 = fn_state.u__PSTATE_EL;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 2u16);
        // C s_8_2: const #440u : u32
        let s_8_2: u32 = 440;
        // D s_8_3: read-reg s_8_2:u8
        let s_8_3: u8 = {
            let value = state.read_register::<u8>(s_8_2 as isize);
            tracer.read_register(s_8_2 as isize, value);
            value
        };
        // D s_8_4: cast zx s_8_3 -> bv
        let s_8_4: Bits = Bits::new(s_8_3 as u128, 2u16);
        // D s_8_5: cmp-eq s_8_1 s_8_4
        let s_8_5: bool = ((s_8_1) == (s_8_4));
        // N s_8_6: branch s_8_5 b58 b9
        if s_8_5 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var __PSTATE_EL:u8
        let s_9_0: u8 = fn_state.u__PSTATE_EL;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 2u16);
        // C s_9_2: const #432u : u32
        let s_9_2: u32 = 432;
        // D s_9_3: read-reg s_9_2:u8
        let s_9_3: u8 = {
            let value = state.read_register::<u8>(s_9_2 as isize);
            tracer.read_register(s_9_2 as isize, value);
            value
        };
        // D s_9_4: cast zx s_9_3 -> bv
        let s_9_4: Bits = Bits::new(s_9_3 as u128, 2u16);
        // D s_9_5: cmp-eq s_9_1 s_9_4
        let s_9_5: bool = ((s_9_1) == (s_9_4));
        // N s_9_6: branch s_9_5 b23 b10
        if s_9_5 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var __PSTATE_EL:u8
        let s_10_0: u8 = fn_state.u__PSTATE_EL;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 2u16);
        // C s_10_2: const #424u : u32
        let s_10_2: u32 = 424;
        // D s_10_3: read-reg s_10_2:u8
        let s_10_3: u8 = {
            let value = state.read_register::<u8>(s_10_2 as isize);
            tracer.read_register(s_10_2 as isize, value);
            value
        };
        // D s_10_4: cast zx s_10_3 -> bv
        let s_10_4: Bits = Bits::new(s_10_3 as u128, 2u16);
        // D s_10_5: cmp-eq s_10_1 s_10_4
        let s_10_5: bool = ((s_10_1) == (s_10_4));
        // N s_10_6: branch s_10_5 b12 b11
        if s_10_5 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #10128u : u32
        let s_12_0: u32 = 10128;
        // D s_12_1: read-reg s_12_0:struct
        let s_12_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_0 as isize);
            tracer.read_register(s_12_0 as isize, value);
            value
        };
        // D s_12_2: call _get_OSLSR_EL1_Type_OSLK(s_12_1)
        let s_12_2: bool = u_get_OSLSR_EL1_Type_OSLK(state, tracer, s_12_1);
        // D s_12_3: cast zx s_12_2 -> bv
        let s_12_3: Bits = Bits::new(s_12_2 as u128, 1u16);
        // C s_12_4: const #0u : u8
        let s_12_4: bool = false;
        // C s_12_5: cast zx s_12_4 -> bv
        let s_12_5: Bits = Bits::new(s_12_4 as u128, 1u16);
        // D s_12_6: cmp-eq s_12_3 s_12_5
        let s_12_6: bool = ((s_12_3) == (s_12_5));
        // N s_12_7: branch s_12_6 b22 b13
        if s_12_6 {
            return block_22(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#61067 <= s_13_0
        fn_state.gs_61067 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#61067:u8
        let s_14_0: bool = fn_state.gs_61067;
        // N s_14_1: branch s_14_0 b21 b15
        if s_14_0 {
            return block_21(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#61068 <= s_15_0
        fn_state.gs_61068 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#61068:u8
        let s_16_0: bool = fn_state.gs_61068;
        // N s_16_1: branch s_16_0 b20 b17
        if s_16_0 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #185u : u32
        let s_17_0: u32 = 185;
        // S s_17_1: call IsFeatureImplemented(s_17_0)
        let s_17_1: bool = IsFeatureImplemented(state, tracer, s_17_0);
        // N s_17_2: branch s_17_1 b19 b18
        if s_17_1 {
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
        // C s_18_1: const #10s : i
        let s_18_1: i128 = 10;
        // C s_18_2: const #19392u : u32
        let s_18_2: u32 = 19392;
        // D s_18_3: read-reg s_18_2:[struct; 64]
        let s_18_3: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_18_2 as isize);
            tracer.read_register(s_18_2 as isize, value);
            value
        };
        // D s_18_4: read-element s_18_3[s_18_1]
        let s_18_4: ProductType5c790c8ef59cc8b2 = s_18_3[(s_18_1) as usize];
        // D s_18_5: call __get_DBGWVR_EL1(s_18_4)
        let s_18_5: ProductType5c790c8ef59cc8b2 = u__get_DBGWVR_EL1(
            state,
            tracer,
            s_18_4,
        );
        // D s_18_6: write-var ga#64159 <= s_18_5
        fn_state.ga_64159 = s_18_5;
        // D s_18_7: read-var ga#64159.0:struct
        let s_18_7: u64 = fn_state.ga_64159._0;
        // D s_18_8: cast zx s_18_7 -> bv
        let s_18_8: Bits = Bits::new(s_18_7 as u128, 64u16);
        // D s_18_9: read-var t:i
        let s_18_9: i128 = fn_state.t;
        // D s_18_10: call X_set(s_18_9, s_18_0, s_18_8)
        let s_18_10: () = X_set(state, tracer, s_18_9, s_18_0, s_18_8);
        // N s_18_11: return
        return;
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #64s : i64
        let s_19_0: i64 = 64;
        // C s_19_1: const #() : ()
        let s_19_1: () = ();
        // S s_19_2: call EffectiveMDSELR_EL1_BANK(s_19_1)
        let s_19_2: u8 = EffectiveMDSELR_EL1_BANK(state, tracer, s_19_1);
        // S s_19_3: cast zx s_19_2 -> bv
        let s_19_3: Bits = Bits::new(s_19_2 as u128, 2u16);
        // S s_19_4: cast zx s_19_3 -> i
        let s_19_4: i128 = (s_19_3.value() as i128);
        // S s_19_5: cast reint s_19_4 -> i64
        let s_19_5: i64 = (s_19_4 as i64);
        // C s_19_6: const #16s : i
        let s_19_6: i128 = 16;
        // S s_19_7: cast zx s_19_5 -> i
        let s_19_7: i128 = (i128::try_from(s_19_5).unwrap());
        // S s_19_8: mul s_19_7 s_19_6
        let s_19_8: i128 = ((s_19_7) * (s_19_6));
        // S s_19_9: cast reint s_19_8 -> i64
        let s_19_9: i64 = (s_19_8 as i64);
        // C s_19_10: const #10s : i
        let s_19_10: i128 = 10;
        // S s_19_11: cast zx s_19_9 -> i
        let s_19_11: i128 = (i128::try_from(s_19_9).unwrap());
        // S s_19_12: add s_19_10 s_19_11
        let s_19_12: i128 = (s_19_10 + s_19_11);
        // S s_19_13: cast reint s_19_12 -> i64
        let s_19_13: i64 = (s_19_12 as i64);
        // C s_19_14: const #19392u : u32
        let s_19_14: u32 = 19392;
        // D s_19_15: read-reg s_19_14:[struct; 64]
        let s_19_15: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_19_14 as isize);
            tracer.read_register(s_19_14 as isize, value);
            value
        };
        // S s_19_16: cast zx s_19_13 -> i
        let s_19_16: i128 = (i128::try_from(s_19_13).unwrap());
        // D s_19_17: read-element s_19_15[s_19_16]
        let s_19_17: ProductType5c790c8ef59cc8b2 = s_19_15[(s_19_16) as usize];
        // D s_19_18: call __get_DBGWVR_EL1(s_19_17)
        let s_19_18: ProductType5c790c8ef59cc8b2 = u__get_DBGWVR_EL1(
            state,
            tracer,
            s_19_17,
        );
        // D s_19_19: write-var ga#64155 <= s_19_18
        fn_state.ga_64155 = s_19_18;
        // D s_19_20: read-var ga#64155.0:struct
        let s_19_20: u64 = fn_state.ga_64155._0;
        // D s_19_21: cast zx s_19_20 -> bv
        let s_19_21: Bits = Bits::new(s_19_20 as u128, 64u16);
        // D s_19_22: read-var t:i
        let s_19_22: i128 = fn_state.t;
        // D s_19_23: call X_set(s_19_22, s_19_0, s_19_21)
        let s_19_23: () = X_set(state, tracer, s_19_22, s_19_0, s_19_21);
        // N s_19_24: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #1160u : u32
        let s_20_0: u32 = 1160;
        // D s_20_1: read-reg s_20_0:u8
        let s_20_1: u8 = {
            let value = state.read_register::<u8>(s_20_0 as isize);
            tracer.read_register(s_20_0 as isize, value);
            value
        };
        // D s_20_2: call Halt(s_20_1)
        let s_20_2: () = Halt(state, tracer, s_20_1);
        // N s_20_3: return
        return;
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var __EDSCR_TDA:u8
        let s_21_0: bool = fn_state.u__EDSCR_TDA;
        // D s_21_1: cast zx s_21_0 -> bv
        let s_21_1: Bits = Bits::new(s_21_0 as u128, 1u16);
        // C s_21_2: const #1u : u8
        let s_21_2: bool = true;
        // C s_21_3: cast zx s_21_2 -> bv
        let s_21_3: Bits = Bits::new(s_21_2 as u128, 1u16);
        // D s_21_4: cmp-eq s_21_1 s_21_3
        let s_21_4: bool = ((s_21_1) == (s_21_3));
        // D s_21_5: write-var gs#61068 <= s_21_4
        fn_state.gs_61068 = s_21_4;
        // N s_21_6: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #() : ()
        let s_22_0: () = ();
        // S s_22_1: call HaltingAllowed(s_22_0)
        let s_22_1: bool = HaltingAllowed(state, tracer, s_22_0);
        // D s_22_2: write-var gs#61067 <= s_22_1
        fn_state.gs_61067 = s_22_1;
        // N s_22_3: jump b14
        return block_14(state, tracer, fn_state);
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
        // N s_23_2: branch s_23_1 b57 b24
        if s_23_1 {
            return block_57(state, tracer, fn_state);
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
        // D s_24_1: write-var gs#61074 <= s_24_0
        fn_state.gs_61074 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#61074:u8
        let s_25_0: bool = fn_state.gs_61074;
        // N s_25_1: branch s_25_0 b56 b26
        if s_25_0 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #0u : u8
        let s_26_0: bool = false;
        // D s_26_1: write-var gs#61075 <= s_26_0
        fn_state.gs_61075 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#61075:u8
        let s_27_0: bool = fn_state.gs_61075;
        // N s_27_1: branch s_27_0 b55 b28
        if s_27_0 {
            return block_55(state, tracer, fn_state);
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
        // D s_28_1: write-var gs#61076 <= s_28_0
        fn_state.gs_61076 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var gs#61076:u8
        let s_29_0: bool = fn_state.gs_61076;
        // N s_29_1: branch s_29_0 b54 b30
        if s_29_0 {
            return block_54(state, tracer, fn_state);
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
        // D s_30_1: write-var gs#61077 <= s_30_0
        fn_state.gs_61077 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var gs#61077:u8
        let s_31_0: bool = fn_state.gs_61077;
        // N s_31_1: branch s_31_0 b53 b32
        if s_31_0 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_32(state, tracer, fn_state);
        };
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
        // N s_32_4: branch s_32_3 b52 b33
        if s_32_3 {
            return block_52(state, tracer, fn_state);
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
        // D s_33_1: write-var gs#61078 <= s_33_0
        fn_state.gs_61078 = s_33_0;
        // N s_33_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var gs#61078:u8
        let s_34_0: bool = fn_state.gs_61078;
        // N s_34_1: branch s_34_0 b46 b35
        if s_34_0 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #10128u : u32
        let s_35_0: u32 = 10128;
        // D s_35_1: read-reg s_35_0:struct
        let s_35_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_35_0 as isize);
            tracer.read_register(s_35_0 as isize, value);
            value
        };
        // D s_35_2: call _get_OSLSR_EL1_Type_OSLK(s_35_1)
        let s_35_2: bool = u_get_OSLSR_EL1_Type_OSLK(state, tracer, s_35_1);
        // D s_35_3: cast zx s_35_2 -> bv
        let s_35_3: Bits = Bits::new(s_35_2 as u128, 1u16);
        // C s_35_4: const #0u : u8
        let s_35_4: bool = false;
        // C s_35_5: cast zx s_35_4 -> bv
        let s_35_5: Bits = Bits::new(s_35_4 as u128, 1u16);
        // D s_35_6: cmp-eq s_35_3 s_35_5
        let s_35_6: bool = ((s_35_3) == (s_35_5));
        // N s_35_7: branch s_35_6 b45 b36
        if s_35_6 {
            return block_45(state, tracer, fn_state);
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
        // D s_36_1: write-var gs#61079 <= s_36_0
        fn_state.gs_61079 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#61079:u8
        let s_37_0: bool = fn_state.gs_61079;
        // N s_37_1: branch s_37_0 b44 b38
        if s_37_0 {
            return block_44(state, tracer, fn_state);
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
        // D s_38_1: write-var gs#61080 <= s_38_0
        fn_state.gs_61080 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#61080:u8
        let s_39_0: bool = fn_state.gs_61080;
        // N s_39_1: branch s_39_0 b43 b40
        if s_39_0 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_40(state, tracer, fn_state);
        };
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #185u : u32
        let s_40_0: u32 = 185;
        // S s_40_1: call IsFeatureImplemented(s_40_0)
        let s_40_1: bool = IsFeatureImplemented(state, tracer, s_40_0);
        // N s_40_2: branch s_40_1 b42 b41
        if s_40_1 {
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
        // C s_41_0: const #64s : i64
        let s_41_0: i64 = 64;
        // C s_41_1: const #10s : i
        let s_41_1: i128 = 10;
        // C s_41_2: const #19392u : u32
        let s_41_2: u32 = 19392;
        // D s_41_3: read-reg s_41_2:[struct; 64]
        let s_41_3: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_41_2 as isize);
            tracer.read_register(s_41_2 as isize, value);
            value
        };
        // D s_41_4: read-element s_41_3[s_41_1]
        let s_41_4: ProductType5c790c8ef59cc8b2 = s_41_3[(s_41_1) as usize];
        // D s_41_5: call __get_DBGWVR_EL1(s_41_4)
        let s_41_5: ProductType5c790c8ef59cc8b2 = u__get_DBGWVR_EL1(
            state,
            tracer,
            s_41_4,
        );
        // D s_41_6: write-var ga#64141 <= s_41_5
        fn_state.ga_64141 = s_41_5;
        // D s_41_7: read-var ga#64141.0:struct
        let s_41_7: u64 = fn_state.ga_64141._0;
        // D s_41_8: cast zx s_41_7 -> bv
        let s_41_8: Bits = Bits::new(s_41_7 as u128, 64u16);
        // D s_41_9: read-var t:i
        let s_41_9: i128 = fn_state.t;
        // D s_41_10: call X_set(s_41_9, s_41_0, s_41_8)
        let s_41_10: () = X_set(state, tracer, s_41_9, s_41_0, s_41_8);
        // N s_41_11: return
        return;
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #64s : i64
        let s_42_0: i64 = 64;
        // C s_42_1: const #() : ()
        let s_42_1: () = ();
        // S s_42_2: call EffectiveMDSELR_EL1_BANK(s_42_1)
        let s_42_2: u8 = EffectiveMDSELR_EL1_BANK(state, tracer, s_42_1);
        // S s_42_3: cast zx s_42_2 -> bv
        let s_42_3: Bits = Bits::new(s_42_2 as u128, 2u16);
        // S s_42_4: cast zx s_42_3 -> i
        let s_42_4: i128 = (s_42_3.value() as i128);
        // S s_42_5: cast reint s_42_4 -> i64
        let s_42_5: i64 = (s_42_4 as i64);
        // C s_42_6: const #16s : i
        let s_42_6: i128 = 16;
        // S s_42_7: cast zx s_42_5 -> i
        let s_42_7: i128 = (i128::try_from(s_42_5).unwrap());
        // S s_42_8: mul s_42_7 s_42_6
        let s_42_8: i128 = ((s_42_7) * (s_42_6));
        // S s_42_9: cast reint s_42_8 -> i64
        let s_42_9: i64 = (s_42_8 as i64);
        // C s_42_10: const #10s : i
        let s_42_10: i128 = 10;
        // S s_42_11: cast zx s_42_9 -> i
        let s_42_11: i128 = (i128::try_from(s_42_9).unwrap());
        // S s_42_12: add s_42_10 s_42_11
        let s_42_12: i128 = (s_42_10 + s_42_11);
        // S s_42_13: cast reint s_42_12 -> i64
        let s_42_13: i64 = (s_42_12 as i64);
        // C s_42_14: const #19392u : u32
        let s_42_14: u32 = 19392;
        // D s_42_15: read-reg s_42_14:[struct; 64]
        let s_42_15: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_42_14 as isize);
            tracer.read_register(s_42_14 as isize, value);
            value
        };
        // S s_42_16: cast zx s_42_13 -> i
        let s_42_16: i128 = (i128::try_from(s_42_13).unwrap());
        // D s_42_17: read-element s_42_15[s_42_16]
        let s_42_17: ProductType5c790c8ef59cc8b2 = s_42_15[(s_42_16) as usize];
        // D s_42_18: call __get_DBGWVR_EL1(s_42_17)
        let s_42_18: ProductType5c790c8ef59cc8b2 = u__get_DBGWVR_EL1(
            state,
            tracer,
            s_42_17,
        );
        // D s_42_19: write-var ga#64137 <= s_42_18
        fn_state.ga_64137 = s_42_18;
        // D s_42_20: read-var ga#64137.0:struct
        let s_42_20: u64 = fn_state.ga_64137._0;
        // D s_42_21: cast zx s_42_20 -> bv
        let s_42_21: Bits = Bits::new(s_42_20 as u128, 64u16);
        // D s_42_22: read-var t:i
        let s_42_22: i128 = fn_state.t;
        // D s_42_23: call X_set(s_42_22, s_42_0, s_42_21)
        let s_42_23: () = X_set(state, tracer, s_42_22, s_42_0, s_42_21);
        // N s_42_24: return
        return;
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #1160u : u32
        let s_43_0: u32 = 1160;
        // D s_43_1: read-reg s_43_0:u8
        let s_43_1: u8 = {
            let value = state.read_register::<u8>(s_43_0 as isize);
            tracer.read_register(s_43_0 as isize, value);
            value
        };
        // D s_43_2: call Halt(s_43_1)
        let s_43_2: () = Halt(state, tracer, s_43_1);
        // N s_43_3: return
        return;
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var __EDSCR_TDA:u8
        let s_44_0: bool = fn_state.u__EDSCR_TDA;
        // D s_44_1: cast zx s_44_0 -> bv
        let s_44_1: Bits = Bits::new(s_44_0 as u128, 1u16);
        // C s_44_2: const #1u : u8
        let s_44_2: bool = true;
        // C s_44_3: cast zx s_44_2 -> bv
        let s_44_3: Bits = Bits::new(s_44_2 as u128, 1u16);
        // D s_44_4: cmp-eq s_44_1 s_44_3
        let s_44_4: bool = ((s_44_1) == (s_44_3));
        // D s_44_5: write-var gs#61080 <= s_44_4
        fn_state.gs_61080 = s_44_4;
        // N s_44_6: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #() : ()
        let s_45_0: () = ();
        // S s_45_1: call HaltingAllowed(s_45_0)
        let s_45_1: bool = HaltingAllowed(state, tracer, s_45_0);
        // D s_45_2: write-var gs#61079 <= s_45_1
        fn_state.gs_61079 = s_45_1;
        // N s_45_3: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #() : ()
        let s_46_0: () = ();
        // S s_46_1: call Halted(s_46_0)
        let s_46_1: bool = Halted(state, tracer, s_46_0);
        // N s_46_2: branch s_46_1 b51 b47
        if s_46_1 {
            return block_51(state, tracer, fn_state);
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
        // D s_47_1: write-var gs#61086 <= s_47_0
        fn_state.gs_61086 = s_47_0;
        // N s_47_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var gs#61086:u8
        let s_48_0: bool = fn_state.gs_61086;
        // N s_48_1: branch s_48_0 b50 b49
        if s_48_0 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #24u : u8
        let s_49_0: u8 = 24;
        // C s_49_1: cast zx s_49_0 -> bv
        let s_49_1: Bits = Bits::new(s_49_0 as u128, 8u16);
        // C s_49_2: cast zx s_49_1 -> i
        let s_49_2: i128 = (s_49_1.value() as i128);
        // C s_49_3: cast reint s_49_2 -> i64
        let s_49_3: i64 = (s_49_2 as i64);
        // C s_49_4: cast zx s_49_3 -> i
        let s_49_4: i128 = (i128::try_from(s_49_3).unwrap());
        // C s_49_5: const #424u : u32
        let s_49_5: u32 = 424;
        // D s_49_6: read-reg s_49_5:u8
        let s_49_6: u8 = {
            let value = state.read_register::<u8>(s_49_5 as isize);
            tracer.read_register(s_49_5 as isize, value);
            value
        };
        // D s_49_7: call AArch64_SystemAccessTrap(s_49_6, s_49_4)
        let s_49_7: () = AArch64_SystemAccessTrap(state, tracer, s_49_6, s_49_4);
        // N s_49_8: return
        return;
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
        // D s_51_0: read-var __EDSCR_SDD:u8
        let s_51_0: bool = fn_state.u__EDSCR_SDD;
        // D s_51_1: cast zx s_51_0 -> bv
        let s_51_1: Bits = Bits::new(s_51_0 as u128, 1u16);
        // C s_51_2: const #1u : u8
        let s_51_2: bool = true;
        // C s_51_3: cast zx s_51_2 -> bv
        let s_51_3: Bits = Bits::new(s_51_2 as u128, 1u16);
        // D s_51_4: cmp-eq s_51_1 s_51_3
        let s_51_4: bool = ((s_51_1) == (s_51_3));
        // D s_51_5: write-var gs#61086 <= s_51_4
        fn_state.gs_61086 = s_51_4;
        // N s_51_6: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var __MDCR_EL3_TDA:u8
        let s_52_0: bool = fn_state.u__MDCR_EL3_TDA;
        // D s_52_1: cast zx s_52_0 -> bv
        let s_52_1: Bits = Bits::new(s_52_0 as u128, 1u16);
        // C s_52_2: const #1u : u8
        let s_52_2: bool = true;
        // C s_52_3: cast zx s_52_2 -> bv
        let s_52_3: Bits = Bits::new(s_52_2 as u128, 1u16);
        // D s_52_4: cmp-eq s_52_1 s_52_3
        let s_52_4: bool = ((s_52_1) == (s_52_3));
        // D s_52_5: write-var gs#61078 <= s_52_4
        fn_state.gs_61078 = s_52_4;
        // N s_52_6: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_53_0: panic
        panic!("{:?}", ());
        // N s_53_1: return
        return;
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var __MDCR_EL3_TDA:u8
        let s_54_0: bool = fn_state.u__MDCR_EL3_TDA;
        // D s_54_1: cast zx s_54_0 -> bv
        let s_54_1: Bits = Bits::new(s_54_0 as u128, 1u16);
        // C s_54_2: const #1u : u8
        let s_54_2: bool = true;
        // C s_54_3: cast zx s_54_2 -> bv
        let s_54_3: Bits = Bits::new(s_54_2 as u128, 1u16);
        // D s_54_4: cmp-eq s_54_1 s_54_3
        let s_54_4: bool = ((s_54_1) == (s_54_3));
        // D s_54_5: write-var gs#61077 <= s_54_4
        fn_state.gs_61077 = s_54_4;
        // N s_54_6: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_55_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_55_1: call __IMPDEF_boolean(s_55_0)
        let s_55_1: bool = u__IMPDEF_boolean(state, tracer, s_55_0);
        // D s_55_2: write-var gs#61076 <= s_55_1
        fn_state.gs_61076 = s_55_1;
        // N s_55_3: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var __EDSCR_SDD:u8
        let s_56_0: bool = fn_state.u__EDSCR_SDD;
        // D s_56_1: cast zx s_56_0 -> bv
        let s_56_1: Bits = Bits::new(s_56_0 as u128, 1u16);
        // C s_56_2: const #1u : u8
        let s_56_2: bool = true;
        // C s_56_3: cast zx s_56_2 -> bv
        let s_56_3: Bits = Bits::new(s_56_2 as u128, 1u16);
        // D s_56_4: cmp-eq s_56_1 s_56_3
        let s_56_4: bool = ((s_56_1) == (s_56_3));
        // D s_56_5: write-var gs#61075 <= s_56_4
        fn_state.gs_61075 = s_56_4;
        // N s_56_6: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_57_0: const #424u : u32
        let s_57_0: u32 = 424;
        // D s_57_1: read-reg s_57_0:u8
        let s_57_1: u8 = {
            let value = state.read_register::<u8>(s_57_0 as isize);
            tracer.read_register(s_57_0 as isize, value);
            value
        };
        // C s_57_2: const #2u : u8
        let s_57_2: u8 = 2;
        // D s_57_3: cmp-lt s_57_1 s_57_2
        let s_57_3: bool = ((s_57_1) < (s_57_2));
        // D s_57_4: write-var gs#61074 <= s_57_3
        fn_state.gs_61074 = s_57_3;
        // N s_57_5: jump b25
        return block_25(state, tracer, fn_state);
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
        // N s_58_2: branch s_58_1 b111 b59
        if s_58_1 {
            return block_111(state, tracer, fn_state);
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
        // D s_59_1: write-var gs#61087 <= s_59_0
        fn_state.gs_61087 = s_59_0;
        // N s_59_2: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var gs#61087:u8
        let s_60_0: bool = fn_state.gs_61087;
        // N s_60_1: branch s_60_0 b110 b61
        if s_60_0 {
            return block_110(state, tracer, fn_state);
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
        // D s_61_1: write-var gs#61088 <= s_61_0
        fn_state.gs_61088 = s_61_0;
        // N s_61_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var gs#61088:u8
        let s_62_0: bool = fn_state.gs_61088;
        // N s_62_1: branch s_62_0 b109 b63
        if s_62_0 {
            return block_109(state, tracer, fn_state);
        } else {
            return block_63(state, tracer, fn_state);
        };
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #0u : u8
        let s_63_0: bool = false;
        // D s_63_1: write-var gs#61089 <= s_63_0
        fn_state.gs_61089 = s_63_0;
        // N s_63_2: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var gs#61089:u8
        let s_64_0: bool = fn_state.gs_61089;
        // N s_64_1: branch s_64_0 b108 b65
        if s_64_0 {
            return block_108(state, tracer, fn_state);
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
        // D s_65_1: write-var gs#61090 <= s_65_0
        fn_state.gs_61090 = s_65_0;
        // N s_65_2: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_66_0: read-var gs#61090:u8
        let s_66_0: bool = fn_state.gs_61090;
        // N s_66_1: branch s_66_0 b107 b67
        if s_66_0 {
            return block_107(state, tracer, fn_state);
        } else {
            return block_67(state, tracer, fn_state);
        };
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #() : ()
        let s_67_0: () = ();
        // S s_67_1: call EL2Enabled(s_67_0)
        let s_67_1: bool = EL2Enabled(state, tracer, s_67_0);
        // N s_67_2: branch s_67_1 b106 b68
        if s_67_1 {
            return block_106(state, tracer, fn_state);
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
        // D s_68_1: write-var gs#61091 <= s_68_0
        fn_state.gs_61091 = s_68_0;
        // N s_68_2: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_69_0: read-var gs#61091:u8
        let s_69_0: bool = fn_state.gs_61091;
        // N s_69_1: branch s_69_0 b102 b70
        if s_69_0 {
            return block_102(state, tracer, fn_state);
        } else {
            return block_70(state, tracer, fn_state);
        };
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #0u : u8
        let s_70_0: bool = false;
        // D s_70_1: write-var gs#61093 <= s_70_0
        fn_state.gs_61093 = s_70_0;
        // N s_70_2: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_71_0: read-var gs#61093:u8
        let s_71_0: bool = fn_state.gs_61093;
        // N s_71_1: branch s_71_0 b101 b72
        if s_71_0 {
            return block_101(state, tracer, fn_state);
        } else {
            return block_72(state, tracer, fn_state);
        };
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #0u : u8
        let s_72_0: bool = false;
        // D s_72_1: write-var gs#61094 <= s_72_0
        fn_state.gs_61094 = s_72_0;
        // N s_72_2: jump b73
        return block_73(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_73_0: read-var gs#61094:u8
        let s_73_0: bool = fn_state.gs_61094;
        // N s_73_1: branch s_73_0 b100 b74
        if s_73_0 {
            return block_100(state, tracer, fn_state);
        } else {
            return block_74(state, tracer, fn_state);
        };
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_74_0: const #() : ()
        let s_74_0: () = ();
        // S s_74_1: call EL2Enabled(s_74_0)
        let s_74_1: bool = EL2Enabled(state, tracer, s_74_0);
        // N s_74_2: branch s_74_1 b99 b75
        if s_74_1 {
            return block_99(state, tracer, fn_state);
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
        // D s_75_1: write-var gs#61095 <= s_75_0
        fn_state.gs_61095 = s_75_0;
        // N s_75_2: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_76_0: read-var gs#61095:u8
        let s_76_0: bool = fn_state.gs_61095;
        // N s_76_1: branch s_76_0 b98 b77
        if s_76_0 {
            return block_98(state, tracer, fn_state);
        } else {
            return block_77(state, tracer, fn_state);
        };
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
        // N s_77_4: branch s_77_3 b97 b78
        if s_77_3 {
            return block_97(state, tracer, fn_state);
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
        // D s_78_1: write-var gs#61096 <= s_78_0
        fn_state.gs_61096 = s_78_0;
        // N s_78_2: jump b79
        return block_79(state, tracer, fn_state);
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_79_0: read-var gs#61096:u8
        let s_79_0: bool = fn_state.gs_61096;
        // N s_79_1: branch s_79_0 b91 b80
        if s_79_0 {
            return block_91(state, tracer, fn_state);
        } else {
            return block_80(state, tracer, fn_state);
        };
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_80_0: const #10128u : u32
        let s_80_0: u32 = 10128;
        // D s_80_1: read-reg s_80_0:struct
        let s_80_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_80_0 as isize);
            tracer.read_register(s_80_0 as isize, value);
            value
        };
        // D s_80_2: call _get_OSLSR_EL1_Type_OSLK(s_80_1)
        let s_80_2: bool = u_get_OSLSR_EL1_Type_OSLK(state, tracer, s_80_1);
        // D s_80_3: cast zx s_80_2 -> bv
        let s_80_3: Bits = Bits::new(s_80_2 as u128, 1u16);
        // C s_80_4: const #0u : u8
        let s_80_4: bool = false;
        // C s_80_5: cast zx s_80_4 -> bv
        let s_80_5: Bits = Bits::new(s_80_4 as u128, 1u16);
        // D s_80_6: cmp-eq s_80_3 s_80_5
        let s_80_6: bool = ((s_80_3) == (s_80_5));
        // N s_80_7: branch s_80_6 b90 b81
        if s_80_6 {
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
        // D s_81_1: write-var gs#61097 <= s_81_0
        fn_state.gs_61097 = s_81_0;
        // N s_81_2: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_82<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_82_0: read-var gs#61097:u8
        let s_82_0: bool = fn_state.gs_61097;
        // N s_82_1: branch s_82_0 b89 b83
        if s_82_0 {
            return block_89(state, tracer, fn_state);
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
        // D s_83_1: write-var gs#61098 <= s_83_0
        fn_state.gs_61098 = s_83_0;
        // N s_83_2: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_84_0: read-var gs#61098:u8
        let s_84_0: bool = fn_state.gs_61098;
        // N s_84_1: branch s_84_0 b88 b85
        if s_84_0 {
            return block_88(state, tracer, fn_state);
        } else {
            return block_85(state, tracer, fn_state);
        };
    }
    fn block_85<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_85_0: const #185u : u32
        let s_85_0: u32 = 185;
        // S s_85_1: call IsFeatureImplemented(s_85_0)
        let s_85_1: bool = IsFeatureImplemented(state, tracer, s_85_0);
        // N s_85_2: branch s_85_1 b87 b86
        if s_85_1 {
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
        // C s_86_0: const #64s : i64
        let s_86_0: i64 = 64;
        // C s_86_1: const #10s : i
        let s_86_1: i128 = 10;
        // C s_86_2: const #19392u : u32
        let s_86_2: u32 = 19392;
        // D s_86_3: read-reg s_86_2:[struct; 64]
        let s_86_3: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_86_2 as isize);
            tracer.read_register(s_86_2 as isize, value);
            value
        };
        // D s_86_4: read-element s_86_3[s_86_1]
        let s_86_4: ProductType5c790c8ef59cc8b2 = s_86_3[(s_86_1) as usize];
        // D s_86_5: call __get_DBGWVR_EL1(s_86_4)
        let s_86_5: ProductType5c790c8ef59cc8b2 = u__get_DBGWVR_EL1(
            state,
            tracer,
            s_86_4,
        );
        // D s_86_6: write-var ga#64111 <= s_86_5
        fn_state.ga_64111 = s_86_5;
        // D s_86_7: read-var ga#64111.0:struct
        let s_86_7: u64 = fn_state.ga_64111._0;
        // D s_86_8: cast zx s_86_7 -> bv
        let s_86_8: Bits = Bits::new(s_86_7 as u128, 64u16);
        // D s_86_9: read-var t:i
        let s_86_9: i128 = fn_state.t;
        // D s_86_10: call X_set(s_86_9, s_86_0, s_86_8)
        let s_86_10: () = X_set(state, tracer, s_86_9, s_86_0, s_86_8);
        // N s_86_11: return
        return;
    }
    fn block_87<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_87_0: const #64s : i64
        let s_87_0: i64 = 64;
        // C s_87_1: const #() : ()
        let s_87_1: () = ();
        // S s_87_2: call EffectiveMDSELR_EL1_BANK(s_87_1)
        let s_87_2: u8 = EffectiveMDSELR_EL1_BANK(state, tracer, s_87_1);
        // S s_87_3: cast zx s_87_2 -> bv
        let s_87_3: Bits = Bits::new(s_87_2 as u128, 2u16);
        // S s_87_4: cast zx s_87_3 -> i
        let s_87_4: i128 = (s_87_3.value() as i128);
        // S s_87_5: cast reint s_87_4 -> i64
        let s_87_5: i64 = (s_87_4 as i64);
        // C s_87_6: const #16s : i
        let s_87_6: i128 = 16;
        // S s_87_7: cast zx s_87_5 -> i
        let s_87_7: i128 = (i128::try_from(s_87_5).unwrap());
        // S s_87_8: mul s_87_7 s_87_6
        let s_87_8: i128 = ((s_87_7) * (s_87_6));
        // S s_87_9: cast reint s_87_8 -> i64
        let s_87_9: i64 = (s_87_8 as i64);
        // C s_87_10: const #10s : i
        let s_87_10: i128 = 10;
        // S s_87_11: cast zx s_87_9 -> i
        let s_87_11: i128 = (i128::try_from(s_87_9).unwrap());
        // S s_87_12: add s_87_10 s_87_11
        let s_87_12: i128 = (s_87_10 + s_87_11);
        // S s_87_13: cast reint s_87_12 -> i64
        let s_87_13: i64 = (s_87_12 as i64);
        // C s_87_14: const #19392u : u32
        let s_87_14: u32 = 19392;
        // D s_87_15: read-reg s_87_14:[struct; 64]
        let s_87_15: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_87_14 as isize);
            tracer.read_register(s_87_14 as isize, value);
            value
        };
        // S s_87_16: cast zx s_87_13 -> i
        let s_87_16: i128 = (i128::try_from(s_87_13).unwrap());
        // D s_87_17: read-element s_87_15[s_87_16]
        let s_87_17: ProductType5c790c8ef59cc8b2 = s_87_15[(s_87_16) as usize];
        // D s_87_18: call __get_DBGWVR_EL1(s_87_17)
        let s_87_18: ProductType5c790c8ef59cc8b2 = u__get_DBGWVR_EL1(
            state,
            tracer,
            s_87_17,
        );
        // D s_87_19: write-var ga#64107 <= s_87_18
        fn_state.ga_64107 = s_87_18;
        // D s_87_20: read-var ga#64107.0:struct
        let s_87_20: u64 = fn_state.ga_64107._0;
        // D s_87_21: cast zx s_87_20 -> bv
        let s_87_21: Bits = Bits::new(s_87_20 as u128, 64u16);
        // D s_87_22: read-var t:i
        let s_87_22: i128 = fn_state.t;
        // D s_87_23: call X_set(s_87_22, s_87_0, s_87_21)
        let s_87_23: () = X_set(state, tracer, s_87_22, s_87_0, s_87_21);
        // N s_87_24: return
        return;
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_88_0: const #1160u : u32
        let s_88_0: u32 = 1160;
        // D s_88_1: read-reg s_88_0:u8
        let s_88_1: u8 = {
            let value = state.read_register::<u8>(s_88_0 as isize);
            tracer.read_register(s_88_0 as isize, value);
            value
        };
        // D s_88_2: call Halt(s_88_1)
        let s_88_2: () = Halt(state, tracer, s_88_1);
        // N s_88_3: return
        return;
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_89_0: read-var __EDSCR_TDA:u8
        let s_89_0: bool = fn_state.u__EDSCR_TDA;
        // D s_89_1: cast zx s_89_0 -> bv
        let s_89_1: Bits = Bits::new(s_89_0 as u128, 1u16);
        // C s_89_2: const #1u : u8
        let s_89_2: bool = true;
        // C s_89_3: cast zx s_89_2 -> bv
        let s_89_3: Bits = Bits::new(s_89_2 as u128, 1u16);
        // D s_89_4: cmp-eq s_89_1 s_89_3
        let s_89_4: bool = ((s_89_1) == (s_89_3));
        // D s_89_5: write-var gs#61098 <= s_89_4
        fn_state.gs_61098 = s_89_4;
        // N s_89_6: jump b84
        return block_84(state, tracer, fn_state);
    }
    fn block_90<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_90_0: const #() : ()
        let s_90_0: () = ();
        // S s_90_1: call HaltingAllowed(s_90_0)
        let s_90_1: bool = HaltingAllowed(state, tracer, s_90_0);
        // D s_90_2: write-var gs#61097 <= s_90_1
        fn_state.gs_61097 = s_90_1;
        // N s_90_3: jump b82
        return block_82(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_91_0: const #() : ()
        let s_91_0: () = ();
        // S s_91_1: call Halted(s_91_0)
        let s_91_1: bool = Halted(state, tracer, s_91_0);
        // N s_91_2: branch s_91_1 b96 b92
        if s_91_1 {
            return block_96(state, tracer, fn_state);
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
        // D s_92_1: write-var gs#61104 <= s_92_0
        fn_state.gs_61104 = s_92_0;
        // N s_92_2: jump b93
        return block_93(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_93_0: read-var gs#61104:u8
        let s_93_0: bool = fn_state.gs_61104;
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
        // C s_94_0: const #24u : u8
        let s_94_0: u8 = 24;
        // C s_94_1: cast zx s_94_0 -> bv
        let s_94_1: Bits = Bits::new(s_94_0 as u128, 8u16);
        // C s_94_2: cast zx s_94_1 -> i
        let s_94_2: i128 = (s_94_1.value() as i128);
        // C s_94_3: cast reint s_94_2 -> i64
        let s_94_3: i64 = (s_94_2 as i64);
        // C s_94_4: cast zx s_94_3 -> i
        let s_94_4: i128 = (i128::try_from(s_94_3).unwrap());
        // C s_94_5: const #424u : u32
        let s_94_5: u32 = 424;
        // D s_94_6: read-reg s_94_5:u8
        let s_94_6: u8 = {
            let value = state.read_register::<u8>(s_94_5 as isize);
            tracer.read_register(s_94_5 as isize, value);
            value
        };
        // D s_94_7: call AArch64_SystemAccessTrap(s_94_6, s_94_4)
        let s_94_7: () = AArch64_SystemAccessTrap(state, tracer, s_94_6, s_94_4);
        // N s_94_8: return
        return;
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_95_0: panic
        panic!("{:?}", ());
        // N s_95_1: return
        return;
    }
    fn block_96<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_96_0: read-var __EDSCR_SDD:u8
        let s_96_0: bool = fn_state.u__EDSCR_SDD;
        // D s_96_1: cast zx s_96_0 -> bv
        let s_96_1: Bits = Bits::new(s_96_0 as u128, 1u16);
        // C s_96_2: const #1u : u8
        let s_96_2: bool = true;
        // C s_96_3: cast zx s_96_2 -> bv
        let s_96_3: Bits = Bits::new(s_96_2 as u128, 1u16);
        // D s_96_4: cmp-eq s_96_1 s_96_3
        let s_96_4: bool = ((s_96_1) == (s_96_3));
        // D s_96_5: write-var gs#61104 <= s_96_4
        fn_state.gs_61104 = s_96_4;
        // N s_96_6: jump b93
        return block_93(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_97_0: read-var __MDCR_EL3_TDA:u8
        let s_97_0: bool = fn_state.u__MDCR_EL3_TDA;
        // D s_97_1: cast zx s_97_0 -> bv
        let s_97_1: Bits = Bits::new(s_97_0 as u128, 1u16);
        // C s_97_2: const #1u : u8
        let s_97_2: bool = true;
        // C s_97_3: cast zx s_97_2 -> bv
        let s_97_3: Bits = Bits::new(s_97_2 as u128, 1u16);
        // D s_97_4: cmp-eq s_97_1 s_97_3
        let s_97_4: bool = ((s_97_1) == (s_97_3));
        // D s_97_5: write-var gs#61096 <= s_97_4
        fn_state.gs_61096 = s_97_4;
        // N s_97_6: jump b79
        return block_79(state, tracer, fn_state);
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
        // C s_98_5: const #432u : u32
        let s_98_5: u32 = 432;
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
        // C s_99_0: const #104880u : u32
        let s_99_0: u32 = 104880;
        // D s_99_1: read-reg s_99_0:struct
        let s_99_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_99_0 as isize);
            tracer.read_register(s_99_0 as isize, value);
            value
        };
        // D s_99_2: call _get_MDCR_EL2_Type_TDE(s_99_1)
        let s_99_2: bool = u_get_MDCR_EL2_Type_TDE(state, tracer, s_99_1);
        // C s_99_3: const #104880u : u32
        let s_99_3: u32 = 104880;
        // D s_99_4: read-reg s_99_3:struct
        let s_99_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_99_3 as isize);
            tracer.read_register(s_99_3 as isize, value);
            value
        };
        // D s_99_5: call _get_MDCR_EL2_Type_TDA(s_99_4)
        let s_99_5: bool = u_get_MDCR_EL2_Type_TDA(state, tracer, s_99_4);
        // D s_99_6: cast zx s_99_2 -> bv
        let s_99_6: Bits = Bits::new(s_99_2 as u128, 1u16);
        // D s_99_7: cast zx s_99_5 -> bv
        let s_99_7: Bits = Bits::new(s_99_5 as u128, 1u16);
        // D s_99_8: cast reint s_99_6 -> u128
        let s_99_8: u128 = (s_99_6.value() as u128);
        // D s_99_9: size-of s_99_6
        let s_99_9: u16 = s_99_6.length();
        // D s_99_10: cast reint s_99_7 -> u128
        let s_99_10: u128 = (s_99_7.value() as u128);
        // D s_99_11: size-of s_99_7
        let s_99_11: u16 = s_99_7.length();
        // D s_99_12: lsl s_99_8 s_99_11
        let s_99_12: u128 = s_99_8 << s_99_11;
        // D s_99_13: or s_99_12 s_99_10
        let s_99_13: u128 = ((s_99_12) | (s_99_10));
        // D s_99_14: add s_99_9 s_99_11
        let s_99_14: u16 = (s_99_9 + s_99_11);
        // D s_99_15: create-bits s_99_13 s_99_14
        let s_99_15: Bits = Bits::new(s_99_13, s_99_14);
        // D s_99_16: cast reint s_99_15 -> u8
        let s_99_16: u8 = (s_99_15.value() as u8);
        // D s_99_17: cast zx s_99_16 -> bv
        let s_99_17: Bits = Bits::new(s_99_16 as u128, 2u16);
        // C s_99_18: const #0u : u8
        let s_99_18: u8 = 0;
        // C s_99_19: cast zx s_99_18 -> bv
        let s_99_19: Bits = Bits::new(s_99_18 as u128, 2u16);
        // D s_99_20: cmp-ne s_99_17 s_99_19
        let s_99_20: bool = ((s_99_17) != (s_99_19));
        // D s_99_21: write-var gs#61095 <= s_99_20
        fn_state.gs_61095 = s_99_20;
        // N s_99_22: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_100_0: const #24u : u8
        let s_100_0: u8 = 24;
        // C s_100_1: cast zx s_100_0 -> bv
        let s_100_1: Bits = Bits::new(s_100_0 as u128, 8u16);
        // C s_100_2: cast zx s_100_1 -> i
        let s_100_2: i128 = (s_100_1.value() as i128);
        // C s_100_3: cast reint s_100_2 -> i64
        let s_100_3: i64 = (s_100_2 as i64);
        // C s_100_4: cast zx s_100_3 -> i
        let s_100_4: i128 = (i128::try_from(s_100_3).unwrap());
        // C s_100_5: const #432u : u32
        let s_100_5: u32 = 432;
        // D s_100_6: read-reg s_100_5:u8
        let s_100_6: u8 = {
            let value = state.read_register::<u8>(s_100_5 as isize);
            tracer.read_register(s_100_5 as isize, value);
            value
        };
        // D s_100_7: call AArch64_SystemAccessTrap(s_100_6, s_100_4)
        let s_100_7: () = AArch64_SystemAccessTrap(state, tracer, s_100_6, s_100_4);
        // N s_100_8: return
        return;
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_101_0: read-var __HDFGRTR_EL2_DBGWVRn_EL1:u8
        let s_101_0: bool = fn_state.u__HDFGRTR_EL2_DBGWVRn_EL1;
        // D s_101_1: cast zx s_101_0 -> bv
        let s_101_1: Bits = Bits::new(s_101_0 as u128, 1u16);
        // C s_101_2: const #1u : u8
        let s_101_2: bool = true;
        // C s_101_3: cast zx s_101_2 -> bv
        let s_101_3: Bits = Bits::new(s_101_2 as u128, 1u16);
        // D s_101_4: cmp-eq s_101_1 s_101_3
        let s_101_4: bool = ((s_101_1) == (s_101_3));
        // D s_101_5: write-var gs#61094 <= s_101_4
        fn_state.gs_61094 = s_101_4;
        // N s_101_6: jump b73
        return block_73(state, tracer, fn_state);
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
        // D s_102_4: not s_102_3
        let s_102_4: bool = !s_102_3;
        // N s_102_5: branch s_102_4 b105 b103
        if s_102_4 {
            return block_105(state, tracer, fn_state);
        } else {
            return block_103(state, tracer, fn_state);
        };
    }
    fn block_103<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_103_0: read-var __SCR_EL3_FGTEn:u8
        let s_103_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_103_1: cast zx s_103_0 -> bv
        let s_103_1: Bits = Bits::new(s_103_0 as u128, 1u16);
        // C s_103_2: const #1u : u8
        let s_103_2: bool = true;
        // C s_103_3: cast zx s_103_2 -> bv
        let s_103_3: Bits = Bits::new(s_103_2 as u128, 1u16);
        // D s_103_4: cmp-eq s_103_1 s_103_3
        let s_103_4: bool = ((s_103_1) == (s_103_3));
        // D s_103_5: write-var gs#61092 <= s_103_4
        fn_state.gs_61092 = s_103_4;
        // N s_103_6: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_104_0: read-var gs#61092:u8
        let s_104_0: bool = fn_state.gs_61092;
        // D s_104_1: write-var gs#61093 <= s_104_0
        fn_state.gs_61093 = s_104_0;
        // N s_104_2: jump b71
        return block_71(state, tracer, fn_state);
    }
    fn block_105<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_105_0: const #1u : u8
        let s_105_0: bool = true;
        // D s_105_1: write-var gs#61092 <= s_105_0
        fn_state.gs_61092 = s_105_0;
        // N s_105_2: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_106<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_106_0: const #146u : u32
        let s_106_0: u32 = 146;
        // S s_106_1: call IsFeatureImplemented(s_106_0)
        let s_106_1: bool = IsFeatureImplemented(state, tracer, s_106_0);
        // D s_106_2: write-var gs#61091 <= s_106_1
        fn_state.gs_61091 = s_106_1;
        // N s_106_3: jump b69
        return block_69(state, tracer, fn_state);
    }
    fn block_107<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_107_0: panic
        panic!("{:?}", ());
        // N s_107_1: return
        return;
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_108_0: read-var __MDCR_EL3_TDA:u8
        let s_108_0: bool = fn_state.u__MDCR_EL3_TDA;
        // D s_108_1: cast zx s_108_0 -> bv
        let s_108_1: Bits = Bits::new(s_108_0 as u128, 1u16);
        // C s_108_2: const #1u : u8
        let s_108_2: bool = true;
        // C s_108_3: cast zx s_108_2 -> bv
        let s_108_3: Bits = Bits::new(s_108_2 as u128, 1u16);
        // D s_108_4: cmp-eq s_108_1 s_108_3
        let s_108_4: bool = ((s_108_1) == (s_108_3));
        // D s_108_5: write-var gs#61090 <= s_108_4
        fn_state.gs_61090 = s_108_4;
        // N s_108_6: jump b66
        return block_66(state, tracer, fn_state);
    }
    fn block_109<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_109_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_109_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_109_1: call __IMPDEF_boolean(s_109_0)
        let s_109_1: bool = u__IMPDEF_boolean(state, tracer, s_109_0);
        // D s_109_2: write-var gs#61089 <= s_109_1
        fn_state.gs_61089 = s_109_1;
        // N s_109_3: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_110<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_110_0: read-var __EDSCR_SDD:u8
        let s_110_0: bool = fn_state.u__EDSCR_SDD;
        // D s_110_1: cast zx s_110_0 -> bv
        let s_110_1: Bits = Bits::new(s_110_0 as u128, 1u16);
        // C s_110_2: const #1u : u8
        let s_110_2: bool = true;
        // C s_110_3: cast zx s_110_2 -> bv
        let s_110_3: Bits = Bits::new(s_110_2 as u128, 1u16);
        // D s_110_4: cmp-eq s_110_1 s_110_3
        let s_110_4: bool = ((s_110_1) == (s_110_3));
        // D s_110_5: write-var gs#61088 <= s_110_4
        fn_state.gs_61088 = s_110_4;
        // N s_110_6: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_111<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_111_0: const #424u : u32
        let s_111_0: u32 = 424;
        // D s_111_1: read-reg s_111_0:u8
        let s_111_1: u8 = {
            let value = state.read_register::<u8>(s_111_0 as isize);
            tracer.read_register(s_111_0 as isize, value);
            value
        };
        // C s_111_2: const #2u : u8
        let s_111_2: u8 = 2;
        // D s_111_3: cmp-lt s_111_1 s_111_2
        let s_111_3: bool = ((s_111_1) < (s_111_2));
        // D s_111_4: write-var gs#61087 <= s_111_3
        fn_state.gs_61087 = s_111_3;
        // N s_111_5: jump b60
        return block_60(state, tracer, fn_state);
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_112_0: panic
        panic!("{:?}", ());
        // N s_112_1: return
        return;
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_113_0: panic
        panic!("{:?}", ());
        // N s_113_1: return
        return;
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_114_0: const #18416u : u32
        let s_114_0: u32 = 18416;
        // D s_114_1: read-reg s_114_0:struct
        let s_114_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_114_0 as isize);
            tracer.read_register(s_114_0 as isize, value);
            value
        };
        // D s_114_2: call _get_MDSELR_EL1_Type_BANK(s_114_1)
        let s_114_2: u8 = u_get_MDSELR_EL1_Type_BANK(state, tracer, s_114_1);
        // D s_114_3: cast zx s_114_2 -> bv
        let s_114_3: Bits = Bits::new(s_114_2 as u128, 2u16);
        // D s_114_4: cast zx s_114_3 -> i
        let s_114_4: i128 = (s_114_3.value() as i128);
        // D s_114_5: cast reint s_114_4 -> i64
        let s_114_5: i64 = (s_114_4 as i64);
        // C s_114_6: const #16s : i
        let s_114_6: i128 = 16;
        // D s_114_7: cast zx s_114_5 -> i
        let s_114_7: i128 = (i128::try_from(s_114_5).unwrap());
        // D s_114_8: mul s_114_7 s_114_6
        let s_114_8: i128 = ((s_114_7) * (s_114_6));
        // D s_114_9: cast reint s_114_8 -> i64
        let s_114_9: i64 = (s_114_8 as i64);
        // C s_114_10: const #10s : i
        let s_114_10: i128 = 10;
        // D s_114_11: cast zx s_114_9 -> i
        let s_114_11: i128 = (i128::try_from(s_114_9).unwrap());
        // D s_114_12: add s_114_10 s_114_11
        let s_114_12: i128 = (s_114_10 + s_114_11);
        // D s_114_13: cast reint s_114_12 -> i64
        let s_114_13: i64 = (s_114_12 as i64);
        // D s_114_14: cast zx s_114_13 -> i
        let s_114_14: i128 = (i128::try_from(s_114_13).unwrap());
        // C s_114_15: const #19360u : u32
        let s_114_15: u32 = 19360;
        // D s_114_16: read-reg s_114_15:i
        let s_114_16: i128 = {
            let value = state.read_register::<i128>(s_114_15 as isize);
            tracer.read_register(s_114_15 as isize, value);
            value
        };
        // D s_114_17: cmp-ge s_114_14 s_114_16
        let s_114_17: bool = ((s_114_14) >= (s_114_16));
        // D s_114_18: write-var gs#61065 <= s_114_17
        fn_state.gs_61065 = s_114_17;
        // N s_114_19: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_115_0: const #1u : u8
        let s_115_0: bool = true;
        // D s_115_1: write-var gs#61066 <= s_115_0
        fn_state.gs_61066 = s_115_0;
        // N s_115_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_116_0: const #10s : i
        let s_116_0: i128 = 10;
        // C s_116_1: const #19360u : u32
        let s_116_1: u32 = 19360;
        // D s_116_2: read-reg s_116_1:i
        let s_116_2: i128 = {
            let value = state.read_register::<i128>(s_116_1 as isize);
            tracer.read_register(s_116_1 as isize, value);
            value
        };
        // D s_116_3: cmp-ge s_116_0 s_116_2
        let s_116_3: bool = ((s_116_0) >= (s_116_2));
        // D s_116_4: write-var gs#61062 <= s_116_3
        fn_state.gs_61062 = s_116_3;
        // N s_116_5: jump b2
        return block_2(state, tracer, fn_state);
    }
}
