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
use ROL::*;
use Elem_set::*;
use Elem_read::*;
use Sbox::*;
use CheckNonStreamingSVEEnabled::*;
use Z_read::*;
use Z_set::*;
use common::*;
pub fn execute_SM4EKEY_Z_ZZ__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    VL: i64,
    d: i64,
    m: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_218093: i64,
        index: i64,
        source: u128,
        intval: u32,
        s: i64,
        roundresult: u128,
        VLshadow_4260: i64,
        result: Bits,
        i: i64,
        operand1: Bits,
        VLshadow_4261: i64,
        operand2: Bits,
        VL: i64,
        d: i64,
        m: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        VL,
        d,
        m,
        n,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var VL:i64
        let s_0_0: i64 = fn_state.VL;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var VLshadow#4260 <= s_0_2
        fn_state.VLshadow_4260 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call CheckNonStreamingSVEEnabled(s_0_4)
        let s_0_5: () = CheckNonStreamingSVEEnabled(state, tracer, s_0_4);
        // N s_0_6: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var VLshadow#4260:i64
        let s_1_0: i64 = fn_state.VLshadow_4260;
        // D s_1_1: write-var VLshadow#4261 <= s_1_0
        fn_state.VLshadow_4261 = s_1_0;
        // C s_1_2: const #128s : i
        let s_1_2: i128 = 128;
        // D s_1_3: read-var VLshadow#4261:i64
        let s_1_3: i64 = fn_state.VLshadow_4261;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: div s_1_4 s_1_2
        let s_1_5: i128 = ((s_1_4) / (s_1_2));
        // D s_1_6: cast reint s_1_5 -> i64
        let s_1_6: i64 = (s_1_5 as i64);
        // D s_1_7: read-var VLshadow#4261:i64
        let s_1_7: i64 = fn_state.VLshadow_4261;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: cast reint s_1_8 -> i64
        let s_1_9: i64 = (s_1_8 as i64);
        // D s_1_10: read-var n:i64
        let s_1_10: i64 = fn_state.n;
        // D s_1_11: cast zx s_1_10 -> i
        let s_1_11: i128 = (i128::try_from(s_1_10).unwrap());
        // D s_1_12: cast zx s_1_9 -> i
        let s_1_12: i128 = (i128::try_from(s_1_9).unwrap());
        // D s_1_13: call Z_read(s_1_11, s_1_12)
        let s_1_13: Bits = Z_read(state, tracer, s_1_11, s_1_12);
        // D s_1_14: write-var operand1 <= s_1_13
        fn_state.operand1 = s_1_13;
        // D s_1_15: read-var VLshadow#4261:i64
        let s_1_15: i64 = fn_state.VLshadow_4261;
        // D s_1_16: cast zx s_1_15 -> i
        let s_1_16: i128 = (i128::try_from(s_1_15).unwrap());
        // D s_1_17: cast reint s_1_16 -> i64
        let s_1_17: i64 = (s_1_16 as i64);
        // D s_1_18: read-var m:i64
        let s_1_18: i64 = fn_state.m;
        // D s_1_19: cast zx s_1_18 -> i
        let s_1_19: i128 = (i128::try_from(s_1_18).unwrap());
        // D s_1_20: cast zx s_1_17 -> i
        let s_1_20: i128 = (i128::try_from(s_1_17).unwrap());
        // D s_1_21: call Z_read(s_1_19, s_1_20)
        let s_1_21: Bits = Z_read(state, tracer, s_1_19, s_1_20);
        // D s_1_22: write-var operand2 <= s_1_21
        fn_state.operand2 = s_1_21;
        // C s_1_23: const #0s : i64
        let s_1_23: i64 = 0;
        // C s_1_24: const #1s : i
        let s_1_24: i128 = 1;
        // D s_1_25: cast zx s_1_6 -> i
        let s_1_25: i128 = (i128::try_from(s_1_6).unwrap());
        // D s_1_26: sub s_1_25 s_1_24
        let s_1_26: i128 = ((s_1_25) - (s_1_24));
        // D s_1_27: cast reint s_1_26 -> i64
        let s_1_27: i64 = (s_1_26 as i64);
        // D s_1_28: write-var gs#218093 <= s_1_27
        fn_state.gs_218093 = s_1_27;
        // D s_1_29: write-var s <= s_1_23
        fn_state.s = s_1_23;
        // N s_1_30: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var s:i64
        let s_2_0: i64 = fn_state.s;
        // D s_2_1: read-var gs#218093:i64
        let s_2_1: i64 = fn_state.gs_218093;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b10 b3
        if s_2_2 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #128s : i64
        let s_3_0: i64 = 128;
        // D s_3_1: read-var s:i64
        let s_3_1: i64 = fn_state.s;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // C s_3_3: cast zx s_3_0 -> i
        let s_3_3: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_4: read-var operand2:bv
        let s_3_4: Bits = fn_state.operand2;
        // D s_3_5: call Elem_read(s_3_4, s_3_2, s_3_3)
        let s_3_5: Bits = Elem_read(state, tracer, s_3_4, s_3_2, s_3_3);
        // D s_3_6: cast reint s_3_5 -> u128
        let s_3_6: u128 = (s_3_5.value() as u128);
        // D s_3_7: write-var source <= s_3_6
        fn_state.source = s_3_6;
        // C s_3_8: const #128s : i64
        let s_3_8: i64 = 128;
        // D s_3_9: read-var s:i64
        let s_3_9: i64 = fn_state.s;
        // D s_3_10: cast zx s_3_9 -> i
        let s_3_10: i128 = (i128::try_from(s_3_9).unwrap());
        // C s_3_11: cast zx s_3_8 -> i
        let s_3_11: i128 = (i128::try_from(s_3_8).unwrap());
        // D s_3_12: read-var operand1:bv
        let s_3_12: Bits = fn_state.operand1;
        // D s_3_13: call Elem_read(s_3_12, s_3_10, s_3_11)
        let s_3_13: Bits = Elem_read(state, tracer, s_3_12, s_3_10, s_3_11);
        // D s_3_14: cast reint s_3_13 -> u128
        let s_3_14: u128 = (s_3_13.value() as u128);
        // D s_3_15: write-var roundresult <= s_3_14
        fn_state.roundresult = s_3_14;
        // C s_3_16: const #0s : i64
        let s_3_16: i64 = 0;
        // D s_3_17: write-var index <= s_3_16
        fn_state.index = s_3_16;
        // N s_3_18: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var index:i64
        let s_4_0: i64 = fn_state.index;
        // C s_4_1: const #3s : i64
        let s_4_1: i64 = 3;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b9 b5
        if s_4_2 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #32s : i64
        let s_5_0: i64 = 32;
        // D s_5_1: read-var source:u128
        let s_5_1: u128 = fn_state.source;
        // D s_5_2: cast zx s_5_1 -> bv
        let s_5_2: Bits = Bits::new(s_5_1 as u128, 128u16);
        // D s_5_3: read-var index:i64
        let s_5_3: i64 = fn_state.index;
        // D s_5_4: cast zx s_5_3 -> i
        let s_5_4: i128 = (i128::try_from(s_5_3).unwrap());
        // C s_5_5: cast zx s_5_0 -> i
        let s_5_5: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_6: call Elem_read(s_5_2, s_5_4, s_5_5)
        let s_5_6: Bits = Elem_read(state, tracer, s_5_2, s_5_4, s_5_5);
        // D s_5_7: cast reint s_5_6 -> u32
        let s_5_7: u32 = (s_5_6.value() as u32);
        // C s_5_8: const #96s : i
        let s_5_8: i128 = 96;
        // D s_5_9: read-var roundresult:u128
        let s_5_9: u128 = fn_state.roundresult;
        // D s_5_10: cast zx s_5_9 -> bv
        let s_5_10: Bits = Bits::new(s_5_9 as u128, 128u16);
        // C s_5_11: const #1s : i64
        let s_5_11: i64 = 1;
        // C s_5_12: cast zx s_5_11 -> i
        let s_5_12: i128 = (i128::try_from(s_5_11).unwrap());
        // C s_5_13: const #31s : i
        let s_5_13: i128 = 31;
        // C s_5_14: add s_5_13 s_5_12
        let s_5_14: i128 = (s_5_13 + s_5_12);
        // D s_5_15: bit-extract s_5_10 s_5_8 s_5_14
        let s_5_15: Bits = (Bits::new(
            ((s_5_10) >> (s_5_8)).value(),
            u16::try_from(s_5_14).unwrap(),
        ));
        // D s_5_16: cast reint s_5_15 -> u32
        let s_5_16: u32 = (s_5_15.value() as u32);
        // C s_5_17: const #64s : i
        let s_5_17: i128 = 64;
        // D s_5_18: read-var roundresult:u128
        let s_5_18: u128 = fn_state.roundresult;
        // D s_5_19: cast zx s_5_18 -> bv
        let s_5_19: Bits = Bits::new(s_5_18 as u128, 128u16);
        // C s_5_20: const #1s : i64
        let s_5_20: i64 = 1;
        // C s_5_21: cast zx s_5_20 -> i
        let s_5_21: i128 = (i128::try_from(s_5_20).unwrap());
        // C s_5_22: const #31s : i
        let s_5_22: i128 = 31;
        // C s_5_23: add s_5_22 s_5_21
        let s_5_23: i128 = (s_5_22 + s_5_21);
        // D s_5_24: bit-extract s_5_19 s_5_17 s_5_23
        let s_5_24: Bits = (Bits::new(
            ((s_5_19) >> (s_5_17)).value(),
            u16::try_from(s_5_23).unwrap(),
        ));
        // D s_5_25: cast reint s_5_24 -> u32
        let s_5_25: u32 = (s_5_24.value() as u32);
        // D s_5_26: cast zx s_5_16 -> bv
        let s_5_26: Bits = Bits::new(s_5_16 as u128, 32u16);
        // D s_5_27: cast zx s_5_25 -> bv
        let s_5_27: Bits = Bits::new(s_5_25 as u128, 32u16);
        // D s_5_28: xor s_5_26 s_5_27
        let s_5_28: Bits = ((s_5_26) ^ (s_5_27));
        // D s_5_29: cast reint s_5_28 -> u32
        let s_5_29: u32 = (s_5_28.value() as u32);
        // C s_5_30: const #32s : i
        let s_5_30: i128 = 32;
        // D s_5_31: read-var roundresult:u128
        let s_5_31: u128 = fn_state.roundresult;
        // D s_5_32: cast zx s_5_31 -> bv
        let s_5_32: Bits = Bits::new(s_5_31 as u128, 128u16);
        // C s_5_33: const #1s : i64
        let s_5_33: i64 = 1;
        // C s_5_34: cast zx s_5_33 -> i
        let s_5_34: i128 = (i128::try_from(s_5_33).unwrap());
        // C s_5_35: const #31s : i
        let s_5_35: i128 = 31;
        // C s_5_36: add s_5_35 s_5_34
        let s_5_36: i128 = (s_5_35 + s_5_34);
        // D s_5_37: bit-extract s_5_32 s_5_30 s_5_36
        let s_5_37: Bits = (Bits::new(
            ((s_5_32) >> (s_5_30)).value(),
            u16::try_from(s_5_36).unwrap(),
        ));
        // D s_5_38: cast reint s_5_37 -> u32
        let s_5_38: u32 = (s_5_37.value() as u32);
        // D s_5_39: cast zx s_5_29 -> bv
        let s_5_39: Bits = Bits::new(s_5_29 as u128, 32u16);
        // D s_5_40: cast zx s_5_38 -> bv
        let s_5_40: Bits = Bits::new(s_5_38 as u128, 32u16);
        // D s_5_41: xor s_5_39 s_5_40
        let s_5_41: Bits = ((s_5_39) ^ (s_5_40));
        // D s_5_42: cast reint s_5_41 -> u32
        let s_5_42: u32 = (s_5_41.value() as u32);
        // D s_5_43: cast zx s_5_42 -> bv
        let s_5_43: Bits = Bits::new(s_5_42 as u128, 32u16);
        // D s_5_44: cast zx s_5_7 -> bv
        let s_5_44: Bits = Bits::new(s_5_7 as u128, 32u16);
        // D s_5_45: xor s_5_43 s_5_44
        let s_5_45: Bits = ((s_5_43) ^ (s_5_44));
        // D s_5_46: cast reint s_5_45 -> u32
        let s_5_46: u32 = (s_5_45.value() as u32);
        // D s_5_47: write-var intval <= s_5_46
        fn_state.intval = s_5_46;
        // C s_5_48: const #0s : i64
        let s_5_48: i64 = 0;
        // D s_5_49: write-var i <= s_5_48
        fn_state.i = s_5_48;
        // N s_5_50: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var i:i64
        let s_6_0: i64 = fn_state.i;
        // C s_6_1: const #3s : i64
        let s_6_1: i64 = 3;
        // D s_6_2: cmp-gt s_6_0 s_6_1
        let s_6_2: bool = ((s_6_0) > (s_6_1));
        // N s_6_3: branch s_6_2 b8 b7
        if s_6_2 {
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
        // C s_7_0: const #8s : i64
        let s_7_0: i64 = 8;
        // C s_7_1: const #8s : i64
        let s_7_1: i64 = 8;
        // D s_7_2: read-var intval:u32
        let s_7_2: u32 = fn_state.intval;
        // D s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 32u16);
        // D s_7_4: read-var i:i64
        let s_7_4: i64 = fn_state.i;
        // D s_7_5: cast zx s_7_4 -> i
        let s_7_5: i128 = (i128::try_from(s_7_4).unwrap());
        // C s_7_6: cast zx s_7_1 -> i
        let s_7_6: i128 = (i128::try_from(s_7_1).unwrap());
        // D s_7_7: call Elem_read(s_7_3, s_7_5, s_7_6)
        let s_7_7: Bits = Elem_read(state, tracer, s_7_3, s_7_5, s_7_6);
        // D s_7_8: cast reint s_7_7 -> u8
        let s_7_8: u8 = (s_7_7.value() as u8);
        // D s_7_9: call Sbox(s_7_8)
        let s_7_9: u8 = Sbox(state, tracer, s_7_8);
        // D s_7_10: read-var intval:u32
        let s_7_10: u32 = fn_state.intval;
        // D s_7_11: cast zx s_7_10 -> bv
        let s_7_11: Bits = Bits::new(s_7_10 as u128, 32u16);
        // D s_7_12: read-var i:i64
        let s_7_12: i64 = fn_state.i;
        // D s_7_13: cast zx s_7_12 -> i
        let s_7_13: i128 = (i128::try_from(s_7_12).unwrap());
        // C s_7_14: cast zx s_7_0 -> i
        let s_7_14: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_15: cast zx s_7_9 -> bv
        let s_7_15: Bits = Bits::new(s_7_9 as u128, 8u16);
        // D s_7_16: call Elem_set(s_7_11, s_7_13, s_7_14, s_7_15)
        let s_7_16: Bits = Elem_set(state, tracer, s_7_11, s_7_13, s_7_14, s_7_15);
        // D s_7_17: cast reint s_7_16 -> u32
        let s_7_17: u32 = (s_7_16.value() as u32);
        // D s_7_18: write-var intval <= s_7_17
        fn_state.intval = s_7_17;
        // D s_7_19: read-var i:i64
        let s_7_19: i64 = fn_state.i;
        // C s_7_20: const #1s : i64
        let s_7_20: i64 = 1;
        // D s_7_21: add s_7_19 s_7_20
        let s_7_21: i64 = (s_7_19 + s_7_20);
        // D s_7_22: write-var i <= s_7_21
        fn_state.i = s_7_21;
        // N s_7_23: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #13s : i
        let s_8_0: i128 = 13;
        // D s_8_1: read-var intval:u32
        let s_8_1: u32 = fn_state.intval;
        // D s_8_2: cast zx s_8_1 -> bv
        let s_8_2: Bits = Bits::new(s_8_1 as u128, 32u16);
        // D s_8_3: call ROL(s_8_2, s_8_0)
        let s_8_3: Bits = ROL(state, tracer, s_8_2, s_8_0);
        // D s_8_4: cast reint s_8_3 -> u32
        let s_8_4: u32 = (s_8_3.value() as u32);
        // D s_8_5: read-var intval:u32
        let s_8_5: u32 = fn_state.intval;
        // D s_8_6: cast zx s_8_5 -> bv
        let s_8_6: Bits = Bits::new(s_8_5 as u128, 32u16);
        // D s_8_7: cast zx s_8_4 -> bv
        let s_8_7: Bits = Bits::new(s_8_4 as u128, 32u16);
        // D s_8_8: xor s_8_6 s_8_7
        let s_8_8: Bits = ((s_8_6) ^ (s_8_7));
        // D s_8_9: cast reint s_8_8 -> u32
        let s_8_9: u32 = (s_8_8.value() as u32);
        // C s_8_10: const #23s : i
        let s_8_10: i128 = 23;
        // D s_8_11: read-var intval:u32
        let s_8_11: u32 = fn_state.intval;
        // D s_8_12: cast zx s_8_11 -> bv
        let s_8_12: Bits = Bits::new(s_8_11 as u128, 32u16);
        // D s_8_13: call ROL(s_8_12, s_8_10)
        let s_8_13: Bits = ROL(state, tracer, s_8_12, s_8_10);
        // D s_8_14: cast reint s_8_13 -> u32
        let s_8_14: u32 = (s_8_13.value() as u32);
        // D s_8_15: cast zx s_8_9 -> bv
        let s_8_15: Bits = Bits::new(s_8_9 as u128, 32u16);
        // D s_8_16: cast zx s_8_14 -> bv
        let s_8_16: Bits = Bits::new(s_8_14 as u128, 32u16);
        // D s_8_17: xor s_8_15 s_8_16
        let s_8_17: Bits = ((s_8_15) ^ (s_8_16));
        // D s_8_18: cast reint s_8_17 -> u32
        let s_8_18: u32 = (s_8_17.value() as u32);
        // D s_8_19: write-var intval <= s_8_18
        fn_state.intval = s_8_18;
        // C s_8_20: const #0s : i
        let s_8_20: i128 = 0;
        // D s_8_21: read-var roundresult:u128
        let s_8_21: u128 = fn_state.roundresult;
        // D s_8_22: cast zx s_8_21 -> bv
        let s_8_22: Bits = Bits::new(s_8_21 as u128, 128u16);
        // C s_8_23: const #1s : i64
        let s_8_23: i64 = 1;
        // C s_8_24: cast zx s_8_23 -> i
        let s_8_24: i128 = (i128::try_from(s_8_23).unwrap());
        // C s_8_25: const #31s : i
        let s_8_25: i128 = 31;
        // C s_8_26: add s_8_25 s_8_24
        let s_8_26: i128 = (s_8_25 + s_8_24);
        // D s_8_27: bit-extract s_8_22 s_8_20 s_8_26
        let s_8_27: Bits = (Bits::new(
            ((s_8_22) >> (s_8_20)).value(),
            u16::try_from(s_8_26).unwrap(),
        ));
        // D s_8_28: cast reint s_8_27 -> u32
        let s_8_28: u32 = (s_8_27.value() as u32);
        // D s_8_29: read-var intval:u32
        let s_8_29: u32 = fn_state.intval;
        // D s_8_30: cast zx s_8_29 -> bv
        let s_8_30: Bits = Bits::new(s_8_29 as u128, 32u16);
        // D s_8_31: cast zx s_8_28 -> bv
        let s_8_31: Bits = Bits::new(s_8_28 as u128, 32u16);
        // D s_8_32: xor s_8_30 s_8_31
        let s_8_32: Bits = ((s_8_30) ^ (s_8_31));
        // D s_8_33: cast reint s_8_32 -> u32
        let s_8_33: u32 = (s_8_32.value() as u32);
        // D s_8_34: write-var intval <= s_8_33
        fn_state.intval = s_8_33;
        // C s_8_35: const #32s : i
        let s_8_35: i128 = 32;
        // D s_8_36: read-var roundresult:u128
        let s_8_36: u128 = fn_state.roundresult;
        // D s_8_37: cast zx s_8_36 -> bv
        let s_8_37: Bits = Bits::new(s_8_36 as u128, 128u16);
        // C s_8_38: const #1s : i64
        let s_8_38: i64 = 1;
        // C s_8_39: cast zx s_8_38 -> i
        let s_8_39: i128 = (i128::try_from(s_8_38).unwrap());
        // C s_8_40: const #31s : i
        let s_8_40: i128 = 31;
        // C s_8_41: add s_8_40 s_8_39
        let s_8_41: i128 = (s_8_40 + s_8_39);
        // D s_8_42: bit-extract s_8_37 s_8_35 s_8_41
        let s_8_42: Bits = (Bits::new(
            ((s_8_37) >> (s_8_35)).value(),
            u16::try_from(s_8_41).unwrap(),
        ));
        // D s_8_43: cast reint s_8_42 -> u32
        let s_8_43: u32 = (s_8_42.value() as u32);
        // C s_8_44: const #0s : i
        let s_8_44: i128 = 0;
        // D s_8_45: read-var roundresult:u128
        let s_8_45: u128 = fn_state.roundresult;
        // D s_8_46: cast zx s_8_45 -> bv
        let s_8_46: Bits = Bits::new(s_8_45 as u128, 128u16);
        // D s_8_47: cast zx s_8_43 -> bv
        let s_8_47: Bits = Bits::new(s_8_43 as u128, 32u16);
        // C s_8_48: const #31s : i
        let s_8_48: i128 = 31;
        // C s_8_49: const #1u : u64
        let s_8_49: u64 = 1;
        // C s_8_50: cast zx s_8_49 -> bv
        let s_8_50: Bits = Bits::new(s_8_49 as u128, 64u16);
        // C s_8_51: lsl s_8_50 s_8_48
        let s_8_51: Bits = s_8_50 << s_8_48;
        // C s_8_52: sub s_8_51 s_8_50
        let s_8_52: Bits = ((s_8_51) - (s_8_50));
        // D s_8_53: and s_8_47 s_8_52
        let s_8_53: Bits = ((s_8_47) & (s_8_52));
        // D s_8_54: lsl s_8_53 s_8_44
        let s_8_54: Bits = s_8_53 << s_8_44;
        // C s_8_55: lsl s_8_52 s_8_44
        let s_8_55: Bits = s_8_52 << s_8_44;
        // C s_8_56: cmpl s_8_55
        let s_8_56: Bits = !s_8_55;
        // D s_8_57: and s_8_46 s_8_56
        let s_8_57: Bits = ((s_8_46) & (s_8_56));
        // D s_8_58: or s_8_57 s_8_54
        let s_8_58: Bits = ((s_8_57) | (s_8_54));
        // D s_8_59: cast reint s_8_58 -> u128
        let s_8_59: u128 = (s_8_58.value() as u128);
        // D s_8_60: write-var roundresult <= s_8_59
        fn_state.roundresult = s_8_59;
        // C s_8_61: const #64s : i
        let s_8_61: i128 = 64;
        // D s_8_62: read-var roundresult:u128
        let s_8_62: u128 = fn_state.roundresult;
        // D s_8_63: cast zx s_8_62 -> bv
        let s_8_63: Bits = Bits::new(s_8_62 as u128, 128u16);
        // C s_8_64: const #1s : i64
        let s_8_64: i64 = 1;
        // C s_8_65: cast zx s_8_64 -> i
        let s_8_65: i128 = (i128::try_from(s_8_64).unwrap());
        // C s_8_66: const #31s : i
        let s_8_66: i128 = 31;
        // C s_8_67: add s_8_66 s_8_65
        let s_8_67: i128 = (s_8_66 + s_8_65);
        // D s_8_68: bit-extract s_8_63 s_8_61 s_8_67
        let s_8_68: Bits = (Bits::new(
            ((s_8_63) >> (s_8_61)).value(),
            u16::try_from(s_8_67).unwrap(),
        ));
        // D s_8_69: cast reint s_8_68 -> u32
        let s_8_69: u32 = (s_8_68.value() as u32);
        // C s_8_70: const #32s : i
        let s_8_70: i128 = 32;
        // D s_8_71: read-var roundresult:u128
        let s_8_71: u128 = fn_state.roundresult;
        // D s_8_72: cast zx s_8_71 -> bv
        let s_8_72: Bits = Bits::new(s_8_71 as u128, 128u16);
        // D s_8_73: cast zx s_8_69 -> bv
        let s_8_73: Bits = Bits::new(s_8_69 as u128, 32u16);
        // C s_8_74: const #31s : i
        let s_8_74: i128 = 31;
        // C s_8_75: const #1u : u64
        let s_8_75: u64 = 1;
        // C s_8_76: cast zx s_8_75 -> bv
        let s_8_76: Bits = Bits::new(s_8_75 as u128, 64u16);
        // C s_8_77: lsl s_8_76 s_8_74
        let s_8_77: Bits = s_8_76 << s_8_74;
        // C s_8_78: sub s_8_77 s_8_76
        let s_8_78: Bits = ((s_8_77) - (s_8_76));
        // D s_8_79: and s_8_73 s_8_78
        let s_8_79: Bits = ((s_8_73) & (s_8_78));
        // D s_8_80: lsl s_8_79 s_8_70
        let s_8_80: Bits = s_8_79 << s_8_70;
        // C s_8_81: lsl s_8_78 s_8_70
        let s_8_81: Bits = s_8_78 << s_8_70;
        // C s_8_82: cmpl s_8_81
        let s_8_82: Bits = !s_8_81;
        // D s_8_83: and s_8_72 s_8_82
        let s_8_83: Bits = ((s_8_72) & (s_8_82));
        // D s_8_84: or s_8_83 s_8_80
        let s_8_84: Bits = ((s_8_83) | (s_8_80));
        // D s_8_85: cast reint s_8_84 -> u128
        let s_8_85: u128 = (s_8_84.value() as u128);
        // D s_8_86: write-var roundresult <= s_8_85
        fn_state.roundresult = s_8_85;
        // C s_8_87: const #96s : i
        let s_8_87: i128 = 96;
        // D s_8_88: read-var roundresult:u128
        let s_8_88: u128 = fn_state.roundresult;
        // D s_8_89: cast zx s_8_88 -> bv
        let s_8_89: Bits = Bits::new(s_8_88 as u128, 128u16);
        // C s_8_90: const #1s : i64
        let s_8_90: i64 = 1;
        // C s_8_91: cast zx s_8_90 -> i
        let s_8_91: i128 = (i128::try_from(s_8_90).unwrap());
        // C s_8_92: const #31s : i
        let s_8_92: i128 = 31;
        // C s_8_93: add s_8_92 s_8_91
        let s_8_93: i128 = (s_8_92 + s_8_91);
        // D s_8_94: bit-extract s_8_89 s_8_87 s_8_93
        let s_8_94: Bits = (Bits::new(
            ((s_8_89) >> (s_8_87)).value(),
            u16::try_from(s_8_93).unwrap(),
        ));
        // D s_8_95: cast reint s_8_94 -> u32
        let s_8_95: u32 = (s_8_94.value() as u32);
        // C s_8_96: const #64s : i
        let s_8_96: i128 = 64;
        // D s_8_97: read-var roundresult:u128
        let s_8_97: u128 = fn_state.roundresult;
        // D s_8_98: cast zx s_8_97 -> bv
        let s_8_98: Bits = Bits::new(s_8_97 as u128, 128u16);
        // D s_8_99: cast zx s_8_95 -> bv
        let s_8_99: Bits = Bits::new(s_8_95 as u128, 32u16);
        // C s_8_100: const #31s : i
        let s_8_100: i128 = 31;
        // C s_8_101: const #1u : u64
        let s_8_101: u64 = 1;
        // C s_8_102: cast zx s_8_101 -> bv
        let s_8_102: Bits = Bits::new(s_8_101 as u128, 64u16);
        // C s_8_103: lsl s_8_102 s_8_100
        let s_8_103: Bits = s_8_102 << s_8_100;
        // C s_8_104: sub s_8_103 s_8_102
        let s_8_104: Bits = ((s_8_103) - (s_8_102));
        // D s_8_105: and s_8_99 s_8_104
        let s_8_105: Bits = ((s_8_99) & (s_8_104));
        // D s_8_106: lsl s_8_105 s_8_96
        let s_8_106: Bits = s_8_105 << s_8_96;
        // C s_8_107: lsl s_8_104 s_8_96
        let s_8_107: Bits = s_8_104 << s_8_96;
        // C s_8_108: cmpl s_8_107
        let s_8_108: Bits = !s_8_107;
        // D s_8_109: and s_8_98 s_8_108
        let s_8_109: Bits = ((s_8_98) & (s_8_108));
        // D s_8_110: or s_8_109 s_8_106
        let s_8_110: Bits = ((s_8_109) | (s_8_106));
        // D s_8_111: cast reint s_8_110 -> u128
        let s_8_111: u128 = (s_8_110.value() as u128);
        // D s_8_112: write-var roundresult <= s_8_111
        fn_state.roundresult = s_8_111;
        // C s_8_113: const #96s : i
        let s_8_113: i128 = 96;
        // D s_8_114: read-var roundresult:u128
        let s_8_114: u128 = fn_state.roundresult;
        // D s_8_115: cast zx s_8_114 -> bv
        let s_8_115: Bits = Bits::new(s_8_114 as u128, 128u16);
        // D s_8_116: read-var intval:u32
        let s_8_116: u32 = fn_state.intval;
        // D s_8_117: cast zx s_8_116 -> bv
        let s_8_117: Bits = Bits::new(s_8_116 as u128, 32u16);
        // C s_8_118: const #31s : i
        let s_8_118: i128 = 31;
        // C s_8_119: const #1u : u64
        let s_8_119: u64 = 1;
        // C s_8_120: cast zx s_8_119 -> bv
        let s_8_120: Bits = Bits::new(s_8_119 as u128, 64u16);
        // C s_8_121: lsl s_8_120 s_8_118
        let s_8_121: Bits = s_8_120 << s_8_118;
        // C s_8_122: sub s_8_121 s_8_120
        let s_8_122: Bits = ((s_8_121) - (s_8_120));
        // D s_8_123: and s_8_117 s_8_122
        let s_8_123: Bits = ((s_8_117) & (s_8_122));
        // D s_8_124: lsl s_8_123 s_8_113
        let s_8_124: Bits = s_8_123 << s_8_113;
        // C s_8_125: lsl s_8_122 s_8_113
        let s_8_125: Bits = s_8_122 << s_8_113;
        // C s_8_126: cmpl s_8_125
        let s_8_126: Bits = !s_8_125;
        // D s_8_127: and s_8_115 s_8_126
        let s_8_127: Bits = ((s_8_115) & (s_8_126));
        // D s_8_128: or s_8_127 s_8_124
        let s_8_128: Bits = ((s_8_127) | (s_8_124));
        // D s_8_129: cast reint s_8_128 -> u128
        let s_8_129: u128 = (s_8_128.value() as u128);
        // D s_8_130: write-var roundresult <= s_8_129
        fn_state.roundresult = s_8_129;
        // D s_8_131: read-var index:i64
        let s_8_131: i64 = fn_state.index;
        // C s_8_132: const #1s : i64
        let s_8_132: i64 = 1;
        // D s_8_133: add s_8_131 s_8_132
        let s_8_133: i64 = (s_8_131 + s_8_132);
        // D s_8_134: write-var index <= s_8_133
        fn_state.index = s_8_133;
        // N s_8_135: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #128s : i64
        let s_9_0: i64 = 128;
        // D s_9_1: read-var s:i64
        let s_9_1: i64 = fn_state.s;
        // D s_9_2: cast zx s_9_1 -> i
        let s_9_2: i128 = (i128::try_from(s_9_1).unwrap());
        // C s_9_3: cast zx s_9_0 -> i
        let s_9_3: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_4: read-var roundresult:u128
        let s_9_4: u128 = fn_state.roundresult;
        // D s_9_5: cast zx s_9_4 -> bv
        let s_9_5: Bits = Bits::new(s_9_4 as u128, 128u16);
        // D s_9_6: read-var result:bv
        let s_9_6: Bits = fn_state.result;
        // D s_9_7: call Elem_set(s_9_6, s_9_2, s_9_3, s_9_5)
        let s_9_7: Bits = Elem_set(state, tracer, s_9_6, s_9_2, s_9_3, s_9_5);
        // D s_9_8: write-var result <= s_9_7
        fn_state.result = s_9_7;
        // D s_9_9: read-var s:i64
        let s_9_9: i64 = fn_state.s;
        // C s_9_10: const #1s : i64
        let s_9_10: i64 = 1;
        // D s_9_11: add s_9_9 s_9_10
        let s_9_11: i64 = (s_9_9 + s_9_10);
        // D s_9_12: write-var s <= s_9_11
        fn_state.s = s_9_11;
        // N s_9_13: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var VLshadow#4261:i64
        let s_10_0: i64 = fn_state.VLshadow_4261;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: cast reint s_10_1 -> i64
        let s_10_2: i64 = (s_10_1 as i64);
        // D s_10_3: read-var d:i64
        let s_10_3: i64 = fn_state.d;
        // D s_10_4: cast zx s_10_3 -> i
        let s_10_4: i128 = (i128::try_from(s_10_3).unwrap());
        // D s_10_5: cast zx s_10_2 -> i
        let s_10_5: i128 = (i128::try_from(s_10_2).unwrap());
        // D s_10_6: read-var result:bv
        let s_10_6: Bits = fn_state.result;
        // D s_10_7: call Z_set(s_10_4, s_10_5, s_10_6)
        let s_10_7: () = Z_set(state, tracer, s_10_4, s_10_5, s_10_6);
        // N s_10_8: return
        return;
    }
}
