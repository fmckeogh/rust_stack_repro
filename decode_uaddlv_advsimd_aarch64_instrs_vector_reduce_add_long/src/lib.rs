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
use execute_aarch64_instrs_vector_reduce_add_long::*;
use common::*;
pub fn decode_uaddlv_advsimd_aarch64_instrs_vector_reduce_add_long<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rd: u8,
    Rn: u8,
    size: u8,
    U: bool,
    Q: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        esize: i64,
        n: i64,
        d: i64,
        ga_264944: i64,
        Rd: u8,
        Rn: u8,
        size: u8,
        U: bool,
        Q: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        size,
        U,
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
        // D s_0_10: read-var size:u8
        let s_0_10: u8 = fn_state.size;
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 2u16);
        // D s_0_12: read-var Q:u8
        let s_0_12: bool = fn_state.Q;
        // D s_0_13: cast zx s_0_12 -> bv
        let s_0_13: Bits = Bits::new(s_0_12 as u128, 1u16);
        // D s_0_14: cast reint s_0_11 -> u128
        let s_0_14: u128 = (s_0_11.value() as u128);
        // D s_0_15: size-of s_0_11
        let s_0_15: u16 = s_0_11.length();
        // D s_0_16: cast reint s_0_13 -> u128
        let s_0_16: u128 = (s_0_13.value() as u128);
        // D s_0_17: size-of s_0_13
        let s_0_17: u16 = s_0_13.length();
        // D s_0_18: lsl s_0_14 s_0_17
        let s_0_18: u128 = s_0_14 << s_0_17;
        // D s_0_19: or s_0_18 s_0_16
        let s_0_19: u128 = ((s_0_18) | (s_0_16));
        // D s_0_20: add s_0_15 s_0_17
        let s_0_20: u16 = (s_0_15 + s_0_17);
        // D s_0_21: create-bits s_0_19 s_0_20
        let s_0_21: Bits = Bits::new(s_0_19, s_0_20);
        // D s_0_22: cast reint s_0_21 -> u8
        let s_0_22: u8 = (s_0_21.value() as u8);
        // D s_0_23: cast zx s_0_22 -> bv
        let s_0_23: Bits = Bits::new(s_0_22 as u128, 3u16);
        // C s_0_24: const #4u : u8
        let s_0_24: u8 = 4;
        // C s_0_25: cast zx s_0_24 -> bv
        let s_0_25: Bits = Bits::new(s_0_24 as u128, 3u16);
        // D s_0_26: cmp-eq s_0_23 s_0_25
        let s_0_26: bool = ((s_0_23) == (s_0_25));
        // N s_0_27: branch s_0_26 b7 b1
        if s_0_26 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var size:u8
        let s_1_0: u8 = fn_state.size;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 2u16);
        // C s_1_2: const #3u : u8
        let s_1_2: u8 = 3;
        // C s_1_3: cast zx s_1_2 -> bv
        let s_1_3: Bits = Bits::new(s_1_2 as u128, 2u16);
        // D s_1_4: cmp-eq s_1_1 s_1_3
        let s_1_4: bool = ((s_1_1) == (s_1_3));
        // N s_1_5: branch s_1_4 b6 b2
        if s_1_4 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var size:u8
        let s_2_0: u8 = fn_state.size;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 2u16);
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (s_2_1.value() as i128);
        // D s_2_3: cast reint s_2_2 -> i64
        let s_2_3: i64 = (s_2_2 as i64);
        // C s_2_4: const #8s : i64
        let s_2_4: i64 = 8;
        // D s_2_5: lsl s_2_4 s_2_3
        let s_2_5: i64 = s_2_4 << s_2_3;
        // D s_2_6: write-var esize <= s_2_5
        fn_state.esize = s_2_5;
        // D s_2_7: read-var Q:u8
        let s_2_7: bool = fn_state.Q;
        // D s_2_8: cast zx s_2_7 -> bv
        let s_2_8: Bits = Bits::new(s_2_7 as u128, 1u16);
        // C s_2_9: const #1u : u8
        let s_2_9: bool = true;
        // C s_2_10: cast zx s_2_9 -> bv
        let s_2_10: Bits = Bits::new(s_2_9 as u128, 1u16);
        // D s_2_11: cmp-eq s_2_8 s_2_10
        let s_2_11: bool = ((s_2_8) == (s_2_10));
        // N s_2_12: branch s_2_11 b5 b3
        if s_2_11 {
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
        // C s_3_0: const #64s : i64
        let s_3_0: i64 = 64;
        // D s_3_1: write-var ga#264944 <= s_3_0
        fn_state.ga_264944 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var ga#264944:i64
        let s_4_0: i64 = fn_state.ga_264944;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: read-var esize:i64
        let s_4_2: i64 = fn_state.esize;
        // D s_4_3: cast zx s_4_2 -> i
        let s_4_3: i128 = (i128::try_from(s_4_2).unwrap());
        // D s_4_4: div s_4_1 s_4_3
        let s_4_4: i128 = ((s_4_1) / (s_4_3));
        // D s_4_5: cast reint s_4_4 -> i64
        let s_4_5: i64 = (s_4_4 as i64);
        // D s_4_6: read-var U:u8
        let s_4_6: bool = fn_state.U;
        // D s_4_7: cast zx s_4_6 -> bv
        let s_4_7: Bits = Bits::new(s_4_6 as u128, 1u16);
        // C s_4_8: const #1u : u8
        let s_4_8: bool = true;
        // C s_4_9: cast zx s_4_8 -> bv
        let s_4_9: Bits = Bits::new(s_4_8 as u128, 1u16);
        // D s_4_10: cmp-eq s_4_7 s_4_9
        let s_4_10: bool = ((s_4_7) == (s_4_9));
        // D s_4_11: cast zx s_4_0 -> i
        let s_4_11: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_12: cast reint s_4_11 -> i64
        let s_4_12: i64 = (s_4_11 as i64);
        // D s_4_13: read-var esize:i64
        let s_4_13: i64 = fn_state.esize;
        // D s_4_14: cast zx s_4_13 -> i
        let s_4_14: i128 = (i128::try_from(s_4_13).unwrap());
        // D s_4_15: cast reint s_4_14 -> i64
        let s_4_15: i64 = (s_4_14 as i64);
        // D s_4_16: cast zx s_4_5 -> i
        let s_4_16: i128 = (i128::try_from(s_4_5).unwrap());
        // D s_4_17: read-var d:i64
        let s_4_17: i64 = fn_state.d;
        // D s_4_18: read-var n:i64
        let s_4_18: i64 = fn_state.n;
        // D s_4_19: call execute_aarch64_instrs_vector_reduce_add_long(s_4_17, s_4_12, s_4_16, s_4_15, s_4_18, s_4_10)
        let s_4_19: () = execute_aarch64_instrs_vector_reduce_add_long(
            state,
            tracer,
            s_4_17,
            s_4_12,
            s_4_16,
            s_4_15,
            s_4_18,
            s_4_10,
        );
        // N s_4_20: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #128s : i64
        let s_5_0: i64 = 128;
        // D s_5_1: write-var ga#264944 <= s_5_0
        fn_state.ga_264944 = s_5_0;
        // N s_5_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_6_0: panic
        panic!("{:?}", ());
        // N s_6_1: return
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
}
