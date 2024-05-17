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
use V_read::*;
use Elem_read::*;
use AArch64_CheckFPAdvSIMDEnabled::*;
use Sbox::*;
use V_set::*;
use common::*;
pub fn execute_aarch64_instrs_vector_crypto_sm4_sm4enc<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    n: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        index: i64,
        Vn: u128,
        intval: u32,
        roundresult: u128,
        i: i64,
        d: i64,
        n: i64,
    }
    let fn_state = FunctionState {
        d,
        n,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call AArch64_CheckFPAdvSIMDEnabled(s_0_0)
        let s_0_1: () = AArch64_CheckFPAdvSIMDEnabled(state, tracer, s_0_0);
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #128s : i64
        let s_1_0: i64 = 128;
        // D s_1_1: read-var n:i64
        let s_1_1: i64 = fn_state.n;
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (i128::try_from(s_1_1).unwrap());
        // D s_1_3: call V_read(s_1_2, s_1_0)
        let s_1_3: Bits = V_read(state, tracer, s_1_2, s_1_0);
        // D s_1_4: cast reint s_1_3 -> u128
        let s_1_4: u128 = (s_1_3.value() as u128);
        // D s_1_5: write-var Vn <= s_1_4
        fn_state.Vn = s_1_4;
        // C s_1_6: const #128s : i64
        let s_1_6: i64 = 128;
        // D s_1_7: read-var d:i64
        let s_1_7: i64 = fn_state.d;
        // D s_1_8: cast zx s_1_7 -> i
        let s_1_8: i128 = (i128::try_from(s_1_7).unwrap());
        // D s_1_9: call V_read(s_1_8, s_1_6)
        let s_1_9: Bits = V_read(state, tracer, s_1_8, s_1_6);
        // D s_1_10: cast reint s_1_9 -> u128
        let s_1_10: u128 = (s_1_9.value() as u128);
        // D s_1_11: write-var roundresult <= s_1_10
        fn_state.roundresult = s_1_10;
        // C s_1_12: const #0s : i64
        let s_1_12: i64 = 0;
        // D s_1_13: write-var index <= s_1_12
        fn_state.index = s_1_12;
        // N s_1_14: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var index:i64
        let s_2_0: i64 = fn_state.index;
        // C s_2_1: const #3s : i64
        let s_2_1: i64 = 3;
        // D s_2_2: cmp-gt s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) > (s_2_1));
        // N s_2_3: branch s_2_2 b7 b3
        if s_2_2 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #32s : i64
        let s_3_0: i64 = 32;
        // D s_3_1: read-var Vn:u128
        let s_3_1: u128 = fn_state.Vn;
        // D s_3_2: cast zx s_3_1 -> bv
        let s_3_2: Bits = Bits::new(s_3_1 as u128, 128u16);
        // D s_3_3: read-var index:i64
        let s_3_3: i64 = fn_state.index;
        // D s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // C s_3_5: cast zx s_3_0 -> i
        let s_3_5: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_6: call Elem_read(s_3_2, s_3_4, s_3_5)
        let s_3_6: Bits = Elem_read(state, tracer, s_3_2, s_3_4, s_3_5);
        // D s_3_7: cast reint s_3_6 -> u32
        let s_3_7: u32 = (s_3_6.value() as u32);
        // C s_3_8: const #96s : i
        let s_3_8: i128 = 96;
        // D s_3_9: read-var roundresult:u128
        let s_3_9: u128 = fn_state.roundresult;
        // D s_3_10: cast zx s_3_9 -> bv
        let s_3_10: Bits = Bits::new(s_3_9 as u128, 128u16);
        // C s_3_11: const #1s : i64
        let s_3_11: i64 = 1;
        // C s_3_12: cast zx s_3_11 -> i
        let s_3_12: i128 = (i128::try_from(s_3_11).unwrap());
        // C s_3_13: const #31s : i
        let s_3_13: i128 = 31;
        // C s_3_14: add s_3_13 s_3_12
        let s_3_14: i128 = (s_3_13 + s_3_12);
        // D s_3_15: bit-extract s_3_10 s_3_8 s_3_14
        let s_3_15: Bits = (Bits::new(
            ((s_3_10) >> (s_3_8)).value(),
            u16::try_from(s_3_14).unwrap(),
        ));
        // D s_3_16: cast reint s_3_15 -> u32
        let s_3_16: u32 = (s_3_15.value() as u32);
        // C s_3_17: const #64s : i
        let s_3_17: i128 = 64;
        // D s_3_18: read-var roundresult:u128
        let s_3_18: u128 = fn_state.roundresult;
        // D s_3_19: cast zx s_3_18 -> bv
        let s_3_19: Bits = Bits::new(s_3_18 as u128, 128u16);
        // C s_3_20: const #1s : i64
        let s_3_20: i64 = 1;
        // C s_3_21: cast zx s_3_20 -> i
        let s_3_21: i128 = (i128::try_from(s_3_20).unwrap());
        // C s_3_22: const #31s : i
        let s_3_22: i128 = 31;
        // C s_3_23: add s_3_22 s_3_21
        let s_3_23: i128 = (s_3_22 + s_3_21);
        // D s_3_24: bit-extract s_3_19 s_3_17 s_3_23
        let s_3_24: Bits = (Bits::new(
            ((s_3_19) >> (s_3_17)).value(),
            u16::try_from(s_3_23).unwrap(),
        ));
        // D s_3_25: cast reint s_3_24 -> u32
        let s_3_25: u32 = (s_3_24.value() as u32);
        // D s_3_26: cast zx s_3_16 -> bv
        let s_3_26: Bits = Bits::new(s_3_16 as u128, 32u16);
        // D s_3_27: cast zx s_3_25 -> bv
        let s_3_27: Bits = Bits::new(s_3_25 as u128, 32u16);
        // D s_3_28: xor s_3_26 s_3_27
        let s_3_28: Bits = ((s_3_26) ^ (s_3_27));
        // D s_3_29: cast reint s_3_28 -> u32
        let s_3_29: u32 = (s_3_28.value() as u32);
        // C s_3_30: const #32s : i
        let s_3_30: i128 = 32;
        // D s_3_31: read-var roundresult:u128
        let s_3_31: u128 = fn_state.roundresult;
        // D s_3_32: cast zx s_3_31 -> bv
        let s_3_32: Bits = Bits::new(s_3_31 as u128, 128u16);
        // C s_3_33: const #1s : i64
        let s_3_33: i64 = 1;
        // C s_3_34: cast zx s_3_33 -> i
        let s_3_34: i128 = (i128::try_from(s_3_33).unwrap());
        // C s_3_35: const #31s : i
        let s_3_35: i128 = 31;
        // C s_3_36: add s_3_35 s_3_34
        let s_3_36: i128 = (s_3_35 + s_3_34);
        // D s_3_37: bit-extract s_3_32 s_3_30 s_3_36
        let s_3_37: Bits = (Bits::new(
            ((s_3_32) >> (s_3_30)).value(),
            u16::try_from(s_3_36).unwrap(),
        ));
        // D s_3_38: cast reint s_3_37 -> u32
        let s_3_38: u32 = (s_3_37.value() as u32);
        // D s_3_39: cast zx s_3_29 -> bv
        let s_3_39: Bits = Bits::new(s_3_29 as u128, 32u16);
        // D s_3_40: cast zx s_3_38 -> bv
        let s_3_40: Bits = Bits::new(s_3_38 as u128, 32u16);
        // D s_3_41: xor s_3_39 s_3_40
        let s_3_41: Bits = ((s_3_39) ^ (s_3_40));
        // D s_3_42: cast reint s_3_41 -> u32
        let s_3_42: u32 = (s_3_41.value() as u32);
        // D s_3_43: cast zx s_3_42 -> bv
        let s_3_43: Bits = Bits::new(s_3_42 as u128, 32u16);
        // D s_3_44: cast zx s_3_7 -> bv
        let s_3_44: Bits = Bits::new(s_3_7 as u128, 32u16);
        // D s_3_45: xor s_3_43 s_3_44
        let s_3_45: Bits = ((s_3_43) ^ (s_3_44));
        // D s_3_46: cast reint s_3_45 -> u32
        let s_3_46: u32 = (s_3_45.value() as u32);
        // D s_3_47: write-var intval <= s_3_46
        fn_state.intval = s_3_46;
        // C s_3_48: const #0s : i64
        let s_3_48: i64 = 0;
        // D s_3_49: write-var i <= s_3_48
        fn_state.i = s_3_48;
        // N s_3_50: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var i:i64
        let s_4_0: i64 = fn_state.i;
        // C s_4_1: const #3s : i64
        let s_4_1: i64 = 3;
        // D s_4_2: cmp-gt s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) > (s_4_1));
        // N s_4_3: branch s_4_2 b6 b5
        if s_4_2 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #8s : i64
        let s_5_0: i64 = 8;
        // C s_5_1: const #8s : i64
        let s_5_1: i64 = 8;
        // D s_5_2: read-var intval:u32
        let s_5_2: u32 = fn_state.intval;
        // D s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 32u16);
        // D s_5_4: read-var i:i64
        let s_5_4: i64 = fn_state.i;
        // D s_5_5: cast zx s_5_4 -> i
        let s_5_5: i128 = (i128::try_from(s_5_4).unwrap());
        // C s_5_6: cast zx s_5_1 -> i
        let s_5_6: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_7: call Elem_read(s_5_3, s_5_5, s_5_6)
        let s_5_7: Bits = Elem_read(state, tracer, s_5_3, s_5_5, s_5_6);
        // D s_5_8: cast reint s_5_7 -> u8
        let s_5_8: u8 = (s_5_7.value() as u8);
        // D s_5_9: call Sbox(s_5_8)
        let s_5_9: u8 = Sbox(state, tracer, s_5_8);
        // D s_5_10: read-var intval:u32
        let s_5_10: u32 = fn_state.intval;
        // D s_5_11: cast zx s_5_10 -> bv
        let s_5_11: Bits = Bits::new(s_5_10 as u128, 32u16);
        // D s_5_12: read-var i:i64
        let s_5_12: i64 = fn_state.i;
        // D s_5_13: cast zx s_5_12 -> i
        let s_5_13: i128 = (i128::try_from(s_5_12).unwrap());
        // C s_5_14: cast zx s_5_0 -> i
        let s_5_14: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_15: cast zx s_5_9 -> bv
        let s_5_15: Bits = Bits::new(s_5_9 as u128, 8u16);
        // D s_5_16: call Elem_set(s_5_11, s_5_13, s_5_14, s_5_15)
        let s_5_16: Bits = Elem_set(state, tracer, s_5_11, s_5_13, s_5_14, s_5_15);
        // D s_5_17: cast reint s_5_16 -> u32
        let s_5_17: u32 = (s_5_16.value() as u32);
        // D s_5_18: write-var intval <= s_5_17
        fn_state.intval = s_5_17;
        // D s_5_19: read-var i:i64
        let s_5_19: i64 = fn_state.i;
        // C s_5_20: const #1s : i64
        let s_5_20: i64 = 1;
        // D s_5_21: add s_5_19 s_5_20
        let s_5_21: i64 = (s_5_19 + s_5_20);
        // D s_5_22: write-var i <= s_5_21
        fn_state.i = s_5_21;
        // N s_5_23: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #2s : i
        let s_6_0: i128 = 2;
        // D s_6_1: read-var intval:u32
        let s_6_1: u32 = fn_state.intval;
        // D s_6_2: cast zx s_6_1 -> bv
        let s_6_2: Bits = Bits::new(s_6_1 as u128, 32u16);
        // D s_6_3: call ROL(s_6_2, s_6_0)
        let s_6_3: Bits = ROL(state, tracer, s_6_2, s_6_0);
        // D s_6_4: cast reint s_6_3 -> u32
        let s_6_4: u32 = (s_6_3.value() as u32);
        // D s_6_5: read-var intval:u32
        let s_6_5: u32 = fn_state.intval;
        // D s_6_6: cast zx s_6_5 -> bv
        let s_6_6: Bits = Bits::new(s_6_5 as u128, 32u16);
        // D s_6_7: cast zx s_6_4 -> bv
        let s_6_7: Bits = Bits::new(s_6_4 as u128, 32u16);
        // D s_6_8: xor s_6_6 s_6_7
        let s_6_8: Bits = ((s_6_6) ^ (s_6_7));
        // D s_6_9: cast reint s_6_8 -> u32
        let s_6_9: u32 = (s_6_8.value() as u32);
        // C s_6_10: const #10s : i
        let s_6_10: i128 = 10;
        // D s_6_11: read-var intval:u32
        let s_6_11: u32 = fn_state.intval;
        // D s_6_12: cast zx s_6_11 -> bv
        let s_6_12: Bits = Bits::new(s_6_11 as u128, 32u16);
        // D s_6_13: call ROL(s_6_12, s_6_10)
        let s_6_13: Bits = ROL(state, tracer, s_6_12, s_6_10);
        // D s_6_14: cast reint s_6_13 -> u32
        let s_6_14: u32 = (s_6_13.value() as u32);
        // D s_6_15: cast zx s_6_9 -> bv
        let s_6_15: Bits = Bits::new(s_6_9 as u128, 32u16);
        // D s_6_16: cast zx s_6_14 -> bv
        let s_6_16: Bits = Bits::new(s_6_14 as u128, 32u16);
        // D s_6_17: xor s_6_15 s_6_16
        let s_6_17: Bits = ((s_6_15) ^ (s_6_16));
        // D s_6_18: cast reint s_6_17 -> u32
        let s_6_18: u32 = (s_6_17.value() as u32);
        // C s_6_19: const #18s : i
        let s_6_19: i128 = 18;
        // D s_6_20: read-var intval:u32
        let s_6_20: u32 = fn_state.intval;
        // D s_6_21: cast zx s_6_20 -> bv
        let s_6_21: Bits = Bits::new(s_6_20 as u128, 32u16);
        // D s_6_22: call ROL(s_6_21, s_6_19)
        let s_6_22: Bits = ROL(state, tracer, s_6_21, s_6_19);
        // D s_6_23: cast reint s_6_22 -> u32
        let s_6_23: u32 = (s_6_22.value() as u32);
        // D s_6_24: cast zx s_6_18 -> bv
        let s_6_24: Bits = Bits::new(s_6_18 as u128, 32u16);
        // D s_6_25: cast zx s_6_23 -> bv
        let s_6_25: Bits = Bits::new(s_6_23 as u128, 32u16);
        // D s_6_26: xor s_6_24 s_6_25
        let s_6_26: Bits = ((s_6_24) ^ (s_6_25));
        // D s_6_27: cast reint s_6_26 -> u32
        let s_6_27: u32 = (s_6_26.value() as u32);
        // C s_6_28: const #24s : i
        let s_6_28: i128 = 24;
        // D s_6_29: read-var intval:u32
        let s_6_29: u32 = fn_state.intval;
        // D s_6_30: cast zx s_6_29 -> bv
        let s_6_30: Bits = Bits::new(s_6_29 as u128, 32u16);
        // D s_6_31: call ROL(s_6_30, s_6_28)
        let s_6_31: Bits = ROL(state, tracer, s_6_30, s_6_28);
        // D s_6_32: cast reint s_6_31 -> u32
        let s_6_32: u32 = (s_6_31.value() as u32);
        // D s_6_33: cast zx s_6_27 -> bv
        let s_6_33: Bits = Bits::new(s_6_27 as u128, 32u16);
        // D s_6_34: cast zx s_6_32 -> bv
        let s_6_34: Bits = Bits::new(s_6_32 as u128, 32u16);
        // D s_6_35: xor s_6_33 s_6_34
        let s_6_35: Bits = ((s_6_33) ^ (s_6_34));
        // D s_6_36: cast reint s_6_35 -> u32
        let s_6_36: u32 = (s_6_35.value() as u32);
        // D s_6_37: write-var intval <= s_6_36
        fn_state.intval = s_6_36;
        // C s_6_38: const #0s : i
        let s_6_38: i128 = 0;
        // D s_6_39: read-var roundresult:u128
        let s_6_39: u128 = fn_state.roundresult;
        // D s_6_40: cast zx s_6_39 -> bv
        let s_6_40: Bits = Bits::new(s_6_39 as u128, 128u16);
        // C s_6_41: const #1s : i64
        let s_6_41: i64 = 1;
        // C s_6_42: cast zx s_6_41 -> i
        let s_6_42: i128 = (i128::try_from(s_6_41).unwrap());
        // C s_6_43: const #31s : i
        let s_6_43: i128 = 31;
        // C s_6_44: add s_6_43 s_6_42
        let s_6_44: i128 = (s_6_43 + s_6_42);
        // D s_6_45: bit-extract s_6_40 s_6_38 s_6_44
        let s_6_45: Bits = (Bits::new(
            ((s_6_40) >> (s_6_38)).value(),
            u16::try_from(s_6_44).unwrap(),
        ));
        // D s_6_46: cast reint s_6_45 -> u32
        let s_6_46: u32 = (s_6_45.value() as u32);
        // D s_6_47: read-var intval:u32
        let s_6_47: u32 = fn_state.intval;
        // D s_6_48: cast zx s_6_47 -> bv
        let s_6_48: Bits = Bits::new(s_6_47 as u128, 32u16);
        // D s_6_49: cast zx s_6_46 -> bv
        let s_6_49: Bits = Bits::new(s_6_46 as u128, 32u16);
        // D s_6_50: xor s_6_48 s_6_49
        let s_6_50: Bits = ((s_6_48) ^ (s_6_49));
        // D s_6_51: cast reint s_6_50 -> u32
        let s_6_51: u32 = (s_6_50.value() as u32);
        // D s_6_52: write-var intval <= s_6_51
        fn_state.intval = s_6_51;
        // C s_6_53: const #32s : i
        let s_6_53: i128 = 32;
        // D s_6_54: read-var roundresult:u128
        let s_6_54: u128 = fn_state.roundresult;
        // D s_6_55: cast zx s_6_54 -> bv
        let s_6_55: Bits = Bits::new(s_6_54 as u128, 128u16);
        // C s_6_56: const #1s : i64
        let s_6_56: i64 = 1;
        // C s_6_57: cast zx s_6_56 -> i
        let s_6_57: i128 = (i128::try_from(s_6_56).unwrap());
        // C s_6_58: const #31s : i
        let s_6_58: i128 = 31;
        // C s_6_59: add s_6_58 s_6_57
        let s_6_59: i128 = (s_6_58 + s_6_57);
        // D s_6_60: bit-extract s_6_55 s_6_53 s_6_59
        let s_6_60: Bits = (Bits::new(
            ((s_6_55) >> (s_6_53)).value(),
            u16::try_from(s_6_59).unwrap(),
        ));
        // D s_6_61: cast reint s_6_60 -> u32
        let s_6_61: u32 = (s_6_60.value() as u32);
        // C s_6_62: const #0s : i
        let s_6_62: i128 = 0;
        // D s_6_63: read-var roundresult:u128
        let s_6_63: u128 = fn_state.roundresult;
        // D s_6_64: cast zx s_6_63 -> bv
        let s_6_64: Bits = Bits::new(s_6_63 as u128, 128u16);
        // D s_6_65: cast zx s_6_61 -> bv
        let s_6_65: Bits = Bits::new(s_6_61 as u128, 32u16);
        // C s_6_66: const #31s : i
        let s_6_66: i128 = 31;
        // C s_6_67: const #1u : u64
        let s_6_67: u64 = 1;
        // C s_6_68: cast zx s_6_67 -> bv
        let s_6_68: Bits = Bits::new(s_6_67 as u128, 64u16);
        // C s_6_69: lsl s_6_68 s_6_66
        let s_6_69: Bits = s_6_68 << s_6_66;
        // C s_6_70: sub s_6_69 s_6_68
        let s_6_70: Bits = ((s_6_69) - (s_6_68));
        // D s_6_71: and s_6_65 s_6_70
        let s_6_71: Bits = ((s_6_65) & (s_6_70));
        // D s_6_72: lsl s_6_71 s_6_62
        let s_6_72: Bits = s_6_71 << s_6_62;
        // C s_6_73: lsl s_6_70 s_6_62
        let s_6_73: Bits = s_6_70 << s_6_62;
        // C s_6_74: cmpl s_6_73
        let s_6_74: Bits = !s_6_73;
        // D s_6_75: and s_6_64 s_6_74
        let s_6_75: Bits = ((s_6_64) & (s_6_74));
        // D s_6_76: or s_6_75 s_6_72
        let s_6_76: Bits = ((s_6_75) | (s_6_72));
        // D s_6_77: cast reint s_6_76 -> u128
        let s_6_77: u128 = (s_6_76.value() as u128);
        // D s_6_78: write-var roundresult <= s_6_77
        fn_state.roundresult = s_6_77;
        // C s_6_79: const #64s : i
        let s_6_79: i128 = 64;
        // D s_6_80: read-var roundresult:u128
        let s_6_80: u128 = fn_state.roundresult;
        // D s_6_81: cast zx s_6_80 -> bv
        let s_6_81: Bits = Bits::new(s_6_80 as u128, 128u16);
        // C s_6_82: const #1s : i64
        let s_6_82: i64 = 1;
        // C s_6_83: cast zx s_6_82 -> i
        let s_6_83: i128 = (i128::try_from(s_6_82).unwrap());
        // C s_6_84: const #31s : i
        let s_6_84: i128 = 31;
        // C s_6_85: add s_6_84 s_6_83
        let s_6_85: i128 = (s_6_84 + s_6_83);
        // D s_6_86: bit-extract s_6_81 s_6_79 s_6_85
        let s_6_86: Bits = (Bits::new(
            ((s_6_81) >> (s_6_79)).value(),
            u16::try_from(s_6_85).unwrap(),
        ));
        // D s_6_87: cast reint s_6_86 -> u32
        let s_6_87: u32 = (s_6_86.value() as u32);
        // C s_6_88: const #32s : i
        let s_6_88: i128 = 32;
        // D s_6_89: read-var roundresult:u128
        let s_6_89: u128 = fn_state.roundresult;
        // D s_6_90: cast zx s_6_89 -> bv
        let s_6_90: Bits = Bits::new(s_6_89 as u128, 128u16);
        // D s_6_91: cast zx s_6_87 -> bv
        let s_6_91: Bits = Bits::new(s_6_87 as u128, 32u16);
        // C s_6_92: const #31s : i
        let s_6_92: i128 = 31;
        // C s_6_93: const #1u : u64
        let s_6_93: u64 = 1;
        // C s_6_94: cast zx s_6_93 -> bv
        let s_6_94: Bits = Bits::new(s_6_93 as u128, 64u16);
        // C s_6_95: lsl s_6_94 s_6_92
        let s_6_95: Bits = s_6_94 << s_6_92;
        // C s_6_96: sub s_6_95 s_6_94
        let s_6_96: Bits = ((s_6_95) - (s_6_94));
        // D s_6_97: and s_6_91 s_6_96
        let s_6_97: Bits = ((s_6_91) & (s_6_96));
        // D s_6_98: lsl s_6_97 s_6_88
        let s_6_98: Bits = s_6_97 << s_6_88;
        // C s_6_99: lsl s_6_96 s_6_88
        let s_6_99: Bits = s_6_96 << s_6_88;
        // C s_6_100: cmpl s_6_99
        let s_6_100: Bits = !s_6_99;
        // D s_6_101: and s_6_90 s_6_100
        let s_6_101: Bits = ((s_6_90) & (s_6_100));
        // D s_6_102: or s_6_101 s_6_98
        let s_6_102: Bits = ((s_6_101) | (s_6_98));
        // D s_6_103: cast reint s_6_102 -> u128
        let s_6_103: u128 = (s_6_102.value() as u128);
        // D s_6_104: write-var roundresult <= s_6_103
        fn_state.roundresult = s_6_103;
        // C s_6_105: const #96s : i
        let s_6_105: i128 = 96;
        // D s_6_106: read-var roundresult:u128
        let s_6_106: u128 = fn_state.roundresult;
        // D s_6_107: cast zx s_6_106 -> bv
        let s_6_107: Bits = Bits::new(s_6_106 as u128, 128u16);
        // C s_6_108: const #1s : i64
        let s_6_108: i64 = 1;
        // C s_6_109: cast zx s_6_108 -> i
        let s_6_109: i128 = (i128::try_from(s_6_108).unwrap());
        // C s_6_110: const #31s : i
        let s_6_110: i128 = 31;
        // C s_6_111: add s_6_110 s_6_109
        let s_6_111: i128 = (s_6_110 + s_6_109);
        // D s_6_112: bit-extract s_6_107 s_6_105 s_6_111
        let s_6_112: Bits = (Bits::new(
            ((s_6_107) >> (s_6_105)).value(),
            u16::try_from(s_6_111).unwrap(),
        ));
        // D s_6_113: cast reint s_6_112 -> u32
        let s_6_113: u32 = (s_6_112.value() as u32);
        // C s_6_114: const #64s : i
        let s_6_114: i128 = 64;
        // D s_6_115: read-var roundresult:u128
        let s_6_115: u128 = fn_state.roundresult;
        // D s_6_116: cast zx s_6_115 -> bv
        let s_6_116: Bits = Bits::new(s_6_115 as u128, 128u16);
        // D s_6_117: cast zx s_6_113 -> bv
        let s_6_117: Bits = Bits::new(s_6_113 as u128, 32u16);
        // C s_6_118: const #31s : i
        let s_6_118: i128 = 31;
        // C s_6_119: const #1u : u64
        let s_6_119: u64 = 1;
        // C s_6_120: cast zx s_6_119 -> bv
        let s_6_120: Bits = Bits::new(s_6_119 as u128, 64u16);
        // C s_6_121: lsl s_6_120 s_6_118
        let s_6_121: Bits = s_6_120 << s_6_118;
        // C s_6_122: sub s_6_121 s_6_120
        let s_6_122: Bits = ((s_6_121) - (s_6_120));
        // D s_6_123: and s_6_117 s_6_122
        let s_6_123: Bits = ((s_6_117) & (s_6_122));
        // D s_6_124: lsl s_6_123 s_6_114
        let s_6_124: Bits = s_6_123 << s_6_114;
        // C s_6_125: lsl s_6_122 s_6_114
        let s_6_125: Bits = s_6_122 << s_6_114;
        // C s_6_126: cmpl s_6_125
        let s_6_126: Bits = !s_6_125;
        // D s_6_127: and s_6_116 s_6_126
        let s_6_127: Bits = ((s_6_116) & (s_6_126));
        // D s_6_128: or s_6_127 s_6_124
        let s_6_128: Bits = ((s_6_127) | (s_6_124));
        // D s_6_129: cast reint s_6_128 -> u128
        let s_6_129: u128 = (s_6_128.value() as u128);
        // D s_6_130: write-var roundresult <= s_6_129
        fn_state.roundresult = s_6_129;
        // C s_6_131: const #96s : i
        let s_6_131: i128 = 96;
        // D s_6_132: read-var roundresult:u128
        let s_6_132: u128 = fn_state.roundresult;
        // D s_6_133: cast zx s_6_132 -> bv
        let s_6_133: Bits = Bits::new(s_6_132 as u128, 128u16);
        // D s_6_134: read-var intval:u32
        let s_6_134: u32 = fn_state.intval;
        // D s_6_135: cast zx s_6_134 -> bv
        let s_6_135: Bits = Bits::new(s_6_134 as u128, 32u16);
        // C s_6_136: const #31s : i
        let s_6_136: i128 = 31;
        // C s_6_137: const #1u : u64
        let s_6_137: u64 = 1;
        // C s_6_138: cast zx s_6_137 -> bv
        let s_6_138: Bits = Bits::new(s_6_137 as u128, 64u16);
        // C s_6_139: lsl s_6_138 s_6_136
        let s_6_139: Bits = s_6_138 << s_6_136;
        // C s_6_140: sub s_6_139 s_6_138
        let s_6_140: Bits = ((s_6_139) - (s_6_138));
        // D s_6_141: and s_6_135 s_6_140
        let s_6_141: Bits = ((s_6_135) & (s_6_140));
        // D s_6_142: lsl s_6_141 s_6_131
        let s_6_142: Bits = s_6_141 << s_6_131;
        // C s_6_143: lsl s_6_140 s_6_131
        let s_6_143: Bits = s_6_140 << s_6_131;
        // C s_6_144: cmpl s_6_143
        let s_6_144: Bits = !s_6_143;
        // D s_6_145: and s_6_133 s_6_144
        let s_6_145: Bits = ((s_6_133) & (s_6_144));
        // D s_6_146: or s_6_145 s_6_142
        let s_6_146: Bits = ((s_6_145) | (s_6_142));
        // D s_6_147: cast reint s_6_146 -> u128
        let s_6_147: u128 = (s_6_146.value() as u128);
        // D s_6_148: write-var roundresult <= s_6_147
        fn_state.roundresult = s_6_147;
        // D s_6_149: read-var index:i64
        let s_6_149: i64 = fn_state.index;
        // C s_6_150: const #1s : i64
        let s_6_150: i64 = 1;
        // D s_6_151: add s_6_149 s_6_150
        let s_6_151: i64 = (s_6_149 + s_6_150);
        // D s_6_152: write-var index <= s_6_151
        fn_state.index = s_6_151;
        // N s_6_153: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #128s : i64
        let s_7_0: i64 = 128;
        // D s_7_1: read-var d:i64
        let s_7_1: i64 = fn_state.d;
        // D s_7_2: cast zx s_7_1 -> i
        let s_7_2: i128 = (i128::try_from(s_7_1).unwrap());
        // D s_7_3: read-var roundresult:u128
        let s_7_3: u128 = fn_state.roundresult;
        // D s_7_4: cast zx s_7_3 -> bv
        let s_7_4: Bits = Bits::new(s_7_3 as u128, 128u16);
        // D s_7_5: call V_set(s_7_2, s_7_0, s_7_4)
        let s_7_5: () = V_set(state, tracer, s_7_2, s_7_0, s_7_4);
        // N s_7_6: return
        return;
    }
}
