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
use u_get_HCR_EL2_Type_TRVM::*;
use NVMem_read::*;
use AArch64_SystemAccessTrap::*;
use IsFeatureImplemented::*;
use u__get_CONTEXTIDR_EL1::*;
use u_get_HCR_EL2_Type_NV1::*;
use X_set::*;
use u__get_CONTEXTIDR_EL2::*;
use u_get_HCR_EL2_Type_NV::*;
use u_get_HFGRTR_EL2_Type_CONTEXTIDR_EL1::*;
use EL2Enabled::*;
use u_get_HCR_EL2_Type_NV2::*;
use common::*;
pub fn CONTEXTIDR_EL2_SysRegRead_0a5ce1fa99f54107<T: Tracer>(
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
        ga_57122: ProductType5c790c8ef59cc8b2,
        gs_58182: bool,
        u__HCR_EL2_E2H: bool,
        u__HFGRTR_EL2_CONTEXTIDR_EL1: bool,
        ga_57126: ProductType5c790c8ef59cc8b2,
        u__SCR_EL3_FGTEn: bool,
        gs_58185: bool,
        gs_58184: bool,
        gs_58181: bool,
        u__PSTATE_EL: u8,
        u__HCR_EL2_TRVM: bool,
        ga_57113: u64,
        ga_57114: ProductType5c790c8ef59cc8b2,
        ga_57119: ProductType5c790c8ef59cc8b2,
        gs_58180: bool,
        gs_58183: bool,
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
        // C s_0_3: const #102552u : u32
        let s_0_3: u32 = 102552;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_HCR_EL2_Type_TRVM(s_0_4)
        let s_0_5: bool = u_get_HCR_EL2_Type_TRVM(state, tracer, s_0_4);
        // D s_0_6: write-var __HCR_EL2_TRVM <= s_0_5
        fn_state.u__HCR_EL2_TRVM = s_0_5;
        // C s_0_7: const #90704u : u32
        let s_0_7: u32 = 90704;
        // D s_0_8: read-reg s_0_7:struct
        let s_0_8: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_7 as isize);
            tracer.read_register(s_0_7 as isize, value);
            value
        };
        // D s_0_9: call _get_SCR_EL3_Type_FGTEn(s_0_8)
        let s_0_9: bool = u_get_SCR_EL3_Type_FGTEn(state, tracer, s_0_8);
        // D s_0_10: write-var __SCR_EL3_FGTEn <= s_0_9
        fn_state.u__SCR_EL3_FGTEn = s_0_9;
        // C s_0_11: const #16592u : u32
        let s_0_11: u32 = 16592;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_HFGRTR_EL2_Type_CONTEXTIDR_EL1(s_0_12)
        let s_0_13: bool = u_get_HFGRTR_EL2_Type_CONTEXTIDR_EL1(state, tracer, s_0_12);
        // D s_0_14: write-var __HFGRTR_EL2_CONTEXTIDR_EL1 <= s_0_13
        fn_state.u__HFGRTR_EL2_CONTEXTIDR_EL1 = s_0_13;
        // C s_0_15: const #102552u : u32
        let s_0_15: u32 = 102552;
        // D s_0_16: read-reg s_0_15:struct
        let s_0_16: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_15 as isize);
            tracer.read_register(s_0_15 as isize, value);
            value
        };
        // D s_0_17: call _get_HCR_EL2_Type_E2H(s_0_16)
        let s_0_17: bool = u_get_HCR_EL2_Type_E2H(state, tracer, s_0_16);
        // D s_0_18: write-var __HCR_EL2_E2H <= s_0_17
        fn_state.u__HCR_EL2_E2H = s_0_17;
        // D s_0_19: read-var __PSTATE_EL:u8
        let s_0_19: u8 = fn_state.u__PSTATE_EL;
        // D s_0_20: cast zx s_0_19 -> bv
        let s_0_20: Bits = Bits::new(s_0_19 as u128, 2u16);
        // C s_0_21: const #448u : u32
        let s_0_21: u32 = 448;
        // D s_0_22: read-reg s_0_21:u8
        let s_0_22: u8 = {
            let value = state.read_register::<u8>(s_0_21 as isize);
            tracer.read_register(s_0_21 as isize, value);
            value
        };
        // D s_0_23: cast zx s_0_22 -> bv
        let s_0_23: Bits = Bits::new(s_0_22 as u128, 2u16);
        // D s_0_24: cmp-eq s_0_20 s_0_23
        let s_0_24: bool = ((s_0_20) == (s_0_23));
        // N s_0_25: branch s_0_24 b35 b1
        if s_0_24 {
            return block_35(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b9 b2
        if s_1_5 {
            return block_9(state, tracer, fn_state);
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
        // C s_5_1: const #10048u : u32
        let s_5_1: u32 = 10048;
        // D s_5_2: read-reg s_5_1:struct
        let s_5_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_5_1 as isize);
            tracer.read_register(s_5_1 as isize, value);
            value
        };
        // D s_5_3: call __get_CONTEXTIDR_EL1(s_5_2)
        let s_5_3: ProductType5c790c8ef59cc8b2 = u__get_CONTEXTIDR_EL1(
            state,
            tracer,
            s_5_2,
        );
        // D s_5_4: write-var ga#57126 <= s_5_3
        fn_state.ga_57126 = s_5_3;
        // D s_5_5: read-var ga#57126.0:struct
        let s_5_5: u64 = fn_state.ga_57126._0;
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
        // D s_6_0: read-var __HCR_EL2_E2H:u8
        let s_6_0: bool = fn_state.u__HCR_EL2_E2H;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 1u16);
        // C s_6_2: const #1u : u8
        let s_6_2: bool = true;
        // C s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 1u16);
        // D s_6_4: cmp-eq s_6_1 s_6_3
        let s_6_4: bool = ((s_6_1) == (s_6_3));
        // N s_6_5: branch s_6_4 b8 b7
        if s_6_4 {
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
        // C s_7_0: const #64s : i64
        let s_7_0: i64 = 64;
        // C s_7_1: const #10048u : u32
        let s_7_1: u32 = 10048;
        // D s_7_2: read-reg s_7_1:struct
        let s_7_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_7_1 as isize);
            tracer.read_register(s_7_1 as isize, value);
            value
        };
        // D s_7_3: call __get_CONTEXTIDR_EL1(s_7_2)
        let s_7_3: ProductType5c790c8ef59cc8b2 = u__get_CONTEXTIDR_EL1(
            state,
            tracer,
            s_7_2,
        );
        // D s_7_4: write-var ga#57122 <= s_7_3
        fn_state.ga_57122 = s_7_3;
        // D s_7_5: read-var ga#57122.0:struct
        let s_7_5: u64 = fn_state.ga_57122._0;
        // D s_7_6: cast zx s_7_5 -> bv
        let s_7_6: Bits = Bits::new(s_7_5 as u128, 64u16);
        // D s_7_7: read-var t:i
        let s_7_7: i128 = fn_state.t;
        // D s_7_8: call X_set(s_7_7, s_7_0, s_7_6)
        let s_7_8: () = X_set(state, tracer, s_7_7, s_7_0, s_7_6);
        // N s_7_9: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #64s : i64
        let s_8_0: i64 = 64;
        // C s_8_1: const #91008u : u32
        let s_8_1: u32 = 91008;
        // D s_8_2: read-reg s_8_1:struct
        let s_8_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_8_1 as isize);
            tracer.read_register(s_8_1 as isize, value);
            value
        };
        // D s_8_3: call __get_CONTEXTIDR_EL2(s_8_2)
        let s_8_3: ProductType5c790c8ef59cc8b2 = u__get_CONTEXTIDR_EL2(
            state,
            tracer,
            s_8_2,
        );
        // D s_8_4: write-var ga#57119 <= s_8_3
        fn_state.ga_57119 = s_8_3;
        // D s_8_5: read-var ga#57119.0:struct
        let s_8_5: u64 = fn_state.ga_57119._0;
        // D s_8_6: cast zx s_8_5 -> bv
        let s_8_6: Bits = Bits::new(s_8_5 as u128, 64u16);
        // D s_8_7: read-var t:i
        let s_8_7: i128 = fn_state.t;
        // D s_8_8: call X_set(s_8_7, s_8_0, s_8_6)
        let s_8_8: () = X_set(state, tracer, s_8_7, s_8_0, s_8_6);
        // N s_8_9: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #() : ()
        let s_9_0: () = ();
        // S s_9_1: call EL2Enabled(s_9_0)
        let s_9_1: bool = EL2Enabled(state, tracer, s_9_0);
        // N s_9_2: branch s_9_1 b34 b10
        if s_9_1 {
            return block_34(state, tracer, fn_state);
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
        // D s_10_1: write-var gs#58180 <= s_10_0
        fn_state.gs_58180 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var gs#58180:u8
        let s_11_0: bool = fn_state.gs_58180;
        // N s_11_1: branch s_11_0 b33 b12
        if s_11_0 {
            return block_33(state, tracer, fn_state);
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
        // S s_12_1: call EL2Enabled(s_12_0)
        let s_12_1: bool = EL2Enabled(state, tracer, s_12_0);
        // N s_12_2: branch s_12_1 b32 b13
        if s_12_1 {
            return block_32(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#58181 <= s_13_0
        fn_state.gs_58181 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#58181:u8
        let s_14_0: bool = fn_state.gs_58181;
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
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // D s_15_1: write-var gs#58183 <= s_15_0
        fn_state.gs_58183 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#58183:u8
        let s_16_0: bool = fn_state.gs_58183;
        // N s_16_1: branch s_16_0 b27 b17
        if s_16_0 {
            return block_27(state, tracer, fn_state);
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
        // D s_17_1: write-var gs#58184 <= s_17_0
        fn_state.gs_58184 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#58184:u8
        let s_18_0: bool = fn_state.gs_58184;
        // N s_18_1: branch s_18_0 b26 b19
        if s_18_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #() : ()
        let s_19_0: () = ();
        // S s_19_1: call EL2Enabled(s_19_0)
        let s_19_1: bool = EL2Enabled(state, tracer, s_19_0);
        // N s_19_2: branch s_19_1 b25 b20
        if s_19_1 {
            return block_25(state, tracer, fn_state);
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
        // D s_20_1: write-var gs#58185 <= s_20_0
        fn_state.gs_58185 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#58185:u8
        let s_21_0: bool = fn_state.gs_58185;
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
        // C s_22_0: const #64s : i64
        let s_22_0: i64 = 64;
        // C s_22_1: const #10048u : u32
        let s_22_1: u32 = 10048;
        // D s_22_2: read-reg s_22_1:struct
        let s_22_2: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_22_1 as isize);
            tracer.read_register(s_22_1 as isize, value);
            value
        };
        // D s_22_3: call __get_CONTEXTIDR_EL1(s_22_2)
        let s_22_3: ProductType5c790c8ef59cc8b2 = u__get_CONTEXTIDR_EL1(
            state,
            tracer,
            s_22_2,
        );
        // D s_22_4: write-var ga#57114 <= s_22_3
        fn_state.ga_57114 = s_22_3;
        // D s_22_5: read-var ga#57114.0:struct
        let s_22_5: u64 = fn_state.ga_57114._0;
        // D s_22_6: cast zx s_22_5 -> bv
        let s_22_6: Bits = Bits::new(s_22_5 as u128, 64u16);
        // D s_22_7: read-var t:i
        let s_22_7: i128 = fn_state.t;
        // D s_22_8: call X_set(s_22_7, s_22_0, s_22_6)
        let s_22_8: () = X_set(state, tracer, s_22_7, s_22_0, s_22_6);
        // N s_22_9: return
        return;
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_23_0: const #264u : u12
        let s_23_0: u16 = 264;
        // C s_23_1: cast zx s_23_0 -> bv
        let s_23_1: Bits = Bits::new(s_23_0 as u128, 12u16);
        // C s_23_2: cast zx s_23_1 -> i
        let s_23_2: i128 = (s_23_1.value() as i128);
        // C s_23_3: cast reint s_23_2 -> i64
        let s_23_3: i64 = (s_23_2 as i64);
        // C s_23_4: cast zx s_23_3 -> i
        let s_23_4: i128 = (i128::try_from(s_23_3).unwrap());
        // S s_23_5: call NVMem_read(s_23_4)
        let s_23_5: u64 = NVMem_read(state, tracer, s_23_4);
        // D s_23_6: write-var ga#57113 <= s_23_5
        fn_state.ga_57113 = s_23_5;
        // N s_23_7: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var ga#57113:u64
        let s_24_0: u64 = fn_state.ga_57113;
        // D s_24_1: cast zx s_24_0 -> bv
        let s_24_1: Bits = Bits::new(s_24_0 as u128, 64u16);
        // D s_24_2: read-var t:i
        let s_24_2: i128 = fn_state.t;
        // C s_24_3: const #64s : i64
        let s_24_3: i64 = 64;
        // D s_24_4: call X_set(s_24_2, s_24_3, s_24_1)
        let s_24_4: () = X_set(state, tracer, s_24_2, s_24_3, s_24_1);
        // N s_24_5: return
        return;
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #102552u : u32
        let s_25_0: u32 = 102552;
        // D s_25_1: read-reg s_25_0:struct
        let s_25_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_25_0 as isize);
            tracer.read_register(s_25_0 as isize, value);
            value
        };
        // D s_25_2: call _get_HCR_EL2_Type_NV2(s_25_1)
        let s_25_2: bool = u_get_HCR_EL2_Type_NV2(state, tracer, s_25_1);
        // C s_25_3: const #102552u : u32
        let s_25_3: u32 = 102552;
        // D s_25_4: read-reg s_25_3:struct
        let s_25_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_25_3 as isize);
            tracer.read_register(s_25_3 as isize, value);
            value
        };
        // D s_25_5: call _get_HCR_EL2_Type_NV1(s_25_4)
        let s_25_5: bool = u_get_HCR_EL2_Type_NV1(state, tracer, s_25_4);
        // C s_25_6: const #102552u : u32
        let s_25_6: u32 = 102552;
        // D s_25_7: read-reg s_25_6:struct
        let s_25_7: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_25_6 as isize);
            tracer.read_register(s_25_6 as isize, value);
            value
        };
        // D s_25_8: call _get_HCR_EL2_Type_NV(s_25_7)
        let s_25_8: bool = u_get_HCR_EL2_Type_NV(state, tracer, s_25_7);
        // D s_25_9: cast zx s_25_5 -> bv
        let s_25_9: Bits = Bits::new(s_25_5 as u128, 1u16);
        // D s_25_10: cast zx s_25_8 -> bv
        let s_25_10: Bits = Bits::new(s_25_8 as u128, 1u16);
        // D s_25_11: cast reint s_25_9 -> u128
        let s_25_11: u128 = (s_25_9.value() as u128);
        // D s_25_12: size-of s_25_9
        let s_25_12: u16 = s_25_9.length();
        // D s_25_13: cast reint s_25_10 -> u128
        let s_25_13: u128 = (s_25_10.value() as u128);
        // D s_25_14: size-of s_25_10
        let s_25_14: u16 = s_25_10.length();
        // D s_25_15: lsl s_25_11 s_25_14
        let s_25_15: u128 = s_25_11 << s_25_14;
        // D s_25_16: or s_25_15 s_25_13
        let s_25_16: u128 = ((s_25_15) | (s_25_13));
        // D s_25_17: add s_25_12 s_25_14
        let s_25_17: u16 = (s_25_12 + s_25_14);
        // D s_25_18: create-bits s_25_16 s_25_17
        let s_25_18: Bits = Bits::new(s_25_16, s_25_17);
        // D s_25_19: cast reint s_25_18 -> u8
        let s_25_19: u8 = (s_25_18.value() as u8);
        // D s_25_20: cast zx s_25_2 -> bv
        let s_25_20: Bits = Bits::new(s_25_2 as u128, 1u16);
        // D s_25_21: cast zx s_25_19 -> bv
        let s_25_21: Bits = Bits::new(s_25_19 as u128, 2u16);
        // D s_25_22: cast reint s_25_20 -> u128
        let s_25_22: u128 = (s_25_20.value() as u128);
        // D s_25_23: size-of s_25_20
        let s_25_23: u16 = s_25_20.length();
        // D s_25_24: cast reint s_25_21 -> u128
        let s_25_24: u128 = (s_25_21.value() as u128);
        // D s_25_25: size-of s_25_21
        let s_25_25: u16 = s_25_21.length();
        // D s_25_26: lsl s_25_22 s_25_25
        let s_25_26: u128 = s_25_22 << s_25_25;
        // D s_25_27: or s_25_26 s_25_24
        let s_25_27: u128 = ((s_25_26) | (s_25_24));
        // D s_25_28: add s_25_23 s_25_25
        let s_25_28: u16 = (s_25_23 + s_25_25);
        // D s_25_29: create-bits s_25_27 s_25_28
        let s_25_29: Bits = Bits::new(s_25_27, s_25_28);
        // D s_25_30: cast reint s_25_29 -> u8
        let s_25_30: u8 = (s_25_29.value() as u8);
        // D s_25_31: cast zx s_25_30 -> bv
        let s_25_31: Bits = Bits::new(s_25_30 as u128, 3u16);
        // C s_25_32: const #7u : u8
        let s_25_32: u8 = 7;
        // C s_25_33: cast zx s_25_32 -> bv
        let s_25_33: Bits = Bits::new(s_25_32 as u128, 3u16);
        // D s_25_34: cmp-eq s_25_31 s_25_33
        let s_25_34: bool = ((s_25_31) == (s_25_33));
        // D s_25_35: write-var gs#58185 <= s_25_34
        fn_state.gs_58185 = s_25_34;
        // N s_25_36: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #24u : u8
        let s_26_0: u8 = 24;
        // C s_26_1: cast zx s_26_0 -> bv
        let s_26_1: Bits = Bits::new(s_26_0 as u128, 8u16);
        // C s_26_2: cast zx s_26_1 -> i
        let s_26_2: i128 = (s_26_1.value() as i128);
        // C s_26_3: cast reint s_26_2 -> i64
        let s_26_3: i64 = (s_26_2 as i64);
        // C s_26_4: cast zx s_26_3 -> i
        let s_26_4: i128 = (i128::try_from(s_26_3).unwrap());
        // C s_26_5: const #432u : u32
        let s_26_5: u32 = 432;
        // D s_26_6: read-reg s_26_5:u8
        let s_26_6: u8 = {
            let value = state.read_register::<u8>(s_26_5 as isize);
            tracer.read_register(s_26_5 as isize, value);
            value
        };
        // D s_26_7: call AArch64_SystemAccessTrap(s_26_6, s_26_4)
        let s_26_7: () = AArch64_SystemAccessTrap(state, tracer, s_26_6, s_26_4);
        // N s_26_8: return
        return;
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var __HFGRTR_EL2_CONTEXTIDR_EL1:u8
        let s_27_0: bool = fn_state.u__HFGRTR_EL2_CONTEXTIDR_EL1;
        // D s_27_1: cast zx s_27_0 -> bv
        let s_27_1: Bits = Bits::new(s_27_0 as u128, 1u16);
        // C s_27_2: const #1u : u8
        let s_27_2: bool = true;
        // C s_27_3: cast zx s_27_2 -> bv
        let s_27_3: Bits = Bits::new(s_27_2 as u128, 1u16);
        // D s_27_4: cmp-eq s_27_1 s_27_3
        let s_27_4: bool = ((s_27_1) == (s_27_3));
        // D s_27_5: write-var gs#58184 <= s_27_4
        fn_state.gs_58184 = s_27_4;
        // N s_27_6: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #424u : u32
        let s_28_0: u32 = 424;
        // D s_28_1: read-reg s_28_0:u8
        let s_28_1: u8 = {
            let value = state.read_register::<u8>(s_28_0 as isize);
            tracer.read_register(s_28_0 as isize, value);
            value
        };
        // C s_28_2: const #2u : u8
        let s_28_2: u8 = 2;
        // D s_28_3: cmp-lt s_28_1 s_28_2
        let s_28_3: bool = ((s_28_1) < (s_28_2));
        // D s_28_4: not s_28_3
        let s_28_4: bool = !s_28_3;
        // N s_28_5: branch s_28_4 b31 b29
        if s_28_4 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_29_0: read-var __SCR_EL3_FGTEn:u8
        let s_29_0: bool = fn_state.u__SCR_EL3_FGTEn;
        // D s_29_1: cast zx s_29_0 -> bv
        let s_29_1: Bits = Bits::new(s_29_0 as u128, 1u16);
        // C s_29_2: const #1u : u8
        let s_29_2: bool = true;
        // C s_29_3: cast zx s_29_2 -> bv
        let s_29_3: Bits = Bits::new(s_29_2 as u128, 1u16);
        // D s_29_4: cmp-eq s_29_1 s_29_3
        let s_29_4: bool = ((s_29_1) == (s_29_3));
        // D s_29_5: write-var gs#58182 <= s_29_4
        fn_state.gs_58182 = s_29_4;
        // N s_29_6: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_30_0: read-var gs#58182:u8
        let s_30_0: bool = fn_state.gs_58182;
        // D s_30_1: write-var gs#58183 <= s_30_0
        fn_state.gs_58183 = s_30_0;
        // N s_30_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #1u : u8
        let s_31_0: bool = true;
        // D s_31_1: write-var gs#58182 <= s_31_0
        fn_state.gs_58182 = s_31_0;
        // N s_31_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #146u : u32
        let s_32_0: u32 = 146;
        // S s_32_1: call IsFeatureImplemented(s_32_0)
        let s_32_1: bool = IsFeatureImplemented(state, tracer, s_32_0);
        // D s_32_2: write-var gs#58181 <= s_32_1
        fn_state.gs_58181 = s_32_1;
        // N s_32_3: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #24u : u8
        let s_33_0: u8 = 24;
        // C s_33_1: cast zx s_33_0 -> bv
        let s_33_1: Bits = Bits::new(s_33_0 as u128, 8u16);
        // C s_33_2: cast zx s_33_1 -> i
        let s_33_2: i128 = (s_33_1.value() as i128);
        // C s_33_3: cast reint s_33_2 -> i64
        let s_33_3: i64 = (s_33_2 as i64);
        // C s_33_4: cast zx s_33_3 -> i
        let s_33_4: i128 = (i128::try_from(s_33_3).unwrap());
        // C s_33_5: const #432u : u32
        let s_33_5: u32 = 432;
        // D s_33_6: read-reg s_33_5:u8
        let s_33_6: u8 = {
            let value = state.read_register::<u8>(s_33_5 as isize);
            tracer.read_register(s_33_5 as isize, value);
            value
        };
        // D s_33_7: call AArch64_SystemAccessTrap(s_33_6, s_33_4)
        let s_33_7: () = AArch64_SystemAccessTrap(state, tracer, s_33_6, s_33_4);
        // N s_33_8: return
        return;
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var __HCR_EL2_TRVM:u8
        let s_34_0: bool = fn_state.u__HCR_EL2_TRVM;
        // D s_34_1: cast zx s_34_0 -> bv
        let s_34_1: Bits = Bits::new(s_34_0 as u128, 1u16);
        // C s_34_2: const #1u : u8
        let s_34_2: bool = true;
        // C s_34_3: cast zx s_34_2 -> bv
        let s_34_3: Bits = Bits::new(s_34_2 as u128, 1u16);
        // D s_34_4: cmp-eq s_34_1 s_34_3
        let s_34_4: bool = ((s_34_1) == (s_34_3));
        // D s_34_5: write-var gs#58180 <= s_34_4
        fn_state.gs_58180 = s_34_4;
        // N s_34_6: jump b11
        return block_11(state, tracer, fn_state);
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
}
