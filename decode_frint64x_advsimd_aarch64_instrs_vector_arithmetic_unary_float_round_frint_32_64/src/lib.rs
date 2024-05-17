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
use FPRoundingMode::*;
use FPCR_read::*;
use HaveFrintExt::*;
use execute_aarch64_instrs_vector_arithmetic_unary_float_round_frint_32_64::*;
use common::*;
pub fn decode_frint64x_advsimd_aarch64_instrs_vector_arithmetic_unary_float_round_frint_32_64<
    T: Tracer,
>(
    state: &mut State,
    tracer: &T,
    Rd: u8,
    Rn: u8,
    op: bool,
    sz: bool,
    U: bool,
    Q: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        esize: i64,
        ga_256239: i64,
        n: i64,
        ga_256236: i64,
        d: i64,
        elements: i64,
        intsize: i64,
        rounding: u32,
        datasize: i64,
        Rd: u8,
        Rn: u8,
        op: bool,
        sz: bool,
        U: bool,
        Q: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        op,
        sz,
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
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call HaveFrintExt(s_0_0)
        let s_0_1: bool = HaveFrintExt(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b13 b1
        if s_0_2 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var Rd:u8
        let s_1_0: u8 = fn_state.Rd;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 5u16);
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (s_1_1.value() as i128);
        // D s_1_3: cast reint s_1_2 -> i64
        let s_1_3: i64 = (s_1_2 as i64);
        // D s_1_4: write-var d <= s_1_3
        fn_state.d = s_1_3;
        // D s_1_5: read-var Rn:u8
        let s_1_5: u8 = fn_state.Rn;
        // D s_1_6: cast zx s_1_5 -> bv
        let s_1_6: Bits = Bits::new(s_1_5 as u128, 5u16);
        // D s_1_7: cast zx s_1_6 -> i
        let s_1_7: i128 = (s_1_6.value() as i128);
        // D s_1_8: cast reint s_1_7 -> i64
        let s_1_8: i64 = (s_1_7 as i64);
        // D s_1_9: write-var n <= s_1_8
        fn_state.n = s_1_8;
        // D s_1_10: read-var sz:u8
        let s_1_10: bool = fn_state.sz;
        // D s_1_11: cast zx s_1_10 -> bv
        let s_1_11: Bits = Bits::new(s_1_10 as u128, 1u16);
        // D s_1_12: read-var Q:u8
        let s_1_12: bool = fn_state.Q;
        // D s_1_13: cast zx s_1_12 -> bv
        let s_1_13: Bits = Bits::new(s_1_12 as u128, 1u16);
        // D s_1_14: cast reint s_1_11 -> u128
        let s_1_14: u128 = (s_1_11.value() as u128);
        // D s_1_15: size-of s_1_11
        let s_1_15: u16 = s_1_11.length();
        // D s_1_16: cast reint s_1_13 -> u128
        let s_1_16: u128 = (s_1_13.value() as u128);
        // D s_1_17: size-of s_1_13
        let s_1_17: u16 = s_1_13.length();
        // D s_1_18: lsl s_1_14 s_1_17
        let s_1_18: u128 = s_1_14 << s_1_17;
        // D s_1_19: or s_1_18 s_1_16
        let s_1_19: u128 = ((s_1_18) | (s_1_16));
        // D s_1_20: add s_1_15 s_1_17
        let s_1_20: u16 = (s_1_15 + s_1_17);
        // D s_1_21: create-bits s_1_19 s_1_20
        let s_1_21: Bits = Bits::new(s_1_19, s_1_20);
        // D s_1_22: cast reint s_1_21 -> u8
        let s_1_22: u8 = (s_1_21.value() as u8);
        // D s_1_23: cast zx s_1_22 -> bv
        let s_1_23: Bits = Bits::new(s_1_22 as u128, 2u16);
        // C s_1_24: const #2u : u8
        let s_1_24: u8 = 2;
        // C s_1_25: cast zx s_1_24 -> bv
        let s_1_25: Bits = Bits::new(s_1_24 as u128, 2u16);
        // D s_1_26: cmp-eq s_1_23 s_1_25
        let s_1_26: bool = ((s_1_23) == (s_1_25));
        // N s_1_27: branch s_1_26 b12 b2
        if s_1_26 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var sz:u8
        let s_2_0: bool = fn_state.sz;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 1u16);
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (s_2_1.value() as i128);
        // D s_2_3: cast reint s_2_2 -> i64
        let s_2_3: i64 = (s_2_2 as i64);
        // C s_2_4: const #32s : i64
        let s_2_4: i64 = 32;
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
        // N s_2_12: branch s_2_11 b11 b3
        if s_2_11 {
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
        // C s_3_0: const #64s : i64
        let s_3_0: i64 = 64;
        // D s_3_1: write-var ga#256236 <= s_3_0
        fn_state.ga_256236 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var ga#256236:i64
        let s_4_0: i64 = fn_state.ga_256236;
        // D s_4_1: write-var datasize <= s_4_0
        fn_state.datasize = s_4_0;
        // D s_4_2: read-var datasize:i64
        let s_4_2: i64 = fn_state.datasize;
        // D s_4_3: cast zx s_4_2 -> i
        let s_4_3: i128 = (i128::try_from(s_4_2).unwrap());
        // D s_4_4: read-var esize:i64
        let s_4_4: i64 = fn_state.esize;
        // D s_4_5: cast zx s_4_4 -> i
        let s_4_5: i128 = (i128::try_from(s_4_4).unwrap());
        // D s_4_6: div s_4_3 s_4_5
        let s_4_6: i128 = ((s_4_3) / (s_4_5));
        // D s_4_7: cast reint s_4_6 -> i64
        let s_4_7: i64 = (s_4_6 as i64);
        // D s_4_8: write-var elements <= s_4_7
        fn_state.elements = s_4_7;
        // D s_4_9: read-var op:u8
        let s_4_9: bool = fn_state.op;
        // D s_4_10: cast zx s_4_9 -> bv
        let s_4_10: Bits = Bits::new(s_4_9 as u128, 1u16);
        // C s_4_11: const #0u : u8
        let s_4_11: bool = false;
        // C s_4_12: cast zx s_4_11 -> bv
        let s_4_12: Bits = Bits::new(s_4_11 as u128, 1u16);
        // D s_4_13: cmp-eq s_4_10 s_4_12
        let s_4_13: bool = ((s_4_10) == (s_4_12));
        // N s_4_14: branch s_4_13 b10 b5
        if s_4_13 {
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
        // D s_5_1: write-var ga#256239 <= s_5_0
        fn_state.ga_256239 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var ga#256239:i64
        let s_6_0: i64 = fn_state.ga_256239;
        // D s_6_1: write-var intsize <= s_6_0
        fn_state.intsize = s_6_0;
        // D s_6_2: read-var U:u8
        let s_6_2: bool = fn_state.U;
        // D s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 1u16);
        // C s_6_4: const #0u : u8
        let s_6_4: bool = false;
        // C s_6_5: cast zx s_6_4 -> bv
        let s_6_5: Bits = Bits::new(s_6_4 as u128, 1u16);
        // D s_6_6: cmp-eq s_6_3 s_6_5
        let s_6_6: bool = ((s_6_3) == (s_6_5));
        // N s_6_7: branch s_6_6 b9 b7
        if s_6_6 {
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
        // C s_7_0: const #() : ()
        let s_7_0: () = ();
        // S s_7_1: call FPCR_read(s_7_0)
        let s_7_1: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_7_0);
        // S s_7_2: call FPRoundingMode(s_7_1)
        let s_7_2: u32 = FPRoundingMode(state, tracer, s_7_1);
        // D s_7_3: write-var rounding <= s_7_2
        fn_state.rounding = s_7_2;
        // N s_7_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var datasize:i64
        let s_8_0: i64 = fn_state.datasize;
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
        // D s_8_6: read-var elements:i64
        let s_8_6: i64 = fn_state.elements;
        // D s_8_7: cast zx s_8_6 -> i
        let s_8_7: i128 = (i128::try_from(s_8_6).unwrap());
        // D s_8_8: read-var d:i64
        let s_8_8: i64 = fn_state.d;
        // D s_8_9: read-var intsize:i64
        let s_8_9: i64 = fn_state.intsize;
        // D s_8_10: read-var n:i64
        let s_8_10: i64 = fn_state.n;
        // D s_8_11: read-var rounding:u32
        let s_8_11: u32 = fn_state.rounding;
        // D s_8_12: call execute_aarch64_instrs_vector_arithmetic_unary_float_round_frint_32_64(s_8_8, s_8_2, s_8_7, s_8_5, s_8_9, s_8_10, s_8_11)
        let s_8_12: () = execute_aarch64_instrs_vector_arithmetic_unary_float_round_frint_32_64(
            state,
            tracer,
            s_8_8,
            s_8_2,
            s_8_7,
            s_8_5,
            s_8_9,
            s_8_10,
            s_8_11,
        );
        // N s_8_13: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #3u : u32
        let s_9_0: u32 = 3;
        // D s_9_1: write-var rounding <= s_9_0
        fn_state.rounding = s_9_0;
        // N s_9_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #32s : i64
        let s_10_0: i64 = 32;
        // D s_10_1: write-var ga#256239 <= s_10_0
        fn_state.ga_256239 = s_10_0;
        // N s_10_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #128s : i64
        let s_11_0: i64 = 128;
        // D s_11_1: write-var ga#256236 <= s_11_0
        fn_state.ga_256236 = s_11_0;
        // N s_11_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_12_0: panic
        panic!("{:?}", ());
        // N s_12_1: return
        return;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_13_0: panic
        panic!("{:?}", ());
        // N s_13_1: return
        return;
    }
}
