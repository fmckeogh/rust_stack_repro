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
use u_get_SCR_EL3_Type_D128En::*;
use Halted::*;
use u_get_HFGRTR_EL2_Type_PAR_EL1::*;
use IsFeatureImplemented::*;
use AArch64_SystemAccessTrap::*;
use u__IMPDEF_boolean::*;
use u_get_HCRX_EL2_Type_D128En::*;
use X_set::*;
use PAR_EL1_read::*;
use IsHCRXEL2Enabled::*;
use u_get_EDSCR_Type_SDD::*;
use EL2Enabled::*;
use EDSCR_read::*;
use common::*;
pub fn PAR_EL1_SysRegRead128_8206ac7c9b0eef57<T: Tracer>(
    state: &mut State,
    tracer: &T,
    el: u8,
    op0: u8,
    op1: u8,
    CRn: u8,
    op2: u8,
    CRm: u8,
    t: i128,
    t2: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_238892: ProductType782ac6922b48c20d,
        u__HCRX_EL2_D128En: bool,
        u__SCR_EL3_D128En: bool,
        gs_137068: bool,
        u__EDSCR_SDD: bool,
        gs_137052: bool,
        ga_238873: ProductTypeb78df3ce1505b121,
        gs_137069: bool,
        gs_137067: bool,
        ga_238867: ProductType782ac6922b48c20d,
        u__SCR_EL3_FGTEn: bool,
        gs_137090: bool,
        ga_238903: ProductType782ac6922b48c20d,
        gs_137078: bool,
        gs_137051: bool,
        gs_137070: bool,
        gs_137055: bool,
        ga_238890: ProductType782ac6922b48c20d,
        gs_137053: bool,
        gs_137074: bool,
        gs_137075: bool,
        ga_238901: ProductType782ac6922b48c20d,
        gs_137071: bool,
        gs_137073: bool,
        gs_137076: bool,
        ga_238869: ProductType782ac6922b48c20d,
        u__HFGRTR_EL2_PAR_EL1: bool,
        u__PSTATE_EL: u8,
        ga_238896: ProductTypeb78df3ce1505b121,
        gs_137077: bool,
        gs_137054: bool,
        gs_137072: bool,
        ga_238907: ProductTypeb78df3ce1505b121,
        el: u8,
        op0: u8,
        op1: u8,
        CRn: u8,
        op2: u8,
        CRm: u8,
        t: i128,
        t2: i128,
    }
    let fn_state = FunctionState {
        el,
        op0,
        op1,
        CRn,
        op2,
        CRm,
        t,
        t2,
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
        // D s_0_9: call _get_SCR_EL3_Type_D128En(s_0_8)
        let s_0_9: bool = u_get_SCR_EL3_Type_D128En(state, tracer, s_0_8);
        // D s_0_10: write-var __SCR_EL3_D128En <= s_0_9
        fn_state.u__SCR_EL3_D128En = s_0_9;
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
        // D s_0_17: call _get_HFGRTR_EL2_Type_PAR_EL1(s_0_16)
        let s_0_17: bool = u_get_HFGRTR_EL2_Type_PAR_EL1(state, tracer, s_0_16);
        // D s_0_18: write-var __HFGRTR_EL2_PAR_EL1 <= s_0_17
        fn_state.u__HFGRTR_EL2_PAR_EL1 = s_0_17;
        // C s_0_19: const #22528u : u32
        let s_0_19: u32 = 22528;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_HCRX_EL2_Type_D128En(s_0_20)
        let s_0_21: bool = u_get_HCRX_EL2_Type_D128En(state, tracer, s_0_20);
        // D s_0_22: write-var __HCRX_EL2_D128En <= s_0_21
        fn_state.u__HCRX_EL2_D128En = s_0_21;
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
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call PAR_EL1_read(s_5_0)
        let s_5_1: ProductType782ac6922b48c20d = PAR_EL1_read(state, tracer, s_5_0);
        // D s_5_2: write-var ga#238901 <= s_5_1
        fn_state.ga_238901 = s_5_1;
        // D s_5_3: read-var ga#238901.0:struct
        let s_5_3: u128 = fn_state.ga_238901._0;
        // C s_5_4: const #64s : i
        let s_5_4: i128 = 64;
        // D s_5_5: cast zx s_5_3 -> bv
        let s_5_5: Bits = Bits::new(s_5_3 as u128, 128u16);
        // C s_5_6: const #1s : i64
        let s_5_6: i64 = 1;
        // C s_5_7: cast zx s_5_6 -> i
        let s_5_7: i128 = (i128::try_from(s_5_6).unwrap());
        // C s_5_8: const #63s : i
        let s_5_8: i128 = 63;
        // C s_5_9: add s_5_8 s_5_7
        let s_5_9: i128 = (s_5_8 + s_5_7);
        // D s_5_10: bit-extract s_5_5 s_5_4 s_5_9
        let s_5_10: Bits = (Bits::new(
            ((s_5_5) >> (s_5_4)).value(),
            u16::try_from(s_5_9).unwrap(),
        ));
        // D s_5_11: cast reint s_5_10 -> u64
        let s_5_11: u64 = (s_5_10.value() as u64);
        // C s_5_12: const #() : ()
        let s_5_12: () = ();
        // S s_5_13: call PAR_EL1_read(s_5_12)
        let s_5_13: ProductType782ac6922b48c20d = PAR_EL1_read(state, tracer, s_5_12);
        // D s_5_14: write-var ga#238903 <= s_5_13
        fn_state.ga_238903 = s_5_13;
        // D s_5_15: read-var ga#238903.0:struct
        let s_5_15: u128 = fn_state.ga_238903._0;
        // C s_5_16: const #0s : i
        let s_5_16: i128 = 0;
        // D s_5_17: cast zx s_5_15 -> bv
        let s_5_17: Bits = Bits::new(s_5_15 as u128, 128u16);
        // C s_5_18: const #1s : i64
        let s_5_18: i64 = 1;
        // C s_5_19: cast zx s_5_18 -> i
        let s_5_19: i128 = (i128::try_from(s_5_18).unwrap());
        // C s_5_20: const #63s : i
        let s_5_20: i128 = 63;
        // C s_5_21: add s_5_20 s_5_19
        let s_5_21: i128 = (s_5_20 + s_5_19);
        // D s_5_22: bit-extract s_5_17 s_5_16 s_5_21
        let s_5_22: Bits = (Bits::new(
            ((s_5_17) >> (s_5_16)).value(),
            u16::try_from(s_5_21).unwrap(),
        ));
        // D s_5_23: cast reint s_5_22 -> u64
        let s_5_23: u64 = (s_5_22.value() as u64);
        // D s_5_24: create-product struct = ["s_5_11", "s_5_23"]
        let s_5_24: ProductTypeb78df3ce1505b121 = ProductTypeb78df3ce1505b121 {
            _0: s_5_11,
            _1: s_5_23,
        };
        // D s_5_25: write-var ga#238907 <= s_5_24
        fn_state.ga_238907 = s_5_24;
        // D s_5_26: read-var ga#238907.0:struct
        let s_5_26: u64 = fn_state.ga_238907._0;
        // D s_5_27: read-var ga#238907.1:struct
        let s_5_27: u64 = fn_state.ga_238907._1;
        // C s_5_28: const #1s : i
        let s_5_28: i128 = 1;
        // D s_5_29: read-var t:i
        let s_5_29: i128 = fn_state.t;
        // D s_5_30: add s_5_29 s_5_28
        let s_5_30: i128 = (s_5_29 + s_5_28);
        // C s_5_31: const #64s : i64
        let s_5_31: i64 = 64;
        // D s_5_32: cast zx s_5_26 -> bv
        let s_5_32: Bits = Bits::new(s_5_26 as u128, 64u16);
        // D s_5_33: call X_set(s_5_30, s_5_31, s_5_32)
        let s_5_33: () = X_set(state, tracer, s_5_30, s_5_31, s_5_32);
        // C s_5_34: const #64s : i64
        let s_5_34: i64 = 64;
        // D s_5_35: cast zx s_5_27 -> bv
        let s_5_35: Bits = Bits::new(s_5_27 as u128, 64u16);
        // D s_5_36: read-var t:i
        let s_5_36: i128 = fn_state.t;
        // D s_5_37: call X_set(s_5_36, s_5_34, s_5_35)
        let s_5_37: () = X_set(state, tracer, s_5_36, s_5_34, s_5_35);
        // N s_5_38: return
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
        // D s_7_1: write-var gs#137051 <= s_7_0
        fn_state.gs_137051 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#137051:u8
        let s_8_0: bool = fn_state.gs_137051;
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
        // D s_9_1: write-var gs#137052 <= s_9_0
        fn_state.gs_137052 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#137052:u8
        let s_10_0: bool = fn_state.gs_137052;
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
        // D s_11_1: write-var gs#137053 <= s_11_0
        fn_state.gs_137053 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var gs#137053:u8
        let s_12_0: bool = fn_state.gs_137053;
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
        // D s_13_1: write-var gs#137054 <= s_13_0
        fn_state.gs_137054 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#137054:u8
        let s_14_0: bool = fn_state.gs_137054;
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
        // D s_16_1: write-var gs#137055 <= s_16_0
        fn_state.gs_137055 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#137055:u8
        let s_17_0: bool = fn_state.gs_137055;
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
        // C s_18_0: const #() : ()
        let s_18_0: () = ();
        // S s_18_1: call PAR_EL1_read(s_18_0)
        let s_18_1: ProductType782ac6922b48c20d = PAR_EL1_read(state, tracer, s_18_0);
        // D s_18_2: write-var ga#238890 <= s_18_1
        fn_state.ga_238890 = s_18_1;
        // D s_18_3: read-var ga#238890.0:struct
        let s_18_3: u128 = fn_state.ga_238890._0;
        // C s_18_4: const #64s : i
        let s_18_4: i128 = 64;
        // D s_18_5: cast zx s_18_3 -> bv
        let s_18_5: Bits = Bits::new(s_18_3 as u128, 128u16);
        // C s_18_6: const #1s : i64
        let s_18_6: i64 = 1;
        // C s_18_7: cast zx s_18_6 -> i
        let s_18_7: i128 = (i128::try_from(s_18_6).unwrap());
        // C s_18_8: const #63s : i
        let s_18_8: i128 = 63;
        // C s_18_9: add s_18_8 s_18_7
        let s_18_9: i128 = (s_18_8 + s_18_7);
        // D s_18_10: bit-extract s_18_5 s_18_4 s_18_9
        let s_18_10: Bits = (Bits::new(
            ((s_18_5) >> (s_18_4)).value(),
            u16::try_from(s_18_9).unwrap(),
        ));
        // D s_18_11: cast reint s_18_10 -> u64
        let s_18_11: u64 = (s_18_10.value() as u64);
        // C s_18_12: const #() : ()
        let s_18_12: () = ();
        // S s_18_13: call PAR_EL1_read(s_18_12)
        let s_18_13: ProductType782ac6922b48c20d = PAR_EL1_read(state, tracer, s_18_12);
        // D s_18_14: write-var ga#238892 <= s_18_13
        fn_state.ga_238892 = s_18_13;
        // D s_18_15: read-var ga#238892.0:struct
        let s_18_15: u128 = fn_state.ga_238892._0;
        // C s_18_16: const #0s : i
        let s_18_16: i128 = 0;
        // D s_18_17: cast zx s_18_15 -> bv
        let s_18_17: Bits = Bits::new(s_18_15 as u128, 128u16);
        // C s_18_18: const #1s : i64
        let s_18_18: i64 = 1;
        // C s_18_19: cast zx s_18_18 -> i
        let s_18_19: i128 = (i128::try_from(s_18_18).unwrap());
        // C s_18_20: const #63s : i
        let s_18_20: i128 = 63;
        // C s_18_21: add s_18_20 s_18_19
        let s_18_21: i128 = (s_18_20 + s_18_19);
        // D s_18_22: bit-extract s_18_17 s_18_16 s_18_21
        let s_18_22: Bits = (Bits::new(
            ((s_18_17) >> (s_18_16)).value(),
            u16::try_from(s_18_21).unwrap(),
        ));
        // D s_18_23: cast reint s_18_22 -> u64
        let s_18_23: u64 = (s_18_22.value() as u64);
        // D s_18_24: create-product struct = ["s_18_11", "s_18_23"]
        let s_18_24: ProductTypeb78df3ce1505b121 = ProductTypeb78df3ce1505b121 {
            _0: s_18_11,
            _1: s_18_23,
        };
        // D s_18_25: write-var ga#238896 <= s_18_24
        fn_state.ga_238896 = s_18_24;
        // D s_18_26: read-var ga#238896.0:struct
        let s_18_26: u64 = fn_state.ga_238896._0;
        // D s_18_27: read-var ga#238896.1:struct
        let s_18_27: u64 = fn_state.ga_238896._1;
        // C s_18_28: const #1s : i
        let s_18_28: i128 = 1;
        // D s_18_29: read-var t:i
        let s_18_29: i128 = fn_state.t;
        // D s_18_30: add s_18_29 s_18_28
        let s_18_30: i128 = (s_18_29 + s_18_28);
        // C s_18_31: const #64s : i64
        let s_18_31: i64 = 64;
        // D s_18_32: cast zx s_18_26 -> bv
        let s_18_32: Bits = Bits::new(s_18_26 as u128, 64u16);
        // D s_18_33: call X_set(s_18_30, s_18_31, s_18_32)
        let s_18_33: () = X_set(state, tracer, s_18_30, s_18_31, s_18_32);
        // C s_18_34: const #64s : i64
        let s_18_34: i64 = 64;
        // D s_18_35: cast zx s_18_27 -> bv
        let s_18_35: Bits = Bits::new(s_18_27 as u128, 64u16);
        // D s_18_36: read-var t:i
        let s_18_36: i128 = fn_state.t;
        // D s_18_37: call X_set(s_18_36, s_18_34, s_18_35)
        let s_18_37: () = X_set(state, tracer, s_18_36, s_18_34, s_18_35);
        // N s_18_38: return
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
        // D s_20_1: write-var gs#137067 <= s_20_0
        fn_state.gs_137067 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#137067:u8
        let s_21_0: bool = fn_state.gs_137067;
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
        // C s_22_0: const #20u : u8
        let s_22_0: u8 = 20;
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
        // D s_24_5: write-var gs#137067 <= s_24_4
        fn_state.gs_137067 = s_24_4;
        // N s_24_6: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var __SCR_EL3_D128En:u8
        let s_25_0: bool = fn_state.u__SCR_EL3_D128En;
        // D s_25_1: cast zx s_25_0 -> bv
        let s_25_1: Bits = Bits::new(s_25_0 as u128, 1u16);
        // C s_25_2: const #0u : u8
        let s_25_2: bool = false;
        // C s_25_3: cast zx s_25_2 -> bv
        let s_25_3: Bits = Bits::new(s_25_2 as u128, 1u16);
        // D s_25_4: cmp-eq s_25_1 s_25_3
        let s_25_4: bool = ((s_25_1) == (s_25_3));
        // D s_25_5: write-var gs#137055 <= s_25_4
        fn_state.gs_137055 = s_25_4;
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
        // D s_27_0: read-var __SCR_EL3_D128En:u8
        let s_27_0: bool = fn_state.u__SCR_EL3_D128En;
        // D s_27_1: cast zx s_27_0 -> bv
        let s_27_1: Bits = Bits::new(s_27_0 as u128, 1u16);
        // C s_27_2: const #0u : u8
        let s_27_2: bool = false;
        // C s_27_3: cast zx s_27_2 -> bv
        let s_27_3: Bits = Bits::new(s_27_2 as u128, 1u16);
        // D s_27_4: cmp-eq s_27_1 s_27_3
        let s_27_4: bool = ((s_27_1) == (s_27_3));
        // D s_27_5: write-var gs#137054 <= s_27_4
        fn_state.gs_137054 = s_27_4;
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
        // D s_28_2: write-var gs#137053 <= s_28_1
        fn_state.gs_137053 = s_28_1;
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
        // D s_29_5: write-var gs#137052 <= s_29_4
        fn_state.gs_137052 = s_29_4;
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
        // D s_30_4: write-var gs#137051 <= s_30_3
        fn_state.gs_137051 = s_30_3;
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
        // N s_31_2: branch s_31_1 b77 b32
        if s_31_1 {
            return block_77(state, tracer, fn_state);
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
        // D s_32_1: write-var gs#137068 <= s_32_0
        fn_state.gs_137068 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#137068:u8
        let s_33_0: bool = fn_state.gs_137068;
        // N s_33_1: branch s_33_0 b76 b34
        if s_33_0 {
            return block_76(state, tracer, fn_state);
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
        // D s_34_1: write-var gs#137069 <= s_34_0
        fn_state.gs_137069 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var gs#137069:u8
        let s_35_0: bool = fn_state.gs_137069;
        // N s_35_1: branch s_35_0 b75 b36
        if s_35_0 {
            return block_75(state, tracer, fn_state);
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
        // D s_36_1: write-var gs#137070 <= s_36_0
        fn_state.gs_137070 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_37_0: read-var gs#137070:u8
        let s_37_0: bool = fn_state.gs_137070;
        // N s_37_1: branch s_37_0 b74 b38
        if s_37_0 {
            return block_74(state, tracer, fn_state);
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
        // D s_38_1: write-var gs#137071 <= s_38_0
        fn_state.gs_137071 = s_38_0;
        // N s_38_2: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_39_0: read-var gs#137071:u8
        let s_39_0: bool = fn_state.gs_137071;
        // N s_39_1: branch s_39_0 b73 b40
        if s_39_0 {
            return block_73(state, tracer, fn_state);
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
        // N s_40_2: branch s_40_1 b72 b41
        if s_40_1 {
            return block_72(state, tracer, fn_state);
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
        // D s_41_1: write-var gs#137072 <= s_41_0
        fn_state.gs_137072 = s_41_0;
        // N s_41_2: jump b42
        return block_42(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_42_0: read-var gs#137072:u8
        let s_42_0: bool = fn_state.gs_137072;
        // N s_42_1: branch s_42_0 b68 b43
        if s_42_0 {
            return block_68(state, tracer, fn_state);
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
        // D s_43_1: write-var gs#137074 <= s_43_0
        fn_state.gs_137074 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var gs#137074:u8
        let s_44_0: bool = fn_state.gs_137074;
        // N s_44_1: branch s_44_0 b67 b45
        if s_44_0 {
            return block_67(state, tracer, fn_state);
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
        // D s_45_1: write-var gs#137075 <= s_45_0
        fn_state.gs_137075 = s_45_0;
        // N s_45_2: jump b46
        return block_46(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_46_0: read-var gs#137075:u8
        let s_46_0: bool = fn_state.gs_137075;
        // N s_46_1: branch s_46_0 b66 b47
        if s_46_0 {
            return block_66(state, tracer, fn_state);
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
        // D s_48_1: write-var gs#137077 <= s_48_0
        fn_state.gs_137077 = s_48_0;
        // N s_48_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_49_0: read-var gs#137077:u8
        let s_49_0: bool = fn_state.gs_137077;
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
        // D s_51_1: write-var gs#137078 <= s_51_0
        fn_state.gs_137078 = s_51_0;
        // N s_51_2: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_52_0: read-var gs#137078:u8
        let s_52_0: bool = fn_state.gs_137078;
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
        // C s_53_0: const #() : ()
        let s_53_0: () = ();
        // S s_53_1: call PAR_EL1_read(s_53_0)
        let s_53_1: ProductType782ac6922b48c20d = PAR_EL1_read(state, tracer, s_53_0);
        // D s_53_2: write-var ga#238867 <= s_53_1
        fn_state.ga_238867 = s_53_1;
        // D s_53_3: read-var ga#238867.0:struct
        let s_53_3: u128 = fn_state.ga_238867._0;
        // C s_53_4: const #64s : i
        let s_53_4: i128 = 64;
        // D s_53_5: cast zx s_53_3 -> bv
        let s_53_5: Bits = Bits::new(s_53_3 as u128, 128u16);
        // C s_53_6: const #1s : i64
        let s_53_6: i64 = 1;
        // C s_53_7: cast zx s_53_6 -> i
        let s_53_7: i128 = (i128::try_from(s_53_6).unwrap());
        // C s_53_8: const #63s : i
        let s_53_8: i128 = 63;
        // C s_53_9: add s_53_8 s_53_7
        let s_53_9: i128 = (s_53_8 + s_53_7);
        // D s_53_10: bit-extract s_53_5 s_53_4 s_53_9
        let s_53_10: Bits = (Bits::new(
            ((s_53_5) >> (s_53_4)).value(),
            u16::try_from(s_53_9).unwrap(),
        ));
        // D s_53_11: cast reint s_53_10 -> u64
        let s_53_11: u64 = (s_53_10.value() as u64);
        // C s_53_12: const #() : ()
        let s_53_12: () = ();
        // S s_53_13: call PAR_EL1_read(s_53_12)
        let s_53_13: ProductType782ac6922b48c20d = PAR_EL1_read(state, tracer, s_53_12);
        // D s_53_14: write-var ga#238869 <= s_53_13
        fn_state.ga_238869 = s_53_13;
        // D s_53_15: read-var ga#238869.0:struct
        let s_53_15: u128 = fn_state.ga_238869._0;
        // C s_53_16: const #0s : i
        let s_53_16: i128 = 0;
        // D s_53_17: cast zx s_53_15 -> bv
        let s_53_17: Bits = Bits::new(s_53_15 as u128, 128u16);
        // C s_53_18: const #1s : i64
        let s_53_18: i64 = 1;
        // C s_53_19: cast zx s_53_18 -> i
        let s_53_19: i128 = (i128::try_from(s_53_18).unwrap());
        // C s_53_20: const #63s : i
        let s_53_20: i128 = 63;
        // C s_53_21: add s_53_20 s_53_19
        let s_53_21: i128 = (s_53_20 + s_53_19);
        // D s_53_22: bit-extract s_53_17 s_53_16 s_53_21
        let s_53_22: Bits = (Bits::new(
            ((s_53_17) >> (s_53_16)).value(),
            u16::try_from(s_53_21).unwrap(),
        ));
        // D s_53_23: cast reint s_53_22 -> u64
        let s_53_23: u64 = (s_53_22.value() as u64);
        // D s_53_24: create-product struct = ["s_53_11", "s_53_23"]
        let s_53_24: ProductTypeb78df3ce1505b121 = ProductTypeb78df3ce1505b121 {
            _0: s_53_11,
            _1: s_53_23,
        };
        // D s_53_25: write-var ga#238873 <= s_53_24
        fn_state.ga_238873 = s_53_24;
        // D s_53_26: read-var ga#238873.0:struct
        let s_53_26: u64 = fn_state.ga_238873._0;
        // D s_53_27: read-var ga#238873.1:struct
        let s_53_27: u64 = fn_state.ga_238873._1;
        // C s_53_28: const #1s : i
        let s_53_28: i128 = 1;
        // D s_53_29: read-var t:i
        let s_53_29: i128 = fn_state.t;
        // D s_53_30: add s_53_29 s_53_28
        let s_53_30: i128 = (s_53_29 + s_53_28);
        // C s_53_31: const #64s : i64
        let s_53_31: i64 = 64;
        // D s_53_32: cast zx s_53_26 -> bv
        let s_53_32: Bits = Bits::new(s_53_26 as u128, 64u16);
        // D s_53_33: call X_set(s_53_30, s_53_31, s_53_32)
        let s_53_33: () = X_set(state, tracer, s_53_30, s_53_31, s_53_32);
        // C s_53_34: const #64s : i64
        let s_53_34: i64 = 64;
        // D s_53_35: cast zx s_53_27 -> bv
        let s_53_35: Bits = Bits::new(s_53_27 as u128, 64u16);
        // D s_53_36: read-var t:i
        let s_53_36: i128 = fn_state.t;
        // D s_53_37: call X_set(s_53_36, s_53_34, s_53_35)
        let s_53_37: () = X_set(state, tracer, s_53_36, s_53_34, s_53_35);
        // N s_53_38: return
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
        // D s_55_1: write-var gs#137090 <= s_55_0
        fn_state.gs_137090 = s_55_0;
        // N s_55_2: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_56_0: read-var gs#137090:u8
        let s_56_0: bool = fn_state.gs_137090;
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
        // C s_57_0: const #20u : u8
        let s_57_0: u8 = 20;
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
        // D s_59_5: write-var gs#137090 <= s_59_4
        fn_state.gs_137090 = s_59_4;
        // N s_59_6: jump b56
        return block_56(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_60_0: read-var __SCR_EL3_D128En:u8
        let s_60_0: bool = fn_state.u__SCR_EL3_D128En;
        // D s_60_1: cast zx s_60_0 -> bv
        let s_60_1: Bits = Bits::new(s_60_0 as u128, 1u16);
        // C s_60_2: const #0u : u8
        let s_60_2: bool = false;
        // C s_60_3: cast zx s_60_2 -> bv
        let s_60_3: Bits = Bits::new(s_60_2 as u128, 1u16);
        // D s_60_4: cmp-eq s_60_1 s_60_3
        let s_60_4: bool = ((s_60_1) == (s_60_3));
        // D s_60_5: write-var gs#137078 <= s_60_4
        fn_state.gs_137078 = s_60_4;
        // N s_60_6: jump b52
        return block_52(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_61_0: const #20u : u8
        let s_61_0: u8 = 20;
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
        // C s_62_0: const #() : ()
        let s_62_0: () = ();
        // S s_62_1: call IsHCRXEL2Enabled(s_62_0)
        let s_62_1: bool = IsHCRXEL2Enabled(state, tracer, s_62_0);
        // S s_62_2: not s_62_1
        let s_62_2: bool = !s_62_1;
        // N s_62_3: branch s_62_2 b65 b63
        if s_62_2 {
            return block_65(state, tracer, fn_state);
        } else {
            return block_63(state, tracer, fn_state);
        };
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_63_0: read-var __HCRX_EL2_D128En:u8
        let s_63_0: bool = fn_state.u__HCRX_EL2_D128En;
        // D s_63_1: cast zx s_63_0 -> bv
        let s_63_1: Bits = Bits::new(s_63_0 as u128, 1u16);
        // C s_63_2: const #0u : u8
        let s_63_2: bool = false;
        // C s_63_3: cast zx s_63_2 -> bv
        let s_63_3: Bits = Bits::new(s_63_2 as u128, 1u16);
        // D s_63_4: cmp-eq s_63_1 s_63_3
        let s_63_4: bool = ((s_63_1) == (s_63_3));
        // D s_63_5: write-var gs#137076 <= s_63_4
        fn_state.gs_137076 = s_63_4;
        // N s_63_6: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_64_0: read-var gs#137076:u8
        let s_64_0: bool = fn_state.gs_137076;
        // D s_64_1: write-var gs#137077 <= s_64_0
        fn_state.gs_137077 = s_64_0;
        // N s_64_2: jump b49
        return block_49(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_65_0: const #1u : u8
        let s_65_0: bool = true;
        // D s_65_1: write-var gs#137076 <= s_65_0
        fn_state.gs_137076 = s_65_0;
        // N s_65_2: jump b64
        return block_64(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_66_0: const #20u : u8
        let s_66_0: u8 = 20;
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
        // D s_67_0: read-var __HFGRTR_EL2_PAR_EL1:u8
        let s_67_0: bool = fn_state.u__HFGRTR_EL2_PAR_EL1;
        // D s_67_1: cast zx s_67_0 -> bv
        let s_67_1: Bits = Bits::new(s_67_0 as u128, 1u16);
        // C s_67_2: const #1u : u8
        let s_67_2: bool = true;
        // C s_67_3: cast zx s_67_2 -> bv
        let s_67_3: Bits = Bits::new(s_67_2 as u128, 1u16);
        // D s_67_4: cmp-eq s_67_1 s_67_3
        let s_67_4: bool = ((s_67_1) == (s_67_3));
        // D s_67_5: write-var gs#137075 <= s_67_4
        fn_state.gs_137075 = s_67_4;
        // N s_67_6: jump b46
        return block_46(state, tracer, fn_state);
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
        // D s_69_5: write-var gs#137073 <= s_69_4
        fn_state.gs_137073 = s_69_4;
        // N s_69_6: jump b70
        return block_70(state, tracer, fn_state);
    }
    fn block_70<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_70_0: read-var gs#137073:u8
        let s_70_0: bool = fn_state.gs_137073;
        // D s_70_1: write-var gs#137074 <= s_70_0
        fn_state.gs_137074 = s_70_0;
        // N s_70_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_71<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_71_0: const #1u : u8
        let s_71_0: bool = true;
        // D s_71_1: write-var gs#137073 <= s_71_0
        fn_state.gs_137073 = s_71_0;
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
        // D s_72_2: write-var gs#137072 <= s_72_1
        fn_state.gs_137072 = s_72_1;
        // N s_72_3: jump b42
        return block_42(state, tracer, fn_state);
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
        // D s_74_0: read-var __SCR_EL3_D128En:u8
        let s_74_0: bool = fn_state.u__SCR_EL3_D128En;
        // D s_74_1: cast zx s_74_0 -> bv
        let s_74_1: Bits = Bits::new(s_74_0 as u128, 1u16);
        // C s_74_2: const #0u : u8
        let s_74_2: bool = false;
        // C s_74_3: cast zx s_74_2 -> bv
        let s_74_3: Bits = Bits::new(s_74_2 as u128, 1u16);
        // D s_74_4: cmp-eq s_74_1 s_74_3
        let s_74_4: bool = ((s_74_1) == (s_74_3));
        // D s_74_5: write-var gs#137071 <= s_74_4
        fn_state.gs_137071 = s_74_4;
        // N s_74_6: jump b39
        return block_39(state, tracer, fn_state);
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
        // D s_75_2: write-var gs#137070 <= s_75_1
        fn_state.gs_137070 = s_75_1;
        // N s_75_3: jump b37
        return block_37(state, tracer, fn_state);
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
        // D s_76_5: write-var gs#137069 <= s_76_4
        fn_state.gs_137069 = s_76_4;
        // N s_76_6: jump b35
        return block_35(state, tracer, fn_state);
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
        // D s_77_4: write-var gs#137068 <= s_77_3
        fn_state.gs_137068 = s_77_3;
        // N s_77_5: jump b33
        return block_33(state, tracer, fn_state);
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
