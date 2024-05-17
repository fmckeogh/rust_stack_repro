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
use Elem_set::*;
use CeilPow2::*;
use HighestSetBit::*;
use IsZero::*;
use integer_subrange::*;
use u__id::*;
use CurrentVL_read::*;
use Zeros::*;
use common::*;
pub fn CounterToPredicate<T: Tracer>(
    state: &mut State,
    tracer: &T,
    pred: u16,
    width: i128,
) -> Bits {
    #[derive(Default)]
    struct FunctionState {
        invert: bool,
        gs_22818: bool,
        e: i64,
        gs_22842: bool,
        esize: i64,
        VLshadow_403: i64,
        gs_22820: bool,
        ga_17709: u8,
        maxbit: i128,
        countshadow_405: i128,
        widthshadow_402: i128,
        PL: i64,
        gs_22880: i64,
        gs_22852: bool,
        return_value: Bits,
        gs_22822: bool,
        pbit: bool,
        gs_22860: bool,
        count: i128,
        gs_22832: bool,
        psize: i64,
        result: Bits,
        pred: u16,
        width: i128,
    }
    let fn_state = FunctionState {
        pred,
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
        // D s_0_1: write-var widthshadow#402 <= s_0_0
        fn_state.widthshadow_402 = s_0_0;
        // C s_0_2: const #() : ()
        let s_0_2: () = ();
        // S s_0_3: call CurrentVL_read(s_0_2)
        let s_0_3: i64 = CurrentVL_read(state, tracer, s_0_2);
        // C s_0_4: const #8s : i64
        let s_0_4: i64 = 8;
        // D s_0_5: write-var esize <= s_0_4
        fn_state.esize = s_0_4;
        // D s_0_6: write-var VLshadow#403 <= s_0_3
        fn_state.VLshadow_403 = s_0_3;
        // C s_0_7: const #8s : i
        let s_0_7: i128 = 8;
        // D s_0_8: read-var VLshadow#403:i64
        let s_0_8: i64 = fn_state.VLshadow_403;
        // D s_0_9: cast zx s_0_8 -> i
        let s_0_9: i128 = (i128::try_from(s_0_8).unwrap());
        // D s_0_10: div s_0_9 s_0_7
        let s_0_10: i128 = ((s_0_9) / (s_0_7));
        // D s_0_11: cast reint s_0_10 -> i64
        let s_0_11: i64 = (s_0_10 as i64);
        // D s_0_12: write-var PL <= s_0_11
        fn_state.PL = s_0_11;
        // C s_0_13: const #4s : i
        let s_0_13: i128 = 4;
        // D s_0_14: read-var PL:i64
        let s_0_14: i64 = fn_state.PL;
        // D s_0_15: cast zx s_0_14 -> i
        let s_0_15: i128 = (i128::try_from(s_0_14).unwrap());
        // D s_0_16: mul s_0_15 s_0_13
        let s_0_16: i128 = ((s_0_15) * (s_0_13));
        // D s_0_17: cast reint s_0_16 -> i64
        let s_0_17: i64 = (s_0_16 as i64);
        // D s_0_18: cast zx s_0_17 -> i
        let s_0_18: i128 = (i128::try_from(s_0_17).unwrap());
        // D s_0_19: call CeilPow2(s_0_18)
        let s_0_19: i128 = CeilPow2(state, tracer, s_0_18);
        // C s_0_20: const #15s : i
        let s_0_20: i128 = 15;
        // C s_0_21: const #0s : i
        let s_0_21: i128 = 0;
        // D s_0_22: call integer_subrange(s_0_19, s_0_20, s_0_21)
        let s_0_22: Bits = integer_subrange(state, tracer, s_0_19, s_0_20, s_0_21);
        // D s_0_23: cast reint s_0_22 -> u16
        let s_0_23: u16 = (s_0_22.value() as u16);
        // D s_0_24: cast zx s_0_23 -> bv
        let s_0_24: Bits = Bits::new(s_0_23 as u128, 16u16);
        // D s_0_25: call HighestSetBit(s_0_24)
        let s_0_25: i128 = HighestSetBit(state, tracer, s_0_24);
        // D s_0_26: write-var maxbit <= s_0_25
        fn_state.maxbit = s_0_25;
        // C s_0_27: const #14s : i
        let s_0_27: i128 = 14;
        // D s_0_28: read-var maxbit:i
        let s_0_28: i128 = fn_state.maxbit;
        // D s_0_29: cmp-le s_0_28 s_0_27
        let s_0_29: bool = ((s_0_28) <= (s_0_27));
        // N s_0_30: assert s_0_29
        let s_0_30: () = assert!(s_0_29);
        // C s_0_31: const #15s : i
        let s_0_31: i128 = 15;
        // D s_0_32: read-var pred:u16
        let s_0_32: u16 = fn_state.pred;
        // D s_0_33: cast zx s_0_32 -> bv
        let s_0_33: Bits = Bits::new(s_0_32 as u128, 16u16);
        // C s_0_34: const #1u : u64
        let s_0_34: u64 = 1;
        // D s_0_35: bit-extract s_0_33 s_0_31 s_0_34
        let s_0_35: Bits = (Bits::new(
            ((s_0_33) >> (s_0_31)).value(),
            u16::try_from(s_0_34).unwrap(),
        ));
        // D s_0_36: cast reint s_0_35 -> u8
        let s_0_36: bool = ((s_0_35.value()) != 0);
        // C s_0_37: const #0s : i
        let s_0_37: i128 = 0;
        // C s_0_38: const #0u : u64
        let s_0_38: u64 = 0;
        // D s_0_39: cast zx s_0_36 -> u64
        let s_0_39: u64 = (s_0_36 as u64);
        // C s_0_40: const #1u : u64
        let s_0_40: u64 = 1;
        // D s_0_41: and s_0_39 s_0_40
        let s_0_41: u64 = ((s_0_39) & (s_0_40));
        // D s_0_42: cmp-eq s_0_41 s_0_40
        let s_0_42: bool = ((s_0_41) == (s_0_40));
        // D s_0_43: lsl s_0_39 s_0_37
        let s_0_43: u64 = s_0_39 << s_0_37;
        // D s_0_44: or s_0_38 s_0_43
        let s_0_44: u64 = ((s_0_38) | (s_0_43));
        // D s_0_45: cmpl s_0_43
        let s_0_45: u64 = !s_0_43;
        // D s_0_46: and s_0_38 s_0_45
        let s_0_46: u64 = ((s_0_38) & (s_0_45));
        // D s_0_47: select s_0_42 s_0_44 s_0_46
        let s_0_47: u64 = if s_0_42 { s_0_44 } else { s_0_46 };
        // D s_0_48: cast trunc s_0_47 -> u8
        let s_0_48: bool = ((s_0_47) != 0);
        // D s_0_49: cast zx s_0_48 -> bv
        let s_0_49: Bits = Bits::new(s_0_48 as u128, 1u16);
        // C s_0_50: const #1u : u8
        let s_0_50: bool = true;
        // C s_0_51: cast zx s_0_50 -> bv
        let s_0_51: Bits = Bits::new(s_0_50 as u128, 1u16);
        // D s_0_52: cmp-eq s_0_49 s_0_51
        let s_0_52: bool = ((s_0_49) == (s_0_51));
        // D s_0_53: write-var invert <= s_0_52
        fn_state.invert = s_0_52;
        // D s_0_54: read-var PL:i64
        let s_0_54: i64 = fn_state.PL;
        // D s_0_55: cast zx s_0_54 -> i
        let s_0_55: i128 = (i128::try_from(s_0_54).unwrap());
        // D s_0_56: read-var widthshadow#402:i
        let s_0_56: i128 = fn_state.widthshadow_402;
        // D s_0_57: cmp-eq s_0_56 s_0_55
        let s_0_57: bool = ((s_0_56) == (s_0_55));
        // N s_0_58: branch s_0_57 b42 b1
        if s_0_57 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_1_0: const #2s : i
        let s_1_0: i128 = 2;
        // D s_1_1: read-var PL:i64
        let s_1_1: i64 = fn_state.PL;
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (i128::try_from(s_1_1).unwrap());
        // D s_1_3: mul s_1_2 s_1_0
        let s_1_3: i128 = ((s_1_2) * (s_1_0));
        // D s_1_4: cast reint s_1_3 -> i64
        let s_1_4: i64 = (s_1_3 as i64);
        // D s_1_5: cast zx s_1_4 -> i
        let s_1_5: i128 = (i128::try_from(s_1_4).unwrap());
        // D s_1_6: read-var widthshadow#402:i
        let s_1_6: i128 = fn_state.widthshadow_402;
        // D s_1_7: cmp-eq s_1_6 s_1_5
        let s_1_7: bool = ((s_1_6) == (s_1_5));
        // D s_1_8: write-var gs#22818 <= s_1_7
        fn_state.gs_22818 = s_1_7;
        // N s_1_9: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_2_0: read-var gs#22818:u8
        let s_2_0: bool = fn_state.gs_22818;
        // N s_2_1: branch s_2_0 b41 b3
        if s_2_0 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_3_0: const #3s : i
        let s_3_0: i128 = 3;
        // D s_3_1: read-var PL:i64
        let s_3_1: i64 = fn_state.PL;
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // D s_3_3: mul s_3_2 s_3_0
        let s_3_3: i128 = ((s_3_2) * (s_3_0));
        // D s_3_4: cast reint s_3_3 -> i64
        let s_3_4: i64 = (s_3_3 as i64);
        // D s_3_5: cast zx s_3_4 -> i
        let s_3_5: i128 = (i128::try_from(s_3_4).unwrap());
        // D s_3_6: read-var widthshadow#402:i
        let s_3_6: i128 = fn_state.widthshadow_402;
        // D s_3_7: cmp-eq s_3_6 s_3_5
        let s_3_7: bool = ((s_3_6) == (s_3_5));
        // D s_3_8: write-var gs#22820 <= s_3_7
        fn_state.gs_22820 = s_3_7;
        // N s_3_9: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_4_0: read-var gs#22820:u8
        let s_4_0: bool = fn_state.gs_22820;
        // N s_4_1: branch s_4_0 b40 b5
        if s_4_0 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_5_0: const #4s : i
        let s_5_0: i128 = 4;
        // D s_5_1: read-var PL:i64
        let s_5_1: i64 = fn_state.PL;
        // D s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: mul s_5_2 s_5_0
        let s_5_3: i128 = ((s_5_2) * (s_5_0));
        // D s_5_4: cast reint s_5_3 -> i64
        let s_5_4: i64 = (s_5_3 as i64);
        // D s_5_5: cast zx s_5_4 -> i
        let s_5_5: i128 = (i128::try_from(s_5_4).unwrap());
        // D s_5_6: read-var widthshadow#402:i
        let s_5_6: i128 = fn_state.widthshadow_402;
        // D s_5_7: cmp-eq s_5_6 s_5_5
        let s_5_7: bool = ((s_5_6) == (s_5_5));
        // D s_5_8: write-var gs#22822 <= s_5_7
        fn_state.gs_22822 = s_5_7;
        // N s_5_9: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_6_0: read-var gs#22822:u8
        let s_6_0: bool = fn_state.gs_22822;
        // N s_6_1: assert s_6_0
        let s_6_1: () = assert!(s_6_0);
        // C s_6_2: const #0s : i
        let s_6_2: i128 = 0;
        // D s_6_3: read-var pred:u16
        let s_6_3: u16 = fn_state.pred;
        // D s_6_4: cast zx s_6_3 -> bv
        let s_6_4: Bits = Bits::new(s_6_3 as u128, 16u16);
        // C s_6_5: const #1s : i64
        let s_6_5: i64 = 1;
        // C s_6_6: cast zx s_6_5 -> i
        let s_6_6: i128 = (i128::try_from(s_6_5).unwrap());
        // C s_6_7: const #3s : i
        let s_6_7: i128 = 3;
        // C s_6_8: add s_6_7 s_6_6
        let s_6_8: i128 = (s_6_7 + s_6_6);
        // D s_6_9: bit-extract s_6_4 s_6_2 s_6_8
        let s_6_9: Bits = (Bits::new(
            ((s_6_4) >> (s_6_2)).value(),
            u16::try_from(s_6_8).unwrap(),
        ));
        // D s_6_10: cast reint s_6_9 -> u8
        let s_6_10: u8 = (s_6_9.value() as u8);
        // D s_6_11: cast zx s_6_10 -> bv
        let s_6_11: Bits = Bits::new(s_6_10 as u128, 4u16);
        // D s_6_12: call IsZero(s_6_11)
        let s_6_12: bool = IsZero(state, tracer, s_6_11);
        // N s_6_13: branch s_6_12 b39 b7
        if s_6_12 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_7_0: const #0s : i
        let s_7_0: i128 = 0;
        // D s_7_1: read-var pred:u16
        let s_7_1: u16 = fn_state.pred;
        // D s_7_2: cast zx s_7_1 -> bv
        let s_7_2: Bits = Bits::new(s_7_1 as u128, 16u16);
        // C s_7_3: const #1s : i64
        let s_7_3: i64 = 1;
        // C s_7_4: cast zx s_7_3 -> i
        let s_7_4: i128 = (i128::try_from(s_7_3).unwrap());
        // C s_7_5: const #3s : i
        let s_7_5: i128 = 3;
        // C s_7_6: add s_7_5 s_7_4
        let s_7_6: i128 = (s_7_5 + s_7_4);
        // D s_7_7: bit-extract s_7_2 s_7_0 s_7_6
        let s_7_7: Bits = (Bits::new(
            ((s_7_2) >> (s_7_0)).value(),
            u16::try_from(s_7_6).unwrap(),
        ));
        // D s_7_8: cast reint s_7_7 -> u8
        let s_7_8: u8 = (s_7_7.value() as u8);
        // D s_7_9: write-var ga#17709 <= s_7_8
        fn_state.ga_17709 = s_7_8;
        // D s_7_10: read-var ga#17709:u8
        let s_7_10: u8 = fn_state.ga_17709;
        // C s_7_11: const #0s : i
        let s_7_11: i128 = 0;
        // D s_7_12: cast zx s_7_10 -> bv
        let s_7_12: Bits = Bits::new(s_7_10 as u128, 4u16);
        // C s_7_13: const #1s : i64
        let s_7_13: i64 = 1;
        // C s_7_14: cast zx s_7_13 -> i
        let s_7_14: i128 = (i128::try_from(s_7_13).unwrap());
        // C s_7_15: const #0s : i
        let s_7_15: i128 = 0;
        // C s_7_16: add s_7_15 s_7_14
        let s_7_16: i128 = (s_7_15 + s_7_14);
        // D s_7_17: bit-extract s_7_12 s_7_11 s_7_16
        let s_7_17: Bits = (Bits::new(
            ((s_7_12) >> (s_7_11)).value(),
            u16::try_from(s_7_16).unwrap(),
        ));
        // D s_7_18: cast reint s_7_17 -> u8
        let s_7_18: bool = ((s_7_17.value()) != 0);
        // D s_7_19: cast zx s_7_18 -> bv
        let s_7_19: Bits = Bits::new(s_7_18 as u128, 1u16);
        // C s_7_20: const #1u : u8
        let s_7_20: bool = true;
        // C s_7_21: cast zx s_7_20 -> bv
        let s_7_21: Bits = Bits::new(s_7_20 as u128, 1u16);
        // D s_7_22: cmp-eq s_7_19 s_7_21
        let s_7_22: bool = ((s_7_19) == (s_7_21));
        // D s_7_23: not s_7_22
        let s_7_23: bool = !s_7_22;
        // N s_7_24: branch s_7_23 b23 b8
        if s_7_23 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_8_0: read-var maxbit:i
        let s_8_0: i128 = fn_state.maxbit;
        // D s_8_1: call __id(s_8_0)
        let s_8_1: i128 = u__id(state, tracer, s_8_0);
        // C s_8_2: const #1s : i
        let s_8_2: i128 = 1;
        // D s_8_3: cmp-le s_8_2 s_8_1
        let s_8_3: bool = ((s_8_2) <= (s_8_1));
        // N s_8_4: branch s_8_3 b22 b9
        if s_8_3 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#22832 <= s_9_0
        fn_state.gs_22832 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_10_0: read-var gs#22832:u8
        let s_10_0: bool = fn_state.gs_22832;
        // N s_10_1: assert s_10_0
        let s_10_1: () = assert!(s_10_0);
        // C s_10_2: const #1s : i
        let s_10_2: i128 = 1;
        // D s_10_3: read-var pred:u16
        let s_10_3: u16 = fn_state.pred;
        // D s_10_4: cast zx s_10_3 -> bv
        let s_10_4: Bits = Bits::new(s_10_3 as u128, 16u16);
        // D s_10_5: read-var maxbit:i
        let s_10_5: i128 = fn_state.maxbit;
        // D s_10_6: call unsigned_subrange(s_10_4, s_10_5, s_10_2)
        let s_10_6: i128 = unsigned_subrange(state, tracer, s_10_4, s_10_5, s_10_2);
        // D s_10_7: write-var count <= s_10_6
        fn_state.count = s_10_6;
        // C s_10_8: const #8s : i64
        let s_10_8: i64 = 8;
        // D s_10_9: write-var esize <= s_10_8
        fn_state.esize = s_10_8;
        // N s_10_10: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_11_0: read-var esize:i64
        let s_11_0: i64 = fn_state.esize;
        // D s_11_1: read-var count:i
        let s_11_1: i128 = fn_state.count;
        // D s_11_2: write-var countshadow#405 <= s_11_1
        fn_state.countshadow_405 = s_11_1;
        // C s_11_3: const #4s : i
        let s_11_3: i128 = 4;
        // D s_11_4: read-var VLshadow#403:i64
        let s_11_4: i64 = fn_state.VLshadow_403;
        // D s_11_5: cast zx s_11_4 -> i
        let s_11_5: i128 = (i128::try_from(s_11_4).unwrap());
        // D s_11_6: mul s_11_5 s_11_3
        let s_11_6: i128 = ((s_11_5) * (s_11_3));
        // D s_11_7: cast reint s_11_6 -> i64
        let s_11_7: i64 = (s_11_6 as i64);
        // D s_11_8: cast zx s_11_7 -> i
        let s_11_8: i128 = (i128::try_from(s_11_7).unwrap());
        // D s_11_9: cast zx s_11_0 -> i
        let s_11_9: i128 = (i128::try_from(s_11_0).unwrap());
        // D s_11_10: div s_11_8 s_11_9
        let s_11_10: i128 = ((s_11_8) / (s_11_9));
        // D s_11_11: cast reint s_11_10 -> i64
        let s_11_11: i64 = (s_11_10 as i64);
        // C s_11_12: const #4s : i
        let s_11_12: i128 = 4;
        // D s_11_13: read-var PL:i64
        let s_11_13: i64 = fn_state.PL;
        // D s_11_14: cast zx s_11_13 -> i
        let s_11_14: i128 = (i128::try_from(s_11_13).unwrap());
        // D s_11_15: mul s_11_14 s_11_12
        let s_11_15: i128 = ((s_11_14) * (s_11_12));
        // D s_11_16: cast reint s_11_15 -> i64
        let s_11_16: i64 = (s_11_15 as i64);
        // D s_11_17: cast zx s_11_16 -> i
        let s_11_17: i128 = (i128::try_from(s_11_16).unwrap());
        // D s_11_18: call Zeros(s_11_17)
        let s_11_18: Bits = Zeros(state, tracer, s_11_17);
        // D s_11_19: write-var result <= s_11_18
        fn_state.result = s_11_18;
        // C s_11_20: const #8s : i
        let s_11_20: i128 = 8;
        // D s_11_21: cast zx s_11_0 -> i
        let s_11_21: i128 = (i128::try_from(s_11_0).unwrap());
        // D s_11_22: div s_11_21 s_11_20
        let s_11_22: i128 = ((s_11_21) / (s_11_20));
        // D s_11_23: cast reint s_11_22 -> i64
        let s_11_23: i64 = (s_11_22 as i64);
        // D s_11_24: write-var psize <= s_11_23
        fn_state.psize = s_11_23;
        // C s_11_25: const #0s : i64
        let s_11_25: i64 = 0;
        // C s_11_26: const #1s : i
        let s_11_26: i128 = 1;
        // D s_11_27: cast zx s_11_11 -> i
        let s_11_27: i128 = (i128::try_from(s_11_11).unwrap());
        // D s_11_28: sub s_11_27 s_11_26
        let s_11_28: i128 = ((s_11_27) - (s_11_26));
        // D s_11_29: cast reint s_11_28 -> i64
        let s_11_29: i64 = (s_11_28 as i64);
        // D s_11_30: write-var gs#22880 <= s_11_29
        fn_state.gs_22880 = s_11_29;
        // D s_11_31: write-var e <= s_11_25
        fn_state.e = s_11_25;
        // N s_11_32: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_12_0: read-var e:i64
        let s_12_0: i64 = fn_state.e;
        // D s_12_1: read-var gs#22880:i64
        let s_12_1: i64 = fn_state.gs_22880;
        // D s_12_2: cmp-gt s_12_0 s_12_1
        let s_12_2: bool = ((s_12_0) > (s_12_1));
        // N s_12_3: branch s_12_2 b20 b13
        if s_12_2 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_13_0: read-var e:i64
        let s_13_0: i64 = fn_state.e;
        // D s_13_1: cast zx s_13_0 -> i
        let s_13_1: i128 = (i128::try_from(s_13_0).unwrap());
        // D s_13_2: read-var countshadow#405:i
        let s_13_2: i128 = fn_state.countshadow_405;
        // D s_13_3: cmp-lt s_13_1 s_13_2
        let s_13_3: bool = ((s_13_1) < (s_13_2));
        // N s_13_4: branch s_13_3 b19 b14
        if s_13_3 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // D s_14_1: write-var pbit <= s_14_0
        fn_state.pbit = s_14_0;
        // N s_14_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_15_0: read-var invert:u8
        let s_15_0: bool = fn_state.invert;
        // N s_15_1: branch s_15_0 b18 b16
        if s_15_0 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // N s_16_0: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_17_0: read-var psize:i64
        let s_17_0: i64 = fn_state.psize;
        // D s_17_1: cast zx s_17_0 -> i
        let s_17_1: i128 = (i128::try_from(s_17_0).unwrap());
        // D s_17_2: cast reint s_17_1 -> i64
        let s_17_2: i64 = (s_17_1 as i64);
        // D s_17_3: read-var pbit:u8
        let s_17_3: bool = fn_state.pbit;
        // D s_17_4: cast zx s_17_3 -> bv
        let s_17_4: Bits = Bits::new(s_17_3 as u128, 1u16);
        // D s_17_5: read-var psize:i64
        let s_17_5: i64 = fn_state.psize;
        // D s_17_6: cast zx s_17_5 -> i
        let s_17_6: i128 = (i128::try_from(s_17_5).unwrap());
        // D s_17_7: bits-cast zx s_17_4 -> bv length s_17_6
        let s_17_7: Bits = s_17_4.zero_extend(s_17_6);
        // D s_17_8: read-var e:i64
        let s_17_8: i64 = fn_state.e;
        // D s_17_9: cast zx s_17_8 -> i
        let s_17_9: i128 = (i128::try_from(s_17_8).unwrap());
        // D s_17_10: cast zx s_17_2 -> i
        let s_17_10: i128 = (i128::try_from(s_17_2).unwrap());
        // D s_17_11: read-var result:bv
        let s_17_11: Bits = fn_state.result;
        // D s_17_12: call Elem_set(s_17_11, s_17_9, s_17_10, s_17_7)
        let s_17_12: Bits = Elem_set(state, tracer, s_17_11, s_17_9, s_17_10, s_17_7);
        // D s_17_13: write-var result <= s_17_12
        fn_state.result = s_17_12;
        // D s_17_14: read-var e:i64
        let s_17_14: i64 = fn_state.e;
        // C s_17_15: const #1s : i64
        let s_17_15: i64 = 1;
        // D s_17_16: add s_17_14 s_17_15
        let s_17_16: i64 = (s_17_14 + s_17_15);
        // D s_17_17: write-var e <= s_17_16
        fn_state.e = s_17_16;
        // N s_17_18: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_18_0: read-var pbit:u8
        let s_18_0: bool = fn_state.pbit;
        // D s_18_1: cast zx s_18_0 -> bv
        let s_18_1: Bits = Bits::new(s_18_0 as u128, 1u16);
        // D s_18_2: not s_18_1
        let s_18_2: Bits = !s_18_1;
        // D s_18_3: cast reint s_18_2 -> u8
        let s_18_3: bool = ((s_18_2.value()) != 0);
        // D s_18_4: write-var pbit <= s_18_3
        fn_state.pbit = s_18_3;
        // N s_18_5: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_19_0: const #1u : u8
        let s_19_0: bool = true;
        // D s_19_1: write-var pbit <= s_19_0
        fn_state.pbit = s_19_0;
        // N s_19_2: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_20_0: const #1s : i
        let s_20_0: i128 = 1;
        // D s_20_1: read-var widthshadow#402:i
        let s_20_1: i128 = fn_state.widthshadow_402;
        // D s_20_2: sub s_20_1 s_20_0
        let s_20_2: i128 = ((s_20_1) - (s_20_0));
        // D s_20_3: cast reint s_20_2 -> i64
        let s_20_3: i64 = (s_20_2 as i64);
        // C s_20_4: const #0s : i
        let s_20_4: i128 = 0;
        // D s_20_5: cast zx s_20_3 -> i
        let s_20_5: i128 = (i128::try_from(s_20_3).unwrap());
        // D s_20_6: read-var result:bv
        let s_20_6: Bits = fn_state.result;
        // C s_20_7: const #1s : i64
        let s_20_7: i64 = 1;
        // C s_20_8: cast zx s_20_7 -> i
        let s_20_8: i128 = (i128::try_from(s_20_7).unwrap());
        // D s_20_9: sub s_20_5 s_20_4
        let s_20_9: i128 = ((s_20_5) - (s_20_4));
        // D s_20_10: add s_20_9 s_20_8
        let s_20_10: i128 = (s_20_9 + s_20_8);
        // D s_20_11: bit-extract s_20_6 s_20_4 s_20_10
        let s_20_11: Bits = (Bits::new(
            ((s_20_6) >> (s_20_4)).value(),
            u16::try_from(s_20_10).unwrap(),
        ));
        // D s_20_12: write-var return_value <= s_20_11
        fn_state.return_value = s_20_11;
        // N s_20_13: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_21_0: read-var return_value:bv
        let s_21_0: Bits = fn_state.return_value;
        // N s_21_1: return s_21_0
        return s_21_0;
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_22_0: read-var maxbit:i
        let s_22_0: i128 = fn_state.maxbit;
        // D s_22_1: call __id(s_22_0)
        let s_22_1: i128 = u__id(state, tracer, s_22_0);
        // C s_22_2: const #16s : i
        let s_22_2: i128 = 16;
        // D s_22_3: cmp-lt s_22_1 s_22_2
        let s_22_3: bool = ((s_22_1) < (s_22_2));
        // D s_22_4: write-var gs#22832 <= s_22_3
        fn_state.gs_22832 = s_22_3;
        // N s_22_5: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_23_0: read-var ga#17709:u8
        let s_23_0: u8 = fn_state.ga_17709;
        // C s_23_1: const #0s : i
        let s_23_1: i128 = 0;
        // D s_23_2: cast zx s_23_0 -> bv
        let s_23_2: Bits = Bits::new(s_23_0 as u128, 4u16);
        // C s_23_3: const #1s : i64
        let s_23_3: i64 = 1;
        // C s_23_4: cast zx s_23_3 -> i
        let s_23_4: i128 = (i128::try_from(s_23_3).unwrap());
        // C s_23_5: const #1s : i
        let s_23_5: i128 = 1;
        // C s_23_6: add s_23_5 s_23_4
        let s_23_6: i128 = (s_23_5 + s_23_4);
        // D s_23_7: bit-extract s_23_2 s_23_1 s_23_6
        let s_23_7: Bits = (Bits::new(
            ((s_23_2) >> (s_23_1)).value(),
            u16::try_from(s_23_6).unwrap(),
        ));
        // D s_23_8: cast reint s_23_7 -> u8
        let s_23_8: u8 = (s_23_7.value() as u8);
        // D s_23_9: cast zx s_23_8 -> bv
        let s_23_9: Bits = Bits::new(s_23_8 as u128, 2u16);
        // C s_23_10: const #2u : u8
        let s_23_10: u8 = 2;
        // C s_23_11: cast zx s_23_10 -> bv
        let s_23_11: Bits = Bits::new(s_23_10 as u128, 2u16);
        // D s_23_12: cmp-eq s_23_9 s_23_11
        let s_23_12: bool = ((s_23_9) == (s_23_11));
        // D s_23_13: not s_23_12
        let s_23_13: bool = !s_23_12;
        // N s_23_14: branch s_23_13 b28 b24
        if s_23_13 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_24_0: read-var maxbit:i
        let s_24_0: i128 = fn_state.maxbit;
        // D s_24_1: call __id(s_24_0)
        let s_24_1: i128 = u__id(state, tracer, s_24_0);
        // C s_24_2: const #2s : i
        let s_24_2: i128 = 2;
        // D s_24_3: cmp-le s_24_2 s_24_1
        let s_24_3: bool = ((s_24_2) <= (s_24_1));
        // N s_24_4: branch s_24_3 b27 b25
        if s_24_3 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_25_0: const #0u : u8
        let s_25_0: bool = false;
        // D s_25_1: write-var gs#22842 <= s_25_0
        fn_state.gs_22842 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_26_0: read-var gs#22842:u8
        let s_26_0: bool = fn_state.gs_22842;
        // N s_26_1: assert s_26_0
        let s_26_1: () = assert!(s_26_0);
        // C s_26_2: const #2s : i
        let s_26_2: i128 = 2;
        // D s_26_3: read-var pred:u16
        let s_26_3: u16 = fn_state.pred;
        // D s_26_4: cast zx s_26_3 -> bv
        let s_26_4: Bits = Bits::new(s_26_3 as u128, 16u16);
        // D s_26_5: read-var maxbit:i
        let s_26_5: i128 = fn_state.maxbit;
        // D s_26_6: call unsigned_subrange(s_26_4, s_26_5, s_26_2)
        let s_26_6: i128 = unsigned_subrange(state, tracer, s_26_4, s_26_5, s_26_2);
        // D s_26_7: write-var count <= s_26_6
        fn_state.count = s_26_6;
        // C s_26_8: const #16s : i64
        let s_26_8: i64 = 16;
        // D s_26_9: write-var esize <= s_26_8
        fn_state.esize = s_26_8;
        // N s_26_10: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_27_0: read-var maxbit:i
        let s_27_0: i128 = fn_state.maxbit;
        // D s_27_1: call __id(s_27_0)
        let s_27_1: i128 = u__id(state, tracer, s_27_0);
        // C s_27_2: const #16s : i
        let s_27_2: i128 = 16;
        // D s_27_3: cmp-lt s_27_1 s_27_2
        let s_27_3: bool = ((s_27_1) < (s_27_2));
        // D s_27_4: write-var gs#22842 <= s_27_3
        fn_state.gs_22842 = s_27_3;
        // N s_27_5: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_28_0: read-var ga#17709:u8
        let s_28_0: u8 = fn_state.ga_17709;
        // C s_28_1: const #0s : i
        let s_28_1: i128 = 0;
        // D s_28_2: cast zx s_28_0 -> bv
        let s_28_2: Bits = Bits::new(s_28_0 as u128, 4u16);
        // C s_28_3: const #1s : i64
        let s_28_3: i64 = 1;
        // C s_28_4: cast zx s_28_3 -> i
        let s_28_4: i128 = (i128::try_from(s_28_3).unwrap());
        // C s_28_5: const #2s : i
        let s_28_5: i128 = 2;
        // C s_28_6: add s_28_5 s_28_4
        let s_28_6: i128 = (s_28_5 + s_28_4);
        // D s_28_7: bit-extract s_28_2 s_28_1 s_28_6
        let s_28_7: Bits = (Bits::new(
            ((s_28_2) >> (s_28_1)).value(),
            u16::try_from(s_28_6).unwrap(),
        ));
        // D s_28_8: cast reint s_28_7 -> u8
        let s_28_8: u8 = (s_28_7.value() as u8);
        // D s_28_9: cast zx s_28_8 -> bv
        let s_28_9: Bits = Bits::new(s_28_8 as u128, 3u16);
        // C s_28_10: const #4u : u8
        let s_28_10: u8 = 4;
        // C s_28_11: cast zx s_28_10 -> bv
        let s_28_11: Bits = Bits::new(s_28_10 as u128, 3u16);
        // D s_28_12: cmp-eq s_28_9 s_28_11
        let s_28_12: bool = ((s_28_9) == (s_28_11));
        // D s_28_13: not s_28_12
        let s_28_13: bool = !s_28_12;
        // N s_28_14: branch s_28_13 b33 b29
        if s_28_13 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_29_0: read-var maxbit:i
        let s_29_0: i128 = fn_state.maxbit;
        // D s_29_1: call __id(s_29_0)
        let s_29_1: i128 = u__id(state, tracer, s_29_0);
        // C s_29_2: const #3s : i
        let s_29_2: i128 = 3;
        // D s_29_3: cmp-le s_29_2 s_29_1
        let s_29_3: bool = ((s_29_2) <= (s_29_1));
        // N s_29_4: branch s_29_3 b32 b30
        if s_29_3 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_30_0: const #0u : u8
        let s_30_0: bool = false;
        // D s_30_1: write-var gs#22852 <= s_30_0
        fn_state.gs_22852 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_31_0: read-var gs#22852:u8
        let s_31_0: bool = fn_state.gs_22852;
        // N s_31_1: assert s_31_0
        let s_31_1: () = assert!(s_31_0);
        // C s_31_2: const #3s : i
        let s_31_2: i128 = 3;
        // D s_31_3: read-var pred:u16
        let s_31_3: u16 = fn_state.pred;
        // D s_31_4: cast zx s_31_3 -> bv
        let s_31_4: Bits = Bits::new(s_31_3 as u128, 16u16);
        // D s_31_5: read-var maxbit:i
        let s_31_5: i128 = fn_state.maxbit;
        // D s_31_6: call unsigned_subrange(s_31_4, s_31_5, s_31_2)
        let s_31_6: i128 = unsigned_subrange(state, tracer, s_31_4, s_31_5, s_31_2);
        // D s_31_7: write-var count <= s_31_6
        fn_state.count = s_31_6;
        // C s_31_8: const #32s : i64
        let s_31_8: i64 = 32;
        // D s_31_9: write-var esize <= s_31_8
        fn_state.esize = s_31_8;
        // N s_31_10: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_32_0: read-var maxbit:i
        let s_32_0: i128 = fn_state.maxbit;
        // D s_32_1: call __id(s_32_0)
        let s_32_1: i128 = u__id(state, tracer, s_32_0);
        // C s_32_2: const #16s : i
        let s_32_2: i128 = 16;
        // D s_32_3: cmp-lt s_32_1 s_32_2
        let s_32_3: bool = ((s_32_1) < (s_32_2));
        // D s_32_4: write-var gs#22852 <= s_32_3
        fn_state.gs_22852 = s_32_3;
        // N s_32_5: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_33_0: read-var ga#17709:u8
        let s_33_0: u8 = fn_state.ga_17709;
        // D s_33_1: cast zx s_33_0 -> bv
        let s_33_1: Bits = Bits::new(s_33_0 as u128, 4u16);
        // C s_33_2: const #8u : u8
        let s_33_2: u8 = 8;
        // C s_33_3: cast zx s_33_2 -> bv
        let s_33_3: Bits = Bits::new(s_33_2 as u128, 4u16);
        // D s_33_4: cmp-eq s_33_1 s_33_3
        let s_33_4: bool = ((s_33_1) == (s_33_3));
        // D s_33_5: not s_33_4
        let s_33_5: bool = !s_33_4;
        // N s_33_6: branch s_33_5 b38 b34
        if s_33_5 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_34_0: read-var maxbit:i
        let s_34_0: i128 = fn_state.maxbit;
        // D s_34_1: call __id(s_34_0)
        let s_34_1: i128 = u__id(state, tracer, s_34_0);
        // C s_34_2: const #4s : i
        let s_34_2: i128 = 4;
        // D s_34_3: cmp-le s_34_2 s_34_1
        let s_34_3: bool = ((s_34_2) <= (s_34_1));
        // N s_34_4: branch s_34_3 b37 b35
        if s_34_3 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_35_0: const #0u : u8
        let s_35_0: bool = false;
        // D s_35_1: write-var gs#22860 <= s_35_0
        fn_state.gs_22860 = s_35_0;
        // N s_35_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_36_0: read-var gs#22860:u8
        let s_36_0: bool = fn_state.gs_22860;
        // N s_36_1: assert s_36_0
        let s_36_1: () = assert!(s_36_0);
        // C s_36_2: const #4s : i
        let s_36_2: i128 = 4;
        // D s_36_3: read-var pred:u16
        let s_36_3: u16 = fn_state.pred;
        // D s_36_4: cast zx s_36_3 -> bv
        let s_36_4: Bits = Bits::new(s_36_3 as u128, 16u16);
        // D s_36_5: read-var maxbit:i
        let s_36_5: i128 = fn_state.maxbit;
        // D s_36_6: call unsigned_subrange(s_36_4, s_36_5, s_36_2)
        let s_36_6: i128 = unsigned_subrange(state, tracer, s_36_4, s_36_5, s_36_2);
        // D s_36_7: write-var count <= s_36_6
        fn_state.count = s_36_6;
        // C s_36_8: const #64s : i64
        let s_36_8: i64 = 64;
        // D s_36_9: write-var esize <= s_36_8
        fn_state.esize = s_36_8;
        // N s_36_10: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_37_0: read-var maxbit:i
        let s_37_0: i128 = fn_state.maxbit;
        // D s_37_1: call __id(s_37_0)
        let s_37_1: i128 = u__id(state, tracer, s_37_0);
        // C s_37_2: const #16s : i
        let s_37_2: i128 = 16;
        // D s_37_3: cmp-lt s_37_1 s_37_2
        let s_37_3: bool = ((s_37_1) < (s_37_2));
        // D s_37_4: write-var gs#22860 <= s_37_3
        fn_state.gs_22860 = s_37_3;
        // N s_37_5: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // N s_38_0: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_39_0: read-var widthshadow#402:i
        let s_39_0: i128 = fn_state.widthshadow_402;
        // D s_39_1: call Zeros(s_39_0)
        let s_39_1: Bits = Zeros(state, tracer, s_39_0);
        // D s_39_2: write-var return_value <= s_39_1
        fn_state.return_value = s_39_1;
        // N s_39_3: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_40_0: const #1u : u8
        let s_40_0: bool = true;
        // D s_40_1: write-var gs#22822 <= s_40_0
        fn_state.gs_22822 = s_40_0;
        // N s_40_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_41_0: const #1u : u8
        let s_41_0: bool = true;
        // D s_41_1: write-var gs#22820 <= s_41_0
        fn_state.gs_22820 = s_41_0;
        // N s_41_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_42_0: const #1u : u8
        let s_42_0: bool = true;
        // D s_42_1: write-var gs#22818 <= s_42_0
        fn_state.gs_22818 = s_42_0;
        // N s_42_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
