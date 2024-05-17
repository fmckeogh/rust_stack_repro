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
use Halted::*;
use u_get_SCR_EL3_Type_GCSEn::*;
use NVMem_read::*;
use AArch64_SystemAccessTrap::*;
use u__IMPDEF_boolean::*;
use IsFeatureImplemented::*;
use u_get_HCR_EL2_Type_NV1::*;
use X_set::*;
use u__get_GCSPR_EL2::*;
use u_get_HCR_EL2_Type_NV::*;
use u__get_GCSPR_EL1::*;
use u_get_EDSCR_Type_SDD::*;
use u_get_HFGRTR_EL2_Type_nGCS_EL1::*;
use EL2Enabled::*;
use EDSCR_read::*;
use u_get_HCR_EL2_Type_NV2::*;
use common::*;
pub fn GCSPR_EL1_SysRegRead_efbe68095a628d64<T: Tracer>(
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
        gs_61873: bool,
        gs_61872: bool,
        ga_66067: ProductType5c790c8ef59cc8b2,
        u__HCR_EL2_E2H: bool,
        u__EDSCR_SDD: bool,
        u__HFGRTR_EL2_nGCS_EL1: bool,
        gs_61889: bool,
        gs_61883: bool,
        u__SCR_EL3_FGTEn: bool,
        gs_61884: bool,
        ga_66087: ProductType5c790c8ef59cc8b2,
        ga_66091: ProductType5c790c8ef59cc8b2,
        gs_61879: bool,
        u__SCR_EL3_GCSEn: bool,
        ga_66084: ProductType5c790c8ef59cc8b2,
        gs_61882: bool,
        gs_61880: bool,
        gs_61874: bool,
        gs_61885: bool,
        ga_66066: u64,
        gs_61892: bool,
        gs_61875: bool,
        gs_61886: bool,
        gs_61888: bool,
        gs_61881: bool,
        u__PSTATE_EL: u8,
        gs_61887: bool,
        gs_61876: bool,
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
        // C s_0_15: const #16592u : u32
        let s_0_15: u32 = 16592;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_HFGRTR_EL2_Type_nGCS_EL1(s_0_16)
        let s_0_17: bool = u_get_HFGRTR_EL2_Type_nGCS_EL1(state, tracer, s_0_16);
        // D s_0_18: write-var __HFGRTR_EL2_nGCS_EL1 <= s_0_17
        fn_state.u__HFGRTR_EL2_nGCS_EL1 = s_0_17;
        // C s_0_19: const #102552u : u32
        let s_0_19: u32 = 102552;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_HCR_EL2_Type_E2H(s_0_20)
        let s_0_21: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_0_20);
        // D s_0_22: write-var __HCR_EL2_E2H <= s_0_21
        fn_state.u__HCR_EL2_E2H = s_0_21;
        // D s_0_23: read-var __PSTATE_EL:u8
        let s_0_23: u8 = fn_state.u__PSTATE_EL;
        // D s_0_24: cast zx s_0_23 -> bv
        let s_0_24: Bits = Bits::new(s_0_23 as u128, 2u16);
        // C s_0_25: const #448u : u32
        let s_0_25: u32 = 448;
        // D s_0_26: read-reg s_0_25:u8
        let s_0_26: u8 = {
            let value = state.read_register::<u8>(s_0_25 as isize);
            tracer.read_register(s_0_25 as isize, value);
            value
        };
        // D s_0_27: cast zx s_0_26 -> bv
        let s_0_27: Bits = Bits::new(s_0_26 as u128, 2u16);
        // D s_0_28: cmp-eq s_0_24 s_0_27
        let s_0_28: bool = ((s_0_24) == (s_0_27));
        // N s_0_29: branch s_0_28 b78 b1
        if s_0_28 {
            return block_78(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b33 b2
        if s_1_5 {
            return block_33(state, tracer, fn_state);
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
        // C s_5_1: const #21912u : u32
        let s_5_1: u32 = 21912;
        // D s_5_2: read-reg s_5_1:struct
        let s_5_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_5_1 as isize);
            tracer.read_register(s_5_1 as isize, value);
            value
        };
        // D s_5_3: call __get_GCSPR_EL1(s_5_2)
        let s_5_3: ProductType5c790c8ef59cc8b2 = u__get_GCSPR_EL1(state, tracer, s_5_2);
        // D s_5_4: write-var ga#66091 <= s_5_3
        fn_state.ga_66091 = s_5_3;
        // D s_5_5: read-var ga#66091.0:struct
        let s_5_5: u64 = fn_state.ga_66091._0;
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
        // N s_6_2: branch s_6_1 b32 b7
        if s_6_1 {
            return block_32(state, tracer, fn_state);
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
        // D s_7_1: write-var gs#61872 <= s_7_0
        fn_state.gs_61872 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#61872:u8
        let s_8_0: bool = fn_state.gs_61872;
        // N s_8_1: branch s_8_0 b31 b9
        if s_8_0 {
            return block_31(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#61873 <= s_9_0
        fn_state.gs_61873 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#61873:u8
        let s_10_0: bool = fn_state.gs_61873;
        // N s_10_1: branch s_10_0 b30 b11
        if s_10_0 {
            return block_30(state, tracer, fn_state);
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
        // D s_11_1: write-var gs#61874 <= s_11_0
        fn_state.gs_61874 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#61874:u8
        let s_12_0: bool = fn_state.gs_61874;
        // N s_12_1: branch s_12_0 b29 b13
        if s_12_0 {
            return block_29(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#61875 <= s_13_0
        fn_state.gs_61875 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#61875:u8
        let s_14_0: bool = fn_state.gs_61875;
        // N s_14_1: branch s_14_0 b28 b15
        if s_14_0 {
            return block_28(state, tracer, fn_state);
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
        // N s_15_4: branch s_15_3 b27 b16
        if s_15_3 {
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
        // C s_16_0: const #0u : u8
        let s_16_0: bool = false;
        // D s_16_1: write-var gs#61876 <= s_16_0
        fn_state.gs_61876 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#61876:u8
        let s_17_0: bool = fn_state.gs_61876;
        // N s_17_1: branch s_17_0 b21 b18
        if s_17_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var __HCR_EL2_E2H:u8
        let s_18_0: bool = fn_state.u__HCR_EL2_E2H;
        // D s_18_1: cast zx s_18_0 -> bv
        let s_18_1: Bits = Bits::new(s_18_0 as u128, 1u16);
        // C s_18_2: const #1u : u8
        let s_18_2: bool = true;
        // C s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 1u16);
        // D s_18_4: cmp-eq s_18_1 s_18_3
        let s_18_4: bool = ((s_18_1) == (s_18_3));
        // N s_18_5: branch s_18_4 b20 b19
        if s_18_4 {
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
        // C s_19_1: const #21912u : u32
        let s_19_1: u32 = 21912;
        // D s_19_2: read-reg s_19_1:struct
        let s_19_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_19_1 as isize);
            tracer.read_register(s_19_1 as isize, value);
            value
        };
        // D s_19_3: call __get_GCSPR_EL1(s_19_2)
        let s_19_3: ProductType5c790c8ef59cc8b2 = u__get_GCSPR_EL1(
            state,
            tracer,
            s_19_2,
        );
        // D s_19_4: write-var ga#66087 <= s_19_3
        fn_state.ga_66087 = s_19_3;
        // D s_19_5: read-var ga#66087.0:struct
        let s_19_5: u64 = fn_state.ga_66087._0;
        // D s_19_6: cast zx s_19_5 -> bv
        let s_19_6: Bits = Bits::new(s_19_5 as u128, 64u16);
        // D s_19_7: read-var t:i
        let s_19_7: i128 = fn_state.t;
        // D s_19_8: call X_set(s_19_7, s_19_0, s_19_6)
        let s_19_8: () = X_set(state, tracer, s_19_7, s_19_0, s_19_6);
        // N s_19_9: return
        return;
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #64s : i64
        let s_20_0: i64 = 64;
        // C s_20_1: const #10216u : u32
        let s_20_1: u32 = 10216;
        // D s_20_2: read-reg s_20_1:struct
        let s_20_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_20_1 as isize);
            tracer.read_register(s_20_1 as isize, value);
            value
        };
        // D s_20_3: call __get_GCSPR_EL2(s_20_2)
        let s_20_3: ProductType5c790c8ef59cc8b2 = u__get_GCSPR_EL2(
            state,
            tracer,
            s_20_2,
        );
        // D s_20_4: write-var ga#66084 <= s_20_3
        fn_state.ga_66084 = s_20_3;
        // D s_20_5: read-var ga#66084.0:struct
        let s_20_5: u64 = fn_state.ga_66084._0;
        // D s_20_6: cast zx s_20_5 -> bv
        let s_20_6: Bits = Bits::new(s_20_5 as u128, 64u16);
        // D s_20_7: read-var t:i
        let s_20_7: i128 = fn_state.t;
        // D s_20_8: call X_set(s_20_7, s_20_0, s_20_6)
        let s_20_8: () = X_set(state, tracer, s_20_7, s_20_0, s_20_6);
        // N s_20_9: return
        return;
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #() : ()
        let s_21_0: () = ();
        // S s_21_1: call Halted(s_21_0)
        let s_21_1: bool = Halted(state, tracer, s_21_0);
        // N s_21_2: branch s_21_1 b26 b22
        if s_21_1 {
            return block_26(state, tracer, fn_state);
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
        // D s_22_1: write-var gs#61879 <= s_22_0
        fn_state.gs_61879 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#61879:u8
        let s_23_0: bool = fn_state.gs_61879;
        // N s_23_1: branch s_23_0 b25 b24
        if s_23_0 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #24u : u8
        let s_24_0: u8 = 24;
        // C s_24_1: cast zx s_24_0 -> bv
        let s_24_1: Bits = Bits::new(s_24_0 as u128, 8u16);
        // C s_24_2: cast zx s_24_1 -> i
        let s_24_2: i128 = (s_24_1.value() as i128);
        // C s_24_3: cast reint s_24_2 -> i64
        let s_24_3: i64 = (s_24_2 as i64);
        // C s_24_4: cast zx s_24_3 -> i
        let s_24_4: i128 = (i128::try_from(s_24_3).unwrap());
        // C s_24_5: const #424u : u32
        let s_24_5: u32 = 424;
        // D s_24_6: read-reg s_24_5:u8
        let s_24_6: u8 = {
            let value = state.read_register::<u8>(s_24_5 as isize);
            tracer.read_register(s_24_5 as isize, value);
            value
        };
        // D s_24_7: call AArch64_SystemAccessTrap(s_24_6, s_24_4)
        let s_24_7: () = AArch64_SystemAccessTrap(state, tracer, s_24_6, s_24_4);
        // N s_24_8: return
        return;
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_25_0: panic
        panic!("{:?}", ());
        // N s_25_1: return
        return;
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var __EDSCR_SDD:u8
        let s_26_0: bool = fn_state.u__EDSCR_SDD;
        // D s_26_1: cast zx s_26_0 -> bv
        let s_26_1: Bits = Bits::new(s_26_0 as u128, 1u16);
        // C s_26_2: const #1u : u8
        let s_26_2: bool = true;
        // C s_26_3: cast zx s_26_2 -> bv
        let s_26_3: Bits = Bits::new(s_26_2 as u128, 1u16);
        // D s_26_4: cmp-eq s_26_1 s_26_3
        let s_26_4: bool = ((s_26_1) == (s_26_3));
        // D s_26_5: write-var gs#61879 <= s_26_4
        fn_state.gs_61879 = s_26_4;
        // N s_26_6: jump b23
        return block_23(state, tracer, fn_state);
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
        // D s_27_5: write-var gs#61876 <= s_27_4
        fn_state.gs_61876 = s_27_4;
        // N s_27_6: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_28_0: panic
        panic!("{:?}", ());
        // N s_28_1: return
        return;
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var __SCR_EL3_GCSEn:u8
        let s_29_0: bool = fn_state.u__SCR_EL3_GCSEn;
        // D s_29_1: cast zx s_29_0 -> bv
        let s_29_1: Bits = Bits::new(s_29_0 as u128, 1u16);
        // C s_29_2: const #0u : u8
        let s_29_2: bool = false;
        // C s_29_3: cast zx s_29_2 -> bv
        let s_29_3: Bits = Bits::new(s_29_2 as u128, 1u16);
        // D s_29_4: cmp-eq s_29_1 s_29_3
        let s_29_4: bool = ((s_29_1) == (s_29_3));
        // D s_29_5: write-var gs#61875 <= s_29_4
        fn_state.gs_61875 = s_29_4;
        // N s_29_6: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_30_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_30_1: call __IMPDEF_boolean(s_30_0)
        let s_30_1: bool = u__IMPDEF_boolean(state, tracer, s_30_0);
        // D s_30_2: write-var gs#61874 <= s_30_1
        fn_state.gs_61874 = s_30_1;
        // N s_30_3: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var __EDSCR_SDD:u8
        let s_31_0: bool = fn_state.u__EDSCR_SDD;
        // D s_31_1: cast zx s_31_0 -> bv
        let s_31_1: Bits = Bits::new(s_31_0 as u128, 1u16);
        // C s_31_2: const #1u : u8
        let s_31_2: bool = true;
        // C s_31_3: cast zx s_31_2 -> bv
        let s_31_3: Bits = Bits::new(s_31_2 as u128, 1u16);
        // D s_31_4: cmp-eq s_31_1 s_31_3
        let s_31_4: bool = ((s_31_1) == (s_31_3));
        // D s_31_5: write-var gs#61873 <= s_31_4
        fn_state.gs_61873 = s_31_4;
        // N s_31_6: jump b10
        return block_10(state, tracer, fn_state);
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
        // D s_32_4: write-var gs#61872 <= s_32_3
        fn_state.gs_61872 = s_32_3;
        // N s_32_5: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #() : ()
        let s_33_0: () = ();
        // S s_33_1: call Halted(s_33_0)
        let s_33_1: bool = Halted(state, tracer, s_33_0);
        // N s_33_2: branch s_33_1 b77 b34
        if s_33_1 {
            return block_77(state, tracer, fn_state);
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
        // D s_34_1: write-var gs#61880 <= s_34_0
        fn_state.gs_61880 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#61880:u8
        let s_35_0: bool = fn_state.gs_61880;
        // N s_35_1: branch s_35_0 b76 b36
        if s_35_0 {
            return block_76(state, tracer, fn_state);
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
        // D s_36_1: write-var gs#61881 <= s_36_0
        fn_state.gs_61881 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#61881:u8
        let s_37_0: bool = fn_state.gs_61881;
        // N s_37_1: branch s_37_0 b75 b38
        if s_37_0 {
            return block_75(state, tracer, fn_state);
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
        // D s_38_1: write-var gs#61882 <= s_38_0
        fn_state.gs_61882 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#61882:u8
        let s_39_0: bool = fn_state.gs_61882;
        // N s_39_1: branch s_39_0 b74 b40
        if s_39_0 {
            return block_74(state, tracer, fn_state);
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
        // D s_40_1: write-var gs#61883 <= s_40_0
        fn_state.gs_61883 = s_40_0;
        // N s_40_2: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var gs#61883:u8
        let s_41_0: bool = fn_state.gs_61883;
        // N s_41_1: branch s_41_0 b73 b42
        if s_41_0 {
            return block_73(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #() : ()
        let s_42_0: () = ();
        // S s_42_1: call EL2Enabled(s_42_0)
        let s_42_1: bool = EL2Enabled(state, tracer, s_42_0);
        // N s_42_2: branch s_42_1 b72 b43
        if s_42_1 {
            return block_72(state, tracer, fn_state);
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
        // D s_43_1: write-var gs#61884 <= s_43_0
        fn_state.gs_61884 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#61884:u8
        let s_44_0: bool = fn_state.gs_61884;
        // N s_44_1: branch s_44_0 b68 b45
        if s_44_0 {
            return block_68(state, tracer, fn_state);
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
        // D s_45_1: write-var gs#61886 <= s_45_0
        fn_state.gs_61886 = s_45_0;
        // N s_45_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var gs#61886:u8
        let s_46_0: bool = fn_state.gs_61886;
        // N s_46_1: branch s_46_0 b67 b47
        if s_46_0 {
            return block_67(state, tracer, fn_state);
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
        // D s_47_1: write-var gs#61887 <= s_47_0
        fn_state.gs_61887 = s_47_0;
        // N s_47_2: jump b48
        return block_48(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_48_0: read-var gs#61887:u8
        let s_48_0: bool = fn_state.gs_61887;
        // N s_48_1: branch s_48_0 b66 b49
        if s_48_0 {
            return block_66(state, tracer, fn_state);
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
        // N s_49_4: branch s_49_3 b65 b50
        if s_49_3 {
            return block_65(state, tracer, fn_state);
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
        // D s_50_1: write-var gs#61888 <= s_50_0
        fn_state.gs_61888 = s_50_0;
        // N s_50_2: jump b51
        return block_51(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_51_0: read-var gs#61888:u8
        let s_51_0: bool = fn_state.gs_61888;
        // N s_51_1: branch s_51_0 b59 b52
        if s_51_0 {
            return block_59(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_52_0: const #() : ()
        let s_52_0: () = ();
        // S s_52_1: call EL2Enabled(s_52_0)
        let s_52_1: bool = EL2Enabled(state, tracer, s_52_0);
        // N s_52_2: branch s_52_1 b58 b53
        if s_52_1 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_53(state, tracer, fn_state);
        };
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_53_0: const #0u : u8
        let s_53_0: bool = false;
        // D s_53_1: write-var gs#61889 <= s_53_0
        fn_state.gs_61889 = s_53_0;
        // N s_53_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_54_0: read-var gs#61889:u8
        let s_54_0: bool = fn_state.gs_61889;
        // N s_54_1: branch s_54_0 b56 b55
        if s_54_0 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_55_0: const #64s : i64
        let s_55_0: i64 = 64;
        // C s_55_1: const #21912u : u32
        let s_55_1: u32 = 21912;
        // D s_55_2: read-reg s_55_1:struct
        let s_55_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_55_1 as isize);
            tracer.read_register(s_55_1 as isize, value);
            value
        };
        // D s_55_3: call __get_GCSPR_EL1(s_55_2)
        let s_55_3: ProductType5c790c8ef59cc8b2 = u__get_GCSPR_EL1(
            state,
            tracer,
            s_55_2,
        );
        // D s_55_4: write-var ga#66067 <= s_55_3
        fn_state.ga_66067 = s_55_3;
        // D s_55_5: read-var ga#66067.0:struct
        let s_55_5: u64 = fn_state.ga_66067._0;
        // D s_55_6: cast zx s_55_5 -> bv
        let s_55_6: Bits = Bits::new(s_55_5 as u128, 64u16);
        // D s_55_7: read-var t:i
        let s_55_7: i128 = fn_state.t;
        // D s_55_8: call X_set(s_55_7, s_55_0, s_55_6)
        let s_55_8: () = X_set(state, tracer, s_55_7, s_55_0, s_55_6);
        // N s_55_9: return
        return;
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_56_0: const #2240u : u12
        let s_56_0: u16 = 2240;
        // C s_56_1: cast zx s_56_0 -> bv
        let s_56_1: Bits = Bits::new(s_56_0 as u128, 12u16);
        // C s_56_2: cast zx s_56_1 -> i
        let s_56_2: i128 = (s_56_1.value() as i128);
        // C s_56_3: cast reint s_56_2 -> i64
        let s_56_3: i64 = (s_56_2 as i64);
        // C s_56_4: cast zx s_56_3 -> i
        let s_56_4: i128 = (i128::try_from(s_56_3).unwrap());
        // S s_56_5: call NVMem_read(s_56_4)
        let s_56_5: u64 = NVMem_read(state, tracer, s_56_4);
        // D s_56_6: write-var ga#66066 <= s_56_5
        fn_state.ga_66066 = s_56_5;
        // N s_56_7: jump b57
        return block_57(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_57_0: read-var ga#66066:u64
        let s_57_0: u64 = fn_state.ga_66066;
        // D s_57_1: cast zx s_57_0 -> bv
        let s_57_1: Bits = Bits::new(s_57_0 as u128, 64u16);
        // D s_57_2: read-var t:i
        let s_57_2: i128 = fn_state.t;
        // C s_57_3: const #64s : i64
        let s_57_3: i64 = 64;
        // D s_57_4: call X_set(s_57_2, s_57_3, s_57_1)
        let s_57_4: () = X_set(state, tracer, s_57_2, s_57_3, s_57_1);
        // N s_57_5: return
        return;
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_58_0: const #102552u : u32
        let s_58_0: u32 = 102552;
        // D s_58_1: read-reg s_58_0:struct
        let s_58_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_58_0 as isize);
            tracer.read_register(s_58_0 as isize, value);
            value
        };
        // D s_58_2: call _get_HCR_EL2_Type_NV2(s_58_1)
        let s_58_2: bool = u_get_HCR_EL2_Type_NV2(state, tracer, s_58_1);
        // C s_58_3: const #102552u : u32
        let s_58_3: u32 = 102552;
        // D s_58_4: read-reg s_58_3:struct
        let s_58_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_58_3 as isize);
            tracer.read_register(s_58_3 as isize, value);
            value
        };
        // D s_58_5: call _get_HCR_EL2_Type_NV1(s_58_4)
        let s_58_5: bool = u_get_HCR_EL2_Type_NV1(state, tracer, s_58_4);
        // C s_58_6: const #102552u : u32
        let s_58_6: u32 = 102552;
        // D s_58_7: read-reg s_58_6:struct
        let s_58_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_58_6 as isize);
            tracer.read_register(s_58_6 as isize, value);
            value
        };
        // D s_58_8: call _get_HCR_EL2_Type_NV(s_58_7)
        let s_58_8: bool = u_get_HCR_EL2_Type_NV(state, tracer, s_58_7);
        // D s_58_9: cast zx s_58_5 -> bv
        let s_58_9: Bits = Bits::new(s_58_5 as u128, 1u16);
        // D s_58_10: cast zx s_58_8 -> bv
        let s_58_10: Bits = Bits::new(s_58_8 as u128, 1u16);
        // D s_58_11: cast reint s_58_9 -> u128
        let s_58_11: u128 = (s_58_9.value() as u128);
        // D s_58_12: size-of s_58_9
        let s_58_12: u16 = s_58_9.length();
        // D s_58_13: cast reint s_58_10 -> u128
        let s_58_13: u128 = (s_58_10.value() as u128);
        // D s_58_14: size-of s_58_10
        let s_58_14: u16 = s_58_10.length();
        // D s_58_15: lsl s_58_11 s_58_14
        let s_58_15: u128 = s_58_11 << s_58_14;
        // D s_58_16: or s_58_15 s_58_13
        let s_58_16: u128 = ((s_58_15) | (s_58_13));
        // D s_58_17: add s_58_12 s_58_14
        let s_58_17: u16 = (s_58_12 + s_58_14);
        // D s_58_18: create-bits s_58_16 s_58_17
        let s_58_18: Bits = Bits::new(s_58_16, s_58_17);
        // D s_58_19: cast reint s_58_18 -> u8
        let s_58_19: u8 = (s_58_18.value() as u8);
        // D s_58_20: cast zx s_58_2 -> bv
        let s_58_20: Bits = Bits::new(s_58_2 as u128, 1u16);
        // D s_58_21: cast zx s_58_19 -> bv
        let s_58_21: Bits = Bits::new(s_58_19 as u128, 2u16);
        // D s_58_22: cast reint s_58_20 -> u128
        let s_58_22: u128 = (s_58_20.value() as u128);
        // D s_58_23: size-of s_58_20
        let s_58_23: u16 = s_58_20.length();
        // D s_58_24: cast reint s_58_21 -> u128
        let s_58_24: u128 = (s_58_21.value() as u128);
        // D s_58_25: size-of s_58_21
        let s_58_25: u16 = s_58_21.length();
        // D s_58_26: lsl s_58_22 s_58_25
        let s_58_26: u128 = s_58_22 << s_58_25;
        // D s_58_27: or s_58_26 s_58_24
        let s_58_27: u128 = ((s_58_26) | (s_58_24));
        // D s_58_28: add s_58_23 s_58_25
        let s_58_28: u16 = (s_58_23 + s_58_25);
        // D s_58_29: create-bits s_58_27 s_58_28
        let s_58_29: Bits = Bits::new(s_58_27, s_58_28);
        // D s_58_30: cast reint s_58_29 -> u8
        let s_58_30: u8 = (s_58_29.value() as u8);
        // D s_58_31: cast zx s_58_30 -> bv
        let s_58_31: Bits = Bits::new(s_58_30 as u128, 3u16);
        // C s_58_32: const #7u : u8
        let s_58_32: u8 = 7;
        // C s_58_33: cast zx s_58_32 -> bv
        let s_58_33: Bits = Bits::new(s_58_32 as u128, 3u16);
        // D s_58_34: cmp-eq s_58_31 s_58_33
        let s_58_34: bool = ((s_58_31) == (s_58_33));
        // D s_58_35: write-var gs#61889 <= s_58_34
        fn_state.gs_61889 = s_58_34;
        // N s_58_36: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_59_0: const #() : ()
        let s_59_0: () = ();
        // S s_59_1: call Halted(s_59_0)
        let s_59_1: bool = Halted(state, tracer, s_59_0);
        // N s_59_2: branch s_59_1 b64 b60
        if s_59_1 {
            return block_64(state, tracer, fn_state);
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
        // D s_60_1: write-var gs#61892 <= s_60_0
        fn_state.gs_61892 = s_60_0;
        // N s_60_2: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_61_0: read-var gs#61892:u8
        let s_61_0: bool = fn_state.gs_61892;
        // N s_61_1: branch s_61_0 b63 b62
        if s_61_0 {
            return block_63(state, tracer, fn_state);
        } else {
            return block_62(state, tracer, fn_state);
        };
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_62_0: const #24u : u8
        let s_62_0: u8 = 24;
        // C s_62_1: cast zx s_62_0 -> bv
        let s_62_1: Bits = Bits::new(s_62_0 as u128, 8u16);
        // C s_62_2: cast zx s_62_1 -> i
        let s_62_2: i128 = (s_62_1.value() as i128);
        // C s_62_3: cast reint s_62_2 -> i64
        let s_62_3: i64 = (s_62_2 as i64);
        // C s_62_4: cast zx s_62_3 -> i
        let s_62_4: i128 = (i128::try_from(s_62_3).unwrap());
        // C s_62_5: const #424u : u32
        let s_62_5: u32 = 424;
        // D s_62_6: read-reg s_62_5:u8
        let s_62_6: u8 = {
            let value = state.read_register::<u8>(s_62_5 as isize);
            tracer.read_register(s_62_5 as isize, value);
            value
        };
        // D s_62_7: call AArch64_SystemAccessTrap(s_62_6, s_62_4)
        let s_62_7: () = AArch64_SystemAccessTrap(state, tracer, s_62_6, s_62_4);
        // N s_62_8: return
        return;
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_63_0: panic
        panic!("{:?}", ());
        // N s_63_1: return
        return;
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var __EDSCR_SDD:u8
        let s_64_0: bool = fn_state.u__EDSCR_SDD;
        // D s_64_1: cast zx s_64_0 -> bv
        let s_64_1: Bits = Bits::new(s_64_0 as u128, 1u16);
        // C s_64_2: const #1u : u8
        let s_64_2: bool = true;
        // C s_64_3: cast zx s_64_2 -> bv
        let s_64_3: Bits = Bits::new(s_64_2 as u128, 1u16);
        // D s_64_4: cmp-eq s_64_1 s_64_3
        let s_64_4: bool = ((s_64_1) == (s_64_3));
        // D s_64_5: write-var gs#61892 <= s_64_4
        fn_state.gs_61892 = s_64_4;
        // N s_64_6: jump b61
        return block_61(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_65_0: read-var __SCR_EL3_GCSEn:u8
        let s_65_0: bool = fn_state.u__SCR_EL3_GCSEn;
        // D s_65_1: cast zx s_65_0 -> bv
        let s_65_1: Bits = Bits::new(s_65_0 as u128, 1u16);
        // C s_65_2: const #0u : u8
        let s_65_2: bool = false;
        // C s_65_3: cast zx s_65_2 -> bv
        let s_65_3: Bits = Bits::new(s_65_2 as u128, 1u16);
        // D s_65_4: cmp-eq s_65_1 s_65_3
        let s_65_4: bool = ((s_65_1) == (s_65_3));
        // D s_65_5: write-var gs#61888 <= s_65_4
        fn_state.gs_61888 = s_65_4;
        // N s_65_6: jump b51
        return block_51(state, tracer, fn_state);
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
        // D s_67_0: read-var __HFGRTR_EL2_nGCS_EL1:u8
        let s_67_0: bool = fn_state.u__HFGRTR_EL2_nGCS_EL1;
        // D s_67_1: cast zx s_67_0 -> bv
        let s_67_1: Bits = Bits::new(s_67_0 as u128, 1u16);
        // C s_67_2: const #0u : u8
        let s_67_2: bool = false;
        // C s_67_3: cast zx s_67_2 -> bv
        let s_67_3: Bits = Bits::new(s_67_2 as u128, 1u16);
        // D s_67_4: cmp-eq s_67_1 s_67_3
        let s_67_4: bool = ((s_67_1) == (s_67_3));
        // D s_67_5: write-var gs#61887 <= s_67_4
        fn_state.gs_61887 = s_67_4;
        // N s_67_6: jump b48
        return block_48(state, tracer, fn_state);
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
        // D s_68_4: not s_68_3
        let s_68_4: bool = !s_68_3;
        // N s_68_5: branch s_68_4 b71 b69
        if s_68_4 {
            return block_71(state, tracer, fn_state);
        } else {
            return block_69(state, tracer, fn_state);
        };
    }
    fn block_69<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_69_0: read-var __SCR_EL3_FGTEn:u8
        let s_69_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_69_1: cast zx s_69_0 -> bv
        let s_69_1: Bits = Bits::new(s_69_0 as u128, 1u16);
        // C s_69_2: const #1u : u8
        let s_69_2: bool = true;
        // C s_69_3: cast zx s_69_2 -> bv
        let s_69_3: Bits = Bits::new(s_69_2 as u128, 1u16);
        // D s_69_4: cmp-eq s_69_1 s_69_3
        let s_69_4: bool = ((s_69_1) == (s_69_3));
        // D s_69_5: write-var gs#61885 <= s_69_4
        fn_state.gs_61885 = s_69_4;
        // N s_69_6: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_70_0: read-var gs#61885:u8
        let s_70_0: bool = fn_state.gs_61885;
        // D s_70_1: write-var gs#61886 <= s_70_0
        fn_state.gs_61886 = s_70_0;
        // N s_70_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #1u : u8
        let s_71_0: bool = true;
        // D s_71_1: write-var gs#61885 <= s_71_0
        fn_state.gs_61885 = s_71_0;
        // N s_71_2: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_72<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_72_0: const #146u : u32
        let s_72_0: u32 = 146;
        // S s_72_1: call IsFeatureImplemented(s_72_0)
        let s_72_1: bool = IsFeatureImplemented(state, tracer, s_72_0);
        // D s_72_2: write-var gs#61884 <= s_72_1
        fn_state.gs_61884 = s_72_1;
        // N s_72_3: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_73<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_73_0: panic
        panic!("{:?}", ());
        // N s_73_1: return
        return;
    }
    fn block_74<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_74_0: read-var __SCR_EL3_GCSEn:u8
        let s_74_0: bool = fn_state.u__SCR_EL3_GCSEn;
        // D s_74_1: cast zx s_74_0 -> bv
        let s_74_1: Bits = Bits::new(s_74_0 as u128, 1u16);
        // C s_74_2: const #0u : u8
        let s_74_2: bool = false;
        // C s_74_3: cast zx s_74_2 -> bv
        let s_74_3: Bits = Bits::new(s_74_2 as u128, 1u16);
        // D s_74_4: cmp-eq s_74_1 s_74_3
        let s_74_4: bool = ((s_74_1) == (s_74_3));
        // D s_74_5: write-var gs#61883 <= s_74_4
        fn_state.gs_61883 = s_74_4;
        // N s_74_6: jump b41
        return block_41(state, tracer, fn_state);
    }
    fn block_75<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_75_0: const #"EL3 trap priority when SDD == '1'" : str
        let s_75_0: &'static str = "EL3 trap priority when SDD == '1'";
        // S s_75_1: call __IMPDEF_boolean(s_75_0)
        let s_75_1: bool = u__IMPDEF_boolean(state, tracer, s_75_0);
        // D s_75_2: write-var gs#61882 <= s_75_1
        fn_state.gs_61882 = s_75_1;
        // N s_75_3: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_76<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_76_0: read-var __EDSCR_SDD:u8
        let s_76_0: bool = fn_state.u__EDSCR_SDD;
        // D s_76_1: cast zx s_76_0 -> bv
        let s_76_1: Bits = Bits::new(s_76_0 as u128, 1u16);
        // C s_76_2: const #1u : u8
        let s_76_2: bool = true;
        // C s_76_3: cast zx s_76_2 -> bv
        let s_76_3: Bits = Bits::new(s_76_2 as u128, 1u16);
        // D s_76_4: cmp-eq s_76_1 s_76_3
        let s_76_4: bool = ((s_76_1) == (s_76_3));
        // D s_76_5: write-var gs#61881 <= s_76_4
        fn_state.gs_61881 = s_76_4;
        // N s_76_6: jump b37
        return block_37(state, tracer, fn_state);
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
        // D s_77_4: write-var gs#61880 <= s_77_3
        fn_state.gs_61880 = s_77_3;
        // N s_77_5: jump b35
        return block_35(state, tracer, fn_state);
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
}
