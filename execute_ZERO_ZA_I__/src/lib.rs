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
use FailTransaction::*;
use CheckSMEAndZAEnabled::*;
use Zeros::*;
use ZAtile_set::*;
use HaveTME::*;
use common::*;
pub fn execute_ZERO_ZA_I__<T: Tracer>(
    state: &mut State,
    tracer: &T,
    SVL: i64,
    dim_dim_esize: i64,
    esize: i64,
    mask: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        SVLshadow_5548: i64,
        result: Bits,
        i: i64,
        SVLshadow_5549: i64,
        dim_dim_esizeshadow_5547: i64,
        gs_258140: bool,
        SVL: i64,
        dim_dim_esize: i64,
        esize: i64,
        mask: u8,
    }
    let fn_state = FunctionState {
        SVL,
        dim_dim_esize,
        esize,
        mask,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var dim_dim_esize:i64
        let s_0_0: i64 = fn_state.dim_dim_esize;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var dim_dim_esizeshadow#5547 <= s_0_2
        fn_state.dim_dim_esizeshadow_5547 = s_0_2;
        // D s_0_4: read-var SVL:i64
        let s_0_4: i64 = fn_state.SVL;
        // D s_0_5: cast zx s_0_4 -> i
        let s_0_5: i128 = (i128::try_from(s_0_4).unwrap());
        // D s_0_6: cast reint s_0_5 -> i64
        let s_0_6: i64 = (s_0_5 as i64);
        // D s_0_7: write-var SVLshadow#5548 <= s_0_6
        fn_state.SVLshadow_5548 = s_0_6;
        // C s_0_8: const #() : ()
        let s_0_8: () = ();
        // S s_0_9: call CheckSMEAndZAEnabled(s_0_8)
        let s_0_9: () = CheckSMEAndZAEnabled(state, tracer, s_0_8);
        // N s_0_10: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var SVLshadow#5548:i64
        let s_1_0: i64 = fn_state.SVLshadow_5548;
        // D s_1_1: write-var SVLshadow#5549 <= s_1_0
        fn_state.SVLshadow_5549 = s_1_0;
        // D s_1_2: read-var dim_dim_esizeshadow#5547:i64
        let s_1_2: i64 = fn_state.dim_dim_esizeshadow_5547;
        // D s_1_3: cast zx s_1_2 -> i
        let s_1_3: i128 = (i128::try_from(s_1_2).unwrap());
        // D s_1_4: call Zeros(s_1_3)
        let s_1_4: Bits = Zeros(state, tracer, s_1_3);
        // D s_1_5: write-var result <= s_1_4
        fn_state.result = s_1_4;
        // C s_1_6: const #() : ()
        let s_1_6: () = ();
        // S s_1_7: call HaveTME(s_1_6)
        let s_1_7: bool = HaveTME(state, tracer, s_1_6);
        // N s_1_8: branch s_1_7 b13 b2
        if s_1_7 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #0u : u8
        let s_2_0: bool = false;
        // D s_2_1: write-var gs#258140 <= s_2_0
        fn_state.gs_258140 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var gs#258140:u8
        let s_3_0: bool = fn_state.gs_258140;
        // N s_3_1: branch s_3_0 b12 b4
        if s_3_0 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_4_0: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0s : i64
        let s_5_0: i64 = 0;
        // D s_5_1: write-var i <= s_5_0
        fn_state.i = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var i:i64
        let s_6_0: i64 = fn_state.i;
        // C s_6_1: const #7s : i64
        let s_6_1: i64 = 7;
        // D s_6_2: cmp-gt s_6_0 s_6_1
        let s_6_2: bool = ((s_6_0) > (s_6_1));
        // N s_6_3: branch s_6_2 b11 b7
        if s_6_2 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var mask:u8
        let s_7_0: u8 = fn_state.mask;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 8u16);
        // D s_7_2: read-var i:i64
        let s_7_2: i64 = fn_state.i;
        // D s_7_3: cast zx s_7_2 -> i
        let s_7_3: i128 = (i128::try_from(s_7_2).unwrap());
        // C s_7_4: const #1u : u64
        let s_7_4: u64 = 1;
        // D s_7_5: bit-extract s_7_1 s_7_3 s_7_4
        let s_7_5: Bits = (Bits::new(
            ((s_7_1) >> (s_7_3)).value(),
            u16::try_from(s_7_4).unwrap(),
        ));
        // D s_7_6: cast reint s_7_5 -> u8
        let s_7_6: bool = ((s_7_5.value()) != 0);
        // C s_7_7: const #0s : i
        let s_7_7: i128 = 0;
        // C s_7_8: const #0u : u64
        let s_7_8: u64 = 0;
        // D s_7_9: cast zx s_7_6 -> u64
        let s_7_9: u64 = (s_7_6 as u64);
        // C s_7_10: const #1u : u64
        let s_7_10: u64 = 1;
        // D s_7_11: and s_7_9 s_7_10
        let s_7_11: u64 = ((s_7_9) & (s_7_10));
        // D s_7_12: cmp-eq s_7_11 s_7_10
        let s_7_12: bool = ((s_7_11) == (s_7_10));
        // D s_7_13: lsl s_7_9 s_7_7
        let s_7_13: u64 = s_7_9 << s_7_7;
        // D s_7_14: or s_7_8 s_7_13
        let s_7_14: u64 = ((s_7_8) | (s_7_13));
        // D s_7_15: cmpl s_7_13
        let s_7_15: u64 = !s_7_13;
        // D s_7_16: and s_7_8 s_7_15
        let s_7_16: u64 = ((s_7_8) & (s_7_15));
        // D s_7_17: select s_7_12 s_7_14 s_7_16
        let s_7_17: u64 = if s_7_12 { s_7_14 } else { s_7_16 };
        // D s_7_18: cast trunc s_7_17 -> u8
        let s_7_18: bool = ((s_7_17) != 0);
        // D s_7_19: cast zx s_7_18 -> bv
        let s_7_19: Bits = Bits::new(s_7_18 as u128, 1u16);
        // C s_7_20: const #1u : u8
        let s_7_20: bool = true;
        // C s_7_21: cast zx s_7_20 -> bv
        let s_7_21: Bits = Bits::new(s_7_20 as u128, 1u16);
        // D s_7_22: cmp-eq s_7_19 s_7_21
        let s_7_22: bool = ((s_7_19) == (s_7_21));
        // N s_7_23: branch s_7_22 b10 b8
        if s_7_22 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_8_0: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var i:i64
        let s_9_0: i64 = fn_state.i;
        // C s_9_1: const #1s : i64
        let s_9_1: i64 = 1;
        // D s_9_2: add s_9_0 s_9_1
        let s_9_2: i64 = (s_9_0 + s_9_1);
        // D s_9_3: write-var i <= s_9_2
        fn_state.i = s_9_2;
        // N s_9_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_10_0: read-var SVLshadow#5549:i64
        let s_10_0: i64 = fn_state.SVLshadow_5549;
        // D s_10_1: cast zx s_10_0 -> i
        let s_10_1: i128 = (i128::try_from(s_10_0).unwrap());
        // D s_10_2: cast reint s_10_1 -> i64
        let s_10_2: i64 = (s_10_1 as i64);
        // D s_10_3: read-var dim_dim_esizeshadow#5547:i64
        let s_10_3: i64 = fn_state.dim_dim_esizeshadow_5547;
        // D s_10_4: cast zx s_10_3 -> i
        let s_10_4: i128 = (i128::try_from(s_10_3).unwrap());
        // D s_10_5: cast reint s_10_4 -> i64
        let s_10_5: i64 = (s_10_4 as i64);
        // D s_10_6: cast zx s_10_2 -> i
        let s_10_6: i128 = (i128::try_from(s_10_2).unwrap());
        // D s_10_7: read-var i:i64
        let s_10_7: i64 = fn_state.i;
        // D s_10_8: cast zx s_10_7 -> i
        let s_10_8: i128 = (i128::try_from(s_10_7).unwrap());
        // D s_10_9: read-var esize:i64
        let s_10_9: i64 = fn_state.esize;
        // D s_10_10: cast zx s_10_9 -> i
        let s_10_10: i128 = (i128::try_from(s_10_9).unwrap());
        // D s_10_11: cast zx s_10_5 -> i
        let s_10_11: i128 = (i128::try_from(s_10_5).unwrap());
        // D s_10_12: read-var result:bv
        let s_10_12: Bits = fn_state.result;
        // D s_10_13: call ZAtile_set(s_10_6, s_10_8, s_10_10, s_10_11, s_10_12)
        let s_10_13: () = ZAtile_set(
            state,
            tracer,
            s_10_6,
            s_10_8,
            s_10_10,
            s_10_11,
            s_10_12,
        );
        // N s_10_14: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_11_0: return
        return;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #2u : u32
        let s_12_0: u32 = 2;
        // C s_12_1: const #0u : u8
        let s_12_1: bool = false;
        // S s_12_2: call FailTransaction(s_12_0, s_12_1)
        let s_12_2: () = FailTransaction(state, tracer, s_12_0, s_12_1);
        // N s_12_3: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #100180u : u32
        let s_13_0: u32 = 100180;
        // D s_13_1: read-reg s_13_0:i
        let s_13_1: i128 = {
            let value = state.read_register::<i128>(s_13_0 as isize);
            tracer.read_register(s_13_0 as isize, value);
            value
        };
        // C s_13_2: const #0s : i
        let s_13_2: i128 = 0;
        // D s_13_3: cmp-gt s_13_1 s_13_2
        let s_13_3: bool = ((s_13_1) > (s_13_2));
        // D s_13_4: write-var gs#258140 <= s_13_3
        fn_state.gs_258140 = s_13_3;
        // N s_13_5: jump b3
        return block_3(state, tracer, fn_state);
    }
}
