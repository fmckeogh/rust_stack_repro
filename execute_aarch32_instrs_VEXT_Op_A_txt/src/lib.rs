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
use D_set::*;
use Q_read::*;
use u__id::*;
use CheckAdvSIMDEnabled::*;
use D_read::*;
use Q_set::*;
use common::*;
pub fn execute_aarch32_instrs_VEXT_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    m: i64,
    n: i64,
    position: i64,
    quadword_operation: bool,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_308827: bool,
        gs_308826: bool,
        d: i64,
        m: i64,
        n: i64,
        position: i64,
        quadword_operation: bool,
    }
    let fn_state = FunctionState {
        d,
        m,
        n,
        position,
        quadword_operation,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call CheckAdvSIMDEnabled(s_0_0)
        let s_0_1: () = CheckAdvSIMDEnabled(state, tracer, s_0_0);
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var quadword_operation:u8
        let s_1_0: bool = fn_state.quadword_operation;
        // N s_1_1: branch s_1_0 b9 b2
        if s_1_0 {
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
        // D s_2_0: read-var position:i64
        let s_2_0: i64 = fn_state.position;
        // D s_2_1: cast zx s_2_0 -> i
        let s_2_1: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_2: call __id(s_2_1)
        let s_2_2: i128 = u__id(state, tracer, s_2_1);
        // D s_2_3: cast reint s_2_2 -> i64
        let s_2_3: i64 = (s_2_2 as i64);
        // C s_2_4: const #0s : i
        let s_2_4: i128 = 0;
        // D s_2_5: cast zx s_2_3 -> i
        let s_2_5: i128 = (i128::try_from(s_2_3).unwrap());
        // D s_2_6: cmp-le s_2_4 s_2_5
        let s_2_6: bool = ((s_2_4) <= (s_2_5));
        // N s_2_7: branch s_2_6 b5 b3
        if s_2_6 {
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
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#308827 <= s_3_0
        fn_state.gs_308827 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#308827:u8
        let s_4_0: bool = fn_state.gs_308827;
        // N s_4_1: assert s_4_0
        let s_4_1: () = assert!(s_4_0);
        // D s_4_2: read-var m:i64
        let s_4_2: i64 = fn_state.m;
        // D s_4_3: cast zx s_4_2 -> i
        let s_4_3: i128 = (i128::try_from(s_4_2).unwrap());
        // D s_4_4: call D_read(s_4_3)
        let s_4_4: u64 = D_read(state, tracer, s_4_3);
        // D s_4_5: read-var n:i64
        let s_4_5: i64 = fn_state.n;
        // D s_4_6: cast zx s_4_5 -> i
        let s_4_6: i128 = (i128::try_from(s_4_5).unwrap());
        // D s_4_7: call D_read(s_4_6)
        let s_4_7: u64 = D_read(state, tracer, s_4_6);
        // D s_4_8: cast zx s_4_4 -> bv
        let s_4_8: Bits = Bits::new(s_4_4 as u128, 64u16);
        // D s_4_9: cast zx s_4_7 -> bv
        let s_4_9: Bits = Bits::new(s_4_7 as u128, 64u16);
        // D s_4_10: cast reint s_4_8 -> u128
        let s_4_10: u128 = (s_4_8.value() as u128);
        // D s_4_11: size-of s_4_8
        let s_4_11: u16 = s_4_8.length();
        // D s_4_12: cast reint s_4_9 -> u128
        let s_4_12: u128 = (s_4_9.value() as u128);
        // D s_4_13: size-of s_4_9
        let s_4_13: u16 = s_4_9.length();
        // D s_4_14: lsl s_4_10 s_4_13
        let s_4_14: u128 = s_4_10 << s_4_13;
        // D s_4_15: or s_4_14 s_4_12
        let s_4_15: u128 = ((s_4_14) | (s_4_12));
        // D s_4_16: add s_4_11 s_4_13
        let s_4_16: u16 = (s_4_11 + s_4_13);
        // D s_4_17: create-bits s_4_15 s_4_16
        let s_4_17: Bits = Bits::new(s_4_15, s_4_16);
        // D s_4_18: cast reint s_4_17 -> u128
        let s_4_18: u128 = (s_4_17.value() as u128);
        // C s_4_19: const #64s : i
        let s_4_19: i128 = 64;
        // D s_4_20: cast zx s_4_18 -> bv
        let s_4_20: Bits = Bits::new(s_4_18 as u128, 128u16);
        // D s_4_21: read-var position:i64
        let s_4_21: i64 = fn_state.position;
        // D s_4_22: cast zx s_4_21 -> i
        let s_4_22: i128 = (i128::try_from(s_4_21).unwrap());
        // D s_4_23: bit-extract s_4_20 s_4_22 s_4_19
        let s_4_23: Bits = (Bits::new(
            ((s_4_20) >> (s_4_22)).value(),
            u16::try_from(s_4_19).unwrap(),
        ));
        // D s_4_24: cast reint s_4_23 -> u64
        let s_4_24: u64 = (s_4_23.value() as u64);
        // D s_4_25: read-var d:i64
        let s_4_25: i64 = fn_state.d;
        // D s_4_26: cast zx s_4_25 -> i
        let s_4_26: i128 = (i128::try_from(s_4_25).unwrap());
        // D s_4_27: call D_set(s_4_26, s_4_24)
        let s_4_27: () = D_set(state, tracer, s_4_26, s_4_24);
        // N s_4_28: return
        return;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var position:i64
        let s_5_0: i64 = fn_state.position;
        // D s_5_1: cast zx s_5_0 -> i
        let s_5_1: i128 = (i128::try_from(s_5_0).unwrap());
        // D s_5_2: call __id(s_5_1)
        let s_5_2: i128 = u__id(state, tracer, s_5_1);
        // D s_5_3: cast reint s_5_2 -> i64
        let s_5_3: i64 = (s_5_2 as i64);
        // D s_5_4: read-var position:i64
        let s_5_4: i64 = fn_state.position;
        // D s_5_5: cast zx s_5_4 -> i
        let s_5_5: i128 = (i128::try_from(s_5_4).unwrap());
        // D s_5_6: call __id(s_5_5)
        let s_5_6: i128 = u__id(state, tracer, s_5_5);
        // D s_5_7: cast reint s_5_6 -> i64
        let s_5_7: i64 = (s_5_6 as i64);
        // C s_5_8: const #63s : i
        let s_5_8: i128 = 63;
        // D s_5_9: cast zx s_5_7 -> i
        let s_5_9: i128 = (i128::try_from(s_5_7).unwrap());
        // D s_5_10: add s_5_9 s_5_8
        let s_5_10: i128 = (s_5_9 + s_5_8);
        // D s_5_11: cast reint s_5_10 -> i64
        let s_5_11: i64 = (s_5_10 as i64);
        // D s_5_12: cast zx s_5_3 -> i
        let s_5_12: i128 = (i128::try_from(s_5_3).unwrap());
        // D s_5_13: cast zx s_5_11 -> i
        let s_5_13: i128 = (i128::try_from(s_5_11).unwrap());
        // D s_5_14: cmp-le s_5_12 s_5_13
        let s_5_14: bool = ((s_5_12) <= (s_5_13));
        // N s_5_15: branch s_5_14 b8 b6
        if s_5_14 {
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
        // D s_6_1: write-var gs#308826 <= s_6_0
        fn_state.gs_308826 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var gs#308826:u8
        let s_7_0: bool = fn_state.gs_308826;
        // D s_7_1: write-var gs#308827 <= s_7_0
        fn_state.gs_308827 = s_7_0;
        // N s_7_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_8_0: read-var position:i64
        let s_8_0: i64 = fn_state.position;
        // D s_8_1: cast zx s_8_0 -> i
        let s_8_1: i128 = (i128::try_from(s_8_0).unwrap());
        // D s_8_2: call __id(s_8_1)
        let s_8_2: i128 = u__id(state, tracer, s_8_1);
        // D s_8_3: cast reint s_8_2 -> i64
        let s_8_3: i64 = (s_8_2 as i64);
        // C s_8_4: const #63s : i
        let s_8_4: i128 = 63;
        // D s_8_5: cast zx s_8_3 -> i
        let s_8_5: i128 = (i128::try_from(s_8_3).unwrap());
        // D s_8_6: add s_8_5 s_8_4
        let s_8_6: i128 = (s_8_5 + s_8_4);
        // D s_8_7: cast reint s_8_6 -> i64
        let s_8_7: i64 = (s_8_6 as i64);
        // C s_8_8: const #128s : i
        let s_8_8: i128 = 128;
        // D s_8_9: cast zx s_8_7 -> i
        let s_8_9: i128 = (i128::try_from(s_8_7).unwrap());
        // D s_8_10: cmp-lt s_8_9 s_8_8
        let s_8_10: bool = ((s_8_9) < (s_8_8));
        // D s_8_11: write-var gs#308826 <= s_8_10
        fn_state.gs_308826 = s_8_10;
        // N s_8_12: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_9_0: const #1s : i64
        let s_9_0: i64 = 1;
        // D s_9_1: read-var d:i64
        let s_9_1: i64 = fn_state.d;
        // D s_9_2: lsr s_9_1 s_9_0
        let s_9_2: i64 = s_9_1 >> s_9_0;
        // C s_9_3: const #1s : i64
        let s_9_3: i64 = 1;
        // D s_9_4: read-var m:i64
        let s_9_4: i64 = fn_state.m;
        // D s_9_5: lsr s_9_4 s_9_3
        let s_9_5: i64 = s_9_4 >> s_9_3;
        // D s_9_6: cast zx s_9_5 -> i
        let s_9_6: i128 = (i128::try_from(s_9_5).unwrap());
        // D s_9_7: call Q_read(s_9_6)
        let s_9_7: u128 = Q_read(state, tracer, s_9_6);
        // C s_9_8: const #1s : i64
        let s_9_8: i64 = 1;
        // D s_9_9: read-var n:i64
        let s_9_9: i64 = fn_state.n;
        // D s_9_10: lsr s_9_9 s_9_8
        let s_9_10: i64 = s_9_9 >> s_9_8;
        // D s_9_11: cast zx s_9_10 -> i
        let s_9_11: i128 = (i128::try_from(s_9_10).unwrap());
        // D s_9_12: call Q_read(s_9_11)
        let s_9_12: u128 = Q_read(state, tracer, s_9_11);
        // D s_9_13: cast zx s_9_7 -> bv
        let s_9_13: Bits = Bits::new(s_9_7 as u128, 128u16);
        // D s_9_14: cast zx s_9_12 -> bv
        let s_9_14: Bits = Bits::new(s_9_12 as u128, 128u16);
        // D s_9_15: cast reint s_9_13 -> u128
        let s_9_15: u128 = (s_9_13.value() as u128);
        // D s_9_16: size-of s_9_13
        let s_9_16: u16 = s_9_13.length();
        // D s_9_17: cast reint s_9_14 -> u128
        let s_9_17: u128 = (s_9_14.value() as u128);
        // D s_9_18: size-of s_9_14
        let s_9_18: u16 = s_9_14.length();
        // D s_9_19: lsl s_9_15 s_9_18
        let s_9_19: u128 = s_9_15 << s_9_18;
        // D s_9_20: or s_9_19 s_9_17
        let s_9_20: u128 = ((s_9_19) | (s_9_17));
        // D s_9_21: add s_9_16 s_9_18
        let s_9_21: u16 = (s_9_16 + s_9_18);
        // D s_9_22: create-bits s_9_20 s_9_21
        let s_9_22: Bits = Bits::new(s_9_20, s_9_21);
        // D s_9_23: cast reint s_9_22 -> u256
        let s_9_23: u64 = (s_9_22.value() as u64);
        // C s_9_24: const #128s : i
        let s_9_24: i128 = 128;
        // D s_9_25: cast zx s_9_23 -> bv
        let s_9_25: Bits = Bits::new(s_9_23 as u128, 256u16);
        // D s_9_26: read-var position:i64
        let s_9_26: i64 = fn_state.position;
        // D s_9_27: cast zx s_9_26 -> i
        let s_9_27: i128 = (i128::try_from(s_9_26).unwrap());
        // D s_9_28: bit-extract s_9_25 s_9_27 s_9_24
        let s_9_28: Bits = (Bits::new(
            ((s_9_25) >> (s_9_27)).value(),
            u16::try_from(s_9_24).unwrap(),
        ));
        // D s_9_29: cast reint s_9_28 -> u128
        let s_9_29: u128 = (s_9_28.value() as u128);
        // D s_9_30: cast zx s_9_2 -> i
        let s_9_30: i128 = (i128::try_from(s_9_2).unwrap());
        // D s_9_31: call Q_set(s_9_30, s_9_29)
        let s_9_31: () = Q_set(state, tracer, s_9_30, s_9_29);
        // N s_9_32: return
        return;
    }
}
