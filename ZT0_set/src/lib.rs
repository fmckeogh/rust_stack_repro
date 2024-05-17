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
use common::*;
pub fn ZT0_set<T: Tracer>(
    state: &mut State,
    tracer: &T,
    width: i64,
    value_name: Bits,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        width: i64,
        value_name: Bits,
    }
    let fn_state = FunctionState {
        width,
        value_name,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #512s : i
        let s_0_0: i128 = 512;
        // D s_0_1: read-var width:i64
        let s_0_1: i64 = fn_state.width;
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (i128::try_from(s_0_1).unwrap());
        // D s_0_3: cmp-eq s_0_2 s_0_0
        let s_0_3: bool = ((s_0_2) == (s_0_0));
        // N s_0_4: assert s_0_3
        let s_0_4: () = assert!(s_0_3);
        // C s_0_5: const #1s : i
        let s_0_5: i128 = 1;
        // D s_0_6: read-var width:i64
        let s_0_6: i64 = fn_state.width;
        // D s_0_7: cast zx s_0_6 -> i
        let s_0_7: i128 = (i128::try_from(s_0_6).unwrap());
        // D s_0_8: sub s_0_7 s_0_5
        let s_0_8: i128 = ((s_0_7) - (s_0_5));
        // D s_0_9: cast reint s_0_8 -> i64
        let s_0_9: i64 = (s_0_8 as i64);
        // C s_0_10: const #0s : i
        let s_0_10: i128 = 0;
        // C s_0_11: const #90520u : u32
        let s_0_11: u32 = 90520;
        // D s_0_12: read-reg s_0_11:u512
        let s_0_12: u64 = {
            let value = state.read_register::<u64>(s_0_11 as isize);
            tracer.read_register(s_0_11 as isize, value);
            value
        };
        // D s_0_13: cast zx s_0_12 -> bv
        let s_0_13: Bits = Bits::new(s_0_12 as u128, 512u16);
        // D s_0_14: cast zx s_0_9 -> i
        let s_0_14: i128 = (i128::try_from(s_0_9).unwrap());
        // D s_0_15: read-var value_name:bv
        let s_0_15: Bits = fn_state.value_name;
        // D s_0_16: sub s_0_14 s_0_10
        let s_0_16: i128 = ((s_0_14) - (s_0_10));
        // C s_0_17: const #1u : u64
        let s_0_17: u64 = 1;
        // C s_0_18: cast zx s_0_17 -> bv
        let s_0_18: Bits = Bits::new(s_0_17 as u128, 64u16);
        // D s_0_19: lsl s_0_18 s_0_16
        let s_0_19: Bits = s_0_18 << s_0_16;
        // D s_0_20: sub s_0_19 s_0_18
        let s_0_20: Bits = ((s_0_19) - (s_0_18));
        // D s_0_21: and s_0_15 s_0_20
        let s_0_21: Bits = ((s_0_15) & (s_0_20));
        // D s_0_22: lsl s_0_21 s_0_10
        let s_0_22: Bits = s_0_21 << s_0_10;
        // D s_0_23: lsl s_0_20 s_0_10
        let s_0_23: Bits = s_0_20 << s_0_10;
        // D s_0_24: cmpl s_0_23
        let s_0_24: Bits = !s_0_23;
        // D s_0_25: and s_0_13 s_0_24
        let s_0_25: Bits = ((s_0_13) & (s_0_24));
        // D s_0_26: or s_0_25 s_0_22
        let s_0_26: Bits = ((s_0_25) | (s_0_22));
        // D s_0_27: cast reint s_0_26 -> u512
        let s_0_27: u64 = (s_0_26.value() as u64);
        // C s_0_28: const #90520u : u32
        let s_0_28: u32 = 90520;
        // N s_0_29: write-reg s_0_28 <= s_0_27
        let s_0_29: () = {
            state.write_register::<u64>(s_0_28 as isize, s_0_27);
            tracer.write_register(s_0_28 as isize, s_0_27);
        };
        // N s_0_30: return
        return;
    }
}
