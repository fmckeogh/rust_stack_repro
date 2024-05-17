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
use ZAvslice_set::*;
use ZAhslice_set::*;
use common::*;
pub fn ZAslice_set<T: Tracer>(
    state: &mut State,
    tracer: &T,
    tile: i128,
    esize: i128,
    vertical: bool,
    slice_name: i128,
    width: i128,
    value_name: Bits,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        widthshadow_396: i128,
        esizeshadow_397: i128,
        tile: i128,
        esize: i128,
        vertical: bool,
        slice_name: i128,
        width: i128,
        value_name: Bits,
    }
    let fn_state = FunctionState {
        tile,
        esize,
        vertical,
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
        // D s_0_1: write-var widthshadow#396 <= s_0_0
        fn_state.widthshadow_396 = s_0_0;
        // D s_0_2: read-var esize:i
        let s_0_2: i128 = fn_state.esize;
        // D s_0_3: write-var esizeshadow#397 <= s_0_2
        fn_state.esizeshadow_397 = s_0_2;
        // D s_0_4: read-var vertical:u8
        let s_0_4: bool = fn_state.vertical;
        // N s_0_5: branch s_0_4 b2 b1
        if s_0_4 {
            return block_2(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var widthshadow#396:i
        let s_1_0: i128 = fn_state.widthshadow_396;
        // D s_1_1: read-var tile:i
        let s_1_1: i128 = fn_state.tile;
        // D s_1_2: read-var esizeshadow#397:i
        let s_1_2: i128 = fn_state.esizeshadow_397;
        // D s_1_3: read-var slice_name:i
        let s_1_3: i128 = fn_state.slice_name;
        // D s_1_4: read-var value_name:bv
        let s_1_4: Bits = fn_state.value_name;
        // D s_1_5: call ZAhslice_set(s_1_1, s_1_2, s_1_3, s_1_0, s_1_4)
        let s_1_5: () = ZAhslice_set(state, tracer, s_1_1, s_1_2, s_1_3, s_1_0, s_1_4);
        // N s_1_6: return
        return;
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var widthshadow#396:i
        let s_2_0: i128 = fn_state.widthshadow_396;
        // D s_2_1: call __id(s_2_0)
        let s_2_1: i128 = u__id(state, tracer, s_2_0);
        // C s_2_2: const #0s : i
        let s_2_2: i128 = 0;
        // D s_2_3: cmp-ge s_2_1 s_2_2
        let s_2_3: bool = ((s_2_1) >= (s_2_2));
        // N s_2_4: assert s_2_3
        let s_2_4: () = assert!(s_2_3);
        // D s_2_5: read-var esizeshadow#397:i
        let s_2_5: i128 = fn_state.esizeshadow_397;
        // D s_2_6: call __id(s_2_5)
        let s_2_6: i128 = u__id(state, tracer, s_2_5);
        // C s_2_7: const #0s : i
        let s_2_7: i128 = 0;
        // D s_2_8: cmp-ge s_2_6 s_2_7
        let s_2_8: bool = ((s_2_6) >= (s_2_7));
        // N s_2_9: assert s_2_8
        let s_2_9: () = assert!(s_2_8);
        // D s_2_10: read-var esizeshadow#397:i
        let s_2_10: i128 = fn_state.esizeshadow_397;
        // D s_2_11: read-var widthshadow#396:i
        let s_2_11: i128 = fn_state.widthshadow_396;
        // D s_2_12: read-var tile:i
        let s_2_12: i128 = fn_state.tile;
        // D s_2_13: read-var slice_name:i
        let s_2_13: i128 = fn_state.slice_name;
        // D s_2_14: read-var value_name:bv
        let s_2_14: Bits = fn_state.value_name;
        // D s_2_15: call ZAvslice_set(s_2_12, s_2_10, s_2_13, s_2_11, s_2_14)
        let s_2_15: () = ZAvslice_set(
            state,
            tracer,
            s_2_12,
            s_2_10,
            s_2_13,
            s_2_11,
            s_2_14,
        );
        // N s_2_16: return
        return;
    }
}
