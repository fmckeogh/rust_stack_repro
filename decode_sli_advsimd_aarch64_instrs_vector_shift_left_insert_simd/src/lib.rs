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
use execute_aarch64_instrs_vector_shift_left_insert_sisd::*;
use HighestSetBit::*;
use u_shl_int_general::*;
use fdiv_int::*;
use common::*;
pub fn decode_sli_advsimd_aarch64_instrs_vector_shift_left_insert_simd<T: Tracer>(
    state: &mut State,
    tracer: &T,
    Rd: u8,
    Rn: u8,
    immb: u8,
    immh: u8,
    Q: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        esize: i128,
        n: i64,
        d: i64,
        ga_266275: i64,
        Rd: u8,
        Rn: u8,
        immb: u8,
        immh: u8,
        Q: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        immb,
        immh,
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
        // D s_0_10: read-var immh:u8
        let s_0_10: u8 = fn_state.immh;
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 4u16);
        // C s_0_12: const #0u : u8
        let s_0_12: u8 = 0;
        // C s_0_13: cast zx s_0_12 -> bv
        let s_0_13: Bits = Bits::new(s_0_12 as u128, 4u16);
        // D s_0_14: cmp-eq s_0_11 s_0_13
        let s_0_14: bool = ((s_0_11) == (s_0_13));
        // N s_0_15: branch s_0_14 b7 b1
        if s_0_14 {
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
        // C s_1_0: const #3s : i
        let s_1_0: i128 = 3;
        // D s_1_1: read-var immh:u8
        let s_1_1: u8 = fn_state.immh;
        // D s_1_2: cast zx s_1_1 -> bv
        let s_1_2: Bits = Bits::new(s_1_1 as u128, 4u16);
        // C s_1_3: const #1u : u64
        let s_1_3: u64 = 1;
        // D s_1_4: bit-extract s_1_2 s_1_0 s_1_3
        let s_1_4: Bits = (Bits::new(
            ((s_1_2) >> (s_1_0)).value(),
            u16::try_from(s_1_3).unwrap(),
        ));
        // D s_1_5: cast reint s_1_4 -> u8
        let s_1_5: bool = ((s_1_4.value()) != 0);
        // C s_1_6: const #0s : i
        let s_1_6: i128 = 0;
        // C s_1_7: const #0u : u64
        let s_1_7: u64 = 0;
        // D s_1_8: cast zx s_1_5 -> u64
        let s_1_8: u64 = (s_1_5 as u64);
        // C s_1_9: const #1u : u64
        let s_1_9: u64 = 1;
        // D s_1_10: and s_1_8 s_1_9
        let s_1_10: u64 = ((s_1_8) & (s_1_9));
        // D s_1_11: cmp-eq s_1_10 s_1_9
        let s_1_11: bool = ((s_1_10) == (s_1_9));
        // D s_1_12: lsl s_1_8 s_1_6
        let s_1_12: u64 = s_1_8 << s_1_6;
        // D s_1_13: or s_1_7 s_1_12
        let s_1_13: u64 = ((s_1_7) | (s_1_12));
        // D s_1_14: cmpl s_1_12
        let s_1_14: u64 = !s_1_12;
        // D s_1_15: and s_1_7 s_1_14
        let s_1_15: u64 = ((s_1_7) & (s_1_14));
        // D s_1_16: select s_1_11 s_1_13 s_1_15
        let s_1_16: u64 = if s_1_11 { s_1_13 } else { s_1_15 };
        // D s_1_17: cast trunc s_1_16 -> u8
        let s_1_17: bool = ((s_1_16) != 0);
        // D s_1_18: cast zx s_1_17 -> bv
        let s_1_18: Bits = Bits::new(s_1_17 as u128, 1u16);
        // D s_1_19: read-var Q:u8
        let s_1_19: bool = fn_state.Q;
        // D s_1_20: cast zx s_1_19 -> bv
        let s_1_20: Bits = Bits::new(s_1_19 as u128, 1u16);
        // D s_1_21: cast reint s_1_18 -> u128
        let s_1_21: u128 = (s_1_18.value() as u128);
        // D s_1_22: size-of s_1_18
        let s_1_22: u16 = s_1_18.length();
        // D s_1_23: cast reint s_1_20 -> u128
        let s_1_23: u128 = (s_1_20.value() as u128);
        // D s_1_24: size-of s_1_20
        let s_1_24: u16 = s_1_20.length();
        // D s_1_25: lsl s_1_21 s_1_24
        let s_1_25: u128 = s_1_21 << s_1_24;
        // D s_1_26: or s_1_25 s_1_23
        let s_1_26: u128 = ((s_1_25) | (s_1_23));
        // D s_1_27: add s_1_22 s_1_24
        let s_1_27: u16 = (s_1_22 + s_1_24);
        // D s_1_28: create-bits s_1_26 s_1_27
        let s_1_28: Bits = Bits::new(s_1_26, s_1_27);
        // D s_1_29: cast reint s_1_28 -> u8
        let s_1_29: u8 = (s_1_28.value() as u8);
        // D s_1_30: cast zx s_1_29 -> bv
        let s_1_30: Bits = Bits::new(s_1_29 as u128, 2u16);
        // C s_1_31: const #2u : u8
        let s_1_31: u8 = 2;
        // C s_1_32: cast zx s_1_31 -> bv
        let s_1_32: Bits = Bits::new(s_1_31 as u128, 2u16);
        // D s_1_33: cmp-eq s_1_30 s_1_32
        let s_1_33: bool = ((s_1_30) == (s_1_32));
        // N s_1_34: branch s_1_33 b6 b2
        if s_1_33 {
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
        // D s_2_0: read-var immh:u8
        let s_2_0: u8 = fn_state.immh;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 4u16);
        // D s_2_2: call HighestSetBit(s_2_1)
        let s_2_2: i128 = HighestSetBit(state, tracer, s_2_1);
        // C s_2_3: const #8s : i
        let s_2_3: i128 = 8;
        // D s_2_4: call _shl_int_general(s_2_3, s_2_2)
        let s_2_4: i128 = u_shl_int_general(state, tracer, s_2_3, s_2_2);
        // D s_2_5: write-var esize <= s_2_4
        fn_state.esize = s_2_4;
        // D s_2_6: read-var Q:u8
        let s_2_6: bool = fn_state.Q;
        // D s_2_7: cast zx s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 1u16);
        // C s_2_8: const #1u : u8
        let s_2_8: bool = true;
        // C s_2_9: cast zx s_2_8 -> bv
        let s_2_9: Bits = Bits::new(s_2_8 as u128, 1u16);
        // D s_2_10: cmp-eq s_2_7 s_2_9
        let s_2_10: bool = ((s_2_7) == (s_2_9));
        // N s_2_11: branch s_2_10 b5 b3
        if s_2_10 {
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
        // D s_3_1: write-var ga#266275 <= s_3_0
        fn_state.ga_266275 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var ga#266275:i64
        let s_4_0: i64 = fn_state.ga_266275;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: read-var esize:i
        let s_4_2: i128 = fn_state.esize;
        // D s_4_3: call fdiv_int(s_4_1, s_4_2)
        let s_4_3: i128 = fdiv_int(state, tracer, s_4_1, s_4_2);
        // D s_4_4: read-var immh:u8
        let s_4_4: u8 = fn_state.immh;
        // D s_4_5: cast zx s_4_4 -> bv
        let s_4_5: Bits = Bits::new(s_4_4 as u128, 4u16);
        // D s_4_6: read-var immb:u8
        let s_4_6: u8 = fn_state.immb;
        // D s_4_7: cast zx s_4_6 -> bv
        let s_4_7: Bits = Bits::new(s_4_6 as u128, 3u16);
        // D s_4_8: cast reint s_4_5 -> u128
        let s_4_8: u128 = (s_4_5.value() as u128);
        // D s_4_9: size-of s_4_5
        let s_4_9: u16 = s_4_5.length();
        // D s_4_10: cast reint s_4_7 -> u128
        let s_4_10: u128 = (s_4_7.value() as u128);
        // D s_4_11: size-of s_4_7
        let s_4_11: u16 = s_4_7.length();
        // D s_4_12: lsl s_4_8 s_4_11
        let s_4_12: u128 = s_4_8 << s_4_11;
        // D s_4_13: or s_4_12 s_4_10
        let s_4_13: u128 = ((s_4_12) | (s_4_10));
        // D s_4_14: add s_4_9 s_4_11
        let s_4_14: u16 = (s_4_9 + s_4_11);
        // D s_4_15: create-bits s_4_13 s_4_14
        let s_4_15: Bits = Bits::new(s_4_13, s_4_14);
        // D s_4_16: cast reint s_4_15 -> u8
        let s_4_16: u8 = (s_4_15.value() as u8);
        // D s_4_17: cast zx s_4_16 -> bv
        let s_4_17: Bits = Bits::new(s_4_16 as u128, 7u16);
        // D s_4_18: cast zx s_4_17 -> i
        let s_4_18: i128 = (s_4_17.value() as i128);
        // D s_4_19: cast reint s_4_18 -> i64
        let s_4_19: i64 = (s_4_18 as i64);
        // D s_4_20: cast zx s_4_19 -> i
        let s_4_20: i128 = (i128::try_from(s_4_19).unwrap());
        // D s_4_21: read-var esize:i
        let s_4_21: i128 = fn_state.esize;
        // D s_4_22: sub s_4_20 s_4_21
        let s_4_22: i128 = ((s_4_20) - (s_4_21));
        // D s_4_23: cast zx s_4_0 -> i
        let s_4_23: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_24: cast reint s_4_23 -> i64
        let s_4_24: i64 = (s_4_23 as i64);
        // D s_4_25: read-var esize:i
        let s_4_25: i128 = fn_state.esize;
        // D s_4_26: cast zx s_4_24 -> i
        let s_4_26: i128 = (i128::try_from(s_4_24).unwrap());
        // D s_4_27: read-var d:i64
        let s_4_27: i64 = fn_state.d;
        // D s_4_28: read-var n:i64
        let s_4_28: i64 = fn_state.n;
        // D s_4_29: call execute_aarch64_instrs_vector_shift_left_insert_sisd(s_4_27, s_4_26, s_4_3, s_4_25, s_4_28, s_4_22)
        let s_4_29: () = execute_aarch64_instrs_vector_shift_left_insert_sisd(
            state,
            tracer,
            s_4_27,
            s_4_26,
            s_4_3,
            s_4_25,
            s_4_28,
            s_4_22,
        );
        // N s_4_30: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #128s : i64
        let s_5_0: i64 = 128;
        // D s_5_1: write-var ga#266275 <= s_5_0
        fn_state.ga_266275 = s_5_0;
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
