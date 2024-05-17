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
use u_get_SCR_Type_NS::*;
use CNTHP_CVAL_read::*;
use R_set::*;
use common::*;
pub fn CNTHP_CVAL_SysRegRead64_bedb5ea1dc82962f<T: Tracer>(
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
        ga_204310: ProductType5c790c8ef59cc8b2,
        ga_204316: ProductType72d61775f103f7e0,
        ga_204322: ProductType5c790c8ef59cc8b2,
        ga_204326: ProductType72d61775f103f7e0,
        ga_204320: ProductType5c790c8ef59cc8b2,
        u__PSTATE_EL: u8,
        ga_204312: ProductType5c790c8ef59cc8b2,
        u__SCR_NS: bool,
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
        // S s_6_1: call CNTHP_CVAL_read(s_6_0)
        let s_6_1: ProductType5c790c8ef59cc8b2 = CNTHP_CVAL_read(state, tracer, s_6_0);
        // D s_6_2: write-var ga#204320 <= s_6_1
        fn_state.ga_204320 = s_6_1;
        // D s_6_3: read-var ga#204320.0:struct
        let s_6_3: u64 = fn_state.ga_204320._0;
        // C s_6_4: const #32s : i
        let s_6_4: i128 = 32;
        // D s_6_5: cast zx s_6_3 -> bv
        let s_6_5: Bits = Bits::new(s_6_3 as u128, 64u16);
        // C s_6_6: const #1s : i64
        let s_6_6: i64 = 1;
        // C s_6_7: cast zx s_6_6 -> i
        let s_6_7: i128 = (i128::try_from(s_6_6).unwrap());
        // C s_6_8: const #31s : i
        let s_6_8: i128 = 31;
        // C s_6_9: add s_6_8 s_6_7
        let s_6_9: i128 = (s_6_8 + s_6_7);
        // D s_6_10: bit-extract s_6_5 s_6_4 s_6_9
        let s_6_10: Bits = (Bits::new(
            ((s_6_5) >> (s_6_4)).value(),
            u16::try_from(s_6_9).unwrap(),
        ));
        // D s_6_11: cast reint s_6_10 -> u32
        let s_6_11: u32 = (s_6_10.value() as u32);
        // C s_6_12: const #() : ()
        let s_6_12: () = ();
        // S s_6_13: call CNTHP_CVAL_read(s_6_12)
        let s_6_13: ProductType5c790c8ef59cc8b2 = CNTHP_CVAL_read(state, tracer, s_6_12);
        // D s_6_14: write-var ga#204322 <= s_6_13
        fn_state.ga_204322 = s_6_13;
        // D s_6_15: read-var ga#204322.0:struct
        let s_6_15: u64 = fn_state.ga_204322._0;
        // C s_6_16: const #0s : i
        let s_6_16: i128 = 0;
        // D s_6_17: cast zx s_6_15 -> bv
        let s_6_17: Bits = Bits::new(s_6_15 as u128, 64u16);
        // C s_6_18: const #1s : i64
        let s_6_18: i64 = 1;
        // C s_6_19: cast zx s_6_18 -> i
        let s_6_19: i128 = (i128::try_from(s_6_18).unwrap());
        // C s_6_20: const #31s : i
        let s_6_20: i128 = 31;
        // C s_6_21: add s_6_20 s_6_19
        let s_6_21: i128 = (s_6_20 + s_6_19);
        // D s_6_22: bit-extract s_6_17 s_6_16 s_6_21
        let s_6_22: Bits = (Bits::new(
            ((s_6_17) >> (s_6_16)).value(),
            u16::try_from(s_6_21).unwrap(),
        ));
        // D s_6_23: cast reint s_6_22 -> u32
        let s_6_23: u32 = (s_6_22.value() as u32);
        // D s_6_24: create-product struct = ["s_6_11", "s_6_23"]
        let s_6_24: ProductType72d61775f103f7e0 = ProductType72d61775f103f7e0 {
            _0: s_6_11,
            _1: s_6_23,
        };
        // D s_6_25: write-var ga#204326 <= s_6_24
        fn_state.ga_204326 = s_6_24;
        // D s_6_26: read-var ga#204326.0:struct
        let s_6_26: u32 = fn_state.ga_204326._0;
        // D s_6_27: read-var ga#204326.1:struct
        let s_6_27: u32 = fn_state.ga_204326._1;
        // D s_6_28: read-var t2:i
        let s_6_28: i128 = fn_state.t2;
        // D s_6_29: call R_set(s_6_28, s_6_26)
        let s_6_29: () = R_set(state, tracer, s_6_28, s_6_26);
        // D s_6_30: read-var t:i
        let s_6_30: i128 = fn_state.t;
        // D s_6_31: call R_set(s_6_30, s_6_27)
        let s_6_31: () = R_set(state, tracer, s_6_30, s_6_27);
        // N s_6_32: return
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
        // S s_8_1: call CNTHP_CVAL_read(s_8_0)
        let s_8_1: ProductType5c790c8ef59cc8b2 = CNTHP_CVAL_read(state, tracer, s_8_0);
        // D s_8_2: write-var ga#204310 <= s_8_1
        fn_state.ga_204310 = s_8_1;
        // D s_8_3: read-var ga#204310.0:struct
        let s_8_3: u64 = fn_state.ga_204310._0;
        // C s_8_4: const #32s : i
        let s_8_4: i128 = 32;
        // D s_8_5: cast zx s_8_3 -> bv
        let s_8_5: Bits = Bits::new(s_8_3 as u128, 64u16);
        // C s_8_6: const #1s : i64
        let s_8_6: i64 = 1;
        // C s_8_7: cast zx s_8_6 -> i
        let s_8_7: i128 = (i128::try_from(s_8_6).unwrap());
        // C s_8_8: const #31s : i
        let s_8_8: i128 = 31;
        // C s_8_9: add s_8_8 s_8_7
        let s_8_9: i128 = (s_8_8 + s_8_7);
        // D s_8_10: bit-extract s_8_5 s_8_4 s_8_9
        let s_8_10: Bits = (Bits::new(
            ((s_8_5) >> (s_8_4)).value(),
            u16::try_from(s_8_9).unwrap(),
        ));
        // D s_8_11: cast reint s_8_10 -> u32
        let s_8_11: u32 = (s_8_10.value() as u32);
        // C s_8_12: const #() : ()
        let s_8_12: () = ();
        // S s_8_13: call CNTHP_CVAL_read(s_8_12)
        let s_8_13: ProductType5c790c8ef59cc8b2 = CNTHP_CVAL_read(state, tracer, s_8_12);
        // D s_8_14: write-var ga#204312 <= s_8_13
        fn_state.ga_204312 = s_8_13;
        // D s_8_15: read-var ga#204312.0:struct
        let s_8_15: u64 = fn_state.ga_204312._0;
        // C s_8_16: const #0s : i
        let s_8_16: i128 = 0;
        // D s_8_17: cast zx s_8_15 -> bv
        let s_8_17: Bits = Bits::new(s_8_15 as u128, 64u16);
        // C s_8_18: const #1s : i64
        let s_8_18: i64 = 1;
        // C s_8_19: cast zx s_8_18 -> i
        let s_8_19: i128 = (i128::try_from(s_8_18).unwrap());
        // C s_8_20: const #31s : i
        let s_8_20: i128 = 31;
        // C s_8_21: add s_8_20 s_8_19
        let s_8_21: i128 = (s_8_20 + s_8_19);
        // D s_8_22: bit-extract s_8_17 s_8_16 s_8_21
        let s_8_22: Bits = (Bits::new(
            ((s_8_17) >> (s_8_16)).value(),
            u16::try_from(s_8_21).unwrap(),
        ));
        // D s_8_23: cast reint s_8_22 -> u32
        let s_8_23: u32 = (s_8_22.value() as u32);
        // D s_8_24: create-product struct = ["s_8_11", "s_8_23"]
        let s_8_24: ProductType72d61775f103f7e0 = ProductType72d61775f103f7e0 {
            _0: s_8_11,
            _1: s_8_23,
        };
        // D s_8_25: write-var ga#204316 <= s_8_24
        fn_state.ga_204316 = s_8_24;
        // D s_8_26: read-var ga#204316.0:struct
        let s_8_26: u32 = fn_state.ga_204316._0;
        // D s_8_27: read-var ga#204316.1:struct
        let s_8_27: u32 = fn_state.ga_204316._1;
        // D s_8_28: read-var t2:i
        let s_8_28: i128 = fn_state.t2;
        // D s_8_29: call R_set(s_8_28, s_8_26)
        let s_8_29: () = R_set(state, tracer, s_8_28, s_8_26);
        // D s_8_30: read-var t:i
        let s_8_30: i128 = fn_state.t;
        // D s_8_31: call R_set(s_8_30, s_8_27)
        let s_8_31: () = R_set(state, tracer, s_8_30, s_8_27);
        // N s_8_32: return
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
