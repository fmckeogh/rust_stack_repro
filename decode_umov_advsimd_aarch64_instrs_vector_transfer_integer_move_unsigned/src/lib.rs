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
use unsigned_subrange::*;
use execute_aarch64_instrs_vector_transfer_integer_move_unsigned::*;
use common::*;
pub fn decode_umov_advsimd_aarch64_instrs_vector_transfer_integer_move_unsigned<
    T: Tracer,
>(state: &mut State, tracer: &T, Rd: u8, Rn: u8, imm5: u8, Q: bool) -> () {
    #[derive(Default)]
    struct FunctionState {
        esize: i64,
        gs_174174: bool,
        n: i64,
        index: i64,
        gs_174188: bool,
        b__0: u8,
        b__1: u8,
        idxdsize: i64,
        gs_174167: bool,
        ga_270102: u8,
        ga_270119: i64,
        b__2: u8,
        d: i64,
        size: i64,
        b__3: u8,
        gs_174181: bool,
        sizeshadow_2009: i64,
        ga_270124: i64,
        Rd: u8,
        Rn: u8,
        imm5: u8,
        Q: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        imm5,
        Q,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var Rd:u8
        let s_0_0: u8 = fn_state.Rd;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 5u16);
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (s_0_1.value() as i128);
        // D s_0_3: cast reint s_0_2 -> i64
        let s_0_3: i64 = (s_0_2 as i64);
        // D s_0_4: write-var d <= s_0_3
        fn_state.d = s_0_3;
        // D s_0_5: read-var Rn:u8
        let s_0_5: u8 = fn_state.Rn;
        // D s_0_6: cast zx s_0_5 -> bv
        let s_0_6: Bits = Bits::new(s_0_5 as u128, 5u16);
        // D s_0_7: cast zx s_0_6 -> i
        let s_0_7: i128 = (s_0_6.value() as i128);
        // D s_0_8: cast reint s_0_7 -> i64
        let s_0_8: i64 = (s_0_7 as i64);
        // D s_0_9: write-var n <= s_0_8
        fn_state.n = s_0_8;
        // C s_0_10: const #0s : i64
        let s_0_10: i64 = 0;
        // D s_0_11: write-var size <= s_0_10
        fn_state.size = s_0_10;
        // D s_0_12: read-var Q:u8
        let s_0_12: bool = fn_state.Q;
        // D s_0_13: cast zx s_0_12 -> bv
        let s_0_13: Bits = Bits::new(s_0_12 as u128, 1u16);
        // D s_0_14: read-var imm5:u8
        let s_0_14: u8 = fn_state.imm5;
        // D s_0_15: cast zx s_0_14 -> bv
        let s_0_15: Bits = Bits::new(s_0_14 as u128, 5u16);
        // D s_0_16: cast reint s_0_13 -> u128
        let s_0_16: u128 = (s_0_13.value() as u128);
        // D s_0_17: size-of s_0_13
        let s_0_17: u16 = s_0_13.length();
        // D s_0_18: cast reint s_0_15 -> u128
        let s_0_18: u128 = (s_0_15.value() as u128);
        // D s_0_19: size-of s_0_15
        let s_0_19: u16 = s_0_15.length();
        // D s_0_20: lsl s_0_16 s_0_19
        let s_0_20: u128 = s_0_16 << s_0_19;
        // D s_0_21: or s_0_20 s_0_18
        let s_0_21: u128 = ((s_0_20) | (s_0_18));
        // D s_0_22: add s_0_17 s_0_19
        let s_0_22: u16 = (s_0_17 + s_0_19);
        // D s_0_23: create-bits s_0_21 s_0_22
        let s_0_23: Bits = Bits::new(s_0_21, s_0_22);
        // D s_0_24: cast reint s_0_23 -> u8
        let s_0_24: u8 = (s_0_23.value() as u8);
        // D s_0_25: write-var ga#270102 <= s_0_24
        fn_state.ga_270102 = s_0_24;
        // D s_0_26: read-var ga#270102:u8
        let s_0_26: u8 = fn_state.ga_270102;
        // D s_0_27: write-var b__0 <= s_0_26
        fn_state.b__0 = s_0_26;
        // C s_0_28: const #5s : i
        let s_0_28: i128 = 5;
        // D s_0_29: read-var b__0:u8
        let s_0_29: u8 = fn_state.b__0;
        // D s_0_30: cast zx s_0_29 -> bv
        let s_0_30: Bits = Bits::new(s_0_29 as u128, 6u16);
        // C s_0_31: const #1s : i64
        let s_0_31: i64 = 1;
        // C s_0_32: cast zx s_0_31 -> i
        let s_0_32: i128 = (i128::try_from(s_0_31).unwrap());
        // C s_0_33: const #0s : i
        let s_0_33: i128 = 0;
        // C s_0_34: add s_0_33 s_0_32
        let s_0_34: i128 = (s_0_33 + s_0_32);
        // D s_0_35: bit-extract s_0_30 s_0_28 s_0_34
        let s_0_35: Bits = (Bits::new(
            ((s_0_30) >> (s_0_28)).value(),
            u16::try_from(s_0_34).unwrap(),
        ));
        // D s_0_36: cast reint s_0_35 -> u8
        let s_0_36: bool = ((s_0_35.value()) != 0);
        // D s_0_37: cast zx s_0_36 -> bv
        let s_0_37: Bits = Bits::new(s_0_36 as u128, 1u16);
        // C s_0_38: const #0u : u8
        let s_0_38: bool = false;
        // C s_0_39: cast zx s_0_38 -> bv
        let s_0_39: Bits = Bits::new(s_0_38 as u128, 1u16);
        // D s_0_40: cmp-eq s_0_37 s_0_39
        let s_0_40: bool = ((s_0_37) == (s_0_39));
        // N s_0_41: branch s_0_40 b27 b1
        if s_0_40 {
            return block_27(state, tracer, fn_state);
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
        // D s_1_1: write-var gs#174167 <= s_1_0
        fn_state.gs_174167 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#174167:u8
        let s_2_0: bool = fn_state.gs_174167;
        // D s_2_1: not s_2_0
        let s_2_1: bool = !s_2_0;
        // N s_2_2: branch s_2_1 b11 b3
        if s_2_1 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0s : i64
        let s_3_0: i64 = 0;
        // D s_3_1: write-var size <= s_3_0
        fn_state.size = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var size:i64
        let s_4_0: i64 = fn_state.size;
        // D s_4_1: write-var sizeshadow#2009 <= s_4_0
        fn_state.sizeshadow_2009 = s_4_0;
        // C s_4_2: const #4s : i
        let s_4_2: i128 = 4;
        // D s_4_3: read-var imm5:u8
        let s_4_3: u8 = fn_state.imm5;
        // D s_4_4: cast zx s_4_3 -> bv
        let s_4_4: Bits = Bits::new(s_4_3 as u128, 5u16);
        // C s_4_5: const #1u : u64
        let s_4_5: u64 = 1;
        // D s_4_6: bit-extract s_4_4 s_4_2 s_4_5
        let s_4_6: Bits = (Bits::new(
            ((s_4_4) >> (s_4_2)).value(),
            u16::try_from(s_4_5).unwrap(),
        ));
        // D s_4_7: cast reint s_4_6 -> u8
        let s_4_7: bool = ((s_4_6.value()) != 0);
        // C s_4_8: const #0s : i
        let s_4_8: i128 = 0;
        // C s_4_9: const #0u : u64
        let s_4_9: u64 = 0;
        // D s_4_10: cast zx s_4_7 -> u64
        let s_4_10: u64 = (s_4_7 as u64);
        // C s_4_11: const #1u : u64
        let s_4_11: u64 = 1;
        // D s_4_12: and s_4_10 s_4_11
        let s_4_12: u64 = ((s_4_10) & (s_4_11));
        // D s_4_13: cmp-eq s_4_12 s_4_11
        let s_4_13: bool = ((s_4_12) == (s_4_11));
        // D s_4_14: lsl s_4_10 s_4_8
        let s_4_14: u64 = s_4_10 << s_4_8;
        // D s_4_15: or s_4_9 s_4_14
        let s_4_15: u64 = ((s_4_9) | (s_4_14));
        // D s_4_16: cmpl s_4_14
        let s_4_16: u64 = !s_4_14;
        // D s_4_17: and s_4_9 s_4_16
        let s_4_17: u64 = ((s_4_9) & (s_4_16));
        // D s_4_18: select s_4_13 s_4_15 s_4_17
        let s_4_18: u64 = if s_4_13 { s_4_15 } else { s_4_17 };
        // D s_4_19: cast trunc s_4_18 -> u8
        let s_4_19: bool = ((s_4_18) != 0);
        // D s_4_20: cast zx s_4_19 -> bv
        let s_4_20: Bits = Bits::new(s_4_19 as u128, 1u16);
        // C s_4_21: const #1u : u8
        let s_4_21: bool = true;
        // C s_4_22: cast zx s_4_21 -> bv
        let s_4_22: Bits = Bits::new(s_4_21 as u128, 1u16);
        // D s_4_23: cmp-eq s_4_20 s_4_22
        let s_4_23: bool = ((s_4_20) == (s_4_22));
        // N s_4_24: branch s_4_23 b10 b5
        if s_4_23 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #64s : i64
        let s_5_0: i64 = 64;
        // D s_5_1: write-var ga#270119 <= s_5_0
        fn_state.ga_270119 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var ga#270119:i64
        let s_6_0: i64 = fn_state.ga_270119;
        // D s_6_1: write-var idxdsize <= s_6_0
        fn_state.idxdsize = s_6_0;
        // C s_6_2: const #1s : i
        let s_6_2: i128 = 1;
        // D s_6_3: read-var sizeshadow#2009:i64
        let s_6_3: i64 = fn_state.sizeshadow_2009;
        // D s_6_4: cast zx s_6_3 -> i
        let s_6_4: i128 = (i128::try_from(s_6_3).unwrap());
        // D s_6_5: add s_6_4 s_6_2
        let s_6_5: i128 = (s_6_4 + s_6_2);
        // D s_6_6: cast reint s_6_5 -> i64
        let s_6_6: i64 = (s_6_5 as i64);
        // C s_6_7: const #4s : i
        let s_6_7: i128 = 4;
        // D s_6_8: read-var imm5:u8
        let s_6_8: u8 = fn_state.imm5;
        // D s_6_9: cast zx s_6_8 -> bv
        let s_6_9: Bits = Bits::new(s_6_8 as u128, 5u16);
        // D s_6_10: cast zx s_6_6 -> i
        let s_6_10: i128 = (i128::try_from(s_6_6).unwrap());
        // D s_6_11: call unsigned_subrange(s_6_9, s_6_7, s_6_10)
        let s_6_11: i128 = unsigned_subrange(state, tracer, s_6_9, s_6_7, s_6_10);
        // D s_6_12: cast reint s_6_11 -> i64
        let s_6_12: i64 = (s_6_11 as i64);
        // D s_6_13: write-var index <= s_6_12
        fn_state.index = s_6_12;
        // C s_6_14: const #8s : i64
        let s_6_14: i64 = 8;
        // D s_6_15: read-var sizeshadow#2009:i64
        let s_6_15: i64 = fn_state.sizeshadow_2009;
        // D s_6_16: lsl s_6_14 s_6_15
        let s_6_16: i64 = s_6_14 << s_6_15;
        // D s_6_17: write-var esize <= s_6_16
        fn_state.esize = s_6_16;
        // D s_6_18: read-var Q:u8
        let s_6_18: bool = fn_state.Q;
        // D s_6_19: cast zx s_6_18 -> bv
        let s_6_19: Bits = Bits::new(s_6_18 as u128, 1u16);
        // C s_6_20: const #1u : u8
        let s_6_20: bool = true;
        // C s_6_21: cast zx s_6_20 -> bv
        let s_6_21: Bits = Bits::new(s_6_20 as u128, 1u16);
        // D s_6_22: cmp-eq s_6_19 s_6_21
        let s_6_22: bool = ((s_6_19) == (s_6_21));
        // N s_6_23: branch s_6_22 b9 b7
        if s_6_22 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_7_0: const #32s : i64
        let s_7_0: i64 = 32;
        // D s_7_1: write-var ga#270124 <= s_7_0
        fn_state.ga_270124 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var ga#270124:i64
        let s_8_0: i64 = fn_state.ga_270124;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: cast reint s_8_1 -> i64
        let s_8_2: i64 = (s_8_1 as i64);
        // D s_8_3: read-var esize:i64
        let s_8_3: i64 = fn_state.esize;
        // D s_8_4: cast zx s_8_3 -> i
        let s_8_4: i128 = (i128::try_from(s_8_3).unwrap());
        // D s_8_5: cast reint s_8_4 -> i64
        let s_8_5: i64 = (s_8_4 as i64);
        // D s_8_6: read-var idxdsize:i64
        let s_8_6: i64 = fn_state.idxdsize;
        // D s_8_7: cast zx s_8_6 -> i
        let s_8_7: i128 = (i128::try_from(s_8_6).unwrap());
        // D s_8_8: cast reint s_8_7 -> i64
        let s_8_8: i64 = (s_8_7 as i64);
        // D s_8_9: read-var index:i64
        let s_8_9: i64 = fn_state.index;
        // D s_8_10: cast zx s_8_9 -> i
        let s_8_10: i128 = (i128::try_from(s_8_9).unwrap());
        // D s_8_11: read-var d:i64
        let s_8_11: i64 = fn_state.d;
        // D s_8_12: read-var n:i64
        let s_8_12: i64 = fn_state.n;
        // D s_8_13: call execute_aarch64_instrs_vector_transfer_integer_move_unsigned(s_8_11, s_8_2, s_8_5, s_8_8, s_8_10, s_8_12)
        let s_8_13: () = execute_aarch64_instrs_vector_transfer_integer_move_unsigned(
            state,
            tracer,
            s_8_11,
            s_8_2,
            s_8_5,
            s_8_8,
            s_8_10,
            s_8_12,
        );
        // N s_8_14: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #64s : i64
        let s_9_0: i64 = 64;
        // D s_9_1: write-var ga#270124 <= s_9_0
        fn_state.ga_270124 = s_9_0;
        // N s_9_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #128s : i64
        let s_10_0: i64 = 128;
        // D s_10_1: write-var ga#270119 <= s_10_0
        fn_state.ga_270119 = s_10_0;
        // N s_10_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var ga#270102:u8
        let s_11_0: u8 = fn_state.ga_270102;
        // D s_11_1: write-var b__1 <= s_11_0
        fn_state.b__1 = s_11_0;
        // C s_11_2: const #5s : i
        let s_11_2: i128 = 5;
        // D s_11_3: read-var b__1:u8
        let s_11_3: u8 = fn_state.b__1;
        // D s_11_4: cast zx s_11_3 -> bv
        let s_11_4: Bits = Bits::new(s_11_3 as u128, 6u16);
        // C s_11_5: const #1s : i64
        let s_11_5: i64 = 1;
        // C s_11_6: cast zx s_11_5 -> i
        let s_11_6: i128 = (i128::try_from(s_11_5).unwrap());
        // C s_11_7: const #0s : i
        let s_11_7: i128 = 0;
        // C s_11_8: add s_11_7 s_11_6
        let s_11_8: i128 = (s_11_7 + s_11_6);
        // D s_11_9: bit-extract s_11_4 s_11_2 s_11_8
        let s_11_9: Bits = (Bits::new(
            ((s_11_4) >> (s_11_2)).value(),
            u16::try_from(s_11_8).unwrap(),
        ));
        // D s_11_10: cast reint s_11_9 -> u8
        let s_11_10: bool = ((s_11_9.value()) != 0);
        // D s_11_11: cast zx s_11_10 -> bv
        let s_11_11: Bits = Bits::new(s_11_10 as u128, 1u16);
        // C s_11_12: const #0u : u8
        let s_11_12: bool = false;
        // C s_11_13: cast zx s_11_12 -> bv
        let s_11_13: Bits = Bits::new(s_11_12 as u128, 1u16);
        // D s_11_14: cmp-eq s_11_11 s_11_13
        let s_11_14: bool = ((s_11_11) == (s_11_13));
        // N s_11_15: branch s_11_14 b26 b12
        if s_11_14 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var gs#174174 <= s_12_0
        fn_state.gs_174174 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var gs#174174:u8
        let s_13_0: bool = fn_state.gs_174174;
        // D s_13_1: not s_13_0
        let s_13_1: bool = !s_13_0;
        // N s_13_2: branch s_13_1 b15 b14
        if s_13_1 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #1s : i64
        let s_14_0: i64 = 1;
        // D s_14_1: write-var size <= s_14_0
        fn_state.size = s_14_0;
        // N s_14_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var ga#270102:u8
        let s_15_0: u8 = fn_state.ga_270102;
        // D s_15_1: write-var b__2 <= s_15_0
        fn_state.b__2 = s_15_0;
        // C s_15_2: const #5s : i
        let s_15_2: i128 = 5;
        // D s_15_3: read-var b__2:u8
        let s_15_3: u8 = fn_state.b__2;
        // D s_15_4: cast zx s_15_3 -> bv
        let s_15_4: Bits = Bits::new(s_15_3 as u128, 6u16);
        // C s_15_5: const #1s : i64
        let s_15_5: i64 = 1;
        // C s_15_6: cast zx s_15_5 -> i
        let s_15_6: i128 = (i128::try_from(s_15_5).unwrap());
        // C s_15_7: const #0s : i
        let s_15_7: i128 = 0;
        // C s_15_8: add s_15_7 s_15_6
        let s_15_8: i128 = (s_15_7 + s_15_6);
        // D s_15_9: bit-extract s_15_4 s_15_2 s_15_8
        let s_15_9: Bits = (Bits::new(
            ((s_15_4) >> (s_15_2)).value(),
            u16::try_from(s_15_8).unwrap(),
        ));
        // D s_15_10: cast reint s_15_9 -> u8
        let s_15_10: bool = ((s_15_9.value()) != 0);
        // D s_15_11: cast zx s_15_10 -> bv
        let s_15_11: Bits = Bits::new(s_15_10 as u128, 1u16);
        // C s_15_12: const #0u : u8
        let s_15_12: bool = false;
        // C s_15_13: cast zx s_15_12 -> bv
        let s_15_13: Bits = Bits::new(s_15_12 as u128, 1u16);
        // D s_15_14: cmp-eq s_15_11 s_15_13
        let s_15_14: bool = ((s_15_11) == (s_15_13));
        // N s_15_15: branch s_15_14 b25 b16
        if s_15_14 {
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
        // D s_16_1: write-var gs#174181 <= s_16_0
        fn_state.gs_174181 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#174181:u8
        let s_17_0: bool = fn_state.gs_174181;
        // D s_17_1: not s_17_0
        let s_17_1: bool = !s_17_0;
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
        // C s_18_0: const #2s : i64
        let s_18_0: i64 = 2;
        // D s_18_1: write-var size <= s_18_0
        fn_state.size = s_18_0;
        // N s_18_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var ga#270102:u8
        let s_19_0: u8 = fn_state.ga_270102;
        // D s_19_1: write-var b__3 <= s_19_0
        fn_state.b__3 = s_19_0;
        // C s_19_2: const #5s : i
        let s_19_2: i128 = 5;
        // D s_19_3: read-var b__3:u8
        let s_19_3: u8 = fn_state.b__3;
        // D s_19_4: cast zx s_19_3 -> bv
        let s_19_4: Bits = Bits::new(s_19_3 as u128, 6u16);
        // C s_19_5: const #1s : i64
        let s_19_5: i64 = 1;
        // C s_19_6: cast zx s_19_5 -> i
        let s_19_6: i128 = (i128::try_from(s_19_5).unwrap());
        // C s_19_7: const #0s : i
        let s_19_7: i128 = 0;
        // C s_19_8: add s_19_7 s_19_6
        let s_19_8: i128 = (s_19_7 + s_19_6);
        // D s_19_9: bit-extract s_19_4 s_19_2 s_19_8
        let s_19_9: Bits = (Bits::new(
            ((s_19_4) >> (s_19_2)).value(),
            u16::try_from(s_19_8).unwrap(),
        ));
        // D s_19_10: cast reint s_19_9 -> u8
        let s_19_10: bool = ((s_19_9.value()) != 0);
        // D s_19_11: cast zx s_19_10 -> bv
        let s_19_11: Bits = Bits::new(s_19_10 as u128, 1u16);
        // C s_19_12: const #1u : u8
        let s_19_12: bool = true;
        // C s_19_13: cast zx s_19_12 -> bv
        let s_19_13: Bits = Bits::new(s_19_12 as u128, 1u16);
        // D s_19_14: cmp-eq s_19_11 s_19_13
        let s_19_14: bool = ((s_19_11) == (s_19_13));
        // N s_19_15: branch s_19_14 b24 b20
        if s_19_14 {
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
        // D s_20_1: write-var gs#174188 <= s_20_0
        fn_state.gs_174188 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#174188:u8
        let s_21_0: bool = fn_state.gs_174188;
        // D s_21_1: not s_21_0
        let s_21_1: bool = !s_21_0;
        // N s_21_2: branch s_21_1 b23 b22
        if s_21_1 {
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
        // C s_22_0: const #3s : i64
        let s_22_0: i64 = 3;
        // D s_22_1: write-var size <= s_22_0
        fn_state.size = s_22_0;
        // N s_22_2: jump b4
        return block_4(state, tracer, fn_state);
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
        // C s_24_0: const #0s : i
        let s_24_0: i128 = 0;
        // D s_24_1: read-var b__3:u8
        let s_24_1: u8 = fn_state.b__3;
        // D s_24_2: cast zx s_24_1 -> bv
        let s_24_2: Bits = Bits::new(s_24_1 as u128, 6u16);
        // C s_24_3: const #1s : i64
        let s_24_3: i64 = 1;
        // C s_24_4: cast zx s_24_3 -> i
        let s_24_4: i128 = (i128::try_from(s_24_3).unwrap());
        // C s_24_5: const #3s : i
        let s_24_5: i128 = 3;
        // C s_24_6: add s_24_5 s_24_4
        let s_24_6: i128 = (s_24_5 + s_24_4);
        // D s_24_7: bit-extract s_24_2 s_24_0 s_24_6
        let s_24_7: Bits = (Bits::new(
            ((s_24_2) >> (s_24_0)).value(),
            u16::try_from(s_24_6).unwrap(),
        ));
        // D s_24_8: cast reint s_24_7 -> u8
        let s_24_8: u8 = (s_24_7.value() as u8);
        // D s_24_9: cast zx s_24_8 -> bv
        let s_24_9: Bits = Bits::new(s_24_8 as u128, 4u16);
        // C s_24_10: const #8u : u8
        let s_24_10: u8 = 8;
        // C s_24_11: cast zx s_24_10 -> bv
        let s_24_11: Bits = Bits::new(s_24_10 as u128, 4u16);
        // D s_24_12: cmp-eq s_24_9 s_24_11
        let s_24_12: bool = ((s_24_9) == (s_24_11));
        // D s_24_13: write-var gs#174188 <= s_24_12
        fn_state.gs_174188 = s_24_12;
        // N s_24_14: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_25_0: const #0s : i
        let s_25_0: i128 = 0;
        // D s_25_1: read-var b__2:u8
        let s_25_1: u8 = fn_state.b__2;
        // D s_25_2: cast zx s_25_1 -> bv
        let s_25_2: Bits = Bits::new(s_25_1 as u128, 6u16);
        // C s_25_3: const #1s : i64
        let s_25_3: i64 = 1;
        // C s_25_4: cast zx s_25_3 -> i
        let s_25_4: i128 = (i128::try_from(s_25_3).unwrap());
        // C s_25_5: const #2s : i
        let s_25_5: i128 = 2;
        // C s_25_6: add s_25_5 s_25_4
        let s_25_6: i128 = (s_25_5 + s_25_4);
        // D s_25_7: bit-extract s_25_2 s_25_0 s_25_6
        let s_25_7: Bits = (Bits::new(
            ((s_25_2) >> (s_25_0)).value(),
            u16::try_from(s_25_6).unwrap(),
        ));
        // D s_25_8: cast reint s_25_7 -> u8
        let s_25_8: u8 = (s_25_7.value() as u8);
        // D s_25_9: cast zx s_25_8 -> bv
        let s_25_9: Bits = Bits::new(s_25_8 as u128, 3u16);
        // C s_25_10: const #4u : u8
        let s_25_10: u8 = 4;
        // C s_25_11: cast zx s_25_10 -> bv
        let s_25_11: Bits = Bits::new(s_25_10 as u128, 3u16);
        // D s_25_12: cmp-eq s_25_9 s_25_11
        let s_25_12: bool = ((s_25_9) == (s_25_11));
        // D s_25_13: write-var gs#174181 <= s_25_12
        fn_state.gs_174181 = s_25_12;
        // N s_25_14: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #0s : i
        let s_26_0: i128 = 0;
        // D s_26_1: read-var b__1:u8
        let s_26_1: u8 = fn_state.b__1;
        // D s_26_2: cast zx s_26_1 -> bv
        let s_26_2: Bits = Bits::new(s_26_1 as u128, 6u16);
        // C s_26_3: const #1s : i64
        let s_26_3: i64 = 1;
        // C s_26_4: cast zx s_26_3 -> i
        let s_26_4: i128 = (i128::try_from(s_26_3).unwrap());
        // C s_26_5: const #1s : i
        let s_26_5: i128 = 1;
        // C s_26_6: add s_26_5 s_26_4
        let s_26_6: i128 = (s_26_5 + s_26_4);
        // D s_26_7: bit-extract s_26_2 s_26_0 s_26_6
        let s_26_7: Bits = (Bits::new(
            ((s_26_2) >> (s_26_0)).value(),
            u16::try_from(s_26_6).unwrap(),
        ));
        // D s_26_8: cast reint s_26_7 -> u8
        let s_26_8: u8 = (s_26_7.value() as u8);
        // D s_26_9: cast zx s_26_8 -> bv
        let s_26_9: Bits = Bits::new(s_26_8 as u128, 2u16);
        // C s_26_10: const #2u : u8
        let s_26_10: u8 = 2;
        // C s_26_11: cast zx s_26_10 -> bv
        let s_26_11: Bits = Bits::new(s_26_10 as u128, 2u16);
        // D s_26_12: cmp-eq s_26_9 s_26_11
        let s_26_12: bool = ((s_26_9) == (s_26_11));
        // D s_26_13: write-var gs#174174 <= s_26_12
        fn_state.gs_174174 = s_26_12;
        // N s_26_14: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #0s : i
        let s_27_0: i128 = 0;
        // D s_27_1: read-var b__0:u8
        let s_27_1: u8 = fn_state.b__0;
        // D s_27_2: cast zx s_27_1 -> bv
        let s_27_2: Bits = Bits::new(s_27_1 as u128, 6u16);
        // C s_27_3: const #1s : i64
        let s_27_3: i64 = 1;
        // C s_27_4: cast zx s_27_3 -> i
        let s_27_4: i128 = (i128::try_from(s_27_3).unwrap());
        // C s_27_5: const #0s : i
        let s_27_5: i128 = 0;
        // C s_27_6: add s_27_5 s_27_4
        let s_27_6: i128 = (s_27_5 + s_27_4);
        // D s_27_7: bit-extract s_27_2 s_27_0 s_27_6
        let s_27_7: Bits = (Bits::new(
            ((s_27_2) >> (s_27_0)).value(),
            u16::try_from(s_27_6).unwrap(),
        ));
        // D s_27_8: cast reint s_27_7 -> u8
        let s_27_8: bool = ((s_27_7.value()) != 0);
        // D s_27_9: cast zx s_27_8 -> bv
        let s_27_9: Bits = Bits::new(s_27_8 as u128, 1u16);
        // C s_27_10: const #1u : u8
        let s_27_10: bool = true;
        // C s_27_11: cast zx s_27_10 -> bv
        let s_27_11: Bits = Bits::new(s_27_10 as u128, 1u16);
        // D s_27_12: cmp-eq s_27_9 s_27_11
        let s_27_12: bool = ((s_27_9) == (s_27_11));
        // D s_27_13: write-var gs#174167 <= s_27_12
        fn_state.gs_174167 = s_27_12;
        // N s_27_14: jump b2
        return block_2(state, tracer, fn_state);
    }
}
