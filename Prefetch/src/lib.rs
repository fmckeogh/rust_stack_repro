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
use Hint_Prefetch::*;
use common::*;
pub fn Prefetch<T: Tracer>(
    state: &mut State,
    tracer: &T,
    address: u64,
    prfop: u8,
) -> () {
    #[derive(Default)]
    struct FunctionState {
        ga_20635: u8,
        hint: u32,
        address: u64,
        prfop: u8,
    }
    let fn_state = FunctionState {
        address,
        prfop,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #3s : i
        let s_0_0: i128 = 3;
        // D s_0_1: read-var prfop:u8
        let s_0_1: u8 = fn_state.prfop;
        // D s_0_2: cast zx s_0_1 -> bv
        let s_0_2: Bits = Bits::new(s_0_1 as u128, 5u16);
        // C s_0_3: const #1s : i64
        let s_0_3: i64 = 1;
        // C s_0_4: cast zx s_0_3 -> i
        let s_0_4: i128 = (i128::try_from(s_0_3).unwrap());
        // C s_0_5: const #1s : i
        let s_0_5: i128 = 1;
        // C s_0_6: add s_0_5 s_0_4
        let s_0_6: i128 = (s_0_5 + s_0_4);
        // D s_0_7: bit-extract s_0_2 s_0_0 s_0_6
        let s_0_7: Bits = (Bits::new(
            ((s_0_2) >> (s_0_0)).value(),
            u16::try_from(s_0_6).unwrap(),
        ));
        // D s_0_8: cast reint s_0_7 -> u8
        let s_0_8: u8 = (s_0_7.value() as u8);
        // D s_0_9: write-var ga#20635 <= s_0_8
        fn_state.ga_20635 = s_0_8;
        // D s_0_10: read-var ga#20635:u8
        let s_0_10: u8 = fn_state.ga_20635;
        // D s_0_11: cast zx s_0_10 -> bv
        let s_0_11: Bits = Bits::new(s_0_10 as u128, 2u16);
        // C s_0_12: const #0u : u8
        let s_0_12: u8 = 0;
        // C s_0_13: cast zx s_0_12 -> bv
        let s_0_13: Bits = Bits::new(s_0_12 as u128, 2u16);
        // D s_0_14: cmp-eq s_0_11 s_0_13
        let s_0_14: bool = ((s_0_11) == (s_0_13));
        // D s_0_15: not s_0_14
        let s_0_15: bool = !s_0_14;
        // N s_0_16: branch s_0_15 b3 b1
        if s_0_15 {
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
        // C s_1_0: const #0u : u32
        let s_1_0: u32 = 0;
        // D s_1_1: write-var hint <= s_1_0
        fn_state.hint = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_2_0: const #1s : i
        let s_2_0: i128 = 1;
        // D s_2_1: read-var prfop:u8
        let s_2_1: u8 = fn_state.prfop;
        // D s_2_2: cast zx s_2_1 -> bv
        let s_2_2: Bits = Bits::new(s_2_1 as u128, 5u16);
        // C s_2_3: const #1s : i64
        let s_2_3: i64 = 1;
        // C s_2_4: cast zx s_2_3 -> i
        let s_2_4: i128 = (i128::try_from(s_2_3).unwrap());
        // C s_2_5: const #1s : i
        let s_2_5: i128 = 1;
        // C s_2_6: add s_2_5 s_2_4
        let s_2_6: i128 = (s_2_5 + s_2_4);
        // D s_2_7: bit-extract s_2_2 s_2_0 s_2_6
        let s_2_7: Bits = (Bits::new(
            ((s_2_2) >> (s_2_0)).value(),
            u16::try_from(s_2_6).unwrap(),
        ));
        // D s_2_8: cast reint s_2_7 -> u8
        let s_2_8: u8 = (s_2_7.value() as u8);
        // D s_2_9: cast zx s_2_8 -> bv
        let s_2_9: Bits = Bits::new(s_2_8 as u128, 2u16);
        // D s_2_10: cast zx s_2_9 -> i
        let s_2_10: i128 = (s_2_9.value() as i128);
        // D s_2_11: cast reint s_2_10 -> i64
        let s_2_11: i64 = (s_2_10 as i64);
        // C s_2_12: const #0s : i
        let s_2_12: i128 = 0;
        // D s_2_13: read-var prfop:u8
        let s_2_13: u8 = fn_state.prfop;
        // D s_2_14: cast zx s_2_13 -> bv
        let s_2_14: Bits = Bits::new(s_2_13 as u128, 5u16);
        // C s_2_15: const #1u : u64
        let s_2_15: u64 = 1;
        // D s_2_16: bit-extract s_2_14 s_2_12 s_2_15
        let s_2_16: Bits = (Bits::new(
            ((s_2_14) >> (s_2_12)).value(),
            u16::try_from(s_2_15).unwrap(),
        ));
        // D s_2_17: cast reint s_2_16 -> u8
        let s_2_17: bool = ((s_2_16.value()) != 0);
        // C s_2_18: const #0s : i
        let s_2_18: i128 = 0;
        // C s_2_19: const #0u : u64
        let s_2_19: u64 = 0;
        // D s_2_20: cast zx s_2_17 -> u64
        let s_2_20: u64 = (s_2_17 as u64);
        // C s_2_21: const #1u : u64
        let s_2_21: u64 = 1;
        // D s_2_22: and s_2_20 s_2_21
        let s_2_22: u64 = ((s_2_20) & (s_2_21));
        // D s_2_23: cmp-eq s_2_22 s_2_21
        let s_2_23: bool = ((s_2_22) == (s_2_21));
        // D s_2_24: lsl s_2_20 s_2_18
        let s_2_24: u64 = s_2_20 << s_2_18;
        // D s_2_25: or s_2_19 s_2_24
        let s_2_25: u64 = ((s_2_19) | (s_2_24));
        // D s_2_26: cmpl s_2_24
        let s_2_26: u64 = !s_2_24;
        // D s_2_27: and s_2_19 s_2_26
        let s_2_27: u64 = ((s_2_19) & (s_2_26));
        // D s_2_28: select s_2_23 s_2_25 s_2_27
        let s_2_28: u64 = if s_2_23 { s_2_25 } else { s_2_27 };
        // D s_2_29: cast trunc s_2_28 -> u8
        let s_2_29: bool = ((s_2_28) != 0);
        // D s_2_30: cast zx s_2_29 -> bv
        let s_2_30: Bits = Bits::new(s_2_29 as u128, 1u16);
        // C s_2_31: const #0u : u8
        let s_2_31: bool = false;
        // C s_2_32: cast zx s_2_31 -> bv
        let s_2_32: Bits = Bits::new(s_2_31 as u128, 1u16);
        // D s_2_33: cmp-ne s_2_30 s_2_32
        let s_2_33: bool = ((s_2_30) != (s_2_32));
        // D s_2_34: cast zx s_2_11 -> i
        let s_2_34: i128 = (i128::try_from(s_2_11).unwrap());
        // D s_2_35: read-var address:u64
        let s_2_35: u64 = fn_state.address;
        // D s_2_36: read-var hint:u32
        let s_2_36: u32 = fn_state.hint;
        // D s_2_37: call Hint_Prefetch(s_2_35, s_2_36, s_2_34, s_2_33)
        let s_2_37: () = Hint_Prefetch(state, tracer, s_2_35, s_2_36, s_2_34, s_2_33);
        // N s_2_38: return
        return;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_3_0: read-var ga#20635:u8
        let s_3_0: u8 = fn_state.ga_20635;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 2u16);
        // C s_3_2: const #1u : u8
        let s_3_2: u8 = 1;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 2u16);
        // D s_3_4: cmp-eq s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) == (s_3_3));
        // D s_3_5: not s_3_4
        let s_3_5: bool = !s_3_4;
        // N s_3_6: branch s_3_5 b5 b4
        if s_3_5 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_4_0: const #2u : u32
        let s_4_0: u32 = 2;
        // D s_4_1: write-var hint <= s_4_0
        fn_state.hint = s_4_0;
        // N s_4_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_5_0: read-var ga#20635:u8
        let s_5_0: u8 = fn_state.ga_20635;
        // D s_5_1: cast zx s_5_0 -> bv
        let s_5_1: Bits = Bits::new(s_5_0 as u128, 2u16);
        // C s_5_2: const #2u : u8
        let s_5_2: u8 = 2;
        // C s_5_3: cast zx s_5_2 -> bv
        let s_5_3: Bits = Bits::new(s_5_2 as u128, 2u16);
        // D s_5_4: cmp-eq s_5_1 s_5_3
        let s_5_4: bool = ((s_5_1) == (s_5_3));
        // D s_5_5: not s_5_4
        let s_5_5: bool = !s_5_4;
        // N s_5_6: branch s_5_5 b7 b6
        if s_5_5 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_6_0: const #1u : u32
        let s_6_0: u32 = 1;
        // D s_6_1: write-var hint <= s_6_0
        fn_state.hint = s_6_0;
        // N s_6_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // N s_7_0: return
        return;
    }
}
