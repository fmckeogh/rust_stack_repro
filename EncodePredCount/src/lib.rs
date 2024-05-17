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
use Zeros::*;
use CurrentVL_read::*;
use integer_subrange::*;
use common::*;
pub fn EncodePredCount<T: Tracer>(
    state: &mut State,
    tracer: &T,
    esize: i128,
    elements: i128,
    count_in: i128,
    invert_in: bool,
    width: i128,
) -> Bits {
    #[derive(Default)]
    struct FunctionState {
        invert: bool,
        gs_22907: bool,
        gs_22905: bool,
        pred: u16,
        return_value: Bits,
        count: i128,
        gs_22904: bool,
        countshadow_407: i128,
        widthshadow_406: i128,
        inv: bool,
        gs_22903: bool,
        esize: i128,
        elements: i128,
        count_in: i128,
        invert_in: bool,
        width: i128,
    }
    let fn_state = FunctionState {
        esize,
        elements,
        count_in,
        invert_in,
        width,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_0_0: read-var width:i
        let s_0_0: i128 = fn_state.width;
        // D s_0_1: write-var widthshadow#406 <= s_0_0
        fn_state.widthshadow_406 = s_0_0;
        // C s_0_2: const #() : ()
        let s_0_2: () = ();
        // S s_0_3: call CurrentVL_read(s_0_2)
        let s_0_3: i64 = CurrentVL_read(state, tracer, s_0_2);
        // D s_0_4: read-var count_in:i
        let s_0_4: i128 = fn_state.count_in;
        // D s_0_5: write-var count <= s_0_4
        fn_state.count = s_0_4;
        // D s_0_6: read-var invert_in:u8
        let s_0_6: bool = fn_state.invert_in;
        // D s_0_7: write-var invert <= s_0_6
        fn_state.invert = s_0_6;
        // C s_0_8: const #8s : i
        let s_0_8: i128 = 8;
        // S s_0_9: cast zx s_0_3 -> i
        let s_0_9: i128 = (i128::try_from(s_0_3).unwrap());
        // S s_0_10: div s_0_9 s_0_8
        let s_0_10: i128 = ((s_0_9) / (s_0_8));
        // S s_0_11: cast reint s_0_10 -> i64
        let s_0_11: i64 = (s_0_10 as i64);
        // S s_0_12: cast zx s_0_11 -> i
        let s_0_12: i128 = (i128::try_from(s_0_11).unwrap());
        // D s_0_13: read-var widthshadow#406:i
        let s_0_13: i128 = fn_state.widthshadow_406;
        // D s_0_14: cmp-eq s_0_13 s_0_12
        let s_0_14: bool = ((s_0_13) == (s_0_12));
        // N s_0_15: assert s_0_14
        let s_0_15: () = assert!(s_0_14);
        // C s_0_16: const #8s : i
        let s_0_16: i128 = 8;
        // D s_0_17: read-var esize:i
        let s_0_17: i128 = fn_state.esize;
        // D s_0_18: cmp-eq s_0_17 s_0_16
        let s_0_18: bool = ((s_0_17) == (s_0_16));
        // N s_0_19: branch s_0_18 b32 b1
        if s_0_18 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_1_0: const #16s : i
        let s_1_0: i128 = 16;
        // D s_1_1: read-var esize:i
        let s_1_1: i128 = fn_state.esize;
        // D s_1_2: cmp-eq s_1_1 s_1_0
        let s_1_2: bool = ((s_1_1) == (s_1_0));
        // N s_1_3: branch s_1_2 b31 b2
        if s_1_2 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_2_0: const #32s : i
        let s_2_0: i128 = 32;
        // D s_2_1: read-var esize:i
        let s_2_1: i128 = fn_state.esize;
        // D s_2_2: cmp-eq s_2_1 s_2_0
        let s_2_2: bool = ((s_2_1) == (s_2_0));
        // N s_2_3: branch s_2_2 b30 b3
        if s_2_2 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_3_0: const #64s : i
        let s_3_0: i128 = 64;
        // D s_3_1: read-var esize:i
        let s_3_1: i128 = fn_state.esize;
        // D s_3_2: cmp-eq s_3_1 s_3_0
        let s_3_2: bool = ((s_3_1) == (s_3_0));
        // D s_3_3: write-var gs#22903 <= s_3_2
        fn_state.gs_22903 = s_3_2;
        // N s_3_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_4_0: read-var gs#22903:u8
        let s_4_0: bool = fn_state.gs_22903;
        // D s_4_1: write-var gs#22904 <= s_4_0
        fn_state.gs_22904 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_5_0: read-var gs#22904:u8
        let s_5_0: bool = fn_state.gs_22904;
        // D s_5_1: write-var gs#22905 <= s_5_0
        fn_state.gs_22905 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_6_0: read-var gs#22905:u8
        let s_6_0: bool = fn_state.gs_22905;
        // N s_6_1: assert s_6_0
        let s_6_1: () = assert!(s_6_0);
        // C s_6_2: const #0s : i
        let s_6_2: i128 = 0;
        // D s_6_3: read-var count:i
        let s_6_3: i128 = fn_state.count;
        // D s_6_4: cmp-ge s_6_3 s_6_2
        let s_6_4: bool = ((s_6_3) >= (s_6_2));
        // N s_6_5: branch s_6_4 b29 b7
        if s_6_4 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#22907 <= s_7_0
        fn_state.gs_22907 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_8_0: read-var gs#22907:u8
        let s_8_0: bool = fn_state.gs_22907;
        // N s_8_1: assert s_8_0
        let s_8_1: () = assert!(s_8_0);
        // C s_8_2: const #0s : i
        let s_8_2: i128 = 0;
        // D s_8_3: read-var count:i
        let s_8_3: i128 = fn_state.count;
        // D s_8_4: cmp-eq s_8_3 s_8_2
        let s_8_4: bool = ((s_8_3) == (s_8_2));
        // N s_8_5: branch s_8_4 b28 b9
        if s_8_4 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_9_0: read-var invert:u8
        let s_9_0: bool = fn_state.invert;
        // N s_9_1: branch s_9_0 b27 b10
        if s_9_0 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_10_0: read-var count:i
        let s_10_0: i128 = fn_state.count;
        // D s_10_1: read-var elements:i
        let s_10_1: i128 = fn_state.elements;
        // D s_10_2: cmp-eq s_10_0 s_10_1
        let s_10_2: bool = ((s_10_0) == (s_10_1));
        // N s_10_3: branch s_10_2 b26 b11
        if s_10_2 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // N s_11_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_12_0: read-var count:i
        let s_12_0: i128 = fn_state.count;
        // D s_12_1: write-var countshadow#407 <= s_12_0
        fn_state.countshadow_407 = s_12_0;
        // D s_12_2: read-var invert:u8
        let s_12_2: bool = fn_state.invert;
        // N s_12_3: branch s_12_2 b25 b13
        if s_12_2 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_13_0: const #0u : u8
        let s_13_0: bool = false;
        // D s_13_1: write-var inv <= s_13_0
        fn_state.inv = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_14_0: read-var esize:i
        let s_14_0: i128 = fn_state.esize;
        // D s_14_1: cast reint s_14_0 -> i64
        let s_14_1: i64 = (s_14_0 as i64);
        // C s_14_2: const #8s : i
        let s_14_2: i128 = 8;
        // D s_14_3: cast zx s_14_1 -> i
        let s_14_3: i128 = (i128::try_from(s_14_1).unwrap());
        // D s_14_4: cmp-eq s_14_3 s_14_2
        let s_14_4: bool = ((s_14_3) == (s_14_2));
        // D s_14_5: not s_14_4
        let s_14_5: bool = !s_14_4;
        // N s_14_6: branch s_14_5 b18 b15
        if s_14_5 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_15_0: const #13s : i
        let s_15_0: i128 = 13;
        // C s_15_1: const #0s : i
        let s_15_1: i128 = 0;
        // D s_15_2: read-var countshadow#407:i
        let s_15_2: i128 = fn_state.countshadow_407;
        // D s_15_3: call integer_subrange(s_15_2, s_15_0, s_15_1)
        let s_15_3: Bits = integer_subrange(state, tracer, s_15_2, s_15_0, s_15_1);
        // D s_15_4: cast reint s_15_3 -> u14
        let s_15_4: u16 = (s_15_3.value() as u16);
        // D s_15_5: read-var inv:u8
        let s_15_5: bool = fn_state.inv;
        // D s_15_6: cast zx s_15_5 -> bv
        let s_15_6: Bits = Bits::new(s_15_5 as u128, 1u16);
        // D s_15_7: cast zx s_15_4 -> bv
        let s_15_7: Bits = Bits::new(s_15_4 as u128, 14u16);
        // D s_15_8: cast reint s_15_6 -> u128
        let s_15_8: u128 = (s_15_6.value() as u128);
        // D s_15_9: size-of s_15_6
        let s_15_9: u16 = s_15_6.length();
        // D s_15_10: cast reint s_15_7 -> u128
        let s_15_10: u128 = (s_15_7.value() as u128);
        // D s_15_11: size-of s_15_7
        let s_15_11: u16 = s_15_7.length();
        // D s_15_12: lsl s_15_8 s_15_11
        let s_15_12: u128 = s_15_8 << s_15_11;
        // D s_15_13: or s_15_12 s_15_10
        let s_15_13: u128 = ((s_15_12) | (s_15_10));
        // D s_15_14: add s_15_9 s_15_11
        let s_15_14: u16 = (s_15_9 + s_15_11);
        // D s_15_15: create-bits s_15_13 s_15_14
        let s_15_15: Bits = Bits::new(s_15_13, s_15_14);
        // D s_15_16: cast reint s_15_15 -> u15
        let s_15_16: u16 = (s_15_15.value() as u16);
        // D s_15_17: cast zx s_15_16 -> bv
        let s_15_17: Bits = Bits::new(s_15_16 as u128, 15u16);
        // C s_15_18: const #1u : u8
        let s_15_18: bool = true;
        // C s_15_19: cast zx s_15_18 -> bv
        let s_15_19: Bits = Bits::new(s_15_18 as u128, 1u16);
        // D s_15_20: cast reint s_15_17 -> u128
        let s_15_20: u128 = (s_15_17.value() as u128);
        // D s_15_21: size-of s_15_17
        let s_15_21: u16 = s_15_17.length();
        // C s_15_22: cast reint s_15_19 -> u128
        let s_15_22: u128 = (s_15_19.value() as u128);
        // D s_15_23: size-of s_15_19
        let s_15_23: u16 = s_15_19.length();
        // D s_15_24: lsl s_15_20 s_15_23
        let s_15_24: u128 = s_15_20 << s_15_23;
        // D s_15_25: or s_15_24 s_15_22
        let s_15_25: u128 = ((s_15_24) | (s_15_22));
        // D s_15_26: add s_15_21 s_15_23
        let s_15_26: u16 = (s_15_21 + s_15_23);
        // D s_15_27: create-bits s_15_25 s_15_26
        let s_15_27: Bits = Bits::new(s_15_25, s_15_26);
        // D s_15_28: cast reint s_15_27 -> u16
        let s_15_28: u16 = (s_15_27.value() as u16);
        // D s_15_29: write-var pred <= s_15_28
        fn_state.pred = s_15_28;
        // N s_15_30: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_16_0: read-var pred:u16
        let s_16_0: u16 = fn_state.pred;
        // D s_16_1: cast zx s_16_0 -> bv
        let s_16_1: Bits = Bits::new(s_16_0 as u128, 16u16);
        // D s_16_2: read-var widthshadow#406:i
        let s_16_2: i128 = fn_state.widthshadow_406;
        // D s_16_3: bits-cast zx s_16_1 -> bv length s_16_2
        let s_16_3: Bits = s_16_1.zero_extend(s_16_2);
        // D s_16_4: write-var return_value <= s_16_3
        fn_state.return_value = s_16_3;
        // N s_16_5: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_17_0: read-var return_value:bv
        let s_17_0: Bits = fn_state.return_value;
        // N s_17_1: return s_17_0
        return s_17_0;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_18_0: read-var esize:i
        let s_18_0: i128 = fn_state.esize;
        // D s_18_1: cast reint s_18_0 -> i64
        let s_18_1: i64 = (s_18_0 as i64);
        // C s_18_2: const #16s : i
        let s_18_2: i128 = 16;
        // D s_18_3: cast zx s_18_1 -> i
        let s_18_3: i128 = (i128::try_from(s_18_1).unwrap());
        // D s_18_4: cmp-eq s_18_3 s_18_2
        let s_18_4: bool = ((s_18_3) == (s_18_2));
        // D s_18_5: not s_18_4
        let s_18_5: bool = !s_18_4;
        // N s_18_6: branch s_18_5 b20 b19
        if s_18_5 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_19_0: const #12s : i
        let s_19_0: i128 = 12;
        // C s_19_1: const #0s : i
        let s_19_1: i128 = 0;
        // D s_19_2: read-var countshadow#407:i
        let s_19_2: i128 = fn_state.countshadow_407;
        // D s_19_3: call integer_subrange(s_19_2, s_19_0, s_19_1)
        let s_19_3: Bits = integer_subrange(state, tracer, s_19_2, s_19_0, s_19_1);
        // D s_19_4: cast reint s_19_3 -> u13
        let s_19_4: u16 = (s_19_3.value() as u16);
        // D s_19_5: read-var inv:u8
        let s_19_5: bool = fn_state.inv;
        // D s_19_6: cast zx s_19_5 -> bv
        let s_19_6: Bits = Bits::new(s_19_5 as u128, 1u16);
        // D s_19_7: cast zx s_19_4 -> bv
        let s_19_7: Bits = Bits::new(s_19_4 as u128, 13u16);
        // D s_19_8: cast reint s_19_6 -> u128
        let s_19_8: u128 = (s_19_6.value() as u128);
        // D s_19_9: size-of s_19_6
        let s_19_9: u16 = s_19_6.length();
        // D s_19_10: cast reint s_19_7 -> u128
        let s_19_10: u128 = (s_19_7.value() as u128);
        // D s_19_11: size-of s_19_7
        let s_19_11: u16 = s_19_7.length();
        // D s_19_12: lsl s_19_8 s_19_11
        let s_19_12: u128 = s_19_8 << s_19_11;
        // D s_19_13: or s_19_12 s_19_10
        let s_19_13: u128 = ((s_19_12) | (s_19_10));
        // D s_19_14: add s_19_9 s_19_11
        let s_19_14: u16 = (s_19_9 + s_19_11);
        // D s_19_15: create-bits s_19_13 s_19_14
        let s_19_15: Bits = Bits::new(s_19_13, s_19_14);
        // D s_19_16: cast reint s_19_15 -> u14
        let s_19_16: u16 = (s_19_15.value() as u16);
        // D s_19_17: cast zx s_19_16 -> bv
        let s_19_17: Bits = Bits::new(s_19_16 as u128, 14u16);
        // C s_19_18: const #2u : u8
        let s_19_18: u8 = 2;
        // C s_19_19: cast zx s_19_18 -> bv
        let s_19_19: Bits = Bits::new(s_19_18 as u128, 2u16);
        // D s_19_20: cast reint s_19_17 -> u128
        let s_19_20: u128 = (s_19_17.value() as u128);
        // D s_19_21: size-of s_19_17
        let s_19_21: u16 = s_19_17.length();
        // C s_19_22: cast reint s_19_19 -> u128
        let s_19_22: u128 = (s_19_19.value() as u128);
        // D s_19_23: size-of s_19_19
        let s_19_23: u16 = s_19_19.length();
        // D s_19_24: lsl s_19_20 s_19_23
        let s_19_24: u128 = s_19_20 << s_19_23;
        // D s_19_25: or s_19_24 s_19_22
        let s_19_25: u128 = ((s_19_24) | (s_19_22));
        // D s_19_26: add s_19_21 s_19_23
        let s_19_26: u16 = (s_19_21 + s_19_23);
        // D s_19_27: create-bits s_19_25 s_19_26
        let s_19_27: Bits = Bits::new(s_19_25, s_19_26);
        // D s_19_28: cast reint s_19_27 -> u16
        let s_19_28: u16 = (s_19_27.value() as u16);
        // D s_19_29: write-var pred <= s_19_28
        fn_state.pred = s_19_28;
        // N s_19_30: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_20_0: read-var esize:i
        let s_20_0: i128 = fn_state.esize;
        // D s_20_1: cast reint s_20_0 -> i64
        let s_20_1: i64 = (s_20_0 as i64);
        // C s_20_2: const #32s : i
        let s_20_2: i128 = 32;
        // D s_20_3: cast zx s_20_1 -> i
        let s_20_3: i128 = (i128::try_from(s_20_1).unwrap());
        // D s_20_4: cmp-eq s_20_3 s_20_2
        let s_20_4: bool = ((s_20_3) == (s_20_2));
        // D s_20_5: not s_20_4
        let s_20_5: bool = !s_20_4;
        // N s_20_6: branch s_20_5 b22 b21
        if s_20_5 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_21_0: const #11s : i
        let s_21_0: i128 = 11;
        // C s_21_1: const #0s : i
        let s_21_1: i128 = 0;
        // D s_21_2: read-var countshadow#407:i
        let s_21_2: i128 = fn_state.countshadow_407;
        // D s_21_3: call integer_subrange(s_21_2, s_21_0, s_21_1)
        let s_21_3: Bits = integer_subrange(state, tracer, s_21_2, s_21_0, s_21_1);
        // D s_21_4: cast reint s_21_3 -> u12
        let s_21_4: u16 = (s_21_3.value() as u16);
        // D s_21_5: read-var inv:u8
        let s_21_5: bool = fn_state.inv;
        // D s_21_6: cast zx s_21_5 -> bv
        let s_21_6: Bits = Bits::new(s_21_5 as u128, 1u16);
        // D s_21_7: cast zx s_21_4 -> bv
        let s_21_7: Bits = Bits::new(s_21_4 as u128, 12u16);
        // D s_21_8: cast reint s_21_6 -> u128
        let s_21_8: u128 = (s_21_6.value() as u128);
        // D s_21_9: size-of s_21_6
        let s_21_9: u16 = s_21_6.length();
        // D s_21_10: cast reint s_21_7 -> u128
        let s_21_10: u128 = (s_21_7.value() as u128);
        // D s_21_11: size-of s_21_7
        let s_21_11: u16 = s_21_7.length();
        // D s_21_12: lsl s_21_8 s_21_11
        let s_21_12: u128 = s_21_8 << s_21_11;
        // D s_21_13: or s_21_12 s_21_10
        let s_21_13: u128 = ((s_21_12) | (s_21_10));
        // D s_21_14: add s_21_9 s_21_11
        let s_21_14: u16 = (s_21_9 + s_21_11);
        // D s_21_15: create-bits s_21_13 s_21_14
        let s_21_15: Bits = Bits::new(s_21_13, s_21_14);
        // D s_21_16: cast reint s_21_15 -> u13
        let s_21_16: u16 = (s_21_15.value() as u16);
        // D s_21_17: cast zx s_21_16 -> bv
        let s_21_17: Bits = Bits::new(s_21_16 as u128, 13u16);
        // C s_21_18: const #4u : u8
        let s_21_18: u8 = 4;
        // C s_21_19: cast zx s_21_18 -> bv
        let s_21_19: Bits = Bits::new(s_21_18 as u128, 3u16);
        // D s_21_20: cast reint s_21_17 -> u128
        let s_21_20: u128 = (s_21_17.value() as u128);
        // D s_21_21: size-of s_21_17
        let s_21_21: u16 = s_21_17.length();
        // C s_21_22: cast reint s_21_19 -> u128
        let s_21_22: u128 = (s_21_19.value() as u128);
        // D s_21_23: size-of s_21_19
        let s_21_23: u16 = s_21_19.length();
        // D s_21_24: lsl s_21_20 s_21_23
        let s_21_24: u128 = s_21_20 << s_21_23;
        // D s_21_25: or s_21_24 s_21_22
        let s_21_25: u128 = ((s_21_24) | (s_21_22));
        // D s_21_26: add s_21_21 s_21_23
        let s_21_26: u16 = (s_21_21 + s_21_23);
        // D s_21_27: create-bits s_21_25 s_21_26
        let s_21_27: Bits = Bits::new(s_21_25, s_21_26);
        // D s_21_28: cast reint s_21_27 -> u16
        let s_21_28: u16 = (s_21_27.value() as u16);
        // D s_21_29: write-var pred <= s_21_28
        fn_state.pred = s_21_28;
        // N s_21_30: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_22_0: read-var esize:i
        let s_22_0: i128 = fn_state.esize;
        // D s_22_1: cast reint s_22_0 -> i64
        let s_22_1: i64 = (s_22_0 as i64);
        // C s_22_2: const #64s : i
        let s_22_2: i128 = 64;
        // D s_22_3: cast zx s_22_1 -> i
        let s_22_3: i128 = (i128::try_from(s_22_1).unwrap());
        // D s_22_4: cmp-eq s_22_3 s_22_2
        let s_22_4: bool = ((s_22_3) == (s_22_2));
        // D s_22_5: not s_22_4
        let s_22_5: bool = !s_22_4;
        // N s_22_6: branch s_22_5 b24 b23
        if s_22_5 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_23_0: const #10s : i
        let s_23_0: i128 = 10;
        // C s_23_1: const #0s : i
        let s_23_1: i128 = 0;
        // D s_23_2: read-var countshadow#407:i
        let s_23_2: i128 = fn_state.countshadow_407;
        // D s_23_3: call integer_subrange(s_23_2, s_23_0, s_23_1)
        let s_23_3: Bits = integer_subrange(state, tracer, s_23_2, s_23_0, s_23_1);
        // D s_23_4: cast reint s_23_3 -> u11
        let s_23_4: u16 = (s_23_3.value() as u16);
        // D s_23_5: read-var inv:u8
        let s_23_5: bool = fn_state.inv;
        // D s_23_6: cast zx s_23_5 -> bv
        let s_23_6: Bits = Bits::new(s_23_5 as u128, 1u16);
        // D s_23_7: cast zx s_23_4 -> bv
        let s_23_7: Bits = Bits::new(s_23_4 as u128, 11u16);
        // D s_23_8: cast reint s_23_6 -> u128
        let s_23_8: u128 = (s_23_6.value() as u128);
        // D s_23_9: size-of s_23_6
        let s_23_9: u16 = s_23_6.length();
        // D s_23_10: cast reint s_23_7 -> u128
        let s_23_10: u128 = (s_23_7.value() as u128);
        // D s_23_11: size-of s_23_7
        let s_23_11: u16 = s_23_7.length();
        // D s_23_12: lsl s_23_8 s_23_11
        let s_23_12: u128 = s_23_8 << s_23_11;
        // D s_23_13: or s_23_12 s_23_10
        let s_23_13: u128 = ((s_23_12) | (s_23_10));
        // D s_23_14: add s_23_9 s_23_11
        let s_23_14: u16 = (s_23_9 + s_23_11);
        // D s_23_15: create-bits s_23_13 s_23_14
        let s_23_15: Bits = Bits::new(s_23_13, s_23_14);
        // D s_23_16: cast reint s_23_15 -> u12
        let s_23_16: u16 = (s_23_15.value() as u16);
        // D s_23_17: cast zx s_23_16 -> bv
        let s_23_17: Bits = Bits::new(s_23_16 as u128, 12u16);
        // C s_23_18: const #8u : u8
        let s_23_18: u8 = 8;
        // C s_23_19: cast zx s_23_18 -> bv
        let s_23_19: Bits = Bits::new(s_23_18 as u128, 4u16);
        // D s_23_20: cast reint s_23_17 -> u128
        let s_23_20: u128 = (s_23_17.value() as u128);
        // D s_23_21: size-of s_23_17
        let s_23_21: u16 = s_23_17.length();
        // C s_23_22: cast reint s_23_19 -> u128
        let s_23_22: u128 = (s_23_19.value() as u128);
        // D s_23_23: size-of s_23_19
        let s_23_23: u16 = s_23_19.length();
        // D s_23_24: lsl s_23_20 s_23_23
        let s_23_24: u128 = s_23_20 << s_23_23;
        // D s_23_25: or s_23_24 s_23_22
        let s_23_25: u128 = ((s_23_24) | (s_23_22));
        // D s_23_26: add s_23_21 s_23_23
        let s_23_26: u16 = (s_23_21 + s_23_23);
        // D s_23_27: create-bits s_23_25 s_23_26
        let s_23_27: Bits = Bits::new(s_23_25, s_23_26);
        // D s_23_28: cast reint s_23_27 -> u16
        let s_23_28: u16 = (s_23_27.value() as u16);
        // D s_23_29: write-var pred <= s_23_28
        fn_state.pred = s_23_28;
        // N s_23_30: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // N s_24_0: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_25_0: const #1u : u8
        let s_25_0: bool = true;
        // D s_25_1: write-var inv <= s_25_0
        fn_state.inv = s_25_0;
        // N s_25_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_26_0: const #0s : i
        let s_26_0: i128 = 0;
        // D s_26_1: write-var count <= s_26_0
        fn_state.count = s_26_0;
        // C s_26_2: const #1u : u8
        let s_26_2: bool = true;
        // D s_26_3: write-var invert <= s_26_2
        fn_state.invert = s_26_2;
        // N s_26_4: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_27_0: read-var elements:i
        let s_27_0: i128 = fn_state.elements;
        // D s_27_1: read-var count:i
        let s_27_1: i128 = fn_state.count;
        // D s_27_2: sub s_27_0 s_27_1
        let s_27_2: i128 = ((s_27_0) - (s_27_1));
        // D s_27_3: write-var count <= s_27_2
        fn_state.count = s_27_2;
        // N s_27_4: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_28_0: read-var widthshadow#406:i
        let s_28_0: i128 = fn_state.widthshadow_406;
        // D s_28_1: call Zeros(s_28_0)
        let s_28_1: Bits = Zeros(state, tracer, s_28_0);
        // D s_28_2: write-var return_value <= s_28_1
        fn_state.return_value = s_28_1;
        // N s_28_3: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_29_0: read-var count:i
        let s_29_0: i128 = fn_state.count;
        // D s_29_1: read-var elements:i
        let s_29_1: i128 = fn_state.elements;
        // D s_29_2: cmp-le s_29_0 s_29_1
        let s_29_2: bool = ((s_29_0) <= (s_29_1));
        // D s_29_3: write-var gs#22907 <= s_29_2
        fn_state.gs_22907 = s_29_2;
        // N s_29_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_30_0: const #1u : u8
        let s_30_0: bool = true;
        // D s_30_1: write-var gs#22903 <= s_30_0
        fn_state.gs_22903 = s_30_0;
        // N s_30_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_31_0: const #1u : u8
        let s_31_0: bool = true;
        // D s_31_1: write-var gs#22904 <= s_31_0
        fn_state.gs_22904 = s_31_0;
        // N s_31_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_32_0: const #1u : u8
        let s_32_0: bool = true;
        // D s_32_1: write-var gs#22905 <= s_32_0
        fn_state.gs_22905 = s_32_0;
        // N s_32_2: jump b6
        return block_6(state, tracer, fn_state);
    }
}
