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
use Elem_set::*;
use u__id::*;
use V_read::*;
use Elem_read::*;
use V_set::*;
use CheckFPAdvSIMDEnabled64::*;
use common::*;
pub fn execute_aarch64_instrs_vector_shift_left_sisd<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    datasize: i128,
    elements: i128,
    esize: i128,
    n: i64,
    shift: i128,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand: Bits,
        gs_168886: bool,
        gs_168892: i64,
        e: i64,
        gs_168884: bool,
        gs_168902: bool,
        gs_168880: bool,
        gs_168908: bool,
        gs_168899: bool,
        gs_168910: bool,
        gs_168909: bool,
        esizeshadow_1843: i128,
        gs_168904: bool,
        gs_168906: bool,
        gs_168882: bool,
        datasizeshadow_1844: i128,
        result: Bits,
        d: i64,
        datasize: i128,
        elements: i128,
        esize: i128,
        n: i64,
        shift: i128,
    }
    let fn_state = FunctionState {
        d,
        datasize,
        elements,
        esize,
        n,
        shift,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var esize:i
        let s_0_0: i128 = fn_state.esize;
        // D s_0_1: write-var esizeshadow#1843 <= s_0_0
        fn_state.esizeshadow_1843 = s_0_0;
        // D s_0_2: read-var datasize:i
        let s_0_2: i128 = fn_state.datasize;
        // D s_0_3: write-var datasizeshadow#1844 <= s_0_2
        fn_state.datasizeshadow_1844 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call CheckFPAdvSIMDEnabled64(s_0_4)
        let s_0_5: () = CheckFPAdvSIMDEnabled64(state, tracer, s_0_4);
        // N s_0_6: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var datasizeshadow#1844:i
        let s_1_0: i128 = fn_state.datasizeshadow_1844;
        // D s_1_1: call __id(s_1_0)
        let s_1_1: i128 = u__id(state, tracer, s_1_0);
        // C s_1_2: const #8s : i
        let s_1_2: i128 = 8;
        // D s_1_3: cmp-eq s_1_1 s_1_2
        let s_1_3: bool = ((s_1_1) == (s_1_2));
        // N s_1_4: branch s_1_3 b37 b2
        if s_1_3 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var datasizeshadow#1844:i
        let s_2_0: i128 = fn_state.datasizeshadow_1844;
        // D s_2_1: call __id(s_2_0)
        let s_2_1: i128 = u__id(state, tracer, s_2_0);
        // C s_2_2: const #16s : i
        let s_2_2: i128 = 16;
        // D s_2_3: cmp-eq s_2_1 s_2_2
        let s_2_3: bool = ((s_2_1) == (s_2_2));
        // D s_2_4: write-var gs#168880 <= s_2_3
        fn_state.gs_168880 = s_2_3;
        // N s_2_5: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#168880:u8
        let s_3_0: bool = fn_state.gs_168880;
        // N s_3_1: branch s_3_0 b36 b4
        if s_3_0 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var datasizeshadow#1844:i
        let s_4_0: i128 = fn_state.datasizeshadow_1844;
        // D s_4_1: call __id(s_4_0)
        let s_4_1: i128 = u__id(state, tracer, s_4_0);
        // C s_4_2: const #32s : i
        let s_4_2: i128 = 32;
        // D s_4_3: cmp-eq s_4_1 s_4_2
        let s_4_3: bool = ((s_4_1) == (s_4_2));
        // D s_4_4: write-var gs#168882 <= s_4_3
        fn_state.gs_168882 = s_4_3;
        // N s_4_5: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#168882:u8
        let s_5_0: bool = fn_state.gs_168882;
        // N s_5_1: branch s_5_0 b35 b6
        if s_5_0 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var datasizeshadow#1844:i
        let s_6_0: i128 = fn_state.datasizeshadow_1844;
        // D s_6_1: call __id(s_6_0)
        let s_6_1: i128 = u__id(state, tracer, s_6_0);
        // C s_6_2: const #64s : i
        let s_6_2: i128 = 64;
        // D s_6_3: cmp-eq s_6_1 s_6_2
        let s_6_3: bool = ((s_6_1) == (s_6_2));
        // D s_6_4: write-var gs#168884 <= s_6_3
        fn_state.gs_168884 = s_6_3;
        // N s_6_5: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#168884:u8
        let s_7_0: bool = fn_state.gs_168884;
        // N s_7_1: branch s_7_0 b34 b8
        if s_7_0 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var datasizeshadow#1844:i
        let s_8_0: i128 = fn_state.datasizeshadow_1844;
        // D s_8_1: call __id(s_8_0)
        let s_8_1: i128 = u__id(state, tracer, s_8_0);
        // C s_8_2: const #128s : i
        let s_8_2: i128 = 128;
        // D s_8_3: cmp-eq s_8_1 s_8_2
        let s_8_3: bool = ((s_8_1) == (s_8_2));
        // D s_8_4: write-var gs#168886 <= s_8_3
        fn_state.gs_168886 = s_8_3;
        // N s_8_5: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#168886:u8
        let s_9_0: bool = fn_state.gs_168886;
        // N s_9_1: assert s_9_0
        let s_9_1: () = assert!(s_9_0);
        // D s_9_2: read-var datasizeshadow#1844:i
        let s_9_2: i128 = fn_state.datasizeshadow_1844;
        // D s_9_3: cast reint s_9_2 -> i64
        let s_9_3: i64 = (s_9_2 as i64);
        // D s_9_4: read-var n:i64
        let s_9_4: i64 = fn_state.n;
        // D s_9_5: cast zx s_9_4 -> i
        let s_9_5: i128 = (i128::try_from(s_9_4).unwrap());
        // D s_9_6: call V_read(s_9_5, s_9_3)
        let s_9_6: Bits = V_read(state, tracer, s_9_5, s_9_3);
        // D s_9_7: write-var operand <= s_9_6
        fn_state.operand = s_9_6;
        // C s_9_8: const #0s : i64
        let s_9_8: i64 = 0;
        // C s_9_9: const #1s : i
        let s_9_9: i128 = 1;
        // D s_9_10: read-var elements:i
        let s_9_10: i128 = fn_state.elements;
        // D s_9_11: sub s_9_10 s_9_9
        let s_9_11: i128 = ((s_9_10) - (s_9_9));
        // D s_9_12: cast reint s_9_11 -> i64
        let s_9_12: i64 = (s_9_11 as i64);
        // D s_9_13: write-var gs#168892 <= s_9_12
        fn_state.gs_168892 = s_9_12;
        // D s_9_14: write-var e <= s_9_8
        fn_state.e = s_9_8;
        // N s_9_15: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var e:i64
        let s_10_0: i64 = fn_state.e;
        // D s_10_1: read-var gs#168892:i64
        let s_10_1: i64 = fn_state.gs_168892;
        // D s_10_2: cmp-gt s_10_0 s_10_1
        let s_10_2: bool = ((s_10_0) > (s_10_1));
        // N s_10_3: branch s_10_2 b33 b11
        if s_10_2 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var esizeshadow#1843:i
        let s_11_0: i128 = fn_state.esizeshadow_1843;
        // D s_11_1: call __id(s_11_0)
        let s_11_1: i128 = u__id(state, tracer, s_11_0);
        // C s_11_2: const #0s : i
        let s_11_2: i128 = 0;
        // D s_11_3: cmp-ge s_11_1 s_11_2
        let s_11_3: bool = ((s_11_1) >= (s_11_2));
        // N s_11_4: assert s_11_3
        let s_11_4: () = assert!(s_11_3);
        // D s_11_5: read-var e:i64
        let s_11_5: i64 = fn_state.e;
        // D s_11_6: cast zx s_11_5 -> i
        let s_11_6: i128 = (i128::try_from(s_11_5).unwrap());
        // D s_11_7: call __id(s_11_6)
        let s_11_7: i128 = u__id(state, tracer, s_11_6);
        // D s_11_8: read-var esizeshadow#1843:i
        let s_11_8: i128 = fn_state.esizeshadow_1843;
        // D s_11_9: call __id(s_11_8)
        let s_11_9: i128 = u__id(state, tracer, s_11_8);
        // D s_11_10: mul s_11_7 s_11_9
        let s_11_10: i128 = ((s_11_7) * (s_11_9));
        // D s_11_11: read-var e:i64
        let s_11_11: i64 = fn_state.e;
        // D s_11_12: cast zx s_11_11 -> i
        let s_11_12: i128 = (i128::try_from(s_11_11).unwrap());
        // D s_11_13: call __id(s_11_12)
        let s_11_13: i128 = u__id(state, tracer, s_11_12);
        // D s_11_14: read-var esizeshadow#1843:i
        let s_11_14: i128 = fn_state.esizeshadow_1843;
        // D s_11_15: call __id(s_11_14)
        let s_11_15: i128 = u__id(state, tracer, s_11_14);
        // D s_11_16: mul s_11_13 s_11_15
        let s_11_16: i128 = ((s_11_13) * (s_11_15));
        // D s_11_17: read-var esizeshadow#1843:i
        let s_11_17: i128 = fn_state.esizeshadow_1843;
        // D s_11_18: call __id(s_11_17)
        let s_11_18: i128 = u__id(state, tracer, s_11_17);
        // D s_11_19: add s_11_16 s_11_18
        let s_11_19: i128 = (s_11_16 + s_11_18);
        // C s_11_20: const #1s : i
        let s_11_20: i128 = 1;
        // D s_11_21: sub s_11_19 s_11_20
        let s_11_21: i128 = ((s_11_19) - (s_11_20));
        // D s_11_22: cmp-le s_11_10 s_11_21
        let s_11_22: bool = ((s_11_10) <= (s_11_21));
        // N s_11_23: branch s_11_22 b32 b12
        if s_11_22 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var e:i64
        let s_12_0: i64 = fn_state.e;
        // D s_12_1: cast zx s_12_0 -> i
        let s_12_1: i128 = (i128::try_from(s_12_0).unwrap());
        // D s_12_2: call __id(s_12_1)
        let s_12_2: i128 = u__id(state, tracer, s_12_1);
        // C s_12_3: const #0s : i
        let s_12_3: i128 = 0;
        // D s_12_4: cmp-ge s_12_2 s_12_3
        let s_12_4: bool = ((s_12_2) >= (s_12_3));
        // N s_12_5: branch s_12_4 b31 b13
        if s_12_4 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #0u : u8
        let s_13_0: bool = false;
        // D s_13_1: write-var gs#168899 <= s_13_0
        fn_state.gs_168899 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#168899:u8
        let s_14_0: bool = fn_state.gs_168899;
        // D s_14_1: not s_14_0
        let s_14_1: bool = !s_14_0;
        // N s_14_2: branch s_14_1 b30 b15
        if s_14_1 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_15_0: read-var datasizeshadow#1844:i
        let s_15_0: i128 = fn_state.datasizeshadow_1844;
        // D s_15_1: call __id(s_15_0)
        let s_15_1: i128 = u__id(state, tracer, s_15_0);
        // D s_15_2: cast reint s_15_1 -> i64
        let s_15_2: i64 = (s_15_1 as i64);
        // C s_15_3: const #8s : i
        let s_15_3: i128 = 8;
        // D s_15_4: cast zx s_15_2 -> i
        let s_15_4: i128 = (i128::try_from(s_15_2).unwrap());
        // D s_15_5: cmp-eq s_15_4 s_15_3
        let s_15_5: bool = ((s_15_4) == (s_15_3));
        // N s_15_6: branch s_15_5 b29 b16
        if s_15_5 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var datasizeshadow#1844:i
        let s_16_0: i128 = fn_state.datasizeshadow_1844;
        // D s_16_1: call __id(s_16_0)
        let s_16_1: i128 = u__id(state, tracer, s_16_0);
        // D s_16_2: cast reint s_16_1 -> i64
        let s_16_2: i64 = (s_16_1 as i64);
        // C s_16_3: const #16s : i
        let s_16_3: i128 = 16;
        // D s_16_4: cast zx s_16_2 -> i
        let s_16_4: i128 = (i128::try_from(s_16_2).unwrap());
        // D s_16_5: cmp-eq s_16_4 s_16_3
        let s_16_5: bool = ((s_16_4) == (s_16_3));
        // D s_16_6: write-var gs#168902 <= s_16_5
        fn_state.gs_168902 = s_16_5;
        // N s_16_7: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var gs#168902:u8
        let s_17_0: bool = fn_state.gs_168902;
        // N s_17_1: branch s_17_0 b28 b18
        if s_17_0 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var datasizeshadow#1844:i
        let s_18_0: i128 = fn_state.datasizeshadow_1844;
        // D s_18_1: call __id(s_18_0)
        let s_18_1: i128 = u__id(state, tracer, s_18_0);
        // D s_18_2: cast reint s_18_1 -> i64
        let s_18_2: i64 = (s_18_1 as i64);
        // C s_18_3: const #32s : i
        let s_18_3: i128 = 32;
        // D s_18_4: cast zx s_18_2 -> i
        let s_18_4: i128 = (i128::try_from(s_18_2).unwrap());
        // D s_18_5: cmp-eq s_18_4 s_18_3
        let s_18_5: bool = ((s_18_4) == (s_18_3));
        // D s_18_6: write-var gs#168904 <= s_18_5
        fn_state.gs_168904 = s_18_5;
        // N s_18_7: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#168904:u8
        let s_19_0: bool = fn_state.gs_168904;
        // N s_19_1: branch s_19_0 b27 b20
        if s_19_0 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var datasizeshadow#1844:i
        let s_20_0: i128 = fn_state.datasizeshadow_1844;
        // D s_20_1: call __id(s_20_0)
        let s_20_1: i128 = u__id(state, tracer, s_20_0);
        // D s_20_2: cast reint s_20_1 -> i64
        let s_20_2: i64 = (s_20_1 as i64);
        // C s_20_3: const #64s : i
        let s_20_3: i128 = 64;
        // D s_20_4: cast zx s_20_2 -> i
        let s_20_4: i128 = (i128::try_from(s_20_2).unwrap());
        // D s_20_5: cmp-eq s_20_4 s_20_3
        let s_20_5: bool = ((s_20_4) == (s_20_3));
        // D s_20_6: write-var gs#168906 <= s_20_5
        fn_state.gs_168906 = s_20_5;
        // N s_20_7: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#168906:u8
        let s_21_0: bool = fn_state.gs_168906;
        // N s_21_1: branch s_21_0 b26 b22
        if s_21_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var datasizeshadow#1844:i
        let s_22_0: i128 = fn_state.datasizeshadow_1844;
        // D s_22_1: call __id(s_22_0)
        let s_22_1: i128 = u__id(state, tracer, s_22_0);
        // D s_22_2: cast reint s_22_1 -> i64
        let s_22_2: i64 = (s_22_1 as i64);
        // C s_22_3: const #128s : i
        let s_22_3: i128 = 128;
        // D s_22_4: cast zx s_22_2 -> i
        let s_22_4: i128 = (i128::try_from(s_22_2).unwrap());
        // D s_22_5: cmp-eq s_22_4 s_22_3
        let s_22_5: bool = ((s_22_4) == (s_22_3));
        // D s_22_6: write-var gs#168908 <= s_22_5
        fn_state.gs_168908 = s_22_5;
        // N s_22_7: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#168908:u8
        let s_23_0: bool = fn_state.gs_168908;
        // D s_23_1: not s_23_0
        let s_23_1: bool = !s_23_0;
        // D s_23_2: write-var gs#168909 <= s_23_1
        fn_state.gs_168909 = s_23_1;
        // N s_23_3: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#168909:u8
        let s_24_0: bool = fn_state.gs_168909;
        // D s_24_1: write-var gs#168910 <= s_24_0
        fn_state.gs_168910 = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#168910:u8
        let s_25_0: bool = fn_state.gs_168910;
        // N s_25_1: assert s_25_0
        let s_25_1: () = assert!(s_25_0);
        // D s_25_2: read-var esizeshadow#1843:i
        let s_25_2: i128 = fn_state.esizeshadow_1843;
        // D s_25_3: read-var esizeshadow#1843:i
        let s_25_3: i128 = fn_state.esizeshadow_1843;
        // D s_25_4: read-var e:i64
        let s_25_4: i64 = fn_state.e;
        // D s_25_5: cast zx s_25_4 -> i
        let s_25_5: i128 = (i128::try_from(s_25_4).unwrap());
        // D s_25_6: read-var operand:bv
        let s_25_6: Bits = fn_state.operand;
        // D s_25_7: call Elem_read(s_25_6, s_25_5, s_25_3)
        let s_25_7: Bits = Elem_read(state, tracer, s_25_6, s_25_5, s_25_3);
        // D s_25_8: read-var shift:i
        let s_25_8: i128 = fn_state.shift;
        // D s_25_9: lsl s_25_7 s_25_8
        let s_25_9: Bits = s_25_7 << s_25_8;
        // D s_25_10: read-var e:i64
        let s_25_10: i64 = fn_state.e;
        // D s_25_11: cast zx s_25_10 -> i
        let s_25_11: i128 = (i128::try_from(s_25_10).unwrap());
        // D s_25_12: read-var result:bv
        let s_25_12: Bits = fn_state.result;
        // D s_25_13: call Elem_set(s_25_12, s_25_11, s_25_2, s_25_9)
        let s_25_13: Bits = Elem_set(state, tracer, s_25_12, s_25_11, s_25_2, s_25_9);
        // D s_25_14: write-var result <= s_25_13
        fn_state.result = s_25_13;
        // D s_25_15: read-var e:i64
        let s_25_15: i64 = fn_state.e;
        // C s_25_16: const #1s : i64
        let s_25_16: i64 = 1;
        // D s_25_17: add s_25_15 s_25_16
        let s_25_17: i64 = (s_25_15 + s_25_16);
        // D s_25_18: write-var e <= s_25_17
        fn_state.e = s_25_17;
        // N s_25_19: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_26_0: const #1u : u8
        let s_26_0: bool = true;
        // D s_26_1: write-var gs#168908 <= s_26_0
        fn_state.gs_168908 = s_26_0;
        // N s_26_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_27_0: const #1u : u8
        let s_27_0: bool = true;
        // D s_27_1: write-var gs#168906 <= s_27_0
        fn_state.gs_168906 = s_27_0;
        // N s_27_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #1u : u8
        let s_28_0: bool = true;
        // D s_28_1: write-var gs#168904 <= s_28_0
        fn_state.gs_168904 = s_28_0;
        // N s_28_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #1u : u8
        let s_29_0: bool = true;
        // D s_29_1: write-var gs#168902 <= s_29_0
        fn_state.gs_168902 = s_29_0;
        // N s_29_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #1u : u8
        let s_30_0: bool = true;
        // D s_30_1: write-var gs#168909 <= s_30_0
        fn_state.gs_168909 = s_30_0;
        // N s_30_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_31_0: read-var e:i64
        let s_31_0: i64 = fn_state.e;
        // D s_31_1: cast zx s_31_0 -> i
        let s_31_1: i128 = (i128::try_from(s_31_0).unwrap());
        // D s_31_2: call __id(s_31_1)
        let s_31_2: i128 = u__id(state, tracer, s_31_1);
        // C s_31_3: const #1s : i
        let s_31_3: i128 = 1;
        // D s_31_4: add s_31_2 s_31_3
        let s_31_4: i128 = (s_31_2 + s_31_3);
        // D s_31_5: read-var esizeshadow#1843:i
        let s_31_5: i128 = fn_state.esizeshadow_1843;
        // D s_31_6: call __id(s_31_5)
        let s_31_6: i128 = u__id(state, tracer, s_31_5);
        // D s_31_7: mul s_31_4 s_31_6
        let s_31_7: i128 = ((s_31_4) * (s_31_6));
        // D s_31_8: read-var datasizeshadow#1844:i
        let s_31_8: i128 = fn_state.datasizeshadow_1844;
        // D s_31_9: call __id(s_31_8)
        let s_31_9: i128 = u__id(state, tracer, s_31_8);
        // D s_31_10: cast reint s_31_9 -> i64
        let s_31_10: i64 = (s_31_9 as i64);
        // D s_31_11: cast zx s_31_10 -> i
        let s_31_11: i128 = (i128::try_from(s_31_10).unwrap());
        // D s_31_12: cmp-le s_31_7 s_31_11
        let s_31_12: bool = ((s_31_7) <= (s_31_11));
        // D s_31_13: write-var gs#168899 <= s_31_12
        fn_state.gs_168899 = s_31_12;
        // N s_31_14: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #1u : u8
        let s_32_0: bool = true;
        // D s_32_1: write-var gs#168910 <= s_32_0
        fn_state.gs_168910 = s_32_0;
        // N s_32_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var datasizeshadow#1844:i
        let s_33_0: i128 = fn_state.datasizeshadow_1844;
        // D s_33_1: cast reint s_33_0 -> i64
        let s_33_1: i64 = (s_33_0 as i64);
        // D s_33_2: read-var d:i64
        let s_33_2: i64 = fn_state.d;
        // D s_33_3: cast zx s_33_2 -> i
        let s_33_3: i128 = (i128::try_from(s_33_2).unwrap());
        // D s_33_4: read-var result:bv
        let s_33_4: Bits = fn_state.result;
        // D s_33_5: call V_set(s_33_3, s_33_1, s_33_4)
        let s_33_5: () = V_set(state, tracer, s_33_3, s_33_1, s_33_4);
        // N s_33_6: return
        return;
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #1u : u8
        let s_34_0: bool = true;
        // D s_34_1: write-var gs#168886 <= s_34_0
        fn_state.gs_168886 = s_34_0;
        // N s_34_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #1u : u8
        let s_35_0: bool = true;
        // D s_35_1: write-var gs#168884 <= s_35_0
        fn_state.gs_168884 = s_35_0;
        // N s_35_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_36_0: const #1u : u8
        let s_36_0: bool = true;
        // D s_36_1: write-var gs#168882 <= s_36_0
        fn_state.gs_168882 = s_36_0;
        // N s_36_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #1u : u8
        let s_37_0: bool = true;
        // D s_37_1: write-var gs#168880 <= s_37_0
        fn_state.gs_168880 = s_37_0;
        // N s_37_2: jump b3
        return block_3(state, tracer, fn_state);
    }
}
