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
use AArch32_TakeHypTrapException::*;
use AArch64_AArch32SystemAccessTrap::*;
use TTBR1_read::*;
use u_get_HSTR_Type_T2::*;
use R_set::*;
use HCR_read::*;
use u_get_HCR_EL2_Type_TRVM::*;
use u_get_HSTR_EL2_Type_T2::*;
use ELUsingAArch32::*;
use u_get_SCR_Type_NS::*;
use HSTR_read::*;
use u_get_HCR_Type_TRVM::*;
use EL2Enabled::*;
use common::*;
pub fn TTBR1_SysRegRead64_6988488afc2dce1a<T: Tracer>(
    state: &mut State,
    tracer: &T,
    el: u8,
    coproc: u8,
    opc1: u8,
    CRm: u8,
    t: i128,
    t2: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_205366: ProductType72d61775f103f7e0,
        gs_123672: bool,
        ga_205358: ProductType72d61775f103f7e0,
        ga_205369: ProductType5c790c8ef59cc8b2,
        u__HCR_TRVM: bool,
        gs_123676: bool,
        ga_205380: ProductType72d61775f103f7e0,
        u__HCR_EL2_TRVM: bool,
        gs_123651: bool,
        u__SCR_NS: bool,
        gs_123669: bool,
        gs_123671: bool,
        ga_205373: ProductType72d61775f103f7e0,
        ga_205352: ProductType5c790c8ef59cc8b2,
        u__HSTR_T2: bool,
        ga_205385: ProductType72d61775f103f7e0,
        ga_205367: ProductType5c790c8ef59cc8b2,
        ga_205351: ProductType72d61775f103f7e0,
        gs_123674: bool,
        gs_123673: bool,
        ga_205354: ProductType5c790c8ef59cc8b2,
        u__HSTR_EL2_T2: bool,
        u__PSTATE_EL: u8,
        gs_123675: bool,
        gs_123668: bool,
        gs_123670: bool,
        el: u8,
        coproc: u8,
        opc1: u8,
        CRm: u8,
        t: i128,
        t2: i128,
    }
    let fn_state = FunctionState {
        el,
        coproc,
        opc1,
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
        // C s_0_3: const #104936u : u32
        let s_0_3: u32 = 104936;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_HSTR_EL2_Type_T2(s_0_4)
        let s_0_5: bool = u_get_HSTR_EL2_Type_T2(state, tracer, s_0_4);
        // D s_0_6: write-var __HSTR_EL2_T2 <= s_0_5
        fn_state.u__HSTR_EL2_T2 = s_0_5;
        // C s_0_7: const #() : ()
        let s_0_7: () = ();
        // S s_0_8: call HSTR_read(s_0_7)
        let s_0_8: ProductType700c18a878c5601b = HSTR_read(state, tracer, s_0_7);
        // S s_0_9: call _get_HSTR_Type_T2(s_0_8)
        let s_0_9: bool = u_get_HSTR_Type_T2(state, tracer, s_0_8);
        // D s_0_10: write-var __HSTR_T2 <= s_0_9
        fn_state.u__HSTR_T2 = s_0_9;
        // C s_0_11: const #102552u : u32
        let s_0_11: u32 = 102552;
        // D s_0_12: read-reg s_0_11:struct
        let s_0_12: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: call _get_HCR_EL2_Type_TRVM(s_0_12)
        let s_0_13: bool = u_get_HCR_EL2_Type_TRVM(state, tracer, s_0_12);
        // D s_0_14: write-var __HCR_EL2_TRVM <= s_0_13
        fn_state.u__HCR_EL2_TRVM = s_0_13;
        // C s_0_15: const #() : ()
        let s_0_15: () = ();
        // S s_0_16: call HCR_read(s_0_15)
        let s_0_16: ProductType700c18a878c5601b = HCR_read(state, tracer, s_0_15);
        // S s_0_17: call _get_HCR_Type_TRVM(s_0_16)
        let s_0_17: bool = u_get_HCR_Type_TRVM(state, tracer, s_0_16);
        // D s_0_18: write-var __HCR_TRVM <= s_0_17
        fn_state.u__HCR_TRVM = s_0_17;
        // C s_0_19: const #20920u : u32
        let s_0_19: u32 = 20920;
        // D s_0_20: read-reg s_0_19:struct
        let s_0_20: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_0_19 as isize);
            tracer.read_register(s_0_19 as isize, value);
            value
        };
        // D s_0_21: call _get_SCR_Type_NS(s_0_20)
        let s_0_21: bool = u_get_SCR_Type_NS(state, tracer, s_0_20);
        // D s_0_22: write-var __SCR_NS <= s_0_21
        fn_state.u__SCR_NS = s_0_21;
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
        // N s_0_29: branch s_0_28 b52 b1
        if s_0_28 {
            return block_52(state, tracer, fn_state);
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
        // N s_1_6: branch s_1_5 b14 b2
        if s_1_5 {
            return block_14(state, tracer, fn_state);
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
        // N s_2_6: branch s_2_5 b8 b3
        if s_2_5 {
            return block_8(state, tracer, fn_state);
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
        // D s_5_0: read-var __SCR_NS:u8
        let s_5_0: bool = fn_state.u__SCR_NS;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 1u16);
        // C s_5_2: const #0u : u8
        let s_5_2: bool = false;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 1u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // N s_5_5: branch s_5_4 b7 b6
        if s_5_4 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #20024u : u32
        let s_6_0: u32 = 20024;
        // D s_6_1: read-reg s_6_0:u64
        let s_6_1: u64 = {
            let value = state.read_register::<u64>(s_6_0 as isize);
            tracer.read_register(s_6_0 as isize, value);
            value
        };
        // C s_6_2: const #32s : i
        let s_6_2: i128 = 32;
        // D s_6_3: cast zx s_6_1 -> bv
        let s_6_3: Bits = Bits::new(s_6_1 as u128, 64u16);
        // C s_6_4: const #1s : i64
        let s_6_4: i64 = 1;
        // C s_6_5: cast zx s_6_4 -> i
        let s_6_5: i128 = (i128::try_from(s_6_4).unwrap());
        // C s_6_6: const #31s : i
        let s_6_6: i128 = 31;
        // C s_6_7: add s_6_6 s_6_5
        let s_6_7: i128 = (s_6_6 + s_6_5);
        // D s_6_8: bit-extract s_6_3 s_6_2 s_6_7
        let s_6_8: Bits = (Bits::new(
            ((s_6_3) >> (s_6_2)).value(),
            u16::try_from(s_6_7).unwrap(),
        ));
        // D s_6_9: cast reint s_6_8 -> u32
        let s_6_9: u32 = (s_6_8.value() as u32);
        // C s_6_10: const #20024u : u32
        let s_6_10: u32 = 20024;
        // D s_6_11: read-reg s_6_10:u64
        let s_6_11: u64 = {
            let value = state.read_register::<u64>(s_6_10 as isize);
            tracer.read_register(s_6_10 as isize, value);
            value
        };
        // C s_6_12: const #0s : i
        let s_6_12: i128 = 0;
        // D s_6_13: cast zx s_6_11 -> bv
        let s_6_13: Bits = Bits::new(s_6_11 as u128, 64u16);
        // C s_6_14: const #1s : i64
        let s_6_14: i64 = 1;
        // C s_6_15: cast zx s_6_14 -> i
        let s_6_15: i128 = (i128::try_from(s_6_14).unwrap());
        // C s_6_16: const #31s : i
        let s_6_16: i128 = 31;
        // C s_6_17: add s_6_16 s_6_15
        let s_6_17: i128 = (s_6_16 + s_6_15);
        // D s_6_18: bit-extract s_6_13 s_6_12 s_6_17
        let s_6_18: Bits = (Bits::new(
            ((s_6_13) >> (s_6_12)).value(),
            u16::try_from(s_6_17).unwrap(),
        ));
        // D s_6_19: cast reint s_6_18 -> u32
        let s_6_19: u32 = (s_6_18.value() as u32);
        // D s_6_20: create-product struct = ["s_6_9", "s_6_19"]
        let s_6_20: ProductType72d61775f103f7e0 = ProductType72d61775f103f7e0 {
            _0: s_6_9,
            _1: s_6_19,
        };
        // D s_6_21: write-var ga#205385 <= s_6_20
        fn_state.ga_205385 = s_6_20;
        // D s_6_22: read-var ga#205385.0:struct
        let s_6_22: u32 = fn_state.ga_205385._0;
        // D s_6_23: read-var ga#205385.1:struct
        let s_6_23: u32 = fn_state.ga_205385._1;
        // D s_6_24: read-var t2:i
        let s_6_24: i128 = fn_state.t2;
        // D s_6_25: call R_set(s_6_24, s_6_22)
        let s_6_25: () = R_set(state, tracer, s_6_24, s_6_22);
        // D s_6_26: read-var t:i
        let s_6_26: i128 = fn_state.t;
        // D s_6_27: call R_set(s_6_26, s_6_23)
        let s_6_27: () = R_set(state, tracer, s_6_26, s_6_23);
        // N s_6_28: return
        return;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #19120u : u32
        let s_7_0: u32 = 19120;
        // D s_7_1: read-reg s_7_0:u64
        let s_7_1: u64 = {
            let value = state.read_register::<u64>(s_7_0 as isize);
            tracer.read_register(s_7_0 as isize, value);
            value
        };
        // C s_7_2: const #32s : i
        let s_7_2: i128 = 32;
        // D s_7_3: cast zx s_7_1 -> bv
        let s_7_3: Bits = Bits::new(s_7_1 as u128, 64u16);
        // C s_7_4: const #1s : i64
        let s_7_4: i64 = 1;
        // C s_7_5: cast zx s_7_4 -> i
        let s_7_5: i128 = (i128::try_from(s_7_4).unwrap());
        // C s_7_6: const #31s : i
        let s_7_6: i128 = 31;
        // C s_7_7: add s_7_6 s_7_5
        let s_7_7: i128 = (s_7_6 + s_7_5);
        // D s_7_8: bit-extract s_7_3 s_7_2 s_7_7
        let s_7_8: Bits = (Bits::new(
            ((s_7_3) >> (s_7_2)).value(),
            u16::try_from(s_7_7).unwrap(),
        ));
        // D s_7_9: cast reint s_7_8 -> u32
        let s_7_9: u32 = (s_7_8.value() as u32);
        // C s_7_10: const #19120u : u32
        let s_7_10: u32 = 19120;
        // D s_7_11: read-reg s_7_10:u64
        let s_7_11: u64 = {
            let value = state.read_register::<u64>(s_7_10 as isize);
            tracer.read_register(s_7_10 as isize, value);
            value
        };
        // C s_7_12: const #0s : i
        let s_7_12: i128 = 0;
        // D s_7_13: cast zx s_7_11 -> bv
        let s_7_13: Bits = Bits::new(s_7_11 as u128, 64u16);
        // C s_7_14: const #1s : i64
        let s_7_14: i64 = 1;
        // C s_7_15: cast zx s_7_14 -> i
        let s_7_15: i128 = (i128::try_from(s_7_14).unwrap());
        // C s_7_16: const #31s : i
        let s_7_16: i128 = 31;
        // C s_7_17: add s_7_16 s_7_15
        let s_7_17: i128 = (s_7_16 + s_7_15);
        // D s_7_18: bit-extract s_7_13 s_7_12 s_7_17
        let s_7_18: Bits = (Bits::new(
            ((s_7_13) >> (s_7_12)).value(),
            u16::try_from(s_7_17).unwrap(),
        ));
        // D s_7_19: cast reint s_7_18 -> u32
        let s_7_19: u32 = (s_7_18.value() as u32);
        // D s_7_20: create-product struct = ["s_7_9", "s_7_19"]
        let s_7_20: ProductType72d61775f103f7e0 = ProductType72d61775f103f7e0 {
            _0: s_7_9,
            _1: s_7_19,
        };
        // D s_7_21: write-var ga#205380 <= s_7_20
        fn_state.ga_205380 = s_7_20;
        // D s_7_22: read-var ga#205380.0:struct
        let s_7_22: u32 = fn_state.ga_205380._0;
        // D s_7_23: read-var ga#205380.1:struct
        let s_7_23: u32 = fn_state.ga_205380._1;
        // D s_7_24: read-var t2:i
        let s_7_24: i128 = fn_state.t2;
        // D s_7_25: call R_set(s_7_24, s_7_22)
        let s_7_25: () = R_set(state, tracer, s_7_24, s_7_22);
        // D s_7_26: read-var t:i
        let s_7_26: i128 = fn_state.t;
        // D s_7_27: call R_set(s_7_26, s_7_23)
        let s_7_27: () = R_set(state, tracer, s_7_26, s_7_23);
        // N s_7_28: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #424u : u32
        let s_8_0: u32 = 424;
        // D s_8_1: read-reg s_8_0:u8
        let s_8_1: u8 = {
            let value = state.read_register::<u8>(s_8_0 as isize);
            tracer.read_register(s_8_0 as isize, value);
            value
        };
        // C s_8_2: const #2u : u8
        let s_8_2: u8 = 2;
        // D s_8_3: cmp-lt s_8_1 s_8_2
        let s_8_3: bool = ((s_8_1) < (s_8_2));
        // N s_8_4: branch s_8_3 b13 b9
        if s_8_3 {
            return block_13(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#123651 <= s_9_0
        fn_state.gs_123651 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#123651:u8
        let s_10_0: bool = fn_state.gs_123651;
        // N s_10_1: branch s_10_0 b12 b11
        if s_10_0 {
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
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call TTBR1_read(s_11_0)
        let s_11_1: ProductType5c790c8ef59cc8b2 = TTBR1_read(state, tracer, s_11_0);
        // D s_11_2: write-var ga#205367 <= s_11_1
        fn_state.ga_205367 = s_11_1;
        // D s_11_3: read-var ga#205367.0:struct
        let s_11_3: u64 = fn_state.ga_205367._0;
        // C s_11_4: const #32s : i
        let s_11_4: i128 = 32;
        // D s_11_5: cast zx s_11_3 -> bv
        let s_11_5: Bits = Bits::new(s_11_3 as u128, 64u16);
        // C s_11_6: const #1s : i64
        let s_11_6: i64 = 1;
        // C s_11_7: cast zx s_11_6 -> i
        let s_11_7: i128 = (i128::try_from(s_11_6).unwrap());
        // C s_11_8: const #31s : i
        let s_11_8: i128 = 31;
        // C s_11_9: add s_11_8 s_11_7
        let s_11_9: i128 = (s_11_8 + s_11_7);
        // D s_11_10: bit-extract s_11_5 s_11_4 s_11_9
        let s_11_10: Bits = (Bits::new(
            ((s_11_5) >> (s_11_4)).value(),
            u16::try_from(s_11_9).unwrap(),
        ));
        // D s_11_11: cast reint s_11_10 -> u32
        let s_11_11: u32 = (s_11_10.value() as u32);
        // C s_11_12: const #() : ()
        let s_11_12: () = ();
        // S s_11_13: call TTBR1_read(s_11_12)
        let s_11_13: ProductType5c790c8ef59cc8b2 = TTBR1_read(state, tracer, s_11_12);
        // D s_11_14: write-var ga#205369 <= s_11_13
        fn_state.ga_205369 = s_11_13;
        // D s_11_15: read-var ga#205369.0:struct
        let s_11_15: u64 = fn_state.ga_205369._0;
        // C s_11_16: const #0s : i
        let s_11_16: i128 = 0;
        // D s_11_17: cast zx s_11_15 -> bv
        let s_11_17: Bits = Bits::new(s_11_15 as u128, 64u16);
        // C s_11_18: const #1s : i64
        let s_11_18: i64 = 1;
        // C s_11_19: cast zx s_11_18 -> i
        let s_11_19: i128 = (i128::try_from(s_11_18).unwrap());
        // C s_11_20: const #31s : i
        let s_11_20: i128 = 31;
        // C s_11_21: add s_11_20 s_11_19
        let s_11_21: i128 = (s_11_20 + s_11_19);
        // D s_11_22: bit-extract s_11_17 s_11_16 s_11_21
        let s_11_22: Bits = (Bits::new(
            ((s_11_17) >> (s_11_16)).value(),
            u16::try_from(s_11_21).unwrap(),
        ));
        // D s_11_23: cast reint s_11_22 -> u32
        let s_11_23: u32 = (s_11_22.value() as u32);
        // D s_11_24: create-product struct = ["s_11_11", "s_11_23"]
        let s_11_24: ProductType72d61775f103f7e0 = ProductType72d61775f103f7e0 {
            _0: s_11_11,
            _1: s_11_23,
        };
        // D s_11_25: write-var ga#205373 <= s_11_24
        fn_state.ga_205373 = s_11_24;
        // D s_11_26: read-var ga#205373.0:struct
        let s_11_26: u32 = fn_state.ga_205373._0;
        // D s_11_27: read-var ga#205373.1:struct
        let s_11_27: u32 = fn_state.ga_205373._1;
        // D s_11_28: read-var t2:i
        let s_11_28: i128 = fn_state.t2;
        // D s_11_29: call R_set(s_11_28, s_11_26)
        let s_11_29: () = R_set(state, tracer, s_11_28, s_11_26);
        // D s_11_30: read-var t:i
        let s_11_30: i128 = fn_state.t;
        // D s_11_31: call R_set(s_11_30, s_11_27)
        let s_11_31: () = R_set(state, tracer, s_11_30, s_11_27);
        // N s_11_32: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #20024u : u32
        let s_12_0: u32 = 20024;
        // D s_12_1: read-reg s_12_0:u64
        let s_12_1: u64 = {
            let value = state.read_register::<u64>(s_12_0 as isize);
            tracer.read_register(s_12_0 as isize, value);
            value
        };
        // C s_12_2: const #32s : i
        let s_12_2: i128 = 32;
        // D s_12_3: cast zx s_12_1 -> bv
        let s_12_3: Bits = Bits::new(s_12_1 as u128, 64u16);
        // C s_12_4: const #1s : i64
        let s_12_4: i64 = 1;
        // C s_12_5: cast zx s_12_4 -> i
        let s_12_5: i128 = (i128::try_from(s_12_4).unwrap());
        // C s_12_6: const #31s : i
        let s_12_6: i128 = 31;
        // C s_12_7: add s_12_6 s_12_5
        let s_12_7: i128 = (s_12_6 + s_12_5);
        // D s_12_8: bit-extract s_12_3 s_12_2 s_12_7
        let s_12_8: Bits = (Bits::new(
            ((s_12_3) >> (s_12_2)).value(),
            u16::try_from(s_12_7).unwrap(),
        ));
        // D s_12_9: cast reint s_12_8 -> u32
        let s_12_9: u32 = (s_12_8.value() as u32);
        // C s_12_10: const #20024u : u32
        let s_12_10: u32 = 20024;
        // D s_12_11: read-reg s_12_10:u64
        let s_12_11: u64 = {
            let value = state.read_register::<u64>(s_12_10 as isize);
            tracer.read_register(s_12_10 as isize, value);
            value
        };
        // C s_12_12: const #0s : i
        let s_12_12: i128 = 0;
        // D s_12_13: cast zx s_12_11 -> bv
        let s_12_13: Bits = Bits::new(s_12_11 as u128, 64u16);
        // C s_12_14: const #1s : i64
        let s_12_14: i64 = 1;
        // C s_12_15: cast zx s_12_14 -> i
        let s_12_15: i128 = (i128::try_from(s_12_14).unwrap());
        // C s_12_16: const #31s : i
        let s_12_16: i128 = 31;
        // C s_12_17: add s_12_16 s_12_15
        let s_12_17: i128 = (s_12_16 + s_12_15);
        // D s_12_18: bit-extract s_12_13 s_12_12 s_12_17
        let s_12_18: Bits = (Bits::new(
            ((s_12_13) >> (s_12_12)).value(),
            u16::try_from(s_12_17).unwrap(),
        ));
        // D s_12_19: cast reint s_12_18 -> u32
        let s_12_19: u32 = (s_12_18.value() as u32);
        // D s_12_20: create-product struct = ["s_12_9", "s_12_19"]
        let s_12_20: ProductType72d61775f103f7e0 = ProductType72d61775f103f7e0 {
            _0: s_12_9,
            _1: s_12_19,
        };
        // D s_12_21: write-var ga#205366 <= s_12_20
        fn_state.ga_205366 = s_12_20;
        // D s_12_22: read-var ga#205366.0:struct
        let s_12_22: u32 = fn_state.ga_205366._0;
        // D s_12_23: read-var ga#205366.1:struct
        let s_12_23: u32 = fn_state.ga_205366._1;
        // D s_12_24: read-var t2:i
        let s_12_24: i128 = fn_state.t2;
        // D s_12_25: call R_set(s_12_24, s_12_22)
        let s_12_25: () = R_set(state, tracer, s_12_24, s_12_22);
        // D s_12_26: read-var t:i
        let s_12_26: i128 = fn_state.t;
        // D s_12_27: call R_set(s_12_26, s_12_23)
        let s_12_27: () = R_set(state, tracer, s_12_26, s_12_23);
        // N s_12_28: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #424u : u32
        let s_13_0: u32 = 424;
        // D s_13_1: read-reg s_13_0:u8
        let s_13_1: u8 = {
            let value = state.read_register::<u8>(s_13_0 as isize);
            tracer.read_register(s_13_0 as isize, value);
            value
        };
        // D s_13_2: call ELUsingAArch32(s_13_1)
        let s_13_2: bool = ELUsingAArch32(state, tracer, s_13_1);
        // D s_13_3: write-var gs#123651 <= s_13_2
        fn_state.gs_123651 = s_13_2;
        // N s_13_4: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call EL2Enabled(s_14_0)
        let s_14_1: bool = EL2Enabled(state, tracer, s_14_0);
        // N s_14_2: branch s_14_1 b51 b15
        if s_14_1 {
            return block_51(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#123668 <= s_15_0
        fn_state.gs_123668 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#123668:u8
        let s_16_0: bool = fn_state.gs_123668;
        // N s_16_1: branch s_16_0 b50 b17
        if s_16_0 {
            return block_50(state, tracer, fn_state);
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
        // D s_17_1: write-var gs#123669 <= s_17_0
        fn_state.gs_123669 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#123669:u8
        let s_18_0: bool = fn_state.gs_123669;
        // N s_18_1: branch s_18_0 b49 b19
        if s_18_0 {
            return block_49(state, tracer, fn_state);
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
        // N s_19_2: branch s_19_1 b48 b20
        if s_19_1 {
            return block_48(state, tracer, fn_state);
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
        // D s_20_1: write-var gs#123670 <= s_20_0
        fn_state.gs_123670 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#123670:u8
        let s_21_0: bool = fn_state.gs_123670;
        // N s_21_1: branch s_21_0 b47 b22
        if s_21_0 {
            return block_47(state, tracer, fn_state);
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
        // D s_22_1: write-var gs#123671 <= s_22_0
        fn_state.gs_123671 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#123671:u8
        let s_23_0: bool = fn_state.gs_123671;
        // N s_23_1: branch s_23_0 b46 b24
        if s_23_0 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_24_0: const #() : ()
        let s_24_0: () = ();
        // S s_24_1: call EL2Enabled(s_24_0)
        let s_24_1: bool = EL2Enabled(state, tracer, s_24_0);
        // N s_24_2: branch s_24_1 b45 b25
        if s_24_1 {
            return block_45(state, tracer, fn_state);
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
        // D s_25_1: write-var gs#123672 <= s_25_0
        fn_state.gs_123672 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#123672:u8
        let s_26_0: bool = fn_state.gs_123672;
        // N s_26_1: branch s_26_0 b44 b27
        if s_26_0 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #0u : u8
        let s_27_0: bool = false;
        // D s_27_1: write-var gs#123673 <= s_27_0
        fn_state.gs_123673 = s_27_0;
        // N s_27_2: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_28_0: read-var gs#123673:u8
        let s_28_0: bool = fn_state.gs_123673;
        // N s_28_1: branch s_28_0 b43 b29
        if s_28_0 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #() : ()
        let s_29_0: () = ();
        // S s_29_1: call EL2Enabled(s_29_0)
        let s_29_1: bool = EL2Enabled(state, tracer, s_29_0);
        // N s_29_2: branch s_29_1 b42 b30
        if s_29_1 {
            return block_42(state, tracer, fn_state);
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
        // D s_30_1: write-var gs#123674 <= s_30_0
        fn_state.gs_123674 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var gs#123674:u8
        let s_31_0: bool = fn_state.gs_123674;
        // N s_31_1: branch s_31_0 b41 b32
        if s_31_0 {
            return block_41(state, tracer, fn_state);
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
        // D s_32_1: write-var gs#123675 <= s_32_0
        fn_state.gs_123675 = s_32_0;
        // N s_32_2: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var gs#123675:u8
        let s_33_0: bool = fn_state.gs_123675;
        // N s_33_1: branch s_33_0 b40 b34
        if s_33_0 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #424u : u32
        let s_34_0: u32 = 424;
        // D s_34_1: read-reg s_34_0:u8
        let s_34_1: u8 = {
            let value = state.read_register::<u8>(s_34_0 as isize);
            tracer.read_register(s_34_0 as isize, value);
            value
        };
        // C s_34_2: const #2u : u8
        let s_34_2: u8 = 2;
        // D s_34_3: cmp-lt s_34_1 s_34_2
        let s_34_3: bool = ((s_34_1) < (s_34_2));
        // N s_34_4: branch s_34_3 b39 b35
        if s_34_3 {
            return block_39(state, tracer, fn_state);
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
        // D s_35_1: write-var gs#123676 <= s_35_0
        fn_state.gs_123676 = s_35_0;
        // N s_35_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var gs#123676:u8
        let s_36_0: bool = fn_state.gs_123676;
        // N s_36_1: branch s_36_0 b38 b37
        if s_36_0 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #() : ()
        let s_37_0: () = ();
        // S s_37_1: call TTBR1_read(s_37_0)
        let s_37_1: ProductType5c790c8ef59cc8b2 = TTBR1_read(state, tracer, s_37_0);
        // D s_37_2: write-var ga#205352 <= s_37_1
        fn_state.ga_205352 = s_37_1;
        // D s_37_3: read-var ga#205352.0:struct
        let s_37_3: u64 = fn_state.ga_205352._0;
        // C s_37_4: const #32s : i
        let s_37_4: i128 = 32;
        // D s_37_5: cast zx s_37_3 -> bv
        let s_37_5: Bits = Bits::new(s_37_3 as u128, 64u16);
        // C s_37_6: const #1s : i64
        let s_37_6: i64 = 1;
        // C s_37_7: cast zx s_37_6 -> i
        let s_37_7: i128 = (i128::try_from(s_37_6).unwrap());
        // C s_37_8: const #31s : i
        let s_37_8: i128 = 31;
        // C s_37_9: add s_37_8 s_37_7
        let s_37_9: i128 = (s_37_8 + s_37_7);
        // D s_37_10: bit-extract s_37_5 s_37_4 s_37_9
        let s_37_10: Bits = (Bits::new(
            ((s_37_5) >> (s_37_4)).value(),
            u16::try_from(s_37_9).unwrap(),
        ));
        // D s_37_11: cast reint s_37_10 -> u32
        let s_37_11: u32 = (s_37_10.value() as u32);
        // C s_37_12: const #() : ()
        let s_37_12: () = ();
        // S s_37_13: call TTBR1_read(s_37_12)
        let s_37_13: ProductType5c790c8ef59cc8b2 = TTBR1_read(state, tracer, s_37_12);
        // D s_37_14: write-var ga#205354 <= s_37_13
        fn_state.ga_205354 = s_37_13;
        // D s_37_15: read-var ga#205354.0:struct
        let s_37_15: u64 = fn_state.ga_205354._0;
        // C s_37_16: const #0s : i
        let s_37_16: i128 = 0;
        // D s_37_17: cast zx s_37_15 -> bv
        let s_37_17: Bits = Bits::new(s_37_15 as u128, 64u16);
        // C s_37_18: const #1s : i64
        let s_37_18: i64 = 1;
        // C s_37_19: cast zx s_37_18 -> i
        let s_37_19: i128 = (i128::try_from(s_37_18).unwrap());
        // C s_37_20: const #31s : i
        let s_37_20: i128 = 31;
        // C s_37_21: add s_37_20 s_37_19
        let s_37_21: i128 = (s_37_20 + s_37_19);
        // D s_37_22: bit-extract s_37_17 s_37_16 s_37_21
        let s_37_22: Bits = (Bits::new(
            ((s_37_17) >> (s_37_16)).value(),
            u16::try_from(s_37_21).unwrap(),
        ));
        // D s_37_23: cast reint s_37_22 -> u32
        let s_37_23: u32 = (s_37_22.value() as u32);
        // D s_37_24: create-product struct = ["s_37_11", "s_37_23"]
        let s_37_24: ProductType72d61775f103f7e0 = ProductType72d61775f103f7e0 {
            _0: s_37_11,
            _1: s_37_23,
        };
        // D s_37_25: write-var ga#205358 <= s_37_24
        fn_state.ga_205358 = s_37_24;
        // D s_37_26: read-var ga#205358.0:struct
        let s_37_26: u32 = fn_state.ga_205358._0;
        // D s_37_27: read-var ga#205358.1:struct
        let s_37_27: u32 = fn_state.ga_205358._1;
        // D s_37_28: read-var t2:i
        let s_37_28: i128 = fn_state.t2;
        // D s_37_29: call R_set(s_37_28, s_37_26)
        let s_37_29: () = R_set(state, tracer, s_37_28, s_37_26);
        // D s_37_30: read-var t:i
        let s_37_30: i128 = fn_state.t;
        // D s_37_31: call R_set(s_37_30, s_37_27)
        let s_37_31: () = R_set(state, tracer, s_37_30, s_37_27);
        // N s_37_32: return
        return;
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #20024u : u32
        let s_38_0: u32 = 20024;
        // D s_38_1: read-reg s_38_0:u64
        let s_38_1: u64 = {
            let value = state.read_register::<u64>(s_38_0 as isize);
            tracer.read_register(s_38_0 as isize, value);
            value
        };
        // C s_38_2: const #32s : i
        let s_38_2: i128 = 32;
        // D s_38_3: cast zx s_38_1 -> bv
        let s_38_3: Bits = Bits::new(s_38_1 as u128, 64u16);
        // C s_38_4: const #1s : i64
        let s_38_4: i64 = 1;
        // C s_38_5: cast zx s_38_4 -> i
        let s_38_5: i128 = (i128::try_from(s_38_4).unwrap());
        // C s_38_6: const #31s : i
        let s_38_6: i128 = 31;
        // C s_38_7: add s_38_6 s_38_5
        let s_38_7: i128 = (s_38_6 + s_38_5);
        // D s_38_8: bit-extract s_38_3 s_38_2 s_38_7
        let s_38_8: Bits = (Bits::new(
            ((s_38_3) >> (s_38_2)).value(),
            u16::try_from(s_38_7).unwrap(),
        ));
        // D s_38_9: cast reint s_38_8 -> u32
        let s_38_9: u32 = (s_38_8.value() as u32);
        // C s_38_10: const #20024u : u32
        let s_38_10: u32 = 20024;
        // D s_38_11: read-reg s_38_10:u64
        let s_38_11: u64 = {
            let value = state.read_register::<u64>(s_38_10 as isize);
            tracer.read_register(s_38_10 as isize, value);
            value
        };
        // C s_38_12: const #0s : i
        let s_38_12: i128 = 0;
        // D s_38_13: cast zx s_38_11 -> bv
        let s_38_13: Bits = Bits::new(s_38_11 as u128, 64u16);
        // C s_38_14: const #1s : i64
        let s_38_14: i64 = 1;
        // C s_38_15: cast zx s_38_14 -> i
        let s_38_15: i128 = (i128::try_from(s_38_14).unwrap());
        // C s_38_16: const #31s : i
        let s_38_16: i128 = 31;
        // C s_38_17: add s_38_16 s_38_15
        let s_38_17: i128 = (s_38_16 + s_38_15);
        // D s_38_18: bit-extract s_38_13 s_38_12 s_38_17
        let s_38_18: Bits = (Bits::new(
            ((s_38_13) >> (s_38_12)).value(),
            u16::try_from(s_38_17).unwrap(),
        ));
        // D s_38_19: cast reint s_38_18 -> u32
        let s_38_19: u32 = (s_38_18.value() as u32);
        // D s_38_20: create-product struct = ["s_38_9", "s_38_19"]
        let s_38_20: ProductType72d61775f103f7e0 = ProductType72d61775f103f7e0 {
            _0: s_38_9,
            _1: s_38_19,
        };
        // D s_38_21: write-var ga#205351 <= s_38_20
        fn_state.ga_205351 = s_38_20;
        // D s_38_22: read-var ga#205351.0:struct
        let s_38_22: u32 = fn_state.ga_205351._0;
        // D s_38_23: read-var ga#205351.1:struct
        let s_38_23: u32 = fn_state.ga_205351._1;
        // D s_38_24: read-var t2:i
        let s_38_24: i128 = fn_state.t2;
        // D s_38_25: call R_set(s_38_24, s_38_22)
        let s_38_25: () = R_set(state, tracer, s_38_24, s_38_22);
        // D s_38_26: read-var t:i
        let s_38_26: i128 = fn_state.t;
        // D s_38_27: call R_set(s_38_26, s_38_23)
        let s_38_27: () = R_set(state, tracer, s_38_26, s_38_23);
        // N s_38_28: return
        return;
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #424u : u32
        let s_39_0: u32 = 424;
        // D s_39_1: read-reg s_39_0:u8
        let s_39_1: u8 = {
            let value = state.read_register::<u8>(s_39_0 as isize);
            tracer.read_register(s_39_0 as isize, value);
            value
        };
        // D s_39_2: call ELUsingAArch32(s_39_1)
        let s_39_2: bool = ELUsingAArch32(state, tracer, s_39_1);
        // D s_39_3: write-var gs#123676 <= s_39_2
        fn_state.gs_123676 = s_39_2;
        // N s_39_4: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #4u : u8
        let s_40_0: u8 = 4;
        // C s_40_1: cast zx s_40_0 -> bv
        let s_40_1: Bits = Bits::new(s_40_0 as u128, 8u16);
        // C s_40_2: cast zx s_40_1 -> i
        let s_40_2: i128 = (s_40_1.value() as i128);
        // C s_40_3: cast reint s_40_2 -> i64
        let s_40_3: i64 = (s_40_2 as i64);
        // C s_40_4: cast zx s_40_3 -> i
        let s_40_4: i128 = (i128::try_from(s_40_3).unwrap());
        // S s_40_5: call AArch32_TakeHypTrapException(s_40_4)
        let s_40_5: () = AArch32_TakeHypTrapException(state, tracer, s_40_4);
        // N s_40_6: return
        return;
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_41_0: read-var __HCR_TRVM:u8
        let s_41_0: bool = fn_state.u__HCR_TRVM;
        // D s_41_1: cast zx s_41_0 -> bv
        let s_41_1: Bits = Bits::new(s_41_0 as u128, 1u16);
        // C s_41_2: const #1u : u8
        let s_41_2: bool = true;
        // C s_41_3: cast zx s_41_2 -> bv
        let s_41_3: Bits = Bits::new(s_41_2 as u128, 1u16);
        // D s_41_4: cmp-eq s_41_1 s_41_3
        let s_41_4: bool = ((s_41_1) == (s_41_3));
        // D s_41_5: write-var gs#123675 <= s_41_4
        fn_state.gs_123675 = s_41_4;
        // N s_41_6: jump b33
        return block_33(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_42_0: const #432u : u32
        let s_42_0: u32 = 432;
        // D s_42_1: read-reg s_42_0:u8
        let s_42_1: u8 = {
            let value = state.read_register::<u8>(s_42_0 as isize);
            tracer.read_register(s_42_0 as isize, value);
            value
        };
        // D s_42_2: call ELUsingAArch32(s_42_1)
        let s_42_2: bool = ELUsingAArch32(state, tracer, s_42_1);
        // D s_42_3: write-var gs#123674 <= s_42_2
        fn_state.gs_123674 = s_42_2;
        // N s_42_4: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_43_0: const #4u : u8
        let s_43_0: u8 = 4;
        // C s_43_1: cast zx s_43_0 -> bv
        let s_43_1: Bits = Bits::new(s_43_0 as u128, 8u16);
        // C s_43_2: cast zx s_43_1 -> i
        let s_43_2: i128 = (s_43_1.value() as i128);
        // C s_43_3: cast reint s_43_2 -> i64
        let s_43_3: i64 = (s_43_2 as i64);
        // C s_43_4: cast zx s_43_3 -> i
        let s_43_4: i128 = (i128::try_from(s_43_3).unwrap());
        // C s_43_5: const #432u : u32
        let s_43_5: u32 = 432;
        // D s_43_6: read-reg s_43_5:u8
        let s_43_6: u8 = {
            let value = state.read_register::<u8>(s_43_5 as isize);
            tracer.read_register(s_43_5 as isize, value);
            value
        };
        // D s_43_7: call AArch64_AArch32SystemAccessTrap(s_43_6, s_43_4)
        let s_43_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_43_6, s_43_4);
        // N s_43_8: return
        return;
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_44_0: read-var __HCR_EL2_TRVM:u8
        let s_44_0: bool = fn_state.u__HCR_EL2_TRVM;
        // D s_44_1: cast zx s_44_0 -> bv
        let s_44_1: Bits = Bits::new(s_44_0 as u128, 1u16);
        // C s_44_2: const #1u : u8
        let s_44_2: bool = true;
        // C s_44_3: cast zx s_44_2 -> bv
        let s_44_3: Bits = Bits::new(s_44_2 as u128, 1u16);
        // D s_44_4: cmp-eq s_44_1 s_44_3
        let s_44_4: bool = ((s_44_1) == (s_44_3));
        // D s_44_5: write-var gs#123673 <= s_44_4
        fn_state.gs_123673 = s_44_4;
        // N s_44_6: jump b28
        return block_28(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_45_0: const #432u : u32
        let s_45_0: u32 = 432;
        // D s_45_1: read-reg s_45_0:u8
        let s_45_1: u8 = {
            let value = state.read_register::<u8>(s_45_0 as isize);
            tracer.read_register(s_45_0 as isize, value);
            value
        };
        // D s_45_2: call ELUsingAArch32(s_45_1)
        let s_45_2: bool = ELUsingAArch32(state, tracer, s_45_1);
        // D s_45_3: not s_45_2
        let s_45_3: bool = !s_45_2;
        // D s_45_4: write-var gs#123672 <= s_45_3
        fn_state.gs_123672 = s_45_3;
        // N s_45_5: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_46_0: const #4u : u8
        let s_46_0: u8 = 4;
        // C s_46_1: cast zx s_46_0 -> bv
        let s_46_1: Bits = Bits::new(s_46_0 as u128, 8u16);
        // C s_46_2: cast zx s_46_1 -> i
        let s_46_2: i128 = (s_46_1.value() as i128);
        // C s_46_3: cast reint s_46_2 -> i64
        let s_46_3: i64 = (s_46_2 as i64);
        // C s_46_4: cast zx s_46_3 -> i
        let s_46_4: i128 = (i128::try_from(s_46_3).unwrap());
        // S s_46_5: call AArch32_TakeHypTrapException(s_46_4)
        let s_46_5: () = AArch32_TakeHypTrapException(state, tracer, s_46_4);
        // N s_46_6: return
        return;
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_47_0: read-var __HSTR_T2:u8
        let s_47_0: bool = fn_state.u__HSTR_T2;
        // D s_47_1: cast zx s_47_0 -> bv
        let s_47_1: Bits = Bits::new(s_47_0 as u128, 1u16);
        // C s_47_2: const #1u : u8
        let s_47_2: bool = true;
        // C s_47_3: cast zx s_47_2 -> bv
        let s_47_3: Bits = Bits::new(s_47_2 as u128, 1u16);
        // D s_47_4: cmp-eq s_47_1 s_47_3
        let s_47_4: bool = ((s_47_1) == (s_47_3));
        // D s_47_5: write-var gs#123671 <= s_47_4
        fn_state.gs_123671 = s_47_4;
        // N s_47_6: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_48_0: const #432u : u32
        let s_48_0: u32 = 432;
        // D s_48_1: read-reg s_48_0:u8
        let s_48_1: u8 = {
            let value = state.read_register::<u8>(s_48_0 as isize);
            tracer.read_register(s_48_0 as isize, value);
            value
        };
        // D s_48_2: call ELUsingAArch32(s_48_1)
        let s_48_2: bool = ELUsingAArch32(state, tracer, s_48_1);
        // D s_48_3: write-var gs#123670 <= s_48_2
        fn_state.gs_123670 = s_48_2;
        // N s_48_4: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_49_0: const #4u : u8
        let s_49_0: u8 = 4;
        // C s_49_1: cast zx s_49_0 -> bv
        let s_49_1: Bits = Bits::new(s_49_0 as u128, 8u16);
        // C s_49_2: cast zx s_49_1 -> i
        let s_49_2: i128 = (s_49_1.value() as i128);
        // C s_49_3: cast reint s_49_2 -> i64
        let s_49_3: i64 = (s_49_2 as i64);
        // C s_49_4: cast zx s_49_3 -> i
        let s_49_4: i128 = (i128::try_from(s_49_3).unwrap());
        // C s_49_5: const #432u : u32
        let s_49_5: u32 = 432;
        // D s_49_6: read-reg s_49_5:u8
        let s_49_6: u8 = {
            let value = state.read_register::<u8>(s_49_5 as isize);
            tracer.read_register(s_49_5 as isize, value);
            value
        };
        // D s_49_7: call AArch64_AArch32SystemAccessTrap(s_49_6, s_49_4)
        let s_49_7: () = AArch64_AArch32SystemAccessTrap(state, tracer, s_49_6, s_49_4);
        // N s_49_8: return
        return;
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_50_0: read-var __HSTR_EL2_T2:u8
        let s_50_0: bool = fn_state.u__HSTR_EL2_T2;
        // D s_50_1: cast zx s_50_0 -> bv
        let s_50_1: Bits = Bits::new(s_50_0 as u128, 1u16);
        // C s_50_2: const #1u : u8
        let s_50_2: bool = true;
        // C s_50_3: cast zx s_50_2 -> bv
        let s_50_3: Bits = Bits::new(s_50_2 as u128, 1u16);
        // D s_50_4: cmp-eq s_50_1 s_50_3
        let s_50_4: bool = ((s_50_1) == (s_50_3));
        // D s_50_5: write-var gs#123669 <= s_50_4
        fn_state.gs_123669 = s_50_4;
        // N s_50_6: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_51_0: const #432u : u32
        let s_51_0: u32 = 432;
        // D s_51_1: read-reg s_51_0:u8
        let s_51_1: u8 = {
            let value = state.read_register::<u8>(s_51_0 as isize);
            tracer.read_register(s_51_0 as isize, value);
            value
        };
        // D s_51_2: call ELUsingAArch32(s_51_1)
        let s_51_2: bool = ELUsingAArch32(state, tracer, s_51_1);
        // D s_51_3: not s_51_2
        let s_51_3: bool = !s_51_2;
        // D s_51_4: write-var gs#123668 <= s_51_3
        fn_state.gs_123668 = s_51_3;
        // N s_51_5: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_52_0: panic
        panic!("{:?}", ());
        // N s_52_1: return
        return;
    }
}
