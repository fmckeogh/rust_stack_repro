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
use execute_aarch64_instrs_vector_transfer_integer_move_signed::*;
use u__id::*;
use common::*;
pub fn decode_smov_advsimd_aarch64_instrs_vector_transfer_integer_move_signed<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rd: u8,
    Rn: u8,
    imm5: u8,
    Q: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        esize: i64,
        ga_267021: i64,
        n: i64,
        index: i64,
        gs_170212: bool,
        gs_170214: bool,
        sizeshadow_1887: i64,
        idxdsize: i64,
        gs_170191: bool,
        b__2: u8,
        d: i64,
        size: i64,
        ga_267026: i64,
        datasize: i64,
        ga_267011: u8,
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
        // D s_0_25: write-var ga#267011 <= s_0_24
        fn_state.ga_267011 = s_0_24;
        // D s_0_26: read-var ga#267011:u8
        let s_0_26: u8 = fn_state.ga_267011;
        // C s_0_27: const #0s : i
        let s_0_27: i128 = 0;
        // D s_0_28: cast zx s_0_26 -> bv
        let s_0_28: Bits = Bits::new(s_0_26 as u128, 6u16);
        // C s_0_29: const #1s : i64
        let s_0_29: i64 = 1;
        // C s_0_30: cast zx s_0_29 -> i
        let s_0_30: i128 = (i128::try_from(s_0_29).unwrap());
        // C s_0_31: const #0s : i
        let s_0_31: i128 = 0;
        // C s_0_32: add s_0_31 s_0_30
        let s_0_32: i128 = (s_0_31 + s_0_30);
        // D s_0_33: bit-extract s_0_28 s_0_27 s_0_32
        let s_0_33: Bits = (Bits::new(
            ((s_0_28) >> (s_0_27)).value(),
            u16::try_from(s_0_32).unwrap(),
        ));
        // D s_0_34: cast reint s_0_33 -> u8
        let s_0_34: bool = ((s_0_33.value()) != 0);
        // D s_0_35: cast zx s_0_34 -> bv
        let s_0_35: Bits = Bits::new(s_0_34 as u128, 1u16);
        // C s_0_36: const #1u : u8
        let s_0_36: bool = true;
        // C s_0_37: cast zx s_0_36 -> bv
        let s_0_37: Bits = Bits::new(s_0_36 as u128, 1u16);
        // D s_0_38: cmp-eq s_0_35 s_0_37
        let s_0_38: bool = ((s_0_35) == (s_0_37));
        // D s_0_39: not s_0_38
        let s_0_39: bool = !s_0_38;
        // N s_0_40: branch s_0_39 b15 b1
        if s_0_39 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0s : i64
        let s_1_0: i64 = 0;
        // D s_1_1: write-var size <= s_1_0
        fn_state.size = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var size:i64
        let s_2_0: i64 = fn_state.size;
        // D s_2_1: write-var sizeshadow#1887 <= s_2_0
        fn_state.sizeshadow_1887 = s_2_0;
        // C s_2_2: const #4s : i
        let s_2_2: i128 = 4;
        // D s_2_3: read-var imm5:u8
        let s_2_3: u8 = fn_state.imm5;
        // D s_2_4: cast zx s_2_3 -> bv
        let s_2_4: Bits = Bits::new(s_2_3 as u128, 5u16);
        // C s_2_5: const #1u : u64
        let s_2_5: u64 = 1;
        // D s_2_6: bit-extract s_2_4 s_2_2 s_2_5
        let s_2_6: Bits = (Bits::new(
            ((s_2_4) >> (s_2_2)).value(),
            u16::try_from(s_2_5).unwrap(),
        ));
        // D s_2_7: cast reint s_2_6 -> u8
        let s_2_7: bool = ((s_2_6.value()) != 0);
        // C s_2_8: const #0s : i
        let s_2_8: i128 = 0;
        // C s_2_9: const #0u : u64
        let s_2_9: u64 = 0;
        // D s_2_10: cast zx s_2_7 -> u64
        let s_2_10: u64 = (s_2_7 as u64);
        // C s_2_11: const #1u : u64
        let s_2_11: u64 = 1;
        // D s_2_12: and s_2_10 s_2_11
        let s_2_12: u64 = ((s_2_10) & (s_2_11));
        // D s_2_13: cmp-eq s_2_12 s_2_11
        let s_2_13: bool = ((s_2_12) == (s_2_11));
        // D s_2_14: lsl s_2_10 s_2_8
        let s_2_14: u64 = s_2_10 << s_2_8;
        // D s_2_15: or s_2_9 s_2_14
        let s_2_15: u64 = ((s_2_9) | (s_2_14));
        // D s_2_16: cmpl s_2_14
        let s_2_16: u64 = !s_2_14;
        // D s_2_17: and s_2_9 s_2_16
        let s_2_17: u64 = ((s_2_9) & (s_2_16));
        // D s_2_18: select s_2_13 s_2_15 s_2_17
        let s_2_18: u64 = if s_2_13 { s_2_15 } else { s_2_17 };
        // D s_2_19: cast trunc s_2_18 -> u8
        let s_2_19: bool = ((s_2_18) != 0);
        // D s_2_20: cast zx s_2_19 -> bv
        let s_2_20: Bits = Bits::new(s_2_19 as u128, 1u16);
        // C s_2_21: const #1u : u8
        let s_2_21: bool = true;
        // C s_2_22: cast zx s_2_21 -> bv
        let s_2_22: Bits = Bits::new(s_2_21 as u128, 1u16);
        // D s_2_23: cmp-eq s_2_20 s_2_22
        let s_2_23: bool = ((s_2_20) == (s_2_22));
        // N s_2_24: branch s_2_23 b14 b3
        if s_2_23 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #64s : i64
        let s_3_0: i64 = 64;
        // D s_3_1: write-var ga#267021 <= s_3_0
        fn_state.ga_267021 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var ga#267021:i64
        let s_4_0: i64 = fn_state.ga_267021;
        // D s_4_1: write-var idxdsize <= s_4_0
        fn_state.idxdsize = s_4_0;
        // C s_4_2: const #1s : i
        let s_4_2: i128 = 1;
        // D s_4_3: read-var sizeshadow#1887:i64
        let s_4_3: i64 = fn_state.sizeshadow_1887;
        // D s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (i128::try_from(s_4_3).unwrap());
        // D s_4_5: add s_4_4 s_4_2
        let s_4_5: i128 = (s_4_4 + s_4_2);
        // D s_4_6: cast reint s_4_5 -> i64
        let s_4_6: i64 = (s_4_5 as i64);
        // C s_4_7: const #4s : i
        let s_4_7: i128 = 4;
        // D s_4_8: read-var imm5:u8
        let s_4_8: u8 = fn_state.imm5;
        // D s_4_9: cast zx s_4_8 -> bv
        let s_4_9: Bits = Bits::new(s_4_8 as u128, 5u16);
        // D s_4_10: cast zx s_4_6 -> i
        let s_4_10: i128 = (i128::try_from(s_4_6).unwrap());
        // D s_4_11: call unsigned_subrange(s_4_9, s_4_7, s_4_10)
        let s_4_11: i128 = unsigned_subrange(state, tracer, s_4_9, s_4_7, s_4_10);
        // D s_4_12: cast reint s_4_11 -> i64
        let s_4_12: i64 = (s_4_11 as i64);
        // D s_4_13: write-var index <= s_4_12
        fn_state.index = s_4_12;
        // C s_4_14: const #8s : i64
        let s_4_14: i64 = 8;
        // D s_4_15: read-var sizeshadow#1887:i64
        let s_4_15: i64 = fn_state.sizeshadow_1887;
        // D s_4_16: lsl s_4_14 s_4_15
        let s_4_16: i64 = s_4_14 << s_4_15;
        // D s_4_17: write-var esize <= s_4_16
        fn_state.esize = s_4_16;
        // D s_4_18: read-var Q:u8
        let s_4_18: bool = fn_state.Q;
        // D s_4_19: cast zx s_4_18 -> bv
        let s_4_19: Bits = Bits::new(s_4_18 as u128, 1u16);
        // C s_4_20: const #1u : u8
        let s_4_20: bool = true;
        // C s_4_21: cast zx s_4_20 -> bv
        let s_4_21: Bits = Bits::new(s_4_20 as u128, 1u16);
        // D s_4_22: cmp-eq s_4_19 s_4_21
        let s_4_22: bool = ((s_4_19) == (s_4_21));
        // N s_4_23: branch s_4_22 b13 b5
        if s_4_22 {
            return block_13(state, tracer, fn_state);
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
        // D s_5_1: write-var ga#267026 <= s_5_0
        fn_state.ga_267026 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var ga#267026:i64
        let s_6_0: i64 = fn_state.ga_267026;
        // D s_6_1: write-var datasize <= s_6_0
        fn_state.datasize = s_6_0;
        // D s_6_2: read-var esize:i64
        let s_6_2: i64 = fn_state.esize;
        // D s_6_3: cast zx s_6_2 -> i
        let s_6_3: i128 = (i128::try_from(s_6_2).unwrap());
        // D s_6_4: call __id(s_6_3)
        let s_6_4: i128 = u__id(state, tracer, s_6_3);
        // D s_6_5: cast reint s_6_4 -> i64
        let s_6_5: i64 = (s_6_4 as i64);
        // C s_6_6: const #8s : i
        let s_6_6: i128 = 8;
        // D s_6_7: cast zx s_6_5 -> i
        let s_6_7: i128 = (i128::try_from(s_6_5).unwrap());
        // D s_6_8: cmp-eq s_6_7 s_6_6
        let s_6_8: bool = ((s_6_7) == (s_6_6));
        // N s_6_9: branch s_6_8 b12 b7
        if s_6_8 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var esize:i64
        let s_7_0: i64 = fn_state.esize;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: call __id(s_7_1)
        let s_7_2: i128 = u__id(state, tracer, s_7_1);
        // D s_7_3: cast reint s_7_2 -> i64
        let s_7_3: i64 = (s_7_2 as i64);
        // C s_7_4: const #16s : i
        let s_7_4: i128 = 16;
        // D s_7_5: cast zx s_7_3 -> i
        let s_7_5: i128 = (i128::try_from(s_7_3).unwrap());
        // D s_7_6: cmp-eq s_7_5 s_7_4
        let s_7_6: bool = ((s_7_5) == (s_7_4));
        // D s_7_7: write-var gs#170212 <= s_7_6
        fn_state.gs_170212 = s_7_6;
        // N s_7_8: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var gs#170212:u8
        let s_8_0: bool = fn_state.gs_170212;
        // N s_8_1: branch s_8_0 b11 b9
        if s_8_0 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var esize:i64
        let s_9_0: i64 = fn_state.esize;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: call __id(s_9_1)
        let s_9_2: i128 = u__id(state, tracer, s_9_1);
        // D s_9_3: cast reint s_9_2 -> i64
        let s_9_3: i64 = (s_9_2 as i64);
        // C s_9_4: const #32s : i
        let s_9_4: i128 = 32;
        // D s_9_5: cast zx s_9_3 -> i
        let s_9_5: i128 = (i128::try_from(s_9_3).unwrap());
        // D s_9_6: cmp-eq s_9_5 s_9_4
        let s_9_6: bool = ((s_9_5) == (s_9_4));
        // D s_9_7: write-var gs#170214 <= s_9_6
        fn_state.gs_170214 = s_9_6;
        // N s_9_8: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var gs#170214:u8
        let s_10_0: bool = fn_state.gs_170214;
        // N s_10_1: assert s_10_0
        let s_10_1: () = assert!(s_10_0);
        // D s_10_2: read-var datasize:i64
        let s_10_2: i64 = fn_state.datasize;
        // D s_10_3: cast zx s_10_2 -> i
        let s_10_3: i128 = (i128::try_from(s_10_2).unwrap());
        // D s_10_4: cast reint s_10_3 -> i64
        let s_10_4: i64 = (s_10_3 as i64);
        // D s_10_5: read-var esize:i64
        let s_10_5: i64 = fn_state.esize;
        // D s_10_6: cast zx s_10_5 -> i
        let s_10_6: i128 = (i128::try_from(s_10_5).unwrap());
        // D s_10_7: cast reint s_10_6 -> i64
        let s_10_7: i64 = (s_10_6 as i64);
        // D s_10_8: read-var idxdsize:i64
        let s_10_8: i64 = fn_state.idxdsize;
        // D s_10_9: cast zx s_10_8 -> i
        let s_10_9: i128 = (i128::try_from(s_10_8).unwrap());
        // D s_10_10: cast reint s_10_9 -> i64
        let s_10_10: i64 = (s_10_9 as i64);
        // D s_10_11: read-var index:i64
        let s_10_11: i64 = fn_state.index;
        // D s_10_12: cast zx s_10_11 -> i
        let s_10_12: i128 = (i128::try_from(s_10_11).unwrap());
        // D s_10_13: read-var d:i64
        let s_10_13: i64 = fn_state.d;
        // D s_10_14: read-var n:i64
        let s_10_14: i64 = fn_state.n;
        // D s_10_15: call execute_aarch64_instrs_vector_transfer_integer_move_signed(s_10_13, s_10_4, s_10_7, s_10_10, s_10_12, s_10_14)
        let s_10_15: () = execute_aarch64_instrs_vector_transfer_integer_move_signed(
            state,
            tracer,
            s_10_13,
            s_10_4,
            s_10_7,
            s_10_10,
            s_10_12,
            s_10_14,
        );
        // N s_10_16: return
        return;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #1u : u8
        let s_11_0: bool = true;
        // D s_11_1: write-var gs#170214 <= s_11_0
        fn_state.gs_170214 = s_11_0;
        // N s_11_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #1u : u8
        let s_12_0: bool = true;
        // D s_12_1: write-var gs#170212 <= s_12_0
        fn_state.gs_170212 = s_12_0;
        // N s_12_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #64s : i64
        let s_13_0: i64 = 64;
        // D s_13_1: write-var ga#267026 <= s_13_0
        fn_state.ga_267026 = s_13_0;
        // N s_13_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #128s : i64
        let s_14_0: i64 = 128;
        // D s_14_1: write-var ga#267021 <= s_14_0
        fn_state.ga_267021 = s_14_0;
        // N s_14_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var ga#267011:u8
        let s_15_0: u8 = fn_state.ga_267011;
        // C s_15_1: const #0s : i
        let s_15_1: i128 = 0;
        // D s_15_2: cast zx s_15_0 -> bv
        let s_15_2: Bits = Bits::new(s_15_0 as u128, 6u16);
        // C s_15_3: const #1s : i64
        let s_15_3: i64 = 1;
        // C s_15_4: cast zx s_15_3 -> i
        let s_15_4: i128 = (i128::try_from(s_15_3).unwrap());
        // C s_15_5: const #1s : i
        let s_15_5: i128 = 1;
        // C s_15_6: add s_15_5 s_15_4
        let s_15_6: i128 = (s_15_5 + s_15_4);
        // D s_15_7: bit-extract s_15_2 s_15_1 s_15_6
        let s_15_7: Bits = (Bits::new(
            ((s_15_2) >> (s_15_1)).value(),
            u16::try_from(s_15_6).unwrap(),
        ));
        // D s_15_8: cast reint s_15_7 -> u8
        let s_15_8: u8 = (s_15_7.value() as u8);
        // D s_15_9: cast zx s_15_8 -> bv
        let s_15_9: Bits = Bits::new(s_15_8 as u128, 2u16);
        // C s_15_10: const #2u : u8
        let s_15_10: u8 = 2;
        // C s_15_11: cast zx s_15_10 -> bv
        let s_15_11: Bits = Bits::new(s_15_10 as u128, 2u16);
        // D s_15_12: cmp-eq s_15_9 s_15_11
        let s_15_12: bool = ((s_15_9) == (s_15_11));
        // D s_15_13: not s_15_12
        let s_15_13: bool = !s_15_12;
        // N s_15_14: branch s_15_13 b17 b16
        if s_15_13 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #1s : i64
        let s_16_0: i64 = 1;
        // D s_16_1: write-var size <= s_16_0
        fn_state.size = s_16_0;
        // N s_16_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var ga#267011:u8
        let s_17_0: u8 = fn_state.ga_267011;
        // D s_17_1: write-var b__2 <= s_17_0
        fn_state.b__2 = s_17_0;
        // C s_17_2: const #5s : i
        let s_17_2: i128 = 5;
        // D s_17_3: read-var b__2:u8
        let s_17_3: u8 = fn_state.b__2;
        // D s_17_4: cast zx s_17_3 -> bv
        let s_17_4: Bits = Bits::new(s_17_3 as u128, 6u16);
        // C s_17_5: const #1s : i64
        let s_17_5: i64 = 1;
        // C s_17_6: cast zx s_17_5 -> i
        let s_17_6: i128 = (i128::try_from(s_17_5).unwrap());
        // C s_17_7: const #0s : i
        let s_17_7: i128 = 0;
        // C s_17_8: add s_17_7 s_17_6
        let s_17_8: i128 = (s_17_7 + s_17_6);
        // D s_17_9: bit-extract s_17_4 s_17_2 s_17_8
        let s_17_9: Bits = (Bits::new(
            ((s_17_4) >> (s_17_2)).value(),
            u16::try_from(s_17_8).unwrap(),
        ));
        // D s_17_10: cast reint s_17_9 -> u8
        let s_17_10: bool = ((s_17_9.value()) != 0);
        // D s_17_11: cast zx s_17_10 -> bv
        let s_17_11: Bits = Bits::new(s_17_10 as u128, 1u16);
        // C s_17_12: const #1u : u8
        let s_17_12: bool = true;
        // C s_17_13: cast zx s_17_12 -> bv
        let s_17_13: Bits = Bits::new(s_17_12 as u128, 1u16);
        // D s_17_14: cmp-eq s_17_11 s_17_13
        let s_17_14: bool = ((s_17_11) == (s_17_13));
        // N s_17_15: branch s_17_14 b22 b18
        if s_17_14 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_18_0: const #0u : u8
        let s_18_0: bool = false;
        // D s_18_1: write-var gs#170191 <= s_18_0
        fn_state.gs_170191 = s_18_0;
        // N s_18_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#170191:u8
        let s_19_0: bool = fn_state.gs_170191;
        // D s_19_1: not s_19_0
        let s_19_1: bool = !s_19_0;
        // N s_19_2: branch s_19_1 b21 b20
        if s_19_1 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_20_0: const #2s : i64
        let s_20_0: i64 = 2;
        // D s_20_1: write-var size <= s_20_0
        fn_state.size = s_20_0;
        // N s_20_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_21_0: panic
        panic!("{:?}", ());
        // N s_21_1: return
        return;
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #0s : i
        let s_22_0: i128 = 0;
        // D s_22_1: read-var b__2:u8
        let s_22_1: u8 = fn_state.b__2;
        // D s_22_2: cast zx s_22_1 -> bv
        let s_22_2: Bits = Bits::new(s_22_1 as u128, 6u16);
        // C s_22_3: const #1s : i64
        let s_22_3: i64 = 1;
        // C s_22_4: cast zx s_22_3 -> i
        let s_22_4: i128 = (i128::try_from(s_22_3).unwrap());
        // C s_22_5: const #2s : i
        let s_22_5: i128 = 2;
        // C s_22_6: add s_22_5 s_22_4
        let s_22_6: i128 = (s_22_5 + s_22_4);
        // D s_22_7: bit-extract s_22_2 s_22_0 s_22_6
        let s_22_7: Bits = (Bits::new(
            ((s_22_2) >> (s_22_0)).value(),
            u16::try_from(s_22_6).unwrap(),
        ));
        // D s_22_8: cast reint s_22_7 -> u8
        let s_22_8: u8 = (s_22_7.value() as u8);
        // D s_22_9: cast zx s_22_8 -> bv
        let s_22_9: Bits = Bits::new(s_22_8 as u128, 3u16);
        // C s_22_10: const #4u : u8
        let s_22_10: u8 = 4;
        // C s_22_11: cast zx s_22_10 -> bv
        let s_22_11: Bits = Bits::new(s_22_10 as u128, 3u16);
        // D s_22_12: cmp-eq s_22_9 s_22_11
        let s_22_12: bool = ((s_22_9) == (s_22_11));
        // D s_22_13: write-var gs#170191 <= s_22_12
        fn_state.gs_170191 = s_22_12;
        // N s_22_14: jump b19
        return block_19(state, tracer, fn_state);
    }
}
