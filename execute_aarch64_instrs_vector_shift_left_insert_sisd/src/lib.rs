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
use V_set::*;
use Elem_set::*;
use u__id::*;
use V_read::*;
use Ones::*;
use Elem_read::*;
use CheckFPAdvSIMDEnabled64::*;
use common::*;
pub fn execute_aarch64_instrs_vector_shift_left_insert_sisd<T: Tracer>(
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
        gs_169058: bool,
        e: i64,
        gs_169027: bool,
        gs_169059: bool,
        gs_169031: bool,
        datasizeshadow_1849: i128,
        gs_169047: bool,
        gs_169053: bool,
        gs_169033: bool,
        gs_169055: bool,
        mask: Bits,
        gs_169029: bool,
        gs_169051: bool,
        gs_169041: i64,
        gs_169060: bool,
        esizeshadow_1848: i128,
        result: Bits,
        gs_169057: bool,
        operand2: Bits,
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
        // D s_0_1: write-var esizeshadow#1848 <= s_0_0
        fn_state.esizeshadow_1848 = s_0_0;
        // D s_0_2: read-var datasize:i
        let s_0_2: i128 = fn_state.datasize;
        // D s_0_3: write-var datasizeshadow#1849 <= s_0_2
        fn_state.datasizeshadow_1849 = s_0_2;
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
        // D s_1_0: read-var datasizeshadow#1849:i
        let s_1_0: i128 = fn_state.datasizeshadow_1849;
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
        // D s_2_0: read-var datasizeshadow#1849:i
        let s_2_0: i128 = fn_state.datasizeshadow_1849;
        // D s_2_1: call __id(s_2_0)
        let s_2_1: i128 = u__id(state, tracer, s_2_0);
        // C s_2_2: const #16s : i
        let s_2_2: i128 = 16;
        // D s_2_3: cmp-eq s_2_1 s_2_2
        let s_2_3: bool = ((s_2_1) == (s_2_2));
        // D s_2_4: write-var gs#169027 <= s_2_3
        fn_state.gs_169027 = s_2_3;
        // N s_2_5: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#169027:u8
        let s_3_0: bool = fn_state.gs_169027;
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
        // D s_4_0: read-var datasizeshadow#1849:i
        let s_4_0: i128 = fn_state.datasizeshadow_1849;
        // D s_4_1: call __id(s_4_0)
        let s_4_1: i128 = u__id(state, tracer, s_4_0);
        // C s_4_2: const #32s : i
        let s_4_2: i128 = 32;
        // D s_4_3: cmp-eq s_4_1 s_4_2
        let s_4_3: bool = ((s_4_1) == (s_4_2));
        // D s_4_4: write-var gs#169029 <= s_4_3
        fn_state.gs_169029 = s_4_3;
        // N s_4_5: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#169029:u8
        let s_5_0: bool = fn_state.gs_169029;
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
        // D s_6_0: read-var datasizeshadow#1849:i
        let s_6_0: i128 = fn_state.datasizeshadow_1849;
        // D s_6_1: call __id(s_6_0)
        let s_6_1: i128 = u__id(state, tracer, s_6_0);
        // C s_6_2: const #64s : i
        let s_6_2: i128 = 64;
        // D s_6_3: cmp-eq s_6_1 s_6_2
        let s_6_3: bool = ((s_6_1) == (s_6_2));
        // D s_6_4: write-var gs#169031 <= s_6_3
        fn_state.gs_169031 = s_6_3;
        // N s_6_5: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#169031:u8
        let s_7_0: bool = fn_state.gs_169031;
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
        // D s_8_0: read-var datasizeshadow#1849:i
        let s_8_0: i128 = fn_state.datasizeshadow_1849;
        // D s_8_1: call __id(s_8_0)
        let s_8_1: i128 = u__id(state, tracer, s_8_0);
        // C s_8_2: const #128s : i
        let s_8_2: i128 = 128;
        // D s_8_3: cmp-eq s_8_1 s_8_2
        let s_8_3: bool = ((s_8_1) == (s_8_2));
        // D s_8_4: write-var gs#169033 <= s_8_3
        fn_state.gs_169033 = s_8_3;
        // N s_8_5: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#169033:u8
        let s_9_0: bool = fn_state.gs_169033;
        // N s_9_1: assert s_9_0
        let s_9_1: () = assert!(s_9_0);
        // D s_9_2: read-var datasizeshadow#1849:i
        let s_9_2: i128 = fn_state.datasizeshadow_1849;
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
        // D s_9_8: read-var datasizeshadow#1849:i
        let s_9_8: i128 = fn_state.datasizeshadow_1849;
        // D s_9_9: cast reint s_9_8 -> i64
        let s_9_9: i64 = (s_9_8 as i64);
        // D s_9_10: read-var d:i64
        let s_9_10: i64 = fn_state.d;
        // D s_9_11: cast zx s_9_10 -> i
        let s_9_11: i128 = (i128::try_from(s_9_10).unwrap());
        // D s_9_12: call V_read(s_9_11, s_9_9)
        let s_9_12: Bits = V_read(state, tracer, s_9_11, s_9_9);
        // D s_9_13: write-var operand2 <= s_9_12
        fn_state.operand2 = s_9_12;
        // D s_9_14: read-var esizeshadow#1848:i
        let s_9_14: i128 = fn_state.esizeshadow_1848;
        // D s_9_15: call __id(s_9_14)
        let s_9_15: i128 = u__id(state, tracer, s_9_14);
        // C s_9_16: const #0s : i
        let s_9_16: i128 = 0;
        // D s_9_17: cmp-ge s_9_15 s_9_16
        let s_9_17: bool = ((s_9_15) >= (s_9_16));
        // N s_9_18: assert s_9_17
        let s_9_18: () = assert!(s_9_17);
        // D s_9_19: read-var esizeshadow#1848:i
        let s_9_19: i128 = fn_state.esizeshadow_1848;
        // D s_9_20: call Ones(s_9_19)
        let s_9_20: Bits = Ones(state, tracer, s_9_19);
        // D s_9_21: read-var shift:i
        let s_9_21: i128 = fn_state.shift;
        // D s_9_22: lsl s_9_20 s_9_21
        let s_9_22: Bits = s_9_20 << s_9_21;
        // D s_9_23: write-var mask <= s_9_22
        fn_state.mask = s_9_22;
        // C s_9_24: const #0s : i64
        let s_9_24: i64 = 0;
        // C s_9_25: const #1s : i
        let s_9_25: i128 = 1;
        // D s_9_26: read-var elements:i
        let s_9_26: i128 = fn_state.elements;
        // D s_9_27: sub s_9_26 s_9_25
        let s_9_27: i128 = ((s_9_26) - (s_9_25));
        // D s_9_28: cast reint s_9_27 -> i64
        let s_9_28: i64 = (s_9_27 as i64);
        // D s_9_29: write-var gs#169041 <= s_9_28
        fn_state.gs_169041 = s_9_28;
        // D s_9_30: write-var e <= s_9_24
        fn_state.e = s_9_24;
        // N s_9_31: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var e:i64
        let s_10_0: i64 = fn_state.e;
        // D s_10_1: read-var gs#169041:i64
        let s_10_1: i64 = fn_state.gs_169041;
        // D s_10_2: cmp-gt s_10_0 s_10_1
        let s_10_2: bool = ((s_10_0) > (s_10_1));
        // N s_10_3: branch s_10_2 b36 b11
        if s_10_2 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_11_0: read-var e:i64
        let s_11_0: i64 = fn_state.e;
        // D s_11_1: cast zx s_11_0 -> i
        let s_11_1: i128 = (i128::try_from(s_11_0).unwrap());
        // D s_11_2: call __id(s_11_1)
        let s_11_2: i128 = u__id(state, tracer, s_11_1);
        // D s_11_3: read-var esizeshadow#1848:i
        let s_11_3: i128 = fn_state.esizeshadow_1848;
        // D s_11_4: call __id(s_11_3)
        let s_11_4: i128 = u__id(state, tracer, s_11_3);
        // D s_11_5: mul s_11_2 s_11_4
        let s_11_5: i128 = ((s_11_2) * (s_11_4));
        // D s_11_6: read-var e:i64
        let s_11_6: i64 = fn_state.e;
        // D s_11_7: cast zx s_11_6 -> i
        let s_11_7: i128 = (i128::try_from(s_11_6).unwrap());
        // D s_11_8: call __id(s_11_7)
        let s_11_8: i128 = u__id(state, tracer, s_11_7);
        // D s_11_9: read-var esizeshadow#1848:i
        let s_11_9: i128 = fn_state.esizeshadow_1848;
        // D s_11_10: call __id(s_11_9)
        let s_11_10: i128 = u__id(state, tracer, s_11_9);
        // D s_11_11: mul s_11_8 s_11_10
        let s_11_11: i128 = ((s_11_8) * (s_11_10));
        // D s_11_12: read-var esizeshadow#1848:i
        let s_11_12: i128 = fn_state.esizeshadow_1848;
        // D s_11_13: call __id(s_11_12)
        let s_11_13: i128 = u__id(state, tracer, s_11_12);
        // D s_11_14: add s_11_11 s_11_13
        let s_11_14: i128 = (s_11_11 + s_11_13);
        // C s_11_15: const #1s : i
        let s_11_15: i128 = 1;
        // D s_11_16: sub s_11_14 s_11_15
        let s_11_16: i128 = ((s_11_14) - (s_11_15));
        // D s_11_17: cmp-le s_11_5 s_11_16
        let s_11_17: bool = ((s_11_5) <= (s_11_16));
        // N s_11_18: branch s_11_17 b35 b12
        if s_11_17 {
            return block_35(state, tracer, fn_state);
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
        // N s_12_5: branch s_12_4 b34 b13
        if s_12_4 {
            return block_34(state, tracer, fn_state);
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
        // D s_13_1: write-var gs#169047 <= s_13_0
        fn_state.gs_169047 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_14_0: read-var gs#169047:u8
        let s_14_0: bool = fn_state.gs_169047;
        // D s_14_1: not s_14_0
        let s_14_1: bool = !s_14_0;
        // N s_14_2: branch s_14_1 b33 b15
        if s_14_1 {
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
        // D s_15_0: read-var esizeshadow#1848:i
        let s_15_0: i128 = fn_state.esizeshadow_1848;
        // D s_15_1: call __id(s_15_0)
        let s_15_1: i128 = u__id(state, tracer, s_15_0);
        // C s_15_2: const #0s : i
        let s_15_2: i128 = 0;
        // D s_15_3: cmp-ge s_15_1 s_15_2
        let s_15_3: bool = ((s_15_1) >= (s_15_2));
        // D s_15_4: not s_15_3
        let s_15_4: bool = !s_15_3;
        // N s_15_5: branch s_15_4 b32 b16
        if s_15_4 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_16_0: read-var datasizeshadow#1849:i
        let s_16_0: i128 = fn_state.datasizeshadow_1849;
        // D s_16_1: call __id(s_16_0)
        let s_16_1: i128 = u__id(state, tracer, s_16_0);
        // D s_16_2: cast reint s_16_1 -> i64
        let s_16_2: i64 = (s_16_1 as i64);
        // C s_16_3: const #8s : i
        let s_16_3: i128 = 8;
        // D s_16_4: cast zx s_16_2 -> i
        let s_16_4: i128 = (i128::try_from(s_16_2).unwrap());
        // D s_16_5: cmp-eq s_16_4 s_16_3
        let s_16_5: bool = ((s_16_4) == (s_16_3));
        // N s_16_6: branch s_16_5 b31 b17
        if s_16_5 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_17_0: read-var datasizeshadow#1849:i
        let s_17_0: i128 = fn_state.datasizeshadow_1849;
        // D s_17_1: call __id(s_17_0)
        let s_17_1: i128 = u__id(state, tracer, s_17_0);
        // D s_17_2: cast reint s_17_1 -> i64
        let s_17_2: i64 = (s_17_1 as i64);
        // C s_17_3: const #16s : i
        let s_17_3: i128 = 16;
        // D s_17_4: cast zx s_17_2 -> i
        let s_17_4: i128 = (i128::try_from(s_17_2).unwrap());
        // D s_17_5: cmp-eq s_17_4 s_17_3
        let s_17_5: bool = ((s_17_4) == (s_17_3));
        // D s_17_6: write-var gs#169051 <= s_17_5
        fn_state.gs_169051 = s_17_5;
        // N s_17_7: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var gs#169051:u8
        let s_18_0: bool = fn_state.gs_169051;
        // N s_18_1: branch s_18_0 b30 b19
        if s_18_0 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_19_0: read-var datasizeshadow#1849:i
        let s_19_0: i128 = fn_state.datasizeshadow_1849;
        // D s_19_1: call __id(s_19_0)
        let s_19_1: i128 = u__id(state, tracer, s_19_0);
        // D s_19_2: cast reint s_19_1 -> i64
        let s_19_2: i64 = (s_19_1 as i64);
        // C s_19_3: const #32s : i
        let s_19_3: i128 = 32;
        // D s_19_4: cast zx s_19_2 -> i
        let s_19_4: i128 = (i128::try_from(s_19_2).unwrap());
        // D s_19_5: cmp-eq s_19_4 s_19_3
        let s_19_5: bool = ((s_19_4) == (s_19_3));
        // D s_19_6: write-var gs#169053 <= s_19_5
        fn_state.gs_169053 = s_19_5;
        // N s_19_7: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var gs#169053:u8
        let s_20_0: bool = fn_state.gs_169053;
        // N s_20_1: branch s_20_0 b29 b21
        if s_20_0 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_21_0: read-var datasizeshadow#1849:i
        let s_21_0: i128 = fn_state.datasizeshadow_1849;
        // D s_21_1: call __id(s_21_0)
        let s_21_1: i128 = u__id(state, tracer, s_21_0);
        // D s_21_2: cast reint s_21_1 -> i64
        let s_21_2: i64 = (s_21_1 as i64);
        // C s_21_3: const #64s : i
        let s_21_3: i128 = 64;
        // D s_21_4: cast zx s_21_2 -> i
        let s_21_4: i128 = (i128::try_from(s_21_2).unwrap());
        // D s_21_5: cmp-eq s_21_4 s_21_3
        let s_21_5: bool = ((s_21_4) == (s_21_3));
        // D s_21_6: write-var gs#169055 <= s_21_5
        fn_state.gs_169055 = s_21_5;
        // N s_21_7: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_22_0: read-var gs#169055:u8
        let s_22_0: bool = fn_state.gs_169055;
        // N s_22_1: branch s_22_0 b28 b23
        if s_22_0 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_23_0: read-var datasizeshadow#1849:i
        let s_23_0: i128 = fn_state.datasizeshadow_1849;
        // D s_23_1: call __id(s_23_0)
        let s_23_1: i128 = u__id(state, tracer, s_23_0);
        // D s_23_2: cast reint s_23_1 -> i64
        let s_23_2: i64 = (s_23_1 as i64);
        // C s_23_3: const #128s : i
        let s_23_3: i128 = 128;
        // D s_23_4: cast zx s_23_2 -> i
        let s_23_4: i128 = (i128::try_from(s_23_2).unwrap());
        // D s_23_5: cmp-eq s_23_4 s_23_3
        let s_23_5: bool = ((s_23_4) == (s_23_3));
        // D s_23_6: write-var gs#169057 <= s_23_5
        fn_state.gs_169057 = s_23_5;
        // N s_23_7: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_24_0: read-var gs#169057:u8
        let s_24_0: bool = fn_state.gs_169057;
        // D s_24_1: not s_24_0
        let s_24_1: bool = !s_24_0;
        // D s_24_2: write-var gs#169058 <= s_24_1
        fn_state.gs_169058 = s_24_1;
        // N s_24_3: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_25_0: read-var gs#169058:u8
        let s_25_0: bool = fn_state.gs_169058;
        // D s_25_1: write-var gs#169059 <= s_25_0
        fn_state.gs_169059 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_26_0: read-var gs#169059:u8
        let s_26_0: bool = fn_state.gs_169059;
        // D s_26_1: write-var gs#169060 <= s_26_0
        fn_state.gs_169060 = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_27_0: read-var gs#169060:u8
        let s_27_0: bool = fn_state.gs_169060;
        // N s_27_1: assert s_27_0
        let s_27_1: () = assert!(s_27_0);
        // D s_27_2: read-var esizeshadow#1848:i
        let s_27_2: i128 = fn_state.esizeshadow_1848;
        // D s_27_3: read-var e:i64
        let s_27_3: i64 = fn_state.e;
        // D s_27_4: cast zx s_27_3 -> i
        let s_27_4: i128 = (i128::try_from(s_27_3).unwrap());
        // D s_27_5: read-var operand:bv
        let s_27_5: Bits = fn_state.operand;
        // D s_27_6: call Elem_read(s_27_5, s_27_4, s_27_2)
        let s_27_6: Bits = Elem_read(state, tracer, s_27_5, s_27_4, s_27_2);
        // D s_27_7: read-var shift:i
        let s_27_7: i128 = fn_state.shift;
        // D s_27_8: lsl s_27_6 s_27_7
        let s_27_8: Bits = s_27_6 << s_27_7;
        // D s_27_9: read-var esizeshadow#1848:i
        let s_27_9: i128 = fn_state.esizeshadow_1848;
        // D s_27_10: read-var esizeshadow#1848:i
        let s_27_10: i128 = fn_state.esizeshadow_1848;
        // D s_27_11: read-var e:i64
        let s_27_11: i64 = fn_state.e;
        // D s_27_12: cast zx s_27_11 -> i
        let s_27_12: i128 = (i128::try_from(s_27_11).unwrap());
        // D s_27_13: read-var operand2:bv
        let s_27_13: Bits = fn_state.operand2;
        // D s_27_14: call Elem_read(s_27_13, s_27_12, s_27_10)
        let s_27_14: Bits = Elem_read(state, tracer, s_27_13, s_27_12, s_27_10);
        // D s_27_15: read-var mask:bv
        let s_27_15: Bits = fn_state.mask;
        // D s_27_16: not s_27_15
        let s_27_16: Bits = !s_27_15;
        // D s_27_17: and s_27_14 s_27_16
        let s_27_17: Bits = ((s_27_14) & (s_27_16));
        // D s_27_18: or s_27_17 s_27_8
        let s_27_18: Bits = ((s_27_17) | (s_27_8));
        // D s_27_19: read-var e:i64
        let s_27_19: i64 = fn_state.e;
        // D s_27_20: cast zx s_27_19 -> i
        let s_27_20: i128 = (i128::try_from(s_27_19).unwrap());
        // D s_27_21: read-var result:bv
        let s_27_21: Bits = fn_state.result;
        // D s_27_22: call Elem_set(s_27_21, s_27_20, s_27_9, s_27_18)
        let s_27_22: Bits = Elem_set(state, tracer, s_27_21, s_27_20, s_27_9, s_27_18);
        // D s_27_23: write-var result <= s_27_22
        fn_state.result = s_27_22;
        // D s_27_24: read-var e:i64
        let s_27_24: i64 = fn_state.e;
        // C s_27_25: const #1s : i64
        let s_27_25: i64 = 1;
        // D s_27_26: add s_27_24 s_27_25
        let s_27_26: i64 = (s_27_24 + s_27_25);
        // D s_27_27: write-var e <= s_27_26
        fn_state.e = s_27_26;
        // N s_27_28: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_28_0: const #1u : u8
        let s_28_0: bool = true;
        // D s_28_1: write-var gs#169057 <= s_28_0
        fn_state.gs_169057 = s_28_0;
        // N s_28_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_29_0: const #1u : u8
        let s_29_0: bool = true;
        // D s_29_1: write-var gs#169055 <= s_29_0
        fn_state.gs_169055 = s_29_0;
        // N s_29_2: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_30_0: const #1u : u8
        let s_30_0: bool = true;
        // D s_30_1: write-var gs#169053 <= s_30_0
        fn_state.gs_169053 = s_30_0;
        // N s_30_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_31_0: const #1u : u8
        let s_31_0: bool = true;
        // D s_31_1: write-var gs#169051 <= s_31_0
        fn_state.gs_169051 = s_31_0;
        // N s_31_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_32_0: const #1u : u8
        let s_32_0: bool = true;
        // D s_32_1: write-var gs#169058 <= s_32_0
        fn_state.gs_169058 = s_32_0;
        // N s_32_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_33_0: const #1u : u8
        let s_33_0: bool = true;
        // D s_33_1: write-var gs#169059 <= s_33_0
        fn_state.gs_169059 = s_33_0;
        // N s_33_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_34_0: read-var e:i64
        let s_34_0: i64 = fn_state.e;
        // D s_34_1: cast zx s_34_0 -> i
        let s_34_1: i128 = (i128::try_from(s_34_0).unwrap());
        // D s_34_2: call __id(s_34_1)
        let s_34_2: i128 = u__id(state, tracer, s_34_1);
        // C s_34_3: const #1s : i
        let s_34_3: i128 = 1;
        // D s_34_4: add s_34_2 s_34_3
        let s_34_4: i128 = (s_34_2 + s_34_3);
        // D s_34_5: read-var esizeshadow#1848:i
        let s_34_5: i128 = fn_state.esizeshadow_1848;
        // D s_34_6: call __id(s_34_5)
        let s_34_6: i128 = u__id(state, tracer, s_34_5);
        // D s_34_7: mul s_34_4 s_34_6
        let s_34_7: i128 = ((s_34_4) * (s_34_6));
        // D s_34_8: read-var datasizeshadow#1849:i
        let s_34_8: i128 = fn_state.datasizeshadow_1849;
        // D s_34_9: call __id(s_34_8)
        let s_34_9: i128 = u__id(state, tracer, s_34_8);
        // D s_34_10: cast reint s_34_9 -> i64
        let s_34_10: i64 = (s_34_9 as i64);
        // D s_34_11: cast zx s_34_10 -> i
        let s_34_11: i128 = (i128::try_from(s_34_10).unwrap());
        // D s_34_12: cmp-le s_34_7 s_34_11
        let s_34_12: bool = ((s_34_7) <= (s_34_11));
        // D s_34_13: write-var gs#169047 <= s_34_12
        fn_state.gs_169047 = s_34_12;
        // N s_34_14: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_35_0: const #1u : u8
        let s_35_0: bool = true;
        // D s_35_1: write-var gs#169060 <= s_35_0
        fn_state.gs_169060 = s_35_0;
        // N s_35_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_36_0: read-var datasizeshadow#1849:i
        let s_36_0: i128 = fn_state.datasizeshadow_1849;
        // D s_36_1: cast reint s_36_0 -> i64
        let s_36_1: i64 = (s_36_0 as i64);
        // D s_36_2: read-var d:i64
        let s_36_2: i64 = fn_state.d;
        // D s_36_3: cast zx s_36_2 -> i
        let s_36_3: i128 = (i128::try_from(s_36_2).unwrap());
        // D s_36_4: read-var result:bv
        let s_36_4: Bits = fn_state.result;
        // D s_36_5: call V_set(s_36_3, s_36_1, s_36_4)
        let s_36_5: () = V_set(state, tracer, s_36_3, s_36_1, s_36_4);
        // N s_36_6: return
        return;
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_37_0: const #1u : u8
        let s_37_0: bool = true;
        // D s_37_1: write-var gs#169033 <= s_37_0
        fn_state.gs_169033 = s_37_0;
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
        // D s_38_1: write-var gs#169031 <= s_38_0
        fn_state.gs_169031 = s_38_0;
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
        // D s_39_1: write-var gs#169029 <= s_39_0
        fn_state.gs_169029 = s_39_0;
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
        // D s_40_1: write-var gs#169027 <= s_40_0
        fn_state.gs_169027 = s_40_0;
        // N s_40_2: jump b3
        return block_3(state, tracer, fn_state);
    }
}
