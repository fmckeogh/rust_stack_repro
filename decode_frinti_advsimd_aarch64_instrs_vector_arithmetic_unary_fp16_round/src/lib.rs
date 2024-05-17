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
pub fn decode_frinti_advsimd_aarch64_instrs_vector_arithmetic_unary_fp16_round<
    T: Tracer,
>(
    state: &mut State,
    tracer: &T,
    Rd: u8,
    Rn: u8,
    o1: bool,
    o2: bool,
    U: bool,
    Q: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_256373: u8,
        ga_256370: i64,
        exact: bool,
        n: i64,
        d: i64,
        elements: i64,
        rounding: u32,
        datasize: i64,
        Rd: u8,
        Rn: u8,
        o1: bool,
        o2: bool,
        U: bool,
        Q: bool,
    }
    let fn_state = FunctionState {
        Rd,
        Rn,
        o1,
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
        // D s_0_10: read-var Q:u8
        let s_0_10: bool = fn_state.Q;
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 1u16);
        // C s_0_12: const #1u : u8
        let s_0_12: bool = true;
        // C s_0_13: cast zx s_0_12 -> bv
        let s_0_13: Bits = Bits::new(s_0_12 as u128, 1u16);
        // D s_0_14: cmp-eq s_0_11 s_0_13
        let s_0_14: bool = ((s_0_11) == (s_0_13));
        // N s_0_15: branch s_0_14 b12 b1
        if s_0_14 {
            return block_12(state, tracer, fn_state);
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
        // D s_1_1: write-var ga#256370 <= s_1_0
        fn_state.ga_256370 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var ga#256370:i64
        let s_2_0: i64 = fn_state.ga_256370;
        // D s_2_1: write-var datasize <= s_2_0
        fn_state.datasize = s_2_0;
        // C s_2_2: const #16s : i
        let s_2_2: i128 = 16;
        // D s_2_3: read-var datasize:i64
        let s_2_3: i64 = fn_state.datasize;
        // D s_2_4: cast zx s_2_3 -> i
        let s_2_4: i128 = (i128::try_from(s_2_3).unwrap());
        // D s_2_5: div s_2_4 s_2_2
        let s_2_5: i128 = ((s_2_4) / (s_2_2));
        // D s_2_6: cast reint s_2_5 -> i64
        let s_2_6: i64 = (s_2_5 as i64);
        // D s_2_7: write-var elements <= s_2_6
        fn_state.elements = s_2_6;
        // C s_2_8: const #0u : u8
        let s_2_8: bool = false;
        // D s_2_9: write-var exact <= s_2_8
        fn_state.exact = s_2_8;
        // D s_2_10: read-var U:u8
        let s_2_10: bool = fn_state.U;
        // D s_2_11: cast zx s_2_10 -> bv
        let s_2_11: Bits = Bits::new(s_2_10 as u128, 1u16);
        // D s_2_12: read-var o1:u8
        let s_2_12: bool = fn_state.o1;
        // D s_2_13: cast zx s_2_12 -> bv
        let s_2_13: Bits = Bits::new(s_2_12 as u128, 1u16);
        // D s_2_14: cast reint s_2_11 -> u128
        let s_2_14: u128 = (s_2_11.value() as u128);
        // D s_2_15: size-of s_2_11
        let s_2_15: u16 = s_2_11.length();
        // D s_2_16: cast reint s_2_13 -> u128
        let s_2_16: u128 = (s_2_13.value() as u128);
        // D s_2_17: size-of s_2_13
        let s_2_17: u16 = s_2_13.length();
        // D s_2_18: lsl s_2_14 s_2_17
        let s_2_18: u128 = s_2_14 << s_2_17;
        // D s_2_19: or s_2_18 s_2_16
        let s_2_19: u128 = ((s_2_18) | (s_2_16));
        // D s_2_20: add s_2_15 s_2_17
        let s_2_20: u16 = (s_2_15 + s_2_17);
        // D s_2_21: create-bits s_2_19 s_2_20
        let s_2_21: Bits = Bits::new(s_2_19, s_2_20);
        // D s_2_22: cast reint s_2_21 -> u8
        let s_2_22: u8 = (s_2_21.value() as u8);
        // D s_2_23: cast zx s_2_22 -> bv
        let s_2_23: Bits = Bits::new(s_2_22 as u128, 2u16);
        // D s_2_24: read-var o2:u8
        let s_2_24: bool = fn_state.o2;
        // D s_2_25: cast zx s_2_24 -> bv
        let s_2_25: Bits = Bits::new(s_2_24 as u128, 1u16);
        // D s_2_26: cast reint s_2_23 -> u128
        let s_2_26: u128 = (s_2_23.value() as u128);
        // D s_2_27: size-of s_2_23
        let s_2_27: u16 = s_2_23.length();
        // D s_2_28: cast reint s_2_25 -> u128
        let s_2_28: u128 = (s_2_25.value() as u128);
        // D s_2_29: size-of s_2_25
        let s_2_29: u16 = s_2_25.length();
        // D s_2_30: lsl s_2_26 s_2_29
        let s_2_30: u128 = s_2_26 << s_2_29;
        // D s_2_31: or s_2_30 s_2_28
        let s_2_31: u128 = ((s_2_30) | (s_2_28));
        // D s_2_32: add s_2_27 s_2_29
        let s_2_32: u16 = (s_2_27 + s_2_29);
        // D s_2_33: create-bits s_2_31 s_2_32
        let s_2_33: Bits = Bits::new(s_2_31, s_2_32);
        // D s_2_34: cast reint s_2_33 -> u8
        let s_2_34: u8 = (s_2_33.value() as u8);
        // D s_2_35: write-var ga#256373 <= s_2_34
        fn_state.ga_256373 = s_2_34;
        // D s_2_36: read-var ga#256373:u8
        let s_2_36: u8 = fn_state.ga_256373;
        // C s_2_37: const #2s : i
        let s_2_37: i128 = 2;
        // D s_2_38: cast zx s_2_36 -> bv
        let s_2_38: Bits = Bits::new(s_2_36 as u128, 3u16);
        // C s_2_39: const #1s : i64
        let s_2_39: i64 = 1;
        // C s_2_40: cast zx s_2_39 -> i
        let s_2_40: i128 = (i128::try_from(s_2_39).unwrap());
        // C s_2_41: const #0s : i
        let s_2_41: i128 = 0;
        // C s_2_42: add s_2_41 s_2_40
        let s_2_42: i128 = (s_2_41 + s_2_40);
        // D s_2_43: bit-extract s_2_38 s_2_37 s_2_42
        let s_2_43: Bits = (Bits::new(
            ((s_2_38) >> (s_2_37)).value(),
            u16::try_from(s_2_42).unwrap(),
        ));
        // D s_2_44: cast reint s_2_43 -> u8
        let s_2_44: bool = ((s_2_43.value()) != 0);
        // D s_2_45: cast zx s_2_44 -> bv
        let s_2_45: Bits = Bits::new(s_2_44 as u128, 1u16);
        // C s_2_46: const #0u : u8
        let s_2_46: bool = false;
        // C s_2_47: cast zx s_2_46 -> bv
        let s_2_47: Bits = Bits::new(s_2_46 as u128, 1u16);
        // D s_2_48: cmp-eq s_2_45 s_2_47
        let s_2_48: bool = ((s_2_45) == (s_2_47));
        // D s_2_49: not s_2_48
        let s_2_49: bool = !s_2_48;
        // N s_2_50: branch s_2_49 b5 b3
        if s_2_49 {
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
        // D s_3_0: read-var o1:u8
        let s_3_0: bool = fn_state.o1;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 1u16);
        // D s_3_2: read-var o2:u8
        let s_3_2: bool = fn_state.o2;
        // D s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 1u16);
        // D s_3_4: cast reint s_3_1 -> u128
        let s_3_4: u128 = (s_3_1.value() as u128);
        // D s_3_5: size-of s_3_1
        let s_3_5: u16 = s_3_1.length();
        // D s_3_6: cast reint s_3_3 -> u128
        let s_3_6: u128 = (s_3_3.value() as u128);
        // D s_3_7: size-of s_3_3
        let s_3_7: u16 = s_3_3.length();
        // D s_3_8: lsl s_3_4 s_3_7
        let s_3_8: u128 = s_3_4 << s_3_7;
        // D s_3_9: or s_3_8 s_3_6
        let s_3_9: u128 = ((s_3_8) | (s_3_6));
        // D s_3_10: add s_3_5 s_3_7
        let s_3_10: u16 = (s_3_5 + s_3_7);
        // D s_3_11: create-bits s_3_9 s_3_10
        let s_3_11: Bits = Bits::new(s_3_9, s_3_10);
        // D s_3_12: cast reint s_3_11 -> u8
        let s_3_12: u8 = (s_3_11.value() as u8);
        // D s_3_13: call FPDecodeRounding(s_3_12)
        let s_3_13: u32 = FPDecodeRounding(state, tracer, s_3_12);
        // D s_3_14: write-var rounding <= s_3_13
        fn_state.rounding = s_3_13;
        // N s_3_15: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var rounding:u32
        let s_4_0: u32 = fn_state.rounding;
        // D s_4_1: read-var datasize:i64
        let s_4_1: i64 = fn_state.datasize;
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (i128::try_from(s_4_1).unwrap());
        // D s_4_3: cast reint s_4_2 -> i64
        let s_4_3: i64 = (s_4_2 as i64);
        // C s_4_4: const #16s : i64
        let s_4_4: i64 = 16;
        // D s_4_5: read-var elements:i64
        let s_4_5: i64 = fn_state.elements;
        // D s_4_6: cast zx s_4_5 -> i
        let s_4_6: i128 = (i128::try_from(s_4_5).unwrap());
        // D s_4_7: read-var d:i64
        let s_4_7: i64 = fn_state.d;
        // D s_4_8: read-var exact:u8
        let s_4_8: bool = fn_state.exact;
        // D s_4_9: read-var n:i64
        let s_4_9: i64 = fn_state.n;
        // D s_4_10: call execute_aarch64_instrs_vector_arithmetic_unary_fp16_round(s_4_7, s_4_3, s_4_6, s_4_4, s_4_8, s_4_9, s_4_0)
        let s_4_10: () = execute_aarch64_instrs_vector_arithmetic_unary_fp16_round(
            state,
            tracer,
            s_4_7,
            s_4_3,
            s_4_6,
            s_4_4,
            s_4_8,
            s_4_9,
            s_4_0,
        );
        // N s_4_11: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var ga#256373:u8
        let s_5_0: u8 = fn_state.ga_256373;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 3u16);
        // C s_5_2: const #4u : u8
        let s_5_2: u8 = 4;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 3u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // D s_5_5: not s_5_4
        let s_5_5: bool = !s_5_4;
        // N s_5_6: branch s_5_5 b7 b6
        if s_5_5 {
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
        // C s_6_0: const #4u : u32
        let s_6_0: u32 = 4;
        // D s_6_1: write-var rounding <= s_6_0
        fn_state.rounding = s_6_0;
        // N s_6_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var ga#256373:u8
        let s_7_0: u8 = fn_state.ga_256373;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 3u16);
        // C s_7_2: const #5u : u8
        let s_7_2: u8 = 5;
        // C s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 3u16);
        // D s_7_4: cmp-eq s_7_1 s_7_3
        let s_7_4: bool = ((s_7_1) == (s_7_3));
        // D s_7_5: not s_7_4
        let s_7_5: bool = !s_7_4;
        // N s_7_6: branch s_7_5 b9 b8
        if s_7_5 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: panic
        panic!("{:?}", ());
        // N s_8_1: return
        return;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var ga#256373:u8
        let s_9_0: u8 = fn_state.ga_256373;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 3u16);
        // C s_9_2: const #6u : u8
        let s_9_2: u8 = 6;
        // C s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 3u16);
        // D s_9_4: cmp-eq s_9_1 s_9_3
        let s_9_4: bool = ((s_9_1) == (s_9_3));
        // D s_9_5: not s_9_4
        let s_9_5: bool = !s_9_4;
        // N s_9_6: branch s_9_5 b11 b10
        if s_9_5 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call FPCR_read(s_10_0)
        let s_10_1: ProductType5c790c8ef59cc8b2 = FPCR_read(state, tracer, s_10_0);
        // S s_10_2: call FPRoundingMode(s_10_1)
        let s_10_2: u32 = FPRoundingMode(state, tracer, s_10_1);
        // D s_10_3: write-var rounding <= s_10_2
        fn_state.rounding = s_10_2;
        // C s_10_4: const #1u : u8
        let s_10_4: bool = true;
        // D s_10_5: write-var exact <= s_10_4
        fn_state.exact = s_10_4;
        // N s_10_6: jump b4
        return block_4(state, tracer, fn_state);
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
        // N s_11_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #128s : i64
        let s_12_0: i64 = 128;
        // D s_12_1: write-var ga#256370 <= s_12_0
        fn_state.ga_256370 = s_12_0;
        // N s_12_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
