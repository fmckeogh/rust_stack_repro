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
use ZAhslice_set::*;
use CurrentSVL_read::*;
use u__id::*;
use Elem_read::*;
use ZAhslice_read::*;
use fdiv_int::*;
use common::*;
pub fn ZAvslice_set<T: Tracer>(
    state: &mut State,
    tracer: &T,
    tile: i128,
    esize: i128,
    slice_name: i128,
    width: i128,
    value_name: Bits,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_22736: bool,
        s: i64,
        hslice: Bits,
        widthshadow_392: i128,
        gs_22729: i64,
        esizeshadow_393: i128,
        gs_22735: bool,
        tile: i128,
        esize: i128,
        slice_name: i128,
        width: i128,
        value_name: Bits,
    }
    let fn_state = FunctionState {
        tile,
        esize,
        slice_name,
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
        // D s_0_1: write-var widthshadow#392 <= s_0_0
        fn_state.widthshadow_392 = s_0_0;
        // D s_0_2: read-var esize:i
        let s_0_2: i128 = fn_state.esize;
        // D s_0_3: write-var esizeshadow#393 <= s_0_2
        fn_state.esizeshadow_393 = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call CurrentSVL_read(s_0_4)
        let s_0_5: i64 = CurrentSVL_read(state, tracer, s_0_4);
        // S s_0_6: cast zx s_0_5 -> i
        let s_0_6: i128 = (i128::try_from(s_0_5).unwrap());
        // D s_0_7: read-var esizeshadow#393:i
        let s_0_7: i128 = fn_state.esizeshadow_393;
        // D s_0_8: call fdiv_int(s_0_6, s_0_7)
        let s_0_8: i128 = fdiv_int(state, tracer, s_0_6, s_0_7);
        // C s_0_9: const #0s : i64
        let s_0_9: i64 = 0;
        // C s_0_10: const #1s : i
        let s_0_10: i128 = 1;
        // D s_0_11: sub s_0_8 s_0_10
        let s_0_11: i128 = ((s_0_8) - (s_0_10));
        // D s_0_12: cast reint s_0_11 -> i64
        let s_0_12: i64 = (s_0_11 as i64);
        // D s_0_13: write-var gs#22729 <= s_0_12
        fn_state.gs_22729 = s_0_12;
        // D s_0_14: write-var s <= s_0_9
        fn_state.s = s_0_9;
        // N s_0_15: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var s:i64
        let s_1_0: i64 = fn_state.s;
        // D s_1_1: read-var gs#22729:i64
        let s_1_1: i64 = fn_state.gs_22729;
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
        // D s_2_0: read-var widthshadow#392:i
        let s_2_0: i128 = fn_state.widthshadow_392;
        // D s_2_1: read-var s:i64
        let s_2_1: i64 = fn_state.s;
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (i128::try_from(s_2_1).unwrap());
        // D s_2_3: read-var tile:i
        let s_2_3: i128 = fn_state.tile;
        // D s_2_4: read-var esizeshadow#393:i
        let s_2_4: i128 = fn_state.esizeshadow_393;
        // D s_2_5: call ZAhslice_read(s_2_3, s_2_4, s_2_2, s_2_0)
        let s_2_5: Bits = ZAhslice_read(state, tracer, s_2_3, s_2_4, s_2_2, s_2_0);
        // D s_2_6: write-var hslice <= s_2_5
        fn_state.hslice = s_2_5;
        // D s_2_7: read-var s:i64
        let s_2_7: i64 = fn_state.s;
        // D s_2_8: cast zx s_2_7 -> i
        let s_2_8: i128 = (i128::try_from(s_2_7).unwrap());
        // D s_2_9: call __id(s_2_8)
        let s_2_9: i128 = u__id(state, tracer, s_2_8);
        // D s_2_10: read-var esizeshadow#393:i
        let s_2_10: i128 = fn_state.esizeshadow_393;
        // D s_2_11: call __id(s_2_10)
        let s_2_11: i128 = u__id(state, tracer, s_2_10);
        // D s_2_12: mul s_2_9 s_2_11
        let s_2_12: i128 = ((s_2_9) * (s_2_11));
        // D s_2_13: read-var s:i64
        let s_2_13: i64 = fn_state.s;
        // D s_2_14: cast zx s_2_13 -> i
        let s_2_14: i128 = (i128::try_from(s_2_13).unwrap());
        // D s_2_15: call __id(s_2_14)
        let s_2_15: i128 = u__id(state, tracer, s_2_14);
        // D s_2_16: read-var esizeshadow#393:i
        let s_2_16: i128 = fn_state.esizeshadow_393;
        // D s_2_17: call __id(s_2_16)
        let s_2_17: i128 = u__id(state, tracer, s_2_16);
        // D s_2_18: mul s_2_15 s_2_17
        let s_2_18: i128 = ((s_2_15) * (s_2_17));
        // D s_2_19: read-var esizeshadow#393:i
        let s_2_19: i128 = fn_state.esizeshadow_393;
        // D s_2_20: call __id(s_2_19)
        let s_2_20: i128 = u__id(state, tracer, s_2_19);
        // D s_2_21: add s_2_18 s_2_20
        let s_2_21: i128 = (s_2_18 + s_2_20);
        // C s_2_22: const #1s : i
        let s_2_22: i128 = 1;
        // D s_2_23: sub s_2_21 s_2_22
        let s_2_23: i128 = ((s_2_21) - (s_2_22));
        // D s_2_24: cmp-le s_2_12 s_2_23
        let s_2_24: bool = ((s_2_12) <= (s_2_23));
        // N s_2_25: branch s_2_24 b8 b3
        if s_2_24 {
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
        // D s_3_0: read-var s:i64
        let s_3_0: i64 = fn_state.s;
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
        // D s_4_1: write-var gs#22735 <= s_4_0
        fn_state.gs_22735 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var gs#22735:u8
        let s_5_0: bool = fn_state.gs_22735;
        // D s_5_1: not s_5_0
        let s_5_1: bool = !s_5_0;
        // D s_5_2: write-var gs#22736 <= s_5_1
        fn_state.gs_22736 = s_5_1;
        // N s_5_3: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#22736:u8
        let s_6_0: bool = fn_state.gs_22736;
        // N s_6_1: assert s_6_0
        let s_6_1: () = assert!(s_6_0);
        // D s_6_2: read-var esizeshadow#393:i
        let s_6_2: i128 = fn_state.esizeshadow_393;
        // D s_6_3: read-var esizeshadow#393:i
        let s_6_3: i128 = fn_state.esizeshadow_393;
        // D s_6_4: read-var s:i64
        let s_6_4: i64 = fn_state.s;
        // D s_6_5: cast zx s_6_4 -> i
        let s_6_5: i128 = (i128::try_from(s_6_4).unwrap());
        // D s_6_6: read-var value_name:bv
        let s_6_6: Bits = fn_state.value_name;
        // D s_6_7: call Elem_read(s_6_6, s_6_5, s_6_3)
        let s_6_7: Bits = Elem_read(state, tracer, s_6_6, s_6_5, s_6_3);
        // D s_6_8: read-var hslice:bv
        let s_6_8: Bits = fn_state.hslice;
        // D s_6_9: read-var slice_name:i
        let s_6_9: i128 = fn_state.slice_name;
        // D s_6_10: call Elem_set(s_6_8, s_6_9, s_6_2, s_6_7)
        let s_6_10: Bits = Elem_set(state, tracer, s_6_8, s_6_9, s_6_2, s_6_7);
        // D s_6_11: write-var hslice <= s_6_10
        fn_state.hslice = s_6_10;
        // D s_6_12: read-var widthshadow#392:i
        let s_6_12: i128 = fn_state.widthshadow_392;
        // D s_6_13: read-var s:i64
        let s_6_13: i64 = fn_state.s;
        // D s_6_14: cast zx s_6_13 -> i
        let s_6_14: i128 = (i128::try_from(s_6_13).unwrap());
        // D s_6_15: read-var tile:i
        let s_6_15: i128 = fn_state.tile;
        // D s_6_16: read-var esizeshadow#393:i
        let s_6_16: i128 = fn_state.esizeshadow_393;
        // D s_6_17: read-var hslice:bv
        let s_6_17: Bits = fn_state.hslice;
        // D s_6_18: call ZAhslice_set(s_6_15, s_6_16, s_6_14, s_6_12, s_6_17)
        let s_6_18: () = ZAhslice_set(
            state,
            tracer,
            s_6_15,
            s_6_16,
            s_6_14,
            s_6_12,
            s_6_17,
        );
        // D s_6_19: read-var s:i64
        let s_6_19: i64 = fn_state.s;
        // C s_6_20: const #1s : i64
        let s_6_20: i64 = 1;
        // D s_6_21: add s_6_19 s_6_20
        let s_6_21: i64 = (s_6_19 + s_6_20);
        // D s_6_22: write-var s <= s_6_21
        fn_state.s = s_6_21;
        // N s_6_23: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var s:i64
        let s_7_0: i64 = fn_state.s;
        // D s_7_1: cast zx s_7_0 -> i
        let s_7_1: i128 = (i128::try_from(s_7_0).unwrap());
        // D s_7_2: call __id(s_7_1)
        let s_7_2: i128 = u__id(state, tracer, s_7_1);
        // C s_7_3: const #1s : i
        let s_7_3: i128 = 1;
        // D s_7_4: add s_7_2 s_7_3
        let s_7_4: i128 = (s_7_2 + s_7_3);
        // D s_7_5: read-var esizeshadow#393:i
        let s_7_5: i128 = fn_state.esizeshadow_393;
        // D s_7_6: call __id(s_7_5)
        let s_7_6: i128 = u__id(state, tracer, s_7_5);
        // D s_7_7: mul s_7_4 s_7_6
        let s_7_7: i128 = ((s_7_4) * (s_7_6));
        // D s_7_8: read-var widthshadow#392:i
        let s_7_8: i128 = fn_state.widthshadow_392;
        // D s_7_9: call __id(s_7_8)
        let s_7_9: i128 = u__id(state, tracer, s_7_8);
        // D s_7_10: cmp-le s_7_7 s_7_9
        let s_7_10: bool = ((s_7_7) <= (s_7_9));
        // D s_7_11: write-var gs#22735 <= s_7_10
        fn_state.gs_22735 = s_7_10;
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
        // D s_8_1: write-var gs#22736 <= s_8_0
        fn_state.gs_22736 = s_8_0;
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
