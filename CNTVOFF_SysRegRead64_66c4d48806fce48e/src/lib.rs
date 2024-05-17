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
use CNTVOFF_read::*;
use u_get_SCR_Type_NS::*;
use R_set::*;
use common::*;
pub fn CNTVOFF_SysRegRead64_66c4d48806fce48e<T: Tracer>(
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
        u__PSTATE_EL: u8,
        ga_204671: ProductType72d61775f103f7e0,
        u__SCR_NS: bool,
        ga_204679: ProductType72d61775f103f7e0,
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
        // C s_0_3: const #20920u : u32
        let s_0_3: u32 = 20920;
        // D s_0_4: read-reg s_0_3:struct
        let s_0_4: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: call _get_SCR_Type_NS(s_0_4)
        let s_0_5: bool = u_get_SCR_Type_NS(state, tracer, s_0_4);
        // D s_0_6: write-var __SCR_NS <= s_0_5
        fn_state.u__SCR_NS = s_0_5;
        // D s_0_7: read-var __PSTATE_EL:u8
        let s_0_7: u8 = fn_state.u__PSTATE_EL;
        // D s_0_8: cast zx s_0_7 -> bv
        let s_0_8: Bits = Bits::new(s_0_7 as u128, 2u16);
        // C s_0_9: const #448u : u32
        let s_0_9: u32 = 448;
        // D s_0_10: read-reg s_0_9:u8
        let s_0_10: u8 = {
            let value = state.read_register::<u8>(s_0_9 as isize);
            tracer.read_register(s_0_9 as isize, value);
            value
        };
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 2u16);
        // D s_0_12: cmp-eq s_0_8 s_0_11
        let s_0_12: bool = ((s_0_8) == (s_0_11));
        // N s_0_13: branch s_0_12 b10 b1
        if s_0_12 {
            return block_10(state, tracer, fn_state);
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
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call CNTVOFF_read(s_6_0)
        let s_6_1: u64 = CNTVOFF_read(state, tracer, s_6_0);
        // C s_6_2: const #32s : i
        let s_6_2: i128 = 32;
        // S s_6_3: cast zx s_6_1 -> bv
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
        // C s_6_10: const #() : ()
        let s_6_10: () = ();
        // S s_6_11: call CNTVOFF_read(s_6_10)
        let s_6_11: u64 = CNTVOFF_read(state, tracer, s_6_10);
        // C s_6_12: const #0s : i
        let s_6_12: i128 = 0;
        // S s_6_13: cast zx s_6_11 -> bv
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
        // D s_6_21: write-var ga#204679 <= s_6_20
        fn_state.ga_204679 = s_6_20;
        // D s_6_22: read-var ga#204679.0:struct
        let s_6_22: u32 = fn_state.ga_204679._0;
        // D s_6_23: read-var ga#204679.1:struct
        let s_6_23: u32 = fn_state.ga_204679._1;
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
        // N s_7_0: panic
        panic!("{:?}", ());
        // N s_7_1: return
        return;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #() : ()
        let s_8_0: () = ();
        // S s_8_1: call CNTVOFF_read(s_8_0)
        let s_8_1: u64 = CNTVOFF_read(state, tracer, s_8_0);
        // C s_8_2: const #32s : i
        let s_8_2: i128 = 32;
        // S s_8_3: cast zx s_8_1 -> bv
        let s_8_3: Bits = Bits::new(s_8_1 as u128, 64u16);
        // C s_8_4: const #1s : i64
        let s_8_4: i64 = 1;
        // C s_8_5: cast zx s_8_4 -> i
        let s_8_5: i128 = (i128::try_from(s_8_4).unwrap());
        // C s_8_6: const #31s : i
        let s_8_6: i128 = 31;
        // C s_8_7: add s_8_6 s_8_5
        let s_8_7: i128 = (s_8_6 + s_8_5);
        // D s_8_8: bit-extract s_8_3 s_8_2 s_8_7
        let s_8_8: Bits = (Bits::new(
            ((s_8_3) >> (s_8_2)).value(),
            u16::try_from(s_8_7).unwrap(),
        ));
        // D s_8_9: cast reint s_8_8 -> u32
        let s_8_9: u32 = (s_8_8.value() as u32);
        // C s_8_10: const #() : ()
        let s_8_10: () = ();
        // S s_8_11: call CNTVOFF_read(s_8_10)
        let s_8_11: u64 = CNTVOFF_read(state, tracer, s_8_10);
        // C s_8_12: const #0s : i
        let s_8_12: i128 = 0;
        // S s_8_13: cast zx s_8_11 -> bv
        let s_8_13: Bits = Bits::new(s_8_11 as u128, 64u16);
        // C s_8_14: const #1s : i64
        let s_8_14: i64 = 1;
        // C s_8_15: cast zx s_8_14 -> i
        let s_8_15: i128 = (i128::try_from(s_8_14).unwrap());
        // C s_8_16: const #31s : i
        let s_8_16: i128 = 31;
        // C s_8_17: add s_8_16 s_8_15
        let s_8_17: i128 = (s_8_16 + s_8_15);
        // D s_8_18: bit-extract s_8_13 s_8_12 s_8_17
        let s_8_18: Bits = (Bits::new(
            ((s_8_13) >> (s_8_12)).value(),
            u16::try_from(s_8_17).unwrap(),
        ));
        // D s_8_19: cast reint s_8_18 -> u32
        let s_8_19: u32 = (s_8_18.value() as u32);
        // D s_8_20: create-product struct = ["s_8_9", "s_8_19"]
        let s_8_20: ProductType72d61775f103f7e0 = ProductType72d61775f103f7e0 {
            _0: s_8_9,
            _1: s_8_19,
        };
        // D s_8_21: write-var ga#204671 <= s_8_20
        fn_state.ga_204671 = s_8_20;
        // D s_8_22: read-var ga#204671.0:struct
        let s_8_22: u32 = fn_state.ga_204671._0;
        // D s_8_23: read-var ga#204671.1:struct
        let s_8_23: u32 = fn_state.ga_204671._1;
        // D s_8_24: read-var t2:i
        let s_8_24: i128 = fn_state.t2;
        // D s_8_25: call R_set(s_8_24, s_8_22)
        let s_8_25: () = R_set(state, tracer, s_8_24, s_8_22);
        // D s_8_26: read-var t:i
        let s_8_26: i128 = fn_state.t;
        // D s_8_27: call R_set(s_8_26, s_8_23)
        let s_8_27: () = R_set(state, tracer, s_8_26, s_8_23);
        // N s_8_28: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: panic
        panic!("{:?}", ());
        // N s_9_1: return
        return;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_10_0: panic
        panic!("{:?}", ());
        // N s_10_1: return
        return;
    }
}
