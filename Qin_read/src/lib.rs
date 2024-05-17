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
use Din_read::*;
use common::*;
pub fn Qin_read<T: Tracer>(state: &mut State, tracer: &T, n: i128) -> u128 {
    #[derive(Default)]
    struct FunctionState {
        gs_31432: bool,
        n: i128,
    }
    let fn_state = FunctionState {
        n,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u128 {
        // C s_0_0: const #0s : i
        let s_0_0: i128 = 0;
        // D s_0_1: read-var n:i
        let s_0_1: i128 = fn_state.n;
        // D s_0_2: cmp-ge s_0_1 s_0_0
        let s_0_2: bool = ((s_0_1) >= (s_0_0));
        // N s_0_3: branch s_0_2 b3 b1
        if s_0_2 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u128 {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#31432 <= s_1_0
        fn_state.gs_31432 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u128 {
        // D s_2_0: read-var gs#31432:u8
        let s_2_0: bool = fn_state.gs_31432;
        // N s_2_1: assert s_2_0
        let s_2_1: () = assert!(s_2_0);
        // C s_2_2: const #2s : i
        let s_2_2: i128 = 2;
        // D s_2_3: read-var n:i
        let s_2_3: i128 = fn_state.n;
        // D s_2_4: mul s_2_2 s_2_3
        let s_2_4: i128 = ((s_2_2) * (s_2_3));
        // D s_2_5: cast reint s_2_4 -> i64
        let s_2_5: i64 = (s_2_4 as i64);
        // C s_2_6: const #1s : i
        let s_2_6: i128 = 1;
        // D s_2_7: cast zx s_2_5 -> i
        let s_2_7: i128 = (i128::try_from(s_2_5).unwrap());
        // D s_2_8: add s_2_7 s_2_6
        let s_2_8: i128 = (s_2_7 + s_2_6);
        // D s_2_9: cast reint s_2_8 -> i64
        let s_2_9: i64 = (s_2_8 as i64);
        // D s_2_10: cast zx s_2_9 -> i
        let s_2_10: i128 = (i128::try_from(s_2_9).unwrap());
        // D s_2_11: call Din_read(s_2_10)
        let s_2_11: u64 = Din_read(state, tracer, s_2_10);
        // C s_2_12: const #2s : i
        let s_2_12: i128 = 2;
        // D s_2_13: read-var n:i
        let s_2_13: i128 = fn_state.n;
        // D s_2_14: mul s_2_12 s_2_13
        let s_2_14: i128 = ((s_2_12) * (s_2_13));
        // D s_2_15: cast reint s_2_14 -> i64
        let s_2_15: i64 = (s_2_14 as i64);
        // D s_2_16: cast zx s_2_15 -> i
        let s_2_16: i128 = (i128::try_from(s_2_15).unwrap());
        // D s_2_17: call Din_read(s_2_16)
        let s_2_17: u64 = Din_read(state, tracer, s_2_16);
        // D s_2_18: cast zx s_2_11 -> bv
        let s_2_18: Bits = Bits::new(s_2_11 as u128, 64u16);
        // D s_2_19: cast zx s_2_17 -> bv
        let s_2_19: Bits = Bits::new(s_2_17 as u128, 64u16);
        // D s_2_20: cast reint s_2_18 -> u128
        let s_2_20: u128 = (s_2_18.value() as u128);
        // D s_2_21: size-of s_2_18
        let s_2_21: u16 = s_2_18.length();
        // D s_2_22: cast reint s_2_19 -> u128
        let s_2_22: u128 = (s_2_19.value() as u128);
        // D s_2_23: size-of s_2_19
        let s_2_23: u16 = s_2_19.length();
        // D s_2_24: lsl s_2_20 s_2_23
        let s_2_24: u128 = s_2_20 << s_2_23;
        // D s_2_25: or s_2_24 s_2_22
        let s_2_25: u128 = ((s_2_24) | (s_2_22));
        // D s_2_26: add s_2_21 s_2_23
        let s_2_26: u16 = (s_2_21 + s_2_23);
        // D s_2_27: create-bits s_2_25 s_2_26
        let s_2_27: Bits = Bits::new(s_2_25, s_2_26);
        // D s_2_28: cast reint s_2_27 -> u128
        let s_2_28: u128 = (s_2_27.value() as u128);
        // N s_2_29: return s_2_28
        return s_2_28;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u128 {
        // C s_3_0: const #15s : i
        let s_3_0: i128 = 15;
        // D s_3_1: read-var n:i
        let s_3_1: i128 = fn_state.n;
        // D s_3_2: cmp-le s_3_1 s_3_0
        let s_3_2: bool = ((s_3_1) <= (s_3_0));
        // D s_3_3: write-var gs#31432 <= s_3_2
        fn_state.gs_31432 = s_3_2;
        // N s_3_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
