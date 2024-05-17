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
use V_set::*;
use V_read::*;
use common::*;
pub fn S_set<T: Tracer>(state: &mut State, tracer: &T, n: i128, value_name: u32) -> () {
    #[derive(Default)]
    struct FunctionState {
        gs_31236: bool,
        n: i128,
        value_name: u32,
    }
    let fn_state = FunctionState {
        n,
        value_name,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
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
    ) -> () {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#31236 <= s_1_0
        fn_state.gs_31236 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#31236:u8
        let s_2_0: bool = fn_state.gs_31236;
        // N s_2_1: assert s_2_0
        let s_2_1: () = assert!(s_2_0);
        // C s_2_2: const #4s : i
        let s_2_2: i128 = 4;
        // D s_2_3: read-var n:i
        let s_2_3: i128 = fn_state.n;
        // D s_2_4: mod s_2_3 s_2_2
        let s_2_4: i128 = ((s_2_3) % (s_2_2));
        // D s_2_5: cast reint s_2_4 -> i64
        let s_2_5: i64 = (s_2_4 as i64);
        // C s_2_6: const #32s : i
        let s_2_6: i128 = 32;
        // D s_2_7: cast zx s_2_5 -> i
        let s_2_7: i128 = (i128::try_from(s_2_5).unwrap());
        // D s_2_8: mul s_2_7 s_2_6
        let s_2_8: i128 = ((s_2_7) * (s_2_6));
        // D s_2_9: cast reint s_2_8 -> i64
        let s_2_9: i64 = (s_2_8 as i64);
        // C s_2_10: const #4s : i
        let s_2_10: i128 = 4;
        // D s_2_11: read-var n:i
        let s_2_11: i128 = fn_state.n;
        // D s_2_12: div s_2_11 s_2_10
        let s_2_12: i128 = ((s_2_11) / (s_2_10));
        // D s_2_13: cast reint s_2_12 -> i64
        let s_2_13: i64 = (s_2_12 as i64);
        // C s_2_14: const #128s : i64
        let s_2_14: i64 = 128;
        // D s_2_15: cast zx s_2_13 -> i
        let s_2_15: i128 = (i128::try_from(s_2_13).unwrap());
        // D s_2_16: call V_read(s_2_15, s_2_14)
        let s_2_16: Bits = V_read(state, tracer, s_2_15, s_2_14);
        // D s_2_17: cast reint s_2_16 -> u128
        let s_2_17: u128 = (s_2_16.value() as u128);
        // C s_2_18: const #31s : i
        let s_2_18: i128 = 31;
        // D s_2_19: cast zx s_2_9 -> i
        let s_2_19: i128 = (i128::try_from(s_2_9).unwrap());
        // D s_2_20: add s_2_19 s_2_18
        let s_2_20: i128 = (s_2_19 + s_2_18);
        // D s_2_21: cast reint s_2_20 -> i64
        let s_2_21: i64 = (s_2_20 as i64);
        // D s_2_22: cast zx s_2_17 -> bv
        let s_2_22: Bits = Bits::new(s_2_17 as u128, 128u16);
        // D s_2_23: cast zx s_2_21 -> i
        let s_2_23: i128 = (i128::try_from(s_2_21).unwrap());
        // D s_2_24: cast zx s_2_9 -> i
        let s_2_24: i128 = (i128::try_from(s_2_9).unwrap());
        // D s_2_25: read-var value_name:u32
        let s_2_25: u32 = fn_state.value_name;
        // D s_2_26: cast zx s_2_25 -> bv
        let s_2_26: Bits = Bits::new(s_2_25 as u128, 32u16);
        // D s_2_27: sub s_2_23 s_2_24
        let s_2_27: i128 = ((s_2_23) - (s_2_24));
        // C s_2_28: const #1u : u64
        let s_2_28: u64 = 1;
        // C s_2_29: cast zx s_2_28 -> bv
        let s_2_29: Bits = Bits::new(s_2_28 as u128, 64u16);
        // D s_2_30: lsl s_2_29 s_2_27
        let s_2_30: Bits = s_2_29 << s_2_27;
        // D s_2_31: sub s_2_30 s_2_29
        let s_2_31: Bits = ((s_2_30) - (s_2_29));
        // D s_2_32: and s_2_26 s_2_31
        let s_2_32: Bits = ((s_2_26) & (s_2_31));
        // D s_2_33: lsl s_2_32 s_2_24
        let s_2_33: Bits = s_2_32 << s_2_24;
        // D s_2_34: lsl s_2_31 s_2_24
        let s_2_34: Bits = s_2_31 << s_2_24;
        // D s_2_35: cmpl s_2_34
        let s_2_35: Bits = !s_2_34;
        // D s_2_36: and s_2_22 s_2_35
        let s_2_36: Bits = ((s_2_22) & (s_2_35));
        // D s_2_37: or s_2_36 s_2_33
        let s_2_37: Bits = ((s_2_36) | (s_2_33));
        // D s_2_38: cast reint s_2_37 -> u128
        let s_2_38: u128 = (s_2_37.value() as u128);
        // C s_2_39: const #4s : i
        let s_2_39: i128 = 4;
        // D s_2_40: read-var n:i
        let s_2_40: i128 = fn_state.n;
        // D s_2_41: div s_2_40 s_2_39
        let s_2_41: i128 = ((s_2_40) / (s_2_39));
        // D s_2_42: cast reint s_2_41 -> i64
        let s_2_42: i64 = (s_2_41 as i64);
        // C s_2_43: const #128s : i64
        let s_2_43: i64 = 128;
        // D s_2_44: cast zx s_2_42 -> i
        let s_2_44: i128 = (i128::try_from(s_2_42).unwrap());
        // D s_2_45: cast zx s_2_38 -> bv
        let s_2_45: Bits = Bits::new(s_2_38 as u128, 128u16);
        // D s_2_46: call V_set(s_2_44, s_2_43, s_2_45)
        let s_2_46: () = V_set(state, tracer, s_2_44, s_2_43, s_2_45);
        // N s_2_47: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #31s : i
        let s_3_0: i128 = 31;
        // D s_3_1: read-var n:i
        let s_3_1: i128 = fn_state.n;
        // D s_3_2: cmp-le s_3_1 s_3_0
        let s_3_2: bool = ((s_3_1) <= (s_3_0));
        // D s_3_3: write-var gs#31236 <= s_3_2
        fn_state.gs_31236 = s_3_2;
        // N s_3_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
