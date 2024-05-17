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
use X_set::*;
use X_read::*;
use u__id::*;
use Zeros::*;
use ROR::*;
use replicate_bits_borealis_internal::*;
use common::*;
pub fn execute_aarch64_instrs_integer_bitfield<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    datasize: i64,
    extend: bool,
    inzero: bool,
    n: i64,
    r: i64,
    s: i64,
    tmask: Bits,
    wmask: Bits,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        dst: Bits,
        bot: Bits,
        gs_144069: bool,
        src: Bits,
        datasizeshadow_1076: i64,
        top: Bits,
        d: i64,
        datasize: i64,
        extend: bool,
        inzero: bool,
        n: i64,
        r: i64,
        s: i64,
        tmask: Bits,
        wmask: Bits,
    }
    let fn_state = FunctionState {
        d,
        datasize,
        extend,
        inzero,
        n,
        r,
        s,
        tmask,
        wmask,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_0_0: read-var datasize:i64
        let s_0_0: i64 = fn_state.datasize;
        // D s_0_1: cast zx s_0_0 -> i
        let s_0_1: i128 = (i128::try_from(s_0_0).unwrap());
        // D s_0_2: cast reint s_0_1 -> i64
        let s_0_2: i64 = (s_0_1 as i64);
        // D s_0_3: write-var datasizeshadow#1076 <= s_0_2
        fn_state.datasizeshadow_1076 = s_0_2;
        // D s_0_4: read-var inzero:u8
        let s_0_4: bool = fn_state.inzero;
        // N s_0_5: branch s_0_4 b9 b1
        if s_0_4 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var datasizeshadow#1076:i64
        let s_1_0: i64 = fn_state.datasizeshadow_1076;
        // D s_1_1: cast zx s_1_0 -> i
        let s_1_1: i128 = (i128::try_from(s_1_0).unwrap());
        // D s_1_2: cast reint s_1_1 -> i64
        let s_1_2: i64 = (s_1_1 as i64);
        // D s_1_3: read-var d:i64
        let s_1_3: i64 = fn_state.d;
        // D s_1_4: cast zx s_1_3 -> i
        let s_1_4: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_5: call X_read(s_1_4, s_1_2)
        let s_1_5: Bits = X_read(state, tracer, s_1_4, s_1_2);
        // D s_1_6: write-var dst <= s_1_5
        fn_state.dst = s_1_5;
        // N s_1_7: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var datasizeshadow#1076:i64
        let s_2_0: i64 = fn_state.datasizeshadow_1076;
        // D s_2_1: cast zx s_2_0 -> i
        let s_2_1: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_2: cast reint s_2_1 -> i64
        let s_2_2: i64 = (s_2_1 as i64);
        // D s_2_3: read-var n:i64
        let s_2_3: i64 = fn_state.n;
        // D s_2_4: cast zx s_2_3 -> i
        let s_2_4: i128 = (i128::try_from(s_2_3).unwrap());
        // D s_2_5: call X_read(s_2_4, s_2_2)
        let s_2_5: Bits = X_read(state, tracer, s_2_4, s_2_2);
        // D s_2_6: write-var src <= s_2_5
        fn_state.src = s_2_5;
        // D s_2_7: read-var wmask:bv
        let s_2_7: Bits = fn_state.wmask;
        // D s_2_8: not s_2_7
        let s_2_8: Bits = !s_2_7;
        // D s_2_9: read-var dst:bv
        let s_2_9: Bits = fn_state.dst;
        // D s_2_10: and s_2_9 s_2_8
        let s_2_10: Bits = ((s_2_9) & (s_2_8));
        // D s_2_11: read-var r:i64
        let s_2_11: i64 = fn_state.r;
        // D s_2_12: cast zx s_2_11 -> i
        let s_2_12: i128 = (i128::try_from(s_2_11).unwrap());
        // D s_2_13: read-var src:bv
        let s_2_13: Bits = fn_state.src;
        // D s_2_14: call ROR(s_2_13, s_2_12)
        let s_2_14: Bits = ROR(state, tracer, s_2_13, s_2_12);
        // D s_2_15: read-var wmask:bv
        let s_2_15: Bits = fn_state.wmask;
        // D s_2_16: and s_2_14 s_2_15
        let s_2_16: Bits = ((s_2_14) & (s_2_15));
        // D s_2_17: or s_2_10 s_2_16
        let s_2_17: Bits = ((s_2_10) | (s_2_16));
        // D s_2_18: write-var bot <= s_2_17
        fn_state.bot = s_2_17;
        // D s_2_19: read-var extend:u8
        let s_2_19: bool = fn_state.extend;
        // N s_2_20: branch s_2_19 b5 b3
        if s_2_19 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var dst:bv
        let s_3_0: Bits = fn_state.dst;
        // D s_3_1: write-var top <= s_3_0
        fn_state.top = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var datasizeshadow#1076:i64
        let s_4_0: i64 = fn_state.datasizeshadow_1076;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: cast reint s_4_1 -> i64
        let s_4_2: i64 = (s_4_1 as i64);
        // D s_4_3: read-var tmask:bv
        let s_4_3: Bits = fn_state.tmask;
        // D s_4_4: not s_4_3
        let s_4_4: Bits = !s_4_3;
        // D s_4_5: read-var top:bv
        let s_4_5: Bits = fn_state.top;
        // D s_4_6: and s_4_5 s_4_4
        let s_4_6: Bits = ((s_4_5) & (s_4_4));
        // D s_4_7: read-var bot:bv
        let s_4_7: Bits = fn_state.bot;
        // D s_4_8: read-var tmask:bv
        let s_4_8: Bits = fn_state.tmask;
        // D s_4_9: and s_4_7 s_4_8
        let s_4_9: Bits = ((s_4_7) & (s_4_8));
        // D s_4_10: or s_4_6 s_4_9
        let s_4_10: Bits = ((s_4_6) | (s_4_9));
        // D s_4_11: read-var d:i64
        let s_4_11: i64 = fn_state.d;
        // D s_4_12: cast zx s_4_11 -> i
        let s_4_12: i128 = (i128::try_from(s_4_11).unwrap());
        // D s_4_13: call X_set(s_4_12, s_4_2, s_4_10)
        let s_4_13: () = X_set(state, tracer, s_4_12, s_4_2, s_4_10);
        // N s_4_14: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var s:i64
        let s_5_0: i64 = fn_state.s;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: call __id(s_5_1)
        let s_5_2: i128 = u__id(state, tracer, s_5_1);
        // D s_5_3: cast reint s_5_2 -> i64
        let s_5_3: i64 = (s_5_2 as i64);
        // C s_5_4: const #0s : i
        let s_5_4: i128 = 0;
        // D s_5_5: cast zx s_5_3 -> i
        let s_5_5: i128 = (i128::try_from(s_5_3).unwrap());
        // D s_5_6: cmp-le s_5_4 s_5_5
        let s_5_6: bool = ((s_5_4) <= (s_5_5));
        // N s_5_7: branch s_5_6 b8 b6
        if s_5_6 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #0u : u8
        let s_6_0: bool = false;
        // D s_6_1: write-var gs#144069 <= s_6_0
        fn_state.gs_144069 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#144069:u8
        let s_7_0: bool = fn_state.gs_144069;
        // N s_7_1: assert s_7_0
        let s_7_1: () = assert!(s_7_0);
        // D s_7_2: read-var s:i64
        let s_7_2: i64 = fn_state.s;
        // D s_7_3: cast zx s_7_2 -> i
        let s_7_3: i128 = (i128::try_from(s_7_2).unwrap());
        // D s_7_4: read-var src:bv
        let s_7_4: Bits = fn_state.src;
        // C s_7_5: const #1u : u64
        let s_7_5: u64 = 1;
        // D s_7_6: bit-extract s_7_4 s_7_3 s_7_5
        let s_7_6: Bits = (Bits::new(
            ((s_7_4) >> (s_7_3)).value(),
            u16::try_from(s_7_5).unwrap(),
        ));
        // D s_7_7: cast reint s_7_6 -> u8
        let s_7_7: bool = ((s_7_6.value()) != 0);
        // C s_7_8: const #0s : i
        let s_7_8: i128 = 0;
        // C s_7_9: const #0u : u64
        let s_7_9: u64 = 0;
        // D s_7_10: cast zx s_7_7 -> u64
        let s_7_10: u64 = (s_7_7 as u64);
        // C s_7_11: const #1u : u64
        let s_7_11: u64 = 1;
        // D s_7_12: and s_7_10 s_7_11
        let s_7_12: u64 = ((s_7_10) & (s_7_11));
        // D s_7_13: cmp-eq s_7_12 s_7_11
        let s_7_13: bool = ((s_7_12) == (s_7_11));
        // D s_7_14: lsl s_7_10 s_7_8
        let s_7_14: u64 = s_7_10 << s_7_8;
        // D s_7_15: or s_7_9 s_7_14
        let s_7_15: u64 = ((s_7_9) | (s_7_14));
        // D s_7_16: cmpl s_7_14
        let s_7_16: u64 = !s_7_14;
        // D s_7_17: and s_7_9 s_7_16
        let s_7_17: u64 = ((s_7_9) & (s_7_16));
        // D s_7_18: select s_7_13 s_7_15 s_7_17
        let s_7_18: u64 = if s_7_13 { s_7_15 } else { s_7_17 };
        // D s_7_19: cast trunc s_7_18 -> u8
        let s_7_19: bool = ((s_7_18) != 0);
        // D s_7_20: cast zx s_7_19 -> bv
        let s_7_20: Bits = Bits::new(s_7_19 as u128, 1u16);
        // D s_7_21: read-var datasizeshadow#1076:i64
        let s_7_21: i64 = fn_state.datasizeshadow_1076;
        // D s_7_22: cast zx s_7_21 -> i
        let s_7_22: i128 = (i128::try_from(s_7_21).unwrap());
        // D s_7_23: cast reint s_7_22 -> u64
        let s_7_23: u64 = (s_7_22 as u64);
        // D s_7_24: call replicate_bits_borealis_internal(s_7_20, s_7_23)
        let s_7_24: Bits = replicate_bits_borealis_internal(
            state,
            tracer,
            s_7_20,
            s_7_23,
        );
        // D s_7_25: write-var top <= s_7_24
        fn_state.top = s_7_24;
        // N s_7_26: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var s:i64
        let s_8_0: i64 = fn_state.s;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: call __id(s_8_1)
        let s_8_2: i128 = u__id(state, tracer, s_8_1);
        // D s_8_3: cast reint s_8_2 -> i64
        let s_8_3: i64 = (s_8_2 as i64);
        // D s_8_4: read-var datasizeshadow#1076:i64
        let s_8_4: i64 = fn_state.datasizeshadow_1076;
        // D s_8_5: cast zx s_8_4 -> i
        let s_8_5: i128 = (i128::try_from(s_8_4).unwrap());
        // D s_8_6: call __id(s_8_5)
        let s_8_6: i128 = u__id(state, tracer, s_8_5);
        // D s_8_7: cast reint s_8_6 -> i64
        let s_8_7: i64 = (s_8_6 as i64);
        // D s_8_8: cast zx s_8_3 -> i
        let s_8_8: i128 = (i128::try_from(s_8_3).unwrap());
        // D s_8_9: cast zx s_8_7 -> i
        let s_8_9: i128 = (i128::try_from(s_8_7).unwrap());
        // D s_8_10: cmp-lt s_8_8 s_8_9
        let s_8_10: bool = ((s_8_8) < (s_8_9));
        // D s_8_11: write-var gs#144069 <= s_8_10
        fn_state.gs_144069 = s_8_10;
        // N s_8_12: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var datasizeshadow#1076:i64
        let s_9_0: i64 = fn_state.datasizeshadow_1076;
        // D s_9_1: cast zx s_9_0 -> i
        let s_9_1: i128 = (i128::try_from(s_9_0).unwrap());
        // D s_9_2: call Zeros(s_9_1)
        let s_9_2: Bits = Zeros(state, tracer, s_9_1);
        // D s_9_3: write-var dst <= s_9_2
        fn_state.dst = s_9_2;
        // N s_9_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
