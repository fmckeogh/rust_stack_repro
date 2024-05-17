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
use Have56BitPAExt::*;
use Have52BitIPAAndPASpaceExt::*;
use Have52BitPAExt::*;
use common::*;
pub fn AArch64_PhysicalAddressSize<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d128: bool,
    encoded_ps: u8,
    tgx: u32,
) -> i128 {
    #[derive(Default)]
    struct FunctionState {
        gs_17655: bool,
        max_ps: i128,
        gs_17657: bool,
        psshadow_297: i64,
        ps: i64,
        d128: bool,
        encoded_ps: u8,
        tgx: u32,
    }
    let fn_state = FunctionState {
        d128,
        encoded_ps,
        tgx,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_0_0: const #32s : i64
        let s_0_0: i64 = 32;
        // D s_0_1: write-var ps <= s_0_0
        fn_state.ps = s_0_0;
        // D s_0_2: read-var encoded_ps:u8
        let s_0_2: u8 = fn_state.encoded_ps;
        // D s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 3u16);
        // C s_0_4: const #0u : u8
        let s_0_4: u8 = 0;
        // C s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 3u16);
        // D s_0_6: cmp-eq s_0_3 s_0_5
        let s_0_6: bool = ((s_0_3) == (s_0_5));
        // D s_0_7: not s_0_6
        let s_0_7: bool = !s_0_6;
        // N s_0_8: branch s_0_7 b16 b1
        if s_0_7 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_1_0: const #32s : i64
        let s_1_0: i64 = 32;
        // D s_1_1: write-var ps <= s_1_0
        fn_state.ps = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_2_0: read-var ps:i64
        let s_2_0: i64 = fn_state.ps;
        // D s_2_1: write-var psshadow#297 <= s_2_0
        fn_state.psshadow_297 = s_2_0;
        // C s_2_2: const #() : ()
        let s_2_2: () = ();
        // S s_2_3: call Have56BitPAExt(s_2_2)
        let s_2_3: bool = Have56BitPAExt(state, tracer, s_2_2);
        // S s_2_4: not s_2_3
        let s_2_4: bool = !s_2_3;
        // N s_2_5: branch s_2_4 b15 b3
        if s_2_4 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_3_0: read-var d128:u8
        let s_3_0: bool = fn_state.d128;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 1u16);
        // C s_3_2: const #0u : u8
        let s_3_2: bool = false;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 1u16);
        // D s_3_4: cmp-eq s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) == (s_3_3));
        // D s_3_5: write-var gs#17655 <= s_3_4
        fn_state.gs_17655 = s_3_4;
        // N s_3_6: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_4_0: read-var gs#17655:u8
        let s_4_0: bool = fn_state.gs_17655;
        // N s_4_1: branch s_4_0 b7 b5
        if s_4_0 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_5_0: const #() : ()
        let s_5_0: () = ();
        // S s_5_1: call AArch64_PAMax(s_5_0)
        let s_5_1: i64 = AArch64_PAMax(state, tracer, s_5_0);
        // S s_5_2: cast zx s_5_1 -> i
        let s_5_2: i128 = (i128::try_from(s_5_1).unwrap());
        // D s_5_3: write-var max_ps <= s_5_2
        fn_state.max_ps = s_5_2;
        // N s_5_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_6_0: read-var max_ps:i
        let s_6_0: i128 = fn_state.max_ps;
        // D s_6_1: read-var psshadow#297:i64
        let s_6_1: i64 = fn_state.psshadow_297;
        // D s_6_2: cast zx s_6_1 -> i
        let s_6_2: i128 = (i128::try_from(s_6_1).unwrap());
        // D s_6_3: cmp-lt s_6_2 s_6_0
        let s_6_3: bool = ((s_6_2) < (s_6_0));
        // D s_6_4: select s_6_3 s_6_2 s_6_0
        let s_6_4: i128 = if s_6_3 { s_6_2 } else { s_6_0 };
        // N s_6_5: return s_6_4
        return s_6_4;
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_7_0: read-var tgx:u32
        let s_7_0: u32 = fn_state.tgx;
        // C s_7_1: const #2u : u32
        let s_7_1: u32 = 2;
        // D s_7_2: cmp-eq s_7_0 s_7_1
        let s_7_2: bool = ((s_7_0) == (s_7_1));
        // N s_7_3: branch s_7_2 b14 b8
        if s_7_2 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var gs#17657 <= s_8_0
        fn_state.gs_17657 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_9_0: read-var gs#17657:u8
        let s_9_0: bool = fn_state.gs_17657;
        // N s_9_1: branch s_9_0 b13 b10
        if s_9_0 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call Have52BitPAExt(s_10_0)
        let s_10_1: bool = Have52BitPAExt(state, tracer, s_10_0);
        // S s_10_2: not s_10_1
        let s_10_2: bool = !s_10_1;
        // N s_10_3: branch s_10_2 b12 b11
        if s_10_2 {
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
        // D s_11_6: write-var max_ps <= s_11_5
        fn_state.max_ps = s_11_5;
        // N s_11_7: jump b6
        return block_6(state, tracer, fn_state);
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
        // D s_12_6: write-var max_ps <= s_12_5
        fn_state.max_ps = s_12_5;
        // N s_12_7: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_13_0: const #() : ()
        let s_13_0: () = ();
        // S s_13_1: call AArch64_PAMax(s_13_0)
        let s_13_1: i64 = AArch64_PAMax(state, tracer, s_13_0);
        // C s_13_2: const #48s : i
        let s_13_2: i128 = 48;
        // S s_13_3: cast zx s_13_1 -> i
        let s_13_3: i128 = (i128::try_from(s_13_1).unwrap());
        // S s_13_4: cmp-lt s_13_2 s_13_3
        let s_13_4: bool = ((s_13_2) < (s_13_3));
        // D s_13_5: select s_13_4 s_13_2 s_13_3
        let s_13_5: i128 = if s_13_4 { s_13_2 } else { s_13_3 };
        // D s_13_6: write-var max_ps <= s_13_5
        fn_state.max_ps = s_13_5;
        // N s_13_7: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_14_0: const #() : ()
        let s_14_0: () = ();
        // S s_14_1: call Have52BitIPAAndPASpaceExt(s_14_0)
        let s_14_1: bool = Have52BitIPAAndPASpaceExt(state, tracer, s_14_0);
        // S s_14_2: not s_14_1
        let s_14_2: bool = !s_14_1;
        // D s_14_3: write-var gs#17657 <= s_14_2
        fn_state.gs_17657 = s_14_2;
        // N s_14_4: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_15_0: const #1u : u8
        let s_15_0: bool = true;
        // D s_15_1: write-var gs#17655 <= s_15_0
        fn_state.gs_17655 = s_15_0;
        // N s_15_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_16_0: read-var encoded_ps:u8
        let s_16_0: u8 = fn_state.encoded_ps;
        // D s_16_1: cast zx s_16_0 -> bv
        let s_16_1: Bits = Bits::new(s_16_0 as u128, 3u16);
        // C s_16_2: const #1u : u8
        let s_16_2: u8 = 1;
        // C s_16_3: cast zx s_16_2 -> bv
        let s_16_3: Bits = Bits::new(s_16_2 as u128, 3u16);
        // D s_16_4: cmp-eq s_16_1 s_16_3
        let s_16_4: bool = ((s_16_1) == (s_16_3));
        // D s_16_5: not s_16_4
        let s_16_5: bool = !s_16_4;
        // N s_16_6: branch s_16_5 b18 b17
        if s_16_5 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_17_0: const #36s : i64
        let s_17_0: i64 = 36;
        // D s_17_1: write-var ps <= s_17_0
        fn_state.ps = s_17_0;
        // N s_17_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_18_0: read-var encoded_ps:u8
        let s_18_0: u8 = fn_state.encoded_ps;
        // D s_18_1: cast zx s_18_0 -> bv
        let s_18_1: Bits = Bits::new(s_18_0 as u128, 3u16);
        // C s_18_2: const #2u : u8
        let s_18_2: u8 = 2;
        // C s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 3u16);
        // D s_18_4: cmp-eq s_18_1 s_18_3
        let s_18_4: bool = ((s_18_1) == (s_18_3));
        // D s_18_5: not s_18_4
        let s_18_5: bool = !s_18_4;
        // N s_18_6: branch s_18_5 b20 b19
        if s_18_5 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_19_0: const #40s : i64
        let s_19_0: i64 = 40;
        // D s_19_1: write-var ps <= s_19_0
        fn_state.ps = s_19_0;
        // N s_19_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_20_0: read-var encoded_ps:u8
        let s_20_0: u8 = fn_state.encoded_ps;
        // D s_20_1: cast zx s_20_0 -> bv
        let s_20_1: Bits = Bits::new(s_20_0 as u128, 3u16);
        // C s_20_2: const #3u : u8
        let s_20_2: u8 = 3;
        // C s_20_3: cast zx s_20_2 -> bv
        let s_20_3: Bits = Bits::new(s_20_2 as u128, 3u16);
        // D s_20_4: cmp-eq s_20_1 s_20_3
        let s_20_4: bool = ((s_20_1) == (s_20_3));
        // D s_20_5: not s_20_4
        let s_20_5: bool = !s_20_4;
        // N s_20_6: branch s_20_5 b22 b21
        if s_20_5 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_21_0: const #42s : i64
        let s_21_0: i64 = 42;
        // D s_21_1: write-var ps <= s_21_0
        fn_state.ps = s_21_0;
        // N s_21_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_22_0: read-var encoded_ps:u8
        let s_22_0: u8 = fn_state.encoded_ps;
        // D s_22_1: cast zx s_22_0 -> bv
        let s_22_1: Bits = Bits::new(s_22_0 as u128, 3u16);
        // C s_22_2: const #4u : u8
        let s_22_2: u8 = 4;
        // C s_22_3: cast zx s_22_2 -> bv
        let s_22_3: Bits = Bits::new(s_22_2 as u128, 3u16);
        // D s_22_4: cmp-eq s_22_1 s_22_3
        let s_22_4: bool = ((s_22_1) == (s_22_3));
        // D s_22_5: not s_22_4
        let s_22_5: bool = !s_22_4;
        // N s_22_6: branch s_22_5 b24 b23
        if s_22_5 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_23_0: const #44s : i64
        let s_23_0: i64 = 44;
        // D s_23_1: write-var ps <= s_23_0
        fn_state.ps = s_23_0;
        // N s_23_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_24_0: read-var encoded_ps:u8
        let s_24_0: u8 = fn_state.encoded_ps;
        // D s_24_1: cast zx s_24_0 -> bv
        let s_24_1: Bits = Bits::new(s_24_0 as u128, 3u16);
        // C s_24_2: const #5u : u8
        let s_24_2: u8 = 5;
        // C s_24_3: cast zx s_24_2 -> bv
        let s_24_3: Bits = Bits::new(s_24_2 as u128, 3u16);
        // D s_24_4: cmp-eq s_24_1 s_24_3
        let s_24_4: bool = ((s_24_1) == (s_24_3));
        // D s_24_5: not s_24_4
        let s_24_5: bool = !s_24_4;
        // N s_24_6: branch s_24_5 b26 b25
        if s_24_5 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_25_0: const #48s : i64
        let s_25_0: i64 = 48;
        // D s_25_1: write-var ps <= s_25_0
        fn_state.ps = s_25_0;
        // N s_25_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_26_0: read-var encoded_ps:u8
        let s_26_0: u8 = fn_state.encoded_ps;
        // D s_26_1: cast zx s_26_0 -> bv
        let s_26_1: Bits = Bits::new(s_26_0 as u128, 3u16);
        // C s_26_2: const #6u : u8
        let s_26_2: u8 = 6;
        // C s_26_3: cast zx s_26_2 -> bv
        let s_26_3: Bits = Bits::new(s_26_2 as u128, 3u16);
        // D s_26_4: cmp-eq s_26_1 s_26_3
        let s_26_4: bool = ((s_26_1) == (s_26_3));
        // D s_26_5: not s_26_4
        let s_26_5: bool = !s_26_4;
        // N s_26_6: branch s_26_5 b28 b27
        if s_26_5 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_27_0: const #52s : i64
        let s_27_0: i64 = 52;
        // D s_27_1: write-var ps <= s_27_0
        fn_state.ps = s_27_0;
        // N s_27_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_28_0: const #56s : i64
        let s_28_0: i64 = 56;
        // D s_28_1: write-var ps <= s_28_0
        fn_state.ps = s_28_0;
        // N s_28_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
