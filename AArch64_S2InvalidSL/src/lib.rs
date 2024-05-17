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
use AArch64_PAMax::*;
use HaveSmallTranslationTableExt::*;
use common::*;
pub fn AArch64_S2InvalidSL<T: Tracer>(
    state: &mut State,
    tracer: &T,
    walkparams: ProductTypeb05ce25a107f0c5e,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        ga_14170: u8,
        ga_14187: bool,
        return_value: bool,
        ga_14167: u32,
        b__0: u8,
        gs_18966: bool,
        ga_14184: u8,
        ga_14179: u8,
        walkparams: ProductTypeb05ce25a107f0c5e,
    }
    let fn_state = FunctionState {
        walkparams,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_0_0: read-var walkparams.26:struct
        let s_0_0: u32 = fn_state.walkparams._26;
        // D s_0_1: write-var ga#14167 <= s_0_0
        fn_state.ga_14167 = s_0_0;
        // C s_0_2: const #0u : u32
        let s_0_2: u32 = 0;
        // D s_0_3: read-var ga#14167:u32
        let s_0_3: u32 = fn_state.ga_14167;
        // D s_0_4: cmp-eq s_0_2 s_0_3
        let s_0_4: bool = ((s_0_2) == (s_0_3));
        // D s_0_5: not s_0_4
        let s_0_5: bool = !s_0_4;
        // N s_0_6: branch s_0_5 b14 b1
        if s_0_5 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_1_0: read-var walkparams.23:struct
        let s_1_0: bool = fn_state.walkparams._23;
        // D s_1_1: read-var walkparams.22:struct
        let s_1_1: u8 = fn_state.walkparams._22;
        // D s_1_2: cast zx s_1_0 -> bv
        let s_1_2: Bits = Bits::new(s_1_0 as u128, 1u16);
        // D s_1_3: cast zx s_1_1 -> bv
        let s_1_3: Bits = Bits::new(s_1_1 as u128, 2u16);
        // D s_1_4: cast reint s_1_2 -> u128
        let s_1_4: u128 = (s_1_2.value() as u128);
        // D s_1_5: size-of s_1_2
        let s_1_5: u16 = s_1_2.length();
        // D s_1_6: cast reint s_1_3 -> u128
        let s_1_6: u128 = (s_1_3.value() as u128);
        // D s_1_7: size-of s_1_3
        let s_1_7: u16 = s_1_3.length();
        // D s_1_8: lsl s_1_4 s_1_7
        let s_1_8: u128 = s_1_4 << s_1_7;
        // D s_1_9: or s_1_8 s_1_6
        let s_1_9: u128 = ((s_1_8) | (s_1_6));
        // D s_1_10: add s_1_5 s_1_7
        let s_1_10: u16 = (s_1_5 + s_1_7);
        // D s_1_11: create-bits s_1_9 s_1_10
        let s_1_11: Bits = Bits::new(s_1_9, s_1_10);
        // D s_1_12: cast reint s_1_11 -> u8
        let s_1_12: u8 = (s_1_11.value() as u8);
        // D s_1_13: write-var ga#14170 <= s_1_12
        fn_state.ga_14170 = s_1_12;
        // D s_1_14: read-var ga#14170:u8
        let s_1_14: u8 = fn_state.ga_14170;
        // D s_1_15: write-var b__0 <= s_1_14
        fn_state.b__0 = s_1_14;
        // C s_1_16: const #2s : i
        let s_1_16: i128 = 2;
        // D s_1_17: read-var b__0:u8
        let s_1_17: u8 = fn_state.b__0;
        // D s_1_18: cast zx s_1_17 -> bv
        let s_1_18: Bits = Bits::new(s_1_17 as u128, 3u16);
        // C s_1_19: const #1s : i64
        let s_1_19: i64 = 1;
        // C s_1_20: cast zx s_1_19 -> i
        let s_1_20: i128 = (i128::try_from(s_1_19).unwrap());
        // C s_1_21: const #0s : i
        let s_1_21: i128 = 0;
        // C s_1_22: add s_1_21 s_1_20
        let s_1_22: i128 = (s_1_21 + s_1_20);
        // D s_1_23: bit-extract s_1_18 s_1_16 s_1_22
        let s_1_23: Bits = (Bits::new(
            ((s_1_18) >> (s_1_16)).value(),
            u16::try_from(s_1_22).unwrap(),
        ));
        // D s_1_24: cast reint s_1_23 -> u8
        let s_1_24: bool = ((s_1_23.value()) != 0);
        // D s_1_25: cast zx s_1_24 -> bv
        let s_1_25: Bits = Bits::new(s_1_24 as u128, 1u16);
        // C s_1_26: const #1u : u8
        let s_1_26: bool = true;
        // C s_1_27: cast zx s_1_26 -> bv
        let s_1_27: Bits = Bits::new(s_1_26 as u128, 1u16);
        // D s_1_28: cmp-eq s_1_25 s_1_27
        let s_1_28: bool = ((s_1_25) == (s_1_27));
        // N s_1_29: branch s_1_28 b13 b2
        if s_1_28 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var gs#18966 <= s_2_0
        fn_state.gs_18966 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_3_0: read-var gs#18966:u8
        let s_3_0: bool = fn_state.gs_18966;
        // D s_3_1: not s_3_0
        let s_3_1: bool = !s_3_0;
        // N s_3_2: branch s_3_1 b6 b4
        if s_3_1 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_4_0: const #1u : u8
        let s_4_0: bool = true;
        // D s_4_1: write-var return_value <= s_4_0
        fn_state.return_value = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_5_0: read-var return_value:u8
        let s_5_0: bool = fn_state.return_value;
        // N s_5_1: return s_5_0
        return s_5_0;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_6_0: read-var ga#14170:u8
        let s_6_0: u8 = fn_state.ga_14170;
        // C s_6_1: const #1s : i
        let s_6_1: i128 = 1;
        // D s_6_2: cast zx s_6_0 -> bv
        let s_6_2: Bits = Bits::new(s_6_0 as u128, 3u16);
        // C s_6_3: const #1s : i64
        let s_6_3: i64 = 1;
        // C s_6_4: cast zx s_6_3 -> i
        let s_6_4: i128 = (i128::try_from(s_6_3).unwrap());
        // C s_6_5: const #1s : i
        let s_6_5: i128 = 1;
        // C s_6_6: add s_6_5 s_6_4
        let s_6_6: i128 = (s_6_5 + s_6_4);
        // D s_6_7: bit-extract s_6_2 s_6_1 s_6_6
        let s_6_7: Bits = (Bits::new(
            ((s_6_2) >> (s_6_1)).value(),
            u16::try_from(s_6_6).unwrap(),
        ));
        // D s_6_8: cast reint s_6_7 -> u8
        let s_6_8: u8 = (s_6_7.value() as u8);
        // D s_6_9: cast zx s_6_8 -> bv
        let s_6_9: Bits = Bits::new(s_6_8 as u128, 2u16);
        // C s_6_10: const #3u : u8
        let s_6_10: u8 = 3;
        // C s_6_11: cast zx s_6_10 -> bv
        let s_6_11: Bits = Bits::new(s_6_10 as u128, 2u16);
        // D s_6_12: cmp-eq s_6_9 s_6_11
        let s_6_12: bool = ((s_6_9) == (s_6_11));
        // D s_6_13: not s_6_12
        let s_6_13: bool = !s_6_12;
        // N s_6_14: branch s_6_13 b8 b7
        if s_6_13 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_7_0: const #1u : u8
        let s_7_0: bool = true;
        // D s_7_1: write-var return_value <= s_7_0
        fn_state.return_value = s_7_0;
        // N s_7_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_8_0: read-var ga#14170:u8
        let s_8_0: u8 = fn_state.ga_14170;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 3u16);
        // C s_8_2: const #2u : u8
        let s_8_2: u8 = 2;
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
    ) -> bool {
        // C s_9_0: const #() : ()
        let s_9_0: () = ();
        // S s_9_1: call AArch64_PAMax(s_9_0)
        let s_9_1: i64 = AArch64_PAMax(state, tracer, s_9_0);
        // C s_9_2: const #44s : i
        let s_9_2: i128 = 44;
        // S s_9_3: cast zx s_9_1 -> i
        let s_9_3: i128 = (i128::try_from(s_9_1).unwrap());
        // S s_9_4: cmp-lt s_9_3 s_9_2
        let s_9_4: bool = ((s_9_3) < (s_9_2));
        // D s_9_5: write-var return_value <= s_9_4
        fn_state.return_value = s_9_4;
        // N s_9_6: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_10_0: read-var ga#14170:u8
        let s_10_0: u8 = fn_state.ga_14170;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 3u16);
        // C s_10_2: const #3u : u8
        let s_10_2: u8 = 3;
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
    ) -> bool {
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call HaveSmallTranslationTableExt(s_11_0)
        let s_11_1: bool = HaveSmallTranslationTableExt(state, tracer, s_11_0);
        // S s_11_2: not s_11_1
        let s_11_2: bool = !s_11_1;
        // D s_11_3: write-var return_value <= s_11_2
        fn_state.return_value = s_11_2;
        // N s_11_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var return_value <= s_12_0
        fn_state.return_value = s_12_0;
        // N s_12_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_13_0: const #0s : i
        let s_13_0: i128 = 0;
        // D s_13_1: read-var b__0:u8
        let s_13_1: u8 = fn_state.b__0;
        // D s_13_2: cast zx s_13_1 -> bv
        let s_13_2: Bits = Bits::new(s_13_1 as u128, 3u16);
        // C s_13_3: const #1s : i64
        let s_13_3: i64 = 1;
        // C s_13_4: cast zx s_13_3 -> i
        let s_13_4: i128 = (i128::try_from(s_13_3).unwrap());
        // C s_13_5: const #0s : i
        let s_13_5: i128 = 0;
        // C s_13_6: add s_13_5 s_13_4
        let s_13_6: i128 = (s_13_5 + s_13_4);
        // D s_13_7: bit-extract s_13_2 s_13_0 s_13_6
        let s_13_7: Bits = (Bits::new(
            ((s_13_2) >> (s_13_0)).value(),
            u16::try_from(s_13_6).unwrap(),
        ));
        // D s_13_8: cast reint s_13_7 -> u8
        let s_13_8: bool = ((s_13_7.value()) != 0);
        // D s_13_9: cast zx s_13_8 -> bv
        let s_13_9: Bits = Bits::new(s_13_8 as u128, 1u16);
        // C s_13_10: const #1u : u8
        let s_13_10: bool = true;
        // C s_13_11: cast zx s_13_10 -> bv
        let s_13_11: Bits = Bits::new(s_13_10 as u128, 1u16);
        // D s_13_12: cmp-eq s_13_9 s_13_11
        let s_13_12: bool = ((s_13_9) == (s_13_11));
        // D s_13_13: write-var gs#18966 <= s_13_12
        fn_state.gs_18966 = s_13_12;
        // N s_13_14: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_14_0: const #1u : u32
        let s_14_0: u32 = 1;
        // D s_14_1: read-var ga#14167:u32
        let s_14_1: u32 = fn_state.ga_14167;
        // D s_14_2: cmp-eq s_14_0 s_14_1
        let s_14_2: bool = ((s_14_0) == (s_14_1));
        // D s_14_3: not s_14_2
        let s_14_3: bool = !s_14_2;
        // N s_14_4: branch s_14_3 b20 b15
        if s_14_3 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_15_0: read-var walkparams.22:struct
        let s_15_0: u8 = fn_state.walkparams._22;
        // D s_15_1: write-var ga#14179 <= s_15_0
        fn_state.ga_14179 = s_15_0;
        // D s_15_2: read-var ga#14179:u8
        let s_15_2: u8 = fn_state.ga_14179;
        // D s_15_3: cast zx s_15_2 -> bv
        let s_15_3: Bits = Bits::new(s_15_2 as u128, 2u16);
        // C s_15_4: const #3u : u8
        let s_15_4: u8 = 3;
        // C s_15_5: cast zx s_15_4 -> bv
        let s_15_5: Bits = Bits::new(s_15_4 as u128, 2u16);
        // D s_15_6: cmp-eq s_15_3 s_15_5
        let s_15_6: bool = ((s_15_3) == (s_15_5));
        // D s_15_7: not s_15_6
        let s_15_7: bool = !s_15_6;
        // N s_15_8: branch s_15_7 b17 b16
        if s_15_7 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_16_0: read-var walkparams.3:struct
        let s_16_0: bool = fn_state.walkparams._3;
        // D s_16_1: cast zx s_16_0 -> bv
        let s_16_1: Bits = Bits::new(s_16_0 as u128, 1u16);
        // C s_16_2: const #0u : u8
        let s_16_2: bool = false;
        // C s_16_3: cast zx s_16_2 -> bv
        let s_16_3: Bits = Bits::new(s_16_2 as u128, 1u16);
        // D s_16_4: cmp-eq s_16_1 s_16_3
        let s_16_4: bool = ((s_16_1) == (s_16_3));
        // D s_16_5: write-var return_value <= s_16_4
        fn_state.return_value = s_16_4;
        // N s_16_6: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_17_0: read-var ga#14179:u8
        let s_17_0: u8 = fn_state.ga_14179;
        // D s_17_1: cast zx s_17_0 -> bv
        let s_17_1: Bits = Bits::new(s_17_0 as u128, 2u16);
        // C s_17_2: const #2u : u8
        let s_17_2: u8 = 2;
        // C s_17_3: cast zx s_17_2 -> bv
        let s_17_3: Bits = Bits::new(s_17_2 as u128, 2u16);
        // D s_17_4: cmp-eq s_17_1 s_17_3
        let s_17_4: bool = ((s_17_1) == (s_17_3));
        // D s_17_5: not s_17_4
        let s_17_5: bool = !s_17_4;
        // N s_17_6: branch s_17_5 b19 b18
        if s_17_5 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_18_0: const #() : ()
        let s_18_0: () = ();
        // S s_18_1: call AArch64_PAMax(s_18_0)
        let s_18_1: i64 = AArch64_PAMax(state, tracer, s_18_0);
        // C s_18_2: const #42s : i
        let s_18_2: i128 = 42;
        // S s_18_3: cast zx s_18_1 -> i
        let s_18_3: i128 = (i128::try_from(s_18_1).unwrap());
        // S s_18_4: cmp-lt s_18_3 s_18_2
        let s_18_4: bool = ((s_18_3) < (s_18_2));
        // D s_18_5: write-var return_value <= s_18_4
        fn_state.return_value = s_18_4;
        // N s_18_6: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_19_0: const #0u : u8
        let s_19_0: bool = false;
        // D s_19_1: write-var return_value <= s_19_0
        fn_state.return_value = s_19_0;
        // N s_19_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_20_0: const #2u : u32
        let s_20_0: u32 = 2;
        // D s_20_1: read-var ga#14167:u32
        let s_20_1: u32 = fn_state.ga_14167;
        // D s_20_2: cmp-eq s_20_0 s_20_1
        let s_20_2: bool = ((s_20_0) == (s_20_1));
        // D s_20_3: not s_20_2
        let s_20_3: bool = !s_20_2;
        // N s_20_4: branch s_20_3 b26 b21
        if s_20_3 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_21_0: read-var walkparams.22:struct
        let s_21_0: u8 = fn_state.walkparams._22;
        // D s_21_1: write-var ga#14184 <= s_21_0
        fn_state.ga_14184 = s_21_0;
        // D s_21_2: read-var ga#14184:u8
        let s_21_2: u8 = fn_state.ga_14184;
        // D s_21_3: cast zx s_21_2 -> bv
        let s_21_3: Bits = Bits::new(s_21_2 as u128, 2u16);
        // C s_21_4: const #3u : u8
        let s_21_4: u8 = 3;
        // C s_21_5: cast zx s_21_4 -> bv
        let s_21_5: Bits = Bits::new(s_21_4 as u128, 2u16);
        // D s_21_6: cmp-eq s_21_3 s_21_5
        let s_21_6: bool = ((s_21_3) == (s_21_5));
        // D s_21_7: not s_21_6
        let s_21_7: bool = !s_21_6;
        // N s_21_8: branch s_21_7 b23 b22
        if s_21_7 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_22_0: const #1u : u8
        let s_22_0: bool = true;
        // D s_22_1: write-var return_value <= s_22_0
        fn_state.return_value = s_22_0;
        // N s_22_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_23_0: read-var ga#14184:u8
        let s_23_0: u8 = fn_state.ga_14184;
        // D s_23_1: cast zx s_23_0 -> bv
        let s_23_1: Bits = Bits::new(s_23_0 as u128, 2u16);
        // C s_23_2: const #2u : u8
        let s_23_2: u8 = 2;
        // C s_23_3: cast zx s_23_2 -> bv
        let s_23_3: Bits = Bits::new(s_23_2 as u128, 2u16);
        // D s_23_4: cmp-eq s_23_1 s_23_3
        let s_23_4: bool = ((s_23_1) == (s_23_3));
        // D s_23_5: not s_23_4
        let s_23_5: bool = !s_23_4;
        // N s_23_6: branch s_23_5 b25 b24
        if s_23_5 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_24_0: const #() : ()
        let s_24_0: () = ();
        // S s_24_1: call AArch64_PAMax(s_24_0)
        let s_24_1: i64 = AArch64_PAMax(state, tracer, s_24_0);
        // C s_24_2: const #44s : i
        let s_24_2: i128 = 44;
        // S s_24_3: cast zx s_24_1 -> i
        let s_24_3: i128 = (i128::try_from(s_24_1).unwrap());
        // S s_24_4: cmp-lt s_24_3 s_24_2
        let s_24_4: bool = ((s_24_3) < (s_24_2));
        // D s_24_5: write-var return_value <= s_24_4
        fn_state.return_value = s_24_4;
        // N s_24_6: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_25_0: const #0u : u8
        let s_25_0: bool = false;
        // D s_25_1: write-var return_value <= s_25_0
        fn_state.return_value = s_25_0;
        // N s_25_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_26_0: read-var ga#14187:u8
        let s_26_0: bool = fn_state.ga_14187;
        // D s_26_1: write-var return_value <= s_26_0
        fn_state.return_value = s_26_0;
        // N s_26_2: jump b5
        return block_5(state, tracer, fn_state);
    }
}
