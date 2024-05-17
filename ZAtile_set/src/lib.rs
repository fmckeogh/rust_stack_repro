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
use u__id::*;
use Elem_read::*;
use ZAhslice_set::*;
use fdiv_int::*;
use common::*;
pub fn ZAtile_set<T: Tracer>(
    state: &mut State,
    tracer: &T,
    SVL: i128,
    tile: i128,
    esize: i128,
    width: i128,
    value_name: Bits,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        widthshadow_400: i128,
        gs_22787: bool,
        gs_22786: bool,
        gs_22780: i64,
        SVLshadow_401: i128,
        slice_name: i64,
        SVL: i128,
        tile: i128,
        esize: i128,
        width: i128,
        value_name: Bits,
    }
    let fn_state = FunctionState {
        SVL,
        tile,
        esize,
        width,
        value_name,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var width:i
        let s_0_0: i128 = fn_state.width;
        // D s_0_1: write-var widthshadow#400 <= s_0_0
        fn_state.widthshadow_400 = s_0_0;
        // D s_0_2: read-var SVL:i
        let s_0_2: i128 = fn_state.SVL;
        // D s_0_3: write-var SVLshadow#401 <= s_0_2
        fn_state.SVLshadow_401 = s_0_2;
        // D s_0_4: read-var SVLshadow#401:i
        let s_0_4: i128 = fn_state.SVLshadow_401;
        // D s_0_5: read-var esize:i
        let s_0_5: i128 = fn_state.esize;
        // D s_0_6: call fdiv_int(s_0_4, s_0_5)
        let s_0_6: i128 = fdiv_int(state, tracer, s_0_4, s_0_5);
        // D s_0_7: read-var SVLshadow#401:i
        let s_0_7: i128 = fn_state.SVLshadow_401;
        // D s_0_8: mul s_0_7 s_0_6
        let s_0_8: i128 = ((s_0_7) * (s_0_6));
        // D s_0_9: read-var widthshadow#400:i
        let s_0_9: i128 = fn_state.widthshadow_400;
        // D s_0_10: cmp-eq s_0_9 s_0_8
        let s_0_10: bool = ((s_0_9) == (s_0_8));
        // N s_0_11: assert s_0_10
        let s_0_11: () = assert!(s_0_10);
        // C s_0_12: const #0s : i64
        let s_0_12: i64 = 0;
        // C s_0_13: const #1s : i
        let s_0_13: i128 = 1;
        // D s_0_14: sub s_0_6 s_0_13
        let s_0_14: i128 = ((s_0_6) - (s_0_13));
        // D s_0_15: cast reint s_0_14 -> i64
        let s_0_15: i64 = (s_0_14 as i64);
        // D s_0_16: write-var gs#22780 <= s_0_15
        fn_state.gs_22780 = s_0_15;
        // D s_0_17: write-var slice_name <= s_0_12
        fn_state.slice_name = s_0_12;
        // N s_0_18: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var slice_name:i64
        let s_1_0: i64 = fn_state.slice_name;
        // D s_1_1: read-var gs#22780:i64
        let s_1_1: i64 = fn_state.gs_22780;
        // D s_1_2: cmp-gt s_1_0 s_1_1
        let s_1_2: bool = ((s_1_0) > (s_1_1));
        // N s_1_3: branch s_1_2 b9 b2
        if s_1_2 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var slice_name:i64
        let s_2_0: i64 = fn_state.slice_name;
        // D s_2_1: cast zx s_2_0 -> i
        let s_2_1: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_2: call __id(s_2_1)
        let s_2_2: i128 = u__id(state, tracer, s_2_1);
        // D s_2_3: read-var SVLshadow#401:i
        let s_2_3: i128 = fn_state.SVLshadow_401;
        // D s_2_4: call __id(s_2_3)
        let s_2_4: i128 = u__id(state, tracer, s_2_3);
        // D s_2_5: mul s_2_2 s_2_4
        let s_2_5: i128 = ((s_2_2) * (s_2_4));
        // D s_2_6: read-var slice_name:i64
        let s_2_6: i64 = fn_state.slice_name;
        // D s_2_7: cast zx s_2_6 -> i
        let s_2_7: i128 = (i128::try_from(s_2_6).unwrap());
        // D s_2_8: call __id(s_2_7)
        let s_2_8: i128 = u__id(state, tracer, s_2_7);
        // D s_2_9: read-var SVLshadow#401:i
        let s_2_9: i128 = fn_state.SVLshadow_401;
        // D s_2_10: call __id(s_2_9)
        let s_2_10: i128 = u__id(state, tracer, s_2_9);
        // D s_2_11: mul s_2_8 s_2_10
        let s_2_11: i128 = ((s_2_8) * (s_2_10));
        // D s_2_12: read-var SVLshadow#401:i
        let s_2_12: i128 = fn_state.SVLshadow_401;
        // D s_2_13: call __id(s_2_12)
        let s_2_13: i128 = u__id(state, tracer, s_2_12);
        // D s_2_14: add s_2_11 s_2_13
        let s_2_14: i128 = (s_2_11 + s_2_13);
        // C s_2_15: const #1s : i
        let s_2_15: i128 = 1;
        // D s_2_16: sub s_2_14 s_2_15
        let s_2_16: i128 = ((s_2_14) - (s_2_15));
        // D s_2_17: cmp-le s_2_5 s_2_16
        let s_2_17: bool = ((s_2_5) <= (s_2_16));
        // N s_2_18: branch s_2_17 b8 b3
        if s_2_17 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var slice_name:i64
        let s_3_0: i64 = fn_state.slice_name;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: call __id(s_3_1)
        let s_3_2: i128 = u__id(state, tracer, s_3_1);
        // C s_3_3: const #0s : i
        let s_3_3: i128 = 0;
        // D s_3_4: cmp-ge s_3_2 s_3_3
        let s_3_4: bool = ((s_3_2) >= (s_3_3));
        // N s_3_5: branch s_3_4 b7 b4
        if s_3_4 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #0u : u8
        let s_4_0: bool = false;
        // D s_4_1: write-var gs#22786 <= s_4_0
        fn_state.gs_22786 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#22786:u8
        let s_5_0: bool = fn_state.gs_22786;
        // D s_5_1: not s_5_0
        let s_5_1: bool = !s_5_0;
        // D s_5_2: write-var gs#22787 <= s_5_1
        fn_state.gs_22787 = s_5_1;
        // N s_5_3: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#22787:u8
        let s_6_0: bool = fn_state.gs_22787;
        // N s_6_1: assert s_6_0
        let s_6_1: () = assert!(s_6_0);
        // D s_6_2: read-var SVLshadow#401:i
        let s_6_2: i128 = fn_state.SVLshadow_401;
        // D s_6_3: read-var SVLshadow#401:i
        let s_6_3: i128 = fn_state.SVLshadow_401;
        // D s_6_4: read-var slice_name:i64
        let s_6_4: i64 = fn_state.slice_name;
        // D s_6_5: cast zx s_6_4 -> i
        let s_6_5: i128 = (i128::try_from(s_6_4).unwrap());
        // D s_6_6: read-var value_name:bv
        let s_6_6: Bits = fn_state.value_name;
        // D s_6_7: call Elem_read(s_6_6, s_6_5, s_6_3)
        let s_6_7: Bits = Elem_read(state, tracer, s_6_6, s_6_5, s_6_3);
        // D s_6_8: read-var slice_name:i64
        let s_6_8: i64 = fn_state.slice_name;
        // D s_6_9: cast zx s_6_8 -> i
        let s_6_9: i128 = (i128::try_from(s_6_8).unwrap());
        // D s_6_10: read-var tile:i
        let s_6_10: i128 = fn_state.tile;
        // D s_6_11: read-var esize:i
        let s_6_11: i128 = fn_state.esize;
        // D s_6_12: call ZAhslice_set(s_6_10, s_6_11, s_6_9, s_6_2, s_6_7)
        let s_6_12: () = ZAhslice_set(
            state,
            tracer,
            s_6_10,
            s_6_11,
            s_6_9,
            s_6_2,
            s_6_7,
        );
        // D s_6_13: read-var slice_name:i64
        let s_6_13: i64 = fn_state.slice_name;
        // C s_6_14: const #1s : i64
        let s_6_14: i64 = 1;
        // D s_6_15: add s_6_13 s_6_14
        let s_6_15: i64 = (s_6_13 + s_6_14);
        // D s_6_16: write-var slice_name <= s_6_15
        fn_state.slice_name = s_6_15;
        // N s_6_17: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var slice_name:i64
        let s_7_0: i64 = fn_state.slice_name;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: call __id(s_7_1)
        let s_7_2: i128 = u__id(state, tracer, s_7_1);
        // C s_7_3: const #1s : i
        let s_7_3: i128 = 1;
        // D s_7_4: add s_7_2 s_7_3
        let s_7_4: i128 = (s_7_2 + s_7_3);
        // D s_7_5: read-var SVLshadow#401:i
        let s_7_5: i128 = fn_state.SVLshadow_401;
        // D s_7_6: call __id(s_7_5)
        let s_7_6: i128 = u__id(state, tracer, s_7_5);
        // D s_7_7: mul s_7_4 s_7_6
        let s_7_7: i128 = ((s_7_4) * (s_7_6));
        // D s_7_8: read-var widthshadow#400:i
        let s_7_8: i128 = fn_state.widthshadow_400;
        // D s_7_9: call __id(s_7_8)
        let s_7_9: i128 = u__id(state, tracer, s_7_8);
        // D s_7_10: cmp-le s_7_7 s_7_9
        let s_7_10: bool = ((s_7_7) <= (s_7_9));
        // D s_7_11: write-var gs#22786 <= s_7_10
        fn_state.gs_22786 = s_7_10;
        // N s_7_12: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #1u : u8
        let s_8_0: bool = true;
        // D s_8_1: write-var gs#22787 <= s_8_0
        fn_state.gs_22787 = s_8_0;
        // N s_8_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_9_0: return
        return;
    }
}
