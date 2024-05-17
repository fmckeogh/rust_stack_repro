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
use V_set::*;
use integer_subrange::*;
use asl_Int::*;
use Zeros::*;
use Elem_read::*;
use RShr::*;
use CheckFPAdvSIMDEnabled64::*;
use common::*;
pub fn execute_aarch64_instrs_vector_shift_right_sisd<T: Tracer>(
    state: &mut State,
    tracer: &T,
    accumulate: bool,
    d: i64,
    datasize: i128,
    elements: i128,
    esize: i128,
    n: i64,
    round: bool,
    shift: i128,
    is_unsigned: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        operand: Bits,
        gs_172643: bool,
        e: i64,
        gs_172659: bool,
        gs_172665: bool,
        esizeshadow_1979: i128,
        gs_172663: bool,
        gs_172641: bool,
        datasizeshadow_1980: i128,
        gs_172661: bool,
        gs_172656: bool,
        gs_172649: i64,
        gs_172637: bool,
        gs_172667: bool,
        gs_172639: bool,
        result: Bits,
        gs_172666: bool,
        operand2: Bits,
        accumulate: bool,
        d: i64,
        datasize: i128,
        elements: i128,
        esize: i128,
        n: i64,
        round: bool,
        shift: i128,
        is_unsigned: bool,
    }
    let fn_state = FunctionState {
        accumulate,
        d,
        datasize,
        elements,
        esize,
        n,
        round,
        shift,
        is_unsigned,
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
        // D s_0_1: write-var esizeshadow#1979 <= s_0_0
        fn_state.esizeshadow_1979 = s_0_0;
        // D s_0_2: read-var datasize:i
        let s_0_2: i128 = fn_state.datasize;
        // D s_0_3: write-var datasizeshadow#1980 <= s_0_2
        fn_state.datasizeshadow_1980 = s_0_2;
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
        // D s_1_0: read-var datasizeshadow#1980:i
        let s_1_0: i128 = fn_state.datasizeshadow_1980;
        // D s_1_1: call __id(s_1_0)
        let s_1_1: i128 = u__id(state, tracer, s_1_0);
        // C s_1_2: const #8s : i
        let s_1_2: i128 = 8;
        // D s_1_3: cmp-eq s_1_1 s_1_2
        let s_1_3: bool = ((s_1_1) == (s_1_2));
        // N s_1_4: branch s_1_3 b40 b2
        if s_1_3 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var datasizeshadow#1980:i
        let s_2_0: i128 = fn_state.datasizeshadow_1980;
        // D s_2_1: call __id(s_2_0)
        let s_2_1: i128 = u__id(state, tracer, s_2_0);
        // C s_2_2: const #16s : i
        let s_2_2: i128 = 16;
        // D s_2_3: cmp-eq s_2_1 s_2_2
        let s_2_3: bool = ((s_2_1) == (s_2_2));
        // D s_2_4: write-var gs#172637 <= s_2_3
        fn_state.gs_172637 = s_2_3;
        // N s_2_5: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#172637:u8
        let s_3_0: bool = fn_state.gs_172637;
        // N s_3_1: branch s_3_0 b39 b4
        if s_3_0 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var datasizeshadow#1980:i
        let s_4_0: i128 = fn_state.datasizeshadow_1980;
        // D s_4_1: call __id(s_4_0)
        let s_4_1: i128 = u__id(state, tracer, s_4_0);
        // C s_4_2: const #32s : i
        let s_4_2: i128 = 32;
        // D s_4_3: cmp-eq s_4_1 s_4_2
        let s_4_3: bool = ((s_4_1) == (s_4_2));
        // D s_4_4: write-var gs#172639 <= s_4_3
        fn_state.gs_172639 = s_4_3;
        // N s_4_5: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#172639:u8
        let s_5_0: bool = fn_state.gs_172639;
        // N s_5_1: branch s_5_0 b38 b6
        if s_5_0 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var datasizeshadow#1980:i
        let s_6_0: i128 = fn_state.datasizeshadow_1980;
        // D s_6_1: call __id(s_6_0)
        let s_6_1: i128 = u__id(state, tracer, s_6_0);
        // C s_6_2: const #64s : i
        let s_6_2: i128 = 64;
        // D s_6_3: cmp-eq s_6_1 s_6_2
        let s_6_3: bool = ((s_6_1) == (s_6_2));
        // D s_6_4: write-var gs#172641 <= s_6_3
        fn_state.gs_172641 = s_6_3;
        // N s_6_5: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#172641:u8
        let s_7_0: bool = fn_state.gs_172641;
        // N s_7_1: branch s_7_0 b37 b8
        if s_7_0 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var datasizeshadow#1980:i
        let s_8_0: i128 = fn_state.datasizeshadow_1980;
        // D s_8_1: call __id(s_8_0)
        let s_8_1: i128 = u__id(state, tracer, s_8_0);
        // C s_8_2: const #128s : i
        let s_8_2: i128 = 128;
        // D s_8_3: cmp-eq s_8_1 s_8_2
        let s_8_3: bool = ((s_8_1) == (s_8_2));
        // D s_8_4: write-var gs#172643 <= s_8_3
        fn_state.gs_172643 = s_8_3;
        // N s_8_5: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#172643:u8
        let s_9_0: bool = fn_state.gs_172643;
        // N s_9_1: assert s_9_0
        let s_9_1: () = assert!(s_9_0);
        // D s_9_2: read-var datasizeshadow#1980:i
        let s_9_2: i128 = fn_state.datasizeshadow_1980;
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
        // D s_9_8: read-var accumulate:u8
        let s_9_8: bool = fn_state.accumulate;
        // N s_9_9: branch s_9_8 b36 b10
        if s_9_8 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var datasizeshadow#1980:i
        let s_10_0: i128 = fn_state.datasizeshadow_1980;
        // D s_10_1: call Zeros(s_10_0)
        let s_10_1: Bits = Zeros(state, tracer, s_10_0);
        // D s_10_2: write-var operand2 <= s_10_1
        fn_state.operand2 = s_10_1;
        // N s_10_3: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #0s : i64
        let s_11_0: i64 = 0;
        // C s_11_1: const #1s : i
        let s_11_1: i128 = 1;
        // D s_11_2: read-var elements:i
        let s_11_2: i128 = fn_state.elements;
        // D s_11_3: sub s_11_2 s_11_1
        let s_11_3: i128 = ((s_11_2) - (s_11_1));
        // D s_11_4: cast reint s_11_3 -> i64
        let s_11_4: i64 = (s_11_3 as i64);
        // D s_11_5: write-var gs#172649 <= s_11_4
        fn_state.gs_172649 = s_11_4;
        // D s_11_6: write-var e <= s_11_0
        fn_state.e = s_11_0;
        // N s_11_7: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_12_0: read-var e:i64
        let s_12_0: i64 = fn_state.e;
        // D s_12_1: read-var gs#172649:i64
        let s_12_1: i64 = fn_state.gs_172649;
        // D s_12_2: cmp-gt s_12_0 s_12_1
        let s_12_2: bool = ((s_12_0) > (s_12_1));
        // N s_12_3: branch s_12_2 b35 b13
        if s_12_2 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_13_0: read-var esizeshadow#1979:i
        let s_13_0: i128 = fn_state.esizeshadow_1979;
        // D s_13_1: call __id(s_13_0)
        let s_13_1: i128 = u__id(state, tracer, s_13_0);
        // C s_13_2: const #0s : i
        let s_13_2: i128 = 0;
        // D s_13_3: cmp-ge s_13_1 s_13_2
        let s_13_3: bool = ((s_13_1) >= (s_13_2));
        // N s_13_4: assert s_13_3
        let s_13_4: () = assert!(s_13_3);
        // D s_13_5: read-var e:i64
        let s_13_5: i64 = fn_state.e;
        // D s_13_6: cast zx s_13_5 -> i
        let s_13_6: i128 = (i128::try_from(s_13_5).unwrap());
        // D s_13_7: call __id(s_13_6)
        let s_13_7: i128 = u__id(state, tracer, s_13_6);
        // D s_13_8: read-var esizeshadow#1979:i
        let s_13_8: i128 = fn_state.esizeshadow_1979;
        // D s_13_9: call __id(s_13_8)
        let s_13_9: i128 = u__id(state, tracer, s_13_8);
        // D s_13_10: mul s_13_7 s_13_9
        let s_13_10: i128 = ((s_13_7) * (s_13_9));
        // D s_13_11: read-var e:i64
        let s_13_11: i64 = fn_state.e;
        // D s_13_12: cast zx s_13_11 -> i
        let s_13_12: i128 = (i128::try_from(s_13_11).unwrap());
        // D s_13_13: call __id(s_13_12)
        let s_13_13: i128 = u__id(state, tracer, s_13_12);
        // D s_13_14: read-var esizeshadow#1979:i
        let s_13_14: i128 = fn_state.esizeshadow_1979;
        // D s_13_15: call __id(s_13_14)
        let s_13_15: i128 = u__id(state, tracer, s_13_14);
        // D s_13_16: mul s_13_13 s_13_15
        let s_13_16: i128 = ((s_13_13) * (s_13_15));
        // D s_13_17: read-var esizeshadow#1979:i
        let s_13_17: i128 = fn_state.esizeshadow_1979;
        // D s_13_18: call __id(s_13_17)
        let s_13_18: i128 = u__id(state, tracer, s_13_17);
        // D s_13_19: add s_13_16 s_13_18
        let s_13_19: i128 = (s_13_16 + s_13_18);
        // C s_13_20: const #1s : i
        let s_13_20: i128 = 1;
        // D s_13_21: sub s_13_19 s_13_20
        let s_13_21: i128 = ((s_13_19) - (s_13_20));
        // D s_13_22: cmp-le s_13_10 s_13_21
        let s_13_22: bool = ((s_13_10) <= (s_13_21));
        // N s_13_23: branch s_13_22 b34 b14
        if s_13_22 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var e:i64
        let s_14_0: i64 = fn_state.e;
        // D s_14_1: cast zx s_14_0 -> i
        let s_14_1: i128 = (i128::try_from(s_14_0).unwrap());
        // D s_14_2: call __id(s_14_1)
        let s_14_2: i128 = u__id(state, tracer, s_14_1);
        // C s_14_3: const #0s : i
        let s_14_3: i128 = 0;
        // D s_14_4: cmp-ge s_14_2 s_14_3
        let s_14_4: bool = ((s_14_2) >= (s_14_3));
        // N s_14_5: branch s_14_4 b33 b15
        if s_14_4 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // D s_15_1: write-var gs#172656 <= s_15_0
        fn_state.gs_172656 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var gs#172656:u8
        let s_16_0: bool = fn_state.gs_172656;
        // D s_16_1: not s_16_0
        let s_16_1: bool = !s_16_0;
        // N s_16_2: branch s_16_1 b32 b17
        if s_16_1 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var datasizeshadow#1980:i
        let s_17_0: i128 = fn_state.datasizeshadow_1980;
        // D s_17_1: call __id(s_17_0)
        let s_17_1: i128 = u__id(state, tracer, s_17_0);
        // D s_17_2: cast reint s_17_1 -> i64
        let s_17_2: i64 = (s_17_1 as i64);
        // C s_17_3: const #8s : i
        let s_17_3: i128 = 8;
        // D s_17_4: cast zx s_17_2 -> i
        let s_17_4: i128 = (i128::try_from(s_17_2).unwrap());
        // D s_17_5: cmp-eq s_17_4 s_17_3
        let s_17_5: bool = ((s_17_4) == (s_17_3));
        // N s_17_6: branch s_17_5 b31 b18
        if s_17_5 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var datasizeshadow#1980:i
        let s_18_0: i128 = fn_state.datasizeshadow_1980;
        // D s_18_1: call __id(s_18_0)
        let s_18_1: i128 = u__id(state, tracer, s_18_0);
        // D s_18_2: cast reint s_18_1 -> i64
        let s_18_2: i64 = (s_18_1 as i64);
        // C s_18_3: const #16s : i
        let s_18_3: i128 = 16;
        // D s_18_4: cast zx s_18_2 -> i
        let s_18_4: i128 = (i128::try_from(s_18_2).unwrap());
        // D s_18_5: cmp-eq s_18_4 s_18_3
        let s_18_5: bool = ((s_18_4) == (s_18_3));
        // D s_18_6: write-var gs#172659 <= s_18_5
        fn_state.gs_172659 = s_18_5;
        // N s_18_7: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var gs#172659:u8
        let s_19_0: bool = fn_state.gs_172659;
        // N s_19_1: branch s_19_0 b30 b20
        if s_19_0 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var datasizeshadow#1980:i
        let s_20_0: i128 = fn_state.datasizeshadow_1980;
        // D s_20_1: call __id(s_20_0)
        let s_20_1: i128 = u__id(state, tracer, s_20_0);
        // D s_20_2: cast reint s_20_1 -> i64
        let s_20_2: i64 = (s_20_1 as i64);
        // C s_20_3: const #32s : i
        let s_20_3: i128 = 32;
        // D s_20_4: cast zx s_20_2 -> i
        let s_20_4: i128 = (i128::try_from(s_20_2).unwrap());
        // D s_20_5: cmp-eq s_20_4 s_20_3
        let s_20_5: bool = ((s_20_4) == (s_20_3));
        // D s_20_6: write-var gs#172661 <= s_20_5
        fn_state.gs_172661 = s_20_5;
        // N s_20_7: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var gs#172661:u8
        let s_21_0: bool = fn_state.gs_172661;
        // N s_21_1: branch s_21_0 b29 b22
        if s_21_0 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var datasizeshadow#1980:i
        let s_22_0: i128 = fn_state.datasizeshadow_1980;
        // D s_22_1: call __id(s_22_0)
        let s_22_1: i128 = u__id(state, tracer, s_22_0);
        // D s_22_2: cast reint s_22_1 -> i64
        let s_22_2: i64 = (s_22_1 as i64);
        // C s_22_3: const #64s : i
        let s_22_3: i128 = 64;
        // D s_22_4: cast zx s_22_2 -> i
        let s_22_4: i128 = (i128::try_from(s_22_2).unwrap());
        // D s_22_5: cmp-eq s_22_4 s_22_3
        let s_22_5: bool = ((s_22_4) == (s_22_3));
        // D s_22_6: write-var gs#172663 <= s_22_5
        fn_state.gs_172663 = s_22_5;
        // N s_22_7: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var gs#172663:u8
        let s_23_0: bool = fn_state.gs_172663;
        // N s_23_1: branch s_23_0 b28 b24
        if s_23_0 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var datasizeshadow#1980:i
        let s_24_0: i128 = fn_state.datasizeshadow_1980;
        // D s_24_1: call __id(s_24_0)
        let s_24_1: i128 = u__id(state, tracer, s_24_0);
        // D s_24_2: cast reint s_24_1 -> i64
        let s_24_2: i64 = (s_24_1 as i64);
        // C s_24_3: const #128s : i
        let s_24_3: i128 = 128;
        // D s_24_4: cast zx s_24_2 -> i
        let s_24_4: i128 = (i128::try_from(s_24_2).unwrap());
        // D s_24_5: cmp-eq s_24_4 s_24_3
        let s_24_5: bool = ((s_24_4) == (s_24_3));
        // D s_24_6: write-var gs#172665 <= s_24_5
        fn_state.gs_172665 = s_24_5;
        // N s_24_7: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#172665:u8
        let s_25_0: bool = fn_state.gs_172665;
        // D s_25_1: not s_25_0
        let s_25_1: bool = !s_25_0;
        // D s_25_2: write-var gs#172666 <= s_25_1
        fn_state.gs_172666 = s_25_1;
        // N s_25_3: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#172666:u8
        let s_26_0: bool = fn_state.gs_172666;
        // D s_26_1: write-var gs#172667 <= s_26_0
        fn_state.gs_172667 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#172667:u8
        let s_27_0: bool = fn_state.gs_172667;
        // N s_27_1: assert s_27_0
        let s_27_1: () = assert!(s_27_0);
        // D s_27_2: read-var esizeshadow#1979:i
        let s_27_2: i128 = fn_state.esizeshadow_1979;
        // D s_27_3: read-var e:i64
        let s_27_3: i64 = fn_state.e;
        // D s_27_4: cast zx s_27_3 -> i
        let s_27_4: i128 = (i128::try_from(s_27_3).unwrap());
        // D s_27_5: read-var operand:bv
        let s_27_5: Bits = fn_state.operand;
        // D s_27_6: call Elem_read(s_27_5, s_27_4, s_27_2)
        let s_27_6: Bits = Elem_read(state, tracer, s_27_5, s_27_4, s_27_2);
        // D s_27_7: read-var is_unsigned:u8
        let s_27_7: bool = fn_state.is_unsigned;
        // D s_27_8: call asl_Int(s_27_6, s_27_7)
        let s_27_8: i128 = asl_Int(state, tracer, s_27_6, s_27_7);
        // D s_27_9: read-var shift:i
        let s_27_9: i128 = fn_state.shift;
        // D s_27_10: read-var round:u8
        let s_27_10: bool = fn_state.round;
        // D s_27_11: call RShr(s_27_8, s_27_9, s_27_10)
        let s_27_11: i128 = RShr(state, tracer, s_27_8, s_27_9, s_27_10);
        // D s_27_12: read-var esizeshadow#1979:i
        let s_27_12: i128 = fn_state.esizeshadow_1979;
        // D s_27_13: read-var esizeshadow#1979:i
        let s_27_13: i128 = fn_state.esizeshadow_1979;
        // D s_27_14: read-var e:i64
        let s_27_14: i64 = fn_state.e;
        // D s_27_15: cast zx s_27_14 -> i
        let s_27_15: i128 = (i128::try_from(s_27_14).unwrap());
        // D s_27_16: read-var operand2:bv
        let s_27_16: Bits = fn_state.operand2;
        // D s_27_17: call Elem_read(s_27_16, s_27_15, s_27_13)
        let s_27_17: Bits = Elem_read(state, tracer, s_27_16, s_27_15, s_27_13);
        // C s_27_18: const #1s : i
        let s_27_18: i128 = 1;
        // D s_27_19: read-var esizeshadow#1979:i
        let s_27_19: i128 = fn_state.esizeshadow_1979;
        // D s_27_20: sub s_27_19 s_27_18
        let s_27_20: i128 = ((s_27_19) - (s_27_18));
        // C s_27_21: const #0s : i
        let s_27_21: i128 = 0;
        // D s_27_22: call integer_subrange(s_27_11, s_27_20, s_27_21)
        let s_27_22: Bits = integer_subrange(state, tracer, s_27_11, s_27_20, s_27_21);
        // D s_27_23: add s_27_17 s_27_22
        let s_27_23: Bits = (s_27_17 + s_27_22);
        // D s_27_24: read-var e:i64
        let s_27_24: i64 = fn_state.e;
        // D s_27_25: cast zx s_27_24 -> i
        let s_27_25: i128 = (i128::try_from(s_27_24).unwrap());
        // D s_27_26: read-var result:bv
        let s_27_26: Bits = fn_state.result;
        // D s_27_27: call Elem_set(s_27_26, s_27_25, s_27_12, s_27_23)
        let s_27_27: Bits = Elem_set(state, tracer, s_27_26, s_27_25, s_27_12, s_27_23);
        // D s_27_28: write-var result <= s_27_27
        fn_state.result = s_27_27;
        // D s_27_29: read-var e:i64
        let s_27_29: i64 = fn_state.e;
        // C s_27_30: const #1s : i64
        let s_27_30: i64 = 1;
        // D s_27_31: add s_27_29 s_27_30
        let s_27_31: i64 = (s_27_29 + s_27_30);
        // D s_27_32: write-var e <= s_27_31
        fn_state.e = s_27_31;
        // N s_27_33: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #1u : u8
        let s_28_0: bool = true;
        // D s_28_1: write-var gs#172665 <= s_28_0
        fn_state.gs_172665 = s_28_0;
        // N s_28_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #1u : u8
        let s_29_0: bool = true;
        // D s_29_1: write-var gs#172663 <= s_29_0
        fn_state.gs_172663 = s_29_0;
        // N s_29_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #1u : u8
        let s_30_0: bool = true;
        // D s_30_1: write-var gs#172661 <= s_30_0
        fn_state.gs_172661 = s_30_0;
        // N s_30_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #1u : u8
        let s_31_0: bool = true;
        // D s_31_1: write-var gs#172659 <= s_31_0
        fn_state.gs_172659 = s_31_0;
        // N s_31_2: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #1u : u8
        let s_32_0: bool = true;
        // D s_32_1: write-var gs#172666 <= s_32_0
        fn_state.gs_172666 = s_32_0;
        // N s_32_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_33_0: read-var e:i64
        let s_33_0: i64 = fn_state.e;
        // D s_33_1: cast zx s_33_0 -> i
        let s_33_1: i128 = (i128::try_from(s_33_0).unwrap());
        // D s_33_2: call __id(s_33_1)
        let s_33_2: i128 = u__id(state, tracer, s_33_1);
        // C s_33_3: const #1s : i
        let s_33_3: i128 = 1;
        // D s_33_4: add s_33_2 s_33_3
        let s_33_4: i128 = (s_33_2 + s_33_3);
        // D s_33_5: read-var esizeshadow#1979:i
        let s_33_5: i128 = fn_state.esizeshadow_1979;
        // D s_33_6: call __id(s_33_5)
        let s_33_6: i128 = u__id(state, tracer, s_33_5);
        // D s_33_7: mul s_33_4 s_33_6
        let s_33_7: i128 = ((s_33_4) * (s_33_6));
        // D s_33_8: read-var datasizeshadow#1980:i
        let s_33_8: i128 = fn_state.datasizeshadow_1980;
        // D s_33_9: call __id(s_33_8)
        let s_33_9: i128 = u__id(state, tracer, s_33_8);
        // D s_33_10: cast reint s_33_9 -> i64
        let s_33_10: i64 = (s_33_9 as i64);
        // D s_33_11: cast zx s_33_10 -> i
        let s_33_11: i128 = (i128::try_from(s_33_10).unwrap());
        // D s_33_12: cmp-le s_33_7 s_33_11
        let s_33_12: bool = ((s_33_7) <= (s_33_11));
        // D s_33_13: write-var gs#172656 <= s_33_12
        fn_state.gs_172656 = s_33_12;
        // N s_33_14: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_34_0: const #1u : u8
        let s_34_0: bool = true;
        // D s_34_1: write-var gs#172667 <= s_34_0
        fn_state.gs_172667 = s_34_0;
        // N s_34_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_35_0: read-var datasizeshadow#1980:i
        let s_35_0: i128 = fn_state.datasizeshadow_1980;
        // D s_35_1: cast reint s_35_0 -> i64
        let s_35_1: i64 = (s_35_0 as i64);
        // D s_35_2: read-var d:i64
        let s_35_2: i64 = fn_state.d;
        // D s_35_3: cast zx s_35_2 -> i
        let s_35_3: i128 = (i128::try_from(s_35_2).unwrap());
        // D s_35_4: read-var result:bv
        let s_35_4: Bits = fn_state.result;
        // D s_35_5: call V_set(s_35_3, s_35_1, s_35_4)
        let s_35_5: () = V_set(state, tracer, s_35_3, s_35_1, s_35_4);
        // N s_35_6: return
        return;
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var datasizeshadow#1980:i
        let s_36_0: i128 = fn_state.datasizeshadow_1980;
        // D s_36_1: cast reint s_36_0 -> i64
        let s_36_1: i64 = (s_36_0 as i64);
        // D s_36_2: read-var d:i64
        let s_36_2: i64 = fn_state.d;
        // D s_36_3: cast zx s_36_2 -> i
        let s_36_3: i128 = (i128::try_from(s_36_2).unwrap());
        // D s_36_4: call V_read(s_36_3, s_36_1)
        let s_36_4: Bits = V_read(state, tracer, s_36_3, s_36_1);
        // D s_36_5: write-var operand2 <= s_36_4
        fn_state.operand2 = s_36_4;
        // N s_36_6: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #1u : u8
        let s_37_0: bool = true;
        // D s_37_1: write-var gs#172643 <= s_37_0
        fn_state.gs_172643 = s_37_0;
        // N s_37_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_38_0: const #1u : u8
        let s_38_0: bool = true;
        // D s_38_1: write-var gs#172641 <= s_38_0
        fn_state.gs_172641 = s_38_0;
        // N s_38_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_39_0: const #1u : u8
        let s_39_0: bool = true;
        // D s_39_1: write-var gs#172639 <= s_39_0
        fn_state.gs_172639 = s_39_0;
        // N s_39_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_40_0: const #1u : u8
        let s_40_0: bool = true;
        // D s_40_1: write-var gs#172637 <= s_40_0
        fn_state.gs_172637 = s_40_0;
        // N s_40_2: jump b3
        return block_3(state, tracer, fn_state);
    }
}
