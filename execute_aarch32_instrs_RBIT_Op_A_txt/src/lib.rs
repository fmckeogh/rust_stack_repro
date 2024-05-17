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
use R_set::*;
use Bit::*;
use R_read::*;
use common::*;
pub fn execute_aarch32_instrs_RBIT_Op_A_txt<T: Tracer>(
    state: &mut State,
    tracer: &T,
    d: i64,
    m: i64,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        result: u32,
        i: i64,
        d: i64,
        m: i64,
    }
    let fn_state = FunctionState {
        d,
        m,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #0s : i64
        let s_0_0: i64 = 0;
        // D s_0_1: write-var i <= s_0_0
        fn_state.i = s_0_0;
        // N s_0_2: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_1_0: read-var i:i64
        let s_1_0: i64 = fn_state.i;
        // C s_1_1: const #31s : i64
        let s_1_1: i64 = 31;
        // D s_1_2: cmp-gt s_1_0 s_1_1
        let s_1_2: bool = ((s_1_0) > (s_1_1));
        // N s_1_3: branch s_1_2 b3 b2
        if s_1_2 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #31s : i
        let s_2_0: i128 = 31;
        // D s_2_1: read-var i:i64
        let s_2_1: i64 = fn_state.i;
        // D s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (i128::try_from(s_2_1).unwrap());
        // D s_2_3: sub s_2_0 s_2_2
        let s_2_3: i128 = ((s_2_0) - (s_2_2));
        // D s_2_4: cast reint s_2_3 -> i64
        let s_2_4: i64 = (s_2_3 as i64);
        // D s_2_5: read-var m:i64
        let s_2_5: i64 = fn_state.m;
        // D s_2_6: cast zx s_2_5 -> i
        let s_2_6: i128 = (i128::try_from(s_2_5).unwrap());
        // D s_2_7: call R_read(s_2_6)
        let s_2_7: u32 = R_read(state, tracer, s_2_6);
        // D s_2_8: cast zx s_2_7 -> bv
        let s_2_8: Bits = Bits::new(s_2_7 as u128, 32u16);
        // D s_2_9: read-var i:i64
        let s_2_9: i64 = fn_state.i;
        // D s_2_10: cast zx s_2_9 -> i
        let s_2_10: i128 = (i128::try_from(s_2_9).unwrap());
        // C s_2_11: const #1u : u64
        let s_2_11: u64 = 1;
        // D s_2_12: bit-extract s_2_8 s_2_10 s_2_11
        let s_2_12: Bits = (Bits::new(
            ((s_2_8) >> (s_2_10)).value(),
            u16::try_from(s_2_11).unwrap(),
        ));
        // D s_2_13: cast reint s_2_12 -> u8
        let s_2_13: bool = ((s_2_12.value()) != 0);
        // C s_2_14: const #0s : i
        let s_2_14: i128 = 0;
        // C s_2_15: const #0u : u64
        let s_2_15: u64 = 0;
        // D s_2_16: cast zx s_2_13 -> u64
        let s_2_16: u64 = (s_2_13 as u64);
        // C s_2_17: const #1u : u64
        let s_2_17: u64 = 1;
        // D s_2_18: and s_2_16 s_2_17
        let s_2_18: u64 = ((s_2_16) & (s_2_17));
        // D s_2_19: cmp-eq s_2_18 s_2_17
        let s_2_19: bool = ((s_2_18) == (s_2_17));
        // D s_2_20: lsl s_2_16 s_2_14
        let s_2_20: u64 = s_2_16 << s_2_14;
        // D s_2_21: or s_2_15 s_2_20
        let s_2_21: u64 = ((s_2_15) | (s_2_20));
        // D s_2_22: cmpl s_2_20
        let s_2_22: u64 = !s_2_20;
        // D s_2_23: and s_2_15 s_2_22
        let s_2_23: u64 = ((s_2_15) & (s_2_22));
        // D s_2_24: select s_2_19 s_2_21 s_2_23
        let s_2_24: u64 = if s_2_19 { s_2_21 } else { s_2_23 };
        // D s_2_25: cast trunc s_2_24 -> u8
        let s_2_25: bool = ((s_2_24) != 0);
        // D s_2_26: call Bit(s_2_25)
        let s_2_26: bool = Bit(state, tracer, s_2_25);
        // D s_2_27: read-var result:u32
        let s_2_27: u32 = fn_state.result;
        // D s_2_28: cast zx s_2_27 -> bv
        let s_2_28: Bits = Bits::new(s_2_27 as u128, 32u16);
        // D s_2_29: cast zx s_2_4 -> i
        let s_2_29: i128 = (i128::try_from(s_2_4).unwrap());
        // C s_2_30: const #1u : u64
        let s_2_30: u64 = 1;
        // D s_2_31: bit-insert s_2_28 s_2_28 s_2_29 s_2_30
        let s_2_31: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_2_30 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_2_28.length(),
            );
            (s_2_28 & mask) | (s_2_28 << s_2_29)
        };
        // D s_2_32: cast reint s_2_31 -> u32
        let s_2_32: u32 = (s_2_31.value() as u32);
        // D s_2_33: write-var result <= s_2_32
        fn_state.result = s_2_32;
        // D s_2_34: read-var i:i64
        let s_2_34: i64 = fn_state.i;
        // C s_2_35: const #1s : i64
        let s_2_35: i64 = 1;
        // D s_2_36: add s_2_34 s_2_35
        let s_2_36: i64 = (s_2_34 + s_2_35);
        // D s_2_37: write-var i <= s_2_36
        fn_state.i = s_2_36;
        // N s_2_38: jump b1
        return block_1(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var d:i64
        let s_3_0: i64 = fn_state.d;
        // D s_3_1: cast zx s_3_0 -> i
        let s_3_1: i128 = (i128::try_from(s_3_0).unwrap());
        // D s_3_2: read-var result:u32
        let s_3_2: u32 = fn_state.result;
        // D s_3_3: call R_set(s_3_1, s_3_2)
        let s_3_3: () = R_set(state, tracer, s_3_1, s_3_2);
        // N s_3_4: return
        return;
    }
}
