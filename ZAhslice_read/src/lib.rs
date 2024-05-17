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
use ZAvector_read::*;
use CurrentSVL_read::*;
use common::*;
pub fn ZAhslice_read<T: Tracer>(
    state: &mut State,
    tracer: &T,
    tile: i128,
    esize: i128,
    slice_name: i128,
    width: i128,
) -> Bits {
    #[derive(Default)]
    struct FunctionState {
        gs_22673: bool,
        gs_22668: bool,
        gs_22667: bool,
        gs_22665: bool,
        gs_22677: bool,
        slices: i64,
        widthshadow_388: i128,
        tiles: i64,
        gs_22666: bool,
        SVL: i64,
        tile: i128,
        esize: i128,
        slice_name: i128,
        width: i128,
    }
    let fn_state = FunctionState {
        tile,
        esize,
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
        // D s_0_1: write-var widthshadow#388 <= s_0_0
        fn_state.widthshadow_388 = s_0_0;
        // C s_0_2: const #() : ()
        let s_0_2: () = ();
        // S s_0_3: call CurrentSVL_read(s_0_2)
        let s_0_3: i64 = CurrentSVL_read(state, tracer, s_0_2);
        // D s_0_4: write-var SVL <= s_0_3
        fn_state.SVL = s_0_3;
        // C s_0_5: const #8s : i
        let s_0_5: i128 = 8;
        // D s_0_6: read-var esize:i
        let s_0_6: i128 = fn_state.esize;
        // D s_0_7: cmp-eq s_0_6 s_0_5
        let s_0_7: bool = ((s_0_6) == (s_0_5));
        // N s_0_8: branch s_0_7 b18 b1
        if s_0_7 {
            return block_18(state, tracer, fn_state);
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
        // N s_1_3: branch s_1_2 b17 b2
        if s_1_2 {
            return block_17(state, tracer, fn_state);
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
        // N s_2_3: branch s_2_2 b16 b3
        if s_2_2 {
            return block_16(state, tracer, fn_state);
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
        // N s_3_3: branch s_3_2 b15 b4
        if s_3_2 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_4_0: const #128s : i
        let s_4_0: i128 = 128;
        // D s_4_1: read-var esize:i
        let s_4_1: i128 = fn_state.esize;
        // D s_4_2: cmp-eq s_4_1 s_4_0
        let s_4_2: bool = ((s_4_1) == (s_4_0));
        // D s_4_3: write-var gs#22665 <= s_4_2
        fn_state.gs_22665 = s_4_2;
        // N s_4_4: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_5_0: read-var gs#22665:u8
        let s_5_0: bool = fn_state.gs_22665;
        // D s_5_1: write-var gs#22666 <= s_5_0
        fn_state.gs_22666 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_6_0: read-var gs#22666:u8
        let s_6_0: bool = fn_state.gs_22666;
        // D s_6_1: write-var gs#22667 <= s_6_0
        fn_state.gs_22667 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_7_0: read-var gs#22667:u8
        let s_7_0: bool = fn_state.gs_22667;
        // D s_7_1: write-var gs#22668 <= s_7_0
        fn_state.gs_22668 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_8_0: read-var gs#22668:u8
        let s_8_0: bool = fn_state.gs_22668;
        // N s_8_1: assert s_8_0
        let s_8_1: () = assert!(s_8_0);
        // C s_8_2: const #8s : i
        let s_8_2: i128 = 8;
        // D s_8_3: read-var esize:i
        let s_8_3: i128 = fn_state.esize;
        // D s_8_4: div s_8_3 s_8_2
        let s_8_4: i128 = ((s_8_3) / (s_8_2));
        // D s_8_5: cast reint s_8_4 -> i64
        let s_8_5: i64 = (s_8_4 as i64);
        // D s_8_6: write-var tiles <= s_8_5
        fn_state.tiles = s_8_5;
        // C s_8_7: const #0s : i
        let s_8_7: i128 = 0;
        // D s_8_8: read-var tile:i
        let s_8_8: i128 = fn_state.tile;
        // D s_8_9: cmp-ge s_8_8 s_8_7
        let s_8_9: bool = ((s_8_8) >= (s_8_7));
        // N s_8_10: branch s_8_9 b14 b9
        if s_8_9 {
            return block_14(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#22673 <= s_9_0
        fn_state.gs_22673 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_10_0: read-var gs#22673:u8
        let s_10_0: bool = fn_state.gs_22673;
        // N s_10_1: assert s_10_0
        let s_10_1: () = assert!(s_10_0);
        // D s_10_2: read-var SVL:i64
        let s_10_2: i64 = fn_state.SVL;
        // D s_10_3: cast zx s_10_2 -> i
        let s_10_3: i128 = (i128::try_from(s_10_2).unwrap());
        // D s_10_4: read-var esize:i
        let s_10_4: i128 = fn_state.esize;
        // D s_10_5: div s_10_3 s_10_4
        let s_10_5: i128 = ((s_10_3) / (s_10_4));
        // D s_10_6: cast reint s_10_5 -> i64
        let s_10_6: i64 = (s_10_5 as i64);
        // D s_10_7: write-var slices <= s_10_6
        fn_state.slices = s_10_6;
        // C s_10_8: const #0s : i
        let s_10_8: i128 = 0;
        // D s_10_9: read-var slice_name:i
        let s_10_9: i128 = fn_state.slice_name;
        // D s_10_10: cmp-ge s_10_9 s_10_8
        let s_10_10: bool = ((s_10_9) >= (s_10_8));
        // N s_10_11: branch s_10_10 b13 b11
        if s_10_10 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // D s_11_1: write-var gs#22677 <= s_11_0
        fn_state.gs_22677 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_12_0: read-var gs#22677:u8
        let s_12_0: bool = fn_state.gs_22677;
        // N s_12_1: assert s_12_0
        let s_12_1: () = assert!(s_12_0);
        // D s_12_2: read-var tiles:i64
        let s_12_2: i64 = fn_state.tiles;
        // D s_12_3: cast zx s_12_2 -> i
        let s_12_3: i128 = (i128::try_from(s_12_2).unwrap());
        // D s_12_4: read-var slice_name:i
        let s_12_4: i128 = fn_state.slice_name;
        // D s_12_5: mul s_12_4 s_12_3
        let s_12_5: i128 = ((s_12_4) * (s_12_3));
        // D s_12_6: cast reint s_12_5 -> i64
        let s_12_6: i64 = (s_12_5 as i64);
        // D s_12_7: cast zx s_12_6 -> i
        let s_12_7: i128 = (i128::try_from(s_12_6).unwrap());
        // D s_12_8: read-var tile:i
        let s_12_8: i128 = fn_state.tile;
        // D s_12_9: add s_12_8 s_12_7
        let s_12_9: i128 = (s_12_8 + s_12_7);
        // D s_12_10: cast reint s_12_9 -> i64
        let s_12_10: i64 = (s_12_9 as i64);
        // D s_12_11: read-var widthshadow#388:i
        let s_12_11: i128 = fn_state.widthshadow_388;
        // D s_12_12: cast zx s_12_10 -> i
        let s_12_12: i128 = (i128::try_from(s_12_10).unwrap());
        // D s_12_13: tail-call ZAvector_read(s_12_12, s_12_11)
        return ZAvector_read(state, tracer, s_12_12, s_12_11);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_13_0: read-var slices:i64
        let s_13_0: i64 = fn_state.slices;
        // D s_13_1: cast zx s_13_0 -> i
        let s_13_1: i128 = (i128::try_from(s_13_0).unwrap());
        // D s_13_2: read-var slice_name:i
        let s_13_2: i128 = fn_state.slice_name;
        // D s_13_3: cmp-lt s_13_2 s_13_1
        let s_13_3: bool = ((s_13_2) < (s_13_1));
        // D s_13_4: write-var gs#22677 <= s_13_3
        fn_state.gs_22677 = s_13_3;
        // N s_13_5: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // D s_14_0: read-var tiles:i64
        let s_14_0: i64 = fn_state.tiles;
        // D s_14_1: cast zx s_14_0 -> i
        let s_14_1: i128 = (i128::try_from(s_14_0).unwrap());
        // D s_14_2: read-var tile:i
        let s_14_2: i128 = fn_state.tile;
        // D s_14_3: cmp-lt s_14_2 s_14_1
        let s_14_3: bool = ((s_14_2) < (s_14_1));
        // D s_14_4: write-var gs#22673 <= s_14_3
        fn_state.gs_22673 = s_14_3;
        // N s_14_5: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_15_0: const #1u : u8
        let s_15_0: bool = true;
        // D s_15_1: write-var gs#22665 <= s_15_0
        fn_state.gs_22665 = s_15_0;
        // N s_15_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_16_0: const #1u : u8
        let s_16_0: bool = true;
        // D s_16_1: write-var gs#22666 <= s_16_0
        fn_state.gs_22666 = s_16_0;
        // N s_16_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_17_0: const #1u : u8
        let s_17_0: bool = true;
        // D s_17_1: write-var gs#22667 <= s_17_0
        fn_state.gs_22667 = s_17_0;
        // N s_17_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> Bits {
        // C s_18_0: const #1u : u8
        let s_18_0: bool = true;
        // D s_18_1: write-var gs#22668 <= s_18_0
        fn_state.gs_22668 = s_18_0;
        // N s_18_2: jump b8
        return block_8(state, tracer, fn_state);
    }
}
