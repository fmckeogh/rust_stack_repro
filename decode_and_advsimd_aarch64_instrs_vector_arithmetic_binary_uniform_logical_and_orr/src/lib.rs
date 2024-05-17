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
use execute_aarch64_instrs_vector_arithmetic_binary_uniform_logical_and_orr::*;
use common::*;
pub fn decode_and_advsimd_aarch64_instrs_vector_arithmetic_binary_uniform_logical_and_orr<
    T: Tracer,
>(state: &mut State, tracer: &T, Rd: u8, Rn: u8, Rm: u8, size: u8, Q: bool) -> () {
    #[derive(Default)]
    struct FunctionState {
        invert: bool,
        m: i64,
        datasize: i64,
        n: i64,
        op: u32,
        ga_249434: i64,
        d: i64,
        Rd: u8,
        Rn: u8,
        Rm: u8,
        size: u8,
        Q: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        Rm,
        size,
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
        // D s_0_10: read-var Rm:u8
        let s_0_10: u8 = fn_state.Rm;
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 5u16);
        // D s_0_12: cast zx s_0_11 -> i
        let s_0_12: i128 = (s_0_11.value() as i128);
        // D s_0_13: cast reint s_0_12 -> i64
        let s_0_13: i64 = (s_0_12 as i64);
        // D s_0_14: write-var m <= s_0_13
        fn_state.m = s_0_13;
        // D s_0_15: read-var Q:u8
        let s_0_15: bool = fn_state.Q;
        // D s_0_16: cast zx s_0_15 -> bv
        let s_0_16: Bits = Bits::new(s_0_15 as u128, 1u16);
        // C s_0_17: const #1u : u8
        let s_0_17: bool = true;
        // C s_0_18: cast zx s_0_17 -> bv
        let s_0_18: Bits = Bits::new(s_0_17 as u128, 1u16);
        // D s_0_19: cmp-eq s_0_16 s_0_18
        let s_0_19: bool = ((s_0_16) == (s_0_18));
        // N s_0_20: branch s_0_19 b6 b1
        if s_0_19 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #64s : i64
        let s_1_0: i64 = 64;
        // D s_1_1: write-var ga#249434 <= s_1_0
        fn_state.ga_249434 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var ga#249434:i64
        let s_2_0: i64 = fn_state.ga_249434;
        // D s_2_1: write-var datasize <= s_2_0
        fn_state.datasize = s_2_0;
        // C s_2_2: const #0s : i
        let s_2_2: i128 = 0;
        // D s_2_3: read-var size:u8
        let s_2_3: u8 = fn_state.size;
        // D s_2_4: cast zx s_2_3 -> bv
        let s_2_4: Bits = Bits::new(s_2_3 as u128, 2u16);
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
        // D s_2_24: write-var invert <= s_2_23
        fn_state.invert = s_2_23;
        // C s_2_25: const #1s : i
        let s_2_25: i128 = 1;
        // D s_2_26: read-var size:u8
        let s_2_26: u8 = fn_state.size;
        // D s_2_27: cast zx s_2_26 -> bv
        let s_2_27: Bits = Bits::new(s_2_26 as u128, 2u16);
        // C s_2_28: const #1u : u64
        let s_2_28: u64 = 1;
        // D s_2_29: bit-extract s_2_27 s_2_25 s_2_28
        let s_2_29: Bits = (Bits::new(
            ((s_2_27) >> (s_2_25)).value(),
            u16::try_from(s_2_28).unwrap(),
        ));
        // D s_2_30: cast reint s_2_29 -> u8
        let s_2_30: bool = ((s_2_29.value()) != 0);
        // C s_2_31: const #0s : i
        let s_2_31: i128 = 0;
        // C s_2_32: const #0u : u64
        let s_2_32: u64 = 0;
        // D s_2_33: cast zx s_2_30 -> u64
        let s_2_33: u64 = (s_2_30 as u64);
        // C s_2_34: const #1u : u64
        let s_2_34: u64 = 1;
        // D s_2_35: and s_2_33 s_2_34
        let s_2_35: u64 = ((s_2_33) & (s_2_34));
        // D s_2_36: cmp-eq s_2_35 s_2_34
        let s_2_36: bool = ((s_2_35) == (s_2_34));
        // D s_2_37: lsl s_2_33 s_2_31
        let s_2_37: u64 = s_2_33 << s_2_31;
        // D s_2_38: or s_2_32 s_2_37
        let s_2_38: u64 = ((s_2_32) | (s_2_37));
        // D s_2_39: cmpl s_2_37
        let s_2_39: u64 = !s_2_37;
        // D s_2_40: and s_2_32 s_2_39
        let s_2_40: u64 = ((s_2_32) & (s_2_39));
        // D s_2_41: select s_2_36 s_2_38 s_2_40
        let s_2_41: u64 = if s_2_36 { s_2_38 } else { s_2_40 };
        // D s_2_42: cast trunc s_2_41 -> u8
        let s_2_42: bool = ((s_2_41) != 0);
        // D s_2_43: cast zx s_2_42 -> bv
        let s_2_43: Bits = Bits::new(s_2_42 as u128, 1u16);
        // C s_2_44: const #1u : u8
        let s_2_44: bool = true;
        // C s_2_45: cast zx s_2_44 -> bv
        let s_2_45: Bits = Bits::new(s_2_44 as u128, 1u16);
        // D s_2_46: cmp-eq s_2_43 s_2_45
        let s_2_46: bool = ((s_2_43) == (s_2_45));
        // N s_2_47: branch s_2_46 b5 b3
        if s_2_46 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0u : u32
        let s_3_0: u32 = 0;
        // D s_3_1: write-var op <= s_3_0
        fn_state.op = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var datasize:i64
        let s_4_0: i64 = fn_state.datasize;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: cast reint s_4_1 -> i64
        let s_4_2: i64 = (s_4_1 as i64);
        // D s_4_3: read-var d:i64
        let s_4_3: i64 = fn_state.d;
        // D s_4_4: read-var invert:u8
        let s_4_4: bool = fn_state.invert;
        // D s_4_5: read-var m:i64
        let s_4_5: i64 = fn_state.m;
        // D s_4_6: read-var n:i64
        let s_4_6: i64 = fn_state.n;
        // D s_4_7: read-var op:u32
        let s_4_7: u32 = fn_state.op;
        // D s_4_8: call execute_aarch64_instrs_vector_arithmetic_binary_uniform_logical_and_orr(s_4_3, s_4_2, s_4_4, s_4_5, s_4_6, s_4_7)
        let s_4_8: () = execute_aarch64_instrs_vector_arithmetic_binary_uniform_logical_and_orr(
            state,
            tracer,
            s_4_3,
            s_4_2,
            s_4_4,
            s_4_5,
            s_4_6,
            s_4_7,
        );
        // N s_4_9: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #2u : u32
        let s_5_0: u32 = 2;
        // D s_5_1: write-var op <= s_5_0
        fn_state.op = s_5_0;
        // N s_5_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #128s : i64
        let s_6_0: i64 = 128;
        // D s_6_1: write-var ga#249434 <= s_6_0
        fn_state.ga_249434 = s_6_0;
        // N s_6_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
