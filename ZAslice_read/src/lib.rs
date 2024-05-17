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
use ZAvslice_read::*;
use u__id::*;
use ZAhslice_read::*;
use common::*;
pub fn ZAslice_read<T: Tracer>(
    state: &mut State,
    tracer: &T,
    tile: i128,
    esize: i128,
    vertical: bool,
    slice_name: i128,
    width: i128,
) -> Bits {
    #[derive(Default)]
    struct FunctionState {
        esizeshadow_395: i128,
        gs_22748: bool,
        widthshadow_394: i128,
        result: Bits,
        gs_22749: bool,
        tile: i128,
        esize: i128,
        vertical: bool,
        slice_name: i128,
        width: i128,
    }
    let fn_state = FunctionState {
        tile,
        esize,
        vertical,
        slice_name,
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
        // D s_0_1: write-var widthshadow#394 <= s_0_0
        fn_state.widthshadow_394 = s_0_0;
        // D s_0_2: read-var esize:i
        let s_0_2: i128 = fn_state.esize;
        // D s_0_3: write-var esizeshadow#395 <= s_0_2
        fn_state.esizeshadow_395 = s_0_2;
        // D s_0_4: read-var vertical:u8
        let s_0_4: bool = fn_state.vertical;
        // N s_0_5: branch s_0_4 b3 b1
        if s_0_4 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_1_0: read-var widthshadow#394:i
        let s_1_0: i128 = fn_state.widthshadow_394;
        // D s_1_1: read-var tile:i
        let s_1_1: i128 = fn_state.tile;
        // D s_1_2: read-var esizeshadow#395:i
        let s_1_2: i128 = fn_state.esizeshadow_395;
        // D s_1_3: read-var slice_name:i
        let s_1_3: i128 = fn_state.slice_name;
        // D s_1_4: call ZAhslice_read(s_1_1, s_1_2, s_1_3, s_1_0)
        let s_1_4: Bits = ZAhslice_read(state, tracer, s_1_1, s_1_2, s_1_3, s_1_0);
        // D s_1_5: write-var result <= s_1_4
        fn_state.result = s_1_4;
        // N s_1_6: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_2_0: read-var result:bv
        let s_2_0: Bits = fn_state.result;
        // N s_2_1: return s_2_0
        return s_2_0;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_3_0: read-var widthshadow#394:i
        let s_3_0: i128 = fn_state.widthshadow_394;
        // D s_3_1: call __id(s_3_0)
        let s_3_1: i128 = u__id(state, tracer, s_3_0);
        // C s_3_2: const #0s : i
        let s_3_2: i128 = 0;
        // D s_3_3: cmp-ge s_3_1 s_3_2
        let s_3_3: bool = ((s_3_1) >= (s_3_2));
        // N s_3_4: assert s_3_3
        let s_3_4: () = assert!(s_3_3);
        // D s_3_5: read-var esizeshadow#395:i
        let s_3_5: i128 = fn_state.esizeshadow_395;
        // D s_3_6: call __id(s_3_5)
        let s_3_6: i128 = u__id(state, tracer, s_3_5);
        // C s_3_7: const #0s : i
        let s_3_7: i128 = 0;
        // D s_3_8: cmp-ge s_3_6 s_3_7
        let s_3_8: bool = ((s_3_6) >= (s_3_7));
        // N s_3_9: assert s_3_8
        let s_3_9: () = assert!(s_3_8);
        // D s_3_10: read-var slice_name:i
        let s_3_10: i128 = fn_state.slice_name;
        // D s_3_11: call __id(s_3_10)
        let s_3_11: i128 = u__id(state, tracer, s_3_10);
        // D s_3_12: read-var esizeshadow#395:i
        let s_3_12: i128 = fn_state.esizeshadow_395;
        // D s_3_13: call __id(s_3_12)
        let s_3_13: i128 = u__id(state, tracer, s_3_12);
        // D s_3_14: mul s_3_11 s_3_13
        let s_3_14: i128 = ((s_3_11) * (s_3_13));
        // D s_3_15: read-var slice_name:i
        let s_3_15: i128 = fn_state.slice_name;
        // D s_3_16: call __id(s_3_15)
        let s_3_16: i128 = u__id(state, tracer, s_3_15);
        // D s_3_17: read-var esizeshadow#395:i
        let s_3_17: i128 = fn_state.esizeshadow_395;
        // D s_3_18: call __id(s_3_17)
        let s_3_18: i128 = u__id(state, tracer, s_3_17);
        // D s_3_19: mul s_3_16 s_3_18
        let s_3_19: i128 = ((s_3_16) * (s_3_18));
        // D s_3_20: read-var esizeshadow#395:i
        let s_3_20: i128 = fn_state.esizeshadow_395;
        // D s_3_21: call __id(s_3_20)
        let s_3_21: i128 = u__id(state, tracer, s_3_20);
        // D s_3_22: add s_3_19 s_3_21
        let s_3_22: i128 = (s_3_19 + s_3_21);
        // C s_3_23: const #1s : i
        let s_3_23: i128 = 1;
        // D s_3_24: sub s_3_22 s_3_23
        let s_3_24: i128 = ((s_3_22) - (s_3_23));
        // D s_3_25: cmp-le s_3_14 s_3_24
        let s_3_25: bool = ((s_3_14) <= (s_3_24));
        // N s_3_26: branch s_3_25 b9 b4
        if s_3_25 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_4_0: read-var slice_name:i
        let s_4_0: i128 = fn_state.slice_name;
        // D s_4_1: call __id(s_4_0)
        let s_4_1: i128 = u__id(state, tracer, s_4_0);
        // C s_4_2: const #0s : i
        let s_4_2: i128 = 0;
        // D s_4_3: cmp-ge s_4_1 s_4_2
        let s_4_3: bool = ((s_4_1) >= (s_4_2));
        // N s_4_4: branch s_4_3 b8 b5
        if s_4_3 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#22748 <= s_5_0
        fn_state.gs_22748 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_6_0: read-var gs#22748:u8
        let s_6_0: bool = fn_state.gs_22748;
        // D s_6_1: not s_6_0
        let s_6_1: bool = !s_6_0;
        // D s_6_2: write-var gs#22749 <= s_6_1
        fn_state.gs_22749 = s_6_1;
        // N s_6_3: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_7_0: read-var gs#22749:u8
        let s_7_0: bool = fn_state.gs_22749;
        // N s_7_1: assert s_7_0
        let s_7_1: () = assert!(s_7_0);
        // D s_7_2: read-var esizeshadow#395:i
        let s_7_2: i128 = fn_state.esizeshadow_395;
        // D s_7_3: read-var widthshadow#394:i
        let s_7_3: i128 = fn_state.widthshadow_394;
        // D s_7_4: read-var tile:i
        let s_7_4: i128 = fn_state.tile;
        // D s_7_5: read-var slice_name:i
        let s_7_5: i128 = fn_state.slice_name;
        // D s_7_6: call ZAvslice_read(s_7_4, s_7_2, s_7_5, s_7_3)
        let s_7_6: Bits = ZAvslice_read(state, tracer, s_7_4, s_7_2, s_7_5, s_7_3);
        // D s_7_7: write-var result <= s_7_6
        fn_state.result = s_7_6;
        // N s_7_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_8_0: read-var slice_name:i
        let s_8_0: i128 = fn_state.slice_name;
        // D s_8_1: call __id(s_8_0)
        let s_8_1: i128 = u__id(state, tracer, s_8_0);
        // C s_8_2: const #1s : i
        let s_8_2: i128 = 1;
        // D s_8_3: add s_8_1 s_8_2
        let s_8_3: i128 = (s_8_1 + s_8_2);
        // D s_8_4: read-var esizeshadow#395:i
        let s_8_4: i128 = fn_state.esizeshadow_395;
        // D s_8_5: call __id(s_8_4)
        let s_8_5: i128 = u__id(state, tracer, s_8_4);
        // D s_8_6: mul s_8_3 s_8_5
        let s_8_6: i128 = ((s_8_3) * (s_8_5));
        // D s_8_7: read-var widthshadow#394:i
        let s_8_7: i128 = fn_state.widthshadow_394;
        // D s_8_8: call __id(s_8_7)
        let s_8_8: i128 = u__id(state, tracer, s_8_7);
        // D s_8_9: cmp-le s_8_6 s_8_8
        let s_8_9: bool = ((s_8_6) <= (s_8_8));
        // D s_8_10: write-var gs#22748 <= s_8_9
        fn_state.gs_22748 = s_8_9;
        // N s_8_11: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_9_0: const #1u : u8
        let s_9_0: bool = true;
        // D s_9_1: write-var gs#22749 <= s_9_0
        fn_state.gs_22749 = s_9_0;
        // N s_9_2: jump b7
        return block_7(state, tracer, fn_state);
    }
}
