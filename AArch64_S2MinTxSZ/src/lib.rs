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
use AArch64_PAMax::*;
use Have52BitPAExt::*;
use common::*;
pub fn AArch64_S2MinTxSZ<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d128: bool,
    ds: bool,
    tgx: u32,
    s1aarch64: bool,
) -> i128 {
    #[derive(Default)]
    struct FunctionState {
        gs_18990: bool,
        ips: i128,
        gs_18991: bool,
        min_txsz: i128,
        d128: bool,
        ds: bool,
        tgx: u32,
        s1aarch64: bool,
    }
    let fn_state = FunctionState {
        d128,
        ds,
        tgx,
        s1aarch64,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call AArch64_PAMax(s_0_0)
        let s_0_1: i64 = AArch64_PAMax(state, tracer, s_0_0);
        // S s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (i128::try_from(s_0_1).unwrap());
        // D s_0_3: write-var ips <= s_0_2
        fn_state.ips = s_0_2;
        // D s_0_4: read-var d128:u8
        let s_0_4: bool = fn_state.d128;
        // D s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 1u16);
        // C s_0_6: const #0u : u8
        let s_0_6: bool = false;
        // C s_0_7: cast zx s_0_6 -> bv
        let s_0_7: Bits = Bits::new(s_0_6 as u128, 1u16);
        // D s_0_8: cmp-eq s_0_5 s_0_7
        let s_0_8: bool = ((s_0_5) == (s_0_7));
        // N s_0_9: branch s_0_8 b6 b1
        if s_0_8 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // N s_1_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_2_0: read-var ips:i
        let s_2_0: i128 = fn_state.ips;
        // C s_2_1: const #64s : i
        let s_2_1: i128 = 64;
        // D s_2_2: sub s_2_1 s_2_0
        let s_2_2: i128 = ((s_2_1) - (s_2_0));
        // D s_2_3: write-var min_txsz <= s_2_2
        fn_state.min_txsz = s_2_2;
        // D s_2_4: read-var s1aarch64:u8
        let s_2_4: bool = fn_state.s1aarch64;
        // D s_2_5: not s_2_4
        let s_2_5: bool = !s_2_4;
        // N s_2_6: branch s_2_5 b5 b3
        if s_2_5 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // N s_3_0: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_4_0: read-var min_txsz:i
        let s_4_0: i128 = fn_state.min_txsz;
        // N s_4_1: return s_4_0
        return s_4_0;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_5_0: const #24s : i
        let s_5_0: i128 = 24;
        // D s_5_1: read-var min_txsz:i
        let s_5_1: i128 = fn_state.min_txsz;
        // D s_5_2: cmp-lt s_5_1 s_5_0
        let s_5_2: bool = ((s_5_1) < (s_5_0));
        // D s_5_3: select s_5_2 s_5_1 s_5_0
        let s_5_3: i128 = if s_5_2 { s_5_1 } else { s_5_0 };
        // D s_5_4: write-var min_txsz <= s_5_3
        fn_state.min_txsz = s_5_3;
        // N s_5_5: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_6_0: const #() : ()
        let s_6_0: () = ();
        // S s_6_1: call Have52BitPAExt(s_6_0)
        let s_6_1: bool = Have52BitPAExt(state, tracer, s_6_0);
        // N s_6_2: branch s_6_1 b14 b7
        if s_6_1 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var gs#18990 <= s_7_0
        fn_state.gs_18990 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_8_0: read-var gs#18990:u8
        let s_8_0: bool = fn_state.gs_18990;
        // N s_8_1: branch s_8_0 b13 b9
        if s_8_0 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#18991 <= s_9_0
        fn_state.gs_18991 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_10_0: read-var gs#18991:u8
        let s_10_0: bool = fn_state.gs_18991;
        // N s_10_1: branch s_10_0 b12 b11
        if s_10_0 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call AArch64_PAMax(s_11_0)
        let s_11_1: i64 = AArch64_PAMax(state, tracer, s_11_0);
        // C s_11_2: const #52s : i
        let s_11_2: i128 = 52;
        // S s_11_3: cast zx s_11_1 -> i
        let s_11_3: i128 = (i128::try_from(s_11_1).unwrap());
        // S s_11_4: cmp-lt s_11_2 s_11_3
        let s_11_4: bool = ((s_11_2) < (s_11_3));
        // D s_11_5: select s_11_4 s_11_2 s_11_3
        let s_11_5: i128 = if s_11_4 { s_11_2 } else { s_11_3 };
        // D s_11_6: write-var ips <= s_11_5
        fn_state.ips = s_11_5;
        // N s_11_7: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_12_0: const #() : ()
        let s_12_0: () = ();
        // S s_12_1: call AArch64_PAMax(s_12_0)
        let s_12_1: i64 = AArch64_PAMax(state, tracer, s_12_0);
        // C s_12_2: const #48s : i
        let s_12_2: i128 = 48;
        // S s_12_3: cast zx s_12_1 -> i
        let s_12_3: i128 = (i128::try_from(s_12_1).unwrap());
        // S s_12_4: cmp-lt s_12_2 s_12_3
        let s_12_4: bool = ((s_12_2) < (s_12_3));
        // D s_12_5: select s_12_4 s_12_2 s_12_3
        let s_12_5: i128 = if s_12_4 { s_12_2 } else { s_12_3 };
        // D s_12_6: write-var ips <= s_12_5
        fn_state.ips = s_12_5;
        // N s_12_7: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_13_0: read-var ds:u8
        let s_13_0: bool = fn_state.ds;
        // D s_13_1: cast zx s_13_0 -> bv
        let s_13_1: Bits = Bits::new(s_13_0 as u128, 1u16);
        // C s_13_2: const #0u : u8
        let s_13_2: bool = false;
        // C s_13_3: cast zx s_13_2 -> bv
        let s_13_3: Bits = Bits::new(s_13_2 as u128, 1u16);
        // D s_13_4: cmp-eq s_13_1 s_13_3
        let s_13_4: bool = ((s_13_1) == (s_13_3));
        // D s_13_5: write-var gs#18991 <= s_13_4
        fn_state.gs_18991 = s_13_4;
        // N s_13_6: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_14_0: read-var tgx:u32
        let s_14_0: u32 = fn_state.tgx;
        // C s_14_1: const #2u : u32
        let s_14_1: u32 = 2;
        // D s_14_2: cmp-eq s_14_0 s_14_1
        let s_14_2: bool = ((s_14_0) == (s_14_1));
        // D s_14_3: write-var gs#18990 <= s_14_2
        fn_state.gs_18990 = s_14_2;
        // N s_14_4: jump b8
        return block_8(state, tracer, fn_state);
    }
}
