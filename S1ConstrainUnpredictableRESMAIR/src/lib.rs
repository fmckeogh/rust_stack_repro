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
use HaveMTE2Ext::*;
use HaveFeatXS::*;
use common::*;
pub fn S1ConstrainUnpredictableRESMAIR<T: Tracer>(
    state: &mut State,
    tracer: &T,
    attr: u8,
    s1aarch64: bool,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_12836: bool,
        return_value: bool,
        gs_12847: bool,
        gs_12835: bool,
        gs_12845: bool,
        b__0: u8,
        gs_12843: bool,
        attr: u8,
        s1aarch64: bool,
    }
    let fn_state = FunctionState {
        attr,
        s1aarch64,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_0_0: read-var attr:u8
        let s_0_0: u8 = fn_state.attr;
        // D s_0_1: write-var b__0 <= s_0_0
        fn_state.b__0 = s_0_0;
        // C s_0_2: const #4s : i
        let s_0_2: i128 = 4;
        // D s_0_3: read-var b__0:u8
        let s_0_3: u8 = fn_state.b__0;
        // D s_0_4: cast zx s_0_3 -> bv
        let s_0_4: Bits = Bits::new(s_0_3 as u128, 8u16);
        // C s_0_5: const #1s : i64
        let s_0_5: i64 = 1;
        // C s_0_6: cast zx s_0_5 -> i
        let s_0_6: i128 = (i128::try_from(s_0_5).unwrap());
        // C s_0_7: const #3s : i
        let s_0_7: i128 = 3;
        // C s_0_8: add s_0_7 s_0_6
        let s_0_8: i128 = (s_0_7 + s_0_6);
        // D s_0_9: bit-extract s_0_4 s_0_2 s_0_8
        let s_0_9: Bits = (Bits::new(
            ((s_0_4) >> (s_0_2)).value(),
            u16::try_from(s_0_8).unwrap(),
        ));
        // D s_0_10: cast reint s_0_9 -> u8
        let s_0_10: u8 = (s_0_9.value() as u8);
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 4u16);
        // C s_0_12: const #0u : u8
        let s_0_12: u8 = 0;
        // C s_0_13: cast zx s_0_12 -> bv
        let s_0_13: Bits = Bits::new(s_0_12 as u128, 4u16);
        // D s_0_14: cmp-eq s_0_11 s_0_13
        let s_0_14: bool = ((s_0_11) == (s_0_13));
        // N s_0_15: branch s_0_14 b28 b1
        if s_0_14 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#12835 <= s_1_0
        fn_state.gs_12835 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_2_0: read-var gs#12835:u8
        let s_2_0: bool = fn_state.gs_12835;
        // D s_2_1: not s_2_0
        let s_2_1: bool = !s_2_0;
        // N s_2_2: branch s_2_1 b8 b3
        if s_2_1 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_3_0: read-var s1aarch64:u8
        let s_3_0: bool = fn_state.s1aarch64;
        // N s_3_1: branch s_3_0 b7 b4
        if s_3_0 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var gs#12836 <= s_4_0
        fn_state.gs_12836 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_5_0: read-var gs#12836:u8
        let s_5_0: bool = fn_state.gs_12836;
        // D s_5_1: not s_5_0
        let s_5_1: bool = !s_5_0;
        // D s_5_2: write-var return_value <= s_5_1
        fn_state.return_value = s_5_1;
        // N s_5_3: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_6_0: read-var return_value:u8
        let s_6_0: bool = fn_state.return_value;
        // N s_6_1: return s_6_0
        return s_6_0;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_7_0: const #() : ()
        let s_7_0: () = ();
        // S s_7_1: call HaveFeatXS(s_7_0)
        let s_7_1: bool = HaveFeatXS(state, tracer, s_7_0);
        // D s_7_2: write-var gs#12836 <= s_7_1
        fn_state.gs_12836 = s_7_1;
        // N s_7_3: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_8_0: read-var attr:u8
        let s_8_0: u8 = fn_state.attr;
        // C s_8_1: const #4s : i
        let s_8_1: i128 = 4;
        // D s_8_2: cast zx s_8_0 -> bv
        let s_8_2: Bits = Bits::new(s_8_0 as u128, 8u16);
        // C s_8_3: const #1s : i64
        let s_8_3: i64 = 1;
        // C s_8_4: cast zx s_8_3 -> i
        let s_8_4: i128 = (i128::try_from(s_8_3).unwrap());
        // C s_8_5: const #3s : i
        let s_8_5: i128 = 3;
        // C s_8_6: add s_8_5 s_8_4
        let s_8_6: i128 = (s_8_5 + s_8_4);
        // D s_8_7: bit-extract s_8_2 s_8_1 s_8_6
        let s_8_7: Bits = (Bits::new(
            ((s_8_2) >> (s_8_1)).value(),
            u16::try_from(s_8_6).unwrap(),
        ));
        // D s_8_8: cast reint s_8_7 -> u8
        let s_8_8: u8 = (s_8_7.value() as u8);
        // D s_8_9: cast zx s_8_8 -> bv
        let s_8_9: Bits = Bits::new(s_8_8 as u128, 4u16);
        // C s_8_10: const #0u : u8
        let s_8_10: u8 = 0;
        // C s_8_11: cast zx s_8_10 -> bv
        let s_8_11: Bits = Bits::new(s_8_10 as u128, 4u16);
        // D s_8_12: cmp-eq s_8_9 s_8_11
        let s_8_12: bool = ((s_8_9) == (s_8_11));
        // D s_8_13: not s_8_12
        let s_8_13: bool = !s_8_12;
        // N s_8_14: branch s_8_13 b10 b9
        if s_8_13 {
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
        // C s_9_0: const #0s : i
        let s_9_0: i128 = 0;
        // D s_9_1: read-var attr:u8
        let s_9_1: u8 = fn_state.attr;
        // D s_9_2: cast zx s_9_1 -> bv
        let s_9_2: Bits = Bits::new(s_9_1 as u128, 8u16);
        // C s_9_3: const #1s : i64
        let s_9_3: i64 = 1;
        // C s_9_4: cast zx s_9_3 -> i
        let s_9_4: i128 = (i128::try_from(s_9_3).unwrap());
        // C s_9_5: const #1s : i
        let s_9_5: i128 = 1;
        // C s_9_6: add s_9_5 s_9_4
        let s_9_6: i128 = (s_9_5 + s_9_4);
        // D s_9_7: bit-extract s_9_2 s_9_0 s_9_6
        let s_9_7: Bits = (Bits::new(
            ((s_9_2) >> (s_9_0)).value(),
            u16::try_from(s_9_6).unwrap(),
        ));
        // D s_9_8: cast reint s_9_7 -> u8
        let s_9_8: u8 = (s_9_7.value() as u8);
        // D s_9_9: cast zx s_9_8 -> bv
        let s_9_9: Bits = Bits::new(s_9_8 as u128, 2u16);
        // C s_9_10: const #0u : u8
        let s_9_10: u8 = 0;
        // C s_9_11: cast zx s_9_10 -> bv
        let s_9_11: Bits = Bits::new(s_9_10 as u128, 2u16);
        // D s_9_12: cmp-ne s_9_9 s_9_11
        let s_9_12: bool = ((s_9_9) != (s_9_11));
        // D s_9_13: write-var return_value <= s_9_12
        fn_state.return_value = s_9_12;
        // N s_9_14: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_10_0: read-var attr:u8
        let s_10_0: u8 = fn_state.attr;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 8u16);
        // C s_10_2: const #64u : u8
        let s_10_2: u8 = 64;
        // C s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 8u16);
        // D s_10_4: cmp-eq s_10_1 s_10_3
        let s_10_4: bool = ((s_10_1) == (s_10_3));
        // D s_10_5: not s_10_4
        let s_10_5: bool = !s_10_4;
        // N s_10_6: branch s_10_5 b15 b11
        if s_10_5 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_11_0: read-var s1aarch64:u8
        let s_11_0: bool = fn_state.s1aarch64;
        // N s_11_1: branch s_11_0 b14 b12
        if s_11_0 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var gs#12843 <= s_12_0
        fn_state.gs_12843 = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_13_0: read-var gs#12843:u8
        let s_13_0: bool = fn_state.gs_12843;
        // D s_13_1: not s_13_0
        let s_13_1: bool = !s_13_0;
        // D s_13_2: write-var return_value <= s_13_1
        fn_state.return_value = s_13_1;
        // N s_13_3: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call HaveFeatXS(s_14_0)
        let s_14_1: bool = HaveFeatXS(state, tracer, s_14_0);
        // D s_14_2: write-var gs#12843 <= s_14_1
        fn_state.gs_12843 = s_14_1;
        // N s_14_3: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_15_0: read-var attr:u8
        let s_15_0: u8 = fn_state.attr;
        // D s_15_1: cast zx s_15_0 -> bv
        let s_15_1: Bits = Bits::new(s_15_0 as u128, 8u16);
        // C s_15_2: const #160u : u8
        let s_15_2: u8 = 160;
        // C s_15_3: cast zx s_15_2 -> bv
        let s_15_3: Bits = Bits::new(s_15_2 as u128, 8u16);
        // D s_15_4: cmp-eq s_15_1 s_15_3
        let s_15_4: bool = ((s_15_1) == (s_15_3));
        // D s_15_5: not s_15_4
        let s_15_5: bool = !s_15_4;
        // N s_15_6: branch s_15_5 b20 b16
        if s_15_5 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_16_0: read-var s1aarch64:u8
        let s_16_0: bool = fn_state.s1aarch64;
        // N s_16_1: branch s_16_0 b19 b17
        if s_16_0 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var gs#12845 <= s_17_0
        fn_state.gs_12845 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_18_0: read-var gs#12845:u8
        let s_18_0: bool = fn_state.gs_12845;
        // D s_18_1: not s_18_0
        let s_18_1: bool = !s_18_0;
        // D s_18_2: write-var return_value <= s_18_1
        fn_state.return_value = s_18_1;
        // N s_18_3: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_19_0: const #() : ()
        let s_19_0: () = ();
        // S s_19_1: call HaveFeatXS(s_19_0)
        let s_19_1: bool = HaveFeatXS(state, tracer, s_19_0);
        // D s_19_2: write-var gs#12845 <= s_19_1
        fn_state.gs_12845 = s_19_1;
        // N s_19_3: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_20_0: read-var attr:u8
        let s_20_0: u8 = fn_state.attr;
        // D s_20_1: cast zx s_20_0 -> bv
        let s_20_1: Bits = Bits::new(s_20_0 as u128, 8u16);
        // C s_20_2: const #240u : u8
        let s_20_2: u8 = 240;
        // C s_20_3: cast zx s_20_2 -> bv
        let s_20_3: Bits = Bits::new(s_20_2 as u128, 8u16);
        // D s_20_4: cmp-eq s_20_1 s_20_3
        let s_20_4: bool = ((s_20_1) == (s_20_3));
        // D s_20_5: not s_20_4
        let s_20_5: bool = !s_20_4;
        // N s_20_6: branch s_20_5 b25 b21
        if s_20_5 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_21_0: read-var s1aarch64:u8
        let s_21_0: bool = fn_state.s1aarch64;
        // N s_21_1: branch s_21_0 b24 b22
        if s_21_0 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_22_0: const #0u : u8
        let s_22_0: bool = false;
        // D s_22_1: write-var gs#12847 <= s_22_0
        fn_state.gs_12847 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_23_0: read-var gs#12847:u8
        let s_23_0: bool = fn_state.gs_12847;
        // D s_23_1: not s_23_0
        let s_23_1: bool = !s_23_0;
        // D s_23_2: write-var return_value <= s_23_1
        fn_state.return_value = s_23_1;
        // N s_23_3: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_24_0: const #() : ()
        let s_24_0: () = ();
        // S s_24_1: call HaveMTE2Ext(s_24_0)
        let s_24_1: bool = HaveMTE2Ext(state, tracer, s_24_0);
        // D s_24_2: write-var gs#12847 <= s_24_1
        fn_state.gs_12847 = s_24_1;
        // N s_24_3: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_25_0: read-var attr:u8
        let s_25_0: u8 = fn_state.attr;
        // C s_25_1: const #0s : i
        let s_25_1: i128 = 0;
        // D s_25_2: cast zx s_25_0 -> bv
        let s_25_2: Bits = Bits::new(s_25_0 as u128, 8u16);
        // C s_25_3: const #1s : i64
        let s_25_3: i64 = 1;
        // C s_25_4: cast zx s_25_3 -> i
        let s_25_4: i128 = (i128::try_from(s_25_3).unwrap());
        // C s_25_5: const #3s : i
        let s_25_5: i128 = 3;
        // C s_25_6: add s_25_5 s_25_4
        let s_25_6: i128 = (s_25_5 + s_25_4);
        // D s_25_7: bit-extract s_25_2 s_25_1 s_25_6
        let s_25_7: Bits = (Bits::new(
            ((s_25_2) >> (s_25_1)).value(),
            u16::try_from(s_25_6).unwrap(),
        ));
        // D s_25_8: cast reint s_25_7 -> u8
        let s_25_8: u8 = (s_25_7.value() as u8);
        // D s_25_9: cast zx s_25_8 -> bv
        let s_25_9: Bits = Bits::new(s_25_8 as u128, 4u16);
        // C s_25_10: const #0u : u8
        let s_25_10: u8 = 0;
        // C s_25_11: cast zx s_25_10 -> bv
        let s_25_11: Bits = Bits::new(s_25_10 as u128, 4u16);
        // D s_25_12: cmp-eq s_25_9 s_25_11
        let s_25_12: bool = ((s_25_9) == (s_25_11));
        // D s_25_13: not s_25_12
        let s_25_13: bool = !s_25_12;
        // N s_25_14: branch s_25_13 b27 b26
        if s_25_13 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_26(state, tracer, fn_state);
        };
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_26_0: const #1u : u8
        let s_26_0: bool = true;
        // D s_26_1: write-var return_value <= s_26_0
        fn_state.return_value = s_26_0;
        // N s_26_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_27_0: const #0u : u8
        let s_27_0: bool = false;
        // D s_27_1: write-var return_value <= s_27_0
        fn_state.return_value = s_27_0;
        // N s_27_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_28_0: const #0s : i
        let s_28_0: i128 = 0;
        // D s_28_1: read-var b__0:u8
        let s_28_1: u8 = fn_state.b__0;
        // D s_28_2: cast zx s_28_1 -> bv
        let s_28_2: Bits = Bits::new(s_28_1 as u128, 8u16);
        // C s_28_3: const #1s : i64
        let s_28_3: i64 = 1;
        // C s_28_4: cast zx s_28_3 -> i
        let s_28_4: i128 = (i128::try_from(s_28_3).unwrap());
        // C s_28_5: const #1s : i
        let s_28_5: i128 = 1;
        // C s_28_6: add s_28_5 s_28_4
        let s_28_6: i128 = (s_28_5 + s_28_4);
        // D s_28_7: bit-extract s_28_2 s_28_0 s_28_6
        let s_28_7: Bits = (Bits::new(
            ((s_28_2) >> (s_28_0)).value(),
            u16::try_from(s_28_6).unwrap(),
        ));
        // D s_28_8: cast reint s_28_7 -> u8
        let s_28_8: u8 = (s_28_7.value() as u8);
        // D s_28_9: cast zx s_28_8 -> bv
        let s_28_9: Bits = Bits::new(s_28_8 as u128, 2u16);
        // C s_28_10: const #1u : u8
        let s_28_10: u8 = 1;
        // C s_28_11: cast zx s_28_10 -> bv
        let s_28_11: Bits = Bits::new(s_28_10 as u128, 2u16);
        // D s_28_12: cmp-eq s_28_9 s_28_11
        let s_28_12: bool = ((s_28_9) == (s_28_11));
        // D s_28_13: write-var gs#12835 <= s_28_12
        fn_state.gs_12835 = s_28_12;
        // N s_28_14: jump b2
        return block_2(state, tracer, fn_state);
    }
}
