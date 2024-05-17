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
use subrange_subrange_concat::*;
use common::*;
pub fn AArch32_SDStageOA<T: Tracer>(
    state: &mut State,
    tracer: &T,
    baseaddress: ProductTypeda0231e9dc169f81,
    va: u32,
    sdftype: u32,
) -> ProductTypeda0231e9dc169f81 {
    #[derive(Default)]
    struct FunctionState {
        tsize: i64,
        oa: ProductTypeda0231e9dc169f81,
        baseaddress: ProductTypeda0231e9dc169f81,
        va: u32,
        sdftype: u32,
    }
    let fn_state = FunctionState {
        baseaddress,
        va,
        sdftype,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeda0231e9dc169f81 {
        // C s_0_0: const #12s : i64
        let s_0_0: i64 = 12;
        // D s_0_1: write-var tsize <= s_0_0
        fn_state.tsize = s_0_0;
        // C s_0_2: const #5u : u32
        let s_0_2: u32 = 5;
        // D s_0_3: read-var sdftype:u32
        let s_0_3: u32 = fn_state.sdftype;
        // D s_0_4: cmp-eq s_0_2 s_0_3
        let s_0_4: bool = ((s_0_2) == (s_0_3));
        // D s_0_5: not s_0_4
        let s_0_5: bool = !s_0_4;
        // N s_0_6: branch s_0_5 b3 b1
        if s_0_5 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeda0231e9dc169f81 {
        // C s_1_0: const #12s : i64
        let s_1_0: i64 = 12;
        // D s_1_1: write-var tsize <= s_1_0
        fn_state.tsize = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeda0231e9dc169f81 {
        // D s_2_0: read-var tsize:i64
        let s_2_0: i64 = fn_state.tsize;
        // D s_2_1: read-var baseaddress.0:struct
        let s_2_1: u64 = fn_state.baseaddress._0;
        // C s_2_2: const #1s : i
        let s_2_2: i128 = 1;
        // D s_2_3: cast zx s_2_0 -> i
        let s_2_3: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_4: sub s_2_3 s_2_2
        let s_2_4: i128 = ((s_2_3) - (s_2_2));
        // D s_2_5: cast reint s_2_4 -> i64
        let s_2_5: i64 = (s_2_4 as i64);
        // C s_2_6: const #56s : i
        let s_2_6: i128 = 56;
        // C s_2_7: const #55s : i
        let s_2_7: i128 = 55;
        // C s_2_8: const #0s : i
        let s_2_8: i128 = 0;
        // D s_2_9: cast zx s_2_1 -> bv
        let s_2_9: Bits = Bits::new(s_2_1 as u128, 56u16);
        // D s_2_10: cast zx s_2_0 -> i
        let s_2_10: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_11: read-var va:u32
        let s_2_11: u32 = fn_state.va;
        // D s_2_12: cast zx s_2_11 -> bv
        let s_2_12: Bits = Bits::new(s_2_11 as u128, 32u16);
        // D s_2_13: cast zx s_2_5 -> i
        let s_2_13: i128 = (i128::try_from(s_2_5).unwrap());
        // D s_2_14: call subrange_subrange_concat(s_2_6, s_2_9, s_2_7, s_2_10, s_2_12, s_2_13, s_2_8)
        let s_2_14: Bits = subrange_subrange_concat(
            state,
            tracer,
            s_2_6,
            s_2_9,
            s_2_7,
            s_2_10,
            s_2_12,
            s_2_13,
            s_2_8,
        );
        // D s_2_15: cast reint s_2_14 -> u56
        let s_2_15: u64 = (s_2_14.value() as u64);
        // D s_2_16: write-var oa.0 <= s_2_15
        fn_state.oa._0 = s_2_15;
        // D s_2_17: read-var baseaddress.1:struct
        let s_2_17: u32 = fn_state.baseaddress._1;
        // D s_2_18: write-var oa.1 <= s_2_17
        fn_state.oa._1 = s_2_17;
        // D s_2_19: read-var oa:struct
        let s_2_19: ProductTypeda0231e9dc169f81 = fn_state.oa;
        // N s_2_20: return s_2_19
        return s_2_19;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeda0231e9dc169f81 {
        // C s_3_0: const #4u : u32
        let s_3_0: u32 = 4;
        // D s_3_1: read-var sdftype:u32
        let s_3_1: u32 = fn_state.sdftype;
        // D s_3_2: cmp-eq s_3_0 s_3_1
        let s_3_2: bool = ((s_3_0) == (s_3_1));
        // D s_3_3: not s_3_2
        let s_3_3: bool = !s_3_2;
        // N s_3_4: branch s_3_3 b5 b4
        if s_3_3 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeda0231e9dc169f81 {
        // C s_4_0: const #16s : i64
        let s_4_0: i64 = 16;
        // D s_4_1: write-var tsize <= s_4_0
        fn_state.tsize = s_4_0;
        // N s_4_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeda0231e9dc169f81 {
        // C s_5_0: const #3u : u32
        let s_5_0: u32 = 3;
        // D s_5_1: read-var sdftype:u32
        let s_5_1: u32 = fn_state.sdftype;
        // D s_5_2: cmp-eq s_5_0 s_5_1
        let s_5_2: bool = ((s_5_0) == (s_5_1));
        // D s_5_3: not s_5_2
        let s_5_3: bool = !s_5_2;
        // N s_5_4: branch s_5_3 b7 b6
        if s_5_3 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeda0231e9dc169f81 {
        // C s_6_0: const #20s : i64
        let s_6_0: i64 = 20;
        // D s_6_1: write-var tsize <= s_6_0
        fn_state.tsize = s_6_0;
        // N s_6_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeda0231e9dc169f81 {
        // C s_7_0: const #2u : u32
        let s_7_0: u32 = 2;
        // D s_7_1: read-var sdftype:u32
        let s_7_1: u32 = fn_state.sdftype;
        // D s_7_2: cmp-eq s_7_0 s_7_1
        let s_7_2: bool = ((s_7_0) == (s_7_1));
        // D s_7_3: not s_7_2
        let s_7_3: bool = !s_7_2;
        // N s_7_4: branch s_7_3 b9 b8
        if s_7_3 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeda0231e9dc169f81 {
        // C s_8_0: const #24s : i64
        let s_8_0: i64 = 24;
        // D s_8_1: write-var tsize <= s_8_0
        fn_state.tsize = s_8_0;
        // N s_8_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeda0231e9dc169f81 {
        // N s_9_0: jump b2
        return block_2(state, tracer, fn_state);
    }
}
