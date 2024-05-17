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
use execute_aarch64_instrs_vector_arithmetic_unary_fp16_round::*;
use FPDecodeRounding::*;
use FPCR_read::*;
use common::*;
pub fn decode_frinta_advsimd_aarch64_instrs_vector_arithmetic_unary_float_round<
    T: Tracer,
>(
    state: &mut State,
    tracer: &T,
    Rd: u8,
    Rn: u8,
    o1: bool,
    sz: bool,
    o2: bool,
    U: bool,
    Q: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        esize: i64,
        exact: bool,
        n: i64,
        ga_256359: u8,
        d: i64,
        elements: i64,
        rounding: u32,
        datasize: i64,
        ga_256356: i64,
        Rd: u8,
        Rn: u8,
        o1: bool,
        sz: bool,
        o2: bool,
        U: bool,
        Q: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        o1,
        sz,
        o2,
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
        // D s_0_10: read-var sz:u8
        let s_0_10: bool = fn_state.sz;
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 1u16);
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
        let s_0_23: Bits = Bits::new(s_0_22 as u128, 2u16);
        // C s_0_24: const #2u : u8
        let s_0_24: u8 = 2;
        // C s_0_25: cast zx s_0_24 -> bv
        let s_0_25: Bits = Bits::new(s_0_24 as u128, 2u16);
        // D s_0_26: cmp-eq s_0_23 s_0_25
        let s_0_26: bool = ((s_0_23) == (s_0_25));
        // N s_0_27: branch s_0_26 b14 b1
        if s_0_26 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var sz:u8
        let s_1_0: bool = fn_state.sz;
        // D s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 1u16);
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (s_1_1.value() as i128);
        // D s_1_3: cast reint s_1_2 -> i64
        let s_1_3: i64 = (s_1_2 as i64);
        // C s_1_4: const #32s : i64
        let s_1_4: i64 = 32;
        // D s_1_5: lsl s_1_4 s_1_3
        let s_1_5: i64 = s_1_4 << s_1_3;
        // D s_1_6: write-var esize <= s_1_5
        fn_state.esize = s_1_5;
        // D s_1_7: read-var Q:u8
        let s_1_7: bool = fn_state.Q;
        // D s_1_8: cast zx s_1_7 -> bv
        let s_1_8: Bits = Bits::new(s_1_7 as u128, 1u16);
        // C s_1_9: const #1u : u8
        let s_1_9: bool = true;
        // C s_1_10: cast zx s_1_9 -> bv
        let s_1_10: Bits = Bits::new(s_1_9 as u128, 1u16);
        // D s_1_11: cmp-eq s_1_8 s_1_10
        let s_1_11: bool = ((s_1_8) == (s_1_10));
        // N s_1_12: branch s_1_11 b13 b2
        if s_1_11 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #64s : i64
        let s_2_0: i64 = 64;
        // D s_2_1: write-var ga#256356 <= s_2_0
        fn_state.ga_256356 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var ga#256356:i64
        let s_3_0: i64 = fn_state.ga_256356;
        // D s_3_1: write-var datasize <= s_3_0
        fn_state.datasize = s_3_0;
        // D s_3_2: read-var datasize:i64
        let s_3_2: i64 = fn_state.datasize;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: read-var esize:i64
        let s_3_4: i64 = fn_state.esize;
        // D s_3_5: cast zx s_3_4 -> i
        let s_3_5: i128 = (i128::try_from(s_3_4).unwrap());
        // D s_3_6: div s_3_3 s_3_5
        let s_3_6: i128 = ((s_3_3) / (s_3_5));
        // D s_3_7: cast reint s_3_6 -> i64
        let s_3_7: i64 = (s_3_6 as i64);
        // D s_3_8: write-var elements <= s_3_7
        fn_state.elements = s_3_7;
        // C s_3_9: const #0u : u8
        let s_3_9: bool = false;
        // D s_3_10: write-var exact <= s_3_9
        fn_state.exact = s_3_9;
        // D s_3_11: read-var U:u8
        let s_3_11: bool = fn_state.U;
        // D s_3_12: cast zx s_3_11 -> bv
        let s_3_12: Bits = Bits::new(s_3_11 as u128, 1u16);
        // D s_3_13: read-var o1:u8
        let s_3_13: bool = fn_state.o1;
        // D s_3_14: cast zx s_3_13 -> bv
        let s_3_14: Bits = Bits::new(s_3_13 as u128, 1u16);
        // D s_3_15: cast reint s_3_12 -> u128
        let s_3_15: u128 = (s_3_12.value() as u128);
        // D s_3_16: size-of s_3_12
        let s_3_16: u16 = s_3_12.length();
        // D s_3_17: cast reint s_3_14 -> u128
        let s_3_17: u128 = (s_3_14.value() as u128);
        // D s_3_18: size-of s_3_14
        let s_3_18: u16 = s_3_14.length();
        // D s_3_19: lsl s_3_15 s_3_18
        let s_3_19: u128 = s_3_15 << s_3_18;
        // D s_3_20: or s_3_19 s_3_17
        let s_3_20: u128 = ((s_3_19) | (s_3_17));
        // D s_3_21: add s_3_16 s_3_18
        let s_3_21: u16 = (s_3_16 + s_3_18);
        // D s_3_22: create-bits s_3_20 s_3_21
        let s_3_22: Bits = Bits::new(s_3_20, s_3_21);
        // D s_3_23: cast reint s_3_22 -> u8
        let s_3_23: u8 = (s_3_22.value() as u8);
        // D s_3_24: cast zx s_3_23 -> bv
        let s_3_24: Bits = Bits::new(s_3_23 as u128, 2u16);
        // D s_3_25: read-var o2:u8
        let s_3_25: bool = fn_state.o2;
        // D s_3_26: cast zx s_3_25 -> bv
        let s_3_26: Bits = Bits::new(s_3_25 as u128, 1u16);
        // D s_3_27: cast reint s_3_24 -> u128
        let s_3_27: u128 = (s_3_24.value() as u128);
        // D s_3_28: size-of s_3_24
        let s_3_28: u16 = s_3_24.length();
        // D s_3_29: cast reint s_3_26 -> u128
        let s_3_29: u128 = (s_3_26.value() as u128);
        // D s_3_30: size-of s_3_26
        let s_3_30: u16 = s_3_26.length();
        // D s_3_31: lsl s_3_27 s_3_30
        let s_3_31: u128 = s_3_27 << s_3_30;
        // D s_3_32: or s_3_31 s_3_29
        let s_3_32: u128 = ((s_3_31) | (s_3_29));
        // D s_3_33: add s_3_28 s_3_30
        let s_3_33: u16 = (s_3_28 + s_3_30);
        // D s_3_34: create-bits s_3_32 s_3_33
        let s_3_34: Bits = Bits::new(s_3_32, s_3_33);
        // D s_3_35: cast reint s_3_34 -> u8
        let s_3_35: u8 = (s_3_34.value() as u8);
        // D s_3_36: write-var ga#256359 <= s_3_35
        fn_state.ga_256359 = s_3_35;
        // D s_3_37: read-var ga#256359:u8
        let s_3_37: u8 = fn_state.ga_256359;
        // C s_3_38: const #2s : i
        let s_3_38: i128 = 2;
        // D s_3_39: cast zx s_3_37 -> bv
        let s_3_39: Bits = Bits::new(s_3_37 as u128, 3u16);
        // C s_3_40: const #1s : i64
        let s_3_40: i64 = 1;
        // C s_3_41: cast zx s_3_40 -> i
        let s_3_41: i128 = (i128::try_from(s_3_40).unwrap());
        // C s_3_42: const #0s : i
        let s_3_42: i128 = 0;
        // C s_3_43: add s_3_42 s_3_41
        let s_3_43: i128 = (s_3_42 + s_3_41);
        // D s_3_44: bit-extract s_3_39 s_3_38 s_3_43
        let s_3_44: Bits = (Bits::new(
            ((s_3_39) >> (s_3_38)).value(),
            u16::try_from(s_3_43).unwrap(),
        ));
        // D s_3_45: cast reint s_3_44 -> u8
        let s_3_45: bool = ((s_3_44.value()) != 0);
        // D s_3_46: cast zx s_3_45 -> bv
        let s_3_46: Bits = Bits::new(s_3_45 as u128, 1u16);
        // C s_3_47: const #0u : u8
        let s_3_47: bool = false;
        // C s_3_48: cast zx s_3_47 -> bv
        let s_3_48: Bits = Bits::new(s_3_47 as u128, 1u16);
        // D s_3_49: cmp-eq s_3_46 s_3_48
        let s_3_49: bool = ((s_3_46) == (s_3_48));
        // D s_3_50: not s_3_49
        let s_3_50: bool = !s_3_49;
        // N s_3_51: branch s_3_50 b6 b4
        if s_3_50 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var o1:u8
        let s_4_0: bool = fn_state.o1;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 1u16);
        // D s_4_2: read-var o2:u8
        let s_4_2: bool = fn_state.o2;
        // D s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 1u16);
        // D s_4_4: cast reint s_4_1 -> u128
        let s_4_4: u128 = (s_4_1.value() as u128);
        // D s_4_5: size-of s_4_1
        let s_4_5: u16 = s_4_1.length();
        // D s_4_6: cast reint s_4_3 -> u128
        let s_4_6: u128 = (s_4_3.value() as u128);
        // D s_4_7: size-of s_4_3
        let s_4_7: u16 = s_4_3.length();
        // D s_4_8: lsl s_4_4 s_4_7
        let s_4_8: u128 = s_4_4 << s_4_7;
        // D s_4_9: or s_4_8 s_4_6
        let s_4_9: u128 = ((s_4_8) | (s_4_6));
        // D s_4_10: add s_4_5 s_4_7
        let s_4_10: u16 = (s_4_5 + s_4_7);
        // D s_4_11: create-bits s_4_9 s_4_10
        let s_4_11: Bits = Bits::new(s_4_9, s_4_10);
        // D s_4_12: cast reint s_4_11 -> u8
        let s_4_12: u8 = (s_4_11.value() as u8);
        // D s_4_13: call FPDecodeRounding(s_4_12)
        let s_4_13: u32 = FPDecodeRounding(state, tracer, s_4_12);
        // D s_4_14: write-var rounding <= s_4_13
        fn_state.rounding = s_4_13;
        // N s_4_15: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var rounding:u32
        let s_5_0: u32 = fn_state.rounding;
        // D s_5_1: read-var datasize:i64
        let s_5_1: i64 = fn_state.datasize;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: cast reint s_5_2 -> i64
        let s_5_3: i64 = (s_5_2 as i64);
        // D s_5_4: read-var esize:i64
        let s_5_4: i64 = fn_state.esize;
        // D s_5_5: cast zx s_5_4 -> i
        let s_5_5: i128 = (i128::try_from(s_5_4).unwrap());
        // D s_5_6: cast reint s_5_5 -> i64
        let s_5_6: i64 = (s_5_5 as i64);
        // D s_5_7: read-var elements:i64
        let s_5_7: i64 = fn_state.elements;
        // D s_5_8: cast zx s_5_7 -> i
        let s_5_8: i128 = (i128::try_from(s_5_7).unwrap());
        // D s_5_9: read-var d:i64
        let s_5_9: i64 = fn_state.d;
        // D s_5_10: read-var exact:u8
        let s_5_10: bool = fn_state.exact;
        // D s_5_11: read-var n:i64
        let s_5_11: i64 = fn_state.n;
        // D s_5_12: call execute_aarch64_instrs_vector_arithmetic_unary_fp16_round(s_5_9, s_5_3, s_5_8, s_5_6, s_5_10, s_5_11, s_5_0)
        let s_5_12: () = execute_aarch64_instrs_vector_arithmetic_unary_fp16_round(
            state,
            tracer,
            s_5_9,
            s_5_3,
            s_5_8,
            s_5_6,
            s_5_10,
            s_5_11,
            s_5_0,
        );
        // N s_5_13: return
        return;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var ga#256359:u8
        let s_6_0: u8 = fn_state.ga_256359;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 3u16);
        // C s_6_2: const #4u : u8
        let s_6_2: u8 = 4;
        // C s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 3u16);
        // D s_6_4: cmp-eq s_6_1 s_6_3
        let s_6_4: bool = ((s_6_1) == (s_6_3));
        // D s_6_5: not s_6_4
        let s_6_5: bool = !s_6_4;
        // N s_6_6: branch s_6_5 b8 b7
        if s_6_5 {
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
        // C s_7_0: const #4u : u32
        let s_7_0: u32 = 4;
        // D s_7_1: write-var rounding <= s_7_0
        fn_state.rounding = s_7_0;
        // N s_7_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var ga#256359:u8
        let s_8_0: u8 = fn_state.ga_256359;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 3u16);
        // C s_8_2: const #5u : u8
        let s_8_2: u8 = 5;
        // C s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 3u16);
        // D s_8_4: cmp-eq s_8_1 s_8_3
        let s_8_4: bool = ((s_8_1) == (s_8_3));
        // D s_8_5: not s_8_4
        let s_8_5: bool = !s_8_4;
        // N s_8_6: branch s_8_5 b10 b9
        if s_8_5 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
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
        // D s_10_0: read-var ga#256359:u8
        let s_10_0: u8 = fn_state.ga_256359;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 3u16);
        // C s_10_2: const #6u : u8
        let s_10_2: u8 = 6;
        // C s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 3u16);
        // D s_10_4: cmp-eq s_10_1 s_10_3
        let s_10_4: bool = ((s_10_1) == (s_10_3));
        // D s_10_5: not s_10_4
        let s_10_5: bool = !s_10_4;
        // N s_10_6: branch s_10_5 b12 b11
        if s_10_5 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call FPCR_read(s_11_0)
        let s_11_1: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_11_0);
        // S s_11_2: call FPRoundingMode(s_11_1)
        let s_11_2: u32 = FPRoundingMode(state, tracer, s_11_1);
        // D s_11_3: write-var rounding <= s_11_2
        fn_state.rounding = s_11_2;
        // C s_11_4: const #1u : u8
        let s_11_4: bool = true;
        // D s_11_5: write-var exact <= s_11_4
        fn_state.exact = s_11_4;
        // N s_11_6: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #() : ()
        let s_12_0: () = ();
        // S s_12_1: call FPCR_read(s_12_0)
        let s_12_1: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_12_0);
        // S s_12_2: call FPRoundingMode(s_12_1)
        let s_12_2: u32 = FPRoundingMode(state, tracer, s_12_1);
        // D s_12_3: write-var rounding <= s_12_2
        fn_state.rounding = s_12_2;
        // N s_12_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #128s : i64
        let s_13_0: i64 = 128;
        // D s_13_1: write-var ga#256356 <= s_13_0
        fn_state.ga_256356 = s_13_0;
        // N s_13_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_14_0: panic
        panic!("{:?}", ());
        // N s_14_1: return
        return;
    }
}
