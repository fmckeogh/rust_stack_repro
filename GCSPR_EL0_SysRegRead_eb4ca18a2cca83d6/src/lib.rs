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
use u_get_HCR_EL2_Type_E2H::*;
use u__get_GCSPR_EL0::*;
use Halted::*;
use u_get_SCR_EL3_Type_GCSEn::*;
use IsFeatureImplemented::*;
use AArch64_SystemAccessTrap::*;
use u__IMPDEF_boolean::*;
use u_get_GCSCRE0_EL1_Type_nTR::*;
use X_set::*;
use u_get_EDSCR_Type_SDD::*;
use u_get_HCR_EL2_Type_TGE::*;
use EL2Enabled::*;
use EDSCR_read::*;
use u_get_HFGRTR_EL2_Type_nGCS_EL0::*;
use common::*;
pub fn GCSPR_EL0_SysRegRead_eb4ca18a2cca83d6<T: Tracer>(
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
        gs_61827: bool,
        gs_61855: bool,
        gs_61826: bool,
        gs_61850: bool,
        gs_61831: bool,
        u__HCR_EL2_TGE: bool,
        u__EDSCR_SDD: bool,
        gs_61852: bool,
        gs_61844: bool,
        gs_61847: bool,
        ga_65943: ProductType5c790c8ef59cc8b2,
        gs_61842: bool,
        u__SCR_EL3_FGTEn: bool,
        gs_61836: bool,
        gs_61825: bool,
        gs_61854: bool,
        gs_61828: bool,
        u__SCR_EL3_GCSEn: bool,
        gs_61834: bool,
        gs_61851: bool,
        gs_61835: bool,
        u__HFGRTR_EL2_nGCS_EL0: bool,
        gs_61848: bool,
        gs_61853: bool,
        ga_65982: ProductType5c790c8ef59cc8b2,
        gs_61830: bool,
        ga_65986: ProductType5c790c8ef59cc8b2,
        gs_61839: bool,
        gs_61833: bool,
        gs_61845: bool,
        gs_61843: bool,
        gs_61857: bool,
        u__PSTATE_EL: u8,
        gs_61838: bool,
        gs_61832: bool,
        gs_61841: bool,
        gs_61846: bool,
        u__GCSCRE0_EL1_nTR: bool,
        gs_61824: bool,
        ga_65966: ProductType5c790c8ef59cc8b2,
        gs_61849: bool,
        gs_61837: bool,
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
        // D s_0_9: call _get_SCR_EL3_Type_GCSEn(s_0_8)
        let s_0_9: bool = u_get_SCR_EL3_Type_GCSEn(state, tracer, s_0_8);
        // D s_0_10: write-var __SCR_EL3_GCSEn <= s_0_9
        fn_state.u__SCR_EL3_GCSEn = s_0_9;
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
        // C s_0_15: const #15608u : u32
        let s_0_15: u32 = 15608;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_GCSCRE0_EL1_Type_nTR(s_0_16)
        let s_0_17: bool = u_get_GCSCRE0_EL1_Type_nTR(state, tracer, s_0_16);
        // D s_0_18: write-var __GCSCRE0_EL1_nTR <= s_0_17
        fn_state.u__GCSCRE0_EL1_nTR = s_0_17;
        // C s_0_19: const #90704u : u32
        let s_0_19: u32 = 90704;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_SCR_EL3_Type_FGTEn(s_0_20)
        let s_0_21: bool = u_get_SCR_EL3_Type_FGTEn(state, tracer, s_0_20);
        // D s_0_22: write-var __SCR_EL3_FGTEn <= s_0_21
        fn_state.u__SCR_EL3_FGTEn = s_0_21;
        // C s_0_23: const #16592u : u32
        let s_0_23: u32 = 16592;
        // D s_0_24: read-reg s_0_23:struct
        let s_0_24: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_23 as isize);
            tracer.read_register(s_0_23 as isize, value);
            value
        };
        // D s_0_25: call _get_HFGRTR_EL2_Type_nGCS_EL0(s_0_24)
        let s_0_25: bool = u_get_HFGRTR_EL2_Type_nGCS_EL0(state, tracer, s_0_24);
        // D s_0_26: write-var __HFGRTR_EL2_nGCS_EL0 <= s_0_25
        fn_state.u__HFGRTR_EL2_nGCS_EL0 = s_0_25;
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
        // N s_0_33: branch s_0_32 b70 b1
        if s_0_32 {
            return block_70(state, tracer, fn_state);
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
        // C s_5_1: const #20840u : u32
        let s_5_1: u32 = 20840;
        // D s_5_2: read-reg s_5_1:struct
        let s_5_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_5_1 as isize);
            tracer.read_register(s_5_1 as isize, value);
            value
        };
        // D s_5_3: call __get_GCSPR_EL0(s_5_2)
        let s_5_3: ProductType5c790c8ef59cc8b2 = u__get_GCSPR_EL0(state, tracer, s_5_2);
        // D s_5_4: write-var ga#65986 <= s_5_3
        fn_state.ga_65986 = s_5_3;
        // D s_5_5: read-var ga#65986.0:struct
        let s_5_5: u64 = fn_state.ga_65986._0;
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
        // D s_7_1: write-var gs#61824 <= s_7_0
        fn_state.gs_61824 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#61824:u8
        let s_8_0: bool = fn_state.gs_61824;
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
        // D s_9_1: write-var gs#61825 <= s_9_0
        fn_state.gs_61825 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#61825:u8
        let s_10_0: bool = fn_state.gs_61825;
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
        // D s_11_1: write-var gs#61826 <= s_11_0
        fn_state.gs_61826 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#61826:u8
        let s_12_0: bool = fn_state.gs_61826;
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
        // D s_13_1: write-var gs#61827 <= s_13_0
        fn_state.gs_61827 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#61827:u8
        let s_14_0: bool = fn_state.gs_61827;
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
        // D s_16_1: write-var gs#61828 <= s_16_0
        fn_state.gs_61828 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#61828:u8
        let s_17_0: bool = fn_state.gs_61828;
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
        // C s_18_1: const #20840u : u32
        let s_18_1: u32 = 20840;
        // D s_18_2: read-reg s_18_1:struct
        let s_18_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_18_1 as isize);
            tracer.read_register(s_18_1 as isize, value);
            value
        };
        // D s_18_3: call __get_GCSPR_EL0(s_18_2)
        let s_18_3: ProductType5c790c8ef59cc8b2 = u__get_GCSPR_EL0(
            state,
            tracer,
            s_18_2,
        );
        // D s_18_4: write-var ga#65982 <= s_18_3
        fn_state.ga_65982 = s_18_3;
        // D s_18_5: read-var ga#65982.0:struct
        let s_18_5: u64 = fn_state.ga_65982._0;
        // D s_18_6: cast zx s_18_5 -> bv
        let s_18_6: Bits = Bits::new(s_18_5 as u128, 64u16);
        // D s_18_7: read-var t:i
        let s_18_7: i128 = fn_state.t;
        // D s_18_8: call X_set(s_18_7, s_18_0, s_18_6)
        let s_18_8: () = X_set(state, tracer, s_18_7, s_18_0, s_18_6);
        // N s_18_9: return
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
        // D s_20_1: write-var gs#61830 <= s_20_0
        fn_state.gs_61830 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#61830:u8
        let s_21_0: bool = fn_state.gs_61830;
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
        // D s_24_5: write-var gs#61830 <= s_24_4
        fn_state.gs_61830 = s_24_4;
        // N s_24_6: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var __SCR_EL3_GCSEn:u8
        let s_25_0: bool = fn_state.u__SCR_EL3_GCSEn;
        // D s_25_1: cast zx s_25_0 -> bv
        let s_25_1: Bits = Bits::new(s_25_0 as u128, 1u16);
        // C s_25_2: const #0u : u8
        let s_25_2: bool = false;
        // C s_25_3: cast zx s_25_2 -> bv
        let s_25_3: Bits = Bits::new(s_25_2 as u128, 1u16);
        // D s_25_4: cmp-eq s_25_1 s_25_3
        let s_25_4: bool = ((s_25_1) == (s_25_3));
        // D s_25_5: write-var gs#61828 <= s_25_4
        fn_state.gs_61828 = s_25_4;
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
        // D s_27_0: read-var __SCR_EL3_GCSEn:u8
        let s_27_0: bool = fn_state.u__SCR_EL3_GCSEn;
        // D s_27_1: cast zx s_27_0 -> bv
        let s_27_1: Bits = Bits::new(s_27_0 as u128, 1u16);
        // C s_27_2: const #0u : u8
        let s_27_2: bool = false;
        // C s_27_3: cast zx s_27_2 -> bv
        let s_27_3: Bits = Bits::new(s_27_2 as u128, 1u16);
        // D s_27_4: cmp-eq s_27_1 s_27_3
        let s_27_4: bool = ((s_27_1) == (s_27_3));
        // D s_27_5: write-var gs#61827 <= s_27_4
        fn_state.gs_61827 = s_27_4;
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
        // D s_28_2: write-var gs#61826 <= s_28_1
        fn_state.gs_61826 = s_28_1;
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
        // D s_29_5: write-var gs#61825 <= s_29_4
        fn_state.gs_61825 = s_29_4;
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
        // D s_30_4: write-var gs#61824 <= s_30_3
        fn_state.gs_61824 = s_30_3;
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
        // N s_31_2: branch s_31_1 b69 b32
        if s_31_1 {
            return block_69(state, tracer, fn_state);
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
        // D s_32_1: write-var gs#61831 <= s_32_0
        fn_state.gs_61831 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#61831:u8
        let s_33_0: bool = fn_state.gs_61831;
        // N s_33_1: branch s_33_0 b68 b34
        if s_33_0 {
            return block_68(state, tracer, fn_state);
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
        // D s_34_1: write-var gs#61832 <= s_34_0
        fn_state.gs_61832 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#61832:u8
        let s_35_0: bool = fn_state.gs_61832;
        // N s_35_1: branch s_35_0 b67 b36
        if s_35_0 {
            return block_67(state, tracer, fn_state);
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
        // D s_36_1: write-var gs#61833 <= s_36_0
        fn_state.gs_61833 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#61833:u8
        let s_37_0: bool = fn_state.gs_61833;
        // N s_37_1: branch s_37_0 b66 b38
        if s_37_0 {
            return block_66(state, tracer, fn_state);
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
        // D s_38_1: write-var gs#61834 <= s_38_0
        fn_state.gs_61834 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#61834:u8
        let s_39_0: bool = fn_state.gs_61834;
        // N s_39_1: branch s_39_0 b65 b40
        if s_39_0 {
            return block_65(state, tracer, fn_state);
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
        // N s_40_2: branch s_40_1 b64 b41
        if s_40_1 {
            return block_64(state, tracer, fn_state);
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
        // D s_41_1: write-var gs#61835 <= s_41_0
        fn_state.gs_61835 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#61835:u8
        let s_42_0: bool = fn_state.gs_61835;
        // N s_42_1: branch s_42_0 b60 b43
        if s_42_0 {
            return block_60(state, tracer, fn_state);
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
        // D s_43_1: write-var gs#61837 <= s_43_0
        fn_state.gs_61837 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#61837:u8
        let s_44_0: bool = fn_state.gs_61837;
        // N s_44_1: branch s_44_0 b59 b45
        if s_44_0 {
            return block_59(state, tracer, fn_state);
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
        // D s_45_1: write-var gs#61838 <= s_45_0
        fn_state.gs_61838 = s_45_0;
        // N s_45_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var gs#61838:u8
        let s_46_0: bool = fn_state.gs_61838;
        // N s_46_1: branch s_46_0 b58 b47
        if s_46_0 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_47_0: const #424u : u32
        let s_47_0: u32 = 424;
        // D s_47_1: read-reg s_47_0:u8
        let s_47_1: u8 = {
            let value = state.read_register::<u8>(s_47_0 as isize);
            tracer.read_register(s_47_0 as isize, value);
            value
        };
        // C s_47_2: const #2u : u8
        let s_47_2: u8 = 2;
        // D s_47_3: cmp-lt s_47_1 s_47_2
        let s_47_3: bool = ((s_47_1) < (s_47_2));
        // N s_47_4: branch s_47_3 b57 b48
        if s_47_3 {
            return block_57(state, tracer, fn_state);
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
        // D s_48_1: write-var gs#61839 <= s_48_0
        fn_state.gs_61839 = s_48_0;
        // N s_48_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var gs#61839:u8
        let s_49_0: bool = fn_state.gs_61839;
        // N s_49_1: branch s_49_0 b51 b50
        if s_49_0 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_50_0: const #64s : i64
        let s_50_0: i64 = 64;
        // C s_50_1: const #20840u : u32
        let s_50_1: u32 = 20840;
        // D s_50_2: read-reg s_50_1:struct
        let s_50_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_50_1 as isize);
            tracer.read_register(s_50_1 as isize, value);
            value
        };
        // D s_50_3: call __get_GCSPR_EL0(s_50_2)
        let s_50_3: ProductType5c790c8ef59cc8b2 = u__get_GCSPR_EL0(
            state,
            tracer,
            s_50_2,
        );
        // D s_50_4: write-var ga#65966 <= s_50_3
        fn_state.ga_65966 = s_50_3;
        // D s_50_5: read-var ga#65966.0:struct
        let s_50_5: u64 = fn_state.ga_65966._0;
        // D s_50_6: cast zx s_50_5 -> bv
        let s_50_6: Bits = Bits::new(s_50_5 as u128, 64u16);
        // D s_50_7: read-var t:i
        let s_50_7: i128 = fn_state.t;
        // D s_50_8: call X_set(s_50_7, s_50_0, s_50_6)
        let s_50_8: () = X_set(state, tracer, s_50_7, s_50_0, s_50_6);
        // N s_50_9: return
        return;
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #() : ()
        let s_51_0: () = ();
        // S s_51_1: call Halted(s_51_0)
        let s_51_1: bool = Halted(state, tracer, s_51_0);
        // N s_51_2: branch s_51_1 b56 b52
        if s_51_1 {
            return block_56(state, tracer, fn_state);
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
        // D s_52_1: write-var gs#61841 <= s_52_0
        fn_state.gs_61841 = s_52_0;
        // N s_52_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_53_0: read-var gs#61841:u8
        let s_53_0: bool = fn_state.gs_61841;
        // N s_53_1: branch s_53_0 b55 b54
        if s_53_0 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_54(state, tracer, fn_state);
        };
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_54_0: const #24u : u8
        let s_54_0: u8 = 24;
        // C s_54_1: cast zx s_54_0 -> bv
        let s_54_1: Bits = Bits::new(s_54_0 as u128, 8u16);
        // C s_54_2: cast zx s_54_1 -> i
        let s_54_2: i128 = (s_54_1.value() as i128);
        // C s_54_3: cast reint s_54_2 -> i64
        let s_54_3: i64 = (s_54_2 as i64);
        // C s_54_4: cast zx s_54_3 -> i
        let s_54_4: i128 = (i128::try_from(s_54_3).unwrap());
        // C s_54_5: const #424u : u32
        let s_54_5: u32 = 424;
        // D s_54_6: read-reg s_54_5:u8
        let s_54_6: u8 = {
            let value = state.read_register::<u8>(s_54_5 as isize);
            tracer.read_register(s_54_5 as isize, value);
            value
        };
        // D s_54_7: call AArch64_SystemAccessTrap(s_54_6, s_54_4)
        let s_54_7: () = AArch64_SystemAccessTrap(state, tracer, s_54_6, s_54_4);
        // N s_54_8: return
        return;
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_55_0: panic
        panic!("{:?}", ());
        // N s_55_1: return
        return;
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
        // D s_56_5: write-var gs#61841 <= s_56_4
        fn_state.gs_61841 = s_56_4;
        // N s_56_6: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var __SCR_EL3_GCSEn:u8
        let s_57_0: bool = fn_state.u__SCR_EL3_GCSEn;
        // D s_57_1: cast zx s_57_0 -> bv
        let s_57_1: Bits = Bits::new(s_57_0 as u128, 1u16);
        // C s_57_2: const #0u : u8
        let s_57_2: bool = false;
        // C s_57_3: cast zx s_57_2 -> bv
        let s_57_3: Bits = Bits::new(s_57_2 as u128, 1u16);
        // D s_57_4: cmp-eq s_57_1 s_57_3
        let s_57_4: bool = ((s_57_1) == (s_57_3));
        // D s_57_5: write-var gs#61839 <= s_57_4
        fn_state.gs_61839 = s_57_4;
        // N s_57_6: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #24u : u8
        let s_58_0: u8 = 24;
        // C s_58_1: cast zx s_58_0 -> bv
        let s_58_1: Bits = Bits::new(s_58_0 as u128, 8u16);
        // C s_58_2: cast zx s_58_1 -> i
        let s_58_2: i128 = (s_58_1.value() as i128);
        // C s_58_3: cast reint s_58_2 -> i64
        let s_58_3: i64 = (s_58_2 as i64);
        // C s_58_4: cast zx s_58_3 -> i
        let s_58_4: i128 = (i128::try_from(s_58_3).unwrap());
        // C s_58_5: const #432u : u32
        let s_58_5: u32 = 432;
        // D s_58_6: read-reg s_58_5:u8
        let s_58_6: u8 = {
            let value = state.read_register::<u8>(s_58_5 as isize);
            tracer.read_register(s_58_5 as isize, value);
            value
        };
        // D s_58_7: call AArch64_SystemAccessTrap(s_58_6, s_58_4)
        let s_58_7: () = AArch64_SystemAccessTrap(state, tracer, s_58_6, s_58_4);
        // N s_58_8: return
        return;
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_59_0: read-var __HFGRTR_EL2_nGCS_EL0:u8
        let s_59_0: bool = fn_state.u__HFGRTR_EL2_nGCS_EL0;
        // D s_59_1: cast zx s_59_0 -> bv
        let s_59_1: Bits = Bits::new(s_59_0 as u128, 1u16);
        // C s_59_2: const #0u : u8
        let s_59_2: bool = false;
        // C s_59_3: cast zx s_59_2 -> bv
        let s_59_3: Bits = Bits::new(s_59_2 as u128, 1u16);
        // D s_59_4: cmp-eq s_59_1 s_59_3
        let s_59_4: bool = ((s_59_1) == (s_59_3));
        // D s_59_5: write-var gs#61838 <= s_59_4
        fn_state.gs_61838 = s_59_4;
        // N s_59_6: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_60_0: const #424u : u32
        let s_60_0: u32 = 424;
        // D s_60_1: read-reg s_60_0:u8
        let s_60_1: u8 = {
            let value = state.read_register::<u8>(s_60_0 as isize);
            tracer.read_register(s_60_0 as isize, value);
            value
        };
        // C s_60_2: const #2u : u8
        let s_60_2: u8 = 2;
        // D s_60_3: cmp-lt s_60_1 s_60_2
        let s_60_3: bool = ((s_60_1) < (s_60_2));
        // D s_60_4: not s_60_3
        let s_60_4: bool = !s_60_3;
        // N s_60_5: branch s_60_4 b63 b61
        if s_60_4 {
            return block_63(state, tracer, fn_state);
        } else {
            return block_61(state, tracer, fn_state);
        };
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_61_0: read-var __SCR_EL3_FGTEn:u8
        let s_61_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_61_1: cast zx s_61_0 -> bv
        let s_61_1: Bits = Bits::new(s_61_0 as u128, 1u16);
        // C s_61_2: const #1u : u8
        let s_61_2: bool = true;
        // C s_61_3: cast zx s_61_2 -> bv
        let s_61_3: Bits = Bits::new(s_61_2 as u128, 1u16);
        // D s_61_4: cmp-eq s_61_1 s_61_3
        let s_61_4: bool = ((s_61_1) == (s_61_3));
        // D s_61_5: write-var gs#61836 <= s_61_4
        fn_state.gs_61836 = s_61_4;
        // N s_61_6: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_62_0: read-var gs#61836:u8
        let s_62_0: bool = fn_state.gs_61836;
        // D s_62_1: write-var gs#61837 <= s_62_0
        fn_state.gs_61837 = s_62_0;
        // N s_62_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_63_0: const #1u : u8
        let s_63_0: bool = true;
        // D s_63_1: write-var gs#61836 <= s_63_0
        fn_state.gs_61836 = s_63_0;
        // N s_63_2: jump b62
        return block_62(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_64_0: const #146u : u32
        let s_64_0: u32 = 146;
        // S s_64_1: call IsFeatureImplemented(s_64_0)
        let s_64_1: bool = IsFeatureImplemented(state, tracer, s_64_0);
        // D s_64_2: write-var gs#61835 <= s_64_1
        fn_state.gs_61835 = s_64_1;
        // N s_64_3: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_65_0: panic
        panic!("{:?}", ());
        // N s_65_1: return
        return;
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_66_0: read-var __SCR_EL3_GCSEn:u8
        let s_66_0: bool = fn_state.u__SCR_EL3_GCSEn;
        // D s_66_1: cast zx s_66_0 -> bv
        let s_66_1: Bits = Bits::new(s_66_0 as u128, 1u16);
        // C s_66_2: const #0u : u8
        let s_66_2: bool = false;
        // C s_66_3: cast zx s_66_2 -> bv
        let s_66_3: Bits = Bits::new(s_66_2 as u128, 1u16);
        // D s_66_4: cmp-eq s_66_1 s_66_3
        let s_66_4: bool = ((s_66_1) == (s_66_3));
        // D s_66_5: write-var gs#61834 <= s_66_4
        fn_state.gs_61834 = s_66_4;
        // N s_66_6: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_67<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_67_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_67_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_67_1: call __IMPDEF_boolean(s_67_0)
        let s_67_1: bool = u__IMPDEF_boolean(state, tracer, s_67_0);
        // D s_67_2: write-var gs#61833 <= s_67_1
        fn_state.gs_61833 = s_67_1;
        // N s_67_3: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_68<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_68_0: read-var __EDSCR_SDD:u8
        let s_68_0: bool = fn_state.u__EDSCR_SDD;
        // D s_68_1: cast zx s_68_0 -> bv
        let s_68_1: Bits = Bits::new(s_68_0 as u128, 1u16);
        // C s_68_2: const #1u : u8
        let s_68_2: bool = true;
        // C s_68_3: cast zx s_68_2 -> bv
        let s_68_3: Bits = Bits::new(s_68_2 as u128, 1u16);
        // D s_68_4: cmp-eq s_68_1 s_68_3
        let s_68_4: bool = ((s_68_1) == (s_68_3));
        // D s_68_5: write-var gs#61832 <= s_68_4
        fn_state.gs_61832 = s_68_4;
        // N s_68_6: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_69_0: const #424u : u32
        let s_69_0: u32 = 424;
        // D s_69_1: read-reg s_69_0:u8
        let s_69_1: u8 = {
            let value = state.read_register::<u8>(s_69_0 as isize);
            tracer.read_register(s_69_0 as isize, value);
            value
        };
        // C s_69_2: const #2u : u8
        let s_69_2: u8 = 2;
        // D s_69_3: cmp-lt s_69_1 s_69_2
        let s_69_3: bool = ((s_69_1) < (s_69_2));
        // D s_69_4: write-var gs#61831 <= s_69_3
        fn_state.gs_61831 = s_69_3;
        // N s_69_5: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_70_0: const #() : ()
        let s_70_0: () = ();
        // S s_70_1: call Halted(s_70_0)
        let s_70_1: bool = Halted(state, tracer, s_70_0);
        // N s_70_2: branch s_70_1 b127 b71
        if s_70_1 {
            return block_127(state, tracer, fn_state);
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
        // D s_71_1: write-var gs#61842 <= s_71_0
        fn_state.gs_61842 = s_71_0;
        // N s_71_2: jump b72
        return block_72(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_72_0: read-var gs#61842:u8
        let s_72_0: bool = fn_state.gs_61842;
        // N s_72_1: branch s_72_0 b126 b73
        if s_72_0 {
            return block_126(state, tracer, fn_state);
        } else {
            return block_73(state, tracer, fn_state);
        };
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_73_0: const #0u : u8
        let s_73_0: bool = false;
        // D s_73_1: write-var gs#61843 <= s_73_0
        fn_state.gs_61843 = s_73_0;
        // N s_73_2: jump b74
        return block_74(state, tracer, fn_state);
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_74_0: read-var gs#61843:u8
        let s_74_0: bool = fn_state.gs_61843;
        // N s_74_1: branch s_74_0 b125 b75
        if s_74_0 {
            return block_125(state, tracer, fn_state);
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
        // D s_75_1: write-var gs#61844 <= s_75_0
        fn_state.gs_61844 = s_75_0;
        // N s_75_2: jump b76
        return block_76(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_76_0: read-var gs#61844:u8
        let s_76_0: bool = fn_state.gs_61844;
        // N s_76_1: branch s_76_0 b124 b77
        if s_76_0 {
            return block_124(state, tracer, fn_state);
        } else {
            return block_77(state, tracer, fn_state);
        };
    }
    fn block_77<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_77_0: const #0u : u8
        let s_77_0: bool = false;
        // D s_77_1: write-var gs#61845 <= s_77_0
        fn_state.gs_61845 = s_77_0;
        // N s_77_2: jump b78
        return block_78(state, tracer, fn_state);
    }
    fn block_78<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_78_0: read-var gs#61845:u8
        let s_78_0: bool = fn_state.gs_61845;
        // N s_78_1: branch s_78_0 b123 b79
        if s_78_0 {
            return block_123(state, tracer, fn_state);
        } else {
            return block_79(state, tracer, fn_state);
        };
    }
    fn block_79<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_79_0: const #() : ()
        let s_79_0: () = ();
        // S s_79_1: call EL2Enabled(s_79_0)
        let s_79_1: bool = EL2Enabled(state, tracer, s_79_0);
        // S s_79_2: not s_79_1
        let s_79_2: bool = !s_79_1;
        // N s_79_3: branch s_79_2 b122 b80
        if s_79_2 {
            return block_122(state, tracer, fn_state);
        } else {
            return block_80(state, tracer, fn_state);
        };
    }
    fn block_80<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_80_0: read-var __HCR_EL2_TGE:u8
        let s_80_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_80_1: cast zx s_80_0 -> bv
        let s_80_1: Bits = Bits::new(s_80_0 as u128, 1u16);
        // C s_80_2: const #1u : u8
        let s_80_2: bool = true;
        // C s_80_3: cast zx s_80_2 -> bv
        let s_80_3: Bits = Bits::new(s_80_2 as u128, 1u16);
        // D s_80_4: cmp-ne s_80_1 s_80_3
        let s_80_4: bool = ((s_80_1) != (s_80_3));
        // D s_80_5: write-var gs#61846 <= s_80_4
        fn_state.gs_61846 = s_80_4;
        // N s_80_6: jump b81
        return block_81(state, tracer, fn_state);
    }
    fn block_81<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_81_0: read-var gs#61846:u8
        let s_81_0: bool = fn_state.gs_61846;
        // N s_81_1: branch s_81_0 b121 b82
        if s_81_0 {
            return block_121(state, tracer, fn_state);
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
        // D s_82_1: write-var gs#61847 <= s_82_0
        fn_state.gs_61847 = s_82_0;
        // N s_82_2: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_83<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_83_0: read-var gs#61847:u8
        let s_83_0: bool = fn_state.gs_61847;
        // N s_83_1: branch s_83_0 b120 b84
        if s_83_0 {
            return block_120(state, tracer, fn_state);
        } else {
            return block_84(state, tracer, fn_state);
        };
    }
    fn block_84<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_84_0: const #() : ()
        let s_84_0: () = ();
        // S s_84_1: call EL2Enabled(s_84_0)
        let s_84_1: bool = EL2Enabled(state, tracer, s_84_0);
        // N s_84_2: branch s_84_1 b119 b85
        if s_84_1 {
            return block_119(state, tracer, fn_state);
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
        // D s_85_1: write-var gs#61848 <= s_85_0
        fn_state.gs_61848 = s_85_0;
        // N s_85_2: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_86<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_86_0: read-var gs#61848:u8
        let s_86_0: bool = fn_state.gs_61848;
        // N s_86_1: branch s_86_0 b118 b87
        if s_86_0 {
            return block_118(state, tracer, fn_state);
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
        // D s_87_1: write-var gs#61849 <= s_87_0
        fn_state.gs_61849 = s_87_0;
        // N s_87_2: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_88<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_88_0: read-var gs#61849:u8
        let s_88_0: bool = fn_state.gs_61849;
        // N s_88_1: branch s_88_0 b117 b89
        if s_88_0 {
            return block_117(state, tracer, fn_state);
        } else {
            return block_89(state, tracer, fn_state);
        };
    }
    fn block_89<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_89_0: const #() : ()
        let s_89_0: () = ();
        // S s_89_1: call EL2Enabled(s_89_0)
        let s_89_1: bool = EL2Enabled(state, tracer, s_89_0);
        // N s_89_2: branch s_89_1 b116 b90
        if s_89_1 {
            return block_116(state, tracer, fn_state);
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
        // D s_90_1: write-var gs#61850 <= s_90_0
        fn_state.gs_61850 = s_90_0;
        // N s_90_2: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_91<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_91_0: read-var gs#61850:u8
        let s_91_0: bool = fn_state.gs_61850;
        // N s_91_1: branch s_91_0 b115 b92
        if s_91_0 {
            return block_115(state, tracer, fn_state);
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
        // D s_92_1: write-var gs#61851 <= s_92_0
        fn_state.gs_61851 = s_92_0;
        // N s_92_2: jump b93
        return block_93(state, tracer, fn_state);
    }
    fn block_93<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_93_0: read-var gs#61851:u8
        let s_93_0: bool = fn_state.gs_61851;
        // N s_93_1: branch s_93_0 b111 b94
        if s_93_0 {
            return block_111(state, tracer, fn_state);
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
        // D s_94_1: write-var gs#61853 <= s_94_0
        fn_state.gs_61853 = s_94_0;
        // N s_94_2: jump b95
        return block_95(state, tracer, fn_state);
    }
    fn block_95<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_95_0: read-var gs#61853:u8
        let s_95_0: bool = fn_state.gs_61853;
        // N s_95_1: branch s_95_0 b110 b96
        if s_95_0 {
            return block_110(state, tracer, fn_state);
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
        // D s_96_1: write-var gs#61854 <= s_96_0
        fn_state.gs_61854 = s_96_0;
        // N s_96_2: jump b97
        return block_97(state, tracer, fn_state);
    }
    fn block_97<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_97_0: read-var gs#61854:u8
        let s_97_0: bool = fn_state.gs_61854;
        // N s_97_1: branch s_97_0 b109 b98
        if s_97_0 {
            return block_109(state, tracer, fn_state);
        } else {
            return block_98(state, tracer, fn_state);
        };
    }
    fn block_98<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_98_0: const #424u : u32
        let s_98_0: u32 = 424;
        // D s_98_1: read-reg s_98_0:u8
        let s_98_1: u8 = {
            let value = state.read_register::<u8>(s_98_0 as isize);
            tracer.read_register(s_98_0 as isize, value);
            value
        };
        // C s_98_2: const #2u : u8
        let s_98_2: u8 = 2;
        // D s_98_3: cmp-lt s_98_1 s_98_2
        let s_98_3: bool = ((s_98_1) < (s_98_2));
        // N s_98_4: branch s_98_3 b108 b99
        if s_98_3 {
            return block_108(state, tracer, fn_state);
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
        // D s_99_1: write-var gs#61855 <= s_99_0
        fn_state.gs_61855 = s_99_0;
        // N s_99_2: jump b100
        return block_100(state, tracer, fn_state);
    }
    fn block_100<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_100_0: read-var gs#61855:u8
        let s_100_0: bool = fn_state.gs_61855;
        // N s_100_1: branch s_100_0 b102 b101
        if s_100_0 {
            return block_102(state, tracer, fn_state);
        } else {
            return block_101(state, tracer, fn_state);
        };
    }
    fn block_101<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_101_0: const #64s : i64
        let s_101_0: i64 = 64;
        // C s_101_1: const #20840u : u32
        let s_101_1: u32 = 20840;
        // D s_101_2: read-reg s_101_1:struct
        let s_101_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_101_1 as isize);
            tracer.read_register(s_101_1 as isize, value);
            value
        };
        // D s_101_3: call __get_GCSPR_EL0(s_101_2)
        let s_101_3: ProductType5c790c8ef59cc8b2 = u__get_GCSPR_EL0(
            state,
            tracer,
            s_101_2,
        );
        // D s_101_4: write-var ga#65943 <= s_101_3
        fn_state.ga_65943 = s_101_3;
        // D s_101_5: read-var ga#65943.0:struct
        let s_101_5: u64 = fn_state.ga_65943._0;
        // D s_101_6: cast zx s_101_5 -> bv
        let s_101_6: Bits = Bits::new(s_101_5 as u128, 64u16);
        // D s_101_7: read-var t:i
        let s_101_7: i128 = fn_state.t;
        // D s_101_8: call X_set(s_101_7, s_101_0, s_101_6)
        let s_101_8: () = X_set(state, tracer, s_101_7, s_101_0, s_101_6);
        // N s_101_9: return
        return;
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
        // D s_103_1: write-var gs#61857 <= s_103_0
        fn_state.gs_61857 = s_103_0;
        // N s_103_2: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_104<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_104_0: read-var gs#61857:u8
        let s_104_0: bool = fn_state.gs_61857;
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
        // D s_107_5: write-var gs#61857 <= s_107_4
        fn_state.gs_61857 = s_107_4;
        // N s_107_6: jump b104
        return block_104(state, tracer, fn_state);
    }
    fn block_108<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_108_0: read-var __SCR_EL3_GCSEn:u8
        let s_108_0: bool = fn_state.u__SCR_EL3_GCSEn;
        // D s_108_1: cast zx s_108_0 -> bv
        let s_108_1: Bits = Bits::new(s_108_0 as u128, 1u16);
        // C s_108_2: const #0u : u8
        let s_108_2: bool = false;
        // C s_108_3: cast zx s_108_2 -> bv
        let s_108_3: Bits = Bits::new(s_108_2 as u128, 1u16);
        // D s_108_4: cmp-eq s_108_1 s_108_3
        let s_108_4: bool = ((s_108_1) == (s_108_3));
        // D s_108_5: write-var gs#61855 <= s_108_4
        fn_state.gs_61855 = s_108_4;
        // N s_108_6: jump b100
        return block_100(state, tracer, fn_state);
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
        // D s_110_0: read-var __HFGRTR_EL2_nGCS_EL0:u8
        let s_110_0: bool = fn_state.u__HFGRTR_EL2_nGCS_EL0;
        // D s_110_1: cast zx s_110_0 -> bv
        let s_110_1: Bits = Bits::new(s_110_0 as u128, 1u16);
        // C s_110_2: const #0u : u8
        let s_110_2: bool = false;
        // C s_110_3: cast zx s_110_2 -> bv
        let s_110_3: Bits = Bits::new(s_110_2 as u128, 1u16);
        // D s_110_4: cmp-eq s_110_1 s_110_3
        let s_110_4: bool = ((s_110_1) == (s_110_3));
        // D s_110_5: write-var gs#61854 <= s_110_4
        fn_state.gs_61854 = s_110_4;
        // N s_110_6: jump b97
        return block_97(state, tracer, fn_state);
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
        // D s_111_4: not s_111_3
        let s_111_4: bool = !s_111_3;
        // N s_111_5: branch s_111_4 b114 b112
        if s_111_4 {
            return block_114(state, tracer, fn_state);
        } else {
            return block_112(state, tracer, fn_state);
        };
    }
    fn block_112<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_112_0: read-var __SCR_EL3_FGTEn:u8
        let s_112_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_112_1: cast zx s_112_0 -> bv
        let s_112_1: Bits = Bits::new(s_112_0 as u128, 1u16);
        // C s_112_2: const #1u : u8
        let s_112_2: bool = true;
        // C s_112_3: cast zx s_112_2 -> bv
        let s_112_3: Bits = Bits::new(s_112_2 as u128, 1u16);
        // D s_112_4: cmp-eq s_112_1 s_112_3
        let s_112_4: bool = ((s_112_1) == (s_112_3));
        // D s_112_5: write-var gs#61852 <= s_112_4
        fn_state.gs_61852 = s_112_4;
        // N s_112_6: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_113<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_113_0: read-var gs#61852:u8
        let s_113_0: bool = fn_state.gs_61852;
        // D s_113_1: write-var gs#61853 <= s_113_0
        fn_state.gs_61853 = s_113_0;
        // N s_113_2: jump b95
        return block_95(state, tracer, fn_state);
    }
    fn block_114<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_114_0: const #1u : u8
        let s_114_0: bool = true;
        // D s_114_1: write-var gs#61852 <= s_114_0
        fn_state.gs_61852 = s_114_0;
        // N s_114_2: jump b113
        return block_113(state, tracer, fn_state);
    }
    fn block_115<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_115_0: const #146u : u32
        let s_115_0: u32 = 146;
        // S s_115_1: call IsFeatureImplemented(s_115_0)
        let s_115_1: bool = IsFeatureImplemented(state, tracer, s_115_0);
        // D s_115_2: write-var gs#61851 <= s_115_1
        fn_state.gs_61851 = s_115_1;
        // N s_115_3: jump b93
        return block_93(state, tracer, fn_state);
    }
    fn block_116<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_116_0: const #102552u : u32
        let s_116_0: u32 = 102552;
        // D s_116_1: read-reg s_116_0:struct
        let s_116_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_116_0 as isize);
            tracer.read_register(s_116_0 as isize, value);
            value
        };
        // D s_116_2: call _get_HCR_EL2_Type_E2H(s_116_1)
        let s_116_2: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_116_1);
        // C s_116_3: const #102552u : u32
        let s_116_3: u32 = 102552;
        // D s_116_4: read-reg s_116_3:struct
        let s_116_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_116_3 as isize);
            tracer.read_register(s_116_3 as isize, value);
            value
        };
        // D s_116_5: call _get_HCR_EL2_Type_TGE(s_116_4)
        let s_116_5: bool = u_get_HCR_EL2_Type_TGE(state, tracer, s_116_4);
        // D s_116_6: cast zx s_116_2 -> bv
        let s_116_6: Bits = Bits::new(s_116_2 as u128, 1u16);
        // D s_116_7: cast zx s_116_5 -> bv
        let s_116_7: Bits = Bits::new(s_116_5 as u128, 1u16);
        // D s_116_8: cast reint s_116_6 -> u128
        let s_116_8: u128 = (s_116_6.value() as u128);
        // D s_116_9: size-of s_116_6
        let s_116_9: u16 = s_116_6.length();
        // D s_116_10: cast reint s_116_7 -> u128
        let s_116_10: u128 = (s_116_7.value() as u128);
        // D s_116_11: size-of s_116_7
        let s_116_11: u16 = s_116_7.length();
        // D s_116_12: lsl s_116_8 s_116_11
        let s_116_12: u128 = s_116_8 << s_116_11;
        // D s_116_13: or s_116_12 s_116_10
        let s_116_13: u128 = ((s_116_12) | (s_116_10));
        // D s_116_14: add s_116_9 s_116_11
        let s_116_14: u16 = (s_116_9 + s_116_11);
        // D s_116_15: create-bits s_116_13 s_116_14
        let s_116_15: Bits = Bits::new(s_116_13, s_116_14);
        // D s_116_16: cast reint s_116_15 -> u8
        let s_116_16: u8 = (s_116_15.value() as u8);
        // D s_116_17: cast zx s_116_16 -> bv
        let s_116_17: Bits = Bits::new(s_116_16 as u128, 2u16);
        // C s_116_18: const #3u : u8
        let s_116_18: u8 = 3;
        // C s_116_19: cast zx s_116_18 -> bv
        let s_116_19: Bits = Bits::new(s_116_18 as u128, 2u16);
        // D s_116_20: cmp-ne s_116_17 s_116_19
        let s_116_20: bool = ((s_116_17) != (s_116_19));
        // D s_116_21: write-var gs#61850 <= s_116_20
        fn_state.gs_61850 = s_116_20;
        // N s_116_22: jump b91
        return block_91(state, tracer, fn_state);
    }
    fn block_117<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_117_0: const #24u : u8
        let s_117_0: u8 = 24;
        // C s_117_1: cast zx s_117_0 -> bv
        let s_117_1: Bits = Bits::new(s_117_0 as u128, 8u16);
        // C s_117_2: cast zx s_117_1 -> i
        let s_117_2: i128 = (s_117_1.value() as i128);
        // C s_117_3: cast reint s_117_2 -> i64
        let s_117_3: i64 = (s_117_2 as i64);
        // C s_117_4: cast zx s_117_3 -> i
        let s_117_4: i128 = (i128::try_from(s_117_3).unwrap());
        // C s_117_5: const #432u : u32
        let s_117_5: u32 = 432;
        // D s_117_6: read-reg s_117_5:u8
        let s_117_6: u8 = {
            let value = state.read_register::<u8>(s_117_5 as isize);
            tracer.read_register(s_117_5 as isize, value);
            value
        };
        // D s_117_7: call AArch64_SystemAccessTrap(s_117_6, s_117_4)
        let s_117_7: () = AArch64_SystemAccessTrap(state, tracer, s_117_6, s_117_4);
        // N s_117_8: return
        return;
    }
    fn block_118<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_118_0: read-var __GCSCRE0_EL1_nTR:u8
        let s_118_0: bool = fn_state.u__GCSCRE0_EL1_nTR;
        // D s_118_1: cast zx s_118_0 -> bv
        let s_118_1: Bits = Bits::new(s_118_0 as u128, 1u16);
        // C s_118_2: const #0u : u8
        let s_118_2: bool = false;
        // C s_118_3: cast zx s_118_2 -> bv
        let s_118_3: Bits = Bits::new(s_118_2 as u128, 1u16);
        // D s_118_4: cmp-eq s_118_1 s_118_3
        let s_118_4: bool = ((s_118_1) == (s_118_3));
        // D s_118_5: write-var gs#61849 <= s_118_4
        fn_state.gs_61849 = s_118_4;
        // N s_118_6: jump b88
        return block_88(state, tracer, fn_state);
    }
    fn block_119<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_119_0: read-var __HCR_EL2_TGE:u8
        let s_119_0: bool = fn_state.u__HCR_EL2_TGE;
        // D s_119_1: cast zx s_119_0 -> bv
        let s_119_1: Bits = Bits::new(s_119_0 as u128, 1u16);
        // C s_119_2: const #1u : u8
        let s_119_2: bool = true;
        // C s_119_3: cast zx s_119_2 -> bv
        let s_119_3: Bits = Bits::new(s_119_2 as u128, 1u16);
        // D s_119_4: cmp-eq s_119_1 s_119_3
        let s_119_4: bool = ((s_119_1) == (s_119_3));
        // D s_119_5: write-var gs#61848 <= s_119_4
        fn_state.gs_61848 = s_119_4;
        // N s_119_6: jump b86
        return block_86(state, tracer, fn_state);
    }
    fn block_120<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_120_0: const #24u : u8
        let s_120_0: u8 = 24;
        // C s_120_1: cast zx s_120_0 -> bv
        let s_120_1: Bits = Bits::new(s_120_0 as u128, 8u16);
        // C s_120_2: cast zx s_120_1 -> i
        let s_120_2: i128 = (s_120_1.value() as i128);
        // C s_120_3: cast reint s_120_2 -> i64
        let s_120_3: i64 = (s_120_2 as i64);
        // C s_120_4: cast zx s_120_3 -> i
        let s_120_4: i128 = (i128::try_from(s_120_3).unwrap());
        // C s_120_5: const #440u : u32
        let s_120_5: u32 = 440;
        // D s_120_6: read-reg s_120_5:u8
        let s_120_6: u8 = {
            let value = state.read_register::<u8>(s_120_5 as isize);
            tracer.read_register(s_120_5 as isize, value);
            value
        };
        // D s_120_7: call AArch64_SystemAccessTrap(s_120_6, s_120_4)
        let s_120_7: () = AArch64_SystemAccessTrap(state, tracer, s_120_6, s_120_4);
        // N s_120_8: return
        return;
    }
    fn block_121<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_121_0: read-var __GCSCRE0_EL1_nTR:u8
        let s_121_0: bool = fn_state.u__GCSCRE0_EL1_nTR;
        // D s_121_1: cast zx s_121_0 -> bv
        let s_121_1: Bits = Bits::new(s_121_0 as u128, 1u16);
        // C s_121_2: const #0u : u8
        let s_121_2: bool = false;
        // C s_121_3: cast zx s_121_2 -> bv
        let s_121_3: Bits = Bits::new(s_121_2 as u128, 1u16);
        // D s_121_4: cmp-eq s_121_1 s_121_3
        let s_121_4: bool = ((s_121_1) == (s_121_3));
        // D s_121_5: write-var gs#61847 <= s_121_4
        fn_state.gs_61847 = s_121_4;
        // N s_121_6: jump b83
        return block_83(state, tracer, fn_state);
    }
    fn block_122<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_122_0: const #1u : u8
        let s_122_0: bool = true;
        // D s_122_1: write-var gs#61846 <= s_122_0
        fn_state.gs_61846 = s_122_0;
        // N s_122_2: jump b81
        return block_81(state, tracer, fn_state);
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
        // D s_124_0: read-var __SCR_EL3_GCSEn:u8
        let s_124_0: bool = fn_state.u__SCR_EL3_GCSEn;
        // D s_124_1: cast zx s_124_0 -> bv
        let s_124_1: Bits = Bits::new(s_124_0 as u128, 1u16);
        // C s_124_2: const #0u : u8
        let s_124_2: bool = false;
        // C s_124_3: cast zx s_124_2 -> bv
        let s_124_3: Bits = Bits::new(s_124_2 as u128, 1u16);
        // D s_124_4: cmp-eq s_124_1 s_124_3
        let s_124_4: bool = ((s_124_1) == (s_124_3));
        // D s_124_5: write-var gs#61845 <= s_124_4
        fn_state.gs_61845 = s_124_4;
        // N s_124_6: jump b78
        return block_78(state, tracer, fn_state);
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
        // D s_125_2: write-var gs#61844 <= s_125_1
        fn_state.gs_61844 = s_125_1;
        // N s_125_3: jump b76
        return block_76(state, tracer, fn_state);
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
        // D s_126_5: write-var gs#61843 <= s_126_4
        fn_state.gs_61843 = s_126_4;
        // N s_126_6: jump b74
        return block_74(state, tracer, fn_state);
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
        // D s_127_4: write-var gs#61842 <= s_127_3
        fn_state.gs_61842 = s_127_3;
        // N s_127_5: jump b72
        return block_72(state, tracer, fn_state);
    }
}
